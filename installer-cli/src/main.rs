use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use installer_core::cmd::CommandExecutionDetails;
use installer_core::{
    ConfigService,
    PhaseEvent,
    PhaseObserver,
    PlatformInfo,
    ProfileLevel,
    RunSummary,
    detect_platform,
    init_logging,
    DistroDriver,
    ErrorSeverity,
    InstallOptions,
    InstallerError,
    InstallerStateSnapshot,
    interaction::InteractionService,
};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{info, warn};

mod catalog;

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
    let drivers = vec![
        installer_arch::driver(),
        installer_debian::driver(),
        installer_fedora::driver(),
    ];
    let interaction_config = config_service.config().interaction.clone();
    let interaction = InteractionService::new(!cli.non_interactive, interaction_config);
    let driver = if cli.non_interactive {
        auto_detect_driver(&drivers, &platform_info).unwrap_or_else(|| drivers[0])
    } else {
        run_driver_selection(&drivers, &platform_info, &interaction)?
    };

    let modules = if cli.non_interactive {
        ModuleSelection::default()
    } else {
        run_module_menu(driver.name(), &interaction)?
    };

    let profile = if cli.non_interactive {
        ProfileLevel::Dev
    } else {
        run_profile_menu(&interaction)?
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
    };

    info!(
        "Selected driver: {} ({}). Profile: {:?}. Modules: {:?}",
        driver.name(),
        driver.description(),
        profile,
        modules
    );

    let mut observer = CliPhaseObserver::new();
    run_installer_with_ui(driver, options, &mut observer).context("installer failed")
}

fn print_banner() {
    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       mash-setup · mega installer            ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();
}

fn print_completion_message(summary: &RunSummary, dry_run: bool) {
    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       Installation complete!                  ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    if dry_run {
        println!("(dry-run mode – no changes were made)");
        println!();
    }

    println!("Post-install notes:");
    println!("  - Log out and back in for docker group membership to take effect.");
    println!("  - Run `mash-setup doctor` to verify everything.");
    println!("  - Config lives at ~/.config/mash-installer/config.toml");
    println!("  - Staging directory: {}", summary.staging_dir.display());
    println!();
}

fn print_error_report(summary: &RunSummary) {
    let mut stdout = io::stdout();
    let _ = write_error_report(summary, &mut stdout);
}

