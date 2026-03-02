mod ai_agents;
mod apt_repo;
mod argon;
mod backend;
mod buildroot;
pub mod catalog;
pub mod cmd;
mod config;
mod context;
pub mod desktop;
mod distro;
mod docker;
mod doctor;
mod driver;
pub mod dry_run;
mod error;
pub mod fonts;
mod github;
pub mod interaction;
pub mod localization;
mod lockfile;
mod logging;
mod options;
mod orchestrator;
mod package_manager;
mod package_spec;
mod phase_registry;
mod phase_runner;
pub mod phases;
pub mod pi4b;
pub mod pi_overlord;
mod pkg;
pub mod platform;
mod rclone;
mod rollback;
mod rust;
mod signal;
mod snapshots;
mod software_tiers;
mod staging;
mod status;
mod sudo;
mod sudo_password;
mod system;
mod systemd;
pub mod theme;
pub mod verify;
mod wallpaper;
mod zsh;

use crate::{dry_run::DryRunLog, localization::Localization};

// --- Core API ---
pub use backend::PkgBackend;
pub use config::{init_config, show_config, ConfigError, MashConfig};
pub use context::{
    ConfigOverrides, ConfigService, PhaseContext, PlatformContext, UIContext, UserOptionsContext,
};
pub use doctor::{run_doctor, DoctorOutput};
pub use driver::{AptRepoConfig, DistroDriver, RepoKind, ServiceName};
pub use error::{
    DriverInfo, ErrorSeverity, InstallationReport, InstallerError, InstallerRunError,
    InstallerStateSnapshot,
};
pub use logging::init as init_logging;
pub use options::{InstallOptions, ProfileLevel};
pub use orchestrator::run_with_driver;
pub use package_spec::{PackageIntent, PackageSpec};
pub use phase_registry::PhaseRegistry;
pub use phase_runner::{
    Phase, PhaseErrorPolicy, PhaseEvent, PhaseObserver, PhaseOutput, PhaseResult, PhaseRunError,
    PhaseRunResult, PhaseRunner,
};
pub use pi_overlord::{PackageCategory, PackageMapping, PiOverlord};
pub use platform::{detect as detect_platform, PlatformInfo};
pub use rollback::RollbackManager;
pub use software_tiers::SoftwareTierPlan;
pub use software_tiers::ThemePlan;
pub use status::{run_status, StatusOutput};
pub use system::{SystemOps, REAL_SYSTEM};
pub use wallpaper::{download_wallpapers, WallpaperConfig, WallpaperError};

/// Trait for validating configuration and options.
pub trait Validator {
    /// Perform validation and return a list of error messages.
    /// An empty vector indicates a valid configuration.
    fn validate(&self) -> Vec<String>;

    /// Return true if the configuration is valid.
    fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

/// Central context threaded through every install phase.
pub struct InstallContext {
    pub options: UserOptionsContext,
    pub platform: PlatformContext,
    pub ui: UIContext,
    pub interaction: interaction::InteractionService,
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
            &self.interaction,
            &self.localization,
            &self.rollback,
            &self.dry_run_log,
        )
    }

    /// Request a sudo password from the user via the provided observer and interaction service.
    pub fn request_sudo_password(
        &self,
        observer: &mut dyn PhaseObserver,
    ) -> anyhow::Result<String> {
        self.interaction
            .sudo_password(|_prompt| observer.sudo_password())
    }
}
// 1984 transition verified
