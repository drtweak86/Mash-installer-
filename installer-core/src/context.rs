use std::path::{Path, PathBuf};

use crate::backend::PkgBackend;
use crate::config;
use crate::driver::DistroDriver;
use crate::localization::Localization;
use crate::platform::PlatformInfo;
use crate::rollback::RollbackManager;
use crate::staging;
use anyhow::Result;

/// CLI-supplied options that guide the installation.
pub struct UserOptionsContext {
    pub profile: crate::ProfileLevel,
    pub staging_dir: PathBuf,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
}

/// Options that override values in the persisted Mash config.
#[derive(Debug, Clone, Default)]
pub struct ConfigOverrides {
    pub staging_dir: Option<PathBuf>,
}

impl ConfigOverrides {
    pub fn staging_dir(&self) -> Option<&Path> {
        self.staging_dir.as_deref()
    }
}

/// Service that loads and exposes the Mash config along with any overrides.
pub struct ConfigService {
    config: config::MashConfig,
    overrides: ConfigOverrides,
}

impl ConfigService {
    pub fn load() -> Result<Self> {
        Self::load_with_overrides(ConfigOverrides::default())
    }

    pub fn load_with_overrides(overrides: ConfigOverrides) -> Result<Self> {
        let config = config::load_or_default()?;
        Ok(Self { config, overrides })
    }

    pub fn config(&self) -> &config::MashConfig {
        &self.config
    }

    pub fn overrides(&self) -> &ConfigOverrides {
        &self.overrides
    }

    pub fn staging_override(&self) -> Option<&Path> {
        self.overrides.staging_dir()
    }

    pub fn resolve_staging_dir(&self) -> Result<PathBuf> {
        staging::resolve(self.staging_override(), self.config())
    }
}

/// Platform-specific data shared across phases.
pub struct PlatformContext {
    pub config_service: ConfigService,
    pub platform: PlatformInfo,
    pub driver_name: &'static str,
    pub driver: &'static dyn DistroDriver,
    pub pkg_backend: PkgBackend,
}

impl PlatformContext {
    pub fn config(&self) -> &config::MashConfig {
        self.config_service.config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn overrides_default_to_none() {
        let overrides = ConfigOverrides::default();
        assert!(overrides.staging_dir().is_none());
    }

    #[test]
    fn staging_override_returns_specified_path() {
        let path = PathBuf::from("/tmp/custom-stage");
        let overrides = ConfigOverrides {
            staging_dir: Some(path.clone()),
        };
        assert_eq!(overrides.staging_dir(), Some(path.as_path()));
    }

    #[test]
    fn config_service_resolves_overrides_internally() {
        let config = config::MashConfig::default();
        let overrides = ConfigOverrides {
            staging_dir: Some(PathBuf::from("/tmp/custom-stage")),
        };
        let service = ConfigService {
            config: config.clone(),
            overrides,
        };
        assert_eq!(service.config(), &config);
        assert!(service.staging_override().is_some());
    }
}

/// UI-related data that may become necessary for rendering progress or logging.
#[derive(Default)]
pub struct UIContext;

/// Combined contexts passed to individual phases.
pub struct PhaseContext<'a> {
    pub options: &'a UserOptionsContext,
    pub platform: &'a PlatformContext,
    pub ui: &'a UIContext,
    pub localization: &'a Localization,
    pub rollback: &'a RollbackManager,
}

impl<'a> PhaseContext<'a> {
    /// Register a rollback action associated with the provided label.
    pub fn register_rollback_action(
        &self,
        label: impl Into<String>,
        action: impl Fn() -> Result<()> + 'static,
    ) {
        self.rollback.register_action(label, action);
    }
}
