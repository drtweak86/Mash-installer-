use crate::{pkg::PkgBackend, platform::PlatformInfo};
use anyhow::Result;

/// Represents named repositories the installer may need to add.
#[derive(Clone, Copy, Debug)]
pub enum RepoKind {
    Docker,
    GitHubCli,
}

/// Supported services that may be enabled or managed by the installer.
#[derive(Clone, Copy, Debug)]
pub enum ServiceName {
    Docker,
    ArgonOne,
}

/// Metadata describing how to add an apt repository.
pub struct AptRepoConfig {
    pub label: &'static str,
    pub key_path: &'static str,
    pub key_url: fn(&PlatformInfo) -> Result<String>,
    pub sources_path: &'static str,
    pub repo_line: fn(&PlatformInfo) -> Result<String>,
}

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

    /// Translate a canonical package name for this distro (Arch renames, etc.).
    fn translate_package(&self, canonical: &str) -> Option<String> {
        Some(canonical.to_string())
    }

    /// Provide apt repository configuration for a named repo type.
    fn apt_repo_config(&self, _repo: RepoKind) -> Option<AptRepoConfig> {
        None
    }

    /// Map a logical service name to the distro’s unit name.
    fn service_unit(&self, service: ServiceName) -> &'static str {
        match service {
            ServiceName::Docker => "docker.service",
            ServiceName::ArgonOne => "argononed.service",
        }
    }
}
