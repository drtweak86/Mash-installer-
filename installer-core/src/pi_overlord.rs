//! Pi Overlord — Cross-distro package mapping and installation system
//!
//! Transmogrification of pi_overlord_grimoire.py into a cross-distro Rust module
//! that provides package name mappings and installation capabilities for multiple
//! Linux distributions.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::context::PhaseContext;
use crate::driver::DistroDriver;

use crate::package_manager::installer_for;

// ── Package Categories ──────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PackageCategory {
    CoreTools,
    KdePlasma,
    Terminals,
    Fonts,
    Shell,
    RustToolchain,
    CargoTools,
    BuildTools,
    GitForge,
    Containers,
    Snapshots,
    Network,
    Languages,
    Performance,
    Wayland,
    Workflow,
    ArgonOne,
}

// ── Package Mapping ────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMapping {
    pub fedora: Vec<&'static str>,
    pub debian: Vec<&'static str>,
    pub arch: Vec<&'static str>,
    pub description: &'static str,
}

impl Default for PackageMapping {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageMapping {
    pub fn new() -> Self {
        Self {
            fedora: Vec::new(),
            debian: Vec::new(),
            arch: Vec::new(),
            description: "",
        }
    }

    pub fn fedora(mut self, packages: &[&'static str]) -> Self {
        self.fedora.extend_from_slice(packages);
        self
    }

    pub fn debian(mut self, packages: &[&'static str]) -> Self {
        self.debian.extend_from_slice(packages);
        self
    }

    pub fn arch(mut self, packages: &[&'static str]) -> Self {
        self.arch.extend_from_slice(packages);
        self
    }

    pub fn desc(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    pub fn get_for_distro(&self, driver: &dyn DistroDriver) -> Vec<String> {
        match driver.name() {
            "fedora" => self.fedora.iter().map(|s| s.to_string()).collect(),
            "debian" | "ubuntu" => self.debian.iter().map(|s| s.to_string()).collect(),
            "arch" => self.arch.iter().map(|s| s.to_string()).collect(),
            _ => Vec::new(),
        }
    }
}

// ── Pi Overlord Package Database ───────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PiOverlordPackages {
    mappings: HashMap<PackageCategory, PackageMapping>,
}

impl Default for PiOverlordPackages {
    fn default() -> Self {
        let mut mappings = HashMap::new();

        // Phase 1: Core CLI Tools
        mappings.insert(
            PackageCategory::CoreTools,
            PackageMapping::new()
                .fedora(&[
                    "btop",
                    "htop",
                    "nvtop",
                    "iotop",
                    "powertop",
                    "lm_sensors",
                    "smartmontools",
                    "usbutils",
                    "pciutils",
                    "lsof",
                    "strace",
                    "tcpdump",
                    "bind-utils",
                    "ncdu",
                    "tree",
                    "ripgrep",
                    "fd-find",
                    "bat",
                    "fzf",
                    "eza",
                    "wget",
                    "curl",
                    "aria2",
                    "rsync",
                    "tmux",
                    "screen",
                    "zstd",
                    "unzip",
                    "p7zip",
                    "p7zip-plugins",
                    "which",
                    "util-linux-user",
                    "pv",
                    "jq",
                ])
                .debian(&[
                    "btop",
                    "htop",
                    "nvtop",
                    "iotop",
                    "powertop",
                    "lm-sensors",
                    "smartmontools",
                    "usbutils",
                    "pciutils",
                    "lsof",
                    "strace",
                    "tcpdump",
                    "dnsutils",
                    "ncdu",
                    "tree",
                    "ripgrep",
                    "fd-find",
                    "bat",
                    "fzf",
                    "eza",
                    "wget",
                    "curl",
                    "aria2",
                    "rsync",
                    "tmux",
                    "screen",
                    "zstd",
                    "unzip",
                    "p7zip",
                    "p7zip-full",
                    "which",
                    "util-linux",
                    "pv",
                    "jq",
                ])
                .arch(&[
                    "btop",
                    "htop",
                    "nvtop",
                    "iotop",
                    "powertop",
                    "lm_sensors",
                    "smartmontools",
                    "usbutils",
                    "pciutils",
                    "lsof",
                    "strace",
                    "tcpdump",
                    "bind",
                    "ncdu",
                    "tree",
                    "ripgrep",
                    "fd",
                    "bat",
                    "fzf",
                    "eza",
                    "wget",
                    "curl",
                    "aria2",
                    "rsync",
                    "tmux",
                    "screen",
                    "zstd",
                    "unzip",
                    "p7zip",
                    "which",
                    "util-linux",
                    "pv",
                    "jq",
                ])
                .desc("Core CLI tools and utilities"),
        );

        // Phase 2: KDE Plasma
        mappings.insert(
            PackageCategory::KdePlasma,
            PackageMapping::new()
                .fedora(&[
                    "sddm",
                    "xorg-x11-server-Xorg",
                    "xorg-x11-xinit",
                    "xorg-x11-utils",
                    "plasma-workspace",
                    "plasma-desktop",
                    "kwin",
                    "kdeplasma-addons",
                    "plasma-nm",
                    "plasma-pa",
                    "kscreen",
                    "powerdevil",
                    "bluedevil",
                    "kate",
                    "dolphin",
                    "ark",
                    "spectacle",
                    "gwenview",
                    "okular",
                    "kcalc",
                    "playerctl",
                    "xdg-utils",
                    "xdg-user-dirs",
                    "network-manager-applet",
                ])
                .debian(&[
                    "sddm",
                    "xserver-xorg",
                    "xinit",
                    "x11-utils",
                    "plasma-workspace",
                    "plasma-desktop",
                    "kwin-x11",
                    "plasma-desktop-data",
                    "plasma-nm",
                    "plasma-pa",
                    "kscreen",
                    "powerdevil",
                    "bluedevil",
                    "kate",
                    "dolphin",
                    "ark",
                    "spectacle",
                    "gwenview",
                    "okular",
                    "kcalc",
                    "playerctl",
                    "xdg-utils",
                    "xdg-user-dirs",
                    "network-manager",
                ])
                .arch(&[
                    "sddm",
                    "xorg-server",
                    "xorg-xinit",
                    "xorg-utils",
                    "plasma",
                    "plasma-desktop",
                    "kwin",
                    "plasma-nm",
                    "plasma-pa",
                    "kscreen",
                    "powerdevil",
                    "bluedevil",
                    "kate",
                    "dolphin",
                    "ark",
                    "spectacle",
                    "gwenview",
                    "okular",
                    "kcalc",
                    "playerctl",
                    "xdg-utils",
                    "xdg-user-dirs",
                    "networkmanager",
                ])
                .desc("KDE Plasma desktop environment with X11"),
        );

        // Phase 3: Terminals
        mappings.insert(
            PackageCategory::Terminals,
            PackageMapping::new()
                .fedora(&["kitty"])
                .debian(&["kitty"])
                .arch(&["kitty"])
                .desc("Kitty terminal emulator"),
        );

        // Phase 4: Fonts
        mappings.insert(
            PackageCategory::Fonts,
            PackageMapping::new()
                .fedora(&["jetbrains-mono-fonts", "fira-code-fonts", "fontconfig"])
                .debian(&["fonts-jetbrains-mono", "fonts-firacode", "fontconfig"])
                .arch(&["ttf-jetbrains-mono", "ttf-firacode-nerd", "fontconfig"])
                .desc("JetBrains Mono and Fira Code fonts"),
        );

        // Phase 5: Shell
        mappings.insert(
            PackageCategory::Shell,
            PackageMapping::new()
                .fedora(&["zsh", "git", "curl"])
                .debian(&["zsh", "git", "curl"])
                .arch(&["zsh", "git", "curl"])
                .desc("Zsh shell with git and curl"),
        );

        // Phase 6: Rust Toolchain
        mappings.insert(
            PackageCategory::RustToolchain,
            PackageMapping::new()
                .fedora(&[
                    "gcc",
                    "gcc-c++",
                    "openssl-devel",
                    "sqlite-devel",
                    "zlib-devel",
                    "libffi-devel",
                ])
                .debian(&[
                    "gcc",
                    "g++",
                    "libssl-dev",
                    "libsqlite3-dev",
                    "zlib1g-dev",
                    "libffi-dev",
                ])
                .arch(&["gcc", "clang", "openssl", "sqlite", "zlib", "libffi"])
                .desc("Rust build prerequisites"),
        );

        // Phase 7: Cargo Tools
        mappings.insert(
            PackageCategory::CargoTools,
            PackageMapping::new().desc("Cargo tools (installed via cargo install)"),
        );

        // Phase 8: Build Tools
        mappings.insert(
            PackageCategory::BuildTools,
            PackageMapping::new()
                .fedora(&[
                    "clang",
                    "llvm",
                    "lldb",
                    "cmake",
                    "ninja-build",
                    "meson",
                    "make",
                    "autoconf",
                    "automake",
                    "libtool",
                    "pkgconf-pkg-config",
                    "uboot-tools",
                    "dtc",
                    "btrfs-progs",
                    "parted",
                    "dosfstools",
                    "cryptsetup",
                    "lvm2",
                    "openssl-devel",
                    "sqlite-devel",
                    "zlib-devel",
                    "libffi-devel",
                    "libxml2-devel",
                    "libcurl-devel",
                    "readline-devel",
                ])
                .debian(&[
                    "clang",
                    "llvm",
                    "lldb",
                    "cmake",
                    "ninja-build",
                    "meson",
                    "make",
                    "autoconf",
                    "automake",
                    "libtool",
                    "pkg-config",
                    "u-boot-tools",
                    "device-tree-compiler",
                    "btrfs-progs",
                    "parted",
                    "dosfstools",
                    "cryptsetup",
                    "lvm2",
                    "libssl-dev",
                    "libsqlite3-dev",
                    "zlib1g-dev",
                    "libffi-dev",
                    "libxml2-dev",
                    "libcurl4-openssl-dev",
                    "libreadline-dev",
                ])
                .arch(&[
                    "clang",
                    "llvm",
                    "lldb",
                    "cmake",
                    "ninja",
                    "meson",
                    "make",
                    "autoconf",
                    "automake",
                    "libtool",
                    "pkgconf",
                    "uboot-tools",
                    "dtc",
                    "btrfs-progs",
                    "parted",
                    "dosfstools",
                    "cryptsetup",
                    "lvm2",
                    "openssl",
                    "sqlite",
                    "zlib",
                    "libffi",
                    "libxml2",
                    "curl",
                    "readline",
                ])
                .desc("Build tools and development libraries"),
        );

        // Phase 9: Git Forge
        mappings.insert(
            PackageCategory::GitForge,
            PackageMapping::new()
                .fedora(&["git", "git-lfs", "gh", "tig", "git-delta"])
                .debian(&["git", "git-lfs", "gh", "tig", "git-delta"])
                .arch(&["git", "git-lfs", "github-cli", "tig", "git-delta"])
                .desc("Git tools and GitHub CLI"),
        );

        // Phase 10: Containers
        mappings.insert(
            PackageCategory::Containers,
            PackageMapping::new()
                .fedora(&[
                    "podman",
                    "buildah",
                    "skopeo",
                    "podman-compose",
                    "toolbox",
                    "aardvark-dns",
                ])
                .debian(&[
                    "podman",
                    "buildah",
                    "skopeo",
                    "podman-compose",
                    "toolbox",
                    "aardvark-dns",
                ])
                .arch(&[
                    "podman",
                    "buildah",
                    "skopeo",
                    "podman-compose",
                    "toolbox",
                    "aardvark-dns",
                ])
                .desc("Podman container stack"),
        );

        // Phase 11: Snapshots
        mappings.insert(
            PackageCategory::Snapshots,
            PackageMapping::new()
                .fedora(&["snapper", "snapper-plugins", "python3-dnf-plugin-snapper"])
                .debian(&["snapper", "snapper-gui", "snapper-tools"])
                .arch(&["snapper", "snapper-support"])
                .desc("Btrfs snapshots with Snapper"),
        );

        // Phase 12: Network
        mappings.insert(
            PackageCategory::Network,
            PackageMapping::new()
                .fedora(&[
                    "nmap",
                    "iperf3",
                    "wireguard-tools",
                    "firewalld",
                    "fail2ban",
                    "openssh-server",
                    "openssh-clients",
                ])
                .debian(&[
                    "nmap",
                    "iperf3",
                    "wireguard",
                    "ufw",
                    "fail2ban",
                    "openssh-server",
                    "openssh-client",
                ])
                .arch(&[
                    "nmap",
                    "iperf3",
                    "wireguard-tools",
                    "ufw",
                    "fail2ban",
                    "openssh",
                ])
                .desc("Network tools and security"),
        );

        // Phase 13: Languages
        mappings.insert(
            PackageCategory::Languages,
            PackageMapping::new()
                .fedora(&[
                    "python3",
                    "python3-pip",
                    "python3-virtualenv",
                    "python3-devel",
                    "nodejs",
                    "npm",
                    "golang",
                    "lua",
                    "lua-devel",
                    "shellcheck",
                    "shfmt",
                ])
                .debian(&[
                    "python3",
                    "python3-pip",
                    "python3-venv",
                    "python3-dev",
                    "nodejs",
                    "npm",
                    "golang",
                    "lua5.4",
                    "liblua5.4-dev",
                    "shellcheck",
                    "shfmt",
                ])
                .arch(&[
                    "python",
                    "python-pip",
                    "nodejs",
                    "npm",
                    "go",
                    "lua",
                    "shellcheck",
                    "shfmt",
                ])
                .desc("Multi-language development stack"),
        );

        // Phase 14: Performance
        mappings.insert(
            PackageCategory::Performance,
            PackageMapping::new()
                .fedora(&["earlyoom", "tuned", "irqbalance", "zram-generator"])
                .debian(&["earlyoom", "tuned", "irqbalance", "zram-tools"])
                .arch(&["earlyoom", "tuned", "irqbalance", "zram-generator"])
                .desc("Performance tuning tools"),
        );

        // Phase 15: Wayland
        mappings.insert(
            PackageCategory::Wayland,
            PackageMapping::new()
                .fedora(&[
                    "wl-clipboard",
                    "grim",
                    "slurp",
                    "direnv",
                    "flatpak",
                    "helix",
                    "neovim",
                    "alacritty",
                ])
                .debian(&[
                    "wl-clipboard",
                    "grim",
                    "slurp",
                    "direnv",
                    "flatpak",
                    "helix",
                    "neovim",
                    "alacritty",
                ])
                .arch(&[
                    "wl-clipboard",
                    "grim",
                    "slurp",
                    "direnv",
                    "flatpak",
                    "helix",
                    "neovim",
                    "alacritty",
                ])
                .desc("Wayland tools and editors"),
        );

        // Phase 16: Workflow
        mappings.insert(
            PackageCategory::Workflow,
            PackageMapping::new()
                .fedora(&[
                    "fastfetch",
                    "neofetch",
                    "papirus-icon-theme",
                    "atuin",
                    "zoxide",
                ])
                .debian(&[
                    "fastfetch",
                    "neofetch",
                    "papirus-icon-theme",
                    "atuin",
                    "zoxide",
                ])
                .arch(&[
                    "fastfetch",
                    "neofetch",
                    "papirus-icon-theme",
                    "atuin",
                    "zoxide",
                ])
                .desc("Workflow and appearance tools"),
        );

        // Phase 19: Argon One
        mappings.insert(
            PackageCategory::ArgonOne,
            PackageMapping::new()
                .fedora(&["gcc", "make", "dtc", "i2c-tools", "i2c-tools-devel"])
                .debian(&[
                    "gcc",
                    "make",
                    "device-tree-compiler",
                    "i2c-tools",
                    "libi2c-dev",
                ])
                .arch(&["gcc", "make", "dtc", "i2c-tools"])
                .desc("Argon One case fan control build dependencies"),
        );

        Self { mappings }
    }
}

impl PiOverlordPackages {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_mapping(&self, category: PackageCategory) -> Option<&PackageMapping> {
        self.mappings.get(&category)
    }

    pub fn get_packages_for_distro(
        &self,
        category: PackageCategory,
        driver: &dyn DistroDriver,
    ) -> Vec<String> {
        self.get_mapping(category)
            .map(|m| m.get_for_distro(driver))
            .unwrap_or_default()
    }

    /// Install packages for a specific category using the provided distro driver
    pub async fn install_category(
        &self,
        category: PackageCategory,
        driver: &dyn DistroDriver,
        ctx: &mut PhaseContext<'_>,
    ) -> Result<()> {
        let packages = self.get_packages_for_distro(category, driver);
        if packages.is_empty() {
            ctx.record_warning(format!(
                "No packages defined for {:?} on this distro",
                category
            ));
            return Ok(());
        }

        ctx.record_action(format!(
            "📦 Installing {} packages for {:?}",
            packages.len(),
            category
        ));

        let package_refs: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
        installer_for(driver).ensure_packages(&package_refs, ctx.options.dry_run)?;

        ctx.record_action(format!(
            "✅ Installed {} packages for {:?}",
            packages.len(),
            category
        ));

        Ok(())
    }

    /// Run the complete Pi Overlord installation sequence
    pub async fn run_full_sequence(
        &self,
        driver: &dyn DistroDriver,
        ctx: &mut PhaseContext<'_>,
    ) -> Result<()> {
        ctx.record_action("🧠 Starting Pi Overlord installation sequence...");

        // Phase 1: Core Tools
        self.install_category(PackageCategory::CoreTools, driver, ctx)
            .await?;

        // Phase 2: KDE Plasma
        self.install_category(PackageCategory::KdePlasma, driver, ctx)
            .await?;

        // Phase 3: Terminals
        self.install_category(PackageCategory::Terminals, driver, ctx)
            .await?;

        // Phase 4: Fonts
        self.install_category(PackageCategory::Fonts, driver, ctx)
            .await?;

        // Phase 5: Shell
        self.install_category(PackageCategory::Shell, driver, ctx)
            .await?;

        // Phase 6: Rust Toolchain (build deps only, rustup handled separately)
        self.install_category(PackageCategory::RustToolchain, driver, ctx)
            .await?;

        // Phase 8: Build Tools
        self.install_category(PackageCategory::BuildTools, driver, ctx)
            .await?;

        // Phase 9: Git Forge
        self.install_category(PackageCategory::GitForge, driver, ctx)
            .await?;

        // Phase 10: Containers
        self.install_category(PackageCategory::Containers, driver, ctx)
            .await?;

        // Phase 11: Snapshots
        self.install_category(PackageCategory::Snapshots, driver, ctx)
            .await?;

        // Phase 12: Network
        self.install_category(PackageCategory::Network, driver, ctx)
            .await?;

        // Phase 13: Languages
        self.install_category(PackageCategory::Languages, driver, ctx)
            .await?;

        // Phase 14: Performance
        self.install_category(PackageCategory::Performance, driver, ctx)
            .await?;

        // Phase 15: Wayland
        self.install_category(PackageCategory::Wayland, driver, ctx)
            .await?;

        // Phase 16: Workflow
        self.install_category(PackageCategory::Workflow, driver, ctx)
            .await?;

        // Phase 19: Argon One (build deps)
        self.install_category(PackageCategory::ArgonOne, driver, ctx)
            .await?;

        ctx.record_action("🎉 Pi Overlord installation sequence completed!");

        Ok(())
    }
}

// ── Public API ──────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PiOverlord {
    packages: PiOverlordPackages,
}

impl Default for PiOverlord {
    fn default() -> Self {
        Self::new()
    }
}

impl PiOverlord {
    pub fn new() -> Self {
        Self {
            packages: PiOverlordPackages::new(),
        }
    }

    pub async fn install_category(
        &self,
        category: PackageCategory,
        driver: &dyn DistroDriver,
        ctx: &mut PhaseContext<'_>,
    ) -> Result<()> {
        self.packages.install_category(category, driver, ctx).await
    }

    pub async fn run_full_sequence(
        &self,
        driver: &dyn DistroDriver,
        ctx: &mut PhaseContext<'_>,
    ) -> Result<()> {
        self.packages.run_full_sequence(driver, ctx).await
    }

    pub fn get_package_mapping(&self, category: PackageCategory) -> Option<&PackageMapping> {
        self.packages.get_mapping(category)
    }
}

// ── Module Tests ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_mapping_creation() {
        let mapping = PackageMapping::new()
            .fedora(&["test-package"])
            .debian(&["test-package-deb"])
            .arch(&["test-package-arch"])
            .desc("Test package");

        assert_eq!(mapping.fedora, vec!["test-package"]);
        assert_eq!(mapping.debian, vec!["test-package-deb"]);
        assert_eq!(mapping.arch, vec!["test-package-arch"]);
        assert_eq!(mapping.description, "Test package");
    }

