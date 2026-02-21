use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use nix::fcntl::{Flock, FlockArg};

/// Exclusive lock that prevents concurrent installer runs.
///
/// The lock is acquired on creation and released automatically when dropped.
pub struct InstallerLock {
    _flock: Flock<File>,
}

impl std::fmt::Debug for InstallerLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstallerLock").finish()
    }
}

impl InstallerLock {
    /// Attempt to acquire an exclusive, non-blocking lock at the default path.
    pub fn acquire() -> Result<Self> {
        Self::acquire_at(&lock_path())
    }

    /// Attempt to acquire an exclusive, non-blocking lock at a specific path.
    fn acquire_at(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating lock directory {}", parent.display()))?;
        }

        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(path)
            .with_context(|| format!("opening lock file {}", path.display()))?;

        let flock = Flock::lock(file, FlockArg::LockExclusiveNonblock).map_err(|(_, errno)| {
            anyhow!(
                "Another mash-installer instance is already running (lock file: {}): {}. \
                     Wait for it to finish or remove the lock file if the previous run crashed.",
                path.display(),
                errno
            )
        })?;

        Ok(Self { _flock: flock })
    }
}

/// Determine the lock file path.
///
/// Prefers `$XDG_RUNTIME_DIR/mash-installer.lock` (user-scoped, tmpfs-backed),
/// falling back to `/var/run/mash-installer.lock` for root installs.
fn lock_path() -> PathBuf {
    std::env::var("XDG_RUNTIME_DIR")
        .map(|dir| PathBuf::from(dir).join("mash-installer.lock"))
        .unwrap_or_else(|_| PathBuf::from("/var/run/mash-installer.lock"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    fn acquire_succeeds_once() {
        let dir = tempdir().unwrap();
        let lock_path = dir.path().join("test.lock");
        let lock = InstallerLock::acquire_at(&lock_path);
        assert!(lock.is_ok(), "first lock should succeed");
    }

    #[test]
    fn double_lock_from_child_process_fails() {
        let dir = tempdir().unwrap();
        let lock_path = dir.path().join("test.lock");
        let _lock1 = InstallerLock::acquire_at(&lock_path).expect("first lock should succeed");

        // flock(2) allows the same process to re-lock, so we need a child
        // process to test the exclusion.
        let output = Command::new("flock")
            .args(["--nonblock", "--exclusive"])
            .arg(&lock_path)
            .arg("true")
            .output()
            .expect("flock command should exist");
        assert!(
            !output.status.success(),
            "child process should fail to acquire the lock"
        );
    }

    #[test]
    fn drop_releases_lock() {
        let dir = tempdir().unwrap();
        let lock_path = dir.path().join("test.lock");
        {
            let _lock = InstallerLock::acquire_at(&lock_path).expect("first lock should succeed");
        }
        // After drop, a new lock should succeed
        let lock2 = InstallerLock::acquire_at(&lock_path);
        assert!(lock2.is_ok(), "lock after drop should succeed");
    }
}
