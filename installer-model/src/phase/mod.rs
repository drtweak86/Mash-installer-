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
