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

/// Start a background thread to keep sudo alive during long operations.
pub fn start_sudo_keepalive() -> SudoKeepalive {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let flag_clone = stop_flag.clone();

    thread::spawn(move || {
        let mut test_cmd = Command::new("sudo");
        test_cmd.args(["-v"]).stdin(std::process::Stdio::inherit());
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
            cmd.args(["-v"]).stdin(std::process::Stdio::inherit());
            if let Err(e) = cmd::run(&mut cmd) {
                warn!("sudo keep-alive refresh failed: {}", e);
                break;
            }
        }
    });

    SudoKeepalive { stop_flag }
}
