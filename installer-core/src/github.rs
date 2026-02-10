use anyhow::Result;
use std::process::Command;

use crate::{apt_repo, cmd, driver::RepoKind, package_manager, PhaseContext, PkgBackend};

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    install_git(ctx)?;
    install_gh(ctx)?;
    install_ssh_tools(ctx)?;
    remind_gh_auth_if_needed();
    print_ssh_notes();
    Ok(())
}

fn install_git(ctx: &mut PhaseContext) -> Result<()> {
    package_manager::ensure_packages(
        ctx.platform.driver,
        &["git", "git-lfs"],
        ctx.options.dry_run,
    )?;

    if !ctx.options.dry_run {
        let mut git_lfs = Command::new("git");
        git_lfs.args(["lfs", "install"]);
        let _ = cmd::run(&mut git_lfs);
    }
    Ok(())
}

fn install_gh(ctx: &mut PhaseContext) -> Result<()> {
    if which::which("gh").is_ok() {
        tracing::info!("GitHub CLI (gh) already installed");
        return Ok(());
    }

    match ctx.platform.pkg_backend {
        PkgBackend::Pacman => {
            // On Arch/Manjaro gh is `github-cli` in community
            package_manager::ensure_packages(ctx.platform.driver, &["gh"], ctx.options.dry_run)?;
        }
        PkgBackend::Apt => {
            apt_repo::ensure_repo(ctx, RepoKind::GitHubCli)?;
            install_gh_apt(ctx)?;
        }
    }

    Ok(())
}

fn install_gh_apt(ctx: &mut PhaseContext) -> Result<()> {
    package_manager::ensure_packages(ctx.platform.driver, &["gh"], false)?;
    Ok(())
}

fn install_ssh_tools(ctx: &mut PhaseContext) -> Result<()> {
    package_manager::ensure_packages(
        ctx.platform.driver,
        &["openssh-client"],
        ctx.options.dry_run,
    )?;
    Ok(())
}

fn remind_gh_auth_if_needed() {
    if which::which("gh").is_err() {
        return;
    }

    let mut cmd = Command::new("gh");
    cmd.args(["auth", "status", "-h", "github.com"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    if cmd::run(&mut cmd).is_err() {
        tracing::info!(
            "GitHub CLI is not authenticated. Run `gh auth login` (select SSH) when you need GitHub access."
        );
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
