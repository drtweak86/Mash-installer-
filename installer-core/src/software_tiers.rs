//! Software tier data model and installation phase.
//!
//! This module owns two concerns that belong in the **core** crate:
//! - **Data model**: [`SoftwareTierPlan`] and [`ThemePlan`] — the user's selections,
//!   constructed by the CLI layer and threaded into every install phase via [`PhaseContext`].
//! - **Install logic**: [`install_phase`] — consumes the plan and actually installs packages.
//!
//! **Boundary note**: UI rendering (menus, prompts, selection) lives exclusively in
//! `installer-cli/src/software_tiers.rs`. Nothing in this module should touch stdio.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::PermissionsExt;

use anyhow::{Context, Result};

use crate::{cmd, package_manager, PhaseContext};

#[derive(Clone, Debug, Default)]
pub enum ThemePlan {
    #[default]
    None,
    RetroOnly,
    RetroWithWallpapers,
}

#[derive(Clone, Debug)]
pub struct SoftwareTierPlan {
    pub full_install: bool,
    pub selections: BTreeMap<&'static str, &'static str>,
    pub theme_plan: ThemePlan,
}

impl SoftwareTierPlan {
    pub fn new(
        full_install: bool,
        selections: BTreeMap<&'static str, &'static str>,
        theme_plan: ThemePlan,
    ) -> Self {
        Self {
            full_install,
            selections,
            theme_plan,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.selections.is_empty() && matches!(self.theme_plan, ThemePlan::None)
    }
}

impl Default for SoftwareTierPlan {
    fn default() -> Self {
        Self {
            full_install: true,
            selections: BTreeMap::new(),
            theme_plan: ThemePlan::None,
        }
    }
}

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    let plan = &ctx.options.software_plan;
    if plan.is_empty() {
        tracing::info!("No software tiers selected; skipping.");
        return Ok(());
    }

    let mut required = BTreeSet::new();
    let mut optional = BTreeSet::new();

    for (category, selection) in plan.selections.iter() {
        if let Some(packages) = packages_for_selection(selection) {
            required.extend(packages.required.iter().copied());
            optional.extend(packages.optional.iter().copied());
        } else {
            ctx.record_warning(format!(
                "No package mapping for software tier selection: {category} -> {selection}"
            ));
        }
    }

    install_packages(ctx, &required, &optional)?;
    apply_theme_plan(ctx, &plan.theme_plan)?;

    Ok(())
}

struct PackageSet {
    required: &'static [&'static str],
    optional: &'static [&'static str],
}

