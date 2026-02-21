//! 1984 Retro-Station Menus — Numbered prompts and station aesthetics.

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use crate::software_catalog::SOFTWARE_CATEGORIES;
use crate::tui::app::{ModuleState, TuiApp, MODULE_LABELS};
use crate::tui::theme;

const WELCOME_BANNER: &str = r"
   _______________________________________
  /                                       \
  |  * * *  MASH BOOT LOADER v1.0  * * *  |
  |                                       |
  |  STATION STATUS: READY                |
  |  IO MODULES:     LOADED               |
  |  PHOSPHOR:       STABILIZED           |
  \_______________________________________/
";

// ── Shared helpers ────────────────────────────────────────────────────────────

fn centered_rect(width_pct: u16, height_pct: u16, area: Rect) -> Rect {
    let vchunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height_pct) / 2),
            Constraint::Percentage(height_pct),
            Constraint::Percentage((100 - height_pct) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_pct) / 2),
            Constraint::Percentage(width_pct),
            Constraint::Percentage((100 - width_pct) / 2),
        ])
        .split(vchunks[1])[1]
}

fn station_block<'a>(title: &'a str) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(theme::outer_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(format!(" {title} "), theme::title_style()))
        .style(theme::default_style())
}

fn command_prompt_line(
    label: impl Into<String>,
    index: usize,
    selected: bool,
) -> ListItem<'static> {
    let label: String = label.into();
    if selected {
        ListItem::new(Line::from(vec![
            Span::styled(format!("  [{index}] > "), theme::selected_style()),
            Span::styled(label, theme::selected_style()),
        ]))
    } else {
        ListItem::new(Line::from(vec![
            Span::styled(format!("   {index}  . "), theme::default_style()),
            Span::styled(label, theme::default_style()),
        ]))
    }
}

// ── Welcome screen ────────────────────────────────────────────────────────────

pub fn draw_welcome(f: &mut Frame, area: Rect, _app: &TuiApp) {
    let block = station_block("MASH OPERATING SYSTEM");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(inner);

    // Banner
    let banner = Paragraph::new(WELCOME_BANNER)
        .style(theme::accent_style())
        .alignment(Alignment::Center);
    f.render_widget(banner, chunks[0]);

    // Tagline
    let tagline = Paragraph::new("STATION IDENTIFICATION: FORGE_TAVERN_PI_4B")
        .style(theme::title_style())
        .alignment(Alignment::Center);
    f.render_widget(tagline, chunks[1]);

    // Prompt
    let prompt = Paragraph::new(Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(
            "COMMAND > BOOT_SYSTEM",
            theme::success_style(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "PRESS ENTER TO COMMENCE INITIALIZATION...",
            theme::dim_style(),
        )),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(prompt, chunks[2]);

    // Footer
    let footer = Paragraph::new(Span::styled(
        "OS REV 0.1.6 (C) 1984 MYTHIC ASSEMBLY",
        theme::dim_style(),
    ))
    .alignment(Alignment::Right);
    f.render_widget(footer, chunks[3]);
}

// ── Distro select ─────────────────────────────────────────────────────────────

pub fn draw_distro_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(70, 60, area);
    let block = station_block("DRIVER CONFIGURATION");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("SELECT TARGET ARCHITECTURE:"), chunks[0]);

    let items: Vec<ListItem> = app
        .drivers
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let label = format!("{} ({})", d.name(), d.description());
            command_prompt_line(label, i + 1, i == app.menu_cursor)
        })
        .collect();

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[1]);

    let prompt = Paragraph::new(vec![Line::from(vec![
        Span::styled("COMMAND > ", theme::success_style()),
        Span::styled(format!("{}", app.menu_cursor + 1), theme::selected_style()),
        Span::styled(
            "_",
            theme::success_style().add_modifier(Modifier::SLOW_BLINK),
        ),
    ])]);
    f.render_widget(prompt, chunks[2]);
}

// ── Module select ─────────────────────────────────────────────────────────────

