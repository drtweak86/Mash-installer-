//! Sysinfo poller thread — reads CPU/RAM via sysinfo, NET/IO from /proc.

use std::fs;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use crate::tui::app::TuiMessage;

// ── /proc helpers ────────────────────────────────────────────────────────────

/// Returns (rx_bytes, tx_bytes) summed across all non-loopback interfaces.
fn read_net_bytes() -> (u64, u64) {
    let Ok(content) = fs::read_to_string("/proc/net/dev") else {
        return (0, 0);
    };
    let mut rx_total = 0u64;
    let mut tx_total = 0u64;
    for line in content.lines().skip(2) {
        // Format: "  eth0: rx_bytes rx_packets ... tx_bytes ..."
        let line = line.trim();
        let Some(colon) = line.find(':') else {
            continue;
        };
        let iface = &line[..colon];
        if iface.trim() == "lo" {
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

/// Returns (read_sectors, write_sectors) summed across all non-loop block devices.
fn read_disk_sectors() -> (u64, u64) {
    let Ok(content) = fs::read_to_string("/proc/diskstats") else {
        return (0, 0);
    };
    let mut read_total = 0u64;
    let mut write_total = 0u64;
    for line in content.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 10 {
            continue;
        }
        let device = fields[2];
        // Skip loop/ram devices
        if device.starts_with("loop") || device.starts_with("ram") {
            continue;
        }
        // Skip partition entries (only aggregate parent devices like sda, nvme0n1)
        if device.chars().last().is_some_and(|c| c.is_ascii_digit())
            && (device.starts_with("sd") || device.starts_with("hd"))
        {
            continue;
        }
        let read_sectors: u64 = fields[5].parse().unwrap_or(0);
        let write_sectors: u64 = fields[9].parse().unwrap_or(0);
        read_total += read_sectors;
        write_total += write_sectors;
    }
    (read_total, write_total)
}

// ── Spawner ──────────────────────────────────────────────────────────────────

/// Spawn the sysinfo poller. Polls every 1 second and sends `TuiMessage::SysStats`.
pub fn spawn_sysinfo_poller(tx: Sender<TuiMessage>) {
    thread::spawn(move || {
        let refresh = RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
            .with_memory(MemoryRefreshKind::nothing().with_ram());

        let mut sys = System::new_with_specifics(refresh);

        let (mut prev_rx, mut prev_tx) = read_net_bytes();
        let (mut prev_read, mut prev_write) = read_disk_sectors();

        loop {
            thread::sleep(Duration::from_secs(1));

            sys.refresh_specifics(refresh);

            let cpu_pct = sys.global_cpu_usage();

            let ram_used_mb = sys.used_memory() / 1_048_576;
            let ram_total_mb = sys.total_memory() / 1_048_576;

            let (rx_bytes, tx_bytes) = read_net_bytes();
            let net_rx_kbps = (rx_bytes.saturating_sub(prev_rx)) as f32 / 1024.0;
            let net_tx_kbps = (tx_bytes.saturating_sub(prev_tx)) as f32 / 1024.0;
            prev_rx = rx_bytes;
            prev_tx = tx_bytes;

            // Sectors are 512 bytes each
            let (read_sec, write_sec) = read_disk_sectors();
            let io_r_kbps = (read_sec.saturating_sub(prev_read)) as f32 * 512.0 / 1024.0;
            let io_w_kbps = (write_sec.saturating_sub(prev_write)) as f32 * 512.0 / 1024.0;
            prev_read = read_sec;
            prev_write = write_sec;

            let msg = TuiMessage::SysStats {
                cpu_pct,
                ram_used_mb,
                ram_total_mb,
                net_rx_kbps,
                net_tx_kbps,
                io_r_kbps,
                io_w_kbps,
            };

            if tx.send(msg).is_err() {
                break;
            }
        }
    });
}
