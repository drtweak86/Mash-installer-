//! Sysinfo poller thread — reads CPU/RAM/NET from /proc.
//! Replaced 'sysinfo' with manual /proc parsing for a leaner forge.

use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use installer_core::proc::{read_net_bytes, CpuStats, MemStats};

use crate::tui::app::TuiMessage;

// ── Spawner ──────────────────────────────────────────────────────────────────

/// Spawn the sysinfo poller. Polls every 1 second and sends `TuiMessage::SysStats`.
pub fn spawn_sysinfo_poller(tx: Sender<TuiMessage>) {
    thread::spawn(move || {
        let (mut prev_rx, _) = read_net_bytes();
        let mut prev_cpu = CpuStats::read().unwrap_or_default();

        loop {
            thread::sleep(Duration::from_secs(1));

            // 1. CPU Usage
            let current_cpu = CpuStats::read().unwrap_or_default();
            let cpu_pct = CpuStats::calculate_usage(&prev_cpu, &current_cpu);
            prev_cpu = current_cpu;

            // 2. Memory Usage
            let mem = MemStats::read().unwrap_or_default();
            let ram_used_mb = mem.used_kb / 1024;
            let ram_total_mb = mem.total_kb / 1024;

            // 3. Network Usage
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
