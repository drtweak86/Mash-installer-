use anyhow::{Context, Result};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{
    apt_repo, cmd,
    driver::{RepoKind, ServiceName},
    package_manager, systemd, PhaseContext, PkgBackend,
};

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    let backend = ctx.platform.pkg_backend;

    if ctx.options.dry_run {
        ctx.record_dry_run(
            "docker",
            "Would install Docker packages",
            Some(format!("Backend: {:?}", backend)),
        );
    }

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
        ctx.record_dry_run(
            "docker",
            "Would add user to docker group",
            Some(format!("User: {user}")),
        );
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
        ctx.record_dry_run(
            "docker",
            "Would enable docker service",
            Some(format!("Service: {service}")),
        );
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

fn daemon_config_path() -> PathBuf {
    env::var_os("MASH_DOCKER_DAEMON_JSON")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/etc/docker/daemon.json"))
}

fn configure_data_root(ctx: &mut PhaseContext, data_root: &std::path::Path) -> Result<()> {
    let daemon_json_path = daemon_config_path();
    let backup_path = daemon_json_path.with_extension("json.bak");

    let original_daemon = if daemon_json_path.exists() {
        Some(fs::read_to_string(&daemon_json_path)?)
    } else {
        None
    };

    let config =
        load_daemon_config(daemon_json_path.as_path())?.unwrap_or_else(|| serde_json::json!({}));
    if let Some(config) = update_data_root_config(config, data_root) {
        if ctx.options.dry_run {
            tracing::info!(
                "[dry-run] would configure Docker data-root to {}",
                data_root.display()
            );
            ctx.record_dry_run(
                "docker",
                "Would configure Docker data-root",
                Some(format!("Path: {}", data_root.display())),
            );
            return Ok(());
        }

        tracing::info!("Configuring Docker data-root to {}", data_root.display());
        crate::staging::ensure_space_for_path(data_root)?;

        let rollback_contents = original_daemon.clone();
        let rollback_daemon = daemon_json_path.clone();
        let rollback_backup = backup_path.clone();
        ctx.register_rollback_action("restore docker daemon config", move || {
            if let Some(contents) = &rollback_contents {
                fs::write(&rollback_daemon, contents)?;
            } else if rollback_daemon.exists() {
                fs::remove_file(&rollback_daemon)?;
            }
            if rollback_backup.exists() {
                fs::remove_file(&rollback_backup)?;
            }
            Ok(())
        });

        fs::create_dir_all(data_root)?;
        let content = serde_json::to_string_pretty(&config)?;

        if daemon_json_path.exists() {
            fs::copy(&daemon_json_path, &backup_path)?;
        }

        let mut tee_cmd = Command::new("sh");
        tee_cmd.arg("-c").arg(format!(
            "echo '{}' | sudo tee {} > /dev/null",
            content,
            daemon_json_path.display()
        ));
        cmd::run(&mut tee_cmd)?;

        let mut restart_cmd = Command::new("sudo");
        restart_cmd.args(["systemctl", "restart", "docker"]);
        let _ = cmd::run(&mut restart_cmd);

        Ok(())
    } else {
        tracing::info!("Docker data-root already set to {}", data_root.display());
        Ok(())
    }
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

fn update_data_root_config(mut config: Value, data_root: &Path) -> Option<Value> {
    let desired = data_root.display().to_string();
    if is_data_root_configured(&config, &desired) {
        return None;
    }
    config["data-root"] = Value::String(desired);
    Some(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        backend::PkgBackend, driver::DistroDriver, dry_run::DryRunLog, ConfigService, Localization,
        PhaseContext, PlatformContext, PlatformInfo, ProfileLevel, RollbackManager, UIContext,
        UserOptionsContext,
    };
    use anyhow::Result;
    use serde_json::json;
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    struct TestDriver;

    impl DistroDriver for TestDriver {
        fn name(&self) -> &'static str {
            "test-docker-driver"
        }

        fn description(&self) -> &'static str {
            "driver used for docker dry run tests"
        }

        fn matches(&self, _: &PlatformInfo) -> bool {
            true
        }

        fn pkg_backend(&self) -> PkgBackend {
            PkgBackend::Apt
        }
    }

    static TEST_DRIVER: TestDriver = TestDriver;

    struct TestPhaseEnv {
        options: UserOptionsContext,
        platform: PlatformContext,
        ui: UIContext,
        localization: Localization,
        rollback: RollbackManager,
        dry_run_log: DryRunLog,
    }

    impl TestPhaseEnv {
        fn new(dry_run: bool) -> Result<Self> {
            let config_service = ConfigService::load()?;
            let platform = PlatformInfo {
                arch: "x86_64".into(),
                distro: "mash-test".into(),
                distro_version: "0".into(),
                distro_codename: "test".into(),
                distro_family: "debian".into(),
                pi_model: None,
            };
            let platform_ctx = PlatformContext {
                config_service,
                platform,
                driver_name: TEST_DRIVER.name(),
                driver: &TEST_DRIVER,
                pkg_backend: PkgBackend::Apt,
            };
            let options = UserOptionsContext {
                profile: ProfileLevel::Minimal,
                staging_dir: PathBuf::from("/tmp/mash-dry-run"),
                dry_run,
                interactive: false,
                enable_argon: false,
                enable_p10k: false,
                docker_data_root: false,
            };
            let localization = Localization::load_default()?;

            Ok(Self {
                options,
                platform: platform_ctx,
                ui: UIContext::default(),
                localization,
                rollback: RollbackManager::new(),
                dry_run_log: DryRunLog::new(),
            })
        }

        fn phase_context(&mut self) -> PhaseContext<'_> {
            PhaseContext {
                options: &self.options,
                platform: &self.platform,
                ui: &self.ui,
                localization: &self.localization,
                rollback: &self.rollback,
                dry_run_log: &self.dry_run_log,
            }
        }
    }

    #[test]
    fn docker_dry_run_helpers_log_actions() -> Result<()> {
        let mut env = TestPhaseEnv::new(true)?;
        std::env::set_var("USER", "nobody");
        {
            let mut phase_ctx = env.phase_context();
            add_user_to_docker_group(&mut phase_ctx)?;
        }
        {
            let mut phase_ctx = env.phase_context();
            enable_docker_service(&mut phase_ctx)?;
        }
        std::env::remove_var("USER");

        let entries = env.dry_run_log.entries();
        assert!(
            entries
                .iter()
                .any(|entry| entry.action == "Would add user to docker group"),
            "expected dry-run log to include user management entry"
        );
        assert!(
            entries
                .iter()
                .any(|entry| entry.action == "Would enable docker service"),
            "expected dry-run log to include service enable entry"
        );
        Ok(())
    }

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
        assert_eq!(
            config.get("data-root").and_then(Value::as_str),
            Some("/data")
        );
        Ok(())
    }

    #[test]
    fn is_data_root_configured_detects_matching_values() {
        let config = json!({"data-root": "/var/lib/docker"});
        assert!(is_data_root_configured(&config, "/var/lib/docker"));
        assert!(!is_data_root_configured(&config, "/tmp/docker"));
    }

    #[test]
    fn update_data_root_config_skips_when_target_present() {
        let config = json!({"data-root": "/mnt/docker"});
        assert!(update_data_root_config(config, Path::new("/mnt/docker")).is_none());
    }

    #[test]
    fn update_data_root_config_applies_when_missing() {
        let config = json!({});
        let updated =
            update_data_root_config(config, Path::new("/data")).expect("should update config");
        assert_eq!(
            updated.get("data-root").and_then(Value::as_str),
            Some("/data")
        );
    }
}
