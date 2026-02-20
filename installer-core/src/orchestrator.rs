use std::{path::PathBuf, process::Command};

use anyhow::{anyhow, Result};
use tracing::{error, info};

use crate::{
    context::{ConfigOverrides, ConfigService, PlatformContext, UIContext, UserOptionsContext},
    driver::DistroDriver,
    dry_run::DryRunLog,
    error::{
        DriverInfo, ErrorSeverity, InstallationReport, InstallerError, InstallerRunError,
        InstallerStateSnapshot,
    },
    localization::Localization,
    logging,
    options::InstallOptions,
    phase_registry::PhaseRegistry,
    phase_runner::{PhaseErrorPolicy, PhaseEvent, PhaseObserver, PhaseRunError, PhaseRunner},
    platform::detect as detect_platform,
    rollback::RollbackManager,
    InstallContext,
};

pub fn run_with_driver(
    driver: &'static dyn DistroDriver,
    opts: InstallOptions,
    observer: &mut dyn PhaseObserver,
) -> Result<InstallationReport, Box<InstallerRunError>> {
    if std::env::var("USER").is_err() {
        let user = std::env::var("SUDO_USER").ok().or_else(|| {
            Command::new("whoami")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
        });

        if let Some(username) = user {
            std::env::set_var("USER", &username);
            info!("Set USER environment variable to: {}", username);
        } else {
            error!("WARNING: USER environment variable not set and could not be detected!");
        }
    }

    let _sudo_keepalive = crate::sudo::start_sudo_keepalive();

    let plat = detect_platform()?;
    info!(
        "Platform: {} {} on {}",
        plat.distro, plat.distro_version, plat.arch
    );
    info!(
        "Using distro driver: {} ({})",
        driver.name(),
        driver.description()
    );

    let is_pi_4b = plat
        .pi_model
        .as_ref()
        .is_some_and(|model| model.contains("Raspberry Pi 4") || model.contains("Pi 4"));

    if let Some(ref model) = plat.pi_model {
        info!("Raspberry Pi model: {}", model);
        if is_pi_4b {
            info!("✓ Running on Raspberry Pi 4 - optimal performance!");
        }
    }

    if !is_pi_4b {
        let detected = plat.pi_model.as_deref().unwrap_or("Non-Pi system");
        let warning = format!(
            "This installer is OPTIMIZED FOR RASPBERRY PI 4B 8GB ONLY.\n\
             Detected: {detected}\n\
             Proceeding at your own risk: no performance guarantees, \
             no bug reports accepted for non-Pi4B systems."
        );
        observer.on_event(PhaseEvent::Warning { message: warning });

        if opts.interactive {
            if !observer.confirm("Do you understand the risks and want to proceed anyway? [y/N]") {
                info!("Installation cancelled by user on non-Pi4B system");
                return Err(warn_non_pi_4b(&opts, driver));
            }
            info!("User acknowledged risks and chose to proceed on non-Pi4B system");
        } else {
            info!("Non-interactive mode; proceeding despite non-Pi4B platform");
        }
    }

    let localization = Localization::load()?;
    let api_options = opts.clone();
    let driver_info = DriverInfo {
        name: driver.name().to_string(),
        description: driver.description().to_string(),
    };

    let overrides = ConfigOverrides {
        staging_dir: opts.staging_dir.clone(),
    };
    let config_service = ConfigService::load_with_overrides(overrides)?;
    let staging = config_service.resolve_staging_dir()?;
    info!("Staging directory: {}", staging.display());

    let options = UserOptionsContext {
        profile: opts.profile,
        staging_dir: staging,
        dry_run: opts.dry_run,
        interactive: opts.interactive,
        enable_argon: opts.enable_argon,
        enable_p10k: opts.enable_p10k,
        docker_data_root: opts.docker_data_root,
    };

    let platform_ctx = PlatformContext {
        config_service,
        platform: plat,
        driver_name: driver.name(),
        driver,
        pkg_backend: driver.pkg_backend(),
    };

    let ctx = InstallContext {
        options,
        platform: platform_ctx,
        ui: UIContext,
        localization,
        rollback: RollbackManager::new(),
        dry_run_log: DryRunLog::new(),
    };

    let registry = PhaseRegistry::default();
    let phases = registry.build_phases(&ctx.options, &ctx.localization);
    let policy = if opts.continue_on_error {
        PhaseErrorPolicy::ContinueOnError
    } else {
        PhaseErrorPolicy::default()
    };
    let runner = PhaseRunner::with_policy(phases, policy);
    let install_span = logging::install_span(&ctx);
    let _install_guard = install_span.enter();
    let run_result = runner.run(&ctx, observer);
    let dry_run_log = ctx.dry_run_log.entries();
    match run_result {
        Ok(result) => Ok(InstallationReport {
            completed_phases: result.completed_phases,
            staging_dir: ctx.options.staging_dir.clone(),
            errors: result.errors,
            outputs: result.outputs,
            events: result.events,
            options: api_options.clone(),
            driver: driver_info.clone(),
            dry_run_log: dry_run_log.clone(),
        }),
        Err(err) => {
            let PhaseRunError {
                result: run_result,
                source,
            } = *err;
            let report = InstallationReport {
                completed_phases: run_result.completed_phases,
                staging_dir: ctx.options.staging_dir.clone(),
                errors: run_result.errors,
                outputs: run_result.outputs,
                events: run_result.events,
                options: api_options,
                driver: driver_info,
                dry_run_log,
            };
            Err(Box::new(InstallerRunError {
                report: Box::new(report),
                source,
            }))
        }
    }
}

