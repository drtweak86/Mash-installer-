use crate::cmd;
use crate::context::UserOptionsContext;
use crate::dry_run::DryRunEntry;
use crate::ConfigError;
use crate::ProfileLevel;
use crate::{InstallOptions, SoftwareTierPlan};
use crate::{PhaseEvent, PhaseOutput};
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
    pub software_plan: SoftwareTierPlan,
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
            software_plan: options.software_plan.clone(),
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
            software_plan: SoftwareTierPlan::default(),
        }
    }
}

impl fmt::Display for InstallerStateSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "profile={:?}, staging={}, dry_run={}, interactive={}, enable_argon={}, enable_p10k={}, docker_data_root={}, software_plan(full={}, theme={:?}, selections={})",
            self.profile,
            self.staging_dir.display(),
            self.dry_run,
            self.interactive,
            self.enable_argon,
            self.enable_p10k,
            self.docker_data_root,
            self.software_plan.full_install,
            self.software_plan.theme_plan,
            self.software_plan.selections.len(),
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
    pub command_output: Option<cmd::CommandExecutionDetails>,
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
        let cause = source.root_cause().to_string().to_uppercase();
        let fix = advice.as_deref().unwrap_or("CHECK_LOGS_FOR_DETAILS").to_uppercase();
        
        let message = format!(
            "STATUS: HALTED\nPHASE:  {}\nERROR:  {}\nLOG:    ~/mash-install.log\nFIX:    {}",
            phase.to_uppercase(),
            cause,
            fix
        );
        let developer_detail = format!("{source:#}");
        let command_output = source
            .downcast_ref::<cmd::CommandExecutionError>()
            .map(|err| err.details().clone());
        Self {
            phase,
            description,
            severity,
            message,
            developer_detail,
            advice,
            state,
            command_output,
        }
    }

    pub fn user_message(&self) -> &str {
        &self.message
    }

    pub fn developer_message(&self) -> &str {
        &self.developer_detail
    }

    pub fn command_output(&self) -> Option<&cmd::CommandExecutionDetails> {
        self.command_output.as_ref()
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
pub struct DriverInfo {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct InstallationReport {
    pub completed_phases: Vec<String>,
    pub staging_dir: PathBuf,
    pub errors: Vec<InstallerError>,
    pub outputs: Vec<PhaseOutput>,
    pub events: Vec<PhaseEvent>,
    pub options: InstallOptions,
    pub driver: DriverInfo,
    pub dry_run_log: Vec<DryRunEntry>,
}

impl InstallationReport {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}

#[derive(Debug)]
pub struct InstallerRunError {
    pub report: Box<InstallationReport>,
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
            report: Box::new(InstallationReport {
                completed_phases: Vec::new(),
                staging_dir: PathBuf::from("<unknown>"),
                errors: vec![installer_error.clone()],
                outputs: Vec::new(),
                events: Vec::new(),
                options: InstallOptions::default(),
                driver: DriverInfo {
                    name: "<unknown>".to_string(),
                    description: "unknown driver".to_string(),
                },
                dry_run_log: Vec::new(),
            }),
            source: installer_error,
        }
    }
}

impl From<Error> for Box<InstallerRunError> {
    fn from(source: Error) -> Self {
        Box::new(InstallerRunError::from(source))
    }
}

impl From<ConfigError> for Box<InstallerRunError> {
    fn from(source: ConfigError) -> Self {
        let installer_error = InstallerError::new(
            "config",
            "configuration load",
            ErrorSeverity::Fatal,
            Error::from(source),
            InstallerStateSnapshot::default(),
            Some("Inspect ~/.config/mash-installer/config.toml for corruption or permissions issues.".to_string()),
        );

        let report = InstallationReport {
            completed_phases: Vec::new(),
            staging_dir: PathBuf::from("<unknown>"),
            errors: vec![installer_error.clone()],
            outputs: Vec::new(),
            events: Vec::new(),
            options: InstallOptions::default(),
            driver: DriverInfo {
                name: "<unknown>".to_string(),
                description: "unknown driver".to_string(),
            },
            dry_run_log: Vec::new(),
        };

        Box::new(InstallerRunError {
            report: Box::new(report),
            source: installer_error,
        })
    }
}
