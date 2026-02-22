use crate::{cmd, package_manager, PhaseContext};
use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    // 1. Install base terminus fonts via package manager
    let pkgs = [
        "fonts-terminus",
        "fonts-noto-color-emoji",
        "xfonts-terminus",
    ];

    ctx.record_action("Installing base Terminus and Emoji fonts");
    package_manager::ensure_packages(ctx.platform.driver, &pkgs, ctx.options.dry_run)?;

    // 2. Install Terminess Nerd Font manually if not present
    install_terminess_nerd_font(ctx)?;

    Ok(())
}

fn install_terminess_nerd_font(ctx: &mut PhaseContext) -> Result<()> {
    let font_dir = dirs::home_dir()
        .unwrap_or_default()
        .join(".local/share/fonts");
    let target_font = font_dir.join("TerminessNerdFont-Regular.ttf");

    if target_font.exists() {
        tracing::info!("Terminess Nerd Font already installed.");
        return Ok(());
    }

    ctx.run_or_record(
        "fonts",
        "Install Terminess Nerd Font",
        Some("Downloading from GitHub Nerd Fonts release".into()),
        |_| {
            if ctx.options.dry_run {
                return Ok(());
            }

            fs::create_dir_all(&font_dir).context("Failed to create font directory")?;

            let version = "v3.2.1";
            let font_name = "Terminus.zip";
            let url = format!(
                "https://github.com/ryanoasis/nerd-fonts/releases/download/{}/{}",
                version, font_name
            );

            let tmp_dir = tempfile::tempdir()?;
            let zip_path = tmp_dir.path().join(font_name);

            // Download
            let mut curl = Command::new("curl");
            curl.args(cmd::curl_flags());
            curl.arg("-o").arg(&zip_path).arg(&url);
            cmd::run(&mut curl).context("Failed to download Terminess Nerd Font")?;

            // Unzip
            let mut unzip = Command::new("unzip");
            unzip.arg("-o").arg(&zip_path).arg("-d").arg(tmp_dir.path());
            cmd::run(&mut unzip).context("Failed to unzip Terminess Nerd Font")?;

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
    )
}
