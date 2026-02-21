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

// ── Spawner ──────────────────────────────────────────────────────────────────

/// Spawn the sysinfo poller. Polls every 1 second and sends `TuiMessage::SysStats`.
pub fn spawn_sysinfo_poller(tx: Sender<TuiMessage>) {
    thread::spawn(move || {
        let refresh = RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
            .with_memory(MemoryRefreshKind::nothing().with_ram());

        let mut sys = System::new_with_specifics(refresh);

        let (mut prev_rx, _) = read_net_bytes();

        loop {
            thread::sleep(Duration::from_secs(1));

            sys.refresh_specifics(refresh);

            let cpu_pct = sys.global_cpu_usage();

            let ram_used_mb = sys.used_memory() / 1_048_576;
            let ram_total_mb = sys.total_memory() / 1_048_576;

            let (rx_bytes, _) = read_net_bytes();
            let net_rx_kbps = (rx_bytes.saturating_sub(prev_rx)) as f32 / 1024.0;
            prev_rx = rx_bytes;

            let msg = TuiMessage::SysStats {
                cpu_pct,
                ram_used_mb,
                ram_total_mb,
                net_rx_kbps,
            };

            if tx.send(msg).is_err() {
                break;
            }
        }
    });
}