fn warn_non_pi_4b(
    opts: &InstallOptions,
    driver: &'static dyn DistroDriver,
) -> Box<InstallerRunError> {
    let report = InstallationReport {
        completed_phases: vec![],
        staging_dir: PathBuf::from("/tmp"),
        errors: vec![],
        outputs: Vec::new(),
        events: vec![],
        options: opts.clone(),
        driver: DriverInfo {
            name: driver.name().to_string(),
            description: driver.description().to_string(),
        },
        dry_run_log: Vec::new(),
    };

    Box::new(InstallerRunError {
        report: Box::new(report),
        source: InstallerError::new(
            "platform_check",
            "Platform compatibility check",
            ErrorSeverity::Fatal,
            anyhow!("User declined to proceed on non-Pi4B system"),
            InstallerStateSnapshot::from_options(&UserOptionsContext {
                profile: opts.profile,
                staging_dir: PathBuf::from("/tmp"),
                dry_run: opts.dry_run,
                interactive: opts.interactive,
                enable_argon: opts.enable_argon,
                enable_p10k: opts.enable_p10k,
                docker_data_root: opts.docker_data_root,
            }),
            Some("This installer is designed for Raspberry Pi 4B only.".to_string()),
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{context::UserOptionsContext, localization::Localization, options::ProfileLevel};
    use std::path::PathBuf;

    fn make_user_options(profile: ProfileLevel, enable_argon: bool) -> UserOptionsContext {
        UserOptionsContext {
            profile,
            staging_dir: PathBuf::from("/tmp/mash-test-staging"),
            dry_run: true,
            interactive: false,
            enable_argon,
            enable_p10k: false,
            docker_data_root: false,
        }
    }

    fn load_localization() -> Localization {
        Localization::load_default().expect("unable to load localization strings")
    }

    #[test]
    fn registry_minimal_profile_only_core_phases() {
        let opts = make_user_options(ProfileLevel::Minimal, false);
        let strings = load_localization();
        let registry = PhaseRegistry::default();
        let phases = registry.build_phases(&opts, &strings);
        let names: Vec<_> = phases.iter().map(|p| p.name()).collect();
        assert_eq!(
            names,
            vec![
                "System packages",
                "Rust toolchain + cargo tools",
                "Git, GitHub CLI, SSH"
            ]
        );
    }

    #[test]
    fn registry_dev_profile_includes_dev_phases() {
        let opts = make_user_options(ProfileLevel::Dev, false);
        let strings = load_localization();
        let registry = PhaseRegistry::default();
        let phases = registry.build_phases(&opts, &strings);
        let names: Vec<_> = phases.iter().map(|p| p.name()).collect();
        let expected_phases = [
            "Buildroot dependencies",
            "Docker Engine",
            "Shell & UX (zsh, starship)",
            "Fonts",
            "rclone",
        ];

        for phase in expected_phases {
            assert!(
                names.contains(&phase),
                "expected phase list to include {} but got {:?}",
                phase,
                names
            );
        }
    }

    #[test]
    fn registry_argon_option_adds_phase() {
        let opts = make_user_options(ProfileLevel::Minimal, true);
        let strings = load_localization();
        let registry = PhaseRegistry::default();
        let phases = registry.build_phases(&opts, &strings);
        let names: Vec<_> = phases.iter().map(|p| p.name()).collect();
        assert!(names.contains(&"Argon One fan script"));
    }
}
