use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Central configuration persisted at ~/.config/mash-installer/config.toml
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct MashConfig {
    #[serde(default = "default_staging_dir")]
    pub staging_dir: PathBuf,

    #[serde(default)]
    pub agents: AgentDirs,

    #[serde(default)]
    pub cache: CacheDirs,

    #[serde(default)]
    pub docker: DockerConfig,

    #[serde(default)]
    pub git: GitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AgentDirs {
    #[serde(default = "default_agent_larry")]
    pub larry: PathBuf,
    #[serde(default = "default_agent_moe")]
    pub moe: PathBuf,
    #[serde(default = "default_agent_claude")]
    pub claude: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct CacheDirs {
    #[serde(default = "default_cache_installer")]
    pub installer: PathBuf,
    #[serde(default = "default_cache_gh")]
    pub gh: PathBuf,
    #[serde(default = "default_cache_cargo")]
    pub cargo: PathBuf,
    #[serde(default = "default_cache_rustup")]
    pub rustup: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct DockerConfig {
    /// Optional custom data-root for Docker daemon.
    #[serde(default)]
    pub data_root: Option<PathBuf>,
    /// Prefer docker compose plugin over standalone docker-compose.
    #[serde(default = "bool_true")]
    pub compose_plugin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct GitConfig {
    /// Enforce SSH remotes (advisory – the installer will not rewrite remotes).
    #[serde(default = "bool_true")]
    pub enforce_ssh: bool,
}

// ── defaults ────────────────────────────────────────────────────

fn bool_true() -> bool {
    true
}

fn home_dir() -> PathBuf {
    if let Some(home) = env::var_os("HOME") {
        PathBuf::from(home)
    } else {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from("/root"))
    }
}

fn default_staging_dir() -> PathBuf {
    // Prefer /mnt/data/mash-installer if /mnt/data exists, else /data/mash-installer
    // if /data exists, else /var/tmp/mash-installer.
    if Path::new("/mnt/data").is_dir() {
        PathBuf::from("/mnt/data/mash-installer")
    } else if Path::new("/data").is_dir() {
        PathBuf::from("/data/mash-installer")
    } else {
        PathBuf::from("/var/tmp/mash-installer")
    }
}

fn default_agent_larry() -> PathBuf {
    home_dir().join(".config/mash-agents/larry")
}
fn default_agent_moe() -> PathBuf {
    home_dir().join(".config/mash-agents/moe")
}
fn default_agent_claude() -> PathBuf {
    home_dir().join(".config/mash-agents/claude")
}

fn default_cache_installer() -> PathBuf {
    home_dir().join(".cache/mash-installer")
}
fn default_cache_gh() -> PathBuf {
    home_dir().join(".cache/gh")
}
fn default_cache_cargo() -> PathBuf {
    home_dir().join(".cache/cargo")
}
fn default_cache_rustup() -> PathBuf {
    home_dir().join(".cache/rustup")
}

impl Default for AgentDirs {
    fn default() -> Self {
        Self {
            larry: default_agent_larry(),
            moe: default_agent_moe(),
            claude: default_agent_claude(),
        }
    }
}

impl Default for CacheDirs {
    fn default() -> Self {
        Self {
            installer: default_cache_installer(),
            gh: default_cache_gh(),
            cargo: default_cache_cargo(),
            rustup: default_cache_rustup(),
        }
    }
}

impl Default for DockerConfig {
    fn default() -> Self {
        Self {
            data_root: None,
            compose_plugin: true,
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self { enforce_ssh: true }
    }
}

impl Default for MashConfig {
    fn default() -> Self {
        Self {
            staging_dir: default_staging_dir(),
            agents: AgentDirs::default(),
            cache: CacheDirs::default(),
            docker: DockerConfig::default(),
            git: GitConfig::default(),
        }
    }
}

// ── public API ──────────────────────────────────────────────────

fn config_path() -> PathBuf {
    home_dir().join(".config/mash-installer/config.toml")
}

/// Load config from disk, falling back to compiled defaults.
pub fn load_or_default() -> Result<MashConfig> {
    let path = config_path();
    if path.exists() {
        let text = fs::read_to_string(&path)
            .with_context(|| format!("reading config from {}", path.display()))?;
        let cfg: MashConfig = toml::from_str(&text).with_context(|| "parsing config.toml")?;
        Ok(cfg)
    } else {
        Ok(MashConfig::default())
    }
}

/// Write the default config to disk (config init).
#[allow(dead_code)]
pub fn init_config() -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if path.exists() {
        let backup = path.with_extension("toml.bak");
        fs::copy(&path, &backup)?;
        println!("Backed up existing config to {}", backup.display());
    }

    let cfg = MashConfig::default();
    let text = toml::to_string_pretty(&cfg)?;
    fs::write(&path, &text)?;
    println!("Wrote default config to {}", path.display());
    Ok(())
}

/// Print the current config (config show).
#[allow(dead_code)]
pub fn show_config() -> Result<()> {
    let cfg = load_or_default()?;
    let text = toml::to_string_pretty(&cfg)?;
    let path = config_path();
    if path.exists() {
        println!("# {}", path.display());
    } else {
        println!("# (no config file found; showing defaults)");
    }
    println!("{text}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::ffi::OsString;
    use std::path::Path;
    use tempfile::tempdir;

    struct HomeGuard(Option<OsString>);

    impl HomeGuard {
        fn set(path: impl AsRef<Path>) -> Self {
            let previous = env::var_os("HOME");
            env::set_var("HOME", path.as_ref());
            HomeGuard(previous)
        }
    }

    impl Drop for HomeGuard {
        fn drop(&mut self) {
            if let Some(prev) = &self.0 {
                env::set_var("HOME", prev);
            } else {
                env::remove_var("HOME");
            }
        }
    }

    #[test]
    fn test_load_or_default_creates_default() -> Result<()> {
        let tmp = tempdir()?;
        let _home_guard = HomeGuard::set(tmp.path());

        let cfg = load_or_default()?;
        assert_eq!(cfg, MashConfig::default());
        Ok(())
    }

    #[test]
    fn test_load_or_default_loads_existing() -> Result<()> {
        let tmp = tempdir()?;
        let _home_guard = HomeGuard::set(tmp.path());

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
