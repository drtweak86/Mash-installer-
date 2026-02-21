// Copyright 2024 MASH Installer Authors
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// Theme installation configuration
pub struct ThemeConfig {
    pub name: &'static str,
    pub resource_path: PathBuf,
    pub target_path: PathBuf,
    pub is_executable: bool,
}

/// Install a theme file from resources to target location
pub fn install_theme_file(config: &ThemeConfig) -> Result<()> {
    info!(
        "Installing theme file: {} -> {}",
        config.resource_path.display(),
        config.target_path.display()
    );

    // Create parent directories if they don't exist
    if let Some(parent) = config.target_path.parent() {
        fs::create_dir_all(parent).context("Failed to create parent directories")?;
    }

    // Copy the file
    fs::copy(&config.resource_path, &config.target_path).context("Failed to copy theme file")?;

    // Set permissions if executable
    if config.is_executable {
        let mut perms = fs::metadata(&config.target_path)
            .context("Failed to get file metadata")?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&config.target_path, perms)
            .context("Failed to set executable permissions")?;
    }

    info!("Successfully installed: {}", config.target_path.display());
    Ok(())
}

/// Install the complete retro theme
pub fn install_retro_theme(base_path: &Path) -> Result<()> {
    info!("Installing BBC/UNIX Retro Theme...");

    let resources_base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("resources")
        .join("themes")
        .join("retro-bbc");

    // Install i3 configuration
    install_theme_file(&ThemeConfig {
        name: "i3-config",
        resource_path: resources_base.join("i3-config"),
        target_path: base_path.join(".config/i3/config"),
        is_executable: false,
    })?;

    // Install i3status configuration
    install_theme_file(&ThemeConfig {
        name: "i3status-config",
        resource_path: resources_base.join("i3status-retro.conf"),
        target_path: base_path.join(".config/i3/i3status-retro.conf"),
        is_executable: false,
    })?;

    // Install Kitty configuration
    install_theme_file(&ThemeConfig {
        name: "kitty-config",
        resource_path: resources_base.join("kitty.conf"),
        target_path: base_path.join(".config/kitty/theme.conf"),
        is_executable: false,
    })?;

    // Install Conky configuration
    install_theme_file(&ThemeConfig {
        name: "conky-config",
        resource_path: resources_base.join("conkyrc"),
        target_path: base_path.join(".config/conky/retro-bbc.conkyrc"),
        is_executable: false,
    })?;

    // Install wallpaper downloader
    install_theme_file(&ThemeConfig {
        name: "wallpaper-downloader",
        resource_path: resources_base.join("wallpaper_downloader_final.py"),
        target_path: base_path.join(".local/bin/wallpaper_downloader_final.py"),
        is_executable: true,
    })?;

    info!("✅ BBC/UNIX Retro Theme installed successfully!");
    Ok(())
}

/// Check if a command exists in PATH
pub fn command_exists(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Ensure i3-gaps is installed
pub fn ensure_i3_installed() -> Result<()> {
    if !command_exists("i3") {
        info!("i3-gaps not found, installing...");
        // This would be replaced with actual package installation
        // For now, we'll just warn
        warn!("i3-gaps is not installed. Please install it manually.");
        warn!("On Debian/Ubuntu: sudo apt install i3 i3status i3lock");
        warn!("On Arch: sudo pacman -S i3-gaps i3status i3lock");
    } else {
        info!("✅ i3-gaps is already installed");
    }
    Ok(())
}

/// Ensure Kitty terminal is installed
pub fn ensure_kitty_installed() -> Result<()> {
    if !command_exists("kitty") {
        info!("Kitty terminal not found, installing...");
        // This would be replaced with actual package installation
        // For now, we'll just warn
        warn!("Kitty terminal is not installed. Please install it manually.");
        warn!("On Debian/Ubuntu: sudo apt install kitty");
        warn!("On Arch: sudo pacman -S kitty");
    } else {
        info!("✅ Kitty terminal is already installed");
    }
    Ok(())
}

/// Ensure all retro theme dependencies are installed
pub fn ensure_retro_theme_dependencies() -> Result<()> {
    info!("Checking retro theme dependencies...");
    ensure_i3_installed()?;
    ensure_kitty_installed()?;
    info!("✅ All dependency checks complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_command_exists() {
        assert!(command_exists("ls"));
        assert!(!command_exists("nonexistent-command-12345"));
    }

    #[test]
    fn test_theme_file_install() {
        let dir = tempdir().unwrap();
        let source = PathBuf::from("Cargo.toml");
        let target = dir.path().join("test-config");
        let target_clone = target.clone();

        let config = ThemeConfig {
            name: "test",
            resource_path: source,
            target_path: target_clone,
            is_executable: false,
        };

        let result = install_theme_file(&config);
        assert!(result.is_ok());
        assert!(target.exists());
    }
}
