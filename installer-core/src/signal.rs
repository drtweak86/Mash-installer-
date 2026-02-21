use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::{Context, Result};

/// Guard that registers SIGINT and SIGTERM handlers and exposes an
/// interruption flag that phases can poll between steps.
///
/// When a signal is received, the flag is set to `true`. The orchestrator
/// and phase runner should check `is_interrupted()` between phases and
/// trigger a graceful rollback if set.
pub struct SignalGuard {
    interrupted: Arc<AtomicBool>,
}

impl SignalGuard {
    /// Register signal handlers for SIGINT and SIGTERM.
    ///
    /// The handlers set an atomic flag instead of terminating the process,
    /// giving the installer a chance to roll back partial changes.
    pub fn new() -> Result<Self> {
        let interrupted = Arc::new(AtomicBool::new(false));

        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&interrupted))
            .context("registering SIGINT handler")?;
        signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&interrupted))
            .context("registering SIGTERM handler")?;

        Ok(Self { interrupted })
    }

    /// Check whether an interrupt signal has been received.
    pub fn is_interrupted(&self) -> bool {
        self.interrupted.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_starts_false() {
        let flag = Arc::new(AtomicBool::new(false));
        assert!(!flag.load(Ordering::Relaxed));
    }

    #[test]
    fn flag_toggles_correctly() {
        let flag = Arc::new(AtomicBool::new(false));
        assert!(!flag.load(Ordering::Relaxed));
        flag.store(true, Ordering::Relaxed);
        assert!(flag.load(Ordering::Relaxed));
    }

    #[test]
    fn signal_guard_starts_uninterrupted() {
        let guard = SignalGuard::new().expect("signal guard creation should succeed");
        assert!(
            !guard.is_interrupted(),
            "guard should start in non-interrupted state"
        );
    }
}
