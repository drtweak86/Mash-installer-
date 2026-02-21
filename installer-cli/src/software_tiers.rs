use super::menu::run_theme_menu;
use anyhow::Result;
use installer_core::{interaction::InteractionService, SoftwareTierPlan};
use std::collections::BTreeMap;
use std::io::{self, Write};

struct SoftwareCategory {
    label: &'static str,
    options: &'static [SoftwareOption],
}

struct SoftwareOption {
    name: &'static str,
    tier: Tier,
    description: &'static str,
}

#[derive(Clone, Copy)]
enum Tier {
    STier,
    ATier,
}

impl Tier {
    fn label(self) -> &'static str {
        match self {
            Tier::STier => "S-tier",
            Tier::ATier => "A-tier",
        }
    }
}

const SOFTWARE_CATEGORIES: &[SoftwareCategory] = &[
    SoftwareCategory {
        label: "Terminal",
        options: &[
            SoftwareOption {
                name: "Kitty",
                tier: Tier::STier,
                description: "GPU-accelerated, ligature-ready, and lightning fast.",
            },
            SoftwareOption {
                name: "Alacritty",
                tier: Tier::ATier,
                description: "Rust-rendered terminal with sane defaults and config file.",
            },
            SoftwareOption {
                name: "WezTerm",
                tier: Tier::ATier,
                description: "Cross-platform GPU terminal with pane-aware multiplexing.",
            },
            SoftwareOption {
                name: "Foot",
                tier: Tier::ATier,
                description: "Leaner Wayland terminal for minimalist desktops.",
            },
            SoftwareOption {
                name: "ST",
                tier: Tier::ATier,
                description: "Minimal suckless terminal for keyboard purists.",
            },
        ],
    },
    SoftwareCategory {
        label: "Shell",
        options: &[
            SoftwareOption {
                name: "Zsh + Starship",
                tier: Tier::STier,
                description: "Pluggable Zsh with Starship prompter and curated polish.",
            },
            SoftwareOption {
                name: "Fish",
                tier: Tier::ATier,
                description: "Smart completions and sane defaults without extra configuration.",
            },
            SoftwareOption {
                name: "Bash",
                tier: Tier::ATier,
                description: "Ubiquitous shell that still shows up in rescue scenarios.",
            },
            SoftwareOption {
                name: "Nu",
                tier: Tier::ATier,
                description: "Structured data pipelines and table-first feedback loops.",
            },
            SoftwareOption {
                name: "PowerShell Core",
                tier: Tier::ATier,
                description: "Object pipelines for cross-platform automation and Azure work.",
            },
        ],
    },
    SoftwareCategory {
        label: "File Manager",
        options: &[
            SoftwareOption {
                name: "eza",
                tier: Tier::STier,
                description: "Icon-aware `ls` replacement with Git + tree views.",
            },
            SoftwareOption {
                name: "lf",
                tier: Tier::ATier,
                description: "Minimal, fast file navigator with custom actions.",
            },
            SoftwareOption {
                name: "nnn",
                tier: Tier::ATier,
                description: "Extensible, plugin-friendly file explorer for the terminal.",
            },
            SoftwareOption {
                name: "ranger",
                tier: Tier::ATier,
                description: "Vim-style columns and file previews for power users.",
            },
            SoftwareOption {
                name: "vifm",
                tier: Tier::ATier,
                description: "Vi-inspired interface with dual panes and scripting.",
            },
        ],
    },
    SoftwareCategory {
        label: "Text Editor",
        options: &[
            SoftwareOption {
                name: "Helix",
                tier: Tier::STier,
                description: "Modern modal editor written in Rust with sensible defaults.",
            },
            SoftwareOption {
                name: "Neovim",
                tier: Tier::ATier,
                description: "Configurable Vim replacement with plugin ecosystem.",
            },
            SoftwareOption {
                name: "Visual Studio Code",
                tier: Tier::ATier,
                description: "Full-featured IDE experience with extension marketplace.",
            },
            SoftwareOption {
                name: "Micro",
                tier: Tier::ATier,
                description: "Nano-like drop-in editor with mouse and plugin support.",
            },
            SoftwareOption {
                name: "Kakoune",
                tier: Tier::ATier,
                description: "Multi-cursor, modal editing that stays lean.",
            },
        ],
    },
    SoftwareCategory {
        label: "Git Client",
        options: &[
            SoftwareOption {
                name: "Lazygit",
                tier: Tier::STier,
                description: "TUI git client with staging, history, and branching insight.",
            },
            SoftwareOption {
                name: "Tig",
                tier: Tier::ATier,
                description: "Ncurses git history viewer for quick diffs.",
            },
            SoftwareOption {
                name: "GitUI",
                tier: Tier::ATier,
                description: "Rust-based terminal UI with visual diffs and staging.",
            },
            SoftwareOption {
                name: "Forge",
                tier: Tier::ATier,
                description: "`gh`-powered workflow with PR/issue navigation baked in.",
            },
            SoftwareOption {
                name: "GitHub CLI (gh)",
                tier: Tier::ATier,
                description: "Scriptable interface for GitHub-native automation.",
            },
        ],
    },
    SoftwareCategory {
        label: "Process Viewer",
        options: &[
            SoftwareOption {
                name: "btop",
                tier: Tier::STier,
                description: "Prettier metrics, graphs, and widgets over htop.",
            },
            SoftwareOption {
                name: "glances",
                tier: Tier::ATier,
                description: "Cross-platform monitoring with web UI and API hooks.",
            },
            SoftwareOption {
                name: "htop",
                tier: Tier::ATier,
                description: "Classic process tree with CPU/memory bars.",
            },
            SoftwareOption {
                name: "bpytop",
                tier: Tier::ATier,
                description: "Python-based, highly configurable brain candy.",
            },
            SoftwareOption {
                name: "gotop",
                tier: Tier::ATier,
                description: "Terminal dashboard with sparklines and disk stats.",
            },
        ],
    },
    SoftwareCategory {
        label: "Browser",
        options: &[
            SoftwareOption {
                name: "Brave",
                tier: Tier::STier,
                description: "Syncable, privacy-first Chromium with built-in rewards blocker.",
            },
            SoftwareOption {
                name: "Librewolf",
                tier: Tier::ATier,
                description: "Firefox fork tuned for privacy and long-term support.",
            },
            SoftwareOption {
                name: "Vivaldi",
                tier: Tier::ATier,
                description: "Highly customizable Chromium with tab stacking.",
            },
            SoftwareOption {
                name: "Firefox",
                tier: Tier::ATier,
                description: "Stable, open-source, and well-integrated with Linux.",
            },
            SoftwareOption {
                name: "Chromium",
                tier: Tier::ATier,
                description: "Vanilla upstream build for WebKit-esque compatibility.",
            },
        ],
    },
    SoftwareCategory {
        label: "Media Player",
        options: &[
            SoftwareOption {
                name: "MPV",
                tier: Tier::STier,
                description: "Lightweight, scriptable player optimized for formats.",
            },
            SoftwareOption {
                name: "VLC",
                tier: Tier::ATier,
                description: "Feature-packed player with codec universe.",
            },
            SoftwareOption {
                name: "SMPlayer",
                tier: Tier::ATier,
                description: "Qt front-end for MPlayer with GUI playlists.",
            },
            SoftwareOption {
                name: "Celluloid",
                tier: Tier::ATier,
                description: "GTK frontend for mpv with GNOME polish.",
            },
            SoftwareOption {
                name: "MPlayer",
                tier: Tier::ATier,
                description: "The veteran Linux media workhorse.",
            },
        ],
    },
    SoftwareCategory {
        label: "HTPC",
        options: &[
            SoftwareOption {
                name: "Kodi",
                tier: Tier::STier,
                description: "All-in-one home theater experience with remote controls.",
            },
            SoftwareOption {
                name: "Plex Media Server",
                tier: Tier::ATier,
                description: "Streaming server with remote library access.",
            },
            SoftwareOption {
                name: "Jellyfin",
                tier: Tier::ATier,
                description: "Self-hosted, open-source Plex alternative.",
            },
            SoftwareOption {
                name: "Emby",
                tier: Tier::ATier,
                description: "Hybrid media server with device streaming.",
            },
            SoftwareOption {
                name: "OSMC",
                tier: Tier::ATier,
                description: "Debian-based Kodi distro for Raspberry Pi theater rigs.",
            },
        ],
    },
    SoftwareCategory {
        label: "VPN",
        options: &[
            SoftwareOption {
                name: "WireGuard",
                tier: Tier::STier,
                description: "Simple, fast, modern VPN protocol with lean config.",
            },
            SoftwareOption {
                name: "OpenVPN",
                tier: Tier::ATier,
                description: "Battle-tested, widely compatible tunnel with TLS.",
            },
            SoftwareOption {
                name: "Tailscale",
                tier: Tier::ATier,
                description: "Mesh VPN built on WireGuard with cloud coordination.",
            },
            SoftwareOption {
                name: "StrongSwan",
                tier: Tier::ATier,
                description: "IPsec daemon for enterprise-grade tunnels.",
            },
            SoftwareOption {
                name: "OpenConnect",
                tier: Tier::ATier,
                description: "Cisco-compatible client for SSL VPN gateways.",
            },
        ],
    },
    SoftwareCategory {
        label: "Firewall",
        options: &[
            SoftwareOption {
                name: "nftables",
                tier: Tier::STier,
                description: "Modern packet filtering framework to replace iptables.",
            },
            SoftwareOption {
                name: "firewalld",
                tier: Tier::ATier,
                description: "zone-based firewall wrapper around nftables/iptables.",
            },
            SoftwareOption {
                name: "ufw",
                tier: Tier::ATier,
                description: "Ubuntu-friendly iptables front-end for quick rules.",
            },
            SoftwareOption {
                name: "iptables",
                tier: Tier::ATier,
                description: "Classic kernel packet filter for legacy scripts.",
            },
            SoftwareOption {
                name: "Shorewall",
                tier: Tier::ATier,
                description: "High-level configuration tool layered over iptables.",
            },
        ],
    },
    SoftwareCategory {
        label: "Backup",
        options: &[
            SoftwareOption {
                name: "rclone + borg",
                tier: Tier::STier,
                description: "Sync clouds with rclone and snapshot with borg.",
            },
            SoftwareOption {
                name: "restic",
                tier: Tier::ATier,
                description: "Content-addressed backups with encryption and snapshots.",
            },
            SoftwareOption {
                name: "duplicacy",
                tier: Tier::ATier,
                description: "Lock-free dedup and chunking for multi-platform servers.",
            },
            SoftwareOption {
                name: "duplicity",
                tier: Tier::ATier,
                description: "GPG-backed incremental backup tool for remote stores.",
            },
            SoftwareOption {
                name: "borgmatic",
                tier: Tier::ATier,
                description: "Wrapper for borg backups with policy automation.",
            },
        ],
    },
];

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
