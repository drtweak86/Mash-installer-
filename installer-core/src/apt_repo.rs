use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

use crate::{
    driver::{AptRepoConfig, RepoKind},
    pkg, InstallContext,
};

/// Ensure the named apt repository is configured according to the distro driver.
pub fn ensure_repo(ctx: &InstallContext, repo: RepoKind) -> Result<()> {
    let config = match ctx.driver.apt_repo_config(repo) {
        Some(cfg) => cfg,
        None => return Ok(()),
    };

    tracing::info!("Ensuring apt repository: {}", config.label);
    if ctx.dry_run {
        tracing::info!(
            "[dry-run] would configure {} apt repo at {}",
            config.label,
            config.sources_path
        );
        return Ok(());
    }

    add_gpg_key(&config, ctx)?;
    if add_sources_list(&config, ctx)? {
        pkg::update(ctx.driver, false)?;
    }

    Ok(())
}

fn add_gpg_key(config: &AptRepoConfig, ctx: &InstallContext) -> Result<()> {
    let key_path = Path::new(config.key_path);
    if key_path.exists() {
        return Ok(());
    }

    if let Some(parent) = key_path.parent() {
        let dir = parent.to_string_lossy();
        Command::new("sudo")
            .args(["install", "-m", "0755", "-d", dir.as_ref()])
            .status()
            .context("creating apt keyring directory")?;
    }

    let key_url = (config.key_url)(&ctx.platform)?;
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "curl -fsSL {key_url} | sudo tee {path} > /dev/null && sudo chmod go+r {path}",
            path = config.key_path
        ))
        .status()
        .context("downloading apt repo GPG key")?;

    if !status.success() {
        bail!("Failed to download GPG key for {}", config.label);
    }

    Ok(())
}

fn add_sources_list(config: &AptRepoConfig, ctx: &InstallContext) -> Result<bool> {
    let sources_path = Path::new(config.sources_path);
    if sources_path.exists() {
        return Ok(false);
    }

    if let Some(parent) = sources_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let repo_line = (config.repo_line)(&ctx.platform)?;
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo '{repo_line}' | sudo tee {path} > /dev/null",
            path = config.sources_path
        ))
        .status()
        .context("writing apt sources list")?;

    if !cmd.success() {
        bail!("Failed to write sources list for {}", config.label);
    }

    Ok(true)
}
