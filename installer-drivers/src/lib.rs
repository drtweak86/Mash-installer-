pub mod arch;
pub mod debian;
pub mod fedora;

use installer_core::DistroDriver;

/// Returns a list of all available distribution drivers.
pub fn all_drivers() -> Vec<&'static dyn DistroDriver> {
    vec![
        &arch::ARCH_DRIVER,
        &debian::DEBIAN_DRIVER,
        &fedora::FEDORA_DRIVER,
    ]
}

/// Returns the driver matching the given system information, if any.
pub fn find_driver(info: &installer_core::PlatformInfo) -> Option<&'static dyn DistroDriver> {
    all_drivers().into_iter().find(|d| d.matches(info))
}
