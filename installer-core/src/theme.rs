// Copyright 2024 MASH Installer Authors
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// A Theme defines the visual style of the environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub description: String,
    pub colors: BTreeMap<String, String>, // key -> hex
    pub font_id: Option<String>,
    pub wallpaper_id: Option<String>,
    pub configs: Vec<ThemeConfigEntry>,
}

/// A configuration file associated with a theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfigEntry {
    pub name: String,
    pub resource_path: String, // Relative to resources/themes/<theme_id>/
    pub target_path: String,   // Relative to user home, e.g., ".config/i3/config"
    pub is_executable: bool,
}

use crate::dotfiles::{DeployStrategy, DotfileManager};

/// Theme installation configuration (internal)
pub struct ThemeConfig {
    pub name: String,
    pub resource_path: PathBuf,
    pub target_path: PathBuf, // Relative to base_path (home)
    pub is_executable: bool,
    pub strategy: DeployStrategy,
}

impl Theme {
    pub fn install(&self, base_path: &Path, dry_run: bool) -> Result<()> {
        info!("Installing theme: {}...", self.name);
        let df_mgr = DotfileManager::new(base_path, dry_run);

        let resources_base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("resources")
            .join("themes")
            .join(&self.id);

        for entry in &self.configs {
            let target_rel = PathBuf::from(&entry.target_path);
            let resource_path = resources_base.join(&entry.resource_path);

            df_mgr.deploy(&resource_path, &target_rel, DeployStrategy::Copy)?;

            // Set permissions if executable
            if entry.is_executable && !dry_run {
                let target_full = base_path.join(&target_rel);
                let mut perms = fs::metadata(&target_full)
                    .context("Failed to get file metadata")?
                    .permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&target_full, perms)
                    .context("Failed to set executable permissions")?;
            }
        }

        info!("✅ Theme '{}' installed successfully!", self.name);
        Ok(())
    }
}

/// Install a theme file from resources to target location (legacy compat)
pub fn install_theme_file(config: &ThemeConfig) -> Result<()> {
    let mgr = DotfileManager::new(Path::new("/"), false); // Legacy assume absolute or handled elsewhere
    mgr.deploy(&config.resource_path, &config.target_path, config.strategy)?;

    if config.is_executable {
        let mut perms = fs::metadata(&config.target_path)
            .context("Failed to get file metadata")?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&config.target_path, perms)
            .context("Failed to set executable permissions")?;
    }
    Ok(())
}

/// Get the legacy retro theme as a data-driven struct
pub fn get_retro_theme() -> Theme {
    Theme {
        id: "retro-bbc".into(),
        name: "BBC/UNIX Retro".into(),
        description: "Classic BBC Micro and Early UNIX terminal aesthetics.".into(),
        colors: [
            ("background".into(), "#000000".into()),
            ("foreground".into(), "#ffffff".into()),
            ("accent".into(), "#ffff00".into()),
        ]
        .into_iter()
        .collect(),
        font_id: Some("JetBrainsMono".into()),
        wallpaper_id: None,
        configs: vec![
            ThemeConfigEntry {
                name: "i3-config".into(),
                resource_path: "i3-config".into(),
                target_path: ".config/i3/config".into(),
                is_executable: false,
            },
            ThemeConfigEntry {
                name: "i3status-config".into(),
                resource_path: "i3status-retro.conf".into(),
                target_path: ".config/i3/i3status-retro.conf".into(),
                is_executable: false,
            },
            ThemeConfigEntry {
                name: "kitty-config".into(),
                resource_path: "kitty.conf".into(),
                target_path: ".config/kitty/theme.conf".into(),
                is_executable: false,
            },
            ThemeConfigEntry {
                name: "conky-config".into(),
                resource_path: "conkyrc".into(),
                target_path: ".config/conky/retro-bbc.conkyrc".into(),
                is_executable: false,
            },
        ],
    }
}

/// Install the complete retro theme
pub fn install_retro_theme(base_path: &Path, dry_run: bool) -> Result<()> {
    get_retro_theme().install(base_path, dry_run)
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
            name: "test".to_string(),
            resource_path: source,
            target_path: target_clone,
            is_executable: false,
            strategy: DeployStrategy::Copy,
        };

        let result = install_theme_file(&config);
        assert!(result.is_ok());
        assert!(target.exists());
    }
}
