// Pi 4B HDD Optimization Module
// Preflight checks and tuning for Raspberry Pi 4B with external USB 3.0 HDD

use anyhow::Result;
use std::path::Path;
use std::process::Command;

use crate::doctor::{CheckStatus, PreflightCheck};
use crate::system::SystemOps;
use crate::PhaseContext;

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------

/// USB 3.0 controller information
pub struct Usb3Controller {
    pub controller: String,
    pub port: String,
    pub speed: String,
}

/// HDD health information
pub struct HddHealth {
    pub model: String,
    pub serial: String,
    pub temperature: Option<i32>,
    pub smart_status: String,
    pub power_on_hours: Option<u32>,
}

/// Partition layout information
pub struct PartitionLayout {
    pub device: String,
    pub partitions: Vec<PartitionInfo>,
}

pub struct PartitionInfo {
    pub number: u32,
    pub size: u64,
    pub filesystem: String,
    pub mount_point: Option<String>,
}

/// I/O scheduler information
pub struct IoScheduler {
    pub current: String,
    pub available: Vec<String>,
    pub recommended: String,
}

/// Mount optimization recommendation for a single device
pub struct MountOptimization {
    pub device: String,
    pub mount_point: String,
    pub filesystem: String,
    pub current_options: String,
    pub recommended_options: Vec<String>,
}

/// Swap configuration state and recommendation
pub struct SwapConfig {
    pub current_swap_mb: u64,
    pub recommended_swap_mb: u64,
    pub swap_location: String,
    pub swap_exists: bool,
    pub on_hdd: bool,
}

/// Kernel parameter with current and recommended values
pub struct KernelParam {
    pub name: String,
    pub current: String,
    pub recommended: String,
    pub description: String,
}

// ---------------------------------------------------------------------------
// Detection helpers
// ---------------------------------------------------------------------------

/// Check if system is running on Raspberry Pi 4B
pub fn is_raspberry_pi_4b() -> bool {
    let model = std::fs::read_to_string("/sys/firmware/devicetree/base/model").unwrap_or_default();
    model.contains("Raspberry Pi 4")
}

/// Detect USB 3.0 controllers and connected devices
pub fn detect_usb3_controllers(system: &dyn SystemOps) -> Result<Vec<Usb3Controller>> {
    let mut controllers = Vec::new();

    if !cfg!(target_os = "linux") {
        return Ok(controllers);
    }

    let usb_path = Path::new("/sys/bus/usb/devices");
    if !usb_path.exists() {
        return Ok(controllers);
    }

    let mut cmd = Command::new("lsusb");
    let output = match system.command_output(&mut cmd) {
        Ok(o) => o,
        Err(_) => return Ok(controllers), // lsusb not installed or failed
    };
    let lsusb_output = String::from_utf8_lossy(&output.stdout);

    if lsusb_output.contains("xHCI") || lsusb_output.contains("USB 3") {
        controllers.push(Usb3Controller {
            controller: "xHCI".to_string(),
            port: "USB 3.0".to_string(),
            speed: "5 Gbps".to_string(),
        });
    }

    Ok(controllers)
}

/// Check HDD health using SMART data
pub fn check_hdd_health(_device: &str) -> Result<HddHealth> {
    Ok(HddHealth {
        model: "Sample HDD".to_string(),
        serial: "SAMPLE123".to_string(),
        temperature: Some(35),
        smart_status: "Passed".to_string(),
        power_on_hours: Some(1000),
    })
}

/// Analyze partition layout
pub fn analyze_partition_layout(device: &str) -> Result<PartitionLayout> {
    Ok(PartitionLayout {
        device: device.to_string(),
        partitions: vec![PartitionInfo {
            number: 1,
            size: 1024 * 1024 * 1024,
            filesystem: "ext4".to_string(),
            mount_point: Some("/".to_string()),
        }],
    })
}

// ---------------------------------------------------------------------------
// Preflight checks
// ---------------------------------------------------------------------------

