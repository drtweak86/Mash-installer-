use anyhow::{Context, Result};
use std::process::Command;

use crate::InstallContext;

// ── Package-manager detection ───────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PkgBackend {
    Apt,
    Pacman,
}

/// Auto-detect the system package manager.
pub fn detect_backend() -> PkgBackend {
    if which::which("pacman").is_ok() {
        PkgBackend::Pacman
    } else {
        PkgBackend::Apt
    }
}

// ── Debian → Arch package-name translation ──────────────────────

/// Translate a canonical (Debian) package name to its Arch equivalent.
/// Returns `None` when the package should be skipped entirely on Arch.
fn translate_for_arch(debian: &str) -> Option<&str> {
    match debian {
        // ── skip on Arch (not applicable) ──
        "software-properties-common" | "apt-transport-https" => None,
        "lsb-release" => None,
        "python3-venv" => None, // included in python on Arch

        // ── renamed ──
        "build-essential" => Some("base-devel"),
        "pkg-config" => Some("pkgconf"),
        "ninja-build" => Some("ninja"),
        "g++" => None, // part of gcc on Arch
        "xz-utils" => Some("xz"),
        "python3" => Some("python"),
        "python3-pip" => Some("python-pip"),
        "fd-find" => Some("fd"),
        "libncurses-dev" => Some("ncurses"),
        "libssl-dev" => Some("openssl"),
        "openssh-client" => Some("openssh"),
        "fonts-terminus" => Some("terminus-font"),
        "fonts-noto-color-emoji" => Some("noto-fonts-emoji"),
        "xfonts-terminus" => None, // X11 bitmap; not relevant on Arch

        // ── Docker (Arch community packages) ──
        "docker-ce" => Some("docker"),
        "docker-ce-cli" => None, // part of `docker`
        "containerd.io" => None, // pulled as dependency
        "docker-buildx-plugin" => Some("docker-buildx"),
        "docker-compose-plugin" => Some("docker-compose"),

        // ── GitHub CLI ──
        "gh" => Some("github-cli"),

        // ── same name on both ──
        _ => Some(debian),
    }
}

/// Map a slice of canonical (Debian) names to the native package names
/// for the active backend.  Drops packages that should be skipped.
fn translate_names(pkgs: &[&str], backend: PkgBackend) -> Vec<String> {
    match backend {
        PkgBackend::Apt => pkgs.iter().map(|s| s.to_string()).collect(),
        PkgBackend::Pacman => pkgs
            .iter()
            .filter_map(|p| translate_for_arch(p))
            .map(|s| s.to_string())
            .collect(),
    }
}

// ── Public helpers (backend-agnostic) ───────────────────────────

/// Check whether a package is already installed.
pub fn is_installed(pkg: &str) -> bool {
    match detect_backend() {
        PkgBackend::Apt => apt_is_installed(pkg),
        PkgBackend::Pacman => {
            let native = translate_for_arch(pkg).unwrap_or(pkg);
            pacman_is_installed(native)
        }
    }
}

/// Refresh the package database.
pub fn update(dry_run: bool) -> Result<()> {
    match detect_backend() {
        PkgBackend::Apt => apt_update(dry_run),
        PkgBackend::Pacman => pacman_sync(dry_run),
    }
}

/// Install a list of packages idempotently.
/// Names are given in Debian-canonical form; they are translated
/// automatically on Arch.
pub fn ensure_packages(pkgs: &[&str], dry_run: bool) -> Result<()> {
    let backend = detect_backend();
    let native = translate_names(pkgs, backend);
    let native_refs: Vec<&str> = native.iter().map(String::as_str).collect();

    match backend {
        PkgBackend::Apt => apt_ensure(&native_refs, dry_run),
        PkgBackend::Pacman => pacman_ensure(&native_refs, dry_run),
    }
}

/// Best-effort install of a single optional package (never fatal).
pub fn try_optional(pkg: &str, dry_run: bool) {
    let backend = detect_backend();
    let native = match backend {
        PkgBackend::Apt => pkg.to_string(),
        PkgBackend::Pacman => match translate_for_arch(pkg) {
            Some(n) => n.to_string(),
            None => return, // skip on Arch
        },
    };

    if is_installed(pkg) {
        return;
    }
    if dry_run {
        tracing::info!("[dry-run] would attempt optional: {native}");
        return;
    }

    let status = match backend {
        PkgBackend::Apt => Command::new("sudo")
            .args(["apt-get", "install", "-y", "--install-recommends", &native])
            .env("DEBIAN_FRONTEND", "noninteractive")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status(),
        PkgBackend::Pacman => Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "--needed", &native])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status(),
    };

    match status {
        Ok(s) if s.success() => tracing::info!("Installed optional package: {native}"),
        _ => tracing::warn!("Optional package '{native}' not available; skipping"),
    }
}

// ── APT backend ─────────────────────────────────────────────────

