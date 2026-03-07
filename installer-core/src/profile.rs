//! System Profile — The machine's true pedigree.
//!
//! This module defines the data structures for auto-detection and system profiling.
//! It serves as the single source of truth for the machine's hardware, OS, and storage landscape.

use anyhow::Result;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;

pub use crate::model::profile::*;

use crate::system::proc::{read_cpu_model, MemStats};
use crate::system::system_ops::SystemOps;

pub trait SystemProfileExt {
    fn detect(system: &dyn SystemOps) -> Result<Self>
    where
        Self: Sized;
}

pub trait PlatformInfoExt {
    fn detect(system: &dyn SystemOps) -> Result<Self>
    where
        Self: Sized;
}

pub trait DistroInfoExt {
    fn detect() -> Result<Self>
    where
        Self: Sized;
}

pub trait CpuInfoExt {
    fn detect() -> Self;
}

pub trait MemoryInfoExt {
    fn detect(system: &dyn SystemOps) -> Self;
}

pub trait GpuInfoExt {
    fn detect(system: &dyn SystemOps) -> Result<Self>
    where
        Self: Sized;
}

pub trait NetworkInfoExt {
    fn detect(system: &dyn SystemOps) -> Result<Self>
    where
        Self: Sized;
}

pub trait SoftwareInfoExt {
    fn detect(system: &dyn SystemOps) -> Result<Self>
    where
        Self: Sized;
}

pub trait SessionInfoExt {
    fn detect() -> Self;
}

pub trait StorageInfoExt {
    fn detect(system: &dyn SystemOps) -> Result<Self>
    where
        Self: Sized;
}

impl SystemProfileExt for SystemProfile {
    /// Full auto-detection of the system.
    fn detect(system: &dyn SystemOps) -> Result<Self> {
        let mut profile = Self::new();

        profile.platform = PlatformInfo::detect(system).unwrap_or_else(|e| {
            tracing::error!("Platform detection failed: {}", e);
            PlatformInfo::default()
        });

        profile.distro = DistroInfo::detect().unwrap_or_else(|e| {
            tracing::error!("Distro detection failed: {}", e);
            DistroInfo::default()
        });

        profile.cpu = CpuInfo::detect();
        profile.memory = MemoryInfo::detect(system);

        profile.gpu = GpuInfo::detect(system).unwrap_or_else(|e| {
            tracing::error!("GPU detection failed: {}", e);
            GpuInfo::default()
        });

        profile.network = NetworkInfo::detect(system).unwrap_or_else(|e| {
            tracing::error!("Network detection failed: {}", e);
            NetworkInfo::default()
        });

        profile.software = SoftwareInfo::detect(system).unwrap_or_else(|e| {
            tracing::error!("Software detection failed: {}", e);
            SoftwareInfo::default()
        });

        profile.session = SessionInfo::detect();

        profile.storage = StorageInfo::detect(system).unwrap_or_else(|e| {
            tracing::error!("Storage detection failed: {}", e);
            StorageInfo::default()
        });

        Ok(profile)
    }
}

