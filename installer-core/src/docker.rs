use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

use crate::{
    apt_repo,
    driver::{RepoKind, ServiceName},
    package_manager, systemd, InstallContext, PkgBackend,
};

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    let backend = ctx.platform.pkg_backend;

    let already = match backend {
        PkgBackend::Apt => package_manager::is_installed(ctx.platform.driver, "docker-ce"),
        PkgBackend::Pacman => package_manager::is_installed(ctx.platform.driver, "docker"),
    };

    if already {
        tracing::info!("Docker already installed");
    } else {
        match backend {
            PkgBackend::Apt => {
                apt_repo::ensure_repo(ctx, RepoKind::Docker)?;
                install_docker_apt(ctx)?;
            }
            PkgBackend::Pacman => {
                install_docker_pacman(ctx)?;
            }
        }
    }

    add_user_to_docker_group(ctx)?;
    enable_docker_service(ctx)?;

    let desired_data_root = if ctx.options.docker_data_root {
        Some(ctx.options.staging_dir.join("docker"))
    } else {
        ctx.platform.config().docker.data_root.clone()
    };
    if let Some(data_root) = desired_data_root {
        configure_data_root(ctx, &data_root)?;
    }

    Ok(())
}

fn install_docker_apt(ctx: &InstallContext) -> Result<()> {
    let pkgs = [
        "docker-ce",
        "docker-ce-cli",
        "containerd.io",
        "docker-buildx-plugin",
        "docker-compose-plugin",
    ];
    package_manager::ensure_packages(ctx.platform.driver, &pkgs, ctx.options.dry_run)
}

// ── Pacman path ─────────────────────────────────────────────────

fn install_docker_pacman(ctx: &InstallContext) -> Result<()> {
    // On Arch/Manjaro, Docker is in the community repo
    let pkgs = ["docker", "docker-buildx", "docker-compose"];
    package_manager::ensure_packages(ctx.platform.driver, &pkgs, ctx.options.dry_run)
}

// ── Common ──────────────────────────────────────────────────────

fn add_user_to_docker_group(ctx: &InstallContext) -> Result<()> {
    let user = std::env::var("SUDO_USER")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "root".into());

    if user == "root" {
        return Ok(());
    }

    let groups_out = Command::new("id").arg("-nG").arg(&user).output()?;
    let groups = String::from_utf8_lossy(&groups_out.stdout);
    if groups.split_whitespace().any(|g| g == "docker") {
        tracing::info!("User '{user}' already in docker group");
        return Ok(());
    }

    tracing::info!("Adding user '{user}' to docker group");
    if ctx.options.dry_run {
        return Ok(());
    }

    let status = Command::new("sudo")
        .args(["usermod", "-aG", "docker", &user])
        .status()
        .context("adding user to docker group")?;
    if !status.success() {
        tracing::warn!("Failed to add user to docker group");
    }
    Ok(())
}

fn enable_docker_service(ctx: &InstallContext) -> Result<()> {
    if ctx.options.dry_run {
        let service = ctx.platform.driver.service_unit(ServiceName::Docker);
        tracing::info!("[dry-run] would enable {service}");
        return Ok(());
    }
    if !systemd::is_available() {
        tracing::warn!("systemd not detected; skipping docker.service enable");
        return Ok(());
    }
    let service = ctx.platform.driver.service_unit(ServiceName::Docker);
    let _ = Command::new("sudo")
        .args(["systemctl", "enable", "--now", service])
        .status();
    Ok(())
}

fn configure_data_root(ctx: &InstallContext, data_root: &std::path::Path) -> Result<()> {
    let daemon_json = std::path::Path::new("/etc/docker/daemon.json");

    tracing::info!("Configuring Docker data-root to {}", data_root.display());
    crate::staging::ensure_space_for_path(data_root)?;
    if ctx.options.dry_run {
        return Ok(());
    }

    fs::create_dir_all(data_root)?;

    let mut config: serde_json::Value = if daemon_json.exists() {
        let text = fs::read_to_string(daemon_json)?;
        match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(obj) => match obj {
                serde_json::Value::Object(_) => obj,
                _ => {
                    anyhow::bail!(
                        "{} must be a JSON object; please fix or remove it before rerunning.",
                        daemon_json.display()
                    )
                }
            },
            Err(err) => {
                anyhow::bail!(
                    "Failed to parse {}: {err}. Please repair the file (comments are not allowed) before rerunning.",
                    daemon_json.display()
                )
            }
        }
    } else {
        serde_json::json!({})
    };

    config["data-root"] = serde_json::Value::String(data_root.display().to_string());

    let content = serde_json::to_string_pretty(&config)?;

    if daemon_json.exists() {
        let backup = daemon_json.with_extension("json.bak");
        fs::copy(daemon_json, &backup)?;
    }

    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo '{}' | sudo tee {} > /dev/null",
            content,
            daemon_json.display()
        ))
        .status()?;

    let _ = Command::new("sudo")
        .args(["systemctl", "restart", "docker"])
        .status();

    Ok(())
}
