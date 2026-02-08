use crate::driver::DistroDriver;
use crate::pkg::PkgBackend;

/// Detect the current package backend by probing the system command.
pub fn detect_backend() -> PkgBackend {
    if which::which("pacman").is_ok() {
        PkgBackend::Pacman
    } else {
        PkgBackend::Apt
    }
}

/// Translate canonical package names via the selected distro driver.
pub fn translate_names(driver: &dyn DistroDriver, pkgs: &[&str]) -> Vec<String> {
    pkgs.iter()
        .filter_map(|pkg| driver.translate_package(pkg))
        .collect()
}