impl StorageInfoExt for StorageInfo {
    fn detect(system: &dyn SystemOps) -> Result<Self> {
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

fn detect_block_devices(system: &dyn SystemOps) -> Result<Vec<BlockDevice>> {
    let mut cmd = std::process::Command::new("lsblk");
    cmd.args([
        "--json",
        "-b",
        "-o",
        "NAME,TYPE,SIZE,FSTYPE,MOUNTPOINT,MODEL,VENDOR,RM",
    ]);

    let output = match system.command_output(&mut cmd) {
        Ok(out) => out,
        Err(err) => {
            tracing::warn!("Failed to detect block devices: {}", err);
            return Ok(Vec::new());
        }
    };

    let lsblk: LsblkOutput = match serde_json::from_slice(&output.stdout) {
        Ok(data) => data,
        Err(err) => {
            tracing::warn!("Failed to parse lsblk output: {}", err);
            return Ok(Vec::new());
        }
    };

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

fn detect_mounts(system: &dyn SystemOps) -> Result<Vec<MountInfo>> {
    let content = match system.read_to_string(Path::new("/proc/mounts")) {
        Ok(c) => c,
        Err(err) => {
            tracing::warn!("Failed to read /proc/mounts: {}", err);
            return Ok(Vec::new());
        }
    };
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

fn detect_btrfs_data(system: &dyn SystemOps, mounts: &[MountInfo]) -> Option<BtrfsData> {
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

impl PlatformInfoExt for PlatformInfo {
    fn detect(system: &dyn SystemOps) -> Result<Self> {
        let model_path = Path::new("/proc/device-tree/model");
        if model_path.exists() {
            let model = system.read_to_string(model_path).unwrap_or_default();
            let model = model.trim_end_matches('\0').trim().to_string();
            if model.to_lowercase().contains("raspberry pi") {
                return Ok(Self {
                    platform_type: PlatformType::RaspberryPi,
                    model,
                    board_revision: detect_board_revision(system),
                    is_laptop: false, // Pis are not laptops (usually)
                });
            }
            if !model.is_empty() {
                return Ok(Self {
                    platform_type: PlatformType::GenericArm,
                    model,
                    board_revision: None,
                    is_laptop: false,
                });
            }
        }

        // Fallback to PC for x86_64 or if no device-tree model found
        Ok(Self {
            platform_type: PlatformType::PC,
            model: "Standard PC".to_string(),
            board_revision: None,
            is_laptop: detect_is_laptop(system),
        })
    }
}

fn detect_is_laptop(_system: &dyn SystemOps) -> bool {
    // Check for battery presence as a proxy for laptop
    let power_supply = Path::new("/sys/class/power_supply");
    if let Ok(entries) = std::fs::read_dir(power_supply) {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                if name.starts_with("BAT") {
                    return true;
                }
            }
        }
    }
    false
}

fn detect_board_revision(system: &dyn SystemOps) -> Option<String> {
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

impl DistroInfoExt for DistroInfo {
    fn detect() -> Result<Self> {
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

impl CpuInfoExt for CpuInfo {
    fn detect() -> Self {
        let model = read_cpu_model();
        let arch = std::env::consts::ARCH.to_string();

        let mut flags = HashSet::new();
        let mut logical_cores = 0;
        let mut physical_cores = 0;

        // Extract flags and core info from /proc/cpuinfo on Linux
        if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in cpuinfo.lines() {
                if line.starts_with("flags") || line.starts_with("Features") {
                    if let Some((_k, v)) = line.split_once(':') {
                        for flag in v.split_whitespace() {
                            flags.insert(flag.to_string());
                        }
                    }
                }
                if line.starts_with("processor") {
                    logical_cores += 1;
                }
                if line.starts_with("cpu cores") {
                    if let Some((_, v)) = line.split_once(':') {
                        if let Ok(count) = v.trim().parse::<u32>() {
                            physical_cores = count;
                        }
                    }
                }
            }
        }

        // Fallback for cores if /proc/cpuinfo parsing failed
        if logical_cores == 0 {
            logical_cores = std::thread::available_parallelism()
                .map(|n| n.get() as u32)
                .unwrap_or(1);
        }
        if physical_cores == 0 {
            physical_cores = logical_cores;
        }

        Self {
            model,
            arch,
            physical_cores: physical_cores as usize,
            logical_cores: logical_cores as usize,
            flags,
        }
    }
}

impl GpuInfoExt for GpuInfo {
    fn detect(system: &dyn SystemOps) -> Result<Self> {
        // Simple shim for GPU detection
        let mut model = "Unknown".to_string();
        let mut driver = "Unknown".to_string();

        // Attempt to scry via lspci if available
        let mut cmd = std::process::Command::new("lspci");
        if let Ok(output) = system.command_output(&mut cmd) {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("VGA") || line.contains("Display") || line.contains("3D") {
                    model = line.to_string();
                    break;
                }
            }
        }

        // Try to identify driver from /proc/modules or similar
        if let Ok(modules) = system.read_to_string(Path::new("/proc/modules")) {
            if modules.contains("nvidia") {
                driver = "nvidia".to_string();
            } else if modules.contains("amdgpu") {
                driver = "amdgpu".to_string();
            } else if modules.contains("i915") {
                driver = "intel".to_string();
            } else if modules.contains("vc4") || modules.contains("v3d") {
                driver = "mesa (broadcom)".to_string();
            }
        }

        Ok(Self { model, driver })
    }
}

impl NetworkInfoExt for NetworkInfo {
    fn detect(system: &dyn SystemOps) -> Result<Self> {
        let mut interfaces = Vec::new();
        if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    interfaces.push(name);
                }
            }
        }

        // Detect online status and latency via ping
        let mut online = false;
        let mut latency_ms = None;

        let mut cmd = std::process::Command::new("ping");
        cmd.args(["-c", "1", "-W", "2", "8.8.8.8"]);
        if let Ok(output) = system.command_output(&mut cmd) {
            if output.status.success() {
                online = true;
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(time_idx) = stdout.find("time=") {
                    let rest = &stdout[time_idx + 5..];
                    if let Some(ms_idx) = rest.find(" ms") {
                        if let Ok(ms) = rest[..ms_idx].parse::<f32>() {
                            latency_ms = Some(ms);
                        }
                    }
                }
            }
        }

        Ok(Self {
            interfaces,
            online,
            latency_ms,
        })
    }
}

impl SoftwareInfoExt for SoftwareInfo {
    fn detect(system: &dyn SystemOps) -> Result<Self> {
        let mut nodejs_version = None;

        let mut cmd = std::process::Command::new("node");
        cmd.arg("--version");
        if let Ok(output) = system.command_output(&mut cmd) {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !version.is_empty() {
                nodejs_version = Some(version);
            }
        }

        Ok(Self { nodejs_version })
    }
}

impl MemoryInfoExt for MemoryInfo {
    fn detect(system: &dyn SystemOps) -> Self {
        let mem = MemStats::read().unwrap_or_default();
        let ram_total_kb = mem.total_kb;
        let ram_avail_kb = mem.available_kb;
        let swap_total_kb = mem.swap_total_kb;

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

impl SessionInfoExt for SessionInfo {
    fn detect() -> Self {
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
    use crate::system::system_ops::REAL_SYSTEM;

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
