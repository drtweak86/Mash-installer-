use anyhow::{anyhow, Context, Result};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{backend::PkgBackend, distro, driver::DistroDriver};
use mash_system::cmd;

static PACMAN_SYNCED: AtomicBool = AtomicBool::new(false);

pub trait PackageInstaller {
    fn is_installed(&self, pkg: &str) -> bool;
    fn update(&self, dry_run: bool) -> Result<()>;
    fn ensure_packages(&self, pkgs: &[&str], dry_run: bool) -> Result<()>;
    fn try_optional(&self, pkg: &str, dry_run: bool);
}

struct AptInstaller;
struct PacmanInstaller;
struct DnfInstaller;

static APT_INSTALLER: AptInstaller = AptInstaller;
static PACMAN_INSTALLER: PacmanInstaller = PacmanInstaller;
static DNF_INSTALLER: DnfInstaller = DnfInstaller;

pub fn installer_for(driver: &dyn DistroDriver) -> &'static dyn PackageInstaller {
    match driver.pkg_backend() {
        PkgBackend::Apt => &APT_INSTALLER,
        PkgBackend::Pacman => &PACMAN_INSTALLER,
        PkgBackend::Dnf => &DNF_INSTALLER,
    }
}

pub fn is_installed(driver: &dyn DistroDriver, pkg: &str) -> bool {
    let native = match driver.translate_package(pkg) {
        Some(n) => n,
        None => return false,
    };
    driver.is_package_installed(&native)
}

/// Backend-specific check for whether a package is installed.
/// This is used by the default implementation of DistroDriver::is_package_installed.
pub fn check_installed(backend: PkgBackend, pkg: &str) -> bool {
    match backend {
        PkgBackend::Apt => APT_INSTALLER.is_installed(pkg),
        PkgBackend::Pacman => PACMAN_INSTALLER.is_installed(pkg),
        PkgBackend::Dnf => DNF_INSTALLER.is_installed(pkg),
    }
}

pub fn update(driver: &dyn DistroDriver, dry_run: bool) -> Result<()> {
    installer_for(driver).update(dry_run)
}

pub fn ensure_packages(driver: &dyn DistroDriver, pkgs: &[&str], dry_run: bool) -> Result<()> {
    let native = distro::translate_names(driver, pkgs);
    let native_refs: Vec<&str> = native.iter().map(String::as_str).collect();
    installer_for(driver).ensure_packages(&native_refs, dry_run)
}

pub fn try_optional(driver: &dyn DistroDriver, pkg: &str, dry_run: bool) {
    let native = match driver.translate_package(pkg) {
        Some(n) => n,
        None => return,
    };
    installer_for(driver).try_optional(&native, dry_run);
}

impl PackageInstaller for AptInstaller {
    fn is_installed(&self, pkg: &str) -> bool {
        cmd::Command::new("dpkg")
            .args(["-s", pkg])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .execute()
            .is_ok()
    }

    fn update(&self, dry_run: bool) -> Result<()> {
        cmd::Command::new("apt-get")
            .sudo()
            .args(["update", "-qq"])
            .dry_run(dry_run)
            .execute()
            .context("running apt-get update")?;
        Ok(())
    }

    fn ensure_packages(&self, pkgs: &[&str], dry_run: bool) -> Result<()> {
        let missing: Vec<&str> = pkgs
            .iter()
            .copied()
            .filter(|p| !self.is_installed(p))
            .collect();

        if missing.is_empty() {
            tracing::info!("All packages already installed");
            return Ok(());
        }

        tracing::info!("Installing packages: {}", missing.join(", "));

        cmd::Command::new("apt-get")
            .sudo()
            .args(["install", "-y", "--install-recommends"])
            .args(&missing)
            .env("DEBIAN_FRONTEND", "noninteractive")
            .dry_run(dry_run)
            .execute()
            .context("running apt-get install")
            .map_err(|err| anyhow!("apt-get install failed for {}: {err}", missing.join(" ")))?;
        Ok(())
    }

