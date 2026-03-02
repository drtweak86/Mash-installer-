//! System Profile — The machine's true pedigree.
//!
//! This module defines the data structures for auto-detection and system profiling.
//! It serves as the single source of truth for the machine's hardware, OS, and storage landscape.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

/// The complete pedigree of the machine we are inhabiting.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemProfile {
    pub platform: PlatformInfo,
    pub distro: DistroInfo,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub session: SessionInfo,
    pub storage: StorageInfo,
    pub timestamp: u64,
}

/// Hardware platform classification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum PlatformType {
    #[default]
    Unknown,
    RaspberryPi,
    GenericArm,
    PC,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformInfo {
    pub platform_type: PlatformType,
    pub model: String,
    pub board_revision: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DistroInfo {
    pub id: String,
    pub version: String,
    pub pretty_name: String,
    pub family: String,
    pub init_system: String,
    pub kernel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuInfo {
    pub model: String,
    pub arch: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub flags: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryInfo {
    pub ram_total_kb: u64,
    pub ram_avail_kb: u64,
    pub swap_total_kb: u64,
    pub zram_total_kb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionInfo {
    pub desktop_environment: String,
    pub window_manager: String,
    pub session_type: String, // x11, wayland, tty
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageInfo {
    pub devices: Vec<BlockDevice>,
    pub mounts: Vec<MountInfo>,
    pub btrfs_data: Option<BtrfsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockDevice {
    pub name: String,
    pub type_name: String, // disk, part, etc.
    pub size_bytes: u64,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub is_removable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MountInfo {
    pub device: String,
    pub destination: String,
    pub fstype: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BtrfsData {
    pub has_subvolumes: bool,
    pub root_is_btrfs: bool,
    pub subvolumes: Vec<String>,
}

impl SystemProfile {
    /// Create a new skeleton profile.
    pub fn new() -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            ..Default::default()
        }
    }

    /// Full auto-detection of the system.
    pub fn detect(system: &dyn crate::SystemOps) -> Result<Self> {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        sys.refresh_all();

        let mut profile = Self::new();
        profile.platform = PlatformInfo::detect(system)?;
        profile.distro = DistroInfo::detect()?;
        profile.cpu = CpuInfo::detect(&sys);
        profile.memory = MemoryInfo::detect(&sys, system);
        profile.session = SessionInfo::detect();
        profile.storage = StorageInfo::detect(system)?;

        Ok(profile)
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    pub fn save_to_config(&self) -> Result<PathBuf> {
        let config_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/root"))
            .join(".config")
            .join("mash-installer");

        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }

        let path = config_dir.join("system_profile.json");
        let json = self.to_json()?;
        std::fs::write(&path, json)?;

        Ok(path)
    }
}

impl StorageInfo {
    pub fn detect(system: &dyn crate::SystemOps) -> Result<Self> {
        let devices = detect_block_devices(system)?;
        let mounts = detect_mounts(system)?;
        let btrfs_data = detect_btrfs_data(system, &mounts);

        Ok(Self {
            devices,
            mounts,
            btrfs_data,
        })
    }
}

#[derive(Deserialize)]
struct LsblkOutput {
    blockdevices: Vec<LsblkDevice>,
}

#[derive(Deserialize)]
struct LsblkDevice {
    name: String,
    #[serde(rename = "type")]
    type_name: String,
    size: Option<serde_json::Value>,
    model: Option<String>,
    vendor: Option<String>,
    rm: Option<bool>,
    children: Option<Vec<LsblkDevice>>,
}

fn detect_block_devices(system: &dyn crate::SystemOps) -> Result<Vec<BlockDevice>> {
    let mut cmd = std::process::Command::new("lsblk");
    cmd.args([
        "--json",
        "-b",
        "-o",
        "NAME,TYPE,SIZE,FSTYPE,MOUNTPOINT,MODEL,VENDOR,RM",
    ]);

    let output = system.command_output(&mut cmd)?;
    let lsblk: LsblkOutput = serde_json::from_slice(&output.stdout)?;

    let mut devices = Vec::new();
    flatten_lsblk_devices(&lsblk.blockdevices, &mut devices);

    Ok(devices)
}

fn flatten_lsblk_devices(lsblk_devs: &[LsblkDevice], out: &mut Vec<BlockDevice>) {
    for dev in lsblk_devs {
        let size_bytes = match &dev.size {
            Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(0),
            Some(serde_json::Value::String(s)) => s.parse::<u64>().unwrap_or(0),
            _ => 0,
        };

        out.push(BlockDevice {
            name: dev.name.clone(),
            type_name: dev.type_name.clone(),
            size_bytes,
            model: dev.model.clone(),
            vendor: dev.vendor.clone(),
            is_removable: dev.rm.unwrap_or(false),
        });

        if let Some(children) = &dev.children {
            flatten_lsblk_devices(children, out);
        }
    }
}

fn detect_mounts(system: &dyn crate::SystemOps) -> Result<Vec<MountInfo>> {
    let content = system.read_to_string(Path::new("/proc/mounts"))?;
    let mut mounts = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            mounts.push(MountInfo {
                device: parts[0].to_string(),
                destination: parts[1].to_string(),
                fstype: parts[2].to_string(),
                options: parts[3].split(',').map(|s| s.to_string()).collect(),
            });
        }
    }

    Ok(mounts)
}

fn detect_btrfs_data(system: &dyn crate::SystemOps, mounts: &[MountInfo]) -> Option<BtrfsData> {
    let root_mount = mounts.iter().find(|m| m.destination == "/")?;
    let root_is_btrfs = root_mount.fstype == "btrfs";

    if !root_is_btrfs && !mounts.iter().any(|m| m.fstype == "btrfs") {
        return None;
    }

    let mut subvolumes = Vec::new();
    let mut cmd = std::process::Command::new("sudo");
    cmd.args(["btrfs", "subvolume", "list", "/"]);

    if let Ok(output) = system.command_output(&mut cmd) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Some(path_idx) = line.find("path ") {
                let path = &line[path_idx + 5..];
                subvolumes.push(path.to_string());
            }
        }
    }

    Some(BtrfsData {
        has_subvolumes: !subvolumes.is_empty(),
        root_is_btrfs,
        subvolumes,
    })
}

impl PlatformInfo {
    pub fn detect(system: &dyn crate::SystemOps) -> Result<Self> {
        let model_path = Path::new("/proc/device-tree/model");
        if model_path.exists() {
            let model = system.read_to_string(model_path).unwrap_or_default();
            let model = model.trim_end_matches('\0').trim().to_string();
            if model.to_lowercase().contains("raspberry pi") {
                return Ok(Self {
                    platform_type: PlatformType::RaspberryPi,
                    model,
                    board_revision: detect_board_revision(system),
                });
            }
            if !model.is_empty() {
                return Ok(Self {
                    platform_type: PlatformType::GenericArm,
                    model,
                    board_revision: None,
                });
            }
        }

        // Fallback to PC for x86_64 or if no device-tree model found
        Ok(Self {
            platform_type: PlatformType::PC,
            model: "Standard PC".to_string(),
            board_revision: None,
        })
    }
}

fn detect_board_revision(system: &dyn crate::SystemOps) -> Option<String> {
    if let Ok(cpuinfo) = system.read_to_string(Path::new("/proc/cpuinfo")) {
        for line in cpuinfo.lines() {
            if line.starts_with("Revision") {
                if let Some((_k, v)) = line.split_once(':') {
                    return Some(v.trim().to_string());
                }
            }
        }
    }
    None
}

impl DistroInfo {
    pub fn detect() -> Result<Self> {
        let os_release = std::fs::read_to_string("/etc/os-release").unwrap_or_default();
        let id = parse_os_field(&os_release, "ID").unwrap_or_else(|| "unknown".into());
        let version = parse_os_field(&os_release, "VERSION_ID").unwrap_or_else(|| "0".into());
        let pretty_name =
            parse_os_field(&os_release, "PRETTY_NAME").unwrap_or_else(|| "Unknown Linux".into());
        let id_like = parse_os_field(&os_release, "ID_LIKE").unwrap_or_default();

        let family = if id == "fedora" || id_like.contains("fedora") {
            "fedora".into()
        } else if id == "arch" || id_like.contains("arch") {
            "arch".into()
        } else if id == "debian"
            || id == "ubuntu"
            || id_like.contains("debian")
            || id_like.contains("ubuntu")
        {
            "debian".into()
        } else {
            "unknown".into()
        };

        let init_system = if Path::new("/run/systemd/system").exists() {
            "systemd".into()
        } else {
            "unknown".into()
        };

        let kernel = std::fs::read_to_string("/proc/version")
            .unwrap_or_default()
            .split_whitespace()
            .nth(2)
            .unwrap_or("unknown")
            .to_string();

        Ok(Self {
            id,
            version,
            pretty_name,
            family,
            init_system,
            kernel,
        })
    }
}

fn parse_os_field(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix(&format!("{key}=")) {
            return Some(rest.trim_matches('"').to_string());
        }
    }
    None
}

impl CpuInfo {
    pub fn detect(sys: &System) -> Self {
        let cpu = sys.cpus().first();
        let model = cpu.map(|c| c.brand().to_string()).unwrap_or_default();
        let arch = std::env::consts::ARCH.to_string();

        let mut flags = HashSet::new();
        // Extract flags from /proc/cpuinfo on Linux
        if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in cpuinfo.lines() {
                if line.starts_with("flags") || line.starts_with("Features") {
                    if let Some((_k, v)) = line.split_once(':') {
                        for flag in v.split_whitespace() {
                            flags.insert(flag.to_string());
                        }
                    }
                }
            }
        }

        Self {
            model,
            arch,
            physical_cores: sys.physical_core_count().unwrap_or(0),
            logical_cores: sys.cpus().len(),
            flags,
        }
    }
}

