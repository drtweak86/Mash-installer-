// Driver Test Harness
// Tests each distro driver against the Phase trait contract

use anyhow::Result;
use installer_core::{
    dry_run::DryRunLog, ConfigService, DistroDriver, ErrorSeverity, InstallContext, Phase,
    PhaseContext, PhaseEvent, PhaseObserver, PhaseRunner, PkgBackend, PlatformInfo, ProfileLevel,
    SoftwareTierPlan, UIContext, UserOptionsContext,
};
use std::path::PathBuf;

// Import actual drivers
use installer_arch::driver as arch_driver;
use installer_debian::driver as debian_driver;
use installer_fedora::driver as fedora_driver;

// Test phases that exercise driver functionality
struct PackageTranslationPhase {
    package_name: &'static str,
    expected_result: Option<&'static str>,
}

impl Phase for PackageTranslationPhase {
    fn name(&self) -> &str {
        "package-translation"
    }

    fn description(&self) -> &str {
        "Test package name translation"
    }

    fn should_run(&self, _: &InstallContext) -> bool {
        true
    }

    fn error_severity(&self) -> ErrorSeverity {
        ErrorSeverity::Fatal
    }

    fn execute(&self, ctx: &mut PhaseContext) -> Result<()> {
        let driver = ctx.platform.driver;
        let translated = driver.translate_package(self.package_name);
        let expected = self.expected_result.map(|s| s.to_string());

        match (&translated, &expected) {
            (Some(t), Some(e)) if t == e => Ok(()),
            (None, None) => Ok(()),
            _ => Err(anyhow::anyhow!(
                "Package translation mismatch for {}: got {:?}, expected {:?}",
                self.package_name,
                translated,
                expected
            )),
        }
    }
}

struct ServiceNamePhase {
    service_name: installer_core::ServiceName,
    expected_unit: &'static str,
}

impl Phase for ServiceNamePhase {
    fn name(&self) -> &str {
        "service-name"
    }

    fn description(&self) -> &str {
        "Test service name mapping"
    }

    fn should_run(&self, _: &InstallContext) -> bool {
        true
    }

    fn error_severity(&self) -> ErrorSeverity {
        ErrorSeverity::Fatal
    }

    fn execute(&self, ctx: &mut PhaseContext) -> Result<()> {
        let driver = ctx.platform.driver;
        let actual = driver.service_unit(self.service_name);

        if actual == self.expected_unit {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Service name mismatch: got {}, expected {}",
                actual,
                self.expected_unit
            ))
        }
    }
}

struct DryRunLoggingPhase {
    package_name: &'static str,
}

impl Phase for DryRunLoggingPhase {
    fn name(&self) -> &str {
        "dry-run-logging"
    }

    fn description(&self) -> &str {
        "Test dry-run logging"
    }

    fn should_run(&self, _: &InstallContext) -> bool {
        true
    }

    fn error_severity(&self) -> ErrorSeverity {
        ErrorSeverity::Fatal
    }

    fn execute(&self, ctx: &mut PhaseContext) -> Result<()> {
        let driver = ctx.platform.driver;
        let translated = driver.translate_package(self.package_name);

        // Log the dry-run entry
        ctx.dry_run_log.record(
            self.name(),
            "package translation",
            Some(format!(
                "Would translate package: {} -> {:?}",
                self.package_name, translated
            )),
        );

        Ok(())
    }
}

// Helper to build install context for a specific driver
fn build_context_for_driver(driver: &'static dyn DistroDriver) -> Result<InstallContext> {
    let config_service = ConfigService::load()?;
    let platform = PlatformInfo {
        arch: "x86_64".into(),
        distro: driver.name().into(),
        distro_version: "0".into(),
        distro_codename: "test".into(),
        distro_family: match driver.pkg_backend() {
            PkgBackend::Apt => "debian",
            PkgBackend::Pacman => "arch",
            PkgBackend::Dnf => "fedora",
        }
        .into(),
        pi_model: None,
    };

    let options = UserOptionsContext {
        profile: ProfileLevel::Minimal,
        staging_dir: PathBuf::from("/tmp/mash-test"),
        dry_run: false,
        interactive: false,
        enable_argon: false,
        enable_p10k: false,
        docker_data_root: false,
        software_plan: SoftwareTierPlan::default(),
    };
    let localization = installer_core::localization::Localization::load_default()?;

    Ok(InstallContext {
        options,
        platform: installer_core::PlatformContext {
            config_service,
            platform,
            driver_name: driver.name(),
            driver,
            pkg_backend: driver.pkg_backend(),
        },
        ui: UIContext,
        localization,
        rollback: installer_core::RollbackManager::new(),
        dry_run_log: DryRunLog::new(),
    })
}

// Test observer that captures dry-run entries
struct DryRunObserver {
    _dry_run_entries: Vec<(String, String)>,
}

impl DryRunObserver {
    fn new() -> Self {
        Self {
            _dry_run_entries: Vec::new(),
        }
    }
}

impl PhaseObserver for DryRunObserver {
    fn on_event(&mut self, _event: PhaseEvent) {
        // Dry-run entries are captured via DryRunLog, not PhaseEvent
        // This observer is kept for future event monitoring
    }
}

