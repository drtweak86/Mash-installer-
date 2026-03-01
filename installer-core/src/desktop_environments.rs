//! Desktop Environment Support
//!
//! Cross-distro desktop environment installation with X11/Wayland support
//! and Raspberry Pi compatibility warnings.

use crate::{
    driver::DistroDriver,
    error::{ErrorSeverity, InstallerError, InstallerStateSnapshot},
};
use anyhow::Result;
use std::collections::HashMap;

/// Supported desktop environments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesktopEnvironment {
    Gnome,
    Kde,
    Xfce,
    Lxqt,
    Mate,
    Cinnamon,
    Budgie,
    Enlightenment,
    Lxde,
    None, // Headless/server mode
}

impl DesktopEnvironment {
    pub fn display_name(&self) -> &'static str {
        match self {
            DesktopEnvironment::Gnome => "GNOME",
            DesktopEnvironment::Kde => "KDE Plasma",
            DesktopEnvironment::Xfce => "XFCE",
            DesktopEnvironment::Lxqt => "LXQt",
            DesktopEnvironment::Mate => "MATE",
            DesktopEnvironment::Cinnamon => "Cinnamon",
            DesktopEnvironment::Budgie => "Budgie",
            DesktopEnvironment::Enlightenment => "Enlightenment",
            DesktopEnvironment::Lxde => "LXDE",
            DesktopEnvironment::None => "None (Headless)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DesktopEnvironment::Gnome => "Modern, feature-rich desktop",
            DesktopEnvironment::Kde => "Highly customizable desktop",
            DesktopEnvironment::Xfce => "Lightweight, traditional desktop",
            DesktopEnvironment::Lxqt => "Lightweight Qt-based desktop",
            DesktopEnvironment::Mate => "Traditional GNOME 2 fork",
            DesktopEnvironment::Cinnamon => "Modern traditional desktop",
            DesktopEnvironment::Budgie => "Modern, elegant desktop",
            DesktopEnvironment::Enlightenment => "Lightweight, customizable",
            DesktopEnvironment::Lxde => "Ultra-lightweight desktop",
            DesktopEnvironment::None => "No desktop environment",
        }
    }

    /// Get Raspberry Pi compatibility warning if applicable
    pub fn pi_warning(&self, is_pi: bool) -> Option<&'static str> {
        if !is_pi {
            return None;
        }

        match self {
            DesktopEnvironment::Gnome => Some("GNOME may be heavy on Raspberry Pi"),
            DesktopEnvironment::Kde => Some("KDE Plasma works well on Pi 4B with 4GB+ RAM"),
            DesktopEnvironment::Xfce => Some("XFCE is recommended for Raspberry Pi"),
            DesktopEnvironment::Lxqt => Some("LXQt is excellent for Raspberry Pi"),
            DesktopEnvironment::Mate => Some("MATE works well on Raspberry Pi"),
            DesktopEnvironment::Cinnamon => Some("Cinnamon may be heavy on Raspberry Pi"),
            DesktopEnvironment::Budgie => Some("Budgie works on Pi 4B with 4GB+ RAM"),
            DesktopEnvironment::Enlightenment => Some("Enlightenment is lightweight for Pi"),
            DesktopEnvironment::Lxde => Some("LXDE is very lightweight for Pi"),
            DesktopEnvironment::None => None,
        }
    }

    /// Check if this DE supports Wayland
    pub fn supports_wayland(&self) -> bool {
        match self {
            DesktopEnvironment::Gnome => true,
            DesktopEnvironment::Kde => true,
            DesktopEnvironment::Xfce => false, // XFCE has experimental Wayland support
            DesktopEnvironment::Lxqt => false, // LXQt has experimental Wayland support
            DesktopEnvironment::Mate => false,
            DesktopEnvironment::Cinnamon => false,
            DesktopEnvironment::Budgie => true,
            DesktopEnvironment::Enlightenment => true,
            DesktopEnvironment::Lxde => false,
            DesktopEnvironment::None => false,
        }
    }
}

/// Display protocol selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayProtocol {
    X11,
    Wayland,
    Auto, // Let the DE decide
}