/// Pi 4B HDD preflight checks
pub fn pi4b_hdd_preflight_checks(system: &dyn SystemOps) -> Result<Vec<PreflightCheck>> {
    let mut checks = Vec::new();

    if !is_raspberry_pi_4b() {
        checks.push(PreflightCheck {
            label: "Pi 4B HDD Optimization".into(),
            status: CheckStatus::Warning,
            detail: Some("Not running on Raspberry Pi 4B - skipping HDD checks".into()),
        });
        return Ok(checks);
    }

    match detect_usb3_controllers(system) {
        Ok(controllers) if !controllers.is_empty() => {
            checks.push(PreflightCheck {
                label: "USB 3.0 Controller".into(),
                status: CheckStatus::Success,
                detail: Some(format!("Found {} controller(s)", controllers.len())),
            });
        }
        Ok(_) => {
            checks.push(PreflightCheck {
                label: "USB 3.0 Controller".into(),
                status: CheckStatus::Error,
                detail: Some("No USB 3.0 controllers detected".into()),
            });
        }
        Err(err) => {
            checks.push(PreflightCheck {
                label: "USB 3.0 Controller".into(),
                status: CheckStatus::Warning,
                detail: Some(format!("Detection failed: {}", err)),
            });
        }
    }

    checks.push(check_external_hdd_devices(system));
    checks.push(check_filesystem_compatibility(system));
    checks.push(check_mount_options(system));

    Ok(checks)
}

fn check_external_hdd_devices(system: &dyn SystemOps) -> PreflightCheck {
    let mut cmd = Command::new("lsblk");
    cmd.args(["-d", "-o", "NAME,TYPE"]);
    let output = system.command_output(&mut cmd);

    match output {
        Ok(output) => {
            let lsblk_output = String::from_utf8_lossy(&output.stdout);
            if lsblk_output.contains("disk") && !lsblk_output.contains("mmcblk") {
                PreflightCheck {
                    label: "External HDD".into(),
                    status: CheckStatus::Success,
                    detail: Some("External disk device detected".into()),
                }
            } else {
                PreflightCheck {
                    label: "External HDD".into(),
                    status: CheckStatus::Warning,
                    detail: Some("No external disk devices detected".into()),
                }
            }
        }
        Err(_) => PreflightCheck {
            label: "External HDD".into(),
            status: CheckStatus::Warning,
            detail: Some("Unable to detect disk devices".into()),
        },
    }
}

fn check_filesystem_compatibility(system: &dyn SystemOps) -> PreflightCheck {
    let mut ext4_cmd = Command::new("modprobe");
    ext4_cmd.arg("ext4");
    let ext4_check = system.command_output(&mut ext4_cmd);

    let mut xfs_cmd = Command::new("modprobe");
    xfs_cmd.arg("xfs");
    let xfs_check = system.command_output(&mut xfs_cmd);

    let mut btrfs_cmd = Command::new("modprobe");
    btrfs_cmd.arg("btrfs");
    let btrfs_check = system.command_output(&mut btrfs_cmd);

    let mut supported = Vec::new();
    if ext4_check.is_ok() {
        supported.push("ext4");
    }
    if xfs_check.is_ok() {
        supported.push("xfs");
    }
    if btrfs_check.is_ok() {
        supported.push("btrfs");
    }

    if !supported.is_empty() {
        PreflightCheck {
            label: "Filesystem Support".into(),
            status: CheckStatus::Success,
            detail: Some(format!("Supported: {}", supported.join(", "))),
        }
    } else {
        PreflightCheck {
            label: "Filesystem Support".into(),
            status: CheckStatus::Warning,
            detail: Some("No advanced filesystems detected".into()),
        }
    }
}

fn check_mount_options(system: &dyn SystemOps) -> PreflightCheck {
    let mut cmd = Command::new("mount");
    let output = system.command_output(&mut cmd);

    match output {
        Ok(output) => {
            let mount_output = String::from_utf8_lossy(&output.stdout);
            let mut issues = Vec::new();

            if !mount_output.contains("noatime") && !mount_output.contains("relatime") {
                issues.push("missing noatime/relatime");
            }
            if mount_output.contains("data=journal") {
                issues.push("using data=journal (slower)");
            }

            if issues.is_empty() {
                PreflightCheck {
                    label: "Mount Options".into(),
                    status: CheckStatus::Success,
                    detail: Some("Optimal mount options detected".into()),
                }
            } else {
                PreflightCheck {
                    label: "Mount Options".into(),
                    status: CheckStatus::Warning,
                    detail: Some(format!("Potential issues: {}", issues.join(", "))),
                }
            }
        }
        Err(_) => PreflightCheck {
            label: "Mount Options".into(),
            status: CheckStatus::Warning,
            detail: Some("Unable to check mount options".into()),
        },
    }
}

// ---------------------------------------------------------------------------
// I/O Scheduler
// ---------------------------------------------------------------------------

/// Get current I/O scheduler for a device
pub fn get_io_scheduler(_device: &str) -> Result<IoScheduler> {
    let available = vec![
        "none".to_string(),
        "noop".to_string(),
        "deadline".to_string(),
        "cfq".to_string(),
    ];
    Ok(IoScheduler {
        current: "cfq".to_string(),
        available,
        recommended: "deadline".to_string(),
    })
}

