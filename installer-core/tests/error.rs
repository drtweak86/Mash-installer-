use anyhow::anyhow;
use installer_core::{ErrorSeverity, InstallerError, InstallerStateSnapshot, ProfileLevel, RunSummary, UserOptionsContext};
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
