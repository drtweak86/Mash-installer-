mod apt_repo;
mod argon;
mod backend;
mod buildroot;
pub mod cmd;
mod config;
mod context;
mod distro;
mod docker;
mod doctor;
mod driver;
mod error;
mod fonts;
mod github;
pub mod localization;
mod package_manager;
mod pkg;
mod platform;
mod rclone;
mod rust;
mod staging;
mod system;
mod systemd;
mod zsh;

use crate::localization::Localization;
use anyhow::Result;
use std::{fmt, path::PathBuf};
use tracing::{error, info};

pub use backend::PkgBackend;
pub use context::{
    ConfigOverrides, ConfigService, PhaseContext, PlatformContext, UIContext, UserOptionsContext,
};
pub use doctor::DoctorOutput;
pub use driver::{AptRepoConfig, DistroDriver, RepoKind, ServiceName};
pub use error::{
    DriverInfo, ErrorSeverity, InstallationReport, InstallerError, InstallerRunError,
    InstallerStateSnapshot, RunSummary,
};
pub use platform::{detect as detect_platform, PlatformInfo};
pub use system::{RealSystem, SystemOps};

/// Options provided by the CLI that drive `run_with_driver`.
#[derive(Clone, Debug)]
pub struct InstallOptions {
    pub profile: ProfileLevel,
    pub staging_dir: Option<PathBuf>,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
    pub continue_on_error: bool,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            profile: ProfileLevel::Minimal,
            staging_dir: None,
            dry_run: false,
            interactive: false,
            enable_argon: false,
            enable_p10k: false,
            docker_data_root: false,
            continue_on_error: false,
        }
    }
}

/// Central context threaded through every install phase.
pub struct InstallContext {
    pub options: UserOptionsContext,
    pub platform: PlatformContext,
    pub ui: UIContext,
    pub localization: Localization,
}

