use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use crate::tui::app::{TuiApp, MODULE_LABELS};
use crate::tui::menus::helpers::{
    command_prompt_line, draw_navigation_info, module_checks, station_block,
};
use crate::tui::theme;

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

pub fn draw_module_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("SUBSYSTEM_MODULATION");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
            Constraint::Length(3),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("TOGGLE SUBSYSTEM LOAD STATUS:"), chunks[0]);

    let checked = module_checks(&app.modules);
    let items: Vec<ListItem> = MODULE_LABELS
        .iter()
        .enumerate()
        .map(|(i, (label, _)): (usize, &(&str, &str))| {
            let status = if checked[i] { "LOADED" } else { "VOID  " };
            let selected = i == app.menu_cursor;
            let style = if selected {
                theme::selected_style()
            } else {
                theme::default_style()
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("  [{}] ", status), style),
                Span::styled(label.to_uppercase(), style),
            ]))
        })
        .collect();

    // Add 'Confirm' as last item
    let confirm_style = if app.menu_cursor == 3 {
        theme::warning_style().add_modifier(Modifier::BOLD)
    } else {
        theme::warning_style()
    };
    let mut items = items;
    items.push(ListItem::new(Line::from(vec![Span::styled(
        "  [>>>>] PROCEED TO DESKTOP CONFIG",
        confirm_style,
    )])));

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    // Help for current item
    if app.menu_cursor < 3 {
        let help = MODULE_LABELS[app.menu_cursor].1;
        f.render_widget(
            Paragraph::new(format!("INTEL: {}", help)).style(theme::dim_style()),
            chunks[2],
        );
    }

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
        ("None", "Maintain system defaults"),
    ];

    let items: Vec<ListItem> = themes
        .iter()
        .enumerate()
        .map(|(i, (label, _))| command_prompt_line(*label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

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
        "None (CLI Only)",
    ];

    let items: Vec<ListItem> = des
        .iter()
        .enumerate()
        .map(|(i, label)| command_prompt_line(*label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

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
            Span::styled("DRIVER:  ", theme::dim_style()),
            Span::styled(
                app.drivers[app.selected_driver_idx].name().to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("PROFILE: ", theme::dim_style()),
            Span::styled(
                format!("{:?}", app.profile_level()).to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("THEME:   ", theme::dim_style()),
            Span::styled(
                app.theme_plan_label().to_uppercase(),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("SOFTWARE:", theme::dim_style()),
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

        for (cat, prog) in &preset.software_plan.selections {
            let cat: &String = cat;
            detail_lines.push(Line::from(format!(
                "  + {:<12} : {}",
                cat.to_uppercase(),
                prog
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
    let block = station_block("SYSTEM_PEDIGREE_SUMMARY");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
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
            "HARDWARE PROFILE:",
            theme::accent_style(),
        )));
        summary.push(Line::from(format!(
            "  {} Cores detected",
            profile.cpu.physical_cores
        )));
        summary.push(Line::from(format!(
            "  {:.1} GB Memory total",
            profile.memory.ram_total_kb as f32 / 1024.0 / 1024.0
        )));
    }

    f.render_widget(
        Paragraph::new(summary).alignment(Alignment::Center),
        chunks[0],
    );

    let prompt = Paragraph::new("PRESS [ENTER] TO PROCEED TO WARDROBE")
        .style(theme::warning_style())
        .alignment(Alignment::Center);
    f.render_widget(prompt, chunks[1]);

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
