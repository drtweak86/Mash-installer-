use anyhow::{Context, Result};
use std::process::Command;

use crate::pkg::PkgBackend;
use crate::InstallContext;

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    install_git(ctx)?;
    install_gh(ctx)?;
    install_ssh_tools(ctx)?;
    print_ssh_notes();
    Ok(())
}

fn install_git(ctx: &InstallContext) -> Result<()> {
    crate::pkg::ensure_packages(&["git", "git-lfs"], ctx.dry_run)?;

    if !ctx.dry_run {
        let _ = Command::new("git").args(["lfs", "install"]).status();
    }
    Ok(())
}

fn install_gh(ctx: &InstallContext) -> Result<()> {
    if which::which("gh").is_ok() {
        tracing::info!("GitHub CLI (gh) already installed");
        return Ok(());
    }

    let backend = crate::pkg::detect_backend();

    match backend {
        PkgBackend::Pacman => {
            // On Arch/Manjaro gh is `github-cli` in community
            crate::pkg::ensure_packages(&["gh"], ctx.dry_run)?;
        }
        PkgBackend::Apt => {
            install_gh_apt(ctx)?;
        }
    }

    Ok(())
}

fn install_gh_apt(ctx: &InstallContext) -> Result<()> {
    tracing::info!("Adding GitHub CLI repository");
    if ctx.dry_run {
        tracing::info!("[dry-run] would add gh apt repo and install gh");
        return Ok(());
    }

    let keyring = "/etc/apt/keyrings/githubcli-archive-keyring.gpg";
    if !std::path::Path::new(keyring).exists() {
        let status = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo tee {keyring} > /dev/null && sudo chmod go+r {keyring}"
            ))
            .status()
            .context("adding gh GPG key")?;
        if !status.success() {
            anyhow::bail!("Failed to add GitHub CLI GPG key");
        }
    }

    let sources = "/etc/apt/sources.list.d/github-cli-stable.list";
    if !std::path::Path::new(sources).exists() {
        let arch_out = Command::new("dpkg").arg("--print-architecture").output()?;
        let arch = String::from_utf8_lossy(&arch_out.stdout).trim().to_string();

        let line = format!(
            "deb [arch={arch} signed-by={keyring}] https://cli.github.com/packages stable main"
        );
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo '{line}' | sudo tee {sources} > /dev/null"))
            .status()?;

        crate::pkg::update(false)?;
    }

    crate::pkg::ensure_packages(&["gh"], false)?;
    Ok(())
}

fn install_ssh_tools(ctx: &InstallContext) -> Result<()> {
    crate::pkg::ensure_packages(&["openssh-client"], ctx.dry_run)?;
    Ok(())
}

fn print_ssh_notes() {
    tracing::info!(
        "SSH note: This installer enforces SSH-based GitHub auth.\n  \
         Ensure you have an SSH key added to your GitHub account.\n  \
         Use `gh auth login` and select SSH to configure.\n  \
         The installer will never convert remotes to HTTPS."
    );
}
