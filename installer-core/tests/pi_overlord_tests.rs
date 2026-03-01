//! Pi Overlord Integration Tests
//!
//! Comprehensive test suite for Pi Overlord cross-distro package mapping
//! and installation functionality.

use anyhow::Result;
use installer_core::pi_overlord::{
    PackageCategory, PackageMapping, PiOverlord, PiOverlordPackages,
};
use installer_core::PkgBackend;

// Mock drivers for testing different distributions
struct MockFedoraDriver;
impl installer_core::DistroDriver for MockFedoraDriver {
    fn name(&self) -> &'static str {
        "fedora"
    }
    fn description(&self) -> &'static str {
        "Mock Fedora Driver"
    }
    fn matches(&self, _info: &installer_core::PlatformInfo) -> bool {
        false
    }
    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Dnf
    }
}

struct MockDebianDriver;
impl installer_core::DistroDriver for MockDebianDriver {
    fn name(&self) -> &'static str {
        "debian"
    }
    fn description(&self) -> &'static str {
        "Mock Debian Driver"
    }
    fn matches(&self, _info: &installer_core::PlatformInfo) -> bool {
        false
    }
    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Apt
    }
}

struct MockArchDriver;
impl installer_core::DistroDriver for MockArchDriver {
    fn name(&self) -> &'static str {
        "arch"
    }
    fn description(&self) -> &'static str {
        "Mock Arch Driver"
    }
    fn matches(&self, _info: &installer_core::PlatformInfo) -> bool {
        false
    }
    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Pacman
    }
}

#[test]
fn test_package_mapping_builder_pattern() -> Result<()> {
    let mapping = PackageMapping::new()
        .fedora(&["fedora-pkg1", "fedora-pkg2"])
        .debian(&["debian-pkg1", "debian-pkg2"])
        .arch(&["arch-pkg1", "arch-pkg2"])
        .desc("Test package mapping");

    assert_eq!(mapping.fedora, vec!["fedora-pkg1", "fedora-pkg2"]);
    assert_eq!(mapping.debian, vec!["debian-pkg1", "debian-pkg2"]);
    assert_eq!(mapping.arch, vec!["arch-pkg1", "arch-pkg2"]);
    assert_eq!(mapping.description, "Test package mapping");

    Ok(())
}

#[test]
fn test_pi_overlord_packages_initialization() -> Result<()> {
    let packages = PiOverlordPackages::new();

    // Verify all expected categories are present
    let expected_categories = [
        PackageCategory::CoreTools,
        PackageCategory::KdePlasma,
        PackageCategory::Terminals,
        PackageCategory::Fonts,
        PackageCategory::Shell,
        PackageCategory::RustToolchain,
        PackageCategory::CargoTools,
        PackageCategory::BuildTools,
        PackageCategory::GitForge,
        PackageCategory::Containers,
        PackageCategory::Snapshots,
        PackageCategory::Network,
        PackageCategory::Languages,
        PackageCategory::Performance,
        PackageCategory::Wayland,
        PackageCategory::Workflow,
        PackageCategory::ArgonOne,
    ];

    for category in expected_categories {
        assert!(
            packages.get_mapping(category).is_some(),
            "Missing package mapping for {:?}",
            category
        );
    }

    Ok(())
}

#[test]
fn test_cross_distro_package_mapping() -> Result<()> {
    let packages = PiOverlordPackages::new();

    // Test CoreTools category
    let core_tools = packages.get_mapping(PackageCategory::CoreTools).unwrap();

    // Test Fedora packages
    let fedora_pkgs = core_tools.get_for_distro(&MockFedoraDriver);
    assert!(fedora_pkgs.contains(&"btop".to_string()));
    assert!(fedora_pkgs.contains(&"htop".to_string()));
    assert!(fedora_pkgs.contains(&"fd-find".to_string())); // Fedora-specific

    // Test Debian packages
    let debian_pkgs = core_tools.get_for_distro(&MockDebianDriver);
    assert!(debian_pkgs.contains(&"btop".to_string()));
    assert!(debian_pkgs.contains(&"htop".to_string()));
    assert!(debian_pkgs.contains(&"fd-find".to_string())); // Debian-specific

    // Test Arch packages
    let arch_pkgs = core_tools.get_for_distro(&MockArchDriver);
    assert!(arch_pkgs.contains(&"btop".to_string()));
    assert!(arch_pkgs.contains(&"htop".to_string()));
    assert!(arch_pkgs.contains(&"fd".to_string())); // Arch-specific

    Ok(())
}

#[test]
fn test_kde_plasma_package_mapping() -> Result<()> {
    let packages = PiOverlordPackages::new();
    let kde_mapping = packages.get_mapping(PackageCategory::KdePlasma).unwrap();

    // Test Fedora KDE packages
    let fedora_pkgs = kde_mapping.get_for_distro(&MockFedoraDriver);
    assert!(fedora_pkgs.contains(&"sddm".to_string()));
    assert!(fedora_pkgs.contains(&"plasma-workspace".to_string()));
    assert!(fedora_pkgs.contains(&"xorg-x11-server-Xorg".to_string()));

    // Test Debian KDE packages
    let debian_pkgs = kde_mapping.get_for_distro(&MockDebianDriver);
    assert!(debian_pkgs.contains(&"sddm".to_string()));
    assert!(debian_pkgs.contains(&"plasma-workspace".to_string()));
    assert!(debian_pkgs.contains(&"xserver-xorg".to_string()));

    Ok(())
}

