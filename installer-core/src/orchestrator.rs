use std::path::PathBuf;

use anyhow::{anyhow, Result};
use tracing::info;

use crate::system::{
    dry_run::DryRunLog,
    error::{
        DriverInfo, ErrorSeverity, InstallationReport, InstallerError, InstallerRunError,
        InstallerStateSnapshot,
    },
    lockfile::InstallerLock,
    sudo_password,
};
use crate::{
    context::{ConfigService, PlatformContext, UIContext, UserOptionsContext},
    driver::DistroDriver,
    localization::Localization,
    logging,
    options::InstallOptions,
    phase_registry::PhaseRegistry,
    phase_runner::{PhaseErrorPolicy, PhaseObserver, PhaseRunner},
    platform::detect as detect_platform,
    rollback::RollbackManager,
    signal::SignalGuard,
    InstallContext,
};

pub fn run_with_driver(
    driver: &'static dyn DistroDriver,
    opts: InstallOptions,
    observer: &mut dyn PhaseObserver,
) -> Result<InstallationReport, Box<InstallerRunError>> {
    // Initialize sudo password storage
    sudo_password::init_sudo_password();

    // Acquire exclusive lock to prevent concurrent runs
    let _lock = InstallerLock::acquire().map_err(|e| {
        let err = InstallerError::new(
            "setup",
            "locking repository",
            ErrorSeverity::Fatal,
            e,
            InstallerStateSnapshot::default(),
            Some("Ensure no other mash-setup process is running.".to_string()),
        );
        InstallerRunError {
            report: Box::new(InstallationReport {
                completed_phases: Vec::new(),
                staging_dir: PathBuf::from("<unknown>"),
                errors: vec![err.clone()],
                outputs: Vec::new(),
                events: Vec::new(),
                options: InstallOptions::default(),
                driver: DriverInfo {
                    name: driver.name().to_string(),
                    description: driver.description().to_string(),
                },
                dry_run_log: Vec::new(),
                audit_report: crate::system::dry_run::PreflightAuditReport::default(),
            }),
            source: err,
        }
    })?;

    // Phase 1: Context initialization and platform detection
    let config_service = ConfigService::load().map_err(|e| {
        let err = InstallerError::new(
            "config",
            "configuration load",
            ErrorSeverity::Fatal,
            anyhow::Error::from(e),
            InstallerStateSnapshot::default(),
            Some("Inspect ~/.config/mash-installer/config.toml for corruption or permissions issues.".to_string()),
        );
        Box::new(InstallerRunError {
            report: Box::new(InstallationReport {
                completed_phases: Vec::new(),
                staging_dir: PathBuf::from("<unknown>"),
                errors: vec![err.clone()],
                outputs: Vec::new(),
                events: Vec::new(),
                options: InstallOptions::default(),
                driver: DriverInfo {
                    name: driver.name().to_string(),
                    description: driver.description().to_string(),
                },
                dry_run_log: Vec::new(),
                audit_report: crate::system::dry_run::PreflightAuditReport::default(),
            }),
            source: err,
        })
    })?;

    let platform = detect_platform().map_err(Box::<InstallerRunError>::from)?;

    // Core validation: Ensure the driver matches the hardware
    if !driver.matches(&platform) {
        let err = anyhow!(
            "Distro driver '{}' does not match the detected platform ({}).",
            driver.name(),
            platform.distro_family
        );
        return Err(Box::new(InstallerRunError::from(err)));
    }

    let platform_ctx = PlatformContext {
        config_service,
        platform,
        driver_name: driver.name(),
        driver,
        pkg_backend: driver.pkg_backend(),
        system: &crate::sys_ops::REAL_SYSTEM,
    };

    let localization = Localization::load_default().map_err(Box::<InstallerRunError>::from)?;

    let ctx = InstallContext {
        options: UserOptionsContext::from_options(&opts),
        platform: platform_ctx,
        ui: UIContext,
        interaction: crate::interaction::InteractionService::new(
            opts.interactive,
            Default::default(),
        ),
        localization,
        rollback: RollbackManager::new(),
        dry_run_log: DryRunLog::new(),
    };

    // Ask for sudo password up front if needed
    if !crate::sudo::ensure_sudo_access() {
        match ctx.request_sudo_password(observer) {
            Ok(password) => {
                sudo_password::set_sudo_password(password);
            }
            Err(e) => {
                return Err(Box::new(InstallerRunError::from(e)));
            }
        }
    }

    let _sudo_keepalive = crate::sudo::start_sudo_keepalive();

    // Registry populates phases based on the active profile level
    let registry = PhaseRegistry::default();
    let phases = registry.build_phases(&ctx.options, &ctx.localization);

    let policy = if opts.continue_on_error {
        PhaseErrorPolicy::ContinueOnError
    } else {
        PhaseErrorPolicy::FailFast
    };

    let runner = PhaseRunner::with_policy(phases, policy);

    // Create a signal guard to handle SIGINT/SIGTERM during installation
    let signal_guard = SignalGuard::new().map_err(Box::<InstallerRunError>::from)?;

    let install_span = logging::install_span(&ctx);
    let _install_guard = install_span.enter();

    info!(
        "Starting installation: profile={:?}, driver={}, target={}",
        opts.profile,
        driver.name(),
        ctx.platform.platform.distro
    );

    let opts_final = opts.clone();
    let staging_final = opts
        .staging_dir
        .clone()
        .unwrap_or(ctx.options.staging_dir.clone());

    let result = runner
        .run(&ctx, observer, Some(&signal_guard))
        .map_err(|e| {
            let run_err = *e;
            Box::new(InstallerRunError {
                report: Box::new(InstallationReport {
                    completed_phases: run_err.result.completed_phases,
                    staging_dir: staging_final.clone(),
                    errors: run_err.result.errors,
                    outputs: run_err.result.outputs,
                    events: run_err.result.events,
                    options: opts_final.clone(),
                    driver: DriverInfo {
                        name: driver.name().to_string(),
                        description: driver.description().to_string(),
                    },
                    dry_run_log: ctx.dry_run_log.entries(),
                    audit_report: crate::system::dry_run::PreflightAuditReport::default(),
                }),
                source: run_err.source,
            })
        })?;

    info!(
        "Installation completed successfully: {} phases completed.",
        result.completed_phases.len()
    );

    Ok(InstallationReport {
        completed_phases: result.completed_phases,
        staging_dir: staging_final,
        errors: result.errors,
        outputs: result.outputs,
        events: result.events,
        options: opts_final,
        driver: DriverInfo {
            name: driver.name().to_string(),
            description: driver.description().to_string(),
        },
        dry_run_log: ctx.dry_run_log.entries(),
        audit_report: crate::system::dry_run::PreflightAuditReport::default(),
    })
}

