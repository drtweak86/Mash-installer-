use std::{
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use tracing::{debug, error, warn};

use crate::cmd;

/// Handle for the sudo keep-alive background thread.
pub struct SudoKeepalive {
    stop_flag: Arc<AtomicBool>,
}

impl Drop for SudoKeepalive {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst);
    }
}

/// Check if sudo requires a password and prompt for it if needed
#[allow(dead_code)]
pub fn ensure_sudo_access() -> bool {
    let mut test_cmd = Command::new("sudo");
    test_cmd.args(["-v"]);
    // Use a pipe for stdin to avoid issues with TUI raw mode
    test_cmd.stdin(std::process::Stdio::piped());

    match cmd::run(&mut test_cmd) {
        Ok(_) => {
            debug!("sudo access verified successfully");
            true
        }
        Err(e) => {
            error!("sudo access check failed: {}", e);
            false
        }
    }
}

/// Start a background thread to keep sudo alive during long operations.
///
/// Note: When running in TUI mode, sudo password prompts may not work properly
/// due to terminal raw mode. Users should either:
/// 1. Configure sudo with NOPASSWD for the installer commands, or
/// 2. Run the installer in non-TUI mode with --no-tui flag
pub fn start_sudo_keepalive() -> SudoKeepalive {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let flag_clone = stop_flag.clone();

    thread::spawn(move || {
        let mut test_cmd = Command::new("sudo");
        test_cmd.args(["-v"]);
        // Use a pipe for stdin to avoid issues with TUI raw mode
        // This prevents sudo from trying to read password from terminal
        test_cmd.stdin(std::process::Stdio::piped());

        if let Err(e) = cmd::run(&mut test_cmd) {
            error!("sudo -v failed: {}. Make sure you have sudo access without password (NOPASSWD) or run the installer manually.", e);
            debug!("Not starting sudo keep-alive due to sudo failure");
            return;
        }

        debug!("Starting sudo keep-alive (refreshes every 30s)");

        loop {
            if flag_clone.load(Ordering::SeqCst) {
                debug!("Stopping sudo keep-alive");
                break;
            }

            thread::sleep(Duration::from_secs(30));

            if flag_clone.load(Ordering::SeqCst) {
                break;
            }

            let mut cmd = Command::new("sudo");
            cmd.args(["-v"]);
            // Use a pipe for stdin to avoid issues with TUI raw mode
            cmd.stdin(std::process::Stdio::piped());
            if let Err(e) = cmd::run(&mut cmd) {
                warn!("sudo keep-alive refresh failed: {}", e);
                break;
            }
        }
    });

    SudoKeepalive { stop_flag }
}