fn apt_is_installed(pkg: &str) -> bool {
    Command::new("dpkg")
        .args(["-s", pkg])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn apt_update(dry_run: bool) -> Result<()> {
    if dry_run {
        tracing::info!("[dry-run] apt-get update");
        return Ok(());
    }
    let status = Command::new("sudo")
        .args(["apt-get", "update", "-qq"])
        .status()
        .context("running apt-get update")?;
    if !status.success() {
        anyhow::bail!("apt-get update failed");
    }
    Ok(())
}

fn apt_ensure(pkgs: &[&str], dry_run: bool) -> Result<()> {
    let missing: Vec<&str> = pkgs
        .iter()
        .copied()
        .filter(|p| !apt_is_installed(p))
        .collect();
    if missing.is_empty() {
        tracing::info!("All packages already installed");
        return Ok(());
    }
    tracing::info!(
        "Installing {} packages: {}",
        missing.len(),
        missing.join(", ")
    );

    if dry_run {
        tracing::info!("[dry-run] would install: {}", missing.join(" "));
        return Ok(());
    }

    let mut cmd = Command::new("sudo");
    cmd.args(["apt-get", "install", "-y", "--install-recommends"]);
    for p in &missing {
        cmd.arg(p);
    }

    let status = cmd
        .env("DEBIAN_FRONTEND", "noninteractive")
        .status()
        .context("running apt-get install")?;

    if !status.success() {
        anyhow::bail!("apt-get install failed for: {}", missing.join(" "));
    }
    Ok(())
}

// ── Pacman backend ──────────────────────────────────────────────

fn pacman_is_installed(pkg: &str) -> bool {
    // `pacman -Q pkg` exits 0 if installed; also handles group members
    Command::new("pacman")
        .args(["-Q", pkg])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn pacman_sync(dry_run: bool) -> Result<()> {
    if dry_run {
        tracing::info!("[dry-run] pacman -Syu");
        return Ok(());
    }
    let status = Command::new("sudo")
        .args(["pacman", "-Syu", "--noconfirm"])
        .status()
        .context("running pacman -Syu")?;
    if !status.success() {
        anyhow::bail!("pacman -Syu failed");
    }
    Ok(())
}

fn pacman_ensure(pkgs: &[&str], dry_run: bool) -> Result<()> {
    if pkgs.is_empty() {
        return Ok(());
    }
    // `--needed` makes this idempotent – already-installed packages are skipped.
    tracing::info!("Ensuring packages via pacman: {}", pkgs.join(", "));

    if dry_run {
        tracing::info!("[dry-run] would install: {}", pkgs.join(" "));
        return Ok(());
    }

    let mut cmd = Command::new("sudo");
    cmd.args(["pacman", "-S", "--noconfirm", "--needed"]);
    for p in pkgs {
        cmd.arg(p);
    }

    let status = cmd.status().context("running pacman -S")?;
    if !status.success() {
        anyhow::bail!("pacman -S failed for: {}", pkgs.join(" "));
    }
    Ok(())
}

// ── Phase 1: core packages ─────────────────────────────────────

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    update(ctx.dry_run)?;

    // Always-needed core packages (Debian canonical names)
    let mut pkgs: Vec<&str> = vec![
        "ca-certificates",
        "curl",
        "wget",
        "xz-utils",
        "tar",
        "coreutils",
        "jq",
        "git",
        "software-properties-common",
        "gnupg",
        "lsb-release",
        "apt-transport-https",
    ];

    // Build essentials (all profiles)
    pkgs.extend_from_slice(&[
        "build-essential",
        "pkg-config",
        "clang",
        "lld",
        "cmake",
        "ninja-build",
        "gcc",
        "g++",
        "gdb",
        "make",
    ]);

    // Dev+ packages
    if ctx.profile >= crate::ProfileLevel::Dev {
        pkgs.extend_from_slice(&[
            "python3",
            "python3-pip",
            "python3-venv",
            "ripgrep",
            "fd-find",
            "fzf",
            "tmux",
            "htop",
            "ncdu",
            "neovim",
        ]);
    }

    // Full profile extras
    if ctx.profile >= crate::ProfileLevel::Full {
        pkgs.extend_from_slice(&["nodejs", "npm"]);
    }

    // Optional packages – may not exist in every distro version
    let optional = ["btop", "eza", "yq", "lldb", "bat"];

    // Split required vs optional
    let required: Vec<&str> = pkgs
        .iter()
        .copied()
        .filter(|p| !optional.contains(p))
        .collect();

    ensure_packages(&required, ctx.dry_run)?;

    // Always attempt lldb
    try_optional("lldb", ctx.dry_run);

    // Dev+ optional packages
    if ctx.profile >= crate::ProfileLevel::Dev {
        for pkg in &["btop", "bat", "eza", "yq"] {
            try_optional(pkg, ctx.dry_run);
        }
    }

    Ok(())
}
