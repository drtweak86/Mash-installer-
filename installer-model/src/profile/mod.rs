use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

/// The complete pedigree of the machine we are inhabiting.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemProfile {
    pub platform: PlatformInfo,
    pub distro: DistroInfo,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub gpu: GpuInfo,
    pub network: NetworkInfo,
    pub software: SoftwareInfo,
    pub session: SessionInfo,
    pub storage: StorageInfo,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SoftwareInfo {
    pub nodejs_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GpuInfo {
    pub model: String,
    pub driver: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkInfo {
    pub interfaces: Vec<String>,
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
    pub is_laptop: bool,
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

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    pub fn save_to_config(&self) -> anyhow::Result<PathBuf> {
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
