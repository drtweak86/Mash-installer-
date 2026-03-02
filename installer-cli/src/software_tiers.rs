//! Software tier **UI** — menu rendering and user selection.
//!
//! This module owns the CLI-layer half of software tiers: it presents menus,
//! collects user choices, and returns a [`SoftwareTierPlan`] to the caller.
//!
//! **Boundary note**: All install logic and data model types live in
//! `installer-core/src/software_tiers.rs`. Nothing in this module installs packages.

use super::menu::run_theme_menu;
use crate::software_catalog;
use anyhow::Result;
use installer_core::interaction::InteractionService;
use installer_core::{SoftwareTierPlan, ThemePlan};
use std::collections::BTreeMap;
use std::io::{self, Write};

pub fn run_software_tier_menu(interaction: &InteractionService) -> Result<SoftwareTierPlan> {
    println!("\nStep 3/4: Curated software tiers");
    println!("Choose how Mash-Installer picks S/A-tier software:");
    let options = [
        "BARD'S RECOMMENDATIONS (S-TIER ONLY)",
        "AUTOMATIC BASELINE (QUICK SYNC)",
        "MANUAL CATEGORY AUDIT (FINE TUNING)",
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

    match choice {
        1 => {
            let catalog = software_catalog::load_s_tier();
            let theme_plan = ThemePlan::RetroWithWallpapers;
            let mut picks = BTreeMap::new();
            for category in &catalog.categories {
                for subcategory in &category.subcategories {
                    if let Some(recommended) = subcategory.programs.iter().find(|p| p.recommended) {
                        picks.insert(category.display_name.clone(), recommended.id.clone());
                    } else if let Some(first) = subcategory.programs.first() {
                        picks.insert(category.display_name.clone(), first.id.clone());
                    }
                }
            }
            Ok(SoftwareTierPlan::new(true, picks, theme_plan, None))
        }
        2 => {
            let catalog = software_catalog::load_s_tier();
            let theme_plan = ThemePlan::RetroOnly;
            let mut picks = BTreeMap::new();
            for category in &catalog.categories {
                for subcategory in &category.subcategories {
                    if let Some(first) = subcategory.programs.first() {
                        picks.insert(category.display_name.clone(), first.id.clone());
                    }
                }
            }
            Ok(SoftwareTierPlan::new(true, picks, theme_plan, None))
        }
        3 => run_custom_selection(interaction),
        _ => unreachable!(),
    }
}

fn run_custom_selection(interaction: &InteractionService) -> Result<SoftwareTierPlan> {
    let catalog = software_catalog::load_full();
    let mut picks = BTreeMap::new();

    for category in &catalog.categories {
        println!("\nCategory: {}", category.display_name);
        let all_programs: Vec<_> = category
            .subcategories
            .iter()
            .flat_map(|sc| &sc.programs)
            .collect();

        let option_lines: Vec<String> = all_programs
            .iter()
            .map(|p| format!("{} [{}] – {}", p.name, p.tier, p.description))
            .collect();
        let option_refs: Vec<&str> = option_lines.iter().map(String::as_str).collect();

        let prompt = format!("Pick a tool for {}", category.display_name);
        let selection = interaction.select_option(
            &format!("software.tier.{}", category.name),
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
        let chosen = all_programs[selection - 1];
        picks.insert(category.display_name.clone(), chosen.id.clone());
    }

    // Add theme selection
    println!("\nStep 3/6: Theme Selection");
    let theme_plan = run_theme_menu(interaction)?;

    Ok(SoftwareTierPlan::new(false, picks, theme_plan, None))
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
