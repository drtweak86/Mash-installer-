use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use crate::{pkg::PkgBackend, systemd, InstallContext};

/// Clone target for the argononed C daemon.
const ARGONONED_REPO: &str = "https://gitlab.com/DarkElvenAngel/argononed.git";
/// Where we clone the source for building.
const ARGONONED_SRC: &str = "/usr/local/src/argononed";

/// Install Argon One fan control.
///
/// Strategy varies by distro family:
/// - **Debian/Ubuntu**: uses the official Argon40 install script (Python-based).
/// - **Arch/Manjaro**: builds the argononed C daemon from source, which is the
///   community-recommended approach (the OEM script does not support Arch).
///
/// See: <https://gitlab.com/DarkElvenAngel/argononed>
pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    // Quick gate: only relevant on Raspberry Pi hardware
    if ctx.platform.pi_model.is_none() {
        tracing::warn!("Not running on a Raspberry Pi; skipping Argon One");
        return Ok(());
    }

    let backend = crate::pkg::detect_backend();
    match backend {
        PkgBackend::Pacman => install_argononed(ctx),
        PkgBackend::Apt => install_argon_oem(ctx),
    }
}

// ── Arch/Manjaro path: argononed C daemon ───────────────────────

fn install_argononed(ctx: &InstallContext) -> Result<()> {
    // Already installed?
    if which::which("argononed").is_ok()
        || Path::new("/usr/sbin/argononed").exists()
        || Path::new("/usr/bin/argononed").exists()
    {
        tracing::info!("argononed daemon already installed");
        return Ok(());
    }

    tracing::info!("Installing argononed C daemon (Arch/Manjaro path)");

    // Ensure build deps: gcc/make already in base-devel, we need dtc
    ensure_argononed_deps(ctx)?;

    if ctx.dry_run {
        tracing::info!("[dry-run] would clone, build, and install argononed");
        return Ok(());
    }

    clone_argononed()?;
    build_argononed()?;
    enable_argononed_service()?;

    tracing::info!("argononed installed and service enabled");
    Ok(())
}

fn ensure_argononed_deps(ctx: &InstallContext) -> Result<()> {
    // dtc = device tree compiler, needed by argononed's build
    // git is already installed from earlier phases
    crate::pkg::ensure_packages(&["dtc"], ctx.dry_run)?;
    Ok(())
}

fn clone_argononed() -> Result<()> {
    let dest = Path::new(ARGONONED_SRC);
    if dest.exists() {
        tracing::info!("argononed source already present at {}", dest.display());
        // Pull latest in case of partial previous build
        let _ = Command::new("sudo")
            .args(["git", "-C", ARGONONED_SRC, "pull", "--ff-only"])
            .status();
        return Ok(());
    }

    let status = Command::new("sudo")
        .args(["git", "clone", "--depth=1", ARGONONED_REPO, ARGONONED_SRC])
        .status()
        .context("cloning argononed")?;

    if !status.success() {
        anyhow::bail!("Failed to clone argononed from {}", ARGONONED_REPO);
    }
    Ok(())
}

fn build_argononed() -> Result<()> {
    // The project provides an `./install` script that wraps configure+make+install.
    // We use it because it also installs the systemd unit and dtoverlay.
    tracing::info!("Building and installing argononed (this may take a moment)");

    let status = Command::new("sudo")
        .arg("bash")
        .arg("-c")
        .arg(format!("cd {ARGONONED_SRC} && ./install"))
        .status()
        .context("running argononed install script")?;

    if !status.success() {
        // Fall back to manual configure+make if the convenience script fails
        tracing::warn!("./install script failed; attempting manual build");
        let status = Command::new("sudo")
            .arg("bash")
            .arg("-c")
            .arg(format!(
                "cd {ARGONONED_SRC} && ./configure && make all && make install"
            ))
            .status()
            .context("running argononed manual build")?;
        if !status.success() {
            anyhow::bail!(
                "argononed build failed. Check build deps (base-devel, dtc) and kernel headers."
            );
        }
    }
    Ok(())
}

fn enable_argononed_service() -> Result<()> {
    if !systemd::is_available() {
        tracing::warn!("systemd not detected; skipping argononed.service enable");
        return Ok(());
    }
    let _ = Command::new("sudo")
        .args(["systemctl", "daemon-reload"])
        .status();
    let status = Command::new("sudo")
        .args(["systemctl", "enable", "argononed.service"])
        .status()
        .context("enabling argononed.service")?;
    if !status.success() {
        tracing::warn!("Failed to enable argononed.service; you may need to reboot first");
    }
    Ok(())
}

// ── Debian/Ubuntu path: OEM script ──────────────────────────────

fn install_argon_oem(ctx: &InstallContext) -> Result<()> {
    if which::which("argonone-config").is_ok() {
        tracing::info!("Argon One OEM scripts already installed");
        return Ok(());
    }

    tracing::info!("Installing Argon One fan control via OEM script (Debian/Ubuntu path)");
    if ctx.dry_run {
        tracing::info!("[dry-run] would run Argon40 OEM install script");
        return Ok(());
    }

    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL https://download.argon40.com/argon1.sh | bash")
        .status()
        .context("running Argon One OEM install script")?;

    if !status.success() {
        tracing::warn!("Argon One OEM install script failed; this is non-critical");
    }

    Ok(())
}