#[allow(dead_code)]
pub fn run_preflight_audit(
    driver: &'static dyn DistroDriver,
    opts: &InstallOptions,
) -> Result<crate::system::dry_run::PreflightAuditReport> {
    let registry = PhaseRegistry::default();
    // Use dummy context for audit
    let platform = detect_platform()?;
    let config_service = ConfigService::load()?;
    let platform_ctx = PlatformContext {
        config_service,
        platform,
        driver_name: driver.name(),
        driver,
        pkg_backend: driver.pkg_backend(),
        system: &crate::sys_ops::REAL_SYSTEM,
    };
    let ctx = InstallContext {
        options: UserOptionsContext::from_options(opts),
        platform: platform_ctx,
        ui: UIContext,
        interaction: crate::interaction::InteractionService::new(
            opts.interactive,
            Default::default(),
        ),
        localization: Localization::load_default()?,
        rollback: RollbackManager::new(),
        dry_run_log: DryRunLog::new(),
    };

    let phases = registry.build_phases(&ctx.options, &ctx.localization);

    for phase in phases {
        if phase.should_run(&ctx) {
            ctx.dry_run_log.record(
                phase.name().to_string(),
                "Planned for execution".to_string(),
                Some(phase.description().to_string()),
            );
        }
    }

    Ok(ctx.dry_run_log.audit_report())
}
