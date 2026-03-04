//! Linux /proc filesystem parsing helpers.
//! Forged to replace the heavy 'sysinfo' dependency.

use std::fs;

/// CPU usage statistics from /proc/stat
#[derive(Debug, Default, Clone, Copy)]
pub struct CpuStats {
    pub total: u64,
    pub idle: u64,
}

impl CpuStats {
    /// Read current CPU stats from /proc/stat
    pub fn read() -> Option<Self> {
        let content = fs::read_to_string("/proc/stat").ok()?;
        let first_line = content.lines().next()?;
        if !first_line.starts_with("cpu ") {
            return None;
        }

        let parts: Vec<u64> = first_line
            .split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse().ok())
            .collect();

        if parts.len() < 7 {
            return None;
        }

        // user + nice + system + idle + iowait + irq + softirq
        let total: u64 = parts.iter().sum();
        let idle = parts[3]; // idle time

        Some(Self { total, idle })
    }

    /// Calculate CPU usage percentage between two snapshots
    pub fn calculate_usage(prev: &Self, current: &Self) -> f32 {
        let total_diff = current.total.saturating_sub(prev.total);
        let idle_diff = current.idle.saturating_sub(prev.idle);

        if total_diff == 0 {
            return 0.0;
        }

        let used_diff = total_diff.saturating_sub(idle_diff);
        (used_diff as f32 / total_diff as f32) * 100.0
    }
}

/// Memory statistics from /proc/meminfo
#[derive(Debug, Default, Clone, Copy)]
pub struct MemStats {
    pub total_kb: u64,
    pub available_kb: u64,
    pub used_kb: u64,
    pub swap_total_kb: u64,
    pub swap_free_kb: u64,
}

impl MemStats {
    pub fn read() -> Option<Self> {
        let content = fs::read_to_string("/proc/meminfo").ok()?;
        let mut stats = Self::default();

        for line in content.lines() {
            let mut parts = line.split_whitespace();
            let key = parts.next()?.trim_end_matches(':');
            let val = parts.next()?.parse::<u64>().ok()?;

            match key {
                "MemTotal" => stats.total_kb = val,
                "MemAvailable" => stats.available_kb = val,
                "SwapTotal" => stats.swap_total_kb = val,
                "SwapFree" => stats.swap_free_kb = val,
                _ => {}
            }
        }

        stats.used_kb = stats.total_kb.saturating_sub(stats.available_kb);
        Some(stats)
    }
}

/// Read CPU model name from /proc/cpuinfo
pub fn read_cpu_model() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") || line.starts_with("Model") {
                if let Some((_, v)) = line.split_once(':') {
                    return v.trim().to_string();
                }
            }
        }
    }
    "Unknown CPU".to_string()
}

/// Returns (rx_bytes, tx_bytes) summed across all non-loopback interfaces.
pub fn read_net_bytes() -> (u64, u64) {
    let Ok(content) = fs::read_to_string("/proc/net/dev") else {
        return (0, 0);
    };
    let mut rx_total = 0u64;
    let mut tx_total = 0u64;
    for line in content.lines().skip(2) {
        let line = line.trim();
        let Some(colon) = line.find(':') else {
            continue;
        };
        let iface = &line[..colon].trim();
        if *iface == "lo" {
            continue;
        }
        let nums: Vec<u64> = line[colon + 1..]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() >= 9 {
            rx_total += nums[0];
            tx_total += nums[8];
        }
    }
    (rx_total, tx_total)
}
