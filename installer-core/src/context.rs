use std::path::PathBuf;

use crate::backend::PkgBackend;
use crate::config;
use crate::driver::DistroDriver;
use crate::platform::PlatformInfo;
use anyhow::Result;

/// CLI-supplied options that guide the installation.
pub struct OptionsContext {
    pub profile: crate::ProfileLevel,
    pub staging_dir: PathBuf,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
}

/// Service that loads and exposes the Mash config.
pub struct ConfigService {
    config: config::MashConfig,
}

impl ConfigService {
    pub fn load() -> Result<Self> {
        Ok(Self {
            config: config::load_or_default()?,
        })
    }

    pub fn config(&self) -> &config::MashConfig {
        &self.config
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