fn packages_for_selection(selection: &str) -> Option<PackageSet> {
    let packages = match selection {
        // Terminal
        "Kitty" => PackageSet {
            required: &["kitty"],
            optional: &[],
        },
        "Alacritty" => PackageSet {
            required: &["alacritty"],
            optional: &[],
        },
        "WezTerm" => PackageSet {
            required: &["wezterm"],
            optional: &[],
        },
        "Foot" => PackageSet {
            required: &["foot"],
            optional: &[],
        },
        "ST" => PackageSet {
            required: &["st"],
            optional: &[],
        },
        // Shell
        "Zsh + Starship" => PackageSet {
            required: &["zsh", "starship"],
            optional: &[],
        },
        "Fish" => PackageSet {
            required: &["fish"],
            optional: &[],
        },
        "Bash" => PackageSet {
            required: &["bash"],
            optional: &[],
        },
        "Nu" => PackageSet {
            required: &["nushell"],
            optional: &[],
        },
        "PowerShell Core" => PackageSet {
            required: &[],
            optional: &["powershell"],
        },
        // File Manager
        "eza" => PackageSet {
            required: &["eza"],
            optional: &[],
        },
        "lf" => PackageSet {
            required: &["lf"],
            optional: &[],
        },
        "nnn" => PackageSet {
            required: &["nnn"],
            optional: &[],
        },
        "ranger" => PackageSet {
            required: &["ranger"],
            optional: &[],
        },
        "vifm" => PackageSet {
            required: &["vifm"],
            optional: &[],
        },
        // Text Editor
        "Helix" => PackageSet {
            required: &["helix"],
            optional: &[],
        },
        "Neovim" => PackageSet {
            required: &["neovim"],
            optional: &[],
        },
        "Visual Studio Code" => PackageSet {
            required: &[],
            optional: &["code"],
        },
        "Micro" => PackageSet {
            required: &["micro"],
            optional: &[],
        },
        "Kakoune" => PackageSet {
            required: &["kakoune"],
            optional: &[],
        },
        // Git Client
        "Lazygit" => PackageSet {
            required: &["lazygit"],
            optional: &[],
        },
        "Tig" => PackageSet {
            required: &["tig"],
            optional: &[],
        },
        "GitUI" => PackageSet {
            required: &["gitui"],
            optional: &[],
        },
        "Forge" => PackageSet {
            required: &[],
            optional: &["forge"],
        },
        "GitHub CLI (gh)" => PackageSet {
            required: &["gh"],
            optional: &[],
        },
        // Process Viewer
        "btop" => PackageSet {
            required: &["btop"],
            optional: &[],
        },
        "glances" => PackageSet {
            required: &["glances"],
            optional: &[],
        },
        "htop" => PackageSet {
            required: &["htop"],
            optional: &[],
        },
        "bpytop" => PackageSet {
            required: &["bpytop"],
            optional: &[],
        },
        "gotop" => PackageSet {
            required: &["gotop"],
            optional: &[],
        },
        // Browser
        "Brave" => PackageSet {
            required: &[],
            optional: &["brave-browser"],
        },
        "Librewolf" => PackageSet {
            required: &[],
            optional: &["librewolf"],
        },
        "Vivaldi" => PackageSet {
            required: &[],
            optional: &["vivaldi-stable"],
        },
        "Firefox" => PackageSet {
            required: &["firefox"],
            optional: &[],
        },
        "Chromium" => PackageSet {
            required: &["chromium"],
            optional: &[],
        },
        // Media Player
        "MPV" => PackageSet {
            required: &["mpv"],
            optional: &[],
        },
        "VLC" => PackageSet {
            required: &["vlc"],
            optional: &[],
        },
        "SMPlayer" => PackageSet {
            required: &["smplayer"],
            optional: &[],
        },
        "Celluloid" => PackageSet {
            required: &["celluloid"],
            optional: &[],
        },
        "MPlayer" => PackageSet {
            required: &["mplayer"],
            optional: &[],
        },
        // HTPC
        "Kodi" => PackageSet {
            required: &["kodi"],
            optional: &[],
        },
        "Plex Media Server" => PackageSet {
            required: &[],
            optional: &["plexmediaserver"],
        },
        "Jellyfin" => PackageSet {
            required: &[],
            optional: &["jellyfin"],
        },
        "Emby" => PackageSet {
            required: &[],
            optional: &["emby-server"],
        },
        "OSMC" => PackageSet {
            required: &[],
            optional: &["osmc"],
        },
        // VPN
        "WireGuard" => PackageSet {
            required: &["wireguard"],
            optional: &[],
        },
        "OpenVPN" => PackageSet {
            required: &["openvpn"],
            optional: &[],
        },
        "Tailscale" => PackageSet {
            required: &["tailscale"],
            optional: &[],
        },
        "StrongSwan" => PackageSet {
            required: &["strongswan"],
            optional: &[],
        },
        "OpenConnect" => PackageSet {
            required: &["openconnect"],
            optional: &[],
        },
        // Firewall
        "nftables" => PackageSet {
            required: &["nftables"],
            optional: &[],
        },
        "firewalld" => PackageSet {
            required: &["firewalld"],
            optional: &[],
        },
        "ufw" => PackageSet {
            required: &["ufw"],
            optional: &[],
        },
        "iptables" => PackageSet {
            required: &["iptables"],
            optional: &[],
        },
        "Shorewall" => PackageSet {
            required: &[],
            optional: &["shorewall"],
        },
        // Backup
        "rclone + borg" => PackageSet {
            required: &["rclone", "borgbackup"],
            optional: &[],
        },
        "restic" => PackageSet {
            required: &["restic"],
            optional: &[],
        },
        "duplicacy" => PackageSet {
            required: &[],
            optional: &["duplicacy"],
        },
        "duplicity" => PackageSet {
            required: &["duplicity"],
            optional: &[],
        },
        "borgmatic" => PackageSet {
            required: &[],
            optional: &["borgmatic"],
        },
        _ => return None,
    };
    Some(packages)
}

fn install_packages(
    ctx: &mut PhaseContext,
    required: &BTreeSet<&'static str>,
    optional: &BTreeSet<&'static str>,
) -> Result<()> {
    if !required.is_empty() {
        let required_vec: Vec<&str> = required.iter().copied().collect();
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

    for pkg in optional.iter().copied() {
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
            crate::theme::install_retro_theme(&home_dir)?;
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
