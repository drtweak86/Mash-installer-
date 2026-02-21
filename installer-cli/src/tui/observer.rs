//! RatatuiPhaseObserver — bridges installer-core events into TUI messages.

use std::sync::mpsc::{self, Sender};

use installer_core::{PhaseEvent, PhaseObserver};

use crate::tui::app::TuiMessage;

pub struct RatatuiPhaseObserver {
    tx: Sender<TuiMessage>,
}

impl RatatuiPhaseObserver {
    pub fn new(tx: Sender<TuiMessage>) -> Self {
        Self { tx }
    }
}

impl PhaseObserver for RatatuiPhaseObserver {
    fn on_event(&mut self, event: PhaseEvent) {
        // Silently ignore send errors — TUI may have quit
        let _ = self.tx.send(TuiMessage::Phase(event));
    }

    fn confirm(&mut self, prompt: &str) -> bool {
        // Create a one-shot reply channel
        let (reply_tx, reply_rx) = mpsc::channel::<bool>();
        let msg = TuiMessage::ConfirmPrompt {
            prompt: prompt.to_string(),
            reply: reply_tx,
        };
        if self.tx.send(msg).is_err() {
            // TUI gone — default to true (proceed)
            return true;
        }
        // Block the installer thread until the TUI user responds
        reply_rx.recv().unwrap_or(true)
    }
}