impl InstallContext {
    fn phase_context(&self) -> PhaseContext<'_> {
        PhaseContext {
            options: &self.options,
            platform: &self.platform,
            ui: &self.ui,
            localization: &self.localization,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProfileLevel {
    Minimal = 0,
    Dev = 1,
    Full = 2,
}

/// Run the installer using the supplied distro driver and CLI options.
pub fn run_with_driver(
    driver: &'static dyn DistroDriver,
    opts: InstallOptions,
    observer: &mut dyn PhaseObserver,
) -> Result<InstallationReport, InstallerRunError> {
    let plat = platform::detect()?;
    info!(
        "Platform: {} {} on {}",
        plat.distro, plat.distro_version, plat.arch
    );
    info!(
        "Using distro driver: {} ({})",
        driver.name(),
        driver.description()
    );
    if let Some(ref model) = plat.pi_model {
        info!("Raspberry Pi model: {}", model);
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
        ui: UIContext::default(),
        localization,
    };

    let phases = build_phase_list(&ctx.options, &ctx.localization);
    let policy = if opts.continue_on_error {
        PhaseErrorPolicy::ContinueOnError
    } else {
        PhaseErrorPolicy::default()
    };
    let runner = PhaseRunner::with_policy(phases, policy);
    match runner.run(&ctx, observer) {
        Ok(result) => Ok(InstallationReport {
            summary: RunSummary {
                completed_phases: result.completed_phases,
                staging_dir: ctx.options.staging_dir.clone(),
                errors: result.errors,
            },
            events: result.events,
            options: api_options.clone(),
            driver: driver_info.clone(),
        }),
        Err(err) => {
            let PhaseRunError {
                result: run_result,
                source,
            } = err;
            let report = InstallationReport {
                summary: RunSummary {
                    completed_phases: run_result.completed_phases,
                    staging_dir: ctx.options.staging_dir.clone(),
                    errors: run_result.errors,
                },
                events: run_result.events,
                options: api_options,
                driver: driver_info,
            };
            Err(InstallerRunError { report, source })
        }
    }
}

fn build_phase_list(options: &UserOptionsContext, strings: &Localization) -> Vec<Box<dyn Phase>> {
    let mut phases: Vec<Box<dyn Phase>> = vec![
        localized_phase(
            strings,
            "system_packages",
            "System packages",
            "System packages installed",
            pkg::install_phase,
        ),
        localized_phase(
            strings,
            "rust_toolchain",
            "Rust toolchain + cargo tools",
            "Rust toolchain ready",
            rust::install_phase,
        ),
        localized_phase(
            strings,
            "git_cli",
            "Git, GitHub CLI, SSH",
            "Git / GitHub CLI ready",
            github::install_phase,
        ),
    ];

    if options.profile >= ProfileLevel::Dev {
        phases.push(localized_phase(
            strings,
            "buildroot_dependencies",
            "Buildroot dependencies",
            "Buildroot dependencies ready",
            buildroot::install_phase,
        ));
        phases.push(localized_phase(
            strings,
            "docker_engine",
            "Docker Engine",
            "Docker Engine ready",
            docker::install_phase,
        ));
        phases.push(localized_phase(
            strings,
            "shell_ux",
            "Shell & UX (zsh, starship)",
            "Shell & UX ready",
            zsh::install_phase,
        ));
        phases.push(localized_phase(
            strings,
            "fonts",
            "Fonts",
            "Fonts installed",
            fonts::install_phase,
        ));
        phases.push(localized_phase(
            strings,
            "rclone",
            "rclone",
            "rclone ready",
            rclone::install_phase,
        ));
    }

    if options.enable_argon {
        phases.push(localized_phase(
            strings,
            "argon_one",
            "Argon One fan script",
            "Argon One installed",
            argon::install_phase,
        ));
    }

    phases
}

fn localized_phase(
    strings: &Localization,
    key: &str,
    label: &'static str,
    description: &'static str,
    run: fn(&mut PhaseContext) -> Result<()>,
) -> Box<dyn Phase> {
    let entry = strings.phase_or_default(key, label, description);
    Box::new(FunctionPhase::new(entry.label, entry.description, run))
}

#[derive(Debug)]
pub struct PhaseRunResult {
    pub completed_phases: Vec<String>,
    pub events: Vec<PhaseEvent>,
    pub errors: Vec<InstallerError>,
}

#[derive(Debug)]
pub struct PhaseRunError {
    pub result: PhaseRunResult,
    pub source: InstallerError,
}

impl fmt::Display for PhaseRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl std::error::Error for PhaseRunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhaseErrorPolicy {
    FailFast,
    ContinueOnError,
}

impl Default for PhaseErrorPolicy {
    fn default() -> Self {
        PhaseErrorPolicy::FailFast
    }
}

pub struct PhaseRunner {
    phases: Vec<Box<dyn Phase>>,
    policy: PhaseErrorPolicy,
}

impl PhaseRunner {
    pub fn from_phases(phases: Vec<Box<dyn Phase>>) -> Self {
        Self::with_policy(phases, PhaseErrorPolicy::default())
    }

    pub fn with_policy(phases: Vec<Box<dyn Phase>>, policy: PhaseErrorPolicy) -> Self {
        Self { phases, policy }
    }

    pub fn run(
        &self,
        ctx: &InstallContext,
        observer: &mut dyn PhaseObserver,
    ) -> Result<PhaseRunResult, PhaseRunError> {
        let total = self.phases.len();
        fn emit_event(
            observer: &mut dyn PhaseObserver,
            events: &mut Vec<PhaseEvent>,
            event: PhaseEvent,
        ) {
            observer.on_event(event.clone());
            events.push(event);
        }

        let mut events = Vec::new();
        emit_event(observer, &mut events, PhaseEvent::Total { total });
        let mut completed = Vec::new();
        let mut errors = Vec::new();

        for (i, phase) in self.phases.iter().enumerate() {
            if !phase.should_run(ctx) {
                emit_event(
                    observer,
                    &mut events,
                    PhaseEvent::Skipped {
                        index: i + 1,
                        phase: phase.name().to_string(),
                    },
                );
                continue;
            }

            emit_event(
                observer,
                &mut events,
                PhaseEvent::Started {
                    index: i + 1,
                    total,
                    phase: phase.name().to_string(),
                },
            );
            let mut phase_ctx = ctx.phase_context();
            match phase.execute(&mut phase_ctx) {
                Ok(()) => {
                    emit_event(
                        observer,
                        &mut events,
                        PhaseEvent::Completed {
                            index: i + 1,
                            phase: phase.name().to_string(),
                            description: phase.description().to_string(),
                        },
                    );
                    completed.push(phase.name().to_string());
                }
                Err(e) => {
                    let severity = phase.error_severity();
                    let installer_error = InstallerError::new(
                        phase.name(),
                        phase.description(),
                        severity,
                        e,
                        InstallerStateSnapshot::from_options(&ctx.options),
                        Some(
                            "Rerun `mash-setup doctor` or remove the staging directory before retrying."
                                .to_string(),
                        ),
                    );
                    let error_message = installer_error.message.clone();
                    emit_event(
                        observer,
                        &mut events,
                        PhaseEvent::Failed {
                            index: i + 1,
                            phase: phase.name().to_string(),
                            error: error_message.clone(),
                        },
                    );
                    errors.push(installer_error.clone());
                    let completed_list = if completed.is_empty() {
                        "none".to_string()
                    } else {
                        completed.join(", ")
                    };
                    error!(
                        "Installation aborted during {} (staging dir: {}). Completed phases: {}. \
                         Rerun `mash-setup doctor` or remove the staging directory before retrying.",
                    phase.name(),
                        ctx.options.staging_dir.display(),
                        completed_list
                    );
                    let should_continue = matches!(self.policy, PhaseErrorPolicy::ContinueOnError)
                        && severity == ErrorSeverity::Recoverable;

                    if should_continue {
                        continue;
                    }

                    let run_result = PhaseRunResult {
                        completed_phases: completed,
                        events,
                        errors,
                    };

                    return Err(PhaseRunError {
                        result: run_result,
                        source: installer_error,
                    });
                }
            }
        }

        Ok(PhaseRunResult {
            completed_phases: completed,
            events,
            errors,
        })
    }
}

#[derive(Clone, Debug)]
pub enum PhaseEvent {
    Total {
        total: usize,
    },
    Started {
        index: usize,
        total: usize,
        phase: String,
    },
    Completed {
        index: usize,
        phase: String,
        description: String,
    },
    Failed {
        index: usize,
        phase: String,
        error: String,
    },
    Skipped {
        index: usize,
        phase: String,
    },
}

pub trait PhaseObserver {
    fn on_event(&mut self, _event: PhaseEvent) {}
}

pub trait Phase {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn should_run(&self, _ctx: &InstallContext) -> bool {
        true
    }
    fn error_severity(&self) -> ErrorSeverity {
        ErrorSeverity::Fatal
    }
    fn execute(&self, ctx: &mut PhaseContext) -> Result<()>;
}

pub struct FunctionPhase {
    name: String,
    description: String,
    run: fn(&mut PhaseContext) -> Result<()>,
}

impl Phase for FunctionPhase {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn execute(&self, ctx: &mut PhaseContext) -> Result<()> {
        (self.run)(ctx)
    }
}

impl FunctionPhase {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        run: fn(&mut PhaseContext) -> Result<()>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            run,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, Result};
    use std::path::PathBuf;

    struct RecordingObserver {
        total: Option<usize>,
        events: Vec<String>,
    }

    impl RecordingObserver {
        fn new() -> Self {
            Self {
                total: None,
                events: Vec::new(),
            }
        }
    }

    impl PhaseObserver for RecordingObserver {
        fn on_event(&mut self, event: PhaseEvent) {
            match event {
                PhaseEvent::Total { total } => {
                    self.total = Some(total);
                    self.events.push(format!("total:{}", total));
                }
                PhaseEvent::Started { index, phase, .. } => {
                    self.events.push(format!("start:{}:{}", index, phase));
                }
                PhaseEvent::Completed {
                    index, description, ..
                } => {
                    self.events
                        .push(format!("success:{}:{}", index, description));
                }
                PhaseEvent::Failed {
                    index,
                    phase,
                    error,
                } => {
                    self.events
                        .push(format!("failure:{}:{}:{}", index, phase, error));
                }
                PhaseEvent::Skipped { index, phase } => {
                    self.events.push(format!("skipped:{}:{}", index, phase));
                }
            }
        }
    }

    struct TestPhase {
        name: &'static str,
        description: &'static str,
        should_run: bool,
        severity: ErrorSeverity,
        run: fn(&mut PhaseContext) -> Result<()>,
    }

    impl Phase for TestPhase {
        fn name(&self) -> &'static str {
            self.name
        }

        fn description(&self) -> &'static str {
            self.description
        }

