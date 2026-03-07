//! Chezmoi dotfile management integration
//!
//! Enables automated restoration of personal dotfiles during provisioning.
//! Chezmoi is installed (via package manager or official script), then
//! initialized from a provided Git repository URL and optionally a branch.

use anyhow::{Context, Result};
use std::process::Command;
use tracing::info;

use crate::system::cmd;
use crate::{PhaseContext, PhaseResult};

/// Main entry point for the chezmoi dotfile restoration phase.
pub fn install_phase(ctx: &mut PhaseContext) -> Result<PhaseResult> {
    let opts = &ctx.options.chezmoi;

    if !opts.enabled {
        info!("Chezmoi integration is disabled; skipping.");
        return Ok(PhaseResult::Success);
    }

    let repo_url = match &opts.repo_url {
        Some(url) => url,
        None => {
            info!("Chezmoi enabled but no repository URL provided; skipping.");
            return Ok(PhaseResult::Success);
        }
    };

    // 1. Ensure chezmoi is installed
    ensure_installed(ctx)?;

    // 2. Initialize and apply dotfiles
    init_and_apply(ctx, repo_url, opts.branch.as_deref())?;

    Ok(PhaseResult::Success)
}

fn is_installed() -> bool {
    which::which("chezmoi").is_ok()
}

fn ensure_installed(ctx: &mut PhaseContext) -> Result<()> {
    if is_installed() {
        info!("chezmoi is already installed.");
        return Ok(());
    }

    info!("chezmoi not found via package manager; installing via official script to ~/.local/bin");

    if ctx.options.dry_run {
        ctx.record_dry_run(
            "chezmoi",
            "Would install chezmoi via official script",
            Some("sh -c 'curl -sfL https://git.io/chezmoi | sh -s -- -b ~/.local/bin'".to_string()),
        );
        return Ok(());
    }

    // Ensure ~/.local/bin exists
    let home = dirs::home_dir().context("Determining home directory")?;
    let bin_dir = home.join(".local/bin");
    if !bin_dir.exists() {
        std::fs::create_dir_all(&bin_dir).context("Creating ~/.local/bin")?;
    }

    let mut install_cmd = Command::new("sh");
    install_cmd
        .arg("-c")
        .arg("curl -sfL https://git.io/chezmoi | sh -s -- -b ~/.local/bin");

    cmd::run(&mut install_cmd).context("Executing chezmoi installation script")?;

    ctx.record_action("Installed chezmoi to ~/.local/bin");
    Ok(())
}

