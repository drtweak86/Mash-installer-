use installer_core::{cmd, DistroDriver, PkgBackend, PlatformInfo};
use std::process::Command;

pub struct FedoraDriver;

impl DistroDriver for FedoraDriver {
    fn name(&self) -> &'static str {
        "Fedora/RHEL"
    }

    fn description(&self) -> &'static str {
        "Fedora/RHEL/CentOS/Rocky/AlmaLinux with dnf backend"
    }

    fn matches(&self, info: &PlatformInfo) -> bool {
        info.distro_family == "fedora"
    }

    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Dnf
    }

    fn translate_package(&self, canonical: &str) -> Option<String> {
        match canonical {
            "software-properties-common" | "apt-transport-https" | "lsb-release" => None,
            "g++" => Some("gcc-c++".to_string()),
            "xz-utils" => Some("xz".to_string()),
            "python3-pip" => Some("python3-pip".to_string()),
            "borgbackup" => Some("borgbackup".to_string()),
            "wireguard" => Some("wireguard-tools".to_string()),
            "fd-find" => Some("fd-find".to_string()),
            "libncurses-dev" => Some("ncurses-devel".to_string()),
            "libssl-dev" => Some("openssl-devel".to_string()),
            "openssh-client" => Some("openssh-clients".to_string()),
            "fonts-terminus" => Some("terminus-fonts".to_string()),
            "fonts-noto-color-emoji" => Some("google-noto-emoji-color-fonts".to_string()),
            "docker-ce" => Some("docker".to_string()),
            "docker-ce-cli" => None,
            "containerd.io" => Some("containerd".to_string()),
            "docker-buildx-plugin" => Some("docker-buildx".to_string()),
            "docker-compose-plugin" => Some("docker-compose".to_string()),
            "gh" => Some("gh".to_string()),
            _ => Some(canonical.to_string()),
        }
    }

    fn is_package_installed(&self, package_name: &str) -> bool {
        let native = match self.translate_package(package_name) {
            Some(name) => name,
            None => return false,
        };
        let mut cmd = Command::new("rpm");
        cmd.args(["-q", native.as_str()]);
        cmd::run(&mut cmd).is_ok()
    }
}

static FEDORA_DRIVER: FedoraDriver = FedoraDriver;

pub fn driver() -> &'static dyn DistroDriver {
    &FEDORA_DRIVER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpp_translates_to_gcc_cxx() {
        assert_eq!(
            driver().translate_package("g++"),
            Some("gcc-c++".to_string())
        );
    }

    #[test]
    fn optional_packages_translate_to_none() {
        assert!(driver().translate_package("docker-ce-cli").is_none());
    }
}
