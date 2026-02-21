//! 1984 Retro-Station Rendering — Single-pane command flow.

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::tui::app::{LogLevel, Screen, TuiApp};
use crate::tui::menus;
use crate::tui::theme;

const BANNER: &str = r"
   _______________________________________
  /                                       \
  |  MASH INSTALLER v0.1.6 - (C) 1984     |
  |  MYTHIC ASSEMBLY & SIGIL HEURISTICS   |
  |  SYSTEM READY.                        |
  \_______________________________________/
";

// ── Top-level dispatch ────────────────────────────────────────────────────────

pub fn draw(f: &mut Frame, app: &TuiApp) {
    // Fill background with pure black
    f.render_widget(Block::default().style(theme::default_style()), f.area());

    match app.screen {
        Screen::Welcome => menus::draw_welcome(f, f.area(), app),
        Screen::DistroSelect => menus::draw_distro_select(f, f.area(), app),
        Screen::ProfileSelect => menus::draw_profile_select(f, f.area(), app),
        Screen::ModuleSelect => menus::draw_module_select(f, f.area(), app),
        Screen::ThemeSelect => menus::draw_theme_select(f, f.area(), app),
        Screen::SoftwareMode => menus::draw_software_mode_select(f, f.area(), app),
        Screen::SoftwareSelect => menus::draw_software_select(f, f.area(), app),
        Screen::Confirm => {
            if app.confirm_state.is_some() {
                menus::draw_mid_install_confirm(f, f.area(), app);
            } else {
                menus::draw_pre_install_confirm(f, f.area(), app);
            }
        }
        Screen::Password => {
            if let Some(state) = &app.password_state {
                menus::draw_password_prompt(f, f.area(), app, state);
            }
        }

        Screen::Installing => draw_installing(f, app),
        Screen::Done => draw_summary(f, app, false),
        Screen::Error => draw_summary(f, app, true),
    }
}

// ── Installing: Single-pane station flow ──────────────────────────────────────

pub fn draw_installing(f: &mut Frame, app: &TuiApp) {
    let area = f.area();

    // Outer chrome - Thick border for that 80s monitor look
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::outer_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(" STATION_01 ", theme::title_style()))
        .style(theme::default_style());
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    // Split into main buffer and status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main terminal buffer
            Constraint::Length(1), // Status bar
        ])
        .split(inner);

    draw_terminal_buffer(f, chunks[0], app);
    draw_status_bar(f, chunks[1], app);
}

// ── Terminal Buffer (Action Log + Phases) ─────────────────────────────────────

fn draw_terminal_buffer(f: &mut Frame, area: Rect, app: &TuiApp) {
    let mut lines: Vec<Line> = Vec::new();

    // Small banner at the top of the buffer
    for line in BANNER.lines() {
        lines.push(Line::from(Span::styled(line, theme::accent_style())));
    }
    lines.push(Line::from(""));

    // Current phase status
    if let Some(current) = app.phases.get(app.current_phase.saturating_sub(1)) {
        lines.push(Line::from(vec![
            Span::styled("EXECUTING: ", theme::accent_style()),
            Span::styled(&current.name, theme::success_style()),
        ]));
    }

    // Progress bar (as a sequence of blocks)
    let progress = if app.total_phases == 0 {
        0
    } else {
        (app.current_phase as u64 * 100) / app.total_phases as u64
    };
    let bar_text = format!(
        "[{:_<width$}] {}%",
        "",
        progress,
        width = area.width as usize - 10
    );
    lines.push(Line::from(Span::styled(bar_text, theme::success_style())));
    lines.push(Line::from(""));

    // Log entries
    for entry in &app.log {
        let ts_span = Span::styled(format!("[{}] ", entry.timestamp), theme::dim_style());
        let msg_style = match entry.level {
            LogLevel::Success => theme::success_style(),
            LogLevel::Warning => theme::warning_style(),
            LogLevel::Error => theme::error_style(),
            LogLevel::Info => theme::default_style(),
        };
        let msg_span = Span::styled(entry.text.clone(), msg_style);
        lines.push(Line::from(vec![ts_span, msg_span]));
    }

    let visible_height = area.height as usize;
    let total_lines = lines.len();
    let start = total_lines.saturating_sub(visible_height);

    let buffer_para = Paragraph::new(Text::from(lines))
        .style(theme::default_style())
        .scroll((start as u16, 0))
        .wrap(Wrap { trim: false });
    f.render_widget(buffer_para, area);
}

// ── Status Bar (CPU/RAM/NET/BBS) ──────────────────────────────────────────────

fn draw_status_bar(f: &mut Frame, area: Rect, app: &TuiApp) {
    let s = &app.sys_stats;
    let cpu = s.cpu_pct as u16;
    let ram_pct = if s.ram_total_mb > 0 {
        (s.ram_used_mb * 100 / s.ram_total_mb).min(100) as u16
    } else {
        0
    };

    let status_line = format!(
        "CPU:{:3}% | RAM:{:3}% | NET:{:.1}kB/s | BBS: {}",
        cpu, ram_pct, s.net_rx_kbps, app.bbs_msg
    );

    let para = Paragraph::new(status_line)
        .style(Style::default().fg(theme::BLACK).bg(theme::GREEN))
        .alignment(Alignment::Left);
    f.render_widget(para, area);
}

// ── Done / Error summary ──────────────────────────────────────────────────────

pub fn draw_summary(f: &mut Frame, app: &TuiApp, is_error: bool) {
    let area = f.area();

    let title = if is_error {
        " ! ABORTED ! "
    } else {
        " * SYSTEM CONFIGURED * "
    };
    let title_style = if is_error {
        theme::error_style()
    } else {
        theme::title_style()
    };

    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::outer_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(title, title_style))
        .style(theme::default_style());
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    let mut lines: Vec<Line> = Vec::new();

    if let Some(msg) = &app.error_msg {
        lines.push(Line::from(Span::styled(
            "FATAL ERROR DETECTED",
            theme::error_style(),
        )));
        lines.push(Line::from(Span::styled(
            format!("MSG: {msg}"),
            theme::error_style(),
        )));
        lines.push(Line::from(""));
    }

    if let Some(report) = &app.report {
        lines.push(Line::from(Span::styled(
            "PHASE AUDIT:",
            theme::accent_style(),
        )));
        for phase in &report.completed_phases {
            lines.push(Line::from(vec![
                Span::styled("  [OK] ", theme::success_style()),
                Span::styled(phase, theme::default_style()),
            ]));
        }

        if !report.errors.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("INCIDENTS:", theme::error_style())));
            for err in &report.errors {
                lines.push(Line::from(Span::styled(
                    format!("  [ERR] {} : {}", err.phase, err.user_message()),
                    theme::error_style(),
                )));
            }
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "READY > _",
        theme::success_style(),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "PRESS Q TO RETURN TO SHELL",
        theme::dim_style(),
    )));

    let text = Text::from(lines);
    let para = Paragraph::new(text)
        .style(theme::default_style())
        .wrap(Wrap { trim: false });
    f.render_widget(para, inner);
}
