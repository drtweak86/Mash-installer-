use anyhow::{Context, Error, Result};
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use installer_core::{
    detect_platform, DistroDriver, InstallOptions, PhaseObserver, PlatformInfo, ProfileLevel,
    RunSummary,
};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{info, warn};

#[derive(Parser)]
#[command(
    name = "installer-cli",
    about = "Workspace-aware mash installer entrypoint"
)]
struct Cli {
    #[arg(long)]
    staging_dir: Option<PathBuf>,

    #[arg(long)]
    dry_run: bool,

    #[arg(long)]
    non_interactive: bool,

    #[arg(long, short)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .without_time()
        .init();

    let platform_info = detect_platform().context("detecting host platform")?;
    let drivers = vec![
        installer_arch::driver(),
        installer_debian::driver(),
        installer_fedora::driver(),
    ];
    let driver = if cli.non_interactive {
        auto_detect_driver(&drivers, &platform_info).unwrap_or_else(|| drivers[0])
    } else {
        run_driver_selection(&drivers, &platform_info)
    };

    let modules = if cli.non_interactive {
        ModuleSelection::default()
    } else {
        run_module_menu(driver.name())
    };

    let profile = if cli.non_interactive {
        ProfileLevel::Dev
    } else {
        run_profile_menu()
    };

    let options = InstallOptions {
        profile,
        staging_dir: cli.staging_dir,
        dry_run: cli.dry_run,
        interactive: !cli.non_interactive,
        enable_argon: modules.enable_argon,
        enable_p10k: modules.enable_p10k,
        docker_data_root: modules.docker_data_root,
    };

    info!(
        "Selected driver: {} ({}). Profile: {:?}. Modules: {:?}",
        driver.name(),
        driver.description(),
        profile,
        modules
    );

    print_banner();

    let mut observer = CliPhaseObserver::new();
    let result = installer_core::run_with_driver(driver, options.clone(), &mut observer);
    observer.finish();

    match result {
        Ok(summary) => {
            print_completion_message(&summary, options.dry_run);
            Ok(())
        }
        Err(err) => Err(err).context("installer failed"),
    }
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
    fn total_phases(&mut self, total: usize) {
        self.overall.set_length(total as u64);
    }

    fn on_phase_started(&mut self, index: usize, total: usize, label: &'static str) {
        self.finish_spinner(" ", "");
        let display = format!("Phase {}/{} · {}", index, total, label);
        self.start_spinner(&display);
    }

    fn on_phase_success(&mut self, _index: usize, done_msg: &'static str) {
        self.finish_spinner("✓", done_msg);
        self.overall.inc(1);
    }

    fn on_phase_failure(&mut self, _index: usize, label: &'static str, err: &Error) {
        let message = format!("{label} FAILED: {err}");
        self.finish_spinner("✗", &message);
        self.overall.inc(1);
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
) -> &'static dyn DistroDriver {
    println!("Step 1/3: Distro selection");
    println!("1) Auto detect (default)");
    println!("2) Select distribution manually");
    let choice = prompt_choice("Choose distro mode", 1, 2);

    if choice == 1 {
        if let Some(driver) = auto_detect_driver(drivers, platform) {
            println!("Auto-detected driver: {}", driver.name());
            return driver;
        }
        warn!("Auto-detection failed; falling back to manual selection.");
    }

    select_driver_from_list(drivers)
}

fn select_driver_from_list(drivers: &[&'static dyn DistroDriver]) -> &'static dyn DistroDriver {
    println!("Available distro drivers:");
    for (idx, driver) in drivers.iter().enumerate() {
        println!(
            "  {}) {} – {}",
            idx + 1,
            driver.name(),
            driver.description()
        );
    }
    let index = prompt_choice("Pick a driver", 1, drivers.len());
    drivers.get(index - 1).copied().unwrap_or(drivers[0])
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

fn run_module_menu(driver_name: &str) -> ModuleSelection {
    println!("\nStep 2/3: Modules for {}", driver_name);
    println!("1) Full install (default – all modules enabled)");
    println!("2) Select modules");
    let choice = prompt_choice("Choose install mode", 1, 2);

    if choice == 1 {
        ModuleSelection::full()
    } else {
        println!("Available module toggles (use aliases to remember choices):");
        for opt in MODULE_OPTIONS {
            println!("  [{}] {} – {}", opt.alias, opt.label, opt.description);
        }
        let mut selection = ModuleSelection::default();
        for opt in MODULE_OPTIONS {
            let prompt = format!("Enable {} (alias {})?", opt.label, opt.alias);
            let enabled = prompt_yes_no(&prompt, opt.default);
            selection.apply_alias(opt.alias, enabled);
        }
        selection
    }
}

fn run_profile_menu() -> ProfileLevel {
    println!("\nStep 3/3: Choose profile");
    println!("1) basics – minimal tooling");
    println!("2) basics-dev – add developer packages");
    println!("3) basics+QoL – dev + shell polish");
    println!("4) full modular – everything (default)");
    let choice = prompt_choice("Pick a profile", 4, 4);

    match choice {
        1 => ProfileLevel::Minimal,
        2 => ProfileLevel::Dev,
        3 => ProfileLevel::Dev,
        _ => ProfileLevel::Full,
    }
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
