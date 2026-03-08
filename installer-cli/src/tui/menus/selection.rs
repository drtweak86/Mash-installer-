use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use crate::tui::app::TuiApp;
use crate::tui::menus::helpers::{command_prompt_line, draw_navigation_info, station_block};
use crate::tui::theme;
use installer_core::desktop::DesktopEnvironment;

pub fn draw_distro_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("DISTRO_SIGIL_SELECTION");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new("CHOOSE YOUR OPERATING ENVIRONMENT:"),
        chunks[0],
    );

    let items: Vec<ListItem> = app
        .drivers
        .iter()
        .enumerate()
        .map(|(i, d): (usize, &&dyn installer_core::DistroDriver)| {
            let label = format!("{} ({})", d.name(), d.description());
            command_prompt_line(label, i + 1, i == app.menu_cursor)
        })
        .collect();

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[1]);

    draw_navigation_info(f, area, app);
}

pub fn draw_profile_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("FORGE_PROFILE_LEVEL");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("SELECT SYSTEM COMPLEXITY:"), chunks[0]);

    let options = [
        ("Minimal", "Lean system, essential tools only"),
        ("Developer", "Full dev toolchain, docker, and polish"),
        ("Full station", "All the bells, whistles, and neon"),
    ];

    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, (label, _))| command_prompt_line(*label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    draw_navigation_info(f, area, app);
}

pub fn draw_theme_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("AESTHETIC_CALIBRATION");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("SELECT VISUAL OVERLAY:"), chunks[0]);

    let themes = [
        ("Retro Only", "Classic BBC/UNIX styling"),
        ("Retro + Wallpapers", "Adds the MASH wallpaper pack"),
        ("Catppuccin", "Soothing pastel mocha theme"),
        ("Nord", "Arctic, north-bluish palette"),
        ("Dracula", "Dark theme for night owls"),
        ("None", "Maintain system defaults"),
    ];

    let items: Vec<ListItem> = themes
        .iter()
        .enumerate()
        .map(|(i, (label, _))| command_prompt_line(*label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    if let Some((_, desc)) = themes.get(app.menu_cursor) {
        f.render_widget(
            Paragraph::new(format!("INTEL: {}", desc)).style(theme::dim_style()),
            chunks[2],
        );
    }

    draw_navigation_info(f, area, app);
}

pub fn draw_de_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("DESKTOP_ENVIRONMENT");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("CHOOSE GUI SHELL:"), chunks[0]);

    let des = [
        "Gnome",
        "KDE Plasma",
        "XFCE",
        "LXQt",
        "Mate",
        "Cinnamon",
        "Budgie",
        "Enlightenment",
        "LXDE",
        "COSMIC (Epoch)",
        "Hyprland (Wayland)",
        "None (CLI Only)",
    ];

    let items: Vec<ListItem> = des
        .iter()
        .enumerate()
        .map(|(i, label)| command_prompt_line(*label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    if let Some(_label) = des.get(app.menu_cursor) {
        let de = match app.menu_cursor {
            0 => DesktopEnvironment::Gnome,
            1 => DesktopEnvironment::Kde,
            2 => DesktopEnvironment::Xfce,
            3 => DesktopEnvironment::Lxqt,
            4 => DesktopEnvironment::Mate,
            5 => DesktopEnvironment::Cinnamon,
            6 => DesktopEnvironment::Budgie,
            7 => DesktopEnvironment::Enlightenment,
            8 => DesktopEnvironment::Lxde,
            9 => DesktopEnvironment::Cosmic,
            10 => DesktopEnvironment::Hyprland,
            _ => DesktopEnvironment::None,
        };

        let mut help_text = format!("INTEL: {}", de.description());
        if let Some(warn) = de.pi_warning(app.platform_info.pi_model.is_some()) {
            help_text.push_str(&format!("\nWARNING: {}", warn));
        }

        f.render_widget(
            Paragraph::new(help_text).style(theme::dim_style()),
            chunks[2],
        );
    }

    draw_navigation_info(f, area, app);
}

pub fn draw_argon_config(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("ARGON_ONE_SETUP");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("CONFIGURE ARGON ONE CASE:"), chunks[0]);

    let options = [
        format!(
            "Quiet Profile     {}",
            if app.argon.enabled && app.argon.cooling_profile == "Quiet" {
                "[X]"
            } else {
                "[ ]"
            }
        ),
        format!(
            "Balanced Profile  {}",
            if app.argon.enabled && app.argon.cooling_profile == "Balanced" {
                "[X]"
            } else {
                "[ ]"
            }
        ),
        format!(
            "Performance Profile {}",
            if app.argon.enabled && app.argon.cooling_profile == "Performance" {
                "[X]"
            } else {
                "[ ]"
            }
        ),
        "CONFIRM AND CONTINUE".to_string(),
    ];

    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, label)| command_prompt_line(label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    f.render_widget(
        Paragraph::new("INTEL: Argon One scripts enable fan control and power button logic.")
            .style(theme::dim_style()),
        chunks[2],
    );

    draw_navigation_info(f, area, app);
}

