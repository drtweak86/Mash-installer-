//! Advice Engine — The Bard's Wisdom.
//!
//! This module provides an intelligent advice engine that analyzes a `SystemProfile`
//! and returns actionable wisdom, performance hints, and critical warnings.

use crate::context::UserOptionsContext;
use crate::SystemProfile;
use serde::{Deserialize, Serialize};

/// Severity of the advice.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Helpful information or performance hint.
    Info,
    /// Non-critical warning or suboptimal configuration.
    Warning,
    /// Critical issue that may cause failure or severe performance degradation.
    Critical,
}

/// A single piece of wisdom from the Bard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdviceEntry {
    pub name: &'static str,
    pub level: Severity,
    pub message: String,
    pub advice: String,
}

impl AdviceEntry {
    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// The foundational trait for system analysis rules.
pub trait Rule: Send + Sync {
    /// Unique name for the rule.
    fn name(&self) -> &'static str;

    /// Check the profile and options and return advice if the rule is triggered.
    fn check(&self, profile: &SystemProfile, options: &UserOptionsContext) -> Option<AdviceEntry>;
}

/// The engine that orchestrates the scrying of wisdom.
pub struct AdviceEngine {
    rules: Vec<Box<dyn Rule>>,
}

impl AdviceEngine {
    /// Create a new engine with a set of rules.
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Self { rules }
    }

    /// Run all rules against the profile and collect the whispers of wisdom.
    pub fn run(&self, profile: &SystemProfile, options: &UserOptionsContext) -> Vec<AdviceEntry> {
        self.rules
            .iter()
            .filter_map(|rule| {
                rule.check(profile, options).map(|mut entry| {
                    entry.name = rule.name();
                    entry
                })
            })
            .collect()
    }
}

impl Default for AdviceEngine {
    /// Create a default engine with standard rules (Phases 2-4).
    fn default() -> Self {
        Self::new(vec![
            Box::new(LowRamRule),
            Box::new(NoSwapRule),
            Box::new(PiWaylandWarning),
            Box::new(PiSdCardWarning),
            Box::new(LaptopDetectedRule),
            Box::new(HighCoreCountOptimization),
            Box::new(BtrfsSnapshotRule),
            Box::new(BtrfsCompressionRule),
            Box::new(SmallRootLargeDataRule),
            Box::new(SdCardWriteWarning),
            Box::new(NoJournalRule),
            Box::new(NodeArm64StabilityRule),
            Box::new(Armv7lCompatibilityWarning),
            Box::new(BrcmfmacFirmwareHint),
            Box::new(GpuDriverWarning),
            Box::new(WaylandNvidiaWarning),
            Box::new(ChezmoiHeuristicRule),
        ])
    }
}

// ... existing rules ...

// ── Software Stability Rules ────────────────────────────────────────────────

struct NodeArm64StabilityRule;
impl Rule for NodeArm64StabilityRule {
    fn name(&self) -> &'static str {
        "node_arm64_curse"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        let distro_ver: u32 = profile
            .distro
            .version
            .split('.')
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let is_node_22 = profile
            .software
            .nodejs_version
            .as_deref()
            .map(|v| v.contains("v22"))
            .unwrap_or(false);

        if profile.cpu.arch == "aarch64" && distro_ver >= 43 && is_node_22 {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Critical,
                message: "Node.js v22 identified on modern ARM64 environment.".into(),
                advice: "Known stability issues detected. Recommend downgrading to Node.js v20 (LTS) for a stable forge experience.".into(),
            })
        } else {
            None
        }
    }
}

struct Armv7lCompatibilityWarning;
impl Rule for Armv7lCompatibilityWarning {
    fn name(&self) -> &'static str {
        "armv7l_limits"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        if profile.cpu.arch == "armv7l" {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Warning,
                message: "32-bit ARM (armv7l) environment detected.".into(),
                advice: "Many modern artifacts (Claude, Gemini, modern Node) have limited support for 32-bit. Consider a 64-bit OS if your hardware supports it.".into(),
            })
        } else {
            None
        }
    }
}

struct BrcmfmacFirmwareHint;
impl Rule for BrcmfmacFirmwareHint {
    fn name(&self) -> &'static str {
        "wifi_firmware"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        // This is a rough proxy: if it's a Pi but no wlan0
        use crate::profile::PlatformType;
        if profile.platform.platform_type == PlatformType::RaspberryPi
            && !profile.network.interfaces.iter().any(|i| i == "wlan0")
        {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Warning,
                message: "Wireless interface (wlan0) is missing on Raspberry Pi hardware.".into(),
                advice: "Ensure 'raspberrypi-firmware' is installed and your kernel is up to date to enable the Broadcom radio.".into(),
            })
        } else {
            None
        }
    }
}

