use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

use crate::{pkg::PkgBackend, systemd, InstallContext};

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    let backend = crate::pkg::detect_backend();

    let already = match backend {
        PkgBackend::Apt => crate::pkg::is_installed("docker-ce"),
        PkgBackend::Pacman => crate::pkg::is_installed("docker"),
    };

    if already {
        tracing::info!("Docker already installed");
    } else {
        match backend {
            PkgBackend::Apt => {
                add_docker_apt_repo(ctx)?;
                install_docker_apt(ctx)?;
            }
            PkgBackend::Pacman => {
                install_docker_pacman(ctx)?;
            }
        }
    }

    add_user_to_docker_group(ctx)?;
    enable_docker_service(ctx)?;

    let desired_data_root = if ctx.docker_data_root {
        Some(ctx.staging_dir.join("docker"))
    } else {
        ctx.config.docker.data_root.clone()
    };
    if let Some(data_root) = desired_data_root {
        configure_data_root(ctx, &data_root)?;
    }

    Ok(())
}

// ── APT path ────────────────────────────────────────────────────

fn add_docker_apt_repo(ctx: &InstallContext) -> Result<()> {
    tracing::info!("Adding Docker official GPG key and repository");
    if ctx.dry_run {
        tracing::info!("[dry-run] would add Docker apt repo");
        return Ok(());
    }

    let repo_os = docker_repo_os(ctx)?;
    let keyring = "/etc/apt/keyrings/docker.asc";
    if !std::path::Path::new(keyring).exists() {
        Command::new("sudo")
            .args(["install", "-m", "0755", "-d", "/etc/apt/keyrings"])
            .status()
            .context("creating keyrings dir")?;

        let status = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "curl -fsSL https://download.docker.com/linux/{repo_os}/gpg | sudo tee {keyring} > /dev/null && sudo chmod a+r {keyring}"
            ))
            .status()
            .context("downloading Docker GPG key")?;
        if !status.success() {
            anyhow::bail!("Failed to add Docker GPG key");
        }
    }

    let sources_list = "/etc/apt/sources.list.d/docker.list";
    if !std::path::Path::new(sources_list).exists() {
        let arch_out = Command::new("dpkg").arg("--print-architecture").output()?;
        let arch = String::from_utf8_lossy(&arch_out.stdout).trim().to_string();

        let codename_out = Command::new("sh")
            .arg("-c")
            .arg(". /etc/os-release && echo $VERSION_CODENAME")
            .output()?;
        let codename = String::from_utf8_lossy(&codename_out.stdout)
            .trim()
            .to_string();
        if codename.is_empty() {
            anyhow::bail!("Unable to determine distro codename for Docker repo setup");
        }

        let repo_line = format!(
            "deb [arch={arch} signed-by={keyring}] https://download.docker.com/linux/{repo_os} {codename} stable"
        );

        Command::new("sh")
            .arg("-c")
            .arg(format!(
                "echo '{repo_line}' | sudo tee {sources_list} > /dev/null"
            ))
            .status()
            .context("writing docker sources list")?;

        crate::pkg::update(false)?;
    }

    Ok(())
}

fn docker_repo_os(ctx: &InstallContext) -> Result<&'static str> {
    let distro = ctx.platform.distro.to_lowercase();
    match distro.as_str() {
        "ubuntu" | "linuxmint" | "pop" | "elementary" | "zorin" => Ok("ubuntu"),
        _ if ctx.platform.distro_family == "debian" => Ok("debian"),
        other => anyhow::bail!("Unsupported distro '{other}' for Docker repo setup"),
    }
}

fn install_docker_apt(ctx: &InstallContext) -> Result<()> {
    let pkgs = [
        "docker-ce",
        "docker-ce-cli",
        "containerd.io",
        "docker-buildx-plugin",
        "docker-compose-plugin",
    ];
    crate::pkg::ensure_packages(&pkgs, ctx.dry_run)
}

// ── Pacman path ─────────────────────────────────────────────────

fn install_docker_pacman(ctx: &InstallContext) -> Result<()> {
    // On Arch/Manjaro, Docker is in the community repo
    let pkgs = ["docker", "docker-buildx", "docker-compose"];
    crate::pkg::ensure_packages(&pkgs, ctx.dry_run)
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
    if ctx.dry_run {
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
    if ctx.dry_run {
        tracing::info!("[dry-run] would enable docker.service");
        return Ok(());
    }
    if !systemd::is_available() {
        tracing::warn!("systemd not detected; skipping docker.service enable");
        return Ok(());
    }
    let _ = Command::new("sudo")
        .args(["systemctl", "enable", "--now", "docker.service"])
        .status();
    Ok(())
}

fn configure_data_root(ctx: &InstallContext, data_root: &std::path::Path) -> Result<()> {
    let daemon_json = std::path::Path::new("/etc/docker/daemon.json");

    tracing::info!("Configuring Docker data-root to {}", data_root.display());
    crate::staging::ensure_space_for_path(data_root)?;
    if ctx.dry_run {
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
