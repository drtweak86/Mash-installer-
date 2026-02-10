use anyhow::{anyhow, Result};
use installer_core::localization::Localization;
use installer_core::RollbackManager;
use installer_core::{
    ConfigService, DistroDriver, ErrorSeverity, InstallContext, Phase, PhaseContext,
    PhaseErrorPolicy, PhaseEvent, PhaseObserver, PhaseRunner, PkgBackend, PlatformContext,
    PlatformInfo, ProfileLevel, UIContext, UserOptionsContext,
};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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

struct TestPhase {
    name: &'static str,
    description: &'static str,
    should_run: bool,
    severity: ErrorSeverity,
    runner: fn(&mut PhaseContext) -> Result<()>,
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

    fn error_severity(&self) -> ErrorSeverity {
        self.severity
    }

    fn execute(&self, ctx: &mut PhaseContext) -> Result<()> {
        (self.runner)(ctx)
    }
}

impl TestPhase {
    fn new(
        name: &'static str,
        description: &'static str,
        should_run: bool,
        severity: ErrorSeverity,
        runner: fn(&mut PhaseContext) -> Result<()>,
    ) -> Self {
        Self {
            name,
            description,
            should_run,
            severity,
            runner,
        }
    }
}

struct RecordingObserver {
    events: Vec<PhaseEvent>,
    total: Option<usize>,
}

impl RecordingObserver {
    fn new() -> Self {
        Self {
            events: Vec::new(),
            total: None,
        }
    }
}

impl PhaseObserver for RecordingObserver {
    fn on_event(&mut self, event: PhaseEvent) {
        match &event {
            PhaseEvent::Total { total } => self.total = Some(*total),
            _ => {}
        }
        self.events.push(event);
    }
}

fn build_install_context() -> Result<InstallContext> {
    let config_service = ConfigService::load()?;
    let platform = PlatformInfo {
        arch: "x86_64".into(),
        distro: "mash-test".into(),
        distro_version: "0".into(),
        distro_codename: "test".into(),
        distro_family: "debian".into(),
        pi_model: None,
    };
    let driver: &'static dyn DistroDriver = &DummyDriver;
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
        rollback: RollbackManager::new(),
    })
}

fn success_phase(_ctx: &mut PhaseContext) -> Result<()> {
    Ok(())
}

fn failing_phase(_ctx: &mut PhaseContext) -> Result<()> {
    Err(anyhow!("boom"))
}

#[test]
fn phase_runner_skips_phases_when_should_run_is_false() -> Result<()> {
    let ctx = build_install_context()?;
    let phases: Vec<Box<dyn Phase>> = vec![
        Box::new(TestPhase::new(
            "phase-one",
            "phase one",
            true,
            ErrorSeverity::Fatal,
            success_phase,
        )),
        Box::new(TestPhase::new(
            "phase-skip",
            "phase skip",
            false,
            ErrorSeverity::Fatal,
            success_phase,
        )),
    ];
    let runner = PhaseRunner::from_phases(phases);
    let mut observer = RecordingObserver::new();

    let result = runner.run(&ctx, &mut observer)?;

    assert_eq!(result.completed_phases, vec!["phase-one".to_string()]);
    assert!(observer
        .events
        .iter()
        .any(|event| matches!(event, PhaseEvent::Skipped { phase, .. } if phase == "phase-skip")));
    Ok(())
}

#[test]
fn phase_runner_aggregates_errors_and_events() -> Result<()> {
    let ctx = build_install_context()?;
    let phases: Vec<Box<dyn Phase>> = vec![
        Box::new(TestPhase::new(
            "phase-one",
            "phase one",
            true,
            ErrorSeverity::Fatal,
            success_phase,
        )),
        Box::new(TestPhase::new(
            "phase-error",
            "phase error",
            true,
            ErrorSeverity::Recoverable,
            failing_phase,
        )),
        Box::new(TestPhase::new(
            "phase-three",
            "phase three",
            true,
            ErrorSeverity::Fatal,
            success_phase,
        )),
    ];
    let mut observer = RecordingObserver::new();
    let runner = PhaseRunner::with_policy(phases, PhaseErrorPolicy::ContinueOnError);

    let result = runner.run(&ctx, &mut observer)?;

    assert_eq!(result.errors.len(), 1);
    assert!(result.events.iter().any(|event| matches!(
        event,
        PhaseEvent::Failed { phase, .. } if phase == "phase-error"
    )));
    Ok(())
}

#[test]
fn phase_runner_triggers_rollback_on_failure() -> Result<()> {
    let ctx = build_install_context()?;
    let executed = Arc::new(Mutex::new(Vec::new()));
    let marker = executed.clone();
    ctx.rollback.register_action("cleanup", move || {
        marker.lock().unwrap().push("cleanup".to_string());
        Ok(())
    });

    let phases: Vec<Box<dyn Phase>> = vec![Box::new(TestPhase::new(
        "phase-error",
        "phase error",
        true,
        ErrorSeverity::Fatal,
        failing_phase,
    ))];
    let runner = PhaseRunner::from_phases(phases);
    let mut observer = RecordingObserver::new();
    assert!(runner.run(&ctx, &mut observer).is_err());

    let history = executed.lock().unwrap();
    assert_eq!(history.as_slice(), ["cleanup"]);
    Ok(())
}
