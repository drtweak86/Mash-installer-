use std::{
    io::Write,
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use tracing::debug;

/// Handle for the sudo keep-alive background thread.
pub struct SudoKeepalive {
    stop_flag: Arc<AtomicBool>,
}

impl Drop for SudoKeepalive {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst);
    }
}

pub fn ensure_sudo_access() -> bool {
    // 1. Try passwordless sudo first
    let mut test_cmd = Command::new("sudo");
    test_cmd.args(["-n", "-v"]); // -n = non-interactive
    test_cmd.stdin(Stdio::null());
    test_cmd.stdout(Stdio::null());
    test_cmd.stderr(Stdio::null());

    if let Ok(status) = test_cmd.status() {
        if status.success() {
            debug!("sudo access verified (passwordless)");
            return true;
        }
    }
    // If passwordless sudo fails, return false. Interactive prompting for password will be handled at a higher layer.
    debug!("passwordless sudo failed; password is required at a higher level");
    false
}

/// Start a background thread to keep sudo alive during long operations.
pub fn start_sudo_keepalive() -> SudoKeepalive {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let flag_clone = stop_flag.clone();

    thread::spawn(move || {
        debug!("Starting sudo keep-alive (refreshes every 30s)");

        loop {
            if flag_clone.load(Ordering::SeqCst) {
                debug!("Stopping sudo keep-alive");
                break;
            }

            let mut cmd = Command::new("sudo");
            // If we have a password, use it
            if let Some(pass) = super::sudo_password::get_sudo_password() {
                cmd.args(["-S", "-p", "", "-v"]);
                cmd.stdin(Stdio::piped());
                cmd.stdout(Stdio::null());
                cmd.stderr(Stdio::null());

                if let Ok(mut child) = cmd.spawn() {
                    if let Some(mut stdin) = child.stdin.take() {
                        let _ = writeln!(stdin, "{}", pass);
                    }
                    let _ = child.wait();
                }
            } else {
                cmd.args(["-n", "-v"]);
                cmd.stdin(Stdio::null());
                cmd.stdout(Stdio::null());
                cmd.stderr(Stdio::null());
                let _ = cmd.status();
            }

            thread::sleep(Duration::from_secs(30));
        }
    });

    SudoKeepalive { stop_flag }
}
