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

use tracing::{debug, error, info, warn};

use crate::cmd;
use crate::phase_runner::PhaseObserver;
use crate::sudo_password;

/// Handle for the sudo keep-alive background thread.
pub struct SudoKeepalive {
    stop_flag: Arc<AtomicBool>,
}

impl Drop for SudoKeepalive {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst);
    }
}

/// Check if sudo access is available. If not, prompt for password and verify.
pub fn ensure_sudo_access(observer: &mut dyn PhaseObserver) -> bool {
    // 1. Try passwordless sudo first
    let mut test_cmd = Command::new("sudo");
    test_cmd.args(["-n", "-v"]); // -n = non-interactive
    
    if cmd::run(&mut test_cmd).is_ok() {
        debug!("sudo access verified (passwordless)");
        return true;
    }

    // 2. If passwordless fails, we need a password
    debug!("sudo requires a password; requesting from observer");
    match observer.sudo_password() {
        Ok(pass) if !pass.is_empty() => {
            // Store the password globally so cmd::run can use it
            sudo_password::set_sudo_password(pass.clone());
            
            // Verify the password with sudo -S -v
            let mut verify_cmd = Command::new("sudo");
            verify_cmd.args(["-S", "-v"]);
            verify_cmd.stdin(Stdio::piped());
            verify_cmd.stdout(Stdio::null());
            verify_cmd.stderr(Stdio::null());

            if let Ok(mut child) = verify_cmd.spawn() {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = writeln!(stdin, "{}", pass);
                }
                match child.wait() {
                    Ok(status) if status.success() => {
                        info!("sudo access verified with password");
                        true
                    }
                    _ => {
                        error!("sudo authentication failed; incorrect password");
                        sudo_password::clear_sudo_password();
                        false
                    }
                }
            } else {
                false
            }
        }
        Ok(_) => {
            warn!("no sudo password provided; certain phases may fail");
            false
        }
        Err(e) => {
            error!("failed to request sudo password: {}", e);
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
        debug!("Starting sudo keep-alive (refreshes every 30s)");

        loop {
            if flag_clone.load(Ordering::SeqCst) {
                debug!("Stopping sudo keep-alive");
                break;
            }

            let mut cmd = Command::new("sudo");
            // If we have a password, use it
            if let Some(pass) = sudo_password::get_sudo_password() {
                cmd.args(["-S", "-v"]);
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
                let _ = cmd.status();
            }

            thread::sleep(Duration::from_secs(30));
        }
    });

    SudoKeepalive { stop_flag }
}
