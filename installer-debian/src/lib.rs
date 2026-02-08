use installer_core::{DistroDriver, PkgBackend, PlatformInfo};

pub struct DebianDriver;

impl DistroDriver for DebianDriver {
    fn name(&self) -> &'static str {
        "Debian/Ubuntu"
    }

    fn description(&self) -> &'static str {
        "Debian-family with apt backend"
    }

    fn matches(&self, info: &PlatformInfo) -> bool {
        info.distro_family == "debian"
    }

    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Apt
    }
}

static DEBIAN_DRIVER: DebianDriver = DebianDriver;

pub fn driver() -> &'static dyn DistroDriver {
    &DEBIAN_DRIVER
}
