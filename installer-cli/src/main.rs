use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use installer_core::cmd::CommandExecutionDetails;
use installer_core::{
    detect_platform, init_logging, interaction::InteractionService, ConfigService, DistroDriver,
    InstallOptions, InstallationReport, ProfileLevel, SoftwareTierPlan,
};
use std::io::{self, Write};
use std::path::PathBuf;
use tracing::info;

mod catalog;
mod menu;
mod software_tiers;
mod ui;

#[derive(Parser)]
#[command(
    name = "installer-cli",
    about = "Workspace-aware mash installer entrypoint"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<CliCommand>,

    #[arg(long)]
    staging_dir: Option<PathBuf>,

    #[arg(long)]
    dry_run: bool,

    #[arg(long)]
    non_interactive: bool,

    #[arg(long, short)]
    verbose: bool,

    #[arg(long)]
    continue_on_error: bool,

    /// Profile to install: minimal, dev, full  (skips the profile menu)
    #[arg(long, value_name = "LEVEL")]
    profile: Option<String>,
}

#[derive(Subcommand)]
enum CliCommand {
    Catalog {
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(CliCommand::Catalog { json }) = cli.command {
        let catalog = installer_core::catalog::curated_catalog();
        return catalog::print_catalog(&catalog, json);
    }

    let config_service = ConfigService::load()?;
    init_logging(&config_service.config().logging, cli.verbose)?;
    let platform_info = detect_platform().context("detecting host platform")?;

    // Build list of available drivers based on compile-time features
    let drivers: Vec<&'static dyn DistroDriver> = vec![
        #[cfg(feature = "arch")]
        installer_arch::driver(),
        #[cfg(feature = "debian")]
        installer_debian::driver(),
        #[cfg(feature = "fedora")]
        installer_fedora::driver(),
    ];

    if drivers.is_empty() {
        anyhow::bail!(
            "No distro drivers available! Recompile with at least one feature: arch, debian, or fedora"
        );
    }
    let interaction_config = config_service.config().interaction.clone();
    let interaction = InteractionService::new(!cli.non_interactive, interaction_config);
    let driver = if cli.non_interactive {
        menu::auto_detect_driver(&drivers, &platform_info).unwrap_or_else(|| drivers[0])
    } else {
        menu::run_driver_selection(&drivers, &platform_info, &interaction)?
    };

    let modules = if cli.non_interactive {
        menu::ModuleSelection::default()
    } else {
        menu::run_module_menu(driver.name(), &interaction)?
    };

    let software_plan = if cli.non_interactive {
        SoftwareTierPlan::default()
    } else {
        software_tiers::run_software_tier_menu(&interaction)?
    };

    let profile = if let Some(ref p) = cli.profile {
        parse_profile_level(p)?
    } else if cli.non_interactive {
        ProfileLevel::Dev
    } else {
        menu::run_profile_menu(&interaction)?
    };

    let options = InstallOptions {
        profile,
        staging_dir: cli.staging_dir,
        dry_run: cli.dry_run,
        interactive: !cli.non_interactive,
        enable_argon: modules.enable_argon,
        enable_p10k: modules.enable_p10k,
        docker_data_root: modules.docker_data_root,
        continue_on_error: cli.continue_on_error,
        software_plan,
    };

    info!(
        "Selected driver: {} ({}). Profile: {:?}. Modules: {:?}",
        driver.name(),
        driver.description(),
        profile,
        modules
    );

    let mut observer = ui::CliPhaseObserver::new();
    run_installer_with_ui(driver, options, &mut observer).context("installer failed")
}

fn parse_profile_level(s: &str) -> Result<ProfileLevel> {
    match s.to_lowercase().as_str() {
        "minimal" | "min" => Ok(ProfileLevel::Minimal),
        "dev" => Ok(ProfileLevel::Dev),
        "full" => Ok(ProfileLevel::Full),
        other => anyhow::bail!("unknown profile '{other}'; valid values: minimal, dev, full"),
    }
}

fn print_completion_message(report: &InstallationReport, dry_run: bool) {
    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       Installation complete!                  ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    if dry_run {
        println!("(dry-run mode – no changes were made)");
        println!();
        print_dry_run_summary(report);
    }

    println!("Post-install notes:");
    println!("  - Log out and back in for docker group membership to take effect.");
    println!("  - Run `mash-setup doctor` to verify everything.");
    println!("  - Config lives at ~/.config/mash-installer/config.toml");
    println!("  - Staging directory: {}", report.staging_dir.display());
    println!();
}

fn print_dry_run_summary(report: &InstallationReport) {
    println!("──── Dry-run summary ────────────────────────────");
    if report.dry_run_log.is_empty() {
        println!("  No dry-run actions were recorded.");
    } else {
        for (idx, entry) in report.dry_run_log.iter().enumerate() {
            println!("  {}. [{}] {}", idx + 1, entry.phase, entry.action);
            if let Some(detail) = &entry.detail {
                println!("     {detail}");
            }
        }
    }
    println!("  No resources were modified during dry run.");
    println!("───────────────────────────────────────────────");
    println!();
}

