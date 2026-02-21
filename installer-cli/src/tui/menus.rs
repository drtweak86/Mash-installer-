//! Interactive menu screens: Welcome, DistroSelect, ModuleSelect, ProfileSelect, Confirm.

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

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

fn hint_line(text: &str) -> Line {
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
    let block = menu_block("STEP 1/3 — SELECT DISTRO");
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
    let block = menu_block("STEP 2/3 — SELECT MODULES");
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
    let block = menu_block("STEP 3/3 — SELECT PROFILE");
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
