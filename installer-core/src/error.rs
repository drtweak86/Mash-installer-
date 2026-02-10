use crate::context::UserOptionsContext;
use crate::InstallOptions;
use crate::PhaseEvent;
use crate::ProfileLevel;
use anyhow::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorSeverity {
    Recoverable,
    Fatal,
}

#[derive(Clone, Debug)]
pub struct InstallerStateSnapshot {
    pub profile: ProfileLevel,
    pub staging_dir: PathBuf,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
}

impl InstallerStateSnapshot {
    pub fn from_options(options: &UserOptionsContext) -> Self {
        Self {
            profile: options.profile,
            staging_dir: options.staging_dir.clone(),
            dry_run: options.dry_run,
            interactive: options.interactive,
            enable_argon: options.enable_argon,
            enable_p10k: options.enable_p10k,
            docker_data_root: options.docker_data_root,
        }
    }
}

impl Default for InstallerStateSnapshot {
    fn default() -> Self {
        Self {
            profile: ProfileLevel::Minimal,
            staging_dir: PathBuf::from("<unknown>"),
            dry_run: false,
            interactive: false,
            enable_argon: false,
            enable_p10k: false,
            docker_data_root: false,
        }
    }
}

impl fmt::Display for InstallerStateSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "profile={:?}, staging={}, dry_run={}, interactive={}, enable_argon={}, enable_p10k={}, docker_data_root={}",
            self.profile,
            self.staging_dir.display(),
            self.dry_run,
            self.interactive,
            self.enable_argon,
            self.enable_p10k,
            self.docker_data_root,
        )
    }
}

#[derive(Clone, Debug)]
pub struct InstallerError {
    pub phase: String,
    pub description: String,
    pub severity: ErrorSeverity,
    pub message: String,
    pub developer_detail: String,
    pub advice: Option<String>,
    pub state: InstallerStateSnapshot,
}

impl InstallerError {
    pub fn new(
        phase: impl Into<String>,
        description: impl Into<String>,
        severity: ErrorSeverity,
        source: Error,
        state: InstallerStateSnapshot,
        advice: Option<String>,
    ) -> Self {
        let phase = phase.into();
        let description = description.into();
        let message = format!("{phase} failed: {}", source.root_cause());
        let developer_detail = format!("{source:#}");
        Self {
            phase,
            description,
            severity,
            message,
            developer_detail,
            advice,
            state,
        }
    }

    pub fn user_message(&self) -> &str {
        &self.message
    }

    pub fn developer_message(&self) -> &str {
        &self.developer_detail
    }
}

impl fmt::Display for InstallerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.phase, self.message)
    }
}

impl std::error::Error for InstallerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Clone, Debug)]
pub struct RunSummary {
    pub completed_phases: Vec<String>,
    pub staging_dir: PathBuf,
    pub errors: Vec<InstallerError>,
}

impl RunSummary {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}

impl Default for RunSummary {
    fn default() -> Self {
        Self {
            completed_phases: Vec::new(),
            staging_dir: PathBuf::from("<unknown>"),
            errors: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DriverInfo {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct InstallationReport {
    pub summary: RunSummary,
    pub events: Vec<PhaseEvent>,
    pub options: InstallOptions,
    pub driver: DriverInfo,
}

#[derive(Debug)]
pub struct InstallerRunError {
    pub report: InstallationReport,
    pub source: InstallerError,
}

impl fmt::Display for InstallerRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source.message)
    }
}

impl std::error::Error for InstallerRunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl From<Error> for InstallerRunError {
    fn from(source: Error) -> Self {
        let installer_error = InstallerError::new(
            "setup",
            "setup stage",
            ErrorSeverity::Fatal,
            source,
            InstallerStateSnapshot::default(),
            Some("Check logs for the full failure report.".to_string()),
        );

        InstallerRunError {
            report: InstallationReport {
                summary: RunSummary {
                    completed_phases: Vec::new(),
                    staging_dir: PathBuf::from("<unknown>"),
                    errors: vec![installer_error.clone()],
                },
                events: Vec::new(),
                options: InstallOptions::default(),
                driver: DriverInfo {
                    name: "<unknown>".to_string(),
                    description: "unknown driver".to_string(),
                },
            },
            source: installer_error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::UserOptionsContext;
    use anyhow::anyhow;
    use std::path::PathBuf;

    fn build_user_options() -> UserOptionsContext {
        UserOptionsContext {
            profile: ProfileLevel::Dev,
            staging_dir: PathBuf::from("/tmp/mash-test"),
            dry_run: true,
            interactive: false,
            enable_argon: true,
            enable_p10k: false,
            docker_data_root: false,
        }
    }

    #[test]
    fn installer_error_exposes_user_and_developer_messages() {
        let options = build_user_options();
        let error = InstallerError::new(
            "phase-one",
            "phase one description",
            ErrorSeverity::Recoverable,
            anyhow!("boom"),
            InstallerStateSnapshot::from_options(&options),
            Some("Try again".to_string()),
        );
        assert_eq!(error.phase, "phase-one");
        assert!(error.user_message().contains("phase-one failed"));
        assert!(error.developer_message().contains("boom"));
        assert_eq!(error.state.profile, ProfileLevel::Dev);
        assert_eq!(error.advice.as_deref(), Some("Try again"));
    }

    #[test]
    fn run_summary_reports_errors() {
        let options = build_user_options();
        let error = InstallerError::new(
            "phase-one",
            "phase one description",
            ErrorSeverity::Recoverable,
            anyhow!("boom"),
            InstallerStateSnapshot::from_options(&options),
            None,
        );
        let summary = RunSummary {
            completed_phases: vec!["phase-one".to_string()],
            staging_dir: PathBuf::from("/tmp/staging"),
            errors: vec![error],
        };
        assert!(summary.has_errors());
        assert_eq!(summary.error_count(), 1);
    }
}
