use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use crate::{
    cmd,
    driver::{AptRepoConfig, RepoKind},
    package_manager, InstallContext,
};

/// Ensure the named apt repository is configured according to the distro driver.
pub fn ensure_repo(ctx: &InstallContext, repo: RepoKind) -> Result<()> {
    let config = match ctx.platform.driver.apt_repo_config(repo) {
        Some(cfg) => cfg,
        None => return Ok(()),
    };

    tracing::info!("Ensuring apt repository: {}", config.label);
    if ctx.options.dry_run {
        tracing::info!(
            "[dry-run] would configure {} apt repo at {}",
            config.label,
            config.sources_path
        );
        return Ok(());
    }

    add_gpg_key(&config, ctx)?;
    if add_sources_list(&config, ctx)? {
        package_manager::update(ctx.platform.driver, false)?;
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
        let mut cmd = Command::new("sudo");
        cmd.args(["install", "-m", "0755", "-d", dir.as_ref()]);
        cmd::run(&mut cmd).context("creating apt keyring directory")?;
    }

    let key_url = (config.key_url)(&ctx.platform.platform)?;
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(format!(
        "curl -fsSL {key_url} | sudo tee {path} > /dev/null && sudo chmod go+r {path}",
        path = config.key_path
    ));
    cmd::run(&mut cmd).context("downloading apt repo GPG key")?;

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

    let repo_line = (config.repo_line)(&ctx.platform.platform)?;
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(format!(
        "echo '{repo_line}' | sudo tee {path} > /dev/null",
        path = config.sources_path
    ));
    cmd::run(&mut cmd).context("writing apt sources list")?;

    Ok(true)
}
