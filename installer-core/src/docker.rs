use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::{
    apt_repo, cmd,
    driver::{RepoKind, ServiceName},
    package_manager, systemd, PhaseContext, PkgBackend,
};

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
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

fn install_docker_apt(ctx: &mut PhaseContext) -> Result<()> {
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

fn install_docker_pacman(ctx: &mut PhaseContext) -> Result<()> {
    // On Arch/Manjaro, Docker is in the community repo
    let pkgs = ["docker", "docker-buildx", "docker-compose"];
    package_manager::ensure_packages(ctx.platform.driver, &pkgs, ctx.options.dry_run)
}

// ── Common ──────────────────────────────────────────────────────

fn add_user_to_docker_group(ctx: &mut PhaseContext) -> Result<()> {
    let user = std::env::var("SUDO_USER")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "root".into());

    if user == "root" {
        return Ok(());
    }

    let mut id_cmd = Command::new("id");
    id_cmd.arg("-nG").arg(&user);
    let groups_out = cmd::run(&mut id_cmd)?;
    let groups = String::from_utf8_lossy(&groups_out.stdout);
    if groups.split_whitespace().any(|g| g == "docker") {
        tracing::info!("User '{user}' already in docker group");
        return Ok(());
    }

    tracing::info!("Adding user '{user}' to docker group");
    if ctx.options.dry_run {
        return Ok(());
    }

    let mut usermod = Command::new("sudo");
    usermod.args(["usermod", "-aG", "docker", &user]);
    if let Err(err) = cmd::run(&mut usermod).context("adding user to docker group") {
        tracing::warn!("Failed to add user to docker group ({err})");
    }
    Ok(())
}

fn enable_docker_service(ctx: &mut PhaseContext) -> Result<()> {
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
    let mut enable_cmd = Command::new("sudo");
    enable_cmd.args(["systemctl", "enable", "--now", service]);
    let _ = cmd::run(&mut enable_cmd);
    Ok(())
}

fn configure_data_root(ctx: &mut PhaseContext, data_root: &std::path::Path) -> Result<()> {
    let daemon_json = Path::new("/etc/docker/daemon.json");

    tracing::info!("Configuring Docker data-root to {}", data_root.display());
    crate::staging::ensure_space_for_path(data_root)?;

    let mut config = load_daemon_config(daemon_json)?.unwrap_or_else(|| serde_json::json!({}));
    let desired_data_root = data_root.display().to_string();
    if is_data_root_configured(&config, &desired_data_root) {
        tracing::info!("Docker data-root already set to {}", data_root.display());
        return Ok(());
    }

    if ctx.options.dry_run {
        tracing::info!("[dry-run] would configure Docker data-root to {}", data_root.display());
        return Ok(());
    }

    fs::create_dir_all(data_root)?;

    config["data-root"] = Value::String(desired_data_root.clone());
    let content = serde_json::to_string_pretty(&config)?;

    if daemon_json.exists() {
        let backup = daemon_json.with_extension("json.bak");
        fs::copy(daemon_json, &backup)?;
    }

    let mut tee_cmd = Command::new("sh");
    tee_cmd
        .arg("-c")
        .arg(format!(
            "echo '{}' | sudo tee {} > /dev/null",
            content,
            daemon_json.display()
        ));
    cmd::run(&mut tee_cmd)?;

    let mut restart_cmd = Command::new("sudo");
    restart_cmd.args(["systemctl", "restart", "docker"]);
    let _ = cmd::run(&mut restart_cmd);

    Ok(())
}

fn load_daemon_config(path: &Path) -> Result<Option<Value>> {
    if !path.exists() {
        return Ok(None);
    }

    let text = fs::read_to_string(path)?;
    match serde_json::from_str::<Value>(&text) {
        Ok(value) => match value {
            Value::Object(_) => Ok(Some(value)),
            _ => anyhow::bail!(
                "{} must be a JSON object; please fix or remove it before rerunning.",
                path.display()
            ),
        },
        Err(err) => anyhow::bail!(
            "Failed to parse {}: {err}. Please repair the file (comments are not allowed) before rerunning.",
            path.display()
        ),
    }
}

fn is_data_root_configured(config: &Value, desired: &str) -> bool {
    match config.get("data-root") {
        Some(Value::String(existing)) => existing == desired,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn load_daemon_config_returns_none_when_missing() -> Result<()> {
        let dir = tempdir()?;
        let missing_path = dir.path().join("daemon.json");
        assert!(load_daemon_config(&missing_path)?.is_none());
        Ok(())
    }

    #[test]
    fn load_daemon_config_parses_json_object() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("daemon.json");
        fs::write(&path, r#"{"data-root": "/data"}"#)?;
        let config = load_daemon_config(&path)?.expect("config should exist");
        assert_eq!(config.get("data-root").and_then(Value::as_str), Some("/data"));
        Ok(())
    }

    #[test]
    fn is_data_root_configured_detects_matching_values() {
        let config = json!({"data-root": "/var/lib/docker"});
        assert!(is_data_root_configured(&config, "/var/lib/docker"));
        assert!(!is_data_root_configured(&config, "/tmp/docker"));
    }
}
