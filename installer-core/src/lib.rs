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
mod fonts;
mod github;
mod package_manager;
mod pkg;
mod platform;
mod rclone;
mod rust;
mod staging;
mod systemd;
mod zsh;

use anyhow::{Error, Result};
use std::path::PathBuf;
use tracing::{error, info};

pub use backend::PkgBackend;
use context::{ConfigService, OptionsContext, PlatformContext};
pub use driver::{AptRepoConfig, DistroDriver, RepoKind, ServiceName};
pub use platform::{detect as detect_platform, PlatformInfo};

/// Options provided by the CLI that drive `run_with_driver`.
#[derive(Clone)]
pub struct InstallOptions {
    pub profile: ProfileLevel,
    pub staging_dir: Option<PathBuf>,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
}

/// Central context threaded through every install phase.
pub struct InstallContext {
    pub options: OptionsContext,
    pub platform: PlatformContext,
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
) -> Result<RunSummary> {
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

    let config_service = ConfigService::load()?;
    let staging = staging::resolve(opts.staging_dir.as_deref(), config_service.config())?;
    info!("Staging directory: {}", staging.display());

    let options = OptionsContext {
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
    };

    let phases = build_phase_list(&ctx.options);
    let runner = PhaseRunner::from_phases(phases);
    let result = runner.run(&ctx, observer)?;

    Ok(RunSummary {
        completed_phases: result.completed_phases,
        staging_dir: ctx.options.staging_dir.clone(),
    })
}

fn build_phase_list(options: &OptionsContext) -> Vec<Box<dyn Phase>> {
    let mut phases: Vec<Box<dyn Phase>> = vec![
        Box::new(FunctionPhase::new(
            "System packages",
            "System packages installed",
            pkg::install_phase,
        )),
        Box::new(FunctionPhase::new(
            "Rust toolchain + cargo tools",
            "Rust toolchain ready",
            rust::install_phase,
        )),
        Box::new(FunctionPhase::new(
            "Git, GitHub CLI, SSH",
            "Git / GitHub CLI ready",
            github::install_phase,
        )),
    ];

    if options.profile >= ProfileLevel::Dev {
        phases.push(Box::new(FunctionPhase::new(
            "Buildroot dependencies",
            "Buildroot dependencies ready",
            buildroot::install_phase,
        )));
        phases.push(Box::new(FunctionPhase::new(
            "Docker Engine",
            "Docker Engine ready",
            docker::install_phase,
        )));
        phases.push(Box::new(FunctionPhase::new(
            "Shell & UX (zsh, starship)",
            "Shell & UX ready",
            zsh::install_phase,
        )));
        phases.push(Box::new(FunctionPhase::new(
            "Fonts",
            "Fonts installed",
            fonts::install_phase,
        )));
        phases.push(Box::new(FunctionPhase::new(
            "rclone",
            "rclone ready",
            rclone::install_phase,
        )));
    }

    if options.enable_argon {
        phases.push(Box::new(FunctionPhase::new(
            "Argon One fan script",
            "Argon One installed",
            argon::install_phase,
        )));
    }

    phases
}