    #[test]
    fn test_pi_overlord_packages_initialization() {
        let packages = PiOverlordPackages::new();
        assert!(!packages.mappings.is_empty());
        assert!(packages.get_mapping(PackageCategory::CoreTools).is_some());
    }

    // Mock drivers for testing
    struct MockFedoraDriver;
    impl DistroDriver for MockFedoraDriver {
        fn name(&self) -> &'static str {
            "fedora"
        }
        fn description(&self) -> &'static str {
            "Mock Fedora Driver"
        }
        fn matches(&self, _info: &crate::PlatformInfo) -> bool {
            false
        }
        fn pkg_backend(&self) -> crate::PkgBackend {
            crate::PkgBackend::Dnf
        }
    }

    struct MockDebianDriver;
    impl DistroDriver for MockDebianDriver {
        fn name(&self) -> &'static str {
            "debian"
        }
        fn description(&self) -> &'static str {
            "Mock Debian Driver"
        }
        fn matches(&self, _info: &crate::PlatformInfo) -> bool {
            false
        }
        fn pkg_backend(&self) -> crate::PkgBackend {
            crate::PkgBackend::Apt
        }
    }

    struct MockArchDriver;
    impl DistroDriver for MockArchDriver {
        fn name(&self) -> &'static str {
            "arch"
        }
        fn description(&self) -> &'static str {
            "Mock Arch Driver"
        }
        fn matches(&self, _info: &crate::PlatformInfo) -> bool {
            false
        }
        fn pkg_backend(&self) -> crate::PkgBackend {
            crate::PkgBackend::Pacman
        }
    }

    #[test]
    fn test_cross_distro_mapping() {
        let packages = PiOverlordPackages::new();
        let mapping = packages.get_mapping(PackageCategory::CoreTools).unwrap();

        // Test that different distros get different packages
        let fedora_pkgs = mapping.get_for_distro(&MockFedoraDriver);
        let debian_pkgs = mapping.get_for_distro(&MockDebianDriver);
        let arch_pkgs = mapping.get_for_distro(&MockArchDriver);

        assert!(!fedora_pkgs.is_empty());
        assert!(!debian_pkgs.is_empty());
        assert!(!arch_pkgs.is_empty());

        // Fedora should have fedora-specific packages
        assert!(fedora_pkgs.contains(&"btop".to_string()));
        // Debian should have debian-specific packages
        assert!(debian_pkgs.contains(&"btop".to_string()));
        // Arch should have arch-specific packages
        assert!(arch_pkgs.contains(&"btop".to_string()));
    }
}
