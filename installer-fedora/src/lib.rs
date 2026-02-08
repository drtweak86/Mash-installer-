use installer_core::{DistroDriver, PkgBackend, PlatformInfo};

pub struct FedoraDriver;

impl DistroDriver for FedoraDriver {
    fn name(&self) -> &'static str {
        "Fedora"
    }

    fn description(&self) -> &'static str {
        "Fedora / RPM-based systems (dnf)"
    }

    fn matches(&self, info: &PlatformInfo) -> bool {
        info.distro == "fedora"
    }

    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Pacman
    }
}

static FEDORA_DRIVER: FedoraDriver = FedoraDriver;

pub fn driver() -> &'static dyn DistroDriver {
    &FEDORA_DRIVER
}
