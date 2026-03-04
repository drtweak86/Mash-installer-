use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhaseOutput {
    pub name: String,
    pub description: String,
    pub actions_taken: Vec<String>,
    pub configured_actions: Vec<String>,
    pub tweaked_actions: Vec<String>,
    pub rollback_actions: Vec<String>,
    pub warnings: Vec<String>,
    pub dry_run: bool,
    pub status: PhaseStatus,
}

impl PhaseOutput {
    pub fn from_metadata(
        name: impl Into<String>,
        description: impl Into<String>,
        metadata: PhaseMetadata,
        status: PhaseStatus,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            actions_taken: metadata.actions_taken,
            configured_actions: metadata.configured_actions,
            tweaked_actions: metadata.tweaked_actions,
            rollback_actions: metadata.rollback_actions,
            warnings: metadata.warnings,
            dry_run: metadata.dry_run,
            status,
        }
    }

    pub fn skipped(name: impl Into<String>, description: impl Into<String>, dry_run: bool) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            actions_taken: Vec::new(),
            configured_actions: Vec::new(),
            tweaked_actions: Vec::new(),
            rollback_actions: Vec::new(),
            warnings: Vec::new(),
            dry_run,
            status: PhaseStatus::Skipped,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PhaseStatus {
    Completed,
    PartialSuccess(String),
    RecoverableFailure(String),
    Failed(String),
    Skipped,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PhaseEvent {
    Total {
        total: usize,
    },
    Started {
        index: usize,
        total: usize,
        phase: String,
    },
    Completed {
        index: usize,
        phase: String,
        description: String,
    },
    Failed {
        index: usize,
        phase: String,
        error: String,
    },
    Skipped {
        index: usize,
        phase: String,
    },
    Warning {
        message: String,
    },
}

/// Collected metadata that each phase can report to the runner.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PhaseMetadata {
    pub actions_taken: Vec<String>,
    pub configured_actions: Vec<String>,
    pub tweaked_actions: Vec<String>,
    pub rollback_actions: Vec<String>,
    pub warnings: Vec<String>,
    pub dry_run: bool,
}

/// Types of interactive authorizations supported by the forge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthType {
    /// GitHub CLI login (`gh auth login`)
    GitHubCli,
    /// SSH key generation (`ssh-keygen`)
    SshKey,
    /// Global Git configuration (`git config --global`)
    GitConfig,
    /// Rclone configuration (`rclone config`)
    RcloneConfig,
    /// Borg repository initialization (`borg init`)
    BorgSetup,
    /// Tailscale node authorization (`tailscale up`)
    TailscaleAuth,
    /// Ngrok authtoken configuration
    NgrokAuth,
    /// Cloudflared tunnel login
    CloudflaredAuth,
    /// Docker registry login
    DockerAuth,
    /// Argon One fan curve configuration
    ArgonOneConfig,
}

/// Observer trait for tracking phase progress and interacting with the user.
pub trait PhaseObserver {
    fn on_event(&mut self, _event: PhaseEvent) {}

    /// Ask the user for confirmation. Returns `true` to proceed, `false` to abort.
    /// Default implementation always proceeds.
    fn confirm(&mut self, _prompt: &str) -> bool {
        true
    }

    /// Ask the user for a sudo password.
    fn sudo_password(&mut self) -> anyhow::Result<String> {
        Ok(String::new())
    }

    /// Ask the user for interactive authorization.
    fn request_auth(&mut self, _auth_type: AuthType) -> anyhow::Result<bool> {
        Ok(false)
    }
}
