use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::InstallContext;

fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/root"))
}

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    install_zsh(ctx)?;
    install_omz(ctx)?;
    install_starship(ctx)?;
    Ok(())
}

fn install_zsh(ctx: &InstallContext) -> Result<()> {
    crate::pkg::ensure_packages(&["zsh"], ctx.dry_run)?;
    Ok(())
}

fn install_omz(ctx: &InstallContext) -> Result<()> {
    let omz_dir = home_dir().join(".oh-my-zsh");
    if omz_dir.exists() {
        tracing::info!("oh-my-zsh already installed");
        return Ok(());
    }

    tracing::info!("Installing oh-my-zsh (unattended)");
    if ctx.dry_run {
        tracing::info!("[dry-run] would install oh-my-zsh");
        return Ok(());
    }

    let status = Command::new("sh")
        .arg("-c")
        .arg(
            r#"RUNZSH=no CHSH=no sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)""#,
        )
        .status()
        .context("installing oh-my-zsh")?;

    if !status.success() {
        tracing::warn!("oh-my-zsh installation returned non-zero; continuing");
    }
    Ok(())
}

fn install_starship(ctx: &InstallContext) -> Result<()> {
    if which::which("starship").is_ok() {
        tracing::info!("starship already installed");
        return Ok(());
    }

    tracing::info!("Installing starship prompt");
    if ctx.dry_run {
        tracing::info!("[dry-run] would install starship");
        return Ok(());
    }

    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -sS https://starship.rs/install.sh | sh -s -- -y")
        .status()
        .context("installing starship")?;

    if !status.success() {
        tracing::warn!("starship installation failed; continuing");
    }

    // Add starship init to .zshrc if not already there
    let zshrc = home_dir().join(".zshrc");
    if zshrc.exists() {
        let content = std::fs::read_to_string(&zshrc).unwrap_or_default();
        if !content.contains("starship init zsh") {
            let addition = "\n# Starship prompt\neval \"$(starship init zsh)\"\n";
            std::fs::write(&zshrc, format!("{content}{addition}"))?;
            tracing::info!("Added starship init to .zshrc");
        }
    }

    Ok(())
}