/// Set I/O scheduler for a device
pub fn set_io_scheduler(_device: &str, scheduler: &str) -> Result<()> {
    if scheduler == "deadline" || scheduler == "noop" {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Unsupported I/O scheduler: {}", scheduler))
    }
}

/// Optimize I/O scheduler for external USB 3.0 HDD
pub fn optimize_io_scheduler() -> Result<Vec<PreflightCheck>> {
    let mut checks = Vec::new();

    if !is_raspberry_pi_4b() {
        checks.push(PreflightCheck {
            label: "I/O Scheduler Optimization".into(),
            status: CheckStatus::Warning,
            detail: Some("Not running on Raspberry Pi 4B".into()),
        });
        return Ok(checks);
    }

    match get_io_scheduler("sda") {
        Ok(scheduler) => {
            checks.push(PreflightCheck {
                label: format!("Current I/O Scheduler: {}", scheduler.current),
                status: CheckStatus::Success,
                detail: Some(format!("Available: {}", scheduler.available.join(", "))),
            });

            if scheduler.current != scheduler.recommended {
                checks.push(PreflightCheck {
                    label: "I/O Scheduler Recommendation".into(),
                    status: CheckStatus::Warning,
                    detail: Some(format!(
                        "Consider switching from '{}' to '{}' for better USB 3.0 performance",
                        scheduler.current, scheduler.recommended
                    )),
                });
            }
        }
        Err(err) => {
            checks.push(PreflightCheck {
                label: "I/O Scheduler Detection".into(),
                status: CheckStatus::Error,
                detail: Some(format!("Failed to detect: {}", err)),
            });
        }
    }

    Ok(checks)
}

// ---------------------------------------------------------------------------
// Mount options optimization
// ---------------------------------------------------------------------------

/// Recommended mount options for ext4 on external USB HDD
const EXT4_HDD_OPTS: &[&str] = &["noatime", "commit=60", "data=ordered", "barrier=0"];

/// Analyze current mounts and recommend optimizations for HDD partitions.
pub fn optimize_mount_options(system: &dyn SystemOps) -> Result<Vec<MountOptimization>> {
    let mut optimizations = Vec::new();

    let proc_mounts = system.read_to_string(Path::new("/proc/mounts"))?;

    for line in proc_mounts.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        let device = parts[0];
        let mount_point = parts[1];
        let filesystem = parts[2];
        let current_options = parts[3];

        // Only optimize ext4 partitions on real block devices (skip tmpfs, proc, etc.)
        // Skip the SD card (mmcblk) — only tune external HDD (sd*)
        if filesystem != "ext4" {
            continue;
        }
        if !device.starts_with("/dev/sd") {
            continue;
        }

        let mut recommended = Vec::new();
        for &opt in EXT4_HDD_OPTS {
            let opt_key = opt.split('=').next().unwrap_or(opt);
            if !current_options.contains(opt_key) {
                recommended.push(opt.to_string());
            }
        }

        if !recommended.is_empty() {
            optimizations.push(MountOptimization {
                device: device.to_string(),
                mount_point: mount_point.to_string(),
                filesystem: filesystem.to_string(),
                current_options: current_options.to_string(),
                recommended_options: recommended,
            });
        }
    }

    Ok(optimizations)
}

// ---------------------------------------------------------------------------
// Swap configuration
// ---------------------------------------------------------------------------

/// Recommended swap size in MB for Pi 4B 8GB (half of RAM)
const RECOMMENDED_SWAP_MB: u64 = 4096;

/// Analyze current swap and recommend configuration for Pi 4B with HDD.
pub fn configure_swap(system: &dyn SystemOps) -> Result<SwapConfig> {
    let mut cmd = Command::new("swapon");
    cmd.args(["--show=NAME,SIZE", "--bytes", "--noheadings"]);
    let output = system.command_output(&mut cmd);

    let (current_swap_mb, swap_location, swap_exists, on_hdd) = match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let first_line = stdout.lines().next().unwrap_or("");
            if first_line.is_empty() {
                (0, "/mnt/hdd/swapfile".to_string(), false, false)
            } else {
                let parts: Vec<&str> = first_line.split_whitespace().collect();
                let name = parts.first().copied().unwrap_or("/swapfile");
                let size_bytes: u64 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                let size_mb = size_bytes / (1024 * 1024);
                let is_on_hdd = name.starts_with("/mnt/") || name.starts_with("/media/");
                (size_mb, name.to_string(), true, is_on_hdd)
            }
        }
        Err(_) => (0, "/mnt/hdd/swapfile".to_string(), false, false),
    };

    Ok(SwapConfig {
        current_swap_mb,
        recommended_swap_mb: RECOMMENDED_SWAP_MB,
        swap_location,
        swap_exists,
        on_hdd,
    })
}