pub fn draw_module_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(70, 60, area);
    let block = station_block("SUBSYSTEM MODULES");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("TOGGLE SUBSYSTEM LOAD STATUS:"), chunks[0]);

    let checked = module_checks(&app.modules);
    let items: Vec<ListItem> = MODULE_LABELS
        .iter()
        .enumerate()
        .map(|(i, (label, _))| {
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

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[1]);

    let prompt = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            "SPACE:TOGGLE  ENTER:CONFIRM  COMMAND > ",
            theme::success_style(),
        ),
        Span::styled(
            "_",
            theme::success_style().add_modifier(Modifier::SLOW_BLINK),
        ),
    ])]);
    f.render_widget(prompt, chunks[2]);
}

fn module_checks(m: &ModuleState) -> [bool; 3] {
    [m.enable_argon, m.enable_p10k, m.docker_data_root]
}

// ── Profile select ────────────────────────────────────────────────────────────

const PROFILE_LABELS: &[(&str, &str)] = &[
    ("MINIMAL", "CORE RUNES ONLY"),
    ("DEVELOPER", "FULL FORGE SUITE"),
    ("ARCHIVE", "COMPLETE SYSTEM DATA"),
];

pub fn draw_profile_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(70, 60, area);
    let block = station_block("SYSTEM PROFILE");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new("SELECT SYSTEM OPERATIONAL DEPTH:"),
        chunks[0],
    );

    let items: Vec<ListItem> = PROFILE_LABELS
        .iter()
        .enumerate()
        .map(|(i, (name, desc))| {
            let label = format!("{} -- {}", name, desc);
            command_prompt_line(label, i + 1, i == app.menu_cursor)
        })
        .collect();

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[1]);

    let prompt = Paragraph::new(vec![Line::from(vec![
        Span::styled("COMMAND > ", theme::success_style()),
        Span::styled(format!("{}", app.menu_cursor + 1), theme::selected_style()),
        Span::styled(
            "_",
            theme::success_style().add_modifier(Modifier::SLOW_BLINK),
        ),
    ])]);
    f.render_widget(prompt, chunks[2]);
}

// ── Theme select ─────────────────────────────────────────────────────────────

const THEME_LABELS: &[&str] = &[
    "1984 RETRO-STATION (GREEN/AMBER)",
    "STATION + VISUAL ARCHIVE (6000+)",
    "NO AESTHETIC OVERRIDE",
];

pub fn draw_theme_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(70, 60, area);
    let block = station_block("AESTHETIC PROTOCOL");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("SELECT VISUAL OUTPUT MODE:"), chunks[0]);

    let items: Vec<ListItem> = THEME_LABELS
        .iter()
        .enumerate()
        .map(|(i, label)| command_prompt_line(*label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[1]);

    let prompt = Paragraph::new(vec![Line::from(vec![
        Span::styled("COMMAND > ", theme::success_style()),
        Span::styled(format!("{}", app.menu_cursor + 1), theme::selected_style()),
        Span::styled(
            "_",
            theme::success_style().add_modifier(Modifier::SLOW_BLINK),
        ),
    ])]);
    f.render_widget(prompt, chunks[2]);
}

// ── Software mode select ─────────────────────────────────────────────────────

pub fn draw_software_mode_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(70, 60, area);
    let block = station_block("SOFTWARE REPOSITORY");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner);

    f.render_widget(Paragraph::new("SELECT INSTALLATION METHOD:"), chunks[0]);

    let options = ["AUTOMATIC S-TIER ALLOCATION", "MANUAL CATEGORY SELECTION"];
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, label)| command_prompt_line(*label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[1]);

    let prompt = Paragraph::new(vec![Line::from(vec![
        Span::styled("COMMAND > ", theme::success_style()),
        Span::styled(format!("{}", app.menu_cursor + 1), theme::selected_style()),
        Span::styled(
            "_",
            theme::success_style().add_modifier(Modifier::SLOW_BLINK),
        ),
    ])]);
    f.render_widget(prompt, chunks[2]);
}

