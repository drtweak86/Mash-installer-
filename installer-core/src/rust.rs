use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::InstallContext;

/// Check if rustup is installed for the current user.
fn has_rustup() -> bool {
    which::which("rustup").is_ok() || cargo_home().join("bin/rustup").exists()
}

fn cargo_home() -> PathBuf {
    std::env::var("CARGO_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::home_dir().unwrap_or_default().join(".cargo"))
}

fn rustup_bin() -> PathBuf {
    which::which("rustup").unwrap_or_else(|_| cargo_home().join("bin/rustup"))
}

fn cargo_bin() -> PathBuf {
    which::which("cargo").unwrap_or_else(|_| cargo_home().join("bin/cargo"))
}

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    // 1. Install rustup + stable toolchain
    install_rustup(ctx)?;

    // 2. Components
    install_components(ctx)?;

    // 3. Cargo tools (dev+ profile)
    if ctx.profile >= crate::ProfileLevel::Dev {
        install_cargo_tools(ctx)?;
    }

    Ok(())
}

fn install_rustup(ctx: &InstallContext) -> Result<()> {
    if has_rustup() {
        tracing::info!("rustup already installed; updating");
        if !ctx.dry_run {
            let _ = Command::new(rustup_bin()).arg("update").status();
        }
        return Ok(());
    }

    tracing::info!("Installing rustup + stable toolchain");
    if ctx.dry_run {
        tracing::info!("[dry-run] curl rustup.rs | sh -s -- -y");
        return Ok(());
    }

    let status = Command::new("sh")
        .arg("-c")
        .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable")
        .status()
        .context("installing rustup")?;

    if !status.success() {
        anyhow::bail!("rustup installation failed");
    }
    Ok(())
}

fn install_components(ctx: &InstallContext) -> Result<()> {
    let components = ["rustfmt", "clippy", "rust-src"];
    for comp in &components {
        tracing::info!("Ensuring component: {comp}");
        if ctx.dry_run {
            continue;
        }
        let _ = Command::new(rustup_bin())
            .args(["component", "add", comp])
            .status();
    }
    Ok(())
}

fn install_cargo_tools(ctx: &InstallContext) -> Result<()> {
    let tools: &[(&str, &str)] = &[
        ("cargo-edit", "cargo-add"), // provides `cargo add`
        ("cargo-watch", "cargo-watch"),
        ("cargo-audit", "cargo-audit"),
        ("bacon", "bacon"),
        ("just", "just"),
        ("sccache", "sccache"),
    ];

    for (crate_name, bin_name) in tools {
        let bin_path = cargo_home().join("bin").join(bin_name);
        if bin_path.exists() || which::which(bin_name).is_ok() {
            tracing::info!("{bin_name} already installed");
            continue;
        }
        tracing::info!("Installing {crate_name} via cargo install");
        if ctx.dry_run {
            continue;
        }
        let status = Command::new(cargo_bin())
            .args(["install", crate_name])
            .status();
        match status {
            Ok(s) if s.success() => tracing::info!("Installed {crate_name}"),
            _ => tracing::warn!("Failed to install {crate_name}; continuing"),
        }
    }

    // flamegraph – only on full profile (needs perf which is tricky on Pi)
    if ctx.profile >= crate::ProfileLevel::Full {
        let bin_path = cargo_home().join("bin/flamegraph");
        if !bin_path.exists() && which::which("flamegraph").is_err() {
            tracing::info!("Installing flamegraph");
            if !ctx.dry_run {
                let _ = Command::new(cargo_bin())
                    .args(["install", "flamegraph"])
                    .status();
            }
        }
    }

    Ok(())
}