struct GpuDriverWarning;
impl Rule for GpuDriverWarning {
    fn name(&self) -> &'static str {
        "gpu_drivers"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        if profile.gpu.driver == "Unknown"
            && profile.platform.platform_type == crate::profile::PlatformType::PC
        {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Info,
                message: "Generic or unknown GPU driver in use.".into(),
                advice: "For the best retro-neon experience, ensure your vendor-specific drivers (Mesa/NVIDIA/Intel) are correctly etched.".into(),
            })
        } else {
            None
        }
    }
}

struct WaylandNvidiaWarning;
impl Rule for WaylandNvidiaWarning {
    fn name(&self) -> &'static str {
        "wayland_nvidia"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        if profile.session.session_type == "wayland" && profile.gpu.driver == "nvidia" {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Warning,
                message: "Wayland session detected with NVIDIA drivers.".into(),
                advice: "Flickering or stability issues may occur. Recommend using X11 or ensuring 'nvidia-drm.modeset=1' is in your kernel runes.".into(),
            })
        } else {
            None
        }
    }
}

// ... existing hardware rules ...

// ── Storage Rules ───────────────────────────────────────────────────────────

struct BtrfsSnapshotRule;
impl Rule for BtrfsSnapshotRule {
    fn name(&self) -> &'static str {
        "btrfs_snapshots"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        let is_btrfs = profile
            .storage
            .mounts
            .iter()
            .any(|m| m.destination == "/" && m.fstype == "btrfs");

        if is_btrfs {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Info,
                message: "Btrfs root filesystem detected.".into(),
                advice: "Recommend installing 'btrfs-assistant' or 'snapper' to manage snapshots and rollback runes.".into(),
            })
        } else {
            None
        }
    }
}

struct BtrfsCompressionRule;
impl Rule for BtrfsCompressionRule {
    fn name(&self) -> &'static str {
        "btrfs_compression"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        let root_mount = profile.storage.mounts.iter().find(|m| m.destination == "/");

        if let Some(m) = root_mount {
            if m.fstype == "btrfs" && !m.options.iter().any(|o| o.contains("compress")) {
                return Some(AdviceEntry {
                    name: "todo",
                    level: Severity::Info,
                    message: "Btrfs detected without active compression.".into(),
                    advice: "Enable 'zstd:3' in /etc/fstab to significantly extend flash storage life and save space.".into(),
                });
            }
        }
        None
    }
}

struct SmallRootLargeDataRule;
impl Rule for SmallRootLargeDataRule {
    fn name(&self) -> &'static str {
        "workspace_relocation"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        let root_size = profile
            .storage
            .devices
            .iter()
            .find(|d| d.name.contains("root") || d.name.contains("nvme0n1p2")) // Rough proxy
            .map(|d| d.size_bytes)
            .unwrap_or(u64::MAX);

        let data_mount = profile
            .storage
            .mounts
            .iter()
            .find(|m| m.destination == "/data");

        if root_size < 30 * 1024 * 1024 * 1024 && data_mount.is_some() {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Warning,
                message: "Small root partition detected with separate data hoard.".into(),
                advice: "Consider relocating your GitHub workspace and Docker data root to /data to prevent root exhaustion.".into(),
            })
        } else {
            None
        }
    }
}

struct SdCardWriteWarning;
impl Rule for SdCardWriteWarning {
    fn name(&self) -> &'static str {
        "sd_card_writes"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        let is_sd = profile
            .storage
            .mounts
            .iter()
            .find(|m| m.destination == "/")
            .map(|m| m.device.contains("mmcblk0"))
            .unwrap_or(false);

        if is_sd {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Warning,
                message: "Heavy write activity detected on an SD card.".into(),
                advice: "Frequent builds will wear out SD flash. Mount a USB SSD for your workspace to protect your data.".into(),
            })
        } else {
            None
        }
    }
}

struct NoJournalRule;
impl Rule for NoJournalRule {
    fn name(&self) -> &'static str {
        "ext4_optimizations"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        let root_mount = profile.storage.mounts.iter().find(|m| m.destination == "/");
        if let Some(m) = root_mount {
            if m.fstype == "ext4" && !m.options.iter().any(|o| o == "noatime") {
                return Some(AdviceEntry {
                    name: "todo",
                    level: Severity::Info,
                    message: "Ext4 root detected without 'noatime' optimization.".into(),
                    advice: "Add 'noatime' to your mount options in /etc/fstab to reduce unnecessary writes to flash storage.".into(),
                });
            }
        }
        None
    }
}