#[test]
fn test_rust_toolchain_package_mapping() -> Result<()> {
    let packages = PiOverlordPackages::new();
    let rust_mapping = packages
        .get_mapping(PackageCategory::RustToolchain)
        .unwrap();

    // Test Fedora Rust build deps
    let fedora_pkgs = rust_mapping.get_for_distro(&MockFedoraDriver);
    assert!(fedora_pkgs.contains(&"gcc".to_string()));
    assert!(fedora_pkgs.contains(&"gcc-c++".to_string()));
    assert!(fedora_pkgs.contains(&"openssl-devel".to_string()));

    // Test Debian Rust build deps
    let debian_pkgs = rust_mapping.get_for_distro(&MockDebianDriver);
    assert!(debian_pkgs.contains(&"gcc".to_string()));
    assert!(debian_pkgs.contains(&"g++".to_string()));
    assert!(debian_pkgs.contains(&"libssl-dev".to_string()));

    Ok(())
}

#[test]
fn test_container_package_mapping() -> Result<()> {
    let packages = PiOverlordPackages::new();
    let container_mapping = packages.get_mapping(PackageCategory::Containers).unwrap();

    // All distros should have the same container packages
    let fedora_pkgs = container_mapping.get_for_distro(&MockFedoraDriver);
    let debian_pkgs = container_mapping.get_for_distro(&MockDebianDriver);
    let arch_pkgs = container_mapping.get_for_distro(&MockArchDriver);

    for pkgs in [fedora_pkgs, debian_pkgs, arch_pkgs] {
        assert!(pkgs.contains(&"podman".to_string()));
        assert!(pkgs.contains(&"buildah".to_string()));
        assert!(pkgs.contains(&"skopeo".to_string()));
        assert!(pkgs.contains(&"podman-compose".to_string()));
    }

    Ok(())
}

#[test]
fn test_pi_overlord_integration() -> Result<()> {
    let pi_overlord = PiOverlord::new();

    // Test that we can access package mappings through the main API
    let mapping = pi_overlord.get_package_mapping(PackageCategory::CoreTools);
    assert!(mapping.is_some());

    let core_tools = mapping.unwrap();
    let fedora_pkgs = core_tools.get_for_distro(&MockFedoraDriver);
    assert!(!fedora_pkgs.is_empty());

    Ok(())
}

#[test]
fn test_all_categories_have_packages() -> Result<()> {
    let packages = PiOverlordPackages::new();

    // Ensure every category has packages for at least one distro
    // Note: CargoTools is special - it uses cargo install, not package managers
    for category in [
        PackageCategory::CoreTools,
        PackageCategory::KdePlasma,
        PackageCategory::Terminals,
        PackageCategory::Fonts,
        PackageCategory::Shell,
        PackageCategory::RustToolchain,
        PackageCategory::BuildTools,
        PackageCategory::GitForge,
        PackageCategory::Containers,
        PackageCategory::Snapshots,
        PackageCategory::Network,
        PackageCategory::Languages,
        PackageCategory::Performance,
        PackageCategory::Wayland,
        PackageCategory::Workflow,
        PackageCategory::ArgonOne,
    ] {
        if let Some(mapping) = packages.get_mapping(category) {
            let fedora_pkgs = mapping.get_for_distro(&MockFedoraDriver);
            let debian_pkgs = mapping.get_for_distro(&MockDebianDriver);
            let arch_pkgs = mapping.get_for_distro(&MockArchDriver);

            // At least one distro should have packages for each category
            assert!(
                !fedora_pkgs.is_empty() || !debian_pkgs.is_empty() || !arch_pkgs.is_empty(),
                "Category {:?} has no packages for any distro",
                category
            );
        }
    }

    // CargoTools is special - it should have empty package lists since cargo tools
    // are installed via `cargo install`, not package managers
    if let Some(cargo_mapping) = packages.get_mapping(PackageCategory::CargoTools) {
        let fedora_pkgs = cargo_mapping.get_for_distro(&MockFedoraDriver);
        let debian_pkgs = cargo_mapping.get_for_distro(&MockDebianDriver);
        let arch_pkgs = cargo_mapping.get_for_distro(&MockArchDriver);

        // CargoTools should have empty package lists for all distros
        assert!(
            fedora_pkgs.is_empty(),
            "CargoTools should have no Fedora packages"
        );
        assert!(
            debian_pkgs.is_empty(),
            "CargoTools should have no Debian packages"
        );
        assert!(
            arch_pkgs.is_empty(),
            "CargoTools should have no Arch packages"
        );
    }

    Ok(())
}

#[test]
fn test_package_descriptions() -> Result<()> {
    let packages = PiOverlordPackages::new();

    // Test that categories have meaningful descriptions
    let core_tools = packages.get_mapping(PackageCategory::CoreTools).unwrap();
    assert!(!core_tools.description.is_empty());
    assert!(core_tools.description.contains("CLI tools"));

    let kde_plasma = packages.get_mapping(PackageCategory::KdePlasma).unwrap();
    assert!(!kde_plasma.description.is_empty());
    assert!(kde_plasma.description.contains("KDE Plasma"));

    Ok(())
}
