//! BBS message bank and cycler thread for the MASH TUI.

use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::tui::app::TuiMessage;

pub const BBS_MESSAGES: &[&str] = &[
    "⚡ FORGE STATUS: WHITE-HOT. RAISING TANKARDS...",
    "🛰️ CONNECTING TO NEON SUBNET... LINK ESTABLISHED.",
    "🍺 THE BARD RECOMMENDS: S-TIER BREWS AND STABLE BUILDS.",
    "🛡️ HARDENING SYSTEM RUNES... SIGILS GLOWING.",
    "🚀 READY TO STRIKE STEEL. THE MINER AWAITS.",
    "💾 SCRYING MACHINE PEDIGREE... ALL SYSTEMS OPTIMAL.",
    "🔥 STAY THIRSTY, KEEP SMITHING!",
    "🛠️ OPTIMIZING I/O CORES... PLASMA ORE REFINED.",
    "🌌 BENDING SPACE-TIME TO INSTALL PACKAGES FASTER THAN LIGHT.",
    "🦀 THE BORROW CHECKER HAS REVIEWED YOUR LIFE CHOICES. APPROVED.",
];

/// Spawn the BBS cycler thread. It sends a new message every 4 seconds.
pub fn spawn_bbs_cycler(tx: Sender<TuiMessage>) {
    thread::spawn(move || {
        let mut idx = 0usize;
        loop {
            let msg = BBS_MESSAGES[idx % BBS_MESSAGES.len()].to_string();
            if tx.send(TuiMessage::BbsMessage(msg)).is_err() {
                break;
            }
            idx += 1;
            thread::sleep(Duration::from_secs(4));
        }
    });
}
