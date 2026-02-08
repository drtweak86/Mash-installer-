use crate::{pkg::PkgBackend, platform::PlatformInfo};

/// Distro-specific driver plugged in by the CLI.
pub trait DistroDriver: Sync + Send {
    /// Human-friendly identifier for the distro/driver.
    fn name(&self) -> &'static str;

    /// Brief description of the distro or special behavior.
    fn description(&self) -> &'static str;

    /// Does this driver match the detected platform?
    fn matches(&self, info: &PlatformInfo) -> bool;

    /// Default package backend for the distro (apt/pacman).
    fn pkg_backend(&self) -> PkgBackend;
}
