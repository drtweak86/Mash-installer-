use crate::desktop::{DesktopEnvironment, DisplayProtocol};
use crate::model::software::SoftwareTierPlan;
use crate::model::Validator;
use crate::profile::SystemProfile;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
)]
#[strum(serialize_all = "snake_case")]
pub enum EnvironmentTag {
    #[default]
    Home,
    Work,
    Traveling,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ChezmoiOptions {
    pub enabled: bool,
    pub repo_url: Option<String>,
    pub branch: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArgonConfig {
    pub enabled: bool,
    pub cooling_profile: String, // e.g., "Quiet", "Balanced", "Performance"
}

impl Default for ArgonConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cooling_profile: "Balanced".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DockerConfig {
    pub enabled: bool,
    pub data_root: Option<PathBuf>,
}

/// Options provided by the CLI that drive `run_with_driver`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallOptions {
    pub profile: ProfileLevel,
    pub staging_dir: Option<PathBuf>,
    pub dry_run: bool,
    pub interactive: bool,
    pub argon: ArgonConfig,
    pub enable_p10k: bool,
    pub docker: DockerConfig,
    pub continue_on_error: bool,
    pub software_plan: SoftwareTierPlan,
    pub system_profile: Option<SystemProfile>,
    pub environment: EnvironmentTag,
    pub chezmoi: ChezmoiOptions,
    pub desktop_environment: Option<DesktopEnvironment>,
    pub display_protocol: DisplayProtocol,
}

impl Validator for InstallOptions {
    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if let Some(ref path) = self.staging_dir {
            if !path.is_absolute() {
                errors.push(format!("staging_dir must be absolute: {}", path.display()));
            }
        }

        if self.chezmoi.enabled && self.chezmoi.repo_url.is_none() {
            errors.push("Chezmoi is enabled but no repository URL was provided.".to_string());
        }

        errors.extend(self.software_plan.validate());

        errors
    }
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            profile: ProfileLevel::Minimal,
            staging_dir: None,
            dry_run: false,
            interactive: false,
            argon: ArgonConfig::default(),
            enable_p10k: false,
            docker: DockerConfig::default(),
            continue_on_error: false,
            software_plan: SoftwareTierPlan::default(),
            system_profile: None,
            environment: EnvironmentTag::Home,
            chezmoi: ChezmoiOptions::default(),
            desktop_environment: None,
            display_protocol: DisplayProtocol::Auto,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum ProfileLevel {
    #[default]
    Minimal = 0,
    Dev = 1,
    Full = 2,
}

/// CLI-supplied options that guide the installation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserOptionsContext {
    pub profile: ProfileLevel,
    pub staging_dir: PathBuf,
    pub dry_run: bool,
    pub interactive: bool,
    pub argon: ArgonConfig,
    pub enable_p10k: bool,
    pub docker: DockerConfig,
    pub software_plan: SoftwareTierPlan,
    pub system_profile: Option<SystemProfile>,
    pub environment: EnvironmentTag,
    pub chezmoi: ChezmoiOptions,
    pub desktop_environment: Option<DesktopEnvironment>,
    pub display_protocol: DisplayProtocol,
}

impl UserOptionsContext {
    pub fn from_options(opts: &InstallOptions) -> Self {
        Self {
            profile: opts.profile,
            staging_dir: opts
                .staging_dir
                .clone()
                .unwrap_or_else(|| PathBuf::from("/tmp/mash-installer/staging")),
            dry_run: opts.dry_run,
            interactive: opts.interactive,
            argon: opts.argon.clone(),
            enable_p10k: opts.enable_p10k,
            docker: opts.docker.clone(),
            software_plan: opts.software_plan.clone(),
            system_profile: opts.system_profile.clone(),
            environment: opts.environment,
            chezmoi: opts.chezmoi.clone(),
            desktop_environment: opts.desktop_environment,
            display_protocol: opts.display_protocol,
        }
    }
}
