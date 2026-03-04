use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tracing::{info, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeployStrategy {
    Copy,
    Symlink,
}

pub struct DotfileManager<'a> {
    base_path: &'a Path, // Usually user home
    dry_run: bool,
}

impl<'a> DotfileManager<'a> {
    pub fn new(base_path: &'a Path, dry_run: bool) -> Self {
        Self { base_path, dry_run }
    }

    /// Deploys a dotfile from source to target.
    /// target_rel is relative to the base_path (home).
    pub fn deploy(&self, source: &Path, target_rel: &Path, strategy: DeployStrategy) -> Result<()> {
        let target = self.base_path.join(target_rel);

        if self.dry_run {
            info!(
                "[dry-run] Would deploy {} to {} using {:?}",
                source.display(),
                target.display(),
                strategy
            );
            return Ok(());
        }

        // 1. Create parent directories
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create parent directory for {}", target.display())
            })?;
        }

        // 2. Handle existing file
        if target.exists() {
            if self.is_identical(source, &target, strategy)? {
                info!("Target {} is already up to date.", target.display());
                return Ok(());
            }
            self.backup(&target)?;
        }

        // 3. Perform deployment
        match strategy {
            DeployStrategy::Copy => {
                fs::copy(source, &target).with_context(|| {
                    format!(
                        "Failed to copy {} to {}",
                        source.display(),
                        target.display()
                    )
                })?;
            }
            DeployStrategy::Symlink => {
                #[cfg(unix)]
                std::os::unix::fs::symlink(source, &target).with_context(|| {
                    format!(
                        "Failed to symlink {} to {}",
                        source.display(),
                        target.display()
                    )
                })?;

                #[cfg(windows)]
                return Err(anyhow::anyhow!(
                    "Symlinking not supported on Windows in this forge."
                ));
            }
        }

        info!("Deployed {} to {}", source.display(), target.display());
        Ok(())
    }

    fn is_identical(&self, source: &Path, target: &Path, strategy: DeployStrategy) -> Result<bool> {
        if strategy == DeployStrategy::Symlink {
            if let Ok(link_target) = fs::read_link(target) {
                return Ok(link_target == source);
            }
        }

        // For Copy or if symlink check failed, compare content
        let source_meta = fs::metadata(source)?;
        let target_meta = fs::metadata(target)?;

        if source_meta.len() != target_meta.len() {
            return Ok(false);
        }

        let source_content = fs::read(source)?;
        let target_content = fs::read(target)?;
        Ok(source_content == target_content)
    }

    fn backup(&self, path: &Path) -> Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let backup_path = path.with_extension(format!("bak.{}", timestamp));

        warn!(
            "Target file {} exists and differs. Backing up to {}...",
            path.display(),
            backup_path.display()
        );

        fs::rename(path, &backup_path).with_context(|| {
            format!(
                "Failed to backup {} to {}",
                path.display(),
                backup_path.display()
            )
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_deploy_copy() -> Result<()> {
        let root = tempdir()?;
        let source_dir = tempdir()?;
        let source_file = source_dir.path().join("config");
        fs::write(&source_file, "original content")?;

        let mgr = DotfileManager::new(root.path(), false);
        let target_rel = Path::new(".config/myapp/config");

        mgr.deploy(&source_file, target_rel, DeployStrategy::Copy)?;

        let target_full = root.path().join(target_rel);
        assert!(target_full.exists());
        assert_eq!(fs::read_to_string(target_full)?, "original content");
        Ok(())
    }

    #[test]
    fn test_backup_logic() -> Result<()> {
        let root = tempdir()?;
        let source_dir = tempdir()?;
        let source_file = source_dir.path().join("config");
        fs::write(&source_file, "new content")?;

        let target_rel = Path::new("config");
        let target_full = root.path().join(target_rel);
        fs::write(&target_full, "old content")?;

        let mgr = DotfileManager::new(root.path(), false);
        mgr.deploy(&source_file, target_rel, DeployStrategy::Copy)?;

        assert_eq!(fs::read_to_string(&target_full)?, "new content");

        // Check if a backup file exists
        let mut entries = fs::read_dir(root.path())?;
        let backup_exists = entries.any(|e| {
            let name = e.unwrap().file_name().to_string_lossy().into_owned();
            name.starts_with("config.bak.")
        });
        assert!(backup_exists);
        Ok(())
    }
}
