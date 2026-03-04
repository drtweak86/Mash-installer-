pub use installer_model::config::MashConfig;

use anyhow::Result as AnyResult;
use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// ── public API ──────────────────────────────────────────────────

pub fn config_path() -> PathBuf {
    if let Some(path) = env::var_os("MASH_CONFIG_PATH") {
        PathBuf::from(path)
    } else {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/root"))
            .join(".config/mash-installer/config.toml")
    }
}

/// Errors that can occur when loading Mash configuration files.
#[derive(Debug)]
pub enum ConfigError {
    /// Unable to read the config file (permissions, missing directories, etc.).
    Read { path: PathBuf, source: io::Error },
    /// Unable to parse the config file as TOML.
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
}

impl ConfigError {
    pub fn path(&self) -> &Path {
        match self {
            ConfigError::Read { path, .. } => path,
            ConfigError::Parse { path, .. } => path,
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Read { path, source } => {
                write!(f, "failed to read config at {}: {}", path.display(), source)
            }
            ConfigError::Parse { path, source } => {
                write!(
                    f,
                    "failed to parse config at {}: {}",
                    path.display(),
                    source
                )
            }
        }
    }
}

impl StdError for ConfigError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ConfigError::Read { source, .. } => Some(source),
            ConfigError::Parse { source, .. } => Some(source),
        }
    }
}

/// Load config from disk, falling back to compiled defaults.
pub fn load_or_default() -> std::result::Result<MashConfig, ConfigError> {
    let path = config_path();
    if path.exists() {
        let text = fs::read_to_string(&path).map_err(|source| ConfigError::Read {
            path: path.clone(),
            source,
        })?;
        let cfg = toml::from_str(&text).map_err(|source| ConfigError::Parse {
            path: path.clone(),
            source,
        })?;
        Ok(cfg)
    } else {
        Ok(MashConfig::default())
    }
}

/// Write the default config to disk (config init).
#[allow(dead_code)]
pub fn init_config(out: &mut dyn io::Write) -> AnyResult<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if path.exists() {
        let backup = path.with_extension("toml.bak");
        fs::copy(&path, &backup)?;
        writeln!(out, "Backed up existing config to {}", backup.display())?;
    }

    let cfg = MashConfig::default();
    let text = toml::to_string_pretty(&cfg)?;
    fs::write(&path, &text)?;
    writeln!(out, "Wrote default config to {}", path.display())?;
    Ok(())
}

/// Show the current config (config show).
#[allow(dead_code)]
pub fn show_config(out: &mut dyn io::Write) -> AnyResult<()> {
    let cfg = load_or_default()?;
    let text = toml::to_string_pretty(&cfg)?;
    let path = config_path();
    if path.exists() {
        writeln!(out, "# {}", path.display())?;
    } else {
        writeln!(out, "# (no config file found; showing defaults)")?;
    }
    writeln!(out, "{text}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};
    use std::sync::{Mutex, MutexGuard, OnceLock};
    use tempfile::tempdir;

    static CONFIG_PATH_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    struct ConfigPathGuard {
        previous: Option<OsString>,
        _lock: MutexGuard<'static, ()>,
    }

    impl ConfigPathGuard {
        fn set(path: impl AsRef<Path>) -> Self {
            let mutex = CONFIG_PATH_ENV_LOCK.get_or_init(|| Mutex::new(()));
            let lock = mutex.lock().expect("config path mutex poisoned");
            let previous = env::var_os("MASH_CONFIG_PATH");
            env::set_var("MASH_CONFIG_PATH", path.as_ref());
            ConfigPathGuard {
                previous,
                _lock: lock,
            }
        }
    }

    impl Drop for ConfigPathGuard {
        fn drop(&mut self) {
            if let Some(previous) = &self.previous {
                env::set_var("MASH_CONFIG_PATH", previous);
            } else {
                env::remove_var("MASH_CONFIG_PATH");
            }
        }
    }

    #[test]
    fn test_load_or_default_creates_default() -> AnyResult<()> {
        let tmp = tempdir()?;
        let _path_guard = ConfigPathGuard::set(tmp.path().join("config.toml"));

        let cfg = load_or_default()?;
        assert_eq!(cfg, MashConfig::default());
        Ok(())
    }

    #[test]
    fn test_load_or_default_loads_existing() -> AnyResult<()> {
        let tmp = tempdir()?;
        let _path_guard = ConfigPathGuard::set(tmp.path().join("config.toml"));

        let expected = MashConfig {
            staging_dir: PathBuf::from("/tmp/custom-staging"),
            ..Default::default()
        };

        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let text = toml::to_string_pretty(&expected)?;
        fs::write(&path, &text)?;

        let loaded = load_or_default()?;
        assert_eq!(loaded, expected);
        Ok(())
    }
}
