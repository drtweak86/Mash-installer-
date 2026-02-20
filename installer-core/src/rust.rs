use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::{cmd, PhaseContext};

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

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
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

fn install_rustup(ctx: &mut PhaseContext) -> Result<()> {
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
        ctx.record_dry_run(
            "rust_toolchain",
            "Would install rustup toolchain",
            Some("curl rustup.rs | sh -s -- -y".into()),
        );
        tracing::info!("[dry-run] curl rustup.rs | sh -s -- -y");
        return Ok(());
    }

    let mut install_cmd = Command::new("sh");
    install_cmd.arg("-c").arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable");
    cmd::run(&mut install_cmd).context("installing rustup")?;
    Ok(())
}

fn install_components(ctx: &mut PhaseContext) -> Result<()> {
    let components = ["rustfmt", "clippy", "rust-src"];
    for comp in &components {
        tracing::info!("Ensuring component: {comp}");
        if ctx.options.dry_run {
            ctx.record_dry_run(
                "rust_toolchain",
                "Would ensure rustup component",
                Some(comp.to_string()),
            );
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

fn ensure_cargo_binstall(ctx: &mut PhaseContext) -> Result<()> {
    if which::which("cargo-binstall").is_ok()
        || cargo_home().join("bin/cargo-binstall").exists()
    {
        tracing::info!("cargo-binstall already installed");
        return Ok(());
    }

    tracing::info!("Installing cargo-binstall (enables fast binary installs)");
    if ctx.options.dry_run {
        ctx.record_dry_run(
            "rust_toolchain",
            "Would install cargo-binstall",
            None,
        );
        return Ok(());
    }

    // Install cargo-binstall using the official installer script
    let mut install_cmd = Command::new("sh");
    install_cmd.arg("-c").arg(
        "curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash"
    );

    if let Err(err) = cmd::run(&mut install_cmd) {
        tracing::warn!("Failed to install cargo-binstall; will use slower cargo install: {err}");
        return Ok(()); // Not fatal, just slower
    }

    tracing::info!("cargo-binstall installed successfully!");
    Ok(())
}

fn install_cargo_tools(ctx: &mut PhaseContext) -> Result<()> {
    // First, try to install cargo-binstall for MUCH faster installs (uses pre-compiled binaries)
    ensure_cargo_binstall(ctx)?;

    let tools: &[(&str, &str)] = &[
        ("cargo-edit", "cargo-add"), // provides `cargo add`
        ("cargo-watch", "cargo-watch"),
        ("cargo-audit", "cargo-audit"),
        ("bacon", "bacon"),
        ("just", "just"),
        ("sccache", "sccache"),
    ];

    let use_binstall = which::which("cargo-binstall").is_ok()
        || cargo_home().join("bin/cargo-binstall").exists();

    if use_binstall {
        tracing::info!("Using cargo-binstall for fast binary installation! 🚀");
    } else {
        tracing::warn!("cargo-binstall not available; falling back to slow cargo install (this will take a while...)");
    }

    for (crate_name, bin_name) in tools {
        let bin_path = cargo_home().join("bin").join(bin_name);
        if bin_path.exists() || which::which(bin_name).is_ok() {
            tracing::info!("{bin_name} already installed");
            continue;
        }

        if ctx.options.dry_run {
            ctx.record_dry_run(
                "rust_toolchain",
                "Would install cargo tool",
                Some(crate_name.to_string()),
            );
            continue;
        }

        if use_binstall {
            // Use cargo-binstall (downloads pre-compiled binaries - MUCH faster!)
            tracing::info!("Installing {crate_name} via cargo-binstall (fast!)");
            let mut install_cmd = Command::new(cargo_bin());
            install_cmd.args(["binstall", "--no-confirm", crate_name]);
            if let Err(err) = cmd::run(&mut install_cmd) {
                tracing::warn!("cargo-binstall failed for {crate_name}, trying cargo install: {err}");
                // Fallback to cargo install
                let mut fallback_cmd = Command::new(cargo_bin());
                fallback_cmd.args(["install", crate_name]);
                if let Err(err2) = cmd::run(&mut fallback_cmd) {
                    tracing::warn!("Failed to install {crate_name}; continuing ({err2})");
                }
            }
        } else {
            // Fallback to cargo install (slow - compiles from source)
            tracing::info!("Installing {crate_name} via cargo install (this may take several minutes...)");
            let mut install_cmd = Command::new(cargo_bin());
            install_cmd.args(["install", crate_name]);
            if let Err(err) = cmd::run(&mut install_cmd) {
                tracing::warn!("Failed to install {crate_name}; continuing ({err})");
            }
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
            } else {
                ctx.record_dry_run("rust_toolchain", "Would install flamegraph", None);
            }
        }
    }

    Ok(())
}
