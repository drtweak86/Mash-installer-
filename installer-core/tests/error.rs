use anyhow::anyhow;
use installer_core::cmd::{CommandExecutionDetails, CommandExecutionError};
use installer_core::model::options::{ArgonConfig, DockerConfig};
use installer_core::{
    DriverInfo, EnvironmentTag, ErrorSeverity, InstallOptions, InstallationReport, InstallerError,
    InstallerStateSnapshot, ProfileLevel, SoftwareTierPlan, UserOptionsContext,
};
use std::path::PathBuf;

fn build_user_options() -> UserOptionsContext {
    UserOptionsContext {
        profile: ProfileLevel::Dev,
        staging_dir: PathBuf::from("/tmp/mash-test"),
        dry_run: true,
        interactive: false,
        argon: ArgonConfig {
            enabled: true,
            cooling_profile: "Balanced".to_string(),
        },
        enable_p10k: false,
        docker: DockerConfig::default(),
        software_plan: SoftwareTierPlan::default(),
        system_profile: None,
        environment: EnvironmentTag::Home,
        chezmoi: Default::default(),
        desktop_environment: None,
        display_protocol: installer_core::desktop::DisplayProtocol::Auto,
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
    assert!(error.user_message().contains("STATUS: HALTED"));
    assert!(error.user_message().contains("PHASE:  PHASE-ONE"));
    assert!(error.developer_message().contains("boom"));
    assert_eq!(error.state.profile, ProfileLevel::Dev);
    assert_eq!(error.advice.as_deref(), Some("Try again"));
}

#[test]
fn installation_report_tracks_errors() {
    let options = build_user_options();
    let installer_error = InstallerError::new(
        "phase1",
        "some action",
        ErrorSeverity::Fatal,
        anyhow!("boom"),
        InstallerStateSnapshot::from_options(&options),
        None,
    );
    let report = InstallationReport {
        completed_phases: vec!["phase1".to_string()],
        staging_dir: PathBuf::from("/tmp/mash-test"),
        errors: vec![installer_error.clone()],
        outputs: Vec::new(),
        events: Vec::new(),
        options: InstallOptions::default(),
        driver: DriverInfo {
            name: "test-driver".to_string(),
            description: "A test driver".to_string(),
        },
        dry_run_log: Vec::new(),
        audit_report: installer_core::dry_run::PreflightAuditReport::default(),
    };

    assert!(report.has_errors());
    assert_eq!(report.error_count(), 1);
}

#[test]
fn installer_error_tracks_command_output() {
    let details = CommandExecutionDetails {
        command: "echo fail".into(),
        status: Some(1),
        stdout: "out".into(),
        stderr: "err".into(),
    };
    let cmd_error = CommandExecutionError::new(details.clone());
    let error = InstallerError::new(
        "phase-cmd",
        "phase command failed",
        ErrorSeverity::Fatal,
        cmd_error.into(),
        InstallerStateSnapshot::default(),
        None,
    );

    let captured = error
        .command_output()
        .expect("command output should be recorded");
    assert_eq!(captured.command, "echo fail");
    assert_eq!(captured.stdout, "out");
    assert_eq!(captured.stderr, "err");
}