fn init_and_apply(ctx: &mut PhaseContext, repo_url: &str, branch: Option<&str>) -> Result<()> {
    info!("Initializing chezmoi from repository: {}", repo_url);

    if ctx.options.dry_run {
        let branch_info = branch
            .map(|b| format!(" (branch: {})", b))
            .unwrap_or_default();
        ctx.record_dry_run(
            "chezmoi",
            format!(
                "Would initialize and apply dotfiles from {}{}",
                repo_url, branch_info
            ),
            Some(format!("chezmoi init --apply --purge-binary {}", repo_url)),
        );
        return Ok(());
    }

    let mut init_cmd = Command::new("chezmoi");
    init_cmd.arg("init").arg("--apply").arg("--purge-binary");

    if let Some(b) = branch {
        init_cmd.arg("--branch").arg(b);
    }

    init_cmd.arg(repo_url);

    // We use cmd::run which logs output and handles failure
    cmd::run(&mut init_cmd).context("Running chezmoi init --apply")?;

    ctx.record_action(format!("Restored dotfiles from {}", repo_url));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{
        ConfigService, PhaseContext, PlatformContext, UIContext, UserOptionsContext,
    };
    use crate::driver::DistroDriver;
    use crate::dry_run::DryRunLog;
    use crate::localization::Localization;
    use crate::model::options::{ChezmoiOptions, EnvironmentTag, ProfileLevel};
    use crate::platform::PlatformInfo;
    use crate::rollback::RollbackManager;
    use crate::{InstallContext, SoftwareTierPlan};
    use std::path::PathBuf;

    struct TestDriver;
    impl DistroDriver for TestDriver {
        fn name(&self) -> &'static str {
            "test"
        }
        fn description(&self) -> &'static str {
            "test"
        }
        fn matches(&self, _: &PlatformInfo) -> bool {
            true
        }
        fn pkg_backend(&self) -> crate::PkgBackend {
            crate::PkgBackend::Apt
        }
    }

    struct NoopObserver;
    impl crate::PhaseObserver for NoopObserver {
        fn on_event(&mut self, _: crate::PhaseEvent) {}
    }

    fn build_test_context(chezmoi: ChezmoiOptions) -> Result<InstallContext> {
        let config_service = ConfigService::load()?;
        let platform = PlatformInfo {
            arch: "x86_64".into(),
            distro: "test".into(),
            distro_version: "0".into(),
            distro_codename: "test".into(),
            distro_family: "debian".into(),
            pi_model: None,
            cpu_model: "test".into(),
            cpu_cores: 4,
            ram_total_gb: 8.0,
        };
        let driver: &'static dyn DistroDriver = &TestDriver;
        let platform_ctx = PlatformContext {
            config_service,
            platform,
            driver_name: "test",
            driver,
            pkg_backend: driver.pkg_backend(),
            system: &crate::sys_ops::REAL_SYSTEM,
        };
        let options = UserOptionsContext {
            profile: ProfileLevel::Dev,
            staging_dir: PathBuf::from("/tmp/mash-test"),
            dry_run: true,
            interactive: false,
            enable_argon: false,
            enable_p10k: false,
            docker_data_root: false,
            software_plan: SoftwareTierPlan::default(),
            system_profile: None,
            environment: EnvironmentTag::Home,
            chezmoi,
            desktop_environment: None,
            display_protocol: crate::desktop::DisplayProtocol::Auto,
        };
        let localization = Localization::load_default()?;
        let cache = crate::ArtifactCache::new(&PathBuf::from("/tmp/mash-test-cache"));

        Ok(InstallContext {
            options,
            platform: platform_ctx,
            ui: UIContext,
            interaction: crate::interaction::InteractionService::new(false, Default::default()),
            localization,
            rollback: RollbackManager::new(),
            dry_run_log: DryRunLog::new(),
            cache,
        })
    }

    #[test]
    fn test_chezmoi_skipped_if_disabled() -> Result<()> {
        let chezmoi = ChezmoiOptions {
            enabled: false,
            repo_url: None,
            branch: None,
        };
        let install_ctx = build_test_context(chezmoi)?;
        let mut observer = NoopObserver;
        let mut phase_ctx = PhaseContext::from_ctx(&install_ctx, &mut observer);

        let result = install_phase(&mut phase_ctx)?;
        assert_eq!(result, PhaseResult::Success);
        assert!(install_ctx.dry_run_log.audit_report().phases.is_empty());
        Ok(())
    }

    #[test]
    fn test_chezmoi_skipped_if_no_url() -> Result<()> {
        let chezmoi = ChezmoiOptions {
            enabled: true,
            repo_url: None,
            branch: None,
        };
        let install_ctx = build_test_context(chezmoi)?;
        let mut observer = NoopObserver;
        let mut phase_ctx = PhaseContext::from_ctx(&install_ctx, &mut observer);

        let result = install_phase(&mut phase_ctx)?;
        assert_eq!(result, PhaseResult::Success);
        Ok(())
    }

    #[test]
    fn test_chezmoi_dry_run_init() -> Result<()> {
        let chezmoi = ChezmoiOptions {
            enabled: true,
            repo_url: Some("https://github.com/user/dotfiles.git".into()),
            branch: Some("main".into()),
        };
        let install_ctx = build_test_context(chezmoi)?;
        let mut observer = NoopObserver;
        let mut phase_ctx = PhaseContext::from_ctx(&install_ctx, &mut observer);

        let result = install_phase(&mut phase_ctx)?;
        assert_eq!(result, PhaseResult::Success);

        let report = install_ctx.dry_run_log.audit_report();
        let chezmoi_entries = report
            .phases
            .get("chezmoi")
            .expect("chezmoi phase not found in report");
        assert!(chezmoi_entries
            .iter()
            .any(|entry| entry.action.contains("initialize and apply")));
        Ok(())
    }
}
