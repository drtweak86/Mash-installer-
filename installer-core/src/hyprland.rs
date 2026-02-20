use anyhow::{Context, Result};
use std::process::Command;

use crate::{cmd, package_manager, PhaseContext, PkgBackend};

/// Fix Hyprland sound on Arch Linux aarch64
///
/// Common issue: Hyprland (Wayland) doesn't have sound out of the box on Arch.
/// This installs and configures PipeWire audio system.
///
/// Optimized for: Raspberry Pi 4B 8GB, Arch Linux aarch64, Hyprland
pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    // Only run on Arch-based systems
    if ctx.platform.pkg_backend != PkgBackend::Pacman {
        tracing::debug!("Skipping Hyprland audio fix (not on Arch)");
        return Ok(());
    }

    // Check if Hyprland is installed
    if !is_hyprland_installed() {
        tracing::info!("Hyprland not detected, skipping audio fix");
        return Ok(());
    }

    tracing::info!("Hyprland detected! Setting up PipeWire audio (fixes sound issue)");

    // Install PipeWire audio packages
    let audio_packages = [
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "wireplumber",
        "pipewire-audio",
    ];

    if ctx.options.dry_run {
        ctx.record_dry_run(
            "hyprland_audio",
            "Would install PipeWire audio stack",
            Some(audio_packages.join(", ")),
        );
    } else {
        tracing::info!("Installing PipeWire audio packages...");
        package_manager::ensure_packages(ctx.platform.driver, &audio_packages, false)?;
    }

    // Add user to audio group
    if let Ok(username) = std::env::var("USER") {
        if !is_user_in_group(&username, "audio") {
            tracing::info!("Adding user '{}' to audio group", username);
            if ctx.options.dry_run {
                ctx.record_dry_run(
                    "hyprland_audio",
                    "Would add user to audio group",
                    Some(username.clone()),
                );
            } else {
                let mut usermod_cmd = Command::new("sudo");
                usermod_cmd.args(["usermod", "-aG", "audio", &username]);
                if let Err(err) = cmd::run(&mut usermod_cmd) {
                    tracing::warn!("Failed to add user to audio group: {err}");
                }
            }
        }
    }

    // Enable and start PipeWire services
    let services = ["pipewire.service", "pipewire-pulse.service", "wireplumber.service"];

    for service in &services {
        if ctx.options.dry_run {
            ctx.record_dry_run(
                "hyprland_audio",
                "Would enable systemd user service",
                Some(service.to_string()),
            );
        } else {
            tracing::info!("Enabling {}", service);
            enable_user_service(service)?;
        }
    }

    tracing::info!("✓ Hyprland audio configured! You may need to log out and back in for audio to work.");

    Ok(())
}

fn is_hyprland_installed() -> bool {
    which::which("Hyprland").is_ok() || which::which("hyprland").is_ok()
}

fn is_user_in_group(username: &str, group: &str) -> bool {
    let mut cmd = Command::new("groups");
    cmd.arg(username);

    if let Ok(output) = cmd.output() {
        if let Ok(groups_str) = String::from_utf8(output.stdout) {
            return groups_str.split_whitespace().any(|g| g == group);
        }
    }
    false
}

fn enable_user_service(service: &str) -> Result<()> {
    // Enable service for current user
    let mut enable_cmd = Command::new("systemctl");
    enable_cmd.args(["--user", "enable", service]);
    cmd::run(&mut enable_cmd).context(format!("enabling {}", service))?;

    // Start service for current user
    let mut start_cmd = Command::new("systemctl");
    start_cmd.args(["--user", "start", service]);
    if let Err(err) = cmd::run(&mut start_cmd) {
        tracing::warn!("Failed to start {} (will start on next login): {}", service, err);
    }

    Ok(())
}
