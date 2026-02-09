use crate::{backend::PkgBackend, platform::PlatformInfo};
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

    /// Check whether a system package (canonical name) is already installed.
    fn is_package_installed(&self, _package: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::PlatformInfo;
    use crate::PkgBackend;

    struct TestDriver;

    impl DistroDriver for TestDriver {
        fn name(&self) -> &'static str {
            "test-driver"
        }

        fn description(&self) -> &'static str {
            "uses default service names"
        }

        fn matches(&self, _: &PlatformInfo) -> bool {
            true
        }

        fn pkg_backend(&self) -> PkgBackend {
            PkgBackend::Apt
        }

        fn translate_package(&self, canonical: &str) -> Option<String> {
            Some(canonical.to_string())
        }
    }

    struct CustomServiceDriver;

    impl DistroDriver for CustomServiceDriver {
        fn name(&self) -> &'static str {
            "custom"
        }

        fn description(&self) -> &'static str {
            "customizes services"
        }

        fn matches(&self, _: &PlatformInfo) -> bool {
            true
        }

        fn pkg_backend(&self) -> PkgBackend {
            PkgBackend::Pacman
        }

        fn translate_package(&self, canonical: &str) -> Option<String> {
            Some(canonical.to_string())
        }

        fn service_unit(&self, service: ServiceName) -> &'static str {
            match service {
                ServiceName::Docker => "custom-docker.service",
                ServiceName::ArgonOne => "custom-argononed.service",
            }
        }
    }

    #[test]
    fn default_service_names_are_returned() {
        let driver = TestDriver;
        assert_eq!(driver.service_unit(ServiceName::Docker), "docker.service");
        assert_eq!(
            driver.service_unit(ServiceName::ArgonOne),
            "argononed.service"
        );
    }

    #[test]
    fn custom_driver_overrides_service_names() {
        let driver = CustomServiceDriver;
        assert_eq!(
            driver.service_unit(ServiceName::Docker),
            "custom-docker.service"
        );
        assert_eq!(
            driver.service_unit(ServiceName::ArgonOne),
            "custom-argononed.service"
        );
    }
}