pub trait PhaseObserver {
    fn total_phases(&mut self, _total: usize) {}
    fn on_phase_started(&mut self, _index: usize, _total: usize, _label: &'static str) {}
    fn on_phase_success(&mut self, _index: usize, _done_msg: &'static str) {}
    fn on_phase_failure(&mut self, _index: usize, _label: &'static str, _err: &Error) {}
    fn on_phase_skipped(&mut self, _index: usize, _label: &'static str) {}
}

#[derive(Debug)]
pub struct PhaseRunResult {
    pub completed_phases: Vec<&'static str>,
}

pub struct PhaseRunner {
    phases: Vec<Box<dyn Phase>>,
}

impl PhaseRunner {
    pub fn from_phases(phases: Vec<Box<dyn Phase>>) -> Self {
        Self { phases }
    }

    pub fn run(
        &self,
        ctx: &InstallContext,
        observer: &mut dyn PhaseObserver,
    ) -> Result<PhaseRunResult> {
        let total = self.phases.len();
        observer.total_phases(total);
        let mut completed = Vec::new();

        for (i, phase) in self.phases.iter().enumerate() {
            if !phase.should_run(&ctx.options) {
                observer.on_phase_skipped(i + 1, phase.label());
                continue;
            }

            observer.on_phase_started(i + 1, total, phase.label());
            match phase.execute(ctx) {
                Ok(()) => {
                    observer.on_phase_success(i + 1, phase.done_msg());
                    completed.push(phase.label());
                }
                Err(e) => {
                    observer.on_phase_failure(i + 1, phase.label(), &e);
                    let completed_list = if completed.is_empty() {
                        "none".to_string()
                    } else {
                        completed.join(", ")
                    };
                    error!(
                        "Installation aborted during {} (staging dir: {}). Completed phases: {}. \
                         Rerun `mash-setup doctor` or remove the staging directory before retrying.",
                        phase.label(),
                        ctx.options.staging_dir.display(),
                        completed_list
                    );
                    return Err(e);
                }
            }
        }

        Ok(PhaseRunResult {
            completed_phases: completed,
        })
    }
}

pub struct RunSummary {
    pub completed_phases: Vec<&'static str>,
    pub staging_dir: PathBuf,
}

pub trait Phase {
    fn label(&self) -> &'static str;
    fn done_msg(&self) -> &'static str;
    fn should_run(&self, _opts: &OptionsContext) -> bool {
        true
    }
    fn execute(&self, ctx: &InstallContext) -> Result<()>;
}

pub struct FunctionPhase {
    label: &'static str,
    done_msg: &'static str,
    run: fn(&InstallContext) -> Result<()>,
}

impl Phase for FunctionPhase {
    fn label(&self) -> &'static str {
        self.label
    }

    fn done_msg(&self) -> &'static str {
        self.done_msg
    }

    fn execute(&self, ctx: &InstallContext) -> Result<()> {
        (self.run)(ctx)
    }
}

impl FunctionPhase {
    pub fn new(
        label: &'static str,
        done_msg: &'static str,
        run: fn(&InstallContext) -> Result<()>,
    ) -> Self {
        Self {
            label,
            done_msg,
            run,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, Error, Result};
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
        fn total_phases(&mut self, total: usize) {
            self.total = Some(total);
            self.events.push(format!("total:{}", total));
        }

        fn on_phase_started(&mut self, index: usize, _: usize, label: &'static str) {
            self.events.push(format!("start:{}:{}", index, label));
        }

        fn on_phase_success(&mut self, index: usize, done_msg: &'static str) {
            self.events.push(format!("success:{}:{}", index, done_msg));
        }

        fn on_phase_failure(&mut self, index: usize, label: &'static str, err: &Error) {
            self.events
                .push(format!("failure:{}:{}:{}", index, label, err));
        }

        fn on_phase_skipped(&mut self, index: usize, label: &'static str) {
            self.events.push(format!("skipped:{}:{}", index, label));
        }
    }

    struct TestPhase {
        label: &'static str,
        done_msg: &'static str,
        should_run: bool,
        run: fn(&InstallContext) -> Result<()>,
    }

    impl Phase for TestPhase {
        fn label(&self) -> &'static str {
            self.label
        }

        fn done_msg(&self) -> &'static str {
            self.done_msg
        }

        fn should_run(&self, _: &OptionsContext) -> bool {
            self.should_run
        }

        fn execute(&self, ctx: &InstallContext) -> Result<()> {
            (self.run)(ctx)
        }
    }

    impl TestPhase {
        fn new(
            label: &'static str,
            done_msg: &'static str,
            should_run: bool,
            run: fn(&InstallContext) -> Result<()>,
        ) -> Self {
            Self {
                label,
                done_msg,
                should_run,
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
        let options = OptionsContext {
            profile: ProfileLevel::Minimal,
            staging_dir: PathBuf::from("/tmp/mash-test"),
            dry_run: true,
            interactive: false,
            enable_argon: false,
            enable_p10k: false,
            docker_data_root: false,
        };

        Ok(InstallContext {
            options,
            platform: platform_ctx,
        })
    }

    fn success_phase(_ctx: &InstallContext) -> Result<()> {
        Ok(())
    }

    fn failing_phase(_ctx: &InstallContext) -> Result<()> {
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
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-skip",
                "phase skip done",
                false,
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-two",
                "phase two done",
                true,
                success_phase,
            )),
        ];
        let runner = PhaseRunner::from_phases(phases);
        let mut observer = RecordingObserver::new();
        let result = runner.run(&ctx, &mut observer)?;

        assert_eq!(result.completed_phases, vec!["phase-one", "phase-two"]);
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
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-error",
                "phase error done",
                true,
                failing_phase,
            )),
            Box::new(TestPhase::new(
                "phase-three",
                "phase three done",
                true,
                success_phase,
            )),
        ];
        let runner = PhaseRunner::from_phases(phases);
        let mut observer = RecordingObserver::new();

        let err = runner.run(&ctx, &mut observer).unwrap_err();
        assert_eq!(err.to_string(), "boom");
        assert!(observer
            .events
            .iter()
            .any(|evt| evt.starts_with("failure:2:phase-error:boom")));
        assert!(observer
            .events
            .iter()
            .any(|evt| evt.starts_with("start:1:phase-one")));
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
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-skip",
                "phase skip done",
                false,
                success_phase,
            )),
            Box::new(TestPhase::new(
                "phase-two",
                "phase two done",
                true,
                success_phase,
            )),
        ];
        let runner = PhaseRunner::from_phases(phases);
        let mut observer = RecordingObserver::new();

        let result = runner.run(&ctx, &mut observer)?;
        assert_eq!(result.completed_phases, vec!["phase-one", "phase-two"]);
        assert!(observer
            .events
            .iter()
            .any(|evt| evt.starts_with("skipped:2:phase-skip")));
        Ok(())
    }
}
