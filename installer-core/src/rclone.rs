use anyhow::{Context, Result};
use std::process::Command;

use crate::{cmd, package_manager, PhaseContext};

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    if which::which("rclone").is_ok() {
        tracing::info!("rclone already installed");
        return Ok(());
    }

    // Try the system package manager first
    if try_pkg(ctx)? {
        return Ok(());
    }

    // Fall back to official install script
    install_via_script(ctx)?;
    Ok(())
}

fn try_pkg(ctx: &mut PhaseContext) -> Result<bool> {
    if package_manager::is_installed(ctx.platform.driver, "rclone") {
        return Ok(true);
    }

    tracing::info!("Attempting rclone install via package manager");
    if ctx.options.dry_run {
        tracing::info!("[dry-run] would try package manager install rclone");
        return Ok(true);
    }

    // ensure_packages uses the right backend automatically
    match package_manager::ensure_packages(ctx.platform.driver, &["rclone"], false) {
        Ok(()) => {
            tracing::info!("Installed rclone via package manager");
            Ok(true)
        }
        Err(_) => {
            tracing::info!("rclone not in repos; will use official script");
            Ok(false)
        }
    }
}

fn install_via_script(ctx: &mut PhaseContext) -> Result<()> {
    tracing::info!("Installing rclone via official install script");
    if ctx.options.dry_run {
        tracing::info!("[dry-run] would run rclone install script");
        return Ok(());
    }

    let mut install_cmd = Command::new("sh");
    install_cmd
        .arg("-c")
        .arg("curl -fsSL https://rclone.org/install.sh | sudo bash");
    if let Err(err) = cmd::run(&mut install_cmd).context("running rclone install script") {
        tracing::warn!("rclone install script failed; continuing ({err})");
    }
    Ok(())
}
