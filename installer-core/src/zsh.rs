use anyhow::{Context, Result};
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::{cmd, options::ProfileLevel, package_manager, PhaseContext, PkgBackend};
use which::which;

fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/root"))
}

/// System-wide install path for Powerlevel10k.
const P10K_SYSTEM_DIR: &str = "/usr/share/powerlevel10k";
/// The theme file that gets sourced in .zshrc.
const P10K_THEME_FILE: &str = "/usr/share/powerlevel10k/powerlevel10k.zsh-theme";
const P10K_TAG: &str = "v1.13.0";

const STARSHIP_CONFIG: &str = include_str!("../../resources/shell/starship.toml");
const KITTY_CONFIG: &str = include_str!("../../resources/shell/kitty.conf");
const EZA_ALIASES_SCRIPT: &str = include_str!("../../resources/shell/eza_aliases.sh");
const STARSHIP_MARKER: &str = "starship init zsh";
const STARSHIP_BLOCK: &str = r#"
# Starship prompt (added by mash-installer)
if command -v starship >/dev/null; then
  eval "$(starship init zsh)"
fi
"#;
const EZA_MARKER: &str = ".eza_aliases";
const EZA_BLOCK: &str = r#"
# Goblin eza aliases (added by mash-installer)
if [ -f "$HOME/.eza_aliases" ]; then
  source "$HOME/.eza_aliases"
fi
"#;

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    install_zsh(ctx)?;
    install_omz(ctx)?;
    install_starship(ctx)?;
    deploy_starship_config(ctx)?;
    ensure_starship_init(ctx)?;
    install_kitty_config(ctx)?;
    install_eza_aliases(ctx)?;

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

    let omz_dir_clone = omz_dir.clone();
    ctx.register_rollback_action("remove oh-my-zsh directory", move || {
        if omz_dir_clone.exists() {
            std::fs::remove_dir_all(&omz_dir_clone)?;
        }
        Ok(())
    });

    if let Err(err) = cmd::Command::new("sh")
        .arg("-c")
        .arg(
            r#"RUNZSH=no CHSH=no sh -c "$(curl -fsSL --proto '=https' --tlsv1.2 https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)""#,
        )
        .execute()
    {
        ctx.record_warning(format!("oh-my-zsh installation returned non-zero ({err})"));
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
        PkgBackend::Apt | PkgBackend::Dnf => {
            // Not in standard Ubuntu/Debian/Fedora repos
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
    writeln!(block)?;
    writeln!(
        block,
        "# Powerlevel10k prompt theme (added by mash-installer)"
    )?;
    writeln!(block, "if [ -f {pacman_theme} ]; then")?;
    writeln!(block, "  source {pacman_theme}")?;
    writeln!(block, "elif [ -f {P10K_THEME_FILE} ]; then")?;
    writeln!(block, "  source {P10K_THEME_FILE}")?;
    writeln!(block, "fi")?;

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

fn install_starship(ctx: &mut PhaseContext) -> Result<()> {
    if which("starship").is_ok() {
        tracing::info!("Starship prompt already installed");
        return Ok(());
    }

    if ctx.options.dry_run {
        ctx.record_action("Would install the Starship prompt");
        return Ok(());
    }

    tracing::info!("Installing Starship prompt via official installer");
    cmd::Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL --proto '=https' --tlsv1.2 https://starship.rs/install.sh | sh -s -- -y")
        .execute()
        .context("while installing Starship prompt")?;

    ctx.record_action("Installed Starship prompt");
    Ok(())
}

fn deploy_starship_config(ctx: &mut PhaseContext) -> Result<()> {
    let path = home_dir().join(".config/starship/starship.toml");
    write_config_file(ctx, &path, STARSHIP_CONFIG, "Starship configuration")
}

fn ensure_starship_init(ctx: &mut PhaseContext) -> Result<()> {
    let zshrc = home_dir().join(".zshrc");
    ensure_block_in_rc(&zshrc, STARSHIP_MARKER, STARSHIP_BLOCK, ctx)
}

fn install_kitty_config(ctx: &mut PhaseContext) -> Result<()> {
    if ctx.options.profile < ProfileLevel::Dev {
        tracing::info!("Skipping Kitty configuration for non-Dev profile");
        return Ok(());
    }

    let path = home_dir().join(".config/kitty/kitty.conf");
    write_config_file(ctx, &path, KITTY_CONFIG, "Kitty configuration")
}

fn install_eza_aliases(ctx: &mut PhaseContext) -> Result<()> {
    if !ctx.platform.driver.is_package_installed("eza") {
        tracing::info!("Skipping eza aliases because eza is not installed");
        return Ok(());
    }

    let alias_path = home_dir().join(".eza_aliases");
    write_config_file(ctx, &alias_path, EZA_ALIASES_SCRIPT, "eza aliases")?;

    let zshrc = home_dir().join(".zshrc");
    ensure_block_in_rc(&zshrc, EZA_MARKER, EZA_BLOCK, ctx)?;

    let bashrc = home_dir().join(".bashrc");
    ensure_block_in_rc(&bashrc, EZA_MARKER, EZA_BLOCK, ctx)
}

fn write_config_file(
    ctx: &mut PhaseContext,
    path: &Path,
    contents: &str,
    description: &str,
) -> Result<()> {
    if ctx.options.dry_run {
        ctx.record_action(format!("Would write {description} to {}", path.display()));
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(path, contents)?;
    ctx.record_action(format!("Wrote {description} to {}", path.display()));
    Ok(())
}

fn ensure_block_in_rc(
    path: &Path,
    marker: &str,
    block: &str,
    ctx: &mut PhaseContext,
) -> Result<()> {
    let content = if path.exists() {
        std::fs::read_to_string(path)?
    } else {
        String::new()
    };

    if content.contains(marker) {
        tracing::info!("Shell fragment already present in {}", path.display());
        return Ok(());
    }

    if ctx.options.dry_run {
        ctx.record_action(format!("Would append shell fragment to {}", path.display()));
        return Ok(());
    }

    if path.exists() {
        backup_file(path)?;
        let mut file = OpenOptions::new().append(true).open(path)?;
        writeln!(file)?;
        writeln!(file, "{block}")?;
    } else {
        std::fs::write(path, block)?;
    }

    ctx.record_action(format!("Appended shell fragment to {}", path.display()));
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
