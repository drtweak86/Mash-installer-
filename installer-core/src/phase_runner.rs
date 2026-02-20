use std::fmt;

use anyhow::Result as AnyhowResult;
use tracing::{error, info};

use crate::{
    context::{PhaseContext, PhaseMetadata},
    error::{ErrorSeverity, InstallerError, InstallerStateSnapshot},
    logging, InstallContext,
};

#[derive(Debug)]
pub struct PhaseRunResult {
    pub completed_phases: Vec<String>,
    pub outputs: Vec<PhaseOutput>,
    pub events: Vec<PhaseEvent>,
    pub errors: Vec<InstallerError>,
}

#[derive(Clone, Debug)]
pub struct PhaseOutput {
    pub name: String,
    pub description: String,
    pub actions_taken: Vec<String>,
    pub rollback_actions: Vec<String>,
    pub warnings: Vec<String>,
    pub dry_run: bool,
    pub status: PhaseStatus,
}

impl PhaseOutput {
    pub fn from_metadata(
        name: impl Into<String>,
        description: impl Into<String>,
        metadata: PhaseMetadata,
        status: PhaseStatus,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            actions_taken: metadata.actions_taken,
            rollback_actions: metadata.rollback_actions,
            warnings: metadata.warnings,
            dry_run: metadata.dry_run,
            status,
        }
    }

    pub fn skipped(name: impl Into<String>, description: impl Into<String>, dry_run: bool) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            actions_taken: Vec::new(),
            rollback_actions: Vec::new(),
            warnings: Vec::new(),
            dry_run,
            status: PhaseStatus::Skipped,
        }
    }
}

#[derive(Clone, Debug)]
pub enum PhaseStatus {
    Completed,
    Failed(String),
    Skipped,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PhaseErrorPolicy {
    #[default]
    FailFast,
    ContinueOnError,
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
    ) -> Result<PhaseRunResult, Box<PhaseRunError>> {
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
        let mut outputs = Vec::new();

        for (i, phase) in self.phases.iter().enumerate() {
            let phase_name = phase.name().to_string();
            let phase_description = phase.description().to_string();

            if !phase.should_run(ctx) {
                emit_event(
                    observer,
                    &mut events,
                    PhaseEvent::Skipped {
                        index: i + 1,
                        phase: phase_name.clone(),
                    },
                );
                outputs.push(PhaseOutput::skipped(
                    phase_name.clone(),
                    phase_description.clone(),
                    ctx.options.dry_run,
                ));
                continue;
            }

            emit_event(
                observer,
                &mut events,
                PhaseEvent::Started {
                    index: i + 1,
                    total,
                    phase: phase_name.clone(),
                },
            );
            let phase_span = logging::phase_span(ctx, phase.as_ref());
            let _phase_guard = phase_span.enter();
            let mut phase_ctx = ctx.phase_context();
            let phase_result = phase_ctx.run_or_record(
                phase_name.clone(),
                "Phase simulated",
                Some(phase_description.clone()),
                |phase_ctx| phase.execute(phase_ctx),
            );
            let metadata = phase_ctx.take_metadata();
            let status = match &phase_result {
                Ok(()) => PhaseStatus::Completed,
                Err(err) => PhaseStatus::Failed(err.to_string()),
            };
            outputs.push(PhaseOutput::from_metadata(
                phase_name.clone(),
                phase_description.clone(),
                metadata,
                status.clone(),
            ));
            match phase_result {
                Ok(()) => {
                    emit_event(
                        observer,
                        &mut events,
                        PhaseEvent::Completed {
                            index: i + 1,
                            phase: phase_name.clone(),
                            description: phase_description.clone(),
                        },
                    );
                    completed.push(phase_name.clone());
                }
                Err(e) => {
                    let severity = phase.error_severity();
                    let installer_error = InstallerError::new(
                        phase_name.clone(),
                        phase_description.clone(),
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
                            phase: phase_name.clone(),
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
                        phase_name,
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
                        outputs,
                        events,
                        errors,
                    };

                    if let Err(rb_err) = ctx.rollback.rollback_all() {
                        error!("rollback encountered errors: {rb_err}");
                    } else {
                        info!("rollback completed after failure");
                    }

                    return Err(Box::new(PhaseRunError {
                        result: run_result,
                        source: installer_error,
                    }));
                }
            }
        }

        Ok(PhaseRunResult {
            completed_phases: completed,
            outputs,
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
    Warning {
        message: String,
    },
}

pub trait PhaseObserver {
    fn on_event(&mut self, _event: PhaseEvent) {}

    /// Ask the user for confirmation. Returns `true` to proceed, `false` to abort.
    /// Default implementation always proceeds.
    fn confirm(&mut self, _prompt: &str) -> bool {
        true
    }
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
    fn execute(&self, ctx: &mut PhaseContext) -> AnyhowResult<()>;
}

pub struct FunctionPhase {
    name: String,
    description: String,
    run: fn(&mut PhaseContext) -> AnyhowResult<()>,
}

impl Phase for FunctionPhase {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn execute(&self, ctx: &mut PhaseContext) -> AnyhowResult<()> {
        (self.run)(ctx)
    }
}

impl FunctionPhase {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        run: fn(&mut PhaseContext) -> AnyhowResult<()>,
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
    use crate::{
        backend::PkgBackend,
        context::{ConfigService, PhaseContext, PlatformContext, UIContext, UserOptionsContext},
        driver::DistroDriver,
        dry_run::DryRunLog,
        localization::Localization,
        platform::PlatformInfo,
        rollback::RollbackManager,
        InstallContext, ProfileLevel,
    };
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
                PhaseEvent::Warning { message } => {
                    self.events.push(format!("warning:{}", message));
                }
            }
        }
    }

    struct TestPhase {
        name: &'static str,
        description: &'static str,
        should_run: bool,
        severity: ErrorSeverity,
        run: fn(&mut PhaseContext) -> AnyhowResult<()>,
    }

    impl Phase for TestPhase {
        fn name(&self) -> &str {
            self.name
        }

        fn description(&self) -> &str {
            self.description
        }

        fn should_run(&self, _: &InstallContext) -> bool {
            self.should_run
        }

        fn execute(&self, ctx: &mut PhaseContext) -> AnyhowResult<()> {
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
            run: fn(&mut PhaseContext) -> AnyhowResult<()>,
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
            pkg_backend: driver.pkg_backend(),
        };
        let options = UserOptionsContext {
            profile: ProfileLevel::Minimal,
            staging_dir: PathBuf::from("/tmp/mash-test"),
            dry_run: false,
            interactive: false,
            enable_argon: false,
            enable_p10k: false,
            docker_data_root: false,
        };
        let localization = Localization::load_default()?;

        Ok(InstallContext {
            options,
            platform: platform_ctx,
            ui: UIContext,
            localization,
            rollback: RollbackManager::new(),
            dry_run_log: DryRunLog::new(),
        })
    }

    fn success_phase(_ctx: &mut PhaseContext) -> AnyhowResult<()> {
        Ok(())
    }

    fn failing_phase(_ctx: &mut PhaseContext) -> AnyhowResult<()> {
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
}