// ── Software tier select (per category) ──────────────────────────────────────

pub fn draw_software_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(80, 70, area);
    let block = station_block("REPOSITORY AUDIT");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner);

    let category = SOFTWARE_CATEGORIES
        .get(app.software_category_idx)
        .map(|cat| cat.label)
        .unwrap_or("UNKNOWN");

    f.render_widget(
        Paragraph::new(format!(
            "CAT: {} ({}/{})",
            category.to_uppercase(),
            app.software_category_idx + 1,
            SOFTWARE_CATEGORIES.len()
        )),
        chunks[0],
    );

    let options = SOFTWARE_CATEGORIES
        .get(app.software_category_idx)
        .map(|cat| cat.options)
        .unwrap_or(&[]);

    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let label = format!("{} [{}]", opt.name.to_uppercase(), opt.tier);
            command_prompt_line(label, i + 1, i == app.menu_cursor)
        })
        .collect();
    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[1]);

    let prompt = Paragraph::new(vec![Line::from(vec![
        Span::styled("COMMAND > ", theme::success_style()),
        Span::styled(format!("{}", app.menu_cursor + 1), theme::selected_style()),
        Span::styled(
            "_",
            theme::success_style().add_modifier(Modifier::SLOW_BLINK),
        ),
    ])]);
    f.render_widget(prompt, chunks[2]);
}

// ── Pre-install confirm ───────────────────────────────────────────────────────

pub fn draw_pre_install_confirm(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(70, 70, area);
    let block = station_block("STATION COMMITMENT");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled("PROVISIONING MANIFEST:", theme::title_style())),
        Line::from(""),
    ];

    lines.push(Line::from(vec![
        Span::styled("  CORE:     ", theme::dim_style()),
        Span::styled(
            app.drivers[app.selected_driver_idx].name().to_uppercase(),
            theme::success_style(),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  PROFILE:  ", theme::dim_style()),
        Span::styled(
            format!("{:?}", app.profile_idx).to_uppercase(),
            theme::success_style(),
        ),
    ]));

    if app.dry_run {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  ! SIMULATION MODE ACTIVE !",
            theme::error_style(),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "CONFIRM EXECUTION? (Y/N)",
        theme::accent_style(),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("COMMAND > ", theme::success_style()),
        Span::styled(
            "_",
            theme::success_style().add_modifier(Modifier::SLOW_BLINK),
        ),
    ]));

    let para = Paragraph::new(Text::from(lines))
        .style(theme::default_style())
        .wrap(Wrap { trim: false });
    f.render_widget(para, inner);
}

// ── Mid-install confirm ─────────────────────────

pub fn draw_mid_install_confirm(f: &mut Frame, area: Rect, app: &TuiApp) {
    let Some(state) = &app.confirm_state else {
        return;
    };

    let popup = centered_rect(60, 30, area);
    let block = station_block("USER INTERVENTION REQUIRED");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let prompt_para = Paragraph::new(state.prompt.to_uppercase())
        .style(theme::default_style())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(prompt_para, inner);
}

// ── Mid-install password prompt ────────────────────────────────────────────

pub fn draw_password_prompt(
    f: &mut Frame,
    area: Rect,
    _app: &TuiApp,
    state: &crate::tui::app::PasswordState,
) {
    let popup = centered_rect(60, 30, area);
    let block = station_block("SECURITY CLEARANCE");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let password_display = "*".repeat(state.password.len());
    let prompt = Text::from(vec![
        Line::from(vec![Span::styled(
            "AUTHENTICATION REQUIRED:",
            theme::default_style(),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("PASSWORD: ", theme::accent_style()),
            Span::styled(password_display, theme::success_style()),
            Span::styled(
                "_",
                theme::success_style().add_modifier(Modifier::SLOW_BLINK),
            ),
        ]),
    ]);

    let para = Paragraph::new(prompt).alignment(Alignment::Center);
    f.render_widget(para, inner);
}