impl DisplayProtocol {
    pub fn display_name(&self) -> &'static str {
        match self {
            DisplayProtocol::X11 => "X11 (Xorg)",
            DisplayProtocol::Wayland => "Wayland",
            DisplayProtocol::Auto => "Auto (DE default)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DisplayProtocol::X11 => "Traditional X11 display server",
            DisplayProtocol::Wayland => "Modern Wayland display protocol",
            DisplayProtocol::Auto => "Let the desktop environment decide",
        }
    }
}

/// Combined DE selection with protocol
#[derive(Debug, Clone, Copy)]
pub struct DesktopSelection {
    pub environment: DesktopEnvironment,
    pub protocol: DisplayProtocol,
}

impl DesktopSelection {
    pub fn new(environment: DesktopEnvironment, protocol: DisplayProtocol) -> Self {
        Self {
            environment,
            protocol,
        }
    }

    /// Validate the selection - ensure protocol is compatible with DE
    #[allow(clippy::result_large_err)] // Complex validation errors
    pub fn validate(&self) -> Result<(), InstallerError> {
        if self.environment == DesktopEnvironment::None && self.protocol != DisplayProtocol::Auto {
            return Err(InstallerError::new(
                "desktop_environment_validation",
                "Cannot select display protocol when no desktop environment is chosen",
                ErrorSeverity::Fatal,
                anyhow::anyhow!("No desktop environment selected but protocol specified"),
                InstallerStateSnapshot::default(),
                Some("Select a desktop environment first or use Auto protocol".to_string()),
            ));
        }

        if !self.environment.supports_wayland() && self.protocol == DisplayProtocol::Wayland {
            return Err(InstallerError::new(
                "desktop_environment_validation",
                format!(
                    "{} does not support Wayland",
                    self.environment.display_name()
                ),
                ErrorSeverity::Fatal,
                anyhow::anyhow!("Wayland not supported by selected DE"),
                InstallerStateSnapshot::default(),
                Some("Choose X11 protocol or a Wayland-compatible desktop environment".to_string()),
            ));
        }

        Ok(())
    }
}

/// Package mappings for desktop environments across different distros
pub struct DesktopPackages {
    packages: HashMap<DesktopEnvironment, Vec<&'static str>>,
}

impl Default for DesktopPackages {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopPackages {
    pub fn new() -> Self {
        let mut packages = HashMap::new();

        // Debian/Ubuntu packages
        packages.insert(
            DesktopEnvironment::Gnome,
            vec!["gnome", "gdm3", "ubuntu-session"],
        );
        packages.insert(
            DesktopEnvironment::Kde,
            vec!["kde-plasma-desktop", "sddm", "kde-standard"],
        );
        packages.insert(DesktopEnvironment::Xfce, vec!["xfce4", "xfce4-goodies"]);
        packages.insert(DesktopEnvironment::Lxqt, vec!["lxqt", "sddm"]);
        packages.insert(DesktopEnvironment::Mate, vec!["ubuntu-mate-desktop"]);
        packages.insert(
            DesktopEnvironment::Cinnamon,
            vec!["cinnamon-desktop-environment"],
        );
        packages.insert(
            DesktopEnvironment::Budgie,
            vec!["budgie-desktop", "lightdm"],
        );
        packages.insert(
            DesktopEnvironment::Enlightenment,
            vec!["enlightenment", "lightdm"],
        );
        packages.insert(DesktopEnvironment::Lxde, vec!["lxde", "lightdm"]);
        packages.insert(DesktopEnvironment::None, vec![]);

        Self { packages }
    }

    /// Get packages for a specific desktop environment
    pub fn get_packages(&self, de: DesktopEnvironment) -> Vec<&'static str> {
        self.packages.get(&de).cloned().unwrap_or_default()
    }

    /// Get packages translated for a specific distro driver
    pub fn get_translated_packages(
        &self,
        de: DesktopEnvironment,
        driver: &dyn DistroDriver,
    ) -> Vec<String> {
        self.get_packages(de)
            .iter()
            .filter_map(|pkg| driver.translate_package(pkg))
            .collect()
    }
}