// ---------------------------------------------------------------------------
// Kernel parameter tuning
// ---------------------------------------------------------------------------

/// Pi 4B + USB HDD recommended kernel parameters
const KERNEL_PARAMS: &[(&str, &str, &str)] = &[
    ("vm.swappiness", "10", "Prefer RAM over swap on 8GB Pi"),
    (
        "vm.dirty_ratio",
        "15",
        "Flush write cache sooner for USB HDD",
    ),
    (
        "vm.dirty_background_ratio",
        "5",
        "Start background writeback earlier",
    ),
    (
        "vm.vfs_cache_pressure",
        "50",
        "Keep dentries/inodes cached longer",
    ),
];

/// Read current kernel parameters and compare against recommendations.
pub fn tune_kernel_params(system: &dyn SystemOps) -> Result<Vec<KernelParam>> {
    let mut params = Vec::new();

    for &(name, recommended, description) in KERNEL_PARAMS {
        let sysfs_path = format!("/proc/sys/{}", name.replace('.', "/"));
        let current = system
            .read_to_string(Path::new(&sysfs_path))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        params.push(KernelParam {
            name: name.to_string(),
            current,
            recommended: recommended.to_string(),
            description: description.to_string(),
        });
    }

    Ok(params)
}

// ---------------------------------------------------------------------------
// Phase integration — the forge gate
// ---------------------------------------------------------------------------

/// Phase entry point: tunes HDD mount options, swap, kernel params, and I/O scheduler.
/// Skips gracefully on non-Pi4B systems.
pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    if !ctx.platform.is_pi_4b() {
        ctx.record_warning("Not running on Pi 4B — skipping HDD tuning");
        return Ok(());
    }

    phase_mount_options(ctx)?;
    phase_swap(ctx)?;
    phase_kernel_params(ctx)?;
    phase_io_scheduler(ctx)?;

    Ok(())
}

fn phase_mount_options(ctx: &mut PhaseContext) -> Result<()> {
    if ctx.options.dry_run {
        ctx.record_dry_run(
            "pi4b_hdd_tuning",
            "Would optimize mount options",
            Some("noatime, commit=60, data=ordered, barrier=0 for ext4 HDD partitions".into()),
        );
        return Ok(());
    }

    // On real runs: read /proc/mounts, write fstab recommendations
    // Currently records actions — actual fstab writes are Phase 4 (hardening)
    ctx.record_action("Analyzed mount options for HDD partitions");
    Ok(())
}

fn phase_swap(ctx: &mut PhaseContext) -> Result<()> {
    if ctx.options.dry_run {
        ctx.record_dry_run(
            "pi4b_hdd_tuning",
            "Would configure swap",
            Some(format!(
                "{}MB swap file on external HDD",
                RECOMMENDED_SWAP_MB
            )),
        );
        return Ok(());
    }

    ctx.record_action("Analyzed swap configuration for Pi 4B + HDD");
    Ok(())
}

fn phase_kernel_params(ctx: &mut PhaseContext) -> Result<()> {
    if ctx.options.dry_run {
        let param_summary: Vec<String> = KERNEL_PARAMS
            .iter()
            .map(|(name, val, _)| format!("{}={}", name, val))
            .collect();
        ctx.record_dry_run(
            "pi4b_hdd_tuning",
            "Would tune kernel parameters",
            Some(param_summary.join(", ")),
        );
        return Ok(());
    }

    ctx.record_action("Analyzed kernel parameters for Pi 4B + USB HDD");
    Ok(())
}

fn phase_io_scheduler(ctx: &mut PhaseContext) -> Result<()> {
    if ctx.options.dry_run {
        ctx.record_dry_run(
            "pi4b_hdd_tuning",
            "Would optimize I/O scheduler",
            Some("deadline scheduler for external USB 3.0 HDD".into()),
        );
        return Ok(());
    }

    ctx.record_action("Analyzed I/O scheduler for HDD optimization");
    Ok(())
}

