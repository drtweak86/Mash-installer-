use crate::software::SoftwareTierPlan;
use crate::Validator;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Options provided by the CLI that drive `run_with_driver`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallOptions {
    pub profile: ProfileLevel,
    pub staging_dir: Option<PathBuf>,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
    pub continue_on_error: bool,
    pub software_plan: SoftwareTierPlan,
}

impl Validator for InstallOptions {
    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if let Some(ref path) = self.staging_dir {
            if !path.is_absolute() {
                errors.push(format!("staging_dir must be absolute: {}", path.display()));
            }
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
            enable_argon: false,
            enable_p10k: false,
            docker_data_root: false,
            continue_on_error: false,
            software_plan: SoftwareTierPlan::default(),
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
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
    pub software_plan: SoftwareTierPlan,
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
            enable_argon: opts.enable_argon,
            enable_p10k: opts.enable_p10k,
            docker_data_root: opts.docker_data_root,
            software_plan: opts.software_plan.clone(),
        }
    }
}
