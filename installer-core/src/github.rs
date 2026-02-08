use anyhow::Result;
use std::process::Command;

use crate::{apt_repo, driver::RepoKind, pkg::PkgBackend, InstallContext};

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    install_git(ctx)?;
    install_gh(ctx)?;
    install_ssh_tools(ctx)?;
    remind_gh_auth_if_needed();
    print_ssh_notes();
    Ok(())
}

fn install_git(ctx: &InstallContext) -> Result<()> {
    crate::pkg::ensure_packages(ctx.driver, &["git", "git-lfs"], ctx.dry_run)?;

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

    match ctx.pkg_backend {
        PkgBackend::Pacman => {
            // On Arch/Manjaro gh is `github-cli` in community
            crate::pkg::ensure_packages(ctx.driver, &["gh"], ctx.dry_run)?;
        }
        PkgBackend::Apt => {
            apt_repo::ensure_repo(ctx, RepoKind::GitHubCli)?;
            install_gh_apt(ctx)?;
        }
    }

    Ok(())
}

fn install_gh_apt(ctx: &InstallContext) -> Result<()> {
    crate::pkg::ensure_packages(ctx.driver, &["gh"], false)?;
    Ok(())
}

fn install_ssh_tools(ctx: &InstallContext) -> Result<()> {
    crate::pkg::ensure_packages(ctx.driver, &["openssh-client"], ctx.dry_run)?;
    Ok(())
}

fn remind_gh_auth_if_needed() {
    if which::which("gh").is_err() {
        return;
    }

    let status = Command::new("gh")
        .args(["auth", "status", "-h", "github.com"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    match status {
        Ok(s) if s.success() => {}
        _ => tracing::info!(
            "GitHub CLI is not authenticated. Run `gh auth login` (select SSH) when you need GitHub access."
        ),
    }
}

fn print_ssh_notes() {
    tracing::info!(
        "SSH note: This installer enforces SSH-based GitHub auth.\n  \
         Ensure you have an SSH key added to your GitHub account.\n  \
         Use `gh auth login` and select SSH to configure.\n  \
         The installer will never convert remotes to HTTPS."
    );
}