pub fn draw_docker_config(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("DOCKER_PROTOCOL");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("CONFIGURE DOCKER DAEMON:"), chunks[0]);

    let options = [
        format!(
            "Relocate Data-Root  {}",
            if app.docker.enabled { "[X]" } else { "[ ]" }
        ),
        "CONFIRM AND CONTINUE".to_string(),
    ];

    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, label)| command_prompt_line(label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    f.render_widget(
        Paragraph::new("INTEL: Docker data-root can be relocated to staging for portability.")
            .style(theme::dim_style()),
        chunks[2],
    );

    draw_navigation_info(f, area, app);
}

pub fn draw_protocol_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("DISPLAY_PROTOCOL");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("SELECT COORDINATION PROTOCOL:"), chunks[0]);

    let protos = ["Auto (Recommended)", "Wayland", "X11"];

    let items: Vec<ListItem> = protos
        .iter()
        .enumerate()
        .map(|(i, label)| command_prompt_line(*label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    draw_navigation_info(f, area, app);
}

pub fn draw_de_confirm(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("GUI_CONFIRMATION");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(inner);

    let de_name = app
        .desktop_environment
        .map(|de| format!("{:?}", de))
        .unwrap_or_else(|| "None".to_string());
    let proto_name = format!("{:?}", app.display_protocol);

    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("ENVIRONMENT: ", theme::dim_style()),
            Span::styled(de_name.to_uppercase(), theme::title_style()),
        ]),
        Line::from(vec![
            Span::styled("PROTOCOL:    ", theme::dim_style()),
            Span::styled(proto_name.to_uppercase(), theme::title_style()),
        ]),
    ])
    .alignment(Alignment::Center);
    f.render_widget(info, chunks[0]);

    let prompt = Paragraph::new("CONFIRM THIS GUI CONFIGURATION?")
        .style(theme::warning_style())
        .alignment(Alignment::Center);
    f.render_widget(prompt, chunks[1]);

    let yes_style = if app.menu_cursor == 0 {
        theme::selected_style().add_modifier(Modifier::BOLD)
    } else {
        theme::default_style()
    };
    let no_style = if app.menu_cursor == 1 {
        theme::selected_style().add_modifier(Modifier::BOLD)
    } else {
        theme::default_style()
    };

    let btns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(
        Paragraph::new("[ YES ]")
            .style(yes_style)
            .alignment(Alignment::Right),
        btns[0],
    );
    f.render_widget(
        Paragraph::new("  [ NO ]")
            .style(no_style)
            .alignment(Alignment::Left),
        btns[1],
    );
}

