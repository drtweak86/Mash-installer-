//! Nerd Fonts Management System
//!
//! This module provides comprehensive Nerd Fonts integration, allowing users to
//! select and install Nerd Fonts from the official GitHub repository.

use crate::system::cmd;
use crate::{package_manager, PhaseContext, PhaseResult};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

/// Available Nerd Fonts from the official repository
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NerdFont {
    pub name: String,
    pub display_name: String,
    pub filename: String,
    pub category: String,
}

impl NerdFont {
    /// Create a new NerdFont instance
    pub fn new(name: &str, display_name: &str, filename: &str, category: &str) -> Self {
        Self {
            name: name.to_string(),
            display_name: display_name.to_string(),
            filename: filename.to_string(),
            category: category.to_string(),
        }
    }
}

/// Nerd Fonts version constant
const NERD_FONT_VERSION: &str = "v3.3.0";

/// Get the list of available Nerd Fonts
pub fn available_fonts() -> Vec<NerdFont> {
    vec![
        // Mono fonts
        NerdFont::new(
            "JetBrainsMono",
            "JetBrains Mono",
            "JetBrainsMono.zip",
            "Mono",
        ),
        NerdFont::new("FiraCode", "Fira Code", "FiraCode.zip", "Mono"),
        NerdFont::new("Hack", "Hack", "Hack.zip", "Mono"),
        NerdFont::new(
            "SourceCodePro",
            "Source Code Pro",
            "SourceCodePro.zip",
            "Mono",
        ),
        NerdFont::new("UbuntuMono", "Ubuntu Mono", "UbuntuMono.zip", "Mono"),
        // Sans fonts
        NerdFont::new("FiraSans", "Fira Sans", "FiraSans.zip", "Sans"),
        NerdFont::new("Ubuntu", "Ubuntu", "Ubuntu.zip", "Sans"),
        NerdFont::new("RobotoMono", "Roboto Mono", "RobotoMono.zip", "Sans"),
        // Classic fonts
        NerdFont::new("Terminus", "Terminus", "Terminus.zip", "Classic"),
        NerdFont::new(
            "DejaVuSansMono",
            "DejaVu Sans Mono",
            "DejaVuSansMono.zip",
            "Classic",
        ),
        // Special fonts
        NerdFont::new(
            "CaskaydiaCove",
            "Caskaydia Cove",
            "CaskaydiaCove.zip",
            "Special",
        ),
        NerdFont::new(
            "DroidSansMono",
            "Droid Sans Mono",
            "DroidSansMono.zip",
            "Special",
        ),
    ]
}

/// Find a NerdFont by name
pub fn find_font_by_name(name: &str) -> Option<NerdFont> {
    available_fonts().into_iter().find(|f| f.name == name)
}

/// Install base terminus fonts via package manager
fn install_base_fonts(ctx: &mut PhaseContext) -> Result<()> {
    let pkgs = [
        "fonts-terminus",
        "fonts-noto-color-emoji",
        "xfonts-terminus",
    ];

    ctx.record_action("Installing base Terminus and Emoji fonts");
    package_manager::ensure_packages(ctx.platform.driver, &pkgs, ctx.options.dry_run)?;

    Ok(())
}

/// Install a specific Nerd Font
pub fn install_nerd_font(ctx: &mut PhaseContext, font: &NerdFont) -> Result<()> {
    let font_dir = dirs::home_dir()
        .unwrap_or_default()
        .join(".local/share/fonts");
    let target_font = font_dir.join(format!("{}-NerdFont-Regular.ttf", font.name));

    if target_font.exists() {
        tracing::info!("{} Nerd Font already installed.", font.display_name);
        return Ok(());
    }

    ctx.run_or_record(
        "fonts",
        format!("Install {} Nerd Font", font.display_name),
        Some(format!(
            "Downloading {} from GitHub Nerd Fonts release",
            font.display_name
        )),
        |_| {
            if ctx.options.dry_run {
                return Ok(());
            }

            fs::create_dir_all(&font_dir).context("Failed to create font directory")?;

            let url = format!(
                "https://github.com/ryanoasis/nerd-fonts/releases/download/{}/{}",
                NERD_FONT_VERSION, font.filename
            );

            let tmp_dir = tempfile::tempdir()?;
            let zip_path = tmp_dir.path().join(&font.filename);

            // Download
            let mut curl = Command::new("curl");
            curl.args(cmd::curl_flags());
            curl.arg("-o").arg(&zip_path).arg(&url);
            cmd::run(&mut curl).context(format!(
                "Failed to download {} Nerd Font",
                font.display_name
            ))?;

            // Unzip
            let mut unzip = Command::new("unzip");
            unzip.arg("-o").arg(&zip_path).arg("-d").arg(tmp_dir.path());
            cmd::run(&mut unzip)
                .context(format!("Failed to unzip {} Nerd Font", font.display_name))?;

            // Copy .ttf files to font_dir
            for entry in fs::read_dir(tmp_dir.path())? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("ttf") {
                    let dest = font_dir.join(path.file_name().unwrap());
                    fs::copy(&path, &dest).context("Failed to copy font file")?;
                }
            }

            // Update font cache
            let mut fc_cache = Command::new("fc-cache");
            fc_cache.arg("-f");
            let _ = cmd::run(&mut fc_cache); // Don't fail if fc-cache missing

            Ok(())
        },
    )?;

    Ok(())
}

/// Main font installation phase
pub fn install_phase(ctx: &mut PhaseContext) -> Result<PhaseResult> {
    // 1. Install base terminus fonts via package manager
    install_base_fonts(ctx)?;

    // 2. Install default JetBrainsMono Nerd Font (maintains backward compatibility)
    let default_font = NerdFont::new(
        "JetBrainsMono",
        "JetBrains Mono",
        "JetBrainsMono.zip",
        "Mono",
    );
    install_nerd_font(ctx, &default_font)?;

    Ok(PhaseResult::Success)
}

/// Get font by name for UI selection
pub fn get_font_by_name(name: &str) -> Option<NerdFont> {
    available_fonts().into_iter().find(|f| f.name == name)
}

/// Get all fonts grouped by category
pub fn get_fonts_by_category() -> std::collections::BTreeMap<String, Vec<NerdFont>> {
    let mut categories: std::collections::BTreeMap<String, Vec<NerdFont>> =
        std::collections::BTreeMap::new();

    for font in available_fonts() {
        categories
            .entry(font.category.clone())
            .or_default()
            .push(font);
    }

    categories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_fonts() {
        let fonts = available_fonts();
        assert!(!fonts.is_empty(), "Should have available fonts");
        assert!(fonts.len() >= 10, "Should have at least 10 fonts");
    }

    #[test]
    fn test_find_font_by_name() {
        let font = find_font_by_name("JetBrainsMono");
        assert!(font.is_some(), "Should find JetBrainsMono");
        assert_eq!(font.unwrap().display_name, "JetBrains Mono");
    }

    #[test]
    fn test_get_font_by_name() {
        let font = get_font_by_name("FiraCode");
        assert!(font.is_some(), "Should find FiraCode");
        assert_eq!(font.unwrap().display_name, "Fira Code");
    }

    #[test]
    fn test_get_fonts_by_category() {
        let categories = get_fonts_by_category();
        assert!(categories.contains_key("Mono"), "Should have Mono category");
        assert!(categories.contains_key("Sans"), "Should have Sans category");
        assert!(
            categories["Mono"].len() >= 3,
            "Mono should have at least 3 fonts"
        );
    }
}
