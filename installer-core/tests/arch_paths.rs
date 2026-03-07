// Architecture Path Testing
// Verifies that the installer handles aarch64 and x86_64 logic correctly.

use anyhow::Result;
use installer_core::{
    dry_run::DryRunLog, ConfigService, DistroDriver, EnvironmentTag, InstallContext, Phase,
    PhaseContext, PhaseObserver, PhaseResult, PhaseRunner, PkgBackend, PlatformInfo, ProfileLevel,
    SoftwareTierPlan, UIContext, UserOptionsContext,
};
use std::path::PathBuf;

// Dummy driver for testing
struct MockDriver {
    name: &'static str,
    arch: &'static str,
}

impl DistroDriver for MockDriver {
    fn name(&self) -> &'static str {
        self.name
    }
    fn description(&self) -> &'static str {
        "mock driver"
    }
    fn matches(&self, info: &PlatformInfo) -> bool {
        info.arch == self.arch
    }
    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Apt
    }
}

fn build_mock_context(arch: &'static str) -> Result<InstallContext> {
    let config_service = ConfigService::load()?;
    let platform = PlatformInfo {
        arch: arch.into(),
        distro: "MockOS".into(),
        distro_version: "1.0".into(),
        distro_codename: "mock".into(),
        distro_family: "debian".into(),
        pi_model: if arch == "aarch64" {
            Some("Raspberry Pi 4 Model B".to_string())
        } else {
            None
        },
        cpu_model: "test".into(),
        cpu_cores: 4,
        ram_total_gb: 8.0,
    };

    let options = UserOptionsContext {
        profile: ProfileLevel::Minimal,
        staging_dir: PathBuf::from("/tmp/mash-test"),
        dry_run: false,
        interactive: false,
        enable_argon: arch == "aarch64",
        enable_p10k: false,
        docker_data_root: false,
        software_plan: SoftwareTierPlan::default(),
        system_profile: None,
        environment: EnvironmentTag::Home,
        chezmoi: Default::default(),
        desktop_environment: None,
        display_protocol: installer_core::desktop::DisplayProtocol::Auto,
    };
    let localization = installer_core::localization::Localization::load_default()?;

    let cache_dir = PathBuf::from("/tmp/mash-test-cache");
    let cache = installer_core::ArtifactCache::new(&cache_dir);

    Ok(InstallContext {
        options,
        platform: installer_core::PlatformContext {
            config_service,
            platform,
            driver_name: "mock",
            driver: Box::leak(Box::new(MockDriver { name: "mock", arch })),
            pkg_backend: PkgBackend::Apt,
            system: &installer_core::REAL_SYSTEM,
        },
        ui: UIContext,
        interaction: installer_core::interaction::InteractionService::new(
            false,
            Default::default(),
        ),
        localization,
        rollback: installer_core::RollbackManager::new(),
        dry_run_log: DryRunLog::new(),
        cache,
    })
}

#[test]
fn test_aarch64_pi_detection() -> Result<()> {
    let ctx = build_mock_context("aarch64")?;
    assert!(ctx.platform.is_pi());
    assert!(ctx.platform.is_pi_4b());
    assert_eq!(ctx.platform.pi_generation(), Some(4));
    Ok(())
}

#[test]
fn test_x86_64_detection() -> Result<()> {
    let ctx = build_mock_context("x86_64")?;
    assert!(!ctx.platform.is_pi());
    assert_eq!(ctx.platform.platform.arch, "x86_64");
    Ok(())
}

struct ArchSpecificPhase;
impl Phase for ArchSpecificPhase {
    fn name(&self) -> &str {
        "arch-check"
    }
    fn description(&self) -> &str {
        "arch check"
    }
    fn execute(&self, ctx: &mut PhaseContext) -> Result<PhaseResult> {
        let arch = &ctx.platform.platform.arch;
        ctx.record_action(format!("Running on {}", arch));
        Ok(PhaseResult::Success)
    }
}

#[test]
fn test_arch_logic_in_phases() -> Result<()> {
    let archs = vec!["aarch64", "x86_64"];
    for arch in archs {
        let ctx = build_mock_context(arch)?;
        let phases: Vec<Box<dyn Phase>> = vec![Box::new(ArchSpecificPhase)];
        let runner = PhaseRunner::from_phases(phases);

        struct NoopObserver;
        impl PhaseObserver for NoopObserver {}
        let mut observer = NoopObserver;

        let result = runner.run(&ctx, &mut observer, None)?;
        assert_eq!(
            result.outputs[0].actions_taken[0],
            format!("Running on {}", arch)
        );
    }
    Ok(())
}
