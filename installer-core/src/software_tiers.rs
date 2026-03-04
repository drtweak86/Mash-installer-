//! Software tier data model and installation phase.
//!
//! This module owns two concerns that belong in the **core** crate:
//! - **Data model**: [`SoftwareTierPlan`] and [`ThemePlan`] — the user's selections,
//!   constructed by the CLI layer and threaded into every install phase via [`PhaseContext`].
//! - **Install logic**: [`install_phase`] — consumes the plan and actually installs packages.
//!
//! **Boundary note**: UI rendering (menus, prompts, selection) lives exclusively in
//! `installer-cli/src/software_tiers.rs`. Nothing in this module should touch stdio.

use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub use installer_model::software::{SoftwareTierPlan, ThemePlan};

use crate::catalog::{Catalog, Program};
use crate::{package_manager, AuthType, AuthorizationService, PhaseContext, PhaseResult};
use mash_system::cmd;

pub fn install_phase(ctx: &mut PhaseContext) -> Result<PhaseResult> {
    let plan = &ctx.options.software_plan;

    if plan.is_empty() {
        tracing::info!("No software tiers selected; skipping.");
        return Ok(PhaseResult::Success);
    }

    // Load catalogs
    let s_tier = Catalog::load_s_tier()?;
    let full = Catalog::load_full()?;
    let languages = Catalog::load_languages()?;

    let mut required = BTreeSet::new();
    let optional = BTreeSet::new();

    let distro_family = &ctx.platform.platform.distro_family;

    for program_id in plan.selections.values() {
        if let Some(program) = find_program_in_catalogs(program_id, &[&s_tier, &full, &languages]) {
            // Optimization: If full_install is true, we only install S-tier if specifically in that mode.
            // Wait, if full_install is true, we WANT all selections.
            // If full_install is false, we might want to filter? No, the selections are explicit.

            // The actual logic should be: if full_install is true AND it's "BardsRecommendations",
            // we already have the S-tier picks.
            // If it's "Auto", we have the first from each category.

            // Let's refine: Only install if it matches the distro family.
            if let Some(pkgs) = program.packages.get(distro_family) {
                for pkg in pkgs {
                    required.insert(pkg.clone());
                }
            } else {
                ctx.record_warning(format!(
                    "No package mapping for program {} on distro family {}",
                    program.name, distro_family
                ));
            }
        } else {
            ctx.record_warning(format!(
                "Program ID {} not found in any catalog",
                program_id
            ));
        }
    }

    install_packages(ctx, &required, &optional)?;
    apply_theme_plan(ctx, &plan.theme_plan)?;

    if ctx.options.interactive {
        let has_borg = plan.selections.values().any(|v| v == "borgbackup");
        if has_borg
            && !AuthorizationService::new(ctx.observer, ctx.options)
                .is_authorized(AuthType::BorgSetup)
            && ctx.observer.request_auth(AuthType::BorgSetup)?
        {
            AuthorizationService::new(ctx.observer, ctx.options).authorize(AuthType::BorgSetup)?;
            ctx.record_configured("Borg backup repository");
        }

        let has_tailscale = plan.selections.values().any(|v| v == "tailscale");
        if has_tailscale
            && !AuthorizationService::new(ctx.observer, ctx.options)
                .is_authorized(AuthType::TailscaleAuth)
            && ctx.observer.request_auth(AuthType::TailscaleAuth)?
        {
            AuthorizationService::new(ctx.observer, ctx.options)
                .authorize(AuthType::TailscaleAuth)?;
            ctx.record_configured("Tailscale (Authorized)");
        }

        let has_ngrok = plan.selections.values().any(|v| v == "ngrok");
        if has_ngrok
            && !AuthorizationService::new(ctx.observer, ctx.options)
                .is_authorized(AuthType::NgrokAuth)
            && ctx.observer.request_auth(AuthType::NgrokAuth)?
        {
            AuthorizationService::new(ctx.observer, ctx.options).authorize(AuthType::NgrokAuth)?;
            ctx.record_configured("Ngrok authtoken");
        }

        let has_cloudflared = plan.selections.values().any(|v| v == "cloudflared");
        if has_cloudflared
            && !AuthorizationService::new(ctx.observer, ctx.options)
                .is_authorized(AuthType::CloudflaredAuth)
            && ctx.observer.request_auth(AuthType::CloudflaredAuth)?
        {
            AuthorizationService::new(ctx.observer, ctx.options)
                .authorize(AuthType::CloudflaredAuth)?;
            ctx.record_configured("Cloudflared (Authorized)");
        }
    }

    Ok(PhaseResult::Success)
}

fn find_program_in_catalogs<'a>(id: &str, catalogs: &[&'a Catalog]) -> Option<&'a Program> {
    for catalog in catalogs {
        for category in &catalog.categories {
            for subcategory in &category.subcategories {
                if let Some(program) = subcategory.programs.iter().find(|p| p.id == id) {
                    return Some(program);
                }
            }
        }
    }
    None
}

