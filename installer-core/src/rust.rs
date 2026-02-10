use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::{cmd, PhaseExecutionContext};

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

pub fn install_phase(ctx: &PhaseExecutionContext) -> Result<()> {
    // 1. Install rustup + stable toolchain
    install_rustup(ctx)?;

    // 2. Components
    install_components(ctx)?;

    // 3. Cargo tools (dev+ profile)
    if ctx.options.profile >= crate::ProfileLevel::Dev {
        install_cargo_tools(ctx)?;
    }

    Ok(())
}

fn install_rustup(ctx: &PhaseExecutionContext) -> Result<()> {
    if has_rustup() {
        tracing::info!("rustup already installed; updating");
        if !ctx.options.dry_run {
            let mut update_cmd = Command::new(rustup_bin());
            update_cmd.arg("update");
            if let Err(err) = cmd::run(&mut update_cmd) {
                tracing::warn!("rustup update failed; continuing ({err})");
            }
        }
        return Ok(());
    }

    tracing::info!("Installing rustup + stable toolchain");
    if ctx.options.dry_run {
        tracing::info!("[dry-run] curl rustup.rs | sh -s -- -y");
        return Ok(());
    }

    let mut install_cmd = Command::new("sh");
    install_cmd.arg("-c").arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable");
    cmd::run(&mut install_cmd).context("installing rustup")?;
    Ok(())
}

fn install_components(ctx: &PhaseExecutionContext) -> Result<()> {
    let components = ["rustfmt", "clippy", "rust-src"];
    for comp in &components {
        tracing::info!("Ensuring component: {comp}");
        if ctx.options.dry_run {
            continue;
        }
        let mut comp_cmd = Command::new(rustup_bin());
        comp_cmd.args(["component", "add", comp]);
        if let Err(err) = cmd::run(&mut comp_cmd) {
            tracing::warn!("Failed to add component {comp}; continuing ({err})");
        }
    }
    Ok(())
}

fn install_cargo_tools(ctx: &PhaseExecutionContext) -> Result<()> {
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
        if ctx.options.dry_run {
            continue;
        }
        let mut install_cmd = Command::new(cargo_bin());
        install_cmd.args(["install", crate_name]);
        if let Err(err) = cmd::run(&mut install_cmd) {
            tracing::warn!("Failed to install {crate_name}; continuing ({err})");
        } else {
            tracing::info!("Installed {crate_name}");
        }
    }

    // flamegraph – only on full profile (needs perf which is tricky on Pi)
    if ctx.options.profile >= crate::ProfileLevel::Full {
        let bin_path = cargo_home().join("bin/flamegraph");
        if !bin_path.exists() && which::which("flamegraph").is_err() {
            tracing::info!("Installing flamegraph");
            if !ctx.options.dry_run {
                let mut flame_cmd = Command::new(cargo_bin());
                flame_cmd.args(["install", "flamegraph"]);
                if let Err(err) = cmd::run(&mut flame_cmd) {
                    tracing::warn!("Failed to install flamegraph; continuing ({err})");
                }
            }
        }
    }

    Ok(())
}
