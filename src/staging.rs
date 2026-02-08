use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::MashConfig;

/// Minimum free space on the staging filesystem in bytes (500 MiB).
const MIN_FREE_BYTES: u64 = 500 * 1024 * 1024;

/// Resolve and validate the staging directory.
///
/// Rules:
/// 1. CLI override wins.
/// 2. Otherwise use config value.
/// 3. Refuse to stage on `/` (root fs) when free space is below threshold.
/// 4. Create the directory if it doesn't exist.
pub fn resolve(cli_override: Option<&Path>, cfg: &MashConfig) -> Result<PathBuf> {
    let dir = match cli_override {
        Some(p) => p.to_path_buf(),
        None => cfg.staging_dir.clone(),
    };

    ensure_space_for_path(&dir)?;

    fs::create_dir_all(&dir).with_context(|| format!("creating staging dir {}", dir.display()))?;

    Ok(dir)
}

/// Ensure the filesystem that would contain `path` has enough free space.
pub fn ensure_space_for_path(path: &Path) -> Result<()> {
    let existing = find_existing_ancestor(path);
    check_space(path, &existing)
}

/// Check that the filesystem containing `path` has enough free space.
/// Refuse to proceed if the mount point is `/` and space is low.
fn check_space(path: &Path, existing: &Path) -> Result<()> {
    let stat = nix::sys::statvfs::statvfs(path)
        .or_else(|_| nix::sys::statvfs::statvfs(existing))
        .with_context(|| format!("statvfs on {}", existing.display()))?;

    let avail = stat.blocks_available() as u64 * stat.fragment_size() as u64;
    let mount = find_mount_point(path);

    if mount.as_deref() == Some(Path::new("/")) && avail < MIN_FREE_BYTES {
        bail!(
            "Staging would land on the root filesystem which only has {} MiB free.\n\
             Pass --staging-dir to point at a larger mounted drive (e.g. /mnt/data/mash-installer).",
            avail / (1024 * 1024)
        );
    }

    if avail < MIN_FREE_BYTES {
        tracing::warn!(
            "Low disk space on {}: {} MiB available",
            path.display(),
            avail / (1024 * 1024)
        );
    }

    Ok(())
}

/// Find the closest existing ancestor so statvfs can succeed.
fn find_existing_ancestor(path: &Path) -> PathBuf {
    for ancestor in path.ancestors() {
        if ancestor.exists() {
            return ancestor.to_path_buf();
        }
    }
    PathBuf::from("/")
}

/// Best-effort mount-point detection via /proc/mounts.
fn find_mount_point(path: &Path) -> Option<PathBuf> {
    let mounts = fs::read_to_string("/proc/mounts").ok()?;
    let canonical = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    let mut best: Option<PathBuf> = None;
    let mut best_len = 0;

    for line in mounts.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let mp = Path::new(parts[1]);
        if canonical.starts_with(mp) {
            let len = mp.as_os_str().len();
            if len > best_len {
                best_len = len;
                best = Some(mp.to_path_buf());
            }
        }
    }
    best
}
