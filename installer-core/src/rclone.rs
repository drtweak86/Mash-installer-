use anyhow::{Context, Result};
use std::process::Command;

use crate::{package_manager, AuthType, AuthorizationService, PhaseContext, PhaseResult};
use mash_system::cmd;

pub fn install_phase(ctx: &mut PhaseContext) -> Result<PhaseResult> {
    if which::which("rclone").is_err() {
        // Try the system package manager first
        if !try_pkg(ctx)? {
            // Fall back to official install script
            install_via_script(ctx)?;
        }
    } else {
        tracing::info!("rclone already installed");
    }

    if ctx.options.interactive {
        // Rclone Config
        if !AuthorizationService::new(ctx.observer, ctx.options)
            .is_authorized(AuthType::RcloneConfig)
            && ctx.observer.request_auth(AuthType::RcloneConfig)?
        {
            AuthorizationService::new(ctx.observer, ctx.options)
                .authorize(AuthType::RcloneConfig)?;
            ctx.record_configured("rclone cloud remotes");
        }
    }

    Ok(PhaseResult::Success)
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
        .arg("curl -fsSL --proto '=https' --tlsv1.2 https://rclone.org/install.sh | sudo bash");
    if let Err(err) = cmd::run(&mut install_cmd).context("running rclone install script") {
        tracing::warn!("rclone install script failed; continuing ({err})");
    }
    Ok(())
}
