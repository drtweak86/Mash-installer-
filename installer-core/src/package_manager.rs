use anyhow::{anyhow, Context, Result};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{backend::PkgBackend, cmd, distro, driver::DistroDriver};

static PACMAN_SYNCED: AtomicBool = AtomicBool::new(false);

pub trait PackageInstaller {
    fn is_installed(&self, pkg: &str) -> bool;
    fn update(&self, dry_run: bool) -> Result<()>;
    fn ensure_packages(&self, pkgs: &[&str], dry_run: bool) -> Result<()>;
    fn try_optional(&self, pkg: &str, dry_run: bool);
}

struct AptInstaller;
struct PacmanInstaller;

static APT_INSTALLER: AptInstaller = AptInstaller;
static PACMAN_INSTALLER: PacmanInstaller = PacmanInstaller;

fn installer_for(driver: &dyn DistroDriver) -> &'static dyn PackageInstaller {
    match driver.pkg_backend() {
        PkgBackend::Apt => &APT_INSTALLER,
        PkgBackend::Pacman => &PACMAN_INSTALLER,
    }
}

pub fn is_installed(driver: &dyn DistroDriver, pkg: &str) -> bool {
    installer_for(driver).is_installed(pkg)
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
    installer_for(driver).try_optional(pkg, dry_run);
}

impl PackageInstaller for AptInstaller {
    fn is_installed(&self, pkg: &str) -> bool {
        apt_is_installed(pkg)
    }

    fn update(&self, dry_run: bool) -> Result<()> {
        apt_update(dry_run)
    }

    fn ensure_packages(&self, pkgs: &[&str], dry_run: bool) -> Result<()> {
        apt_ensure(pkgs, dry_run)
    }

    fn try_optional(&self, pkg: &str, dry_run: bool) {
        if self.is_installed(pkg) {
            return;
        }
        if dry_run {
            tracing::info!("[dry-run] would attempt optional: {pkg}");
            return;
        }
        let mut cmd = Command::new("sudo");
        cmd.args(["apt-get", "install", "-y", "--install-recommends", pkg])
            .env("DEBIAN_FRONTEND", "noninteractive")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        match cmd::run(&mut cmd) {
            Ok(_) => tracing::info!("Installed optional package: {pkg}"),
            Err(_) => tracing::warn!("Optional package '{pkg}' not available; skipping"),
        }
    }
}

impl PackageInstaller for PacmanInstaller {
    fn is_installed(&self, pkg: &str) -> bool {
        pacman_is_installed(pkg)
    }

    fn update(&self, dry_run: bool) -> Result<()> {
        ensure_pacman_synced(dry_run)
    }

    fn ensure_packages(&self, pkgs: &[&str], dry_run: bool) -> Result<()> {
        ensure_pacman_synced(dry_run)?;
        pacman_ensure(pkgs, dry_run)
    }

    fn try_optional(&self, pkg: &str, dry_run: bool) {
        if self.is_installed(pkg) {
            return;
        }
        if dry_run {
            tracing::info!("[dry-run] would attempt optional: {pkg}");
            return;
        }
        let mut cmd = Command::new("sudo");
        cmd.args(["pacman", "-S", "--noconfirm", "--needed", pkg])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        match cmd::run(&mut cmd) {
            Ok(_) => tracing::info!("Installed optional package: {pkg}"),
            Err(_) => tracing::warn!("Optional package '{pkg}' not available; skipping"),
        }
    }
}

fn apt_is_installed(pkg: &str) -> bool {
    let mut cmd = Command::new("dpkg");
    cmd.args(["-s", pkg])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    cmd::run(&mut cmd).map(|_| true).unwrap_or(false)
}

fn apt_update(dry_run: bool) -> Result<()> {
    if dry_run {
        tracing::info!("[dry-run] apt-get update");
        return Ok(());
    }
    let mut cmd = Command::new("sudo");
    cmd.args(["apt-get", "update", "-qq"]);
    cmd::run(&mut cmd).context("running apt-get update")?;
    Ok(())
}

fn apt_ensure(pkgs: &[&str], dry_run: bool) -> Result<()> {
    let missing: Vec<&str> = pkgs
        .iter()
        .copied()
        .filter(|p| !apt_is_installed(p))
        .collect();
    if missing.is_empty() {
        tracing::info!("All packages already installed");
        return Ok(());
    }
    tracing::info!(
        "Installing {} packages: {}",
        missing.len(),
        missing.join(", ")
    );
    if dry_run {
        tracing::info!("[dry-run] would install: {}", missing.join(" "));
        return Ok(());
    }
    let mut cmd = Command::new("sudo");
    cmd.args(["apt-get", "install", "-y", "--install-recommends"]);
    for p in &missing {
        cmd.arg(p);
    }
    cmd.env("DEBIAN_FRONTEND", "noninteractive");
    cmd::run(&mut cmd)
        .context("running apt-get install")
        .map_err(|err| anyhow!("apt-get install failed for {}: {err}", missing.join(" ")))?;
    Ok(())
}

fn pacman_is_installed(pkg: &str) -> bool {
    let mut cmd = Command::new("pacman");
    cmd.args(["-Q", pkg])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    cmd::run(&mut cmd).map(|_| true).unwrap_or(false)
}

fn pacman_sync(dry_run: bool) -> Result<()> {
    if dry_run {
        tracing::info!("[dry-run] pacman -Syu");
        return Ok(());
    }
    let mut cmd = Command::new("sudo");
    cmd.args(["pacman", "-Syu", "--noconfirm"]);
    cmd::run(&mut cmd).context("running pacman -Syu")?;
    Ok(())
}

fn ensure_pacman_synced(dry_run: bool) -> Result<()> {
    if PACMAN_SYNCED.load(Ordering::SeqCst) {
        return Ok(());
    }
    let res = pacman_sync(dry_run);
    if res.is_ok() {
        PACMAN_SYNCED.store(true, Ordering::SeqCst);
    }
    res
}

fn pacman_ensure(pkgs: &[&str], dry_run: bool) -> Result<()> {
    if pkgs.is_empty() {
        return Ok(());
    }
    tracing::info!("Ensuring packages via pacman: {}", pkgs.join(", "));
    if dry_run {
        tracing::info!("[dry-run] would install: {}", pkgs.join(" "));
        return Ok(());
    }
    let mut cmd = Command::new("sudo");
    cmd.args(["pacman", "-S", "--noconfirm", "--needed"]);
    for p in pkgs {
        cmd.arg(p);
    }
    cmd::run(&mut cmd)
        .context("running pacman -S")
        .map_err(|err| anyhow!("pacman -S failed for {}: {err}", pkgs.join(" ")))?;
    Ok(())
}