// ── Hardware Rules ──────────────────────────────────────────────────────────

struct LowRamRule;
impl Rule for LowRamRule {
    fn name(&self) -> &'static str {
        "low_ram"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        let ram_gb = profile.memory.ram_total_kb as f32 / (1024.0 * 1024.0);
        if ram_gb < 7.5 {
            // A bit less than 8GB to account for reserved mem
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Warning,
                message: format!("Limited system memory detected ({:.1} GB).", ram_gb),
                advice: "Consider using the 'Minimal' profile to ensure stable performance.".into(),
            })
        } else {
            None
        }
    }
}

struct NoSwapRule;
impl Rule for NoSwapRule {
    fn name(&self) -> &'static str {
        "no_swap"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        let ram_gb = profile.memory.ram_total_kb as f32 / (1024.0 * 1024.0);
        if ram_gb < 8.0 && profile.memory.swap_total_kb == 0 && profile.memory.zram_total_kb == 0 {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Critical,
                message: "No swap or ZRAM detected on a limited memory system.".into(),
                advice: "Strongly recommend enabling ZRAM via the 'Pi 4B Tuning' module or manually to prevent OOM panics.".into(),
            })
        } else {
            None
        }
    }
}

struct PiWaylandWarning;
impl Rule for PiWaylandWarning {
    fn name(&self) -> &'static str {
        "pi_wayland"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        use crate::profile::PlatformType;
        if profile.platform.platform_type == PlatformType::RaspberryPi
            && profile.session.session_type == "wayland"
        {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Warning,
                message: "Wayland session detected on Raspberry Pi.".into(),
                advice: "While supported, X11 often provides better performance and stability for heavy desktop use on Pi 4 hardware.".into(),
            })
        } else {
            None
        }
    }
}

struct PiSdCardWarning;
impl Rule for PiSdCardWarning {
    fn name(&self) -> &'static str {
        "pi_sd_card"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        let is_sd = profile
            .storage
            .mounts
            .iter()
            .find(|m| m.destination == "/")
            .map(|m| m.device.contains("mmcblk0"))
            .unwrap_or(false);

        if is_sd {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Info,
                message: "Operating system is running from an SD card.".into(),
                advice: "Compilation of heavy tools (Rust/Node) will be significantly throttled by I/O. Consider moving to a USB 3.0 SSD for serious smithing.".into(),
            })
        } else {
            None
        }
    }
}

struct LaptopDetectedRule;
impl Rule for LaptopDetectedRule {
    fn name(&self) -> &'static str {
        "laptop_detected"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        if profile.platform.is_laptop {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Info,
                message: "Portable station (laptop) identified.".into(),
                advice: "Recommend installing 'auto-cpufreq' or 'TLP' to optimize power draw and thermals while mobile.".into(),
            })
        } else {
            None
        }
    }
}

struct HighCoreCountOptimization;
impl Rule for HighCoreCountOptimization {
    fn name(&self) -> &'static str {
        "high_core_count"
    }
    fn check(&self, profile: &SystemProfile, _options: &UserOptionsContext) -> Option<AdviceEntry> {
        if profile.cpu.physical_cores > 16 {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Info,
                message: "High core count CPU detected.".into(),
                advice: "Ensure 'sccache' is configured to maximize your parallel smithing power and reduce build times.".into(),
            })
        } else {
            None
        }
    }
}