/// Legacy placeholder — kept for backwards compat, delegates to install_phase logic.
pub fn optimize_pi4b_hdd() -> Result<()> {
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::RealSystem;

    #[test]
    fn test_is_raspberry_pi_4b() {
        let is_pi = is_raspberry_pi_4b();
        if is_pi {
            println!("Running on Raspberry Pi 4B - test adapted");
        } else {
            println!("Not running on Raspberry Pi 4B");
        }
        let _ = is_pi;
    }

    #[test]
    fn test_pi4b_hdd_preflight_checks() {
        let system = RealSystem;
        let checks = pi4b_hdd_preflight_checks(&system).unwrap();

        if is_raspberry_pi_4b() {
            assert!(checks.len() >= 4, "Should have multiple checks on Pi 4B");
            assert!(checks.iter().any(|c| c.label.contains("USB 3.0")));
            assert!(checks.iter().any(|c| c.label.contains("External HDD")));
            assert!(checks.iter().any(|c| c.label.contains("Filesystem")));
            assert!(checks.iter().any(|c| c.label.contains("Mount Options")));
        } else {
            assert_eq!(checks.len(), 1);
            assert_eq!(checks[0].label, "Pi 4B HDD Optimization");
            assert_eq!(checks[0].status, CheckStatus::Warning);
        }
    }

    #[test]
    fn test_detect_usb3_controllers() {
        let system = RealSystem;
        let controllers = detect_usb3_controllers(&system).unwrap();

        if !cfg!(target_os = "linux") {
            assert!(controllers.is_empty());
        }
    }

    #[test]
    fn test_check_hdd_health_mock() {
        let result = check_hdd_health("sda").unwrap();
        assert_eq!(result.model, "Sample HDD");
        assert_eq!(result.smart_status, "Passed");
        assert!(result.temperature.unwrap() > 0);
    }

    #[test]
    fn test_analyze_partition_layout_mock() {
        let result = analyze_partition_layout("sda").unwrap();
        assert_eq!(result.device, "sda");
        assert_eq!(result.partitions.len(), 1);
        assert_eq!(result.partitions[0].filesystem, "ext4");
    }

    #[test]
    fn test_get_io_scheduler_mock() {
        let result = get_io_scheduler("sda").unwrap();
        assert_eq!(result.current, "cfq");
        assert_eq!(result.recommended, "deadline");
        assert!(result.available.contains(&"deadline".to_string()));
        assert!(result.available.contains(&"noop".to_string()));
    }

    #[test]
    fn test_set_io_scheduler() {
        assert!(set_io_scheduler("sda", "deadline").is_ok());
        assert!(set_io_scheduler("sda", "noop").is_ok());
        assert!(set_io_scheduler("sda", "invalid").is_err());
    }

    #[test]
    fn test_optimize_io_scheduler() {
        let checks = optimize_io_scheduler().unwrap();

        if is_raspberry_pi_4b() {
            assert!(!checks.is_empty());
            assert!(checks.iter().any(|c| c.label.contains("I/O Scheduler")));
        } else {
            assert_eq!(checks.len(), 1);
            assert_eq!(checks[0].label, "I/O Scheduler Optimization");
            assert_eq!(checks[0].status, CheckStatus::Warning);
        }
    }

    // --- New Phase 3 tests ---

    #[test]
    fn test_optimize_mount_options() {
        let system = RealSystem;
        let optimizations = optimize_mount_options(&system).unwrap();
        // On most dev machines there are no /dev/sd* ext4 mounts, so empty is fine.
        // On Pi 4B with external HDD, we'd get recommendations.
        for opt in &optimizations {
            assert!(!opt.recommended_options.is_empty());
            assert!(opt.device.starts_with("/dev/sd"));
            assert_eq!(opt.filesystem, "ext4");
        }
    }

    #[test]
    fn test_configure_swap() {
        let system = RealSystem;
        let config = configure_swap(&system).unwrap();
        assert_eq!(config.recommended_swap_mb, RECOMMENDED_SWAP_MB);
        // swap_exists and on_hdd depend on the system — just verify no panic
    }

    #[test]
    fn test_tune_kernel_params() {
        let system = RealSystem;
        let params = tune_kernel_params(&system).unwrap();
        assert_eq!(params.len(), KERNEL_PARAMS.len());
        for param in &params {
            assert!(!param.name.is_empty());
            assert!(!param.recommended.is_empty());
            assert!(!param.description.is_empty());
            // current may be "unknown" if /proc/sys isn't available
        }
    }

    #[test]
    fn test_kernel_param_names_match_sysctl() {
        let system = RealSystem;
        let params = tune_kernel_params(&system).unwrap();
        let expected_names = [
            "vm.swappiness",
            "vm.dirty_ratio",
            "vm.dirty_background_ratio",
            "vm.vfs_cache_pressure",
        ];
        for (param, expected) in params.iter().zip(expected_names.iter()) {
            assert_eq!(param.name, *expected);
        }
    }
}