        fn should_run(&self, _: &InstallContext) -> bool {
            self.should_run
        }

        fn execute(&self, ctx: &mut PhaseContext) -> Result<()> {
            (self.run)(ctx)
        }

        fn error_severity(&self) -> ErrorSeverity {
            self.severity
        }
    }

    impl TestPhase {
        fn new(
            name: &'static str,
            description: &'static str,
            should_run: bool,
            severity: ErrorSeverity,
            run: fn(&mut PhaseContext) -> Result<()>,
        ) -> Self {
            Self {
                name,
                description,
                should_run,
                severity,
                run,
            }
        }
    }

    struct DummyDriver;

    impl DistroDriver for DummyDriver {
        fn name(&self) -> &'static str {
            "dummy"
        }

        fn description(&self) -> &'static str {
            "dummy driver for tests"
        }

        fn matches(&self, _: &PlatformInfo) -> bool {
            true
        }

        fn pkg_backend(&self) -> PkgBackend {
            PkgBackend::Apt
        }
    }

    static TEST_DRIVER: DummyDriver = DummyDriver;

    fn build_test_context() -> Result<InstallContext> {
        let config_service = ConfigService::load()?;
        let platform = PlatformInfo {
            arch: "x86_64".into(),
            distro: "mash-test".into(),
            distro_version: "0".into(),
            distro_codename: "test".into(),
            distro_family: "debian".into(),
            pi_model: None,
        };
        let driver: &'static dyn DistroDriver = &TEST_DRIVER;
        let platform_ctx = PlatformContext {
            config_service,
            platform,
            driver_name: "dummy",
            driver,
            pkg_backend: PkgBackend::Apt,
        };
        let options = UserOptionsContext {
            profile: ProfileLevel::Minimal,
            staging_dir: PathBuf::from("/tmp/mash-test"),
            dry_run: true,
            interactive: false,
            enable_argon: false,
            enable_p10k: false,
            docker_data_root: false,
        };
        let localization = Localization::load_default()?;

        Ok(InstallContext {
            options,
            platform: platform_ctx,
            ui: UIContext::default(),
            localization,
        })
    }

    fn success_phase(_ctx: &mut PhaseContext) -> Result<()> {
        Ok(())
    }

    fn failing_phase(_ctx: &mut PhaseContext) -> Result<()> {
        Err(anyhow!("boom"))
    }

    #[test]
    fn phase_runner_notifies_observer_and_records_success() -> Result<()> {
        let ctx = build_test_context()?;
        let phases: Vec<Box<dyn Phase>> = vec![
            Box::new(TestPhase::new(
                "phase-one",
                "phase one done",
                true,
                ErrorSeverity::Fatal,
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-skip",
                "phase skip done",
                false,
                ErrorSeverity::Fatal,
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-two",
                "phase two done",
                true,
                ErrorSeverity::Fatal,
                success_phase,
            )),
        ];
        let runner = PhaseRunner::from_phases(phases);
        let mut observer = RecordingObserver::new();
        let result = runner.run(&ctx, &mut observer)?;

        assert_eq!(
            result.completed_phases,
            vec!["phase-one".to_string(), "phase-two".to_string()]
        );
        assert_eq!(observer.total, Some(3));
        assert!(observer
            .events
            .iter()
            .any(|evt| evt.starts_with("start:1:phase-one")));
        assert!(observer
            .events
            .iter()
            .any(|evt| evt.starts_with("success:3:phase two done")));
        assert!(observer
            .events
            .iter()
            .any(|evt| evt.starts_with("skipped:2:phase-skip")));
        Ok(())
    }

    #[test]
    fn phase_runner_stops_on_error() -> Result<()> {
        let ctx = build_test_context()?;
        let phases: Vec<Box<dyn Phase>> = vec![
            Box::new(TestPhase::new(
                "phase-one",
                "phase one done",
                true,
                ErrorSeverity::Fatal,
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-error",
                "phase error done",
                true,
                ErrorSeverity::Fatal,
                failing_phase,
            )),
            Box::new(TestPhase::new(
                "phase-three",
                "phase three done",
                true,
                ErrorSeverity::Fatal,
                success_phase,
            )),
        ];
        let runner = PhaseRunner::from_phases(phases);
        let mut observer = RecordingObserver::new();

        let err = runner.run(&ctx, &mut observer).unwrap_err();
        assert_eq!(err.source.phase, "phase-error");
        assert_eq!(err.source.user_message(), "phase-error failed: boom");
        assert_eq!(err.result.errors.len(), 1);
        assert!(observer
            .events
            .iter()
            .any(|evt| evt.starts_with("failure:2:phase-error:phase-error failed: boom")));
        assert!(observer
            .events
            .iter()
            .any(|evt| evt.starts_with("start:1:phase-one")));
        Ok(())
    }

    #[test]
    fn phase_runner_continues_on_recoverable_errors() -> Result<()> {
        let ctx = build_test_context()?;
        let phases: Vec<Box<dyn Phase>> = vec![
            Box::new(TestPhase::new(
                "phase-one",
                "phase one done",
                true,
                ErrorSeverity::Fatal,
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-error",
                "phase error done",
                true,
                ErrorSeverity::Recoverable,
                failing_phase,
            )),
            Box::new(TestPhase::new(
                "phase-three",
                "phase three done",
                true,
                ErrorSeverity::Fatal,
                success_phase,
            )),
        ];
        let mut observer = RecordingObserver::new();
        let runner = PhaseRunner::with_policy(phases, PhaseErrorPolicy::ContinueOnError);

        let result = runner.run(&ctx, &mut observer)?;
        assert_eq!(
            result.completed_phases,
            vec!["phase-one".to_string(), "phase-three".to_string()]
        );
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].severity, ErrorSeverity::Recoverable);
        assert!(observer
            .events
            .iter()
            .any(|evt| evt.starts_with("failure:2:phase-error:phase-error failed: boom")));
        Ok(())
    }

    #[test]
    fn phase_runner_reports_skipped_phases() -> Result<()> {
        let ctx = build_test_context()?;
        let phases: Vec<Box<dyn Phase>> = vec![
            Box::new(TestPhase::new(
                "phase-one",
                "phase one done",
                true,
                ErrorSeverity::Fatal,
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-skip",
                "phase skip done",
                false,
                ErrorSeverity::Fatal,
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-two",
                "phase two done",
                true,
                ErrorSeverity::Fatal,
                success_phase,
            )),
        ];
        let runner = PhaseRunner::from_phases(phases);
        let mut observer = RecordingObserver::new();

        let result = runner.run(&ctx, &mut observer)?;
        assert_eq!(
            result.completed_phases,
            vec!["phase-one".to_string(), "phase-two".to_string()]
        );
        assert!(observer
            .events
            .iter()
            .any(|evt| evt.starts_with("skipped:2:phase-skip")));
        Ok(())
    }

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
    fn build_phase_list_minimal_profile_only_core_phases() {
        let opts = make_user_options(ProfileLevel::Minimal, false);
        let strings = load_localization();
        let phases = build_phase_list(&opts, &strings);
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
    fn build_phase_list_dev_profile_includes_dev_phases() {
        let opts = make_user_options(ProfileLevel::Dev, false);
        let strings = load_localization();
        let phases = build_phase_list(&opts, &strings);
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
    fn build_phase_list_with_argon_option_adds_argon_phase() {
        let opts = make_user_options(ProfileLevel::Minimal, true);
        let strings = load_localization();
        let phases = build_phase_list(&opts, &strings);
        let names: Vec<_> = phases.iter().map(|p| p.name()).collect();
        assert!(names.contains(&"Argon One fan script"));
    }
}
