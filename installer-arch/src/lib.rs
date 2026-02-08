use installer_core::{DistroDriver, PkgBackend, PlatformInfo};

pub struct ArchDriver;

impl DistroDriver for ArchDriver {
    fn name(&self) -> &'static str {
        "Arch/Manjaro"
    }

    fn description(&self) -> &'static str {
        "Arch-based with pacman backend"
    }

    fn matches(&self, info: &PlatformInfo) -> bool {
        info.distro_family == "arch"
    }

    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Pacman
    }

    fn translate_package(&self, canonical: &str) -> Option<String> {
        match canonical {
            "software-properties-common" | "apt-transport-https" => None,
            "lsb-release" => None,
            "python3-venv" => None,
            "build-essential" => Some("base-devel".to_string()),
            "pkg-config" => Some("pkgconf".to_string()),
            "ninja-build" => Some("ninja".to_string()),
            "g++" => None,
            "xz-utils" => Some("xz".to_string()),
            "python3" => Some("python".to_string()),
            "python3-pip" => Some("python-pip".to_string()),
            "fd-find" => Some("fd".to_string()),
            "libncurses-dev" => Some("ncurses".to_string()),
            "libssl-dev" => Some("openssl".to_string()),
            "openssh-client" => Some("openssh".to_string()),
            "fonts-terminus" => Some("terminus-font".to_string()),
            "fonts-noto-color-emoji" => Some("noto-fonts-emoji".to_string()),
            "xfonts-terminus" => None,
            "docker-ce" => Some("docker".to_string()),
            "docker-ce-cli" => None,
            "containerd.io" => None,
            "docker-buildx-plugin" => Some("docker-buildx".to_string()),
            "docker-compose-plugin" => Some("docker-compose".to_string()),
            "gh" => Some("github-cli".to_string()),
            _ => Some(canonical.to_string()),
        }
    }
}

static ARCH_DRIVER: ArchDriver = ArchDriver;

pub fn driver() -> &'static dyn DistroDriver {
    &ARCH_DRIVER
}