// Test each driver
#[test]
fn test_arch_driver_package_translation() -> Result<()> {
    let driver = arch_driver();
    let ctx = build_context_for_driver(driver)?;

    let phases: Vec<Box<dyn Phase>> = vec![
        Box::new(PackageTranslationPhase {
            package_name: "build-essential",
            expected_result: Some("base-devel"),
        }),
        Box::new(PackageTranslationPhase {
            package_name: "docker-ce",
            expected_result: Some("docker"),
        }),
        Box::new(PackageTranslationPhase {
            package_name: "apt-transport-https",
            expected_result: None,
        }),
    ];

    let runner = PhaseRunner::from_phases(phases);
    let mut observer = DryRunObserver::new();

    let result = runner.run(&ctx, &mut observer, None)?;

    assert_eq!(result.completed_phases.len(), 3);
    assert!(result.errors.is_empty());

    Ok(())
}

#[test]
fn test_debian_driver_package_translation() -> Result<()> {
    let driver = debian_driver();
    let ctx = build_context_for_driver(driver)?;

    let phases: Vec<Box<dyn Phase>> = vec![
        Box::new(PackageTranslationPhase {
            package_name: "build-essential",
            expected_result: Some("build-essential"),
        }),
        Box::new(PackageTranslationPhase {
            package_name: "docker-ce",
            expected_result: Some("docker-ce"),
        }),
    ];

    let runner = PhaseRunner::from_phases(phases);
    let mut observer = DryRunObserver::new();

    let result = runner.run(&ctx, &mut observer, None)?;

    assert_eq!(result.completed_phases.len(), 2);
    assert!(result.errors.is_empty());

    Ok(())
}

#[test]
fn test_fedora_driver_package_translation() -> Result<()> {
    let driver = fedora_driver();
    let ctx = build_context_for_driver(driver)?;

    let phases: Vec<Box<dyn Phase>> = vec![
        Box::new(PackageTranslationPhase {
            package_name: "build-essential",
            expected_result: Some("build-essential"),
        }),
        Box::new(PackageTranslationPhase {
            package_name: "docker-ce",
            expected_result: Some("docker"),
        }),
    ];

    let runner = PhaseRunner::from_phases(phases);
    let mut observer = DryRunObserver::new();

    let result = runner.run(&ctx, &mut observer, None)?;

    assert_eq!(result.completed_phases.len(), 2);
    assert!(result.errors.is_empty());

    Ok(())
}

#[test]
fn test_arch_driver_service_names() -> Result<()> {
    let driver = arch_driver();
    let ctx = build_context_for_driver(driver)?;

    let phases: Vec<Box<dyn Phase>> = vec![
        Box::new(ServiceNamePhase {
            service_name: installer_core::ServiceName::Docker,
            expected_unit: "docker.service",
        }),
        Box::new(ServiceNamePhase {
            service_name: installer_core::ServiceName::ArgonOne,
            expected_unit: "argononed.service",
        }),
    ];

    let runner = PhaseRunner::from_phases(phases);
    let mut observer = DryRunObserver::new();

    let result = runner.run(&ctx, &mut observer, None)?;

    assert_eq!(result.completed_phases.len(), 2);
    assert!(result.errors.is_empty());

    Ok(())
}

#[test]
fn test_dry_run_logging_for_all_drivers() -> Result<()> {
    let drivers = vec![arch_driver(), debian_driver(), fedora_driver()];

    for driver in drivers {
        let ctx = build_context_for_driver(driver)?;

        let phases: Vec<Box<dyn Phase>> = vec![Box::new(DryRunLoggingPhase {
            package_name: "docker-ce",
        })];

        let runner = PhaseRunner::from_phases(phases);
        let mut observer = DryRunObserver::new();

        let result = runner.run(&ctx, &mut observer, None)?;

        assert_eq!(result.completed_phases.len(), 1);
        assert!(result.errors.is_empty());

        // Verify dry-run entries were recorded
        let entries = ctx.dry_run_log.entries();
        assert!(!entries.is_empty());
    }

    Ok(())
}

#[test]
fn test_driver_matches_correct_platform() -> Result<()> {
    let arch = arch_driver();
    let debian = debian_driver();
    let fedora = fedora_driver();

    // Arch should match arch family
    let arch_platform = PlatformInfo {
        arch: "x86_64".into(),
        distro: "Arch".into(),
        distro_version: "1".into(),
        distro_codename: "".into(),
        distro_family: "arch".into(),
        pi_model: None,
    };
    assert!(arch.matches(&arch_platform));
    assert!(!debian.matches(&arch_platform));
    assert!(!fedora.matches(&arch_platform));

    // Debian should match debian family
    let debian_platform = PlatformInfo {
        arch: "x86_64".into(),
        distro: "Ubuntu".into(),
        distro_version: "22.04".into(),
        distro_codename: "jammy".into(),
        distro_family: "debian".into(),
        pi_model: None,
    };
    assert!(!arch.matches(&debian_platform));
    assert!(debian.matches(&debian_platform));
    assert!(!fedora.matches(&debian_platform));

    // Fedora should match fedora family
    let fedora_platform = PlatformInfo {
        arch: "x86_64".into(),
        distro: "Fedora".into(),
        distro_version: "38".into(),
        distro_codename: "".into(),
        distro_family: "fedora".into(),
        pi_model: None,
    };
    assert!(!arch.matches(&fedora_platform));
    assert!(!debian.matches(&fedora_platform));
    assert!(fedora.matches(&fedora_platform));

    Ok(())
}
