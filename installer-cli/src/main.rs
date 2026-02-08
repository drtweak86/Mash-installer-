use anyhow::{Context, Result};
use clap::Parser;
use installer_arch;
use installer_core::{detect_platform, DistroDriver, InstallOptions, PlatformInfo, ProfileLevel};
use installer_debian;
use installer_fedora;
use std::io::{self, Write};
use std::path::PathBuf;
use tracing::{info, warn};
use tracing_subscriber;

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

    installer_core::run_with_driver(driver, options).context("installer failed")
}

fn auto_detect_driver<'a>(
    drivers: &'a [&'static dyn DistroDriver],
    platform: &PlatformInfo,
) -> Option<&'static dyn DistroDriver> {
    drivers.iter().copied().find(|d| d.matches(platform))
}

fn run_driver_selection<'a>(
    drivers: &'a [&'static dyn DistroDriver],
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

fn select_driver_from_list<'a>(
    drivers: &'a [&'static dyn DistroDriver],
) -> &'static dyn DistroDriver {
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

#[derive(Debug)]
struct ModuleSelection {
    enable_argon: bool,
    enable_p10k: bool,
    docker_data_root: bool,
}

impl Default for ModuleSelection {
    fn default() -> Self {
        Self {
            enable_argon: false,
            enable_p10k: false,
            docker_data_root: false,
        }
    }
}

fn run_module_menu(driver_name: &str) -> ModuleSelection {
    println!("\nStep 2/3: Modules for {}", driver_name);
    println!("1) Full install (default – all modules enabled)");
    println!("2) Select modules");
    let choice = prompt_choice("Choose install mode", 1, 2);

    if choice == 1 {
        ModuleSelection {
            enable_argon: true,
            enable_p10k: true,
            docker_data_root: true,
        }
    } else {
        ModuleSelection {
            enable_argon: prompt_yes_no("Enable Argon One fan control?", false),
            enable_p10k: prompt_yes_no("Enable Powerlevel10k prompt?", true),
            docker_data_root: prompt_yes_no("Manage Docker data-root?", false),
        }
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
    loop {
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
        return default;
    }
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
