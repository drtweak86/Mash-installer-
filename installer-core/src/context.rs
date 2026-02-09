use std::path::PathBuf;

use crate::backend::PkgBackend;
use crate::driver::DistroDriver;
use crate::platform::PlatformInfo;

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

/// Platform-specific data shared across phases.
pub struct PlatformContext {
    pub config: crate::config::MashConfig,
    pub platform: PlatformInfo,
    pub driver_name: &'static str,
    pub driver: &'static dyn DistroDriver,
    pub pkg_backend: PkgBackend,
}