impl MemoryInfo {
    pub fn detect(sys: &System, system: &dyn crate::SystemOps) -> Self {
        let ram_total_kb = sys.total_memory() / 1024;
        let ram_avail_kb = sys.available_memory() / 1024;
        let swap_total_kb = sys.total_swap() / 1024;

        // Detect ZRAM
        let mut zram_total_kb = 0;
        if let Ok(swaps) = system.read_to_string(Path::new("/proc/swaps")) {
            for line in swaps.lines() {
                if line.contains("zram") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(size_kb) = parts.get(2).and_then(|s| s.parse::<u64>().ok()) {
                        zram_total_kb += size_kb;
                    }
                }
            }
        }

        Self {
            ram_total_kb,
            ram_avail_kb,
            swap_total_kb,
            zram_total_kb,
        }
    }
}

impl SessionInfo {
    pub fn detect() -> Self {
        let desktop_environment =
            std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "unknown".into());
        let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "tty".into());

        // Inference for WM
        let window_manager = if desktop_environment == "unknown" {
            // Check for common WMs in processes or env
            if std::env::var("I3SOCK").is_ok() {
                "i3".into()
            } else if std::env::var("SWAYSOCK").is_ok() {
                "sway".into()
            } else {
                "unknown".into()
            }
        } else {
            desktop_environment.clone()
        };

        Self {
            desktop_environment,
            window_manager,
            session_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::REAL_SYSTEM;

    #[test]
    fn test_profile_serialization() {
        let mut profile = SystemProfile::new();
        profile.platform.platform_type = PlatformType::RaspberryPi;
        profile.platform.model = "Raspberry Pi 4 Model B Rev 1.5".to_string();

        let json = profile.to_json().expect("Failed to serialize");
        assert!(json.contains("RaspberryPi"));

        let deserialized: SystemProfile =
            SystemProfile::from_json(&json).expect("Failed to deserialize");
        assert_eq!(
            deserialized.platform.platform_type,
            PlatformType::RaspberryPi
        );
        assert_eq!(
            deserialized.platform.model,
            "Raspberry Pi 4 Model B Rev 1.5"
        );
    }

    #[test]
    fn test_system_detection() {
        let profile = SystemProfile::detect(&REAL_SYSTEM).expect("Detection failed");

        // Basic sanity checks
        assert!(profile.timestamp > 0);
        assert!(!profile.distro.id.is_empty());
        assert!(!profile.cpu.arch.is_empty());
        assert!(profile.cpu.logical_cores > 0);
        assert!(profile.memory.ram_total_kb > 0);

        println!("Detected System Profile:\n{}", profile.to_json().unwrap());
    }

    #[test]
    fn test_distro_detection() {
        let distro = DistroInfo::detect().expect("Distro detection failed");
        assert!(!distro.id.is_empty());
        assert!(!distro.pretty_name.is_empty());
        assert!(!distro.kernel.is_empty());
    }
}