pub fn draw_pre_install_confirm(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("FINAL_MISSION_BRIEFING");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(inner);

    let mut lines = vec![
        Line::from(vec![
            Span::styled("DRIVER:      ", theme::dim_style()),
            Span::styled(
                app.drivers[app.selected_driver_idx].name().to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("PROFILE:     ", theme::dim_style()),
            Span::styled(
                format!("{:?}", app.profile_level()).to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("ENVIRONMENT: ", theme::dim_style()),
            Span::styled(
                format!("{:?}", app.environment()).to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("DESKTOP:     ", theme::dim_style()),
            Span::styled(
                format!(
                    "{:?} ({:?})",
                    app.desktop_environment.unwrap_or(DesktopEnvironment::None),
                    app.display_protocol
                )
                .to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("THEME:       ", theme::dim_style()),
            Span::styled(
                format!("{:?}", app.theme_plan).to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("DOTFILES:    ", theme::dim_style()),
            if app.chezmoi_enabled {
                Span::styled(
                    format!("CHEZMOI ({})", app.chezmoi_repo),
                    theme::success_style(),
                )
            } else {
                Span::styled("SKIPPED", theme::dim_style())
            },
        ]),
        Line::from(vec![
            Span::styled("SOFTWARE:    ", theme::dim_style()),
            Span::styled(
                app.software_plan_label().to_uppercase(),
                theme::success_style(),
            ),
        ]),
    ];

    if app.dry_run {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "*** DRY RUN MODE ACTIVE ***",
            theme::warning_style(),
        )));
    }

    let summary = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(summary, chunks[0]);

    let prompt = Paragraph::new("INITIALIZE SYSTEM OVERWRITE?")
        .style(theme::warning_style())
        .alignment(Alignment::Center);
    f.render_widget(prompt, chunks[1]);

    let yes_style = if app.menu_cursor == 0 {
        theme::selected_style().add_modifier(Modifier::BOLD)
    } else {
        theme::default_style()
    };
    let no_style = if app.menu_cursor == 1 {
        theme::selected_style().add_modifier(Modifier::BOLD)
    } else {
        theme::default_style()
    };

    let btns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(
        Paragraph::new("[ YES ]")
            .style(yes_style)
            .alignment(Alignment::Right),
        btns[0],
    );
    f.render_widget(
        Paragraph::new("  [ NO ]")
            .style(no_style)
            .alignment(Alignment::Left),
        btns[1],
    );
}

pub fn draw_font_prep(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("FONT_PREPARATION");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner);

    let text = vec![
        Line::from("STATION_01 IS PREPARING NEON TYPEFACES."),
        Line::from(""),
        Line::from(Span::styled(
            "JETBRAINS MONO / NERD FONTS REQUIRED",
            theme::warning_style(),
        )),
        Line::from(""),
        Line::from("THESE WILL BE DEPLOYED DURING INSTALLATION."),
    ];

    f.render_widget(Paragraph::new(text).alignment(Alignment::Center), chunks[0]);

    let prompt = Paragraph::new("PRESS [ENTER] TO ACKNOWLEDGE")
        .style(theme::warning_style())
        .alignment(Alignment::Center);
    f.render_widget(prompt, chunks[1]);

    draw_navigation_info(f, area, app);
}

pub fn draw_wardrobe(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("THE_WARDROBE_PRESETS");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(inner);

    // Left: Preset list
    let mut items: Vec<ListItem> = app
        .available_presets
        .iter()
        .enumerate()
        .map(|(i, p)| command_prompt_line(&p.name, i + 1, i == app.menu_cursor))
        .collect();

    // Add 'Back'
    let back_selected = app.menu_cursor == app.available_presets.len();
    items.push(command_prompt_line(
        "Back to summary",
        app.available_presets.len() + 1,
        back_selected,
    ));

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[0]);

    // Right: Preset details
    if app.menu_cursor < app.available_presets.len() {
        let preset = &app.available_presets[app.menu_cursor];
        let mut detail_lines = vec![
            Line::from(Span::styled(
                preset.name.to_uppercase(),
                theme::title_style(),
            )),
            Line::from(preset.description.to_uppercase()),
            Line::from(""),
            Line::from(Span::styled("COMPONENTS:", theme::accent_style())),
        ];

        for (cat, progs) in &preset.software_plan.selections {
            detail_lines.push(Line::from(format!(
                "  + {:<12} : {}",
                cat.to_string().to_uppercase(),
                progs.join(", ")
            )));
        }

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::from(Span::styled("TWEAKS:", theme::accent_style())));
        for tweak in &preset.tweaks {
            let tweak: &String = tweak;
            detail_lines.push(Line::from(format!(
                "  > {}",
                tweak.replace('_', " ").to_uppercase()
            )));
        }

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::from(Span::styled("THEME ID:", theme::accent_style())));
        detail_lines.push(Line::from(format!(
            "  @ {}",
            preset.theme_id.to_uppercase()
        )));

        let details = Paragraph::new(detail_lines)
            .style(theme::default_style())
            .wrap(Wrap { trim: false });
        f.render_widget(details, chunks[1]);
    }

    draw_navigation_info(f, area, app);
}

