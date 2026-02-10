use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use crate::{cmd, driver::ServiceName, package_manager, systemd, PhaseContext, PkgBackend};

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
pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    // Quick gate: only relevant on Raspberry Pi hardware
    if ctx.platform.platform.pi_model.is_none() {
        tracing::warn!("Not running on a Raspberry Pi; skipping Argon One");
        return Ok(());
    }

    match ctx.platform.pkg_backend {
        PkgBackend::Pacman => install_argononed(ctx),
        PkgBackend::Apt => install_argon_oem(ctx),
    }
}

// ── Arch/Manjaro path: argononed C daemon ───────────────────────

fn install_argononed(ctx: &mut PhaseContext) -> Result<()> {
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

    if ctx.options.dry_run {
        tracing::info!("[dry-run] would clone, build, and install argononed");
        return Ok(());
    }

    clone_argononed()?;
    build_argononed()?;
    enable_argononed_service(ctx)?;

    tracing::info!("argononed installed and service enabled");
    Ok(())
}

fn ensure_argononed_deps(ctx: &mut PhaseContext) -> Result<()> {
    // dtc = device tree compiler, needed by argononed's build
    // git is already installed from earlier phases
    package_manager::ensure_packages(ctx.platform.driver, &["dtc"], ctx.options.dry_run)?;
    Ok(())
}

fn clone_argononed() -> Result<()> {
    let dest = Path::new(ARGONONED_SRC);
    if dest.exists() {
        tracing::info!("argononed source already present at {}", dest.display());
        // Pull latest in case of partial previous build
        let mut pull_cmd = Command::new("sudo");
        pull_cmd.args(["git", "-C", ARGONONED_SRC, "pull", "--ff-only"]);
        let _ = cmd::run(&mut pull_cmd);
        return Ok(());
    }

    let mut clone_cmd = Command::new("sudo");
    clone_cmd.args(["git", "clone", "--depth=1", ARGONONED_REPO, ARGONONED_SRC]);
    cmd::run(&mut clone_cmd).context("cloning argononed")?;
    Ok(())
}

fn build_argononed() -> Result<()> {
    // The project provides an `./install` script that wraps configure+make+install.
    // We use it because it also installs the systemd unit and dtoverlay.
    tracing::info!("Building and installing argononed (this may take a moment)");

    let mut install_cmd = Command::new("sudo");
    install_cmd
        .arg("bash")
        .arg("-c")
        .arg(format!("cd {ARGONONED_SRC} && ./install"));
    if let Err(err) = cmd::run(&mut install_cmd) {
        tracing::warn!("./install script failed; attempting manual build ({err})");
        let mut manual_cmd = Command::new("sudo");
        manual_cmd.arg("bash").arg("-c").arg(format!(
            "cd {ARGONONED_SRC} && ./configure && make all && make install"
        ));
        cmd::run(&mut manual_cmd).context("running argononed manual build")?;
    }
    Ok(())
}

fn enable_argononed_service(ctx: &mut PhaseContext) -> Result<()> {
    if !systemd::is_available() {
        tracing::warn!("systemd not detected; skipping argononed.service enable");
        return Ok(());
    }
    let mut reload_cmd = Command::new("sudo");
    reload_cmd.args(["systemctl", "daemon-reload"]);
    let _ = cmd::run(&mut reload_cmd);
    let service = ctx.platform.driver.service_unit(ServiceName::ArgonOne);
    let mut enable_cmd = Command::new("sudo");
    enable_cmd.args(["systemctl", "enable", service]);
    if let Err(err) = cmd::run(&mut enable_cmd).context(format!("enabling {service}")) {
        tracing::warn!("Failed to enable {service}; you may need to reboot first ({err})");
    }
    Ok(())
}

// ── Debian/Ubuntu path: OEM script ──────────────────────────────

fn install_argon_oem(ctx: &mut PhaseContext) -> Result<()> {
    if which::which("argonone-config").is_ok() {
        tracing::info!("Argon One OEM scripts already installed");
        return Ok(());
    }

    tracing::info!("Installing Argon One fan control via OEM script (Debian/Ubuntu path)");
    if ctx.options.dry_run {
        tracing::info!("[dry-run] would run Argon40 OEM install script");
        return Ok(());
    }

    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg("curl -fsSL https://download.argon40.com/argon1.sh | bash");
    if let Err(err) = cmd::run(&mut cmd).context("running Argon One OEM install script") {
        tracing::warn!("Argon One OEM install script failed; this is non-critical ({err})");
    }

    Ok(())
}