struct ChezmoiHeuristicRule;
impl Rule for ChezmoiHeuristicRule {
    fn name(&self) -> &'static str {
        "chezmoi_recommendation"
    }
    fn check(&self, _profile: &SystemProfile, options: &UserOptionsContext) -> Option<AdviceEntry> {
        use crate::options::ProfileLevel;
        if options.profile >= ProfileLevel::Dev && !options.chezmoi.enabled {
            Some(AdviceEntry {
                name: "todo",
                level: Severity::Info,
                message: "Professional Developer profile active without dotfile restoration.".into(),
                advice: "Recommend enabling 'Chezmoi' to automatically restore your personal environment and tool configurations.".into(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::UserOptionsContext;
    use crate::options::ProfileLevel;
    use crate::profile::SystemProfile;
    use std::path::PathBuf;

    fn default_options() -> UserOptionsContext {
        UserOptionsContext {
            profile: ProfileLevel::Dev,
            staging_dir: PathBuf::from("/tmp"),
            dry_run: true,
            interactive: false,
            enable_argon: false,
            enable_p10k: false,
            docker_data_root: false,
            software_plan: Default::default(),
            system_profile: None,
            environment: crate::options::EnvironmentTag::Home,
            chezmoi: crate::model::options::ChezmoiOptions::default(),
        }
    }

    #[test]
    fn test_hardware_rules() {
        let engine = AdviceEngine::default();
        let mut profile = SystemProfile::default();
        let mut options = default_options();

        // Test: Low RAM
        profile.memory.ram_total_kb = 4 * 1024 * 1024;
        let advice = engine.run(&profile, &options);
        assert!(advice
            .iter()
            .any(|a| a.message.contains("Limited system memory")));

        // Test: No Swap on Low RAM
        profile.memory.swap_total_kb = 0;
        profile.memory.zram_total_kb = 0;
        let advice = engine.run(&profile, &options);
        assert!(advice
            .iter()
            .any(|a| a.level == Severity::Critical && a.message.contains("No swap")));

        // Test: Pi Wayland
        profile.platform.platform_type = crate::profile::PlatformType::RaspberryPi;
        profile.session.session_type = "wayland".into();
        let advice = engine.run(&profile, &options);
        assert!(advice.iter().any(|a| a
            .message
            .contains("Wayland session detected on Raspberry Pi")));

        // Test: Laptop
        profile.platform.is_laptop = true;
        let advice = engine.run(&profile, &options);
        assert!(advice
            .iter()
            .any(|a| a.message.contains("Portable station")));

        // Test: Btrfs
        profile.storage.mounts.push(crate::profile::MountInfo {
            device: "/dev/sda2".into(),
            destination: "/".into(),
            fstype: "btrfs".into(),
            options: vec!["rw".into(), "relatime".into()],
        });
        let advice = engine.run(&profile, &options);
        assert!(advice.iter().any(|a| a.message.contains("Btrfs root")));
        assert!(advice.iter().any(|a| a
            .message
            .contains("Btrfs detected without active compression")));

        // Test: Workspace Relocation
        profile.storage.devices.push(crate::profile::BlockDevice {
            name: "nvme0n1p2".into(),
            type_name: "part".into(),
            size_bytes: 20 * 1024 * 1024 * 1024, // 20GB
            model: None,
            vendor: None,
            is_removable: false,
        });
        profile.storage.mounts.push(crate::profile::MountInfo {
            device: "/dev/sda3".into(),
            destination: "/data".into(),
            fstype: "ext4".into(),
            options: vec!["rw".into()],
        });
        let advice = engine.run(&profile, &options);
        assert!(advice.iter().any(|a| a.message.contains("Small root")));

        // Test: Node ARM64 Curse
        profile.cpu.arch = "aarch64".into();
        profile.distro.version = "43".into();
        profile.software.nodejs_version = Some("v22.0.0".into());
        let advice = engine.run(&profile, &options);
        assert!(advice.iter().any(|a| a.message.contains("Node.js v22")));

        // Test: NVIDIA Wayland
        profile.session.session_type = "wayland".into();
        profile.gpu.driver = "nvidia".into();
        let advice = engine.run(&profile, &options);
        assert!(advice.iter().any(|a| a.message.contains("NVIDIA drivers")));
    }

    #[test]
    fn test_chezmoi_rule() {
        let engine = AdviceEngine::default();
        let profile = SystemProfile::default();
        let mut options = default_options();

        // 1. Dev profile, Chezmoi disabled -> Advice should trigger
        options.profile = ProfileLevel::Dev;
        options.chezmoi.enabled = false;
        let advice = engine.run(&profile, &options);
        assert!(advice.iter().any(|a| a.name() == "chezmoi_recommendation"));

        // 2. Dev profile, Chezmoi enabled -> Advice should NOT trigger
        options.chezmoi.enabled = true;
        let advice = engine.run(&profile, &options);
        assert!(!advice.iter().any(|a| a.name() == "chezmoi_recommendation"));

        // 3. Minimal profile, Chezmoi disabled -> Advice should NOT trigger
        options.profile = ProfileLevel::Minimal;
        options.chezmoi.enabled = false;
        let advice = engine.run(&profile, &options);
        assert!(!advice.iter().any(|a| a.name() == "chezmoi_recommendation"));
    }
}