pub fn draw_system_summary(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("SYSTEM_PEDIGREE_RESULTS");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    let mut summary = vec![
        Line::from(vec![
            Span::styled("OS:      ", theme::dim_style()),
            Span::styled(
                app.platform_info.distro.to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("ARCH:    ", theme::dim_style()),
            Span::styled(
                app.platform_info.arch.to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("MODEL:   ", theme::dim_style()),
            Span::styled(
                app.platform_info
                    .pi_model
                    .clone()
                    .unwrap_or_else(|| "Generic Station".to_string())
                    .to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("CPU:     ", theme::dim_style()),
            Span::styled(
                app.platform_info.cpu_model.to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("RAM:     ", theme::dim_style()),
            Span::styled(
                format!("{:.1} GB", app.platform_info.ram_total_gb),
                theme::success_style(),
            ),
        ]),
    ];

    if let Some(ref profile) = app.system_profile {
        summary.push(Line::from(""));
        summary.push(Line::from(Span::styled(
            "HARDWARE LANDSCAPE:",
            theme::accent_style(),
        )));
        summary.push(Line::from(format!(
            "  {} PHYSICAL CORES DETECTED",
            profile.cpu.physical_cores
        )));
        if profile.memory.zram_total_kb > 0 {
            summary.push(Line::from(format!(
                "  {:.1} GB ZRAM OPTIMIZED",
                profile.memory.zram_total_kb as f32 / 1024.0 / 1024.0
            )));
        }
    }

    f.render_widget(
        Paragraph::new(summary).alignment(Alignment::Center),
        chunks[0],
    );

    // BARD'S WISDOM (Advice Engine)
    let mut wisdom = vec![Line::from(Span::styled(
        "── BARD'S ANCESTRAL WISDOM ──────────────────────────",
        theme::title_style(),
    ))];

    if let Some(ref profile) = app.system_profile {
        let engine = installer_core::advice::AdviceEngine::default();
        let options = app.build_options();
        let advice = engine.run(
            profile,
            &installer_core::UserOptionsContext::from_options(&options),
        );

        if advice.is_empty() {
            wisdom.push(Line::from(
                "  THE FORGE IS OPTIMAL. NO CRITICAL OMENS DETECTED.",
            ));
        } else {
            for entry in advice {
                let color = match entry.level {
                    installer_core::advice::Severity::Critical => theme::error_style(),
                    installer_core::advice::Severity::Warning => theme::warning_style(),
                    installer_core::advice::Severity::Info => theme::accent_style(),
                };
                wisdom.push(Line::from(vec![
                    Span::styled(format!("  [{:?}] ", entry.level).to_uppercase(), color),
                    Span::styled(entry.message.to_uppercase(), theme::default_style()),
                ]));
            }
        }
    } else {
        wisdom.push(Line::from("  WAITING FOR STATION TELEMETRY..."));
    }

    f.render_widget(Paragraph::new(wisdom).wrap(Wrap { trim: false }), chunks[1]);

    let prompt = Paragraph::new("PRESS [ENTER] TO ACKNOWLEDGE WISDOM")
        .style(theme::warning_style())
        .alignment(Alignment::Center);
    f.render_widget(prompt, chunks[2]);

    draw_navigation_info(f, area, app);
}

pub fn draw_auth_screen(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("STATION_AUTHORIZATION");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let Some(ref auth) = app.auth_state else {
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(inner);

    let info = Paragraph::new(vec![
        Line::from("A SUBSYSTEM REQUIRES EXTERNAL AUTHORIZATION:"),
        Line::from(""),
        Line::from(Span::styled(
            format!("{:?}", auth.auth_type).to_uppercase(),
            theme::warning_style(),
        )),
        Line::from(""),
        Line::from("PLEASE GRANT OR DENY PERMISSION TO PROCEED."),
    ])
    .alignment(Alignment::Center);
    f.render_widget(info, chunks[0]);

    let yes_style = if auth.selected {
        theme::selected_style().add_modifier(Modifier::BOLD)
    } else {
        theme::default_style()
    };
    let no_style = if !auth.selected {
        theme::selected_style().add_modifier(Modifier::BOLD)
    } else {
        theme::default_style()
    };

    let btns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(
        Paragraph::new("[ GRANT ]")
            .style(yes_style)
            .alignment(Alignment::Right),
        btns[0],
    );
    f.render_widget(
        Paragraph::new("  [ DENY ]")
            .style(no_style)
            .alignment(Alignment::Left),
        btns[1],
    );
}

pub fn draw_password_screen(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("STATION_SECURITY_GATE");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let Some(ref pw) = app.password_state else {
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(inner);

    let info = Paragraph::new(vec![
        Line::from("SUDO PRIVILEGES ARE REQUIRED FOR THIS OPERATION."),
        Line::from(""),
        Line::from("INPUT STATION SECURITY CREDENTIALS:"),
    ])
    .alignment(Alignment::Center);
    f.render_widget(info, chunks[0]);

    let stars = "*".repeat(pw.password.len());
    let input = Paragraph::new(format!("  [{}]", stars))
        .style(theme::warning_style())
        .alignment(Alignment::Center);
    f.render_widget(input, chunks[1]);

    f.render_widget(
        Paragraph::new("PRESS [ENTER] TO SUBMIT OR [ESC] TO ABORT")
            .style(theme::dim_style())
            .alignment(Alignment::Center),
        chunks[2],
    );
}

pub fn draw_chezmoi_config(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("DOTFILE_RESTORATION_SIGIL");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3), // Help text
            Constraint::Length(3),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new("CONFIGURE CHEZMOI DOTFILE RECOVERY:"),
        chunks[0],
    );

    let mut items = Vec::new();
    items.push(command_prompt_line(
        format!(
            "Enable Chezmoi: [{}]",
            if app.chezmoi_enabled { "X" } else { " " }
        ),
        1,
        app.menu_cursor == 0,
    ));

    if app.chezmoi_enabled {
        items.push(command_prompt_line(
            format!("Repository URL: {}", app.chezmoi_repo),
            2,
            app.menu_cursor == 1,
        ));
        items.push(command_prompt_line(
            format!("Branch (Optional): {}", app.chezmoi_branch),
            3,
            app.menu_cursor == 2,
        ));
        items.push(command_prompt_line(
            "Proceed to Summary".to_string(),
            4,
            app.menu_cursor == 3,
        ));
    } else {
        items.push(command_prompt_line(
            "Skip to Summary".to_string(),
            2,
            app.menu_cursor == 1,
        ));
    }

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[1]);

    let help_text = if app.chezmoi_enabled {
        match app.menu_cursor {
            0 => "Toggle dotfile restoration using chezmoi.",
            1 => "Enter the Git repository URL (e.g., https://github.com/user/dotfiles).",
            2 => "Enter the branch to use (optional, leave empty for default).",
            3 => "Save configuration and proceed to the final summary.",
            _ => "Configuring dotfiles...",
        }
    } else {
        match app.menu_cursor {
            0 => "Toggle dotfile restoration using chezmoi.",
            1 => "Skip dotfile restoration and proceed to the summary.",
            _ => "Skipping dotfiles...",
        }
    };

    f.render_widget(
        Paragraph::new(help_text)
            .style(theme::dim_style())
            .wrap(Wrap { trim: true }),
        chunks[2],
    );

    draw_navigation_info(f, area, app);
}
