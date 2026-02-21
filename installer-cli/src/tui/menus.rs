//! Interactive menu screens: Welcome, DistroSelect, ModuleSelect, ProfileSelect, Confirm.

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use crate::software_catalog::SOFTWARE_CATEGORIES;
use crate::tui::app::{ModuleState, TuiApp, MODULE_LABELS};
use crate::tui::theme;

const WELCOME_BANNER: &str = r"
 ███╗   ███╗ █████╗ ███████╗██╗  ██╗
 ████╗ ████║██╔══██╗██╔════╝██║  ██║
 ██╔████╔██║███████║███████╗███████║
 ██║╚██╔╝██║██╔══██║╚════██║██╔══██║
 ██║ ╚═╝ ██║██║  ██║███████║██║  ██║
 ╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝";

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

fn menu_block<'a>(title: &'a str) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(theme::outer_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(format!(" {title} "), theme::title_style()))
        .style(theme::default_style())
}

fn list_item_line(label: impl Into<String>, selected: bool) -> ListItem<'static> {
    let label: String = label.into();
    if selected {
        ListItem::new(Line::from(vec![
            Span::styled(" > ", theme::selected_style()),
            Span::styled(label, theme::selected_style()),
        ]))
    } else {
        ListItem::new(Line::from(vec![
            Span::raw("   "),
            Span::styled(label, theme::default_style()),
        ]))
    }
}

fn hint_line(text: &str) -> Line<'_> {
    Line::from(Span::styled(
        text,
        theme::dim_style().add_modifier(Modifier::ITALIC),
    ))
}

// ── Welcome screen ────────────────────────────────────────────────────────────

pub fn draw_welcome(f: &mut Frame, area: Rect, _app: &TuiApp) {
    let block = menu_block("MASH INSTALLER — FORGE TAVERN EDITION");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    // Banner
    let banner = Paragraph::new(WELCOME_BANNER)
        .style(theme::accent_style())
        .alignment(Alignment::Center);
    f.render_widget(banner, chunks[0]);

    // Tagline
    let tagline = Paragraph::new("Mythic Assembly & Sigil Heuristics · Linux System Installer")
        .style(theme::title_style())
        .alignment(Alignment::Center);
    f.render_widget(tagline, chunks[1]);

    // Prompt
    let prompt = Paragraph::new(Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Press ENTER to begin your quest...",
            theme::success_style().add_modifier(Modifier::BOLD),
        )),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(prompt, chunks[2]);

    // Footer
    let footer = Paragraph::new(Text::from(vec![hint_line(
        "  q = quit   ↑/↓ or j/k = navigate   Enter = select   Space = toggle",
    )]))
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[3]);
}

// ── Distro select ─────────────────────────────────────────────────────────────

pub fn draw_distro_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(60, 70, area);
    let block = menu_block("STEP 1/5 — SELECT DISTRO");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner);

    let items: Vec<ListItem> = app
        .drivers
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let label = format!("{} — {}", d.name(), d.description());
            list_item_line(label, i == app.menu_cursor)
        })
        .collect();

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[0]);

    let hint = Paragraph::new(hint_line("↑/↓ navigate · Enter select · Esc back"))
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[1]);
}

// ── Module select ─────────────────────────────────────────────────────────────

pub fn draw_module_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(60, 70, area);
    let block = menu_block("STEP 3/5 — SELECT OPTIONS");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner);

    let checked = module_checks(&app.modules);
    let items: Vec<ListItem> = MODULE_LABELS
        .iter()
        .enumerate()
        .map(|(i, (label, desc))| {
            let box_glyph = if checked[i] { "[✓]" } else { "[ ]" };
            let selected = i == app.menu_cursor;
            let (box_style, label_style) = if selected {
                (theme::selected_style(), theme::selected_style())
            } else if checked[i] {
                (theme::success_style(), theme::default_style())
            } else {
                (theme::dim_style(), theme::default_style())
            };
            let prefix = if selected { " > " } else { "   " };
            ListItem::new(Line::from(vec![
                Span::styled(
                    prefix,
                    if selected {
                        theme::selected_style()
                    } else {
                        theme::default_style()
                    },
                ),
                Span::styled(box_glyph, box_style),
                Span::styled(format!(" {label} — {desc}"), label_style),
            ]))
            .style(theme::default_style())
        })
        .collect();

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[0]);

    let hint = Paragraph::new(hint_line(
        "↑/↓ navigate · Space toggle · Enter confirm · Esc back",
    ))
    .alignment(Alignment::Center);
    f.render_widget(hint, chunks[1]);
}

