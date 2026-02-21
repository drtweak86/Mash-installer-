//! BBS message bank and cycler thread for the MASH TUI.

use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::tui::app::TuiMessage;

pub const BBS_MESSAGES: &[&str] = &[
    "🔮 Summoning the daemon lords of package management...",
    "⚡ Hacking the Gibson... relax, it's just docker",
    "🧙 sudo make me a sandwich — root graciously agrees",
    "🦀 Rustaceans ferrying cargo at ludicrous speed",
    "💾 Rewinding the tape drive... psych, it's NVMe",
    "🌐 Downloading more RAM... that's still not how it works",
    "🎲 Rolling d20 on dependency resolution... CRITICAL HIT",
    "⚔️  Fighting dependency hell with a +5 sword of semver",
    "📡 Transmitting signal to the orbital package depot",
    "🧬 Splicing your dev environment's DNA sequence",
    "🎸 Compiling at 11... clippy says dial it back to -D warnings",
    "🌌 Bending space-time to install packages faster than light",
    "🍕 Have you tried turning it off and on again? (We did.)",
    "🧪 Lab report: zero bugs detected (fingers firmly crossed)",
    "⛏️  Mining plasma ore from the digital cosmos",
    "🐉 The dragon of broken configs has been slain. Probably.",
    "🏴‍☠️  Arrr, hoisting the Jolly Roger of open source",
    "🤖 SYSTEM BOOTING... beep boop... praise your new digital overlord",
    "🎯 Target acquired: your system. Installing awesome. Stand by.",
    "🔬 Microscopic analysis: 0 bugs (in prod. dev has infinite)",
    "🛸 Phoning home to download dependencies from orbit",
    "🧩 Solving the 4D puzzle of transitive dependency trees",
    "🧲 Attracting packages with a magnetic personality",
    "⚗️  Transmuting base metals into deployable artifacts",
    "🌊 Surfing the wave of functional purity",
    "🎪 Step right up! Watch as 47 packages become one binary!",
    "🦄 This installer runs on pure unicorn farts and clippy green",
    "🕹️  INSERT COIN TO CONTINUE... just kidding, it's free software",
    "📻 BBS calling... *screee khhhhh* ... connected at 56K",
    "🗡️  Your system is being upgraded by the legendary Sword of sudo",
    "🔐 Encrypting your hopes and dreams with AES-256",
    "🎰 Dependency slot machine: three crates in a row! Jackpot!",
    "🐧 Tux approves of this installation. Probably.",
    "🏗️  Scaffolding the scaffolding that scaffolds things",
    "💿 Installing drivers for your drivers' drivers",
    "🌀 Entering the dependency matrix... no red pill for you",
    "📦 Unboxing 1,337 crates of pure digital potential",
    "🔭 Scanning the galaxy for compatible versions... found 3",
    "🧊 Keeping your system cool under the pressure of apt-get",
    "🎆 Firing up the afterburners — cargo build --release",
    "🏎️  Turbo mode activated: now compiling at the speed of boredom",
    "🌺 Planting seeds of configuration in the soil of /etc",
    "🦔 Hedgehog mode: curling up all the dependencies",
    "🎷 Jazz hands? No, just cargo --features jazz",
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