fn print_error_report(report: &InstallationReport) {
    let mut stdout = io::stdout();
    let _ = write_error_report(report, &mut stdout);
}

fn write_error_report(report: &InstallationReport, out: &mut dyn Write) -> std::io::Result<()> {
    if report.errors.is_empty() {
        writeln!(out, "No additional error details were recorded.")?;
        return Ok(());
    }

    let completed = if report.completed_phases.is_empty() {
        "none".to_string()
    } else {
        report.completed_phases.join(", ")
    };

    writeln!(out)?;
    writeln!(out, "╔══════════════════════════════════════════════╗")?;
    writeln!(out, "║       Installation completed with errors     ║")?;
    writeln!(out, "╚══════════════════════════════════════════════╝")?;
    writeln!(out)?;
    writeln!(out, "Completed phases: {}", completed)?;
    writeln!(out, "Staging directory: {}", report.staging_dir.display())?;
    writeln!(out)?;

    for err in &report.errors {
        writeln!(out, "  • Phase: {} – {}", err.phase, err.user_message())?;
        if let Some(advice) = &err.advice {
            writeln!(out, "    Advice: {}", advice)?;
        }
        writeln!(out, "    Context: {}", err.state)?;
        writeln!(out, "    Details: {}", err.developer_message())?;
        if let Some(details) = err.command_output() {
            write_command_output(out, details)?;
        }
        writeln!(out)?;
    }

    Ok(())
}

fn write_command_output(
    out: &mut dyn Write,
    details: &CommandExecutionDetails,
) -> std::io::Result<()> {
    writeln!(out, "    Command: {}", details.command)?;
    match details.status {
        Some(code) => writeln!(out, "    Exit status: {code}")?,
        None => writeln!(out, "    Exit status: unknown")?,
    }
    write_multiline(out, "stdout", &details.stdout)?;
    write_multiline(out, "stderr", &details.stderr)
}

fn write_multiline(out: &mut dyn Write, label: &str, text: &str) -> std::io::Result<()> {
    if text.trim().is_empty() {
        return Ok(());
    }
    writeln!(out, "    {label}:")?;
    for line in text.trim_end_matches('\n').lines() {
        writeln!(out, "      {line}")?;
    }
    Ok(())
}

fn run_installer_with_ui(
    driver: &'static dyn DistroDriver,
    options: InstallOptions,
    observer: &mut ui::CliPhaseObserver,
) -> Result<()> {
    ui::print_banner();
    let dry_run = options.dry_run;
    let run_result = installer_core::run_with_driver(driver, options, observer);
    observer.finish();

    match run_result {
        Ok(report) => {
            print_completion_message(&report, dry_run);
            Ok(())
        }
        Err(err) => {
            print_error_report(&err.report);
            Err(err.into())
        }
    }
}

#[cfg(test)]
mod error_report_tests {
    use super::*;
    use anyhow::anyhow;
    use installer_core::{
        cmd::CommandExecutionDetails, DriverInfo, ErrorSeverity, InstallOptions, InstallerError,
        InstallerStateSnapshot,
    };
    use std::path::PathBuf;

    fn make_report_with_error() -> InstallationReport {
        let mut error = InstallerError::new(
            "phase-one",
            "phase one failed",
            ErrorSeverity::Recoverable,
            anyhow!("boom"),
            InstallerStateSnapshot::default(),
            Some("Check connectivity".to_string()),
        );
        error.command_output = Some(CommandExecutionDetails {
            command: "echo fail".into(),
            status: Some(1),
            stdout: "out".into(),
            stderr: "err".into(),
        });

        InstallationReport {
            completed_phases: vec!["phase-one".to_string()],
            staging_dir: PathBuf::from("/tmp/staging"),
            errors: vec![error],
            outputs: Vec::new(),
            events: Vec::new(),
            options: InstallOptions::default(),
            driver: DriverInfo {
                name: "test".into(),
                description: "test driver".into(),
            },
            dry_run_log: Vec::new(),
        }
    }

    #[test]
    fn write_error_report_prints_phase_and_advice() {
        let report = make_report_with_error();
        let mut buf = Vec::new();
        write_error_report(&report, &mut buf).expect("write failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        assert!(output.contains("Phase: phase-one"));
        assert!(output.contains("Advice: Check connectivity"));
        assert!(output.contains("Context: profile=Minimal"));
        assert!(output.contains("Command: echo fail"));
    }

    #[test]
    fn write_error_report_outputs_no_errors_message() {
        let report = InstallationReport {
            completed_phases: Vec::new(),
            staging_dir: PathBuf::from("/tmp/staging"),
            errors: Vec::new(),
            outputs: Vec::new(),
            events: Vec::new(),
            options: InstallOptions::default(),
            driver: DriverInfo {
                name: "test".into(),
                description: "test driver".into(),
            },
            dry_run_log: Vec::new(),
        };
        let mut buf = Vec::new();
        write_error_report(&report, &mut buf).expect("write failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        assert!(output.contains("No additional error details were recorded."));
    }
}