/// Check if a desktop environment is currently installed
pub fn is_de_installed(de: DesktopEnvironment, driver: &dyn DistroDriver) -> bool {
    let packages = DesktopPackages::new();
    let de_packages = packages.get_packages(de);

    // Check if any of the core packages are installed
    de_packages
        .iter()
        .any(|pkg| driver.is_package_installed(pkg))
}

/// Get the current display protocol (if detectable)
pub fn detect_current_protocol() -> Option<DisplayProtocol> {
    // Check for Wayland session
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        return Some(DisplayProtocol::Wayland);
    }

    // Check for X11 session
    if std::env::var("DISPLAY").is_ok() {
        return Some(DisplayProtocol::X11);
    }

    None
}

/// Get Raspberry Pi-specific recommendations
pub fn get_pi_recommendations(is_pi: bool) -> Vec<DesktopEnvironment> {
    if is_pi {
        vec![
            DesktopEnvironment::Lxqt,
            DesktopEnvironment::Xfce,
            DesktopEnvironment::Mate,
            DesktopEnvironment::Lxde,
        ]
    } else {
        vec![
            DesktopEnvironment::Gnome,
            DesktopEnvironment::Kde,
            DesktopEnvironment::Xfce,
            DesktopEnvironment::Lxqt,
        ]
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::testing::MockDriver;
    use super::*;

    #[test]
    fn test_de_display_names() {
        assert_eq!(DesktopEnvironment::Gnome.display_name(), "GNOME");
        assert_eq!(DesktopEnvironment::Kde.display_name(), "KDE Plasma");
        assert_eq!(DesktopEnvironment::None.display_name(), "None (Headless)");
    }

    #[test]
    fn test_protocol_display_names() {
        assert_eq!(DisplayProtocol::X11.display_name(), "X11 (Xorg)");
        assert_eq!(DisplayProtocol::Wayland.display_name(), "Wayland");
        assert_eq!(DisplayProtocol::Auto.display_name(), "Auto (DE default)");
    }

    #[test]
    fn test_wayland_support() {
        assert!(DesktopEnvironment::Gnome.supports_wayland());
        assert!(DesktopEnvironment::Kde.supports_wayland());
        assert!(!DesktopEnvironment::Xfce.supports_wayland());
    }

    #[test]
    fn test_pi_warnings() {
        let warning = DesktopEnvironment::Gnome.pi_warning(true);
        assert!(warning.is_some());
        assert!(warning.unwrap().contains("heavy"));

        let no_warning = DesktopEnvironment::Gnome.pi_warning(false);
        assert!(no_warning.is_none());
    }

    #[test]
    fn test_selection_validation() {
        let selection = DesktopSelection::new(DesktopEnvironment::None, DisplayProtocol::Wayland);
        assert!(selection.validate().is_err());

        let valid_selection =
            DesktopSelection::new(DesktopEnvironment::Gnome, DisplayProtocol::Wayland);
        assert!(valid_selection.validate().is_ok());
    }

    #[test]
    fn test_package_mappings() {
        let packages = DesktopPackages::new();
        let gnome_packages = packages.get_packages(DesktopEnvironment::Gnome);
        assert!(gnome_packages.contains(&"gnome"));
        assert!(gnome_packages.contains(&"gdm3"));
    }
}

// Mock driver for testing
#[cfg(test)]
mod testing {
    use super::*;
    use crate::{PkgBackend, PlatformInfo};

    #[allow(dead_code)]
    pub struct MockDriver;

    impl DistroDriver for MockDriver {
        fn name(&self) -> &'static str {
            "Mock"
        }

        fn description(&self) -> &'static str {
            "Mock driver for testing"
        }

        fn matches(&self, _: &PlatformInfo) -> bool {
            true
        }

        fn pkg_backend(&self) -> PkgBackend {
            PkgBackend::Apt
        }

        fn is_package_installed(&self, package: &str) -> bool {
            // Mock some packages as installed
            matches!(package, "gnome" | "kde-plasma-desktop" | "xfce4" | "lxqt")
        }
    }
}
