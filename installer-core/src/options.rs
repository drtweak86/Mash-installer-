use std::path::PathBuf;

use crate::{SoftwareTierPlan, Validator};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProfileLevel {
    Minimal = 0,
    Dev = 1,
    Full = 2,
}