fn install_packages(
    ctx: &mut PhaseContext,
    required: &BTreeSet<String>,
    optional: &BTreeSet<String>,
) -> Result<()> {
    if !required.is_empty() {
        let required_vec: Vec<&str> = required.iter().map(|s| s.as_str()).collect();
        if ctx.options.dry_run {
            ctx.record_dry_run(
                "software_tiers",
                "Would install selected packages",
                Some(required_vec.join(", ")),
            );
        }
        if let Err(err) = package_manager::ensure_packages(
            ctx.platform.driver,
            &required_vec,
            ctx.options.dry_run,
        ) {
            ctx.record_warning(format!("Software tier package install failed: {err}"));
        } else {
            ctx.record_action(format!(
                "Installed {} software-tier packages",
                required_vec.len()
            ));
        }
    }

    for pkg in optional {
        if ctx.options.dry_run {
            ctx.record_dry_run(
                "software_tiers",
                "Would attempt optional package",
                Some(pkg.to_string()),
            );
        }
        package_manager::try_optional(ctx.platform.driver, pkg, ctx.options.dry_run);
    }

    Ok(())
}

fn apply_theme_plan(ctx: &mut PhaseContext, plan: &ThemePlan) -> Result<()> {
    match plan {
        ThemePlan::None => Ok(()),
        ThemePlan::RetroOnly => install_retro_theme_plan(ctx, false),
        ThemePlan::RetroWithWallpapers => install_retro_theme_plan(ctx, true),
    }
}

fn install_retro_theme_plan(ctx: &mut PhaseContext, with_wallpapers: bool) -> Result<()> {
    let mut required = vec!["i3", "i3status", "i3lock", "kitty", "conky"];
    if with_wallpapers {
        required.extend_from_slice(&["feh", "python3", "python3-pip"]);
    }

    if ctx.options.dry_run {
        ctx.record_dry_run(
            "software_tiers",
            "Would install retro theme dependencies",
            Some(required.join(", ")),
        );
    }
    if let Err(err) =
        package_manager::ensure_packages(ctx.platform.driver, &required, ctx.options.dry_run)
    {
        ctx.record_warning(format!("Retro theme dependency install failed: {err}"));
        return Ok(());
    }

    let Some(home_dir) = dirs::home_dir() else {
        ctx.record_warning("Unable to locate home directory; skipping retro theme install");
        return Ok(());
    };
    if let Err(err) = ctx.run_or_record(
        "software_tiers",
        "Install BBC/UNIX Retro Theme",
        Some(home_dir.display().to_string()),
        |ctx| {
            crate::theme::install_retro_theme(&home_dir, ctx.options.dry_run)?;
            ctx.record_action("Installed BBC/UNIX Retro Theme");
            Ok(())
        },
    ) {
        ctx.record_warning(format!("Retro theme install failed: {err}"));
        return Ok(());
    }

    if with_wallpapers {
        if let Err(err) = install_wallpaper_downloader(ctx, &home_dir) {
            ctx.record_warning(format!("Wallpaper pack download failed: {err}"));
        }
    }

    Ok(())
}

fn install_wallpaper_downloader(ctx: &mut PhaseContext, home_dir: &std::path::Path) -> Result<()> {
    let script_path = home_dir.join(".local/bin/wallpaper_downloader_final.py");
    let pip_cmd = ["-m", "pip", "install", "--user", "requests"];
    let first_boot_script = home_dir.join(".local/bin/mash-retro-wallpapers-first-boot.sh");
    let systemd_dir = home_dir.join(".config/systemd/user");
    let service_path = systemd_dir.join("mash-retro-wallpapers.service");
    let marker_path = home_dir.join(".config/mash-installer/retro-wallpapers.done");

    if ctx.options.dry_run {
        ctx.record_dry_run(
            "software_tiers",
            "Would install Python requests dependency",
            Some("python3 -m pip install --user requests".to_string()),
        );
        ctx.record_dry_run(
            "software_tiers",
            "Would configure first-boot wallpaper download",
            Some(first_boot_script.display().to_string()),
        );
        return Ok(());
    }

    cmd::Command::new("python3")
        .args(pip_cmd)
        .execute()
        .context("installing Python requests dependency")?;

    if let Some(parent) = first_boot_script.parent() {
        fs::create_dir_all(parent).context("creating first-boot script directory")?;
    }
    if let Some(parent) = service_path.parent() {
        fs::create_dir_all(parent).context("creating systemd user directory")?;
    }

    let script_body = format!(
        r#"#!/bin/sh
set -e
MARKER="{marker}"
if [ -f "$MARKER" ]; then
  exit 0
fi
python3 "{downloader}" --first-boot
mkdir -p "$(dirname "$MARKER")"
touch "$MARKER"
"#,
        marker = marker_path.display(),
        downloader = script_path.display()
    );
    fs::write(&first_boot_script, script_body).context("writing first-boot script")?;
    let mut perms = fs::metadata(&first_boot_script)
        .context("reading first-boot script permissions")?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&first_boot_script, perms)
        .context("setting first-boot script permissions")?;

    let service_body = format!(
        r#"[Unit]
Description=MASH Retro Wallpapers (first boot)
After=network-online.target

[Service]
Type=oneshot
ExecStart={script}

[Install]
WantedBy=default.target
"#,
        script = first_boot_script.display()
    );
    fs::write(&service_path, service_body).context("writing systemd user service")?;

    if crate::theme::command_exists("systemctl") {
        if let Err(err) = cmd::Command::new("systemctl")
            .args(["--user", "enable", "--now", "mash-retro-wallpapers.service"])
            .execute()
        {
            ctx.record_warning(format!(
                "Failed to enable first-boot wallpaper service: {err}"
            ));
        }
    } else {
        ctx.record_warning("systemctl not found; first-boot wallpaper service not enabled");
    }

    ctx.record_action("Configured first-boot retro wallpaper download");
    Ok(())
}
