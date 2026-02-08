use anyhow::{Context, Result};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{distro, driver::DistroDriver, InstallContext};

// ── Package-manager detection ───────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PkgBackend {
    Apt,
    Pacman,
}

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

/// Auto-detect the system package manager.
#[allow(dead_code)]
pub fn detect_backend() -> PkgBackend {
    distro::detect_backend()
}

// ── Public helpers (backend-agnostic) ───────────────────────────

/// Check whether a package is already installed.
pub fn is_installed(driver: &dyn DistroDriver, pkg: &str) -> bool {
    installer_for(driver).is_installed(pkg)
}

/// Refresh the package database.
pub fn update(driver: &dyn DistroDriver, dry_run: bool) -> Result<()> {
    installer_for(driver).update(dry_run)
}

/// Install a list of packages idempotently.
/// Names are given in Debian-canonical form; they are translated
/// by the driver.
pub fn ensure_packages(driver: &dyn DistroDriver, pkgs: &[&str], dry_run: bool) -> Result<()> {
    let native = distro::translate_names(driver, pkgs);
    let native_refs: Vec<&str> = native.iter().map(String::as_str).collect();
    installer_for(driver).ensure_packages(&native_refs, dry_run)
}

/// Best-effort install of a single optional package (never fatal).
pub fn try_optional(driver: &dyn DistroDriver, pkg: &str, dry_run: bool) {
    installer_for(driver).try_optional(pkg, dry_run);
}

// ── APT backend ─────────────────────────────────────────────────

fn apt_is_installed(pkg: &str) -> bool {
    Command::new("dpkg")
        .args(["-s", pkg])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn apt_update(dry_run: bool) -> Result<()> {
    if dry_run {
        tracing::info!("[dry-run] apt-get update");
        return Ok(());
    }
    let status = Command::new("sudo")
        .args(["apt-get", "update", "-qq"])
        .status()
        .context("running apt-get update")?;
    if !status.success() {
        anyhow::bail!("apt-get update failed");
    }
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

    let status = cmd
        .env("DEBIAN_FRONTEND", "noninteractive")
        .status()
        .context("running apt-get install")?;

    if !status.success() {
        anyhow::bail!("apt-get install failed for: {}", missing.join(" "));
    }
    Ok(())
}

// ── Pacman backend ──────────────────────────────────────────────

fn pacman_is_installed(pkg: &str) -> bool {
    // `pacman -Q pkg` exits 0 if installed; also handles group members
    Command::new("pacman")
        .args(["-Q", pkg])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn pacman_sync(dry_run: bool) -> Result<()> {
    if dry_run {
        tracing::info!("[dry-run] pacman -Syu");
        return Ok(());
    }
    let status = Command::new("sudo")
        .args(["pacman", "-Syu", "--noconfirm"])
        .status()
        .context("running pacman -Syu")?;
    if !status.success() {
        anyhow::bail!("pacman -Syu failed");
    }
    Ok(())
}

fn ensure_pacman_synced(dry_run: bool) -> Result<()> {
    if PACMAN_SYNCED.load(Ordering::SeqCst) {
        return Ok(());
    }

    // Run `pacman -Syu` before any installs to avoid the partial-upgrade risk noted in docs/QAREPORT.md.
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
    // `--needed` makes this idempotent – already-installed packages are skipped.
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

    let status = cmd.status().context("running pacman -S")?;
    if !status.success() {
        anyhow::bail!("pacman -S failed for: {}", pkgs.join(" "));
    }
    Ok(())
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

        let status = Command::new("sudo")
            .args(["apt-get", "install", "-y", "--install-recommends", pkg])
            .env("DEBIAN_FRONTEND", "noninteractive")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        match status {
            Ok(s) if s.success() => tracing::info!("Installed optional package: {pkg}"),
            _ => tracing::warn!("Optional package '{pkg}' not available; skipping"),
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

        let status = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "--needed", pkg])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        match status {
            Ok(s) if s.success() => tracing::info!("Installed optional package: {pkg}"),
            _ => tracing::warn!("Optional package '{pkg}' not available; skipping"),
        }
    }
}

// ── Phase 1: core packages ─────────────────────────────────────

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    update(ctx.driver, ctx.dry_run)?;

    // Always-needed core packages (Debian canonical names)
    let mut pkgs: Vec<&str> = vec![
        "ca-certificates",
        "curl",
        "wget",
        "xz-utils",
        "tar",
        "coreutils",
        "jq",
        "git",
        "software-properties-common",
        "gnupg",
        "lsb-release",
        "apt-transport-https",
    ];

    // Build essentials (all profiles)
    pkgs.extend_from_slice(&[
        "build-essential",
        "pkg-config",
        "clang",
        "lld",
        "cmake",
        "ninja-build",
        "gcc",
        "g++",
        "gdb",
        "make",
    ]);

    // Dev+ packages
    if ctx.profile >= crate::ProfileLevel::Dev {
        pkgs.extend_from_slice(&[
            "python3",
            "python3-pip",
            "python3-venv",
            "ripgrep",
            "fd-find",
            "fzf",
            "tmux",
            "htop",
            "ncdu",
            "neovim",
        ]);
    }

    // Full profile extras
    if ctx.profile >= crate::ProfileLevel::Full {
        pkgs.extend_from_slice(&["nodejs", "npm"]);
    }

    // Optional packages – may not exist in every distro version
    let optional = ["btop", "eza", "yq", "lldb", "bat"];

    // Split required vs optional
    let required: Vec<&str> = pkgs
        .iter()
        .copied()
        .filter(|p| !optional.contains(p))
        .collect();

    ensure_packages(ctx.driver, &required, ctx.dry_run)?;

    // Always attempt lldb
    try_optional(ctx.driver, "lldb", ctx.dry_run);

    // Dev+ optional packages
    if ctx.profile >= crate::ProfileLevel::Dev {
        for pkg in &["btop", "bat", "eza", "yq"] {
            try_optional(ctx.driver, pkg, ctx.dry_run);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        distro,
        driver::{RepoKind, ServiceName},
    };

    struct TestDriver;

    impl DistroDriver for TestDriver {
        fn name(&self) -> &'static str {
            "test"
        }

        fn description(&self) -> &'static str {
            "test driver"
        }

        fn matches(&self, _: &crate::platform::PlatformInfo) -> bool {
            true
        }

        fn pkg_backend(&self) -> PkgBackend {
            PkgBackend::Apt
        }

        fn translate_package(&self, canonical: &str) -> Option<String> {
            match canonical {
                "foo" => Some("foo-native".to_string()),
                "drop" => None,
                _ => Some(canonical.to_string()),
            }
        }

        fn apt_repo_config(&self, _repo: RepoKind) -> Option<crate::driver::AptRepoConfig> {
            None
        }

        fn service_unit(&self, _service: ServiceName) -> &'static str {
            "test.service"
        }
    }

    #[test]
    fn translate_names_respects_driver() {
        let driver = TestDriver;
        let pkgs = ["foo", "bar", "drop"];
        let names = distro::translate_names(&driver, &pkgs);
        assert_eq!(names, vec!["foo-native".to_string(), "bar".to_string()]);
    }
}