fn write_error_report(summary: &RunSummary, out: &mut dyn Write) -> std::io::Result<()> {
    if summary.errors.is_empty() {
        writeln!(out, "No additional error details were recorded.")?;
        return Ok(());
    }

    let completed = if summary.completed_phases.is_empty() {
        "none".to_string()
    } else {
        summary.completed_phases.join(", ")
    };

    writeln!(out)?;
    writeln!(out, "╔══════════════════════════════════════════════╗")?;
    writeln!(out, "║       Installation completed with errors     ║")?;
    writeln!(out, "╚══════════════════════════════════════════════╝")?;
    writeln!(out)?;
    writeln!(out, "Completed phases: {}", completed)?;
    writeln!(out, "Staging directory: {}", summary.staging_dir.display())?;
    writeln!(out)?;

    for err in &summary.errors {
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

#[cfg(test)]
mod error_report_tests {
    use super::*;
    use anyhow::anyhow;
    use std::path::PathBuf;

    fn make_summary_with_error() -> RunSummary {
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
        RunSummary {
            completed_phases: vec!["phase-one".to_string()],
            staging_dir: PathBuf::from("/tmp/staging"),
            errors: vec![error],
        }
    }

    #[test]
    fn write_error_report_prints_phase_and_advice() {
        let summary = make_summary_with_error();
        let mut buf = Vec::new();
        write_error_report(&summary, &mut buf).expect("write failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        assert!(output.contains("Phase: phase-one"));
        assert!(output.contains("Advice: Check connectivity"));
        assert!(output.contains("Context: profile=Minimal"));
        assert!(output.contains("Command: echo fail"));
    }

    #[test]
    fn write_error_report_outputs_no_errors_message() {
        let summary = RunSummary::default();
        let mut buf = Vec::new();
        write_error_report(&summary, &mut buf).expect("write failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        assert!(output.contains("No additional error details were recorded."));
    }
}

fn run_installer_with_ui(
    driver: &'static dyn DistroDriver,
    options: InstallOptions,
    observer: &mut CliPhaseObserver,
) -> Result<()> {
    print_banner();
    let dry_run = options.dry_run;
    let run_result = installer_core::run_with_driver(driver, options, observer);
    observer.finish();

    match run_result {
        Ok(report) => {
            print_completion_message(&report.summary, dry_run);
            Ok(())
        }
        Err(err) => {
            print_error_report(&err.report.summary);
            Err(err.into())
        }
    }
}

struct CliPhaseObserver {
    mp: MultiProgress,
    overall: ProgressBar,
    spinner: Option<ProgressBar>,
}

impl CliPhaseObserver {
    fn new() -> Self {
        let mp = MultiProgress::new();
        let overall = mp.add(ProgressBar::new(0));
        overall.set_style(
            ProgressStyle::with_template(
                "{spinner:.cyan} [{bar:30.green/dim}] {pos}/{len} phases  {percent}%  ETA {eta}  [{elapsed}]",
            )
            .unwrap()
            .progress_chars("━╸─")
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
        );
        overall.enable_steady_tick(Duration::from_millis(200));

        Self {
            mp,
            overall,
            spinner: None,
        }
    }

    fn finish_spinner(&mut self, prefix: &'static str, msg: &str) {
        if let Some(pb) = self.spinner.take() {
            pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
            pb.set_prefix(prefix);
            pb.finish_with_message(msg.to_string());
        }
    }

    fn start_spinner(&mut self, msg: &str) {
        self.spinner = Some(
            self.mp
                .insert_before(&self.overall, ProgressBar::new_spinner()),
        );
        if let Some(pb) = &self.spinner {
            pb.set_style(
                ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg}")
                    .unwrap()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
            );
            pb.set_message(msg.to_string());
            pb.enable_steady_tick(Duration::from_millis(120));
        }
    }
}

impl PhaseObserver for CliPhaseObserver {
    fn on_event(&mut self, event: PhaseEvent) {
        match event {
            PhaseEvent::Total { total } => self.overall.set_length(total as u64),
            PhaseEvent::Started {
                index,
                total,
                phase,
            } => {
                self.finish_spinner(" ", "");
                let display = format!("Phase {}/{} · {}", index, total, phase);
                self.start_spinner(&display);
            }
            PhaseEvent::Completed { description, .. } => {
                self.finish_spinner("✓", &description);
                self.overall.inc(1);
            }
            PhaseEvent::Failed { error, .. } => {
                let message = format!("Phase FAILED: {error}");
                self.finish_spinner("✗", &message);
                self.overall.inc(1);
            }
            PhaseEvent::Skipped { phase, .. } => {
                self.finish_spinner("–", &phase);
                self.overall.inc(1);
            }
        }
    }
}

impl CliPhaseObserver {
    fn finish(&mut self) {
        self.finish_spinner(" ", "");
        self.overall.finish_and_clear();
    }
}

fn auto_detect_driver(
    drivers: &[&'static dyn DistroDriver],
    platform: &PlatformInfo,
) -> Option<&'static dyn DistroDriver> {
    drivers.iter().copied().find(|d| d.matches(platform))
}

fn run_driver_selection(
    drivers: &[&'static dyn DistroDriver],
    platform: &PlatformInfo,
    interaction: &InteractionService,
) -> Result<&'static dyn DistroDriver> {
    println!("Step 1/3: Distro selection");
    println!("1) Auto detect (default)");
    println!("2) Select distribution manually");

    let choice = interaction.select_option(
        "driver.selection.mode",
        "Choose distro mode",
        &["Auto detect (default)", "Select distribution manually"],
        1,
        |prompt, options| {
            for (idx, option) in options.iter().enumerate() {
                println!("{}) {}", idx + 1, option);
            }
            let selection = prompt_choice(prompt, 1, options.len());
            Ok(selection)
        },
    )?;

    if choice == 1 {
        if let Some(driver) = auto_detect_driver(drivers, platform) {
            println!("Auto-detected driver: {}", driver.name());
            return Ok(driver);
        }
        warn!("Auto-detection failed; falling back to manual selection.");
    }

    select_driver_from_list(drivers, interaction)
}

fn select_driver_from_list(
    drivers: &[&'static dyn DistroDriver],
    interaction: &InteractionService,
) -> Result<&'static dyn DistroDriver> {
    println!("Available distro drivers:");
    for (idx, driver) in drivers.iter().enumerate() {
        println!(
            "  {}) {} – {}",
            idx + 1,
            driver.name(),
            driver.description()
        );
    }
    let descriptions: Vec<String> = drivers
        .iter()
        .map(|driver| format!("{} – {}", driver.name(), driver.description()))
        .collect();
    let options: Vec<&str> = descriptions.iter().map(|desc| desc.as_str()).collect();
    let index = interaction.select_option(
        "driver.selection.manual",
        "Pick a driver",
        &options,
        1,
        |prompt, options| {
            for (idx, option) in options.iter().enumerate() {
                println!(" {}: {}", idx + 1, option);
            }
            let selection = prompt_choice(prompt, 1, options.len());
            Ok(selection)
        },
    )?;
    Ok(drivers.get(index - 1).copied().unwrap_or(drivers[0]))
}

#[derive(Debug, Default)]
struct ModuleSelection {
    enable_argon: bool,
    enable_p10k: bool,
    docker_data_root: bool,
}

impl ModuleSelection {
    fn full() -> Self {
        Self {
            enable_argon: true,
            enable_p10k: true,
            docker_data_root: true,
        }
    }

    fn apply_alias(&mut self, alias: &str, enabled: bool) -> bool {
        for opt in MODULE_OPTIONS {
            if opt.alias.eq_ignore_ascii_case(alias) {
                (opt.setter)(self, enabled);
                return true;
            }
        }
        false
    }
}

struct ModuleOption {
    alias: &'static str,
    label: &'static str,
    description: &'static str,
    default: bool,
    setter: fn(&mut ModuleSelection, bool),
}

const MODULE_OPTIONS: &[ModuleOption] = &[
    ModuleOption {
        alias: "A",
        label: "Argon One fan control",
        description: "Install Argon One scripts (Pi only)",
        default: false,
        setter: set_argon,
    },
    ModuleOption {
        alias: "P",
        label: "Powerlevel10k prompt",
        description: "Enable p10k + zsh polish modules",
        default: true,
        setter: set_p10k,
    },
    ModuleOption {
        alias: "D",
        label: "Docker data-root",
        description: "Manage Docker data-root inside staging",
        default: false,
        setter: set_docker_data_root,
    },
];

fn set_argon(selection: &mut ModuleSelection, value: bool) {
    selection.enable_argon = value;
}

fn set_p10k(selection: &mut ModuleSelection, value: bool) {
    selection.enable_p10k = value;
}

fn set_docker_data_root(selection: &mut ModuleSelection, value: bool) {
    selection.docker_data_root = value;
}

fn run_module_menu(driver_name: &str, interaction: &InteractionService) -> Result<ModuleSelection> {
    println!("\nStep 2/3: Modules for {}", driver_name);
    println!("Available module selection modes:");
    let modes = [
        "Full install (default – all modules enabled)",
        "Select modules",
    ];
    let choice = interaction.select_option(
        "modules.selection.mode",
        "Choose install mode",
        &modes,
        1,
        |prompt, options| {
            for (idx, option) in options.iter().enumerate() {
                println!("{}) {}", idx + 1, option);
            }
            let selection = prompt_choice(prompt, 1, options.len());
            Ok(selection)
        },
    )?;

    if choice == 1 {
        Ok(ModuleSelection::full())
    } else {
        println!("Available module toggles (use aliases to remember choices):");
        for opt in MODULE_OPTIONS {
            println!("  [{}] {} – {}", opt.alias, opt.label, opt.description);
        }
        let mut selection = ModuleSelection::default();
        for opt in MODULE_OPTIONS {
            let prompt = format!("Enable {} (alias {})?", opt.label, opt.alias);
            let enabled = interaction.confirm(
                &format!("module.{}.enable", opt.alias),
                &prompt,
                opt.default,
                || Ok(prompt_yes_no(&prompt, opt.default)),
            )?;
            selection.apply_alias(opt.alias, enabled);
        }
        Ok(selection)
    }
}

fn run_profile_menu(interaction: &InteractionService) -> Result<ProfileLevel> {
    println!("\nStep 3/3: Choose profile");
    let options = [
        "basics – minimal tooling",
        "basics-dev – add developer packages",
        "basics+QoL – dev + shell polish",
        "full modular – everything (default)",
    ];
    let choice = interaction.select_option(
        "profile.selection",
        "Pick a profile",
        &options,
        4,
        |prompt, options| {
            for (idx, option) in options.iter().enumerate() {
                println!("{}) {}", idx + 1, option);
            }
            let selection = prompt_choice(prompt, 4, options.len());
            Ok(selection)
        },
    )?;

    Ok(match choice {
        1 => ProfileLevel::Minimal,
        2 => ProfileLevel::Dev,
        3 => ProfileLevel::Dev,
        _ => ProfileLevel::Full,
    })
}

fn prompt_choice(prompt: &str, default: usize, max_choice: usize) -> usize {
    print!("{} [{}]: ", prompt, default);
    io::stdout().flush().ok();
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return default;
    }
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return default;
    }
    if let Ok(idx) = trimmed.parse::<usize>() {
        if idx > 0 && idx <= max_choice {
            return idx;
        }
    }
    println!("Invalid choice, defaulting to {}", default);
    default
}

fn prompt_yes_no(prompt: &str, default: bool) -> bool {
    let default_marker = if default { "Y/n" } else { "y/N" };
    loop {
        print!("{prompt} [{default_marker}]: ");
        io::stdout().flush().ok();
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            return default;
        }
        match line.trim().to_lowercase().as_str() {
            "" => return default,
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please answer y or n."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_selection_alias_applies() {
        let mut selection = ModuleSelection::default();
        assert!(!selection.enable_argon);
        assert!(selection.apply_alias("A", true));
        assert!(selection.enable_argon);
        assert!(!selection.apply_alias("invalid", true));
    }

    #[test]
    fn module_selection_full_flag() {
        let selection = ModuleSelection::full();
        assert!(selection.enable_argon);
        assert!(selection.enable_p10k);
        assert!(selection.docker_data_root);
    }

    #[test]
    fn module_selection_defaults_off() {
        let selection = ModuleSelection::default();
        assert!(!selection.enable_argon);
        assert!(!selection.enable_p10k);
        assert!(!selection.docker_data_root);
    }

    #[test]
    fn module_selection_allows_disabling_alias() {
        let mut selection = ModuleSelection::full();
        assert!(selection.enable_p10k);
        assert!(selection.apply_alias("P", false));
        assert!(!selection.enable_p10k);
    }

    #[test]
    fn module_selection_aliases_are_case_insensitive() {
        let mut selection = ModuleSelection::default();
        assert!(selection.apply_alias("p", true));
        assert!(selection.enable_p10k);
        assert!(selection.apply_alias("D", true));
        assert!(selection.docker_data_root);
        assert!(selection.apply_alias("P", false));
        assert!(!selection.enable_p10k);
    }
}