fn module_checks(m: &ModuleState) -> [bool; 3] {
    [m.enable_argon, m.enable_p10k, m.docker_data_root]
}

// ── Profile select ────────────────────────────────────────────────────────────

const PROFILE_LABELS: &[(&str, &str)] = &[
    ("Minimal", "Basic tooling only"),
    ("Dev", "Developer packages + shell polish  (recommended)"),
    ("Full", "Everything — the whole enchilada"),
];

pub fn draw_profile_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(60, 60, area);
    let block = menu_block("STEP 2/5 — SELECT PROFILE");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner);

    let items: Vec<ListItem> = PROFILE_LABELS
        .iter()
        .enumerate()
        .map(|(i, (name, desc))| {
            let label = format!("{name} — {desc}");
            list_item_line(label, i == app.menu_cursor)
        })
        .collect();

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[0]);

    let hint = Paragraph::new(hint_line("↑/↓ navigate · Enter select · Esc back"))
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[1]);
}

// ── Theme select ─────────────────────────────────────────────────────────────

const THEME_LABELS: &[&str] = &[
    "BBC/UNIX Retro Theme (i3 + Kitty)",
    "Retro Theme + Wallpaper Pack (6000+)",
    "No theme changes",
];

pub fn draw_theme_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(70, 60, area);
    let block = menu_block("STEP 4/5 — SELECT THEME");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner);

    let items: Vec<ListItem> = THEME_LABELS
        .iter()
        .enumerate()
        .map(|(i, label)| list_item_line(*label, i == app.menu_cursor))
        .collect();

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[0]);

    let hint = Paragraph::new(hint_line("↑/↓ navigate · Enter select · Esc back"))
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[1]);
}

// ── Software mode select ─────────────────────────────────────────────────────

pub fn draw_software_mode_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(70, 60, area);
    let block = menu_block("STEP 5/5 — SOFTWARE TIERS");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner);

    let options = [
        "Full S-tier install (recommended)",
        "Custom selections per category",
    ];
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, label)| list_item_line(*label, i == app.menu_cursor))
        .collect();

    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[0]);

    let hint = Paragraph::new(hint_line("↑/↓ navigate · Enter select · Esc back"))
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[1]);
}

// ── Software tier select (per category) ──────────────────────────────────────

pub fn draw_software_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(80, 70, area);
    let block = menu_block("STEP 5/5 — SOFTWARE TIERS");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(inner);

    let category = SOFTWARE_CATEGORIES
        .get(app.software_category_idx)
        .map(|cat| cat.label)
        .unwrap_or("Unknown");
    let header = Paragraph::new(Line::from(vec![
        Span::styled("Category: ", theme::dim_style()),
        Span::styled(category, theme::title_style()),
        Span::styled(
            format!(
                "  ({}/{})",
                app.software_category_idx + 1,
                SOFTWARE_CATEGORIES.len()
            ),
            theme::dim_style(),
        ),
    ]));
    f.render_widget(header, chunks[0]);

    let options = SOFTWARE_CATEGORIES
        .get(app.software_category_idx)
        .map(|cat| cat.options)
        .unwrap_or(&[]);

    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let label = format!("{} ({}) — {}", opt.name, opt.tier, opt.description);
            list_item_line(label, i == app.menu_cursor)
        })
        .collect();
    let list = List::new(items).style(theme::default_style());
    f.render_widget(list, chunks[1]);

    let hint = Paragraph::new(hint_line("↑/↓ navigate · Enter select · Esc back"))
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[2]);
}

// ── Pre-install confirm ───────────────────────────────────────────────────────

