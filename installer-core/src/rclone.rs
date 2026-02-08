use anyhow::{Context, Result};
use std::process::Command;

use crate::InstallContext;

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
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

fn try_pkg(ctx: &InstallContext) -> Result<bool> {
    if crate::pkg::is_installed(ctx.driver, "rclone") {
        return Ok(true);
    }

    tracing::info!("Attempting rclone install via package manager");
    if ctx.dry_run {
        tracing::info!("[dry-run] would try package manager install rclone");
        return Ok(true);
    }

    // ensure_packages uses the right backend automatically
    match crate::pkg::ensure_packages(ctx.driver, &["rclone"], false) {
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

fn install_via_script(ctx: &InstallContext) -> Result<()> {
    tracing::info!("Installing rclone via official install script");
    if ctx.dry_run {
        tracing::info!("[dry-run] would run rclone install script");
        return Ok(());
    }

    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL https://rclone.org/install.sh | sudo bash")
        .status()
        .context("running rclone install script")?;

    if !status.success() {
        tracing::warn!("rclone install script failed; continuing");
    }
    Ok(())
}
