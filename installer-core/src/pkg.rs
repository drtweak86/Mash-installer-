use anyhow::{Context, Result};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{driver::DistroDriver, InstallContext};

// ── Package-manager detection ───────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PkgBackend {
    Apt,
    Pacman,
}

static PACMAN_SYNCED: AtomicBool = AtomicBool::new(false);

/// Auto-detect the system package manager.
#[allow(dead_code)]
pub fn detect_backend() -> PkgBackend {
    if which::which("pacman").is_ok() {
        PkgBackend::Pacman
    } else {
        PkgBackend::Apt
    }
}

fn translate_names(driver: &dyn DistroDriver, pkgs: &[&str]) -> Vec<String> {
    pkgs.iter()
        .filter_map(|pkg| driver.translate_package(pkg))
        .collect()
}

// ── Public helpers (backend-agnostic) ───────────────────────────

/// Check whether a package is already installed.
pub fn is_installed(driver: &dyn DistroDriver, pkg: &str) -> bool {
    match driver.pkg_backend() {
        PkgBackend::Apt => apt_is_installed(pkg),
        PkgBackend::Pacman => {
            let native = driver
                .translate_package(pkg)
                .unwrap_or_else(|| pkg.to_string());
            pacman_is_installed(native.as_str())
        }
    }
}

/// Refresh the package database.
pub fn update(driver: &dyn DistroDriver, dry_run: bool) -> Result<()> {
    match driver.pkg_backend() {
        PkgBackend::Apt => apt_update(dry_run),
        PkgBackend::Pacman => ensure_pacman_synced(dry_run),
    }
}

/// Install a list of packages idempotently.
/// Names are given in Debian-canonical form; they are translated
/// by the driver.
pub fn ensure_packages(driver: &dyn DistroDriver, pkgs: &[&str], dry_run: bool) -> Result<()> {
    let backend = driver.pkg_backend();
    let native = translate_names(driver, pkgs);
    let native_refs: Vec<&str> = native.iter().map(String::as_str).collect();

    match backend {
        PkgBackend::Apt => apt_ensure(&native_refs, dry_run),
        PkgBackend::Pacman => {
            ensure_pacman_synced(dry_run)?;
            pacman_ensure(&native_refs, dry_run)
        }
    }
}

/// Best-effort install of a single optional package (never fatal).
pub fn try_optional(driver: &dyn DistroDriver, pkg: &str, dry_run: bool) {
    let backend = driver.pkg_backend();
    let native = match driver.translate_package(pkg) {
        Some(name) => name,
        None => return,
    };

    if is_installed(driver, pkg) {
        return;
    }
    if dry_run {
        tracing::info!("[dry-run] would attempt optional: {native}");
        return;
    }

    let status = match backend {
        PkgBackend::Apt => Command::new("sudo")
            .args(["apt-get", "install", "-y", "--install-recommends", &native])
            .env("DEBIAN_FRONTEND", "noninteractive")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status(),
        PkgBackend::Pacman => Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "--needed", &native])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status(),
    };

    match status {
        Ok(s) if s.success() => tracing::info!("Installed optional package: {native}"),
        _ => tracing::warn!("Optional package '{native}' not available; skipping"),
    }
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
