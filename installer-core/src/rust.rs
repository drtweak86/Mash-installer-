//! Rust toolchain installation and cargo tools
//!
//! Optimized for: Raspberry Pi 4B 8GB RAM, 4 cores, external USB 3.0 HDD
//! - Uses cargo-binstall for pre-compiled binaries (avoids 30+ min compilation)
//! - Batch installs all tools at once for parallel downloads
//! - Sets CARGO_BUILD_JOBS=4 to utilize all Pi 4B cores
//! - Minimal rustup profile to reduce disk usage

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
                ctx.record_warning(format!("rustup update failed; continuing ({err})"));
            }
        }
        return Ok(());
    }

    tracing::info!("Installing rustup + stable toolchain (minimal profile for faster install)");
    if ctx.options.dry_run {
        ctx.record_dry_run(
            "rust_toolchain",
            "Would install rustup toolchain",
            Some("curl rustup.rs | sh -s -- -y --profile minimal".into()),
        );
        tracing::info!("[dry-run] curl rustup.rs | sh -s -- -y --profile minimal");
        return Ok(());
    }

    // Use minimal profile to reduce download/install time (optimized for Pi 4B)
    let mut install_cmd = Command::new("sh");
    install_cmd.arg("-c").arg(
        "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal"
    );
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
            ctx.record_warning(format!(
                "Failed to add component {comp}; continuing ({err})"
            ));
        }
    }
    Ok(())
}

fn ensure_cargo_binstall(ctx: &mut PhaseContext) -> Result<()> {
    if which::which("cargo-binstall").is_ok() || cargo_home().join("bin/cargo-binstall").exists() {
        tracing::info!("cargo-binstall already installed");
        return Ok(());
    }

    tracing::info!("Installing cargo-binstall (enables fast binary installs)");
    if ctx.options.dry_run {
        ctx.record_dry_run("rust_toolchain", "Would install cargo-binstall", None);
        return Ok(());
    }

    // Install cargo-binstall using the official installer script
    let mut install_cmd = Command::new("sh");
    install_cmd.arg("-c").arg(
        "curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash"
    );

    if let Err(err) = cmd::run(&mut install_cmd) {
        ctx.record_warning(format!(
            "Failed to install cargo-binstall; will use slower cargo install: {err}"
        ));
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

    let use_binstall =
        which::which("cargo-binstall").is_ok() || cargo_home().join("bin/cargo-binstall").exists();

    if use_binstall {
        tracing::info!("Using cargo-binstall for fast parallel installation! 🚀");
    } else {
        tracing::warn!("cargo-binstall not available; falling back to slow cargo install (this will take a while...)");
    }

    // Filter out already-installed tools
    let mut missing_tools: Vec<&str> = Vec::new();
    for (crate_name, bin_name) in tools {
        let bin_path = cargo_home().join("bin").join(bin_name);
        if bin_path.exists() || which::which(bin_name).is_ok() {
            tracing::info!("{bin_name} already installed");
        } else {
            missing_tools.push(crate_name);
        }
    }

    if missing_tools.is_empty() {
        tracing::info!("All cargo tools already installed");
        return Ok(());
    }

    if ctx.options.dry_run {
        ctx.record_dry_run(
            "rust_toolchain",
            "Would install cargo tools",
            Some(missing_tools.join(", ")),
        );
        return Ok(());
    }

    if use_binstall {
        // BATCH INSTALL ALL TOOLS AT ONCE - Much faster! (optimized for Pi 4B)
        tracing::info!(
            "Installing {} tools in one batch: {}",
            missing_tools.len(),
            missing_tools.join(", ")
        );
        let mut install_cmd = Command::new(cargo_bin());
        install_cmd.arg("binstall").arg("--no-confirm");

        // Optimize for Pi 4B: 4 cores, external USB 3.0 HDD
        install_cmd
            .env("CARGO_BUILD_JOBS", "4")
            .env("CARGO_NET_GIT_FETCH_WITH_CLI", "true");

        for crate_name in &missing_tools {
            install_cmd.arg(crate_name);
        }

        if let Err(err) = cmd::run(&mut install_cmd) {
            ctx.record_warning(format!(
                "Batch cargo-binstall failed, trying one-by-one: {err}"
            ));
            // Fallback: install one by one
            for crate_name in &missing_tools {
                tracing::info!("Installing {crate_name} individually...");
                let mut retry_cmd = Command::new(cargo_bin());
                retry_cmd.args(["binstall", "--no-confirm", crate_name]);
                if let Err(err2) = cmd::run(&mut retry_cmd) {
                    ctx.record_warning(format!("Failed to install {crate_name} ({err2})"));
                }
            }
        } else {
            tracing::info!("✓ All cargo tools installed successfully!");
        }
    } else {
        // Fallback to cargo install (slow - compiles from source)
        tracing::warn!(
            "Installing {} tools one-by-one (this will take 10-30 minutes...)",
            missing_tools.len()
        );
        for crate_name in &missing_tools {
            tracing::info!("Installing {crate_name} via cargo install...");
            let mut install_cmd = Command::new(cargo_bin());
            install_cmd
                .args(["install", crate_name])
                .env("CARGO_BUILD_JOBS", "4"); // Use all 4 cores on Pi 4B
            if let Err(err) = cmd::run(&mut install_cmd) {
                ctx.record_warning(format!("Failed to install {crate_name} ({err})"));
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
                    ctx.record_warning(format!("Failed to install flamegraph ({err})"));
                }
            } else {
                ctx.record_dry_run("rust_toolchain", "Would install flamegraph", None);
            }
        }
    }

    Ok(())
}
