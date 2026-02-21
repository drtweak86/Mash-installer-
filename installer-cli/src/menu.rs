//! Interactive menu system for driver, module, and profile selection

use anyhow::Result;
use installer_core::{
    interaction::InteractionService, DistroDriver, PlatformInfo, ProfileLevel, ThemePlan,
};
use std::io::{self, Write};
use tracing::warn;

#[derive(Debug, Default)]
pub struct ModuleSelection {
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
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

pub fn run_theme_menu(interaction: &InteractionService) -> Result<ThemePlan> {
    println!("\nStep 3/6: Theme Selection");
    println!("Choose your window manager theme:");

    let options = vec![
        "BBC/UNIX Retro Theme (i3 + Kitty) - Classic 1980s computing aesthetic",
        "BBC/UNIX Retro Theme + Wallpaper Pack - Complete retro experience with 6000+ wallpapers",
        "No theme changes - Keep current configuration",
    ];

    let choice = interaction.select_option(
        "theme.selection",
        "Select theme option",
        &options,
        3,
        |prompt, options| {
            for (idx, option) in options.iter().enumerate() {
                println!("{}) {}", idx + 1, option);
            }
            Ok(prompt_choice(prompt, 3, options.len()))
        },
    )?;

    match choice {
        1 => Ok(ThemePlan::RetroOnly),
        2 => Ok(ThemePlan::RetroWithWallpapers),
        _ => Ok(ThemePlan::None),
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

pub fn auto_detect_driver(
    drivers: &[&'static dyn DistroDriver],
    platform: &PlatformInfo,
) -> Option<&'static dyn DistroDriver> {
    drivers.iter().copied().find(|d| d.matches(platform))
}

pub fn run_driver_selection(
    drivers: &[&'static dyn DistroDriver],
    platform: &PlatformInfo,
    interaction: &InteractionService,
) -> Result<&'static dyn DistroDriver> {
    println!("Step 1/4: Distro selection");
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

pub fn run_module_menu(
    driver_name: &str,
    interaction: &InteractionService,
) -> Result<ModuleSelection> {
    println!("\nStep 2/4: Modules for {}", driver_name);
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

pub fn run_profile_menu(interaction: &InteractionService) -> Result<ProfileLevel> {
    println!("\nStep 4/4: Choose profile");
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