pub fn draw_pre_install_confirm(f: &mut Frame, area: Rect, app: &TuiApp) {
    let popup = centered_rect(70, 70, area);
    let block = menu_block("CONFIRM INSTALLATION");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let driver_name = app
        .drivers
        .get(app.selected_driver_idx)
        .map(|d| d.name())
        .unwrap_or("unknown");
    let profile_label = match app.profile_idx {
        0 => "Minimal",
        1 => "Dev",
        _ => "Full",
    };

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled("Installation Summary", theme::title_style())),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Driver:  ", theme::dim_style()),
            Span::styled(driver_name, theme::success_style()),
        ]),
        Line::from(vec![
            Span::styled("  Profile: ", theme::dim_style()),
            Span::styled(profile_label, theme::success_style()),
        ]),
        Line::from(vec![
            Span::styled("  Theme:   ", theme::dim_style()),
            Span::styled(app.theme_plan_label(), theme::success_style()),
        ]),
        Line::from(vec![
            Span::styled("  Software:", theme::dim_style()),
            Span::styled(app.software_plan_label(), theme::success_style()),
        ]),
        Line::from(""),
        Line::from(Span::styled("  Modules:", theme::dim_style())),
    ];

    let checked = module_checks(&app.modules);
    for (i, (label, _)) in MODULE_LABELS.iter().enumerate() {
        let (glyph, style) = if checked[i] {
            ("✓", theme::success_style())
        } else {
            ("○", theme::dim_style())
        };
        lines.push(Line::from(vec![
            Span::styled(format!("    {glyph} "), style),
            Span::styled(*label, style),
        ]));
    }

    if app.dry_run {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  ⚠ DRY-RUN MODE — no changes will be made",
            theme::warning_style(),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Press Enter or Y to begin  ·  Esc or N to go back",
        theme::accent_style().add_modifier(Modifier::BOLD),
    )));

    let para = Paragraph::new(Text::from(lines))
        .style(theme::default_style())
        .wrap(Wrap { trim: false });
    f.render_widget(para, inner);
}

// ── Mid-install confirm (from PhaseObserver::confirm) ─────────────────────────

pub fn draw_mid_install_confirm(f: &mut Frame, area: Rect, app: &TuiApp) {
    let Some(state) = &app.confirm_state else {
        return;
    };

    let popup = centered_rect(60, 30, area);
    let block = menu_block("CONFIRMATION REQUIRED");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner);

    let prompt_para = Paragraph::new(state.prompt.as_str())
        .style(theme::default_style())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(prompt_para, chunks[0]);

    // Yes / No buttons
    let (yes_style, no_style) = if state.selected {
        (
            theme::success_style().add_modifier(Modifier::BOLD | Modifier::REVERSED),
            theme::dim_style(),
        )
    } else {
        (
            theme::dim_style(),
            theme::error_style().add_modifier(Modifier::BOLD | Modifier::REVERSED),
        )
    };
    let buttons = Line::from(vec![
        Span::styled("  [ Yes ] ", yes_style),
        Span::raw("  "),
        Span::styled("  [ No ]  ", no_style),
    ]);
    let btn_para = Paragraph::new(buttons).alignment(Alignment::Center);
    f.render_widget(btn_para, chunks[1]);
}

// ── Mid-install password prompt ────────────────────────────────────────────

pub fn draw_password_prompt(
    f: &mut Frame,
    area: Rect,
    _app: &TuiApp,
    state: &crate::tui::app::PasswordState,
) {
    let prompt_rect = centered_rect(60, 30, area);
    let block = menu_block("🔐 Sudo Password Required");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(prompt_rect);

    // Prompt text with security notice
    let prompt_text = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(state.prompt.clone(), theme::default_style())),
        Line::from(Span::styled(
            "⚠️  Password is stored temporarily in memory only and will be cleared after installation.",
            theme::warning_style()
        )),
    ]))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
    f.render_widget(prompt_text, chunks[0]);

    // Password input field (show asterisks)
    let password_display = "*".repeat(state.password.len());
    let input_text = Paragraph::new(Text::from(password_display.clone()))
        .alignment(Alignment::Center)
        .style(theme::accent_style());
    f.render_widget(input_text, chunks[2]);

    // Instructions
    let instructions = Line::from(vec![
        Span::styled("Enter password: ", theme::default_style()),
        Span::styled(&password_display, theme::accent_style()),
    ]);
    let instr_para = Paragraph::new(instructions).alignment(Alignment::Center);
    f.render_widget(instr_para, chunks[2]);

    // Key hints
    let hints = Paragraph::new(Text::from(vec![Line::from(Span::styled(
        "Enter: Confirm  Esc: Cancel  Backspace: Delete",
        theme::dim_style(),
    ))]))
    .alignment(Alignment::Center);
    f.render_widget(hints, chunks[3]);

    f.render_widget(block, prompt_rect);
}