    fn try_optional(&self, pkg: &str, dry_run: bool) {
        if self.is_installed(pkg) {
            return;
        }
        let res = cmd::Command::new("apt-get")
            .sudo()
            .args(["install", "-y", "--install-recommends", pkg])
            .env("DEBIAN_FRONTEND", "noninteractive")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .dry_run(dry_run)
            .execute();

        match res {
            Ok(_) => tracing::info!("Installed optional package: {pkg}"),
            Err(_) => tracing::warn!("Optional package '{pkg}' not available; skipping"),
        }
    }
}

impl PackageInstaller for PacmanInstaller {
    fn is_installed(&self, pkg: &str) -> bool {
        cmd::Command::new("pacman")
            .args(["-Q", pkg])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .execute()
            .is_ok()
    }

    fn update(&self, dry_run: bool) -> Result<()> {
        if PACMAN_SYNCED.load(Ordering::SeqCst) {
            return Ok(());
        }
        cmd::Command::new("pacman")
            .sudo()
            .args(["-Syu", "--noconfirm"])
            .dry_run(dry_run)
            .execute()
            .context("running pacman -Syu")?;

        PACMAN_SYNCED.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn ensure_packages(&self, pkgs: &[&str], dry_run: bool) -> Result<()> {
        if pkgs.is_empty() {
            return Ok(());
        }
        self.update(dry_run)?;

        tracing::info!("Ensuring packages via pacman: {}", pkgs.join(", "));

        cmd::Command::new("pacman")
            .sudo()
            .args(["-S", "--noconfirm", "--needed"])
            .args(pkgs)
            .dry_run(dry_run)
            .execute()
            .context("running pacman -S")
            .map_err(|err| anyhow!("pacman -S failed for {}: {err}", pkgs.join(" ")))?;
        Ok(())
    }

    fn try_optional(&self, pkg: &str, dry_run: bool) {
        if self.is_installed(pkg) {
            return;
        }
        let res = cmd::Command::new("pacman")
            .sudo()
            .args(["-S", "--noconfirm", "--needed", pkg])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .dry_run(dry_run)
            .execute();

        match res {
            Ok(_) => tracing::info!("Installed optional package: {pkg}"),
            Err(_) => tracing::warn!("Optional package '{pkg}' not available; skipping"),
        }
    }
}

impl PackageInstaller for DnfInstaller {
    fn is_installed(&self, pkg: &str) -> bool {
        cmd::Command::new("rpm")
            .args(["-q", pkg])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .execute()
            .is_ok()
    }

    fn update(&self, dry_run: bool) -> Result<()> {
        // dnf check-update returns 100 if updates are available, 0 if none, error otherwise.
        // We just run it to sync metadata.
        let _ = cmd::Command::new("dnf")
            .sudo()
            .args(["check-update", "-q"])
            .dry_run(dry_run)
            .execute();
        Ok(())
    }

    fn ensure_packages(&self, pkgs: &[&str], dry_run: bool) -> Result<()> {
        let missing: Vec<&str> = pkgs
            .iter()
            .copied()
            .filter(|p| !self.is_installed(p))
            .collect();

        if missing.is_empty() {
            tracing::info!("All packages already installed");
            return Ok(());
        }

        tracing::info!("Installing packages via dnf: {}", missing.join(", "));

        cmd::Command::new("dnf")
            .sudo()
            .args(["install", "-y"])
            .args(&missing)
            .dry_run(dry_run)
            .execute()
            .context("running dnf install")
            .map_err(|err| anyhow!("dnf install failed for {}: {err}", missing.join(" ")))?;
        Ok(())
    }

    fn try_optional(&self, pkg: &str, dry_run: bool) {
        if self.is_installed(pkg) {
            return;
        }
        let res = cmd::Command::new("dnf")
            .sudo()
            .args(["install", "-y", pkg])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .dry_run(dry_run)
            .execute();

        match res {
            Ok(_) => tracing::info!("Installed optional package: {pkg}"),
            Err(_) => tracing::warn!("Optional package '{pkg}' not available; skipping"),
        }
    }
}
