//! Software tier **UI** — menu rendering and user selection.
//!
//! This module owns the CLI-layer half of software tiers: it presents menus,
//! collects user choices, and returns a [`SoftwareTierPlan`] to the caller.
//!
//! **Boundary note**: All install logic and data model types live in
//! `installer-core/src/software_tiers.rs`. Nothing in this module installs packages.

use super::menu::run_theme_menu;
use crate::software_catalog::{SoftwareOption, SOFTWARE_CATEGORIES};
use anyhow::Result;
use installer_core::{interaction::InteractionService, SoftwareTierPlan};
use std::collections::BTreeMap;
use std::io::{self, Write};

pub fn run_software_tier_menu(interaction: &InteractionService) -> Result<SoftwareTierPlan> {
    println!("\nStep 3/4: Curated software tiers");
    println!("Choose how Mash-Installer picks S/A-tier software:");
    let options = [
        "Full S-tier install (recommended bundle)",
        "Choose per category",
    ];
    let choice = interaction.select_option(
        "software.tiers.mode",
        "Pick tier mode",
        &options,
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
        // For full S-tier install, we still need to ask about theme
        println!("\nStep 3/6: Theme Selection");
        let theme_plan = run_theme_menu(interaction)?;
        let mut picks = BTreeMap::new();
        for category in SOFTWARE_CATEGORIES {
            if let Some(recommended) = category.options.first() {
                picks.insert(category.label, recommended.name);
            }
        }
        Ok(SoftwareTierPlan::new(true, picks, theme_plan))
    } else {
        run_custom_selection(interaction)
    }
}

fn run_custom_selection(interaction: &InteractionService) -> Result<SoftwareTierPlan> {
    let mut picks = BTreeMap::new();
    for category in SOFTWARE_CATEGORIES {
        println!("\nCategory: {}", category.label);
        let option_lines: Vec<String> = category.options.iter().map(format_option).collect();
        let option_refs: Vec<&str> = option_lines.iter().map(String::as_str).collect();
        let prompt = format!("Pick a tool for {}", category.label);
        let selection = interaction.select_option(
            &format!("software.tier.{}", category.label),
            &prompt,
            &option_refs,
            1,
            |prompt, options| {
                for (idx, option) in options.iter().enumerate() {
                    println!("{}) {}", idx + 1, option);
                }
                Ok(prompt_choice(prompt, 1, options.len()))
            },
        )?;
        let chosen = &category.options[selection - 1];
        picks.insert(category.label, chosen.name);
    }

    // Add theme selection
    println!("\nStep 3/6: Theme Selection");
    let theme_plan = run_theme_menu(interaction)?;

    Ok(SoftwareTierPlan::new(false, picks, theme_plan))
}

fn format_option(option: &SoftwareOption) -> String {
    format!(
        "{} ({}) – {}",
        option.name,
        option.tier.label(),
        option.description
    )
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
