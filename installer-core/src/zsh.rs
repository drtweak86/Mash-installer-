use anyhow::{Context, Result};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::{cmd, package_manager, PhaseContext, PkgBackend};

fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/root"))
}

/// System-wide install path for Powerlevel10k.
const P10K_SYSTEM_DIR: &str = "/usr/share/powerlevel10k";
/// The theme file that gets sourced in .zshrc.
const P10K_THEME_FILE: &str = "/usr/share/powerlevel10k/powerlevel10k.zsh-theme";
const P10K_TAG: &str = "v1.13.0";

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    install_zsh(ctx)?;
    install_omz(ctx)?;

    if ctx.options.enable_p10k {
        install_p10k(ctx)?;
    } else {
        tracing::info!("Powerlevel10k skipped (pass --enable-p10k to install)");
    }

    Ok(())
}

fn install_zsh(ctx: &mut PhaseContext) -> Result<()> {
    package_manager::ensure_packages(ctx.platform.driver, &["zsh"], ctx.options.dry_run)?;
    Ok(())
}

fn install_omz(ctx: &mut PhaseContext) -> Result<()> {
    let omz_dir = home_dir().join(".oh-my-zsh");
    if omz_dir.exists() {
        tracing::info!("oh-my-zsh already installed");
        return Ok(());
    }

    tracing::info!("Installing oh-my-zsh (unattended)");
    if ctx.options.dry_run {
        tracing::info!("[dry-run] would install oh-my-zsh");
        return Ok(());
    }

    if let Err(err) = cmd::Command::new("sh")
        .arg("-c")
        .arg(
            r#"RUNZSH=no CHSH=no sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)""#,
        )
        .execute()
    {
        tracing::warn!("oh-my-zsh installation returned non-zero; continuing ({err})");
    }
    Ok(())
}

// ── Powerlevel10k ───────────────────────────────────────────────

fn install_p10k(ctx: &mut PhaseContext) -> Result<()> {
    // 1. Try system package manager first (Arch has zsh-theme-powerlevel10k)
    if try_p10k_pkg(ctx)? {
        add_p10k_source_to_zshrc(ctx)?;
        return Ok(());
    }

    // 2. Git-clone fallback to system-wide location
    install_p10k_git(ctx)?;
    add_p10k_source_to_zshrc(ctx)?;
    Ok(())
}

/// Try installing Powerlevel10k via the system package manager.
/// Returns true if it succeeded (or was already installed).
fn try_p10k_pkg(ctx: &mut PhaseContext) -> Result<bool> {
    match ctx.platform.pkg_backend {
        PkgBackend::Pacman => {
            // Manjaro/Arch: available as `zsh-theme-powerlevel10k`
            if package_manager::is_installed(ctx.platform.driver, "zsh-theme-powerlevel10k")
                || Path::new("/usr/share/zsh-theme-powerlevel10k").exists()
            {
                tracing::info!("Powerlevel10k already installed via package manager");
                return Ok(true);
            }
            tracing::info!("Attempting Powerlevel10k install via pacman");
            if ctx.options.dry_run {
                tracing::info!("[dry-run] would install zsh-theme-powerlevel10k");
                return Ok(true);
            }
            match package_manager::ensure_packages(
                ctx.platform.driver,
                &["zsh-theme-powerlevel10k"],
                false,
            ) {
                Ok(()) => {
                    tracing::info!("Installed Powerlevel10k via pacman");
                    Ok(true)
                }
                Err(_) => {
                    tracing::info!(
                        "zsh-theme-powerlevel10k not in repos; falling back to git clone"
                    );
                    Ok(false)
                }
            }
        }
        PkgBackend::Apt => {
            // Not in standard Ubuntu/Debian repos
            Ok(false)
        }
    }
}

/// Clone Powerlevel10k into the system-wide directory.
fn install_p10k_git(ctx: &mut PhaseContext) -> Result<()> {
    let dest = Path::new(P10K_SYSTEM_DIR);

    if dest.exists() {
        tracing::info!("Powerlevel10k already present at {}", dest.display());
        return Ok(());
    }

    tracing::info!("Cloning Powerlevel10k to {}", dest.display());
    if ctx.options.dry_run {
        tracing::info!(
            "[dry-run] would git clone powerlevel10k to {}",
            dest.display()
        );
        return Ok(());
    }

    cmd::Command::new("sudo")
        .args([
            "git",
            "clone",
            "--depth=1",
            "--branch",
            P10K_TAG,
            "--single-branch",
            "https://github.com/romkatv/powerlevel10k.git",
            P10K_SYSTEM_DIR,
        ])
        .execute()
        .context("cloning powerlevel10k")?;

    tracing::info!("Powerlevel10k installed to {}", dest.display());
    Ok(())
}

/// Add a guarded source block to the user's .zshrc if not already present.
/// Backs up the file before modifying.
fn add_p10k_source_to_zshrc(ctx: &mut PhaseContext) -> Result<()> {
    let zshrc = home_dir().join(".zshrc");

    // Determine the theme file path.
    // Arch pacman installs to /usr/share/zsh-theme-powerlevel10k/;
    // our git-clone installs to /usr/share/powerlevel10k/.
    let pacman_theme = "/usr/share/zsh-theme-powerlevel10k/powerlevel10k.zsh-theme";
    let source_marker = "powerlevel10k.zsh-theme";

    if zshrc.exists() {
        let content = std::fs::read_to_string(&zshrc).unwrap_or_default();
        if content.contains(source_marker) {
            tracing::info!("Powerlevel10k source already present in .zshrc");
            return Ok(());
        }
    }

    // Build a guarded source block that checks both possible locations
    let mut block = String::new();
    writeln!(block).unwrap();
    writeln!(
        block,
        "# Powerlevel10k prompt theme (added by mash-installer)"
    )
    .unwrap();
    writeln!(block, "if [ -f {pacman_theme} ]; then").unwrap();
    writeln!(block, "  source {pacman_theme}").unwrap();
    writeln!(block, "elif [ -f {P10K_THEME_FILE} ]; then").unwrap();
    writeln!(block, "  source {P10K_THEME_FILE}").unwrap();
    writeln!(block, "fi").unwrap();

    if ctx.options.dry_run {
        tracing::info!("[dry-run] would append Powerlevel10k source block to .zshrc");
        return Ok(());
    }

    if zshrc.exists() {
        backup_file(&zshrc)?;
        let content = std::fs::read_to_string(&zshrc).unwrap_or_default();
        std::fs::write(&zshrc, format!("{content}{block}"))?;
    } else {
        std::fs::write(&zshrc, block)?;
    }

    tracing::info!("Added Powerlevel10k source block to .zshrc");
    Ok(())
}

// ── Helpers ─────────────────────────────────────────────────────

/// Create a timestamped .bak copy of a file before modifying it.
fn backup_file(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let backup = path.with_extension(format!("bak.{ts}"));
    std::fs::copy(path, &backup)
        .with_context(|| format!("backing up {} to {}", path.display(), backup.display()))?;
    tracing::info!("Backed up {} → {}", path.display(), backup.display());
    Ok(())
}
