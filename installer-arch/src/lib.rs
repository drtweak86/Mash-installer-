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
}

static ARCH_DRIVER: ArchDriver = ArchDriver;

pub fn driver() -> &'static dyn DistroDriver {
    &ARCH_DRIVER
}
