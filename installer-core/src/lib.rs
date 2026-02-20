mod apt_repo;
mod argon;
mod backend;
mod buildroot;
pub mod catalog;
pub mod cmd;
mod config;
mod context;
mod distro;
mod docker;
mod doctor;
mod driver;
pub mod dry_run;
mod error;
mod fonts;
mod github;
mod hyprland;
pub mod interaction;
pub mod localization;
mod logging;
mod options;
mod orchestrator;
mod package_manager;
mod package_spec;
mod phase_registry;
mod phase_runner;
mod pkg;
mod platform;
mod rclone;
mod registry;
mod rollback;
mod runner;
mod rust;
mod staging;
mod sudo;
mod system;
mod systemd;
mod zsh;

use crate::{dry_run::DryRunLog, localization::Localization};

pub use backend::PkgBackend;
pub use config::{ConfigError, MashConfig};
pub use context::{
    ConfigOverrides, ConfigService, PhaseContext, PlatformContext, UIContext, UserOptionsContext,
};
pub use doctor::DoctorOutput;
pub use driver::{AptRepoConfig, DistroDriver, RepoKind, ServiceName};
pub use error::{
    DriverInfo, ErrorSeverity, InstallationReport, InstallerError, InstallerRunError,
    InstallerStateSnapshot,
};
pub use logging::init as init_logging;
pub use options::{InstallOptions, ProfileLevel};
pub use orchestrator::run_with_driver;
pub use package_spec::{PackageIntent, PackageSpec};
pub use platform::{detect as detect_platform, PlatformInfo};
pub use registry::PhaseRegistry;
pub use rollback::RollbackManager;
pub use runner::{
    Phase, PhaseErrorPolicy, PhaseEvent, PhaseObserver, PhaseOutput, PhaseRunError, PhaseRunResult,
    PhaseRunner,
};
pub use system::SystemOps;

/// Central context threaded through every install phase.
pub struct InstallContext {
    pub options: UserOptionsContext,
    pub platform: PlatformContext,
    pub ui: UIContext,
    pub localization: Localization,
    pub rollback: RollbackManager,
    pub dry_run_log: DryRunLog,
}

impl InstallContext {
    fn phase_context(&self) -> PhaseContext<'_> {
        PhaseContext::new(
            &self.options,
            &self.platform,
            &self.ui,
            &self.localization,
            &self.rollback,
            &self.dry_run_log,
        )
    }
}
