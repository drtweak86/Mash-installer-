mod model;
mod system;
mod wallpaper;

pub mod advice;
mod ai_agents;
mod apt_repo;
mod argon;
pub mod authorization;
mod backend;
mod buildroot;
pub mod catalog;

mod config;
mod context;
mod dependency_graph;
pub mod desktop;
mod distro;
mod docker;
mod doctor;
pub mod dotfiles;
mod driver;
pub use crate::system::error;

pub mod fonts;
mod github;
pub mod interaction;
pub mod localization;
pub mod logging;

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
pub mod preset;
pub mod profile;
mod rclone;
mod rollback;
mod rust;
pub mod scrubber;
mod signal;
mod snapshots;
mod software_tiers;
mod staging;
mod status;

mod systemd;
pub mod theme;
pub mod verify;
mod zsh;

use crate::localization::Localization;
pub use advice::{AdviceEngine, AdviceEntry, Rule, Severity as AdviceSeverity};
pub use system::{cmd, dry_run, logging as sys_logging, sudo, system_ops as sys_ops};

// --- Core API ---
pub use authorization::AuthorizationService;
pub use backend::PkgBackend;
pub use config::{init_config, show_config, ConfigError, MashConfig};
pub use context::{
    ConfigOverrides, ConfigService, PhaseContext, PlatformContext, UIContext, UserOptionsContext,
};
pub use doctor::{run_doctor, DoctorOutput};
pub use driver::{AptRepoConfig, DistroDriver, RepoKind, ServiceName};
pub use model::phase::AuthType;
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
pub use profile::{
    BlockDevice, CpuInfo, DistroInfo, MemoryInfo, MountInfo, PlatformInfo as ProfilePlatformInfo,
    PlatformType, SessionInfo, StorageInfo, SystemProfile, SystemProfileExt,
};
pub use rollback::RollbackManager;
pub use software_tiers::SoftwareTierPlan;
pub use software_tiers::ThemePlan;
pub use status::{run_status, StatusOutput};
pub use system::error::{
    DriverInfo, ErrorSeverity, InstallationReport, InstallerError, InstallerRunError,
    InstallerStateSnapshot,
};
pub use system::logging::init as init_logging;
pub use system::proc;
pub use system::system_ops::{SystemOps, REAL_SYSTEM};
pub use wallpaper::{
    download_wallpapers, harvest_wallpapers, HarvestConfig, WallpaperConfig, WallpaperError,
};

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
    pub dry_run_log: system::dry_run::DryRunLog,
}

impl InstallContext {
    fn phase_context<'a>(&'a self, observer: &'a mut dyn PhaseObserver) -> PhaseContext<'a> {
        PhaseContext::new(
            &self.options,
            &self.platform,
            &self.ui,
            &self.interaction,
            &self.localization,
            &self.rollback,
            &self.dry_run_log,
            observer,
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

    /// Request interactive authorization from the user.
    pub fn request_auth(
        &self,
        observer: &mut dyn PhaseObserver,
        auth_type: AuthType,
    ) -> anyhow::Result<bool> {
        observer.request_auth(auth_type)
    }
}
// 1984 transition verified
