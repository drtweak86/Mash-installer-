use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

// ── Interaction Config ───────────────────────────────────────────

/// Configuration-driven defaults for interaction points.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(default)]
pub struct InteractionConfig {
    #[serde(default)]
    pub confirm_defaults: HashMap<String, bool>,
    #[serde(default)]
    pub text_defaults: HashMap<String, String>,
    #[serde(default)]
    pub selection_defaults: HashMap<String, usize>,
}

// ── Logging Config ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Human,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default)]
    pub format: LogFormat,
    #[serde(default)]
    pub file: Option<PathBuf>,
}

fn default_log_level() -> String {
    "info".into()
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: LogFormat::default(),
            file: None,
        }
    }
}

// ── Main Mash Config ────────────────────────────────────────────

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

    #[serde(default)]
    pub interaction: InteractionConfig,

    #[serde(default)]
    pub logging: LoggingConfig,
}

impl crate::Validator for MashConfig {
    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if !self.staging_dir.is_absolute() {
            errors.push(format!(
                "staging_dir must be absolute: {}",
                self.staging_dir.display()
            ));
        }

        // Validate Agent Dirs
        if !self.agents.larry.is_absolute() {
            errors.push(format!(
                "agents.larry must be absolute: {}",
                self.agents.larry.display()
            ));
        }
        if !self.agents.moe.is_absolute() {
            errors.push(format!(
                "agents.moe must be absolute: {}",
                self.agents.moe.display()
            ));
        }
        if !self.agents.claude.is_absolute() {
            errors.push(format!(
                "agents.claude must be absolute: {}",
                self.agents.claude.display()
            ));
        }

        // Validate Cache Dirs
        if !self.cache.installer.is_absolute() {
            errors.push(format!(
                "cache.installer must be absolute: {}",
                self.cache.installer.display()
            ));
        }
        if !self.cache.gh.is_absolute() {
            errors.push(format!(
                "cache.gh must be absolute: {}",
                self.cache.gh.display()
            ));
        }
        if !self.cache.cargo.is_absolute() {
            errors.push(format!(
                "cache.cargo must be absolute: {}",
                self.cache.cargo.display()
            ));
        }
        if !self.cache.rustup.is_absolute() {
            errors.push(format!(
                "cache.rustup must be absolute: {}",
                self.cache.rustup.display()
            ));
        }

        // Docker data-root must be absolute if specified
        if let Some(ref path) = self.docker.data_root {
            if !path.is_absolute() {
                errors.push(format!(
                    "docker.data_root must be absolute: {}",
                    path.display()
                ));
            }
        }

        errors
    }
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
            interaction: InteractionConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}
