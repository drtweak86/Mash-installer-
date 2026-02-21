//! All ratatui draw() calls — layout + 4-pane rendering.

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use crate::tui::app::{LogLevel, PhaseStatus, Screen, TuiApp};
use crate::tui::menus;
use crate::tui::theme;

const BANNER: &str = r"
 ███╗   ███╗ █████╗ ███████╗██╗  ██╗
 ████╗ ████║██╔══██╗██╔════╝██║  ██║
 ██╔████╔██║███████║███████╗███████║
 ██║╚██╔╝██║██╔══██║╚════██║██╔══██║
 ██║ ╚═╝ ██║██║  ██║███████║██║  ██║
 ╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝
   Mythic Assembly & Sigil Heuristics";

// ── Top-level dispatch ────────────────────────────────────────────────────────

pub fn draw(f: &mut Frame, app: &TuiApp) {
    // Fill background
    f.render_widget(Block::default().style(theme::default_style()), f.area());

    match app.screen {
        Screen::Welcome => menus::draw_welcome(f, f.area(), app),
        Screen::DistroSelect => menus::draw_distro_select(f, f.area(), app),
        Screen::ModuleSelect => menus::draw_module_select(f, f.area(), app),
        Screen::ProfileSelect => menus::draw_profile_select(f, f.area(), app),
        Screen::Confirm => {
            if app.confirm_state.is_some() {
                menus::draw_mid_install_confirm(f, f.area(), app);
            } else {
                menus::draw_pre_install_confirm(f, f.area(), app);
            }
        }
        Screen::Installing => draw_installing(f, app),
        Screen::Done => draw_summary(f, app, false),
        Screen::Error => draw_summary(f, app, true),
    }
}

// ── Installing: 4-pane layout ─────────────────────────────────────────────────

pub fn draw_installing(f: &mut Frame, app: &TuiApp) {
    let area = f.area();

    // Outer chrome
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::outer_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(" ⚡ MASH INSTALLER ⚡ ", theme::title_style()))
        .style(theme::default_style());
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    // Split into top (main + log/stats) and BBS strip
    let vchunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner);

    let top_area = vchunks[0];
    let bbs_area = vchunks[1];

    // Split top into left (65%) and right (35%)
    let hchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(top_area);

    let main_area = hchunks[0];
    let right_area = hchunks[1];

    // Split right into log (top ~60%) and stats (bottom ~40%)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(right_area);

    let log_area = right_chunks[0];
    let stats_area = right_chunks[1];

    draw_main_pane(f, main_area, app);
    draw_action_log(f, log_area, app);
    draw_sys_stats(f, stats_area, app);
    draw_bbs(f, bbs_area, app);
}

// ── Main pane (top-left) ──────────────────────────────────────────────────────

fn draw_main_pane(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::inner_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(" MAIN ", theme::title_style()))
        .style(theme::default_style());
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Split inner into: banner, elapsed, progress bar, phase list
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // banner
            Constraint::Length(1), // elapsed
            Constraint::Length(2), // progress bar
            Constraint::Min(0),    // phase list
        ])
        .split(inner);

    // Banner
    let banner_para = Paragraph::new(BANNER)
        .style(theme::accent_style())
        .alignment(Alignment::Center);
    f.render_widget(banner_para, chunks[0]);

    // Elapsed
    let elapsed = app.start_time.elapsed();
    let elapsed_str = format!(
        "Elapsed: {:02}:{:02}:{:02}",
        elapsed.as_secs() / 3600,
        (elapsed.as_secs() / 60) % 60,
        elapsed.as_secs() % 60
    );
    let elapsed_para = Paragraph::new(elapsed_str)
        .style(theme::dim_style())
        .alignment(Alignment::Right);
    f.render_widget(elapsed_para, chunks[1]);

    // Progress bar
    let progress = if app.total_phases == 0 {
        0
    } else {
        (app.current_phase as u64 * 100) / app.total_phases as u64
    };
    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(theme::progress_filled_style())
        .percent(progress as u16)
        .label(Span::styled(
            format!("{}/{} phases", app.current_phase, app.total_phases),
            Style::default()
                .fg(theme::WHITE)
                .add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, chunks[2]);

    // Phase list
    let phase_items: Vec<ListItem> = app
        .phases
        .iter()
        .map(|row| {
            let (glyph, style) = match row.status {
                PhaseStatus::Done => ("✓", theme::success_style()),
                PhaseStatus::Running => ("▶", theme::accent_style()),
                PhaseStatus::Failed => ("✗", theme::error_style()),
                PhaseStatus::Skipped => ("─", theme::dim_style()),
                PhaseStatus::Pending => ("○", theme::dim_style()),
            };
            let line = Line::from(vec![
                Span::styled(format!(" {glyph} "), style),
                Span::styled(row.name.clone(), style),
            ]);
            ListItem::new(line)
        })
        .collect();

    let phase_list = List::new(phase_items).style(theme::default_style());
    f.render_widget(phase_list, chunks[3]);
}

// ── Action log (top-right) ────────────────────────────────────────────────────

fn draw_action_log(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::inner_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(" ACTION LOG ", theme::title_style()))
        .style(theme::default_style());
    let inner = block.inner(area);
    f.render_widget(block, area);

    let visible_height = inner.height as usize;
    let total = app.log.len();

    let start = if total > visible_height {
        let max_scroll = total - visible_height;
        app.log_scroll.min(max_scroll)
    } else {
        0
    };

    let items: Vec<ListItem> = app
        .log
        .iter()
        .skip(start)
        .take(visible_height)
        .map(|entry| {
            let ts_span = Span::styled(format!("[{}] ", entry.timestamp), theme::dim_style());
            let msg_style = match entry.level {
                LogLevel::Success => theme::success_style(),
                LogLevel::Warning => theme::warning_style(),
                LogLevel::Error => theme::error_style(),
                LogLevel::Info => theme::default_style(),
            };
            let msg_span = Span::styled(entry.text.clone(), msg_style);
            ListItem::new(Line::from(vec![ts_span, msg_span]))
        })
        .collect();

    let log_list = List::new(items).style(theme::default_style());
    f.render_widget(log_list, inner);
}

// ── Sys stats (bottom-right) ──────────────────────────────────────────────────

fn draw_sys_stats(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::inner_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(" SYS STATS ", theme::title_style()))
        .style(theme::default_style());
    let inner = block.inner(area);
    f.render_widget(block, area);

    let s = &app.sys_stats;
    let cpu_pct = s.cpu_pct.clamp(0.0, 100.0) as u16;
    let ram_pct = if s.ram_total_mb > 0 {
        (s.ram_used_mb * 100 / s.ram_total_mb).min(100) as u16
    } else {
        0
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // CPU gauge
            Constraint::Length(1), // RAM gauge
            Constraint::Length(1), // NET
            Constraint::Length(1), // I/O
            Constraint::Min(0),
        ])
        .split(inner);

    // CPU gauge
    let cpu_gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(theme::CYAN))
        .percent(cpu_pct)
        .label(format!("CPU {cpu_pct:3}%"));
    f.render_widget(cpu_gauge, chunks[0]);

    // RAM gauge
    let ram_gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(theme::MAGENTA))
        .percent(ram_pct)
        .label(format!("RAM {}/{} MB", s.ram_used_mb, s.ram_total_mb));
    f.render_widget(ram_gauge, chunks[1]);

    // NET
    let net_text = format!("NET ↓{:.1} ↑{:.1} kB/s", s.net_rx_kbps, s.net_tx_kbps);
    let net_para = Paragraph::new(net_text).style(theme::success_style());
    f.render_widget(net_para, chunks[2]);

    // I/O
    let io_text = format!("I/O R:{:.1} W:{:.1} kB/s", s.io_r_kbps, s.io_w_kbps);
    let io_para = Paragraph::new(io_text).style(theme::warning_style());
    f.render_widget(io_para, chunks[3]);
}

// ── BBS strip (full-width bottom) ─────────────────────────────────────────────

fn draw_bbs(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::inner_border_type())
        .border_style(theme::border_style())
        .style(theme::default_style());
    let inner = block.inner(area);
    f.render_widget(block, area);

    let bbs_para = Paragraph::new(app.bbs_msg.as_str())
        .style(theme::bbs_style())
        .alignment(Alignment::Center);
    f.render_widget(bbs_para, inner);
}

// ── Done / Error summary ──────────────────────────────────────────────────────

pub fn draw_summary(f: &mut Frame, app: &TuiApp, is_error: bool) {
    let area = f.area();

    let title = if is_error {
        " ✗ INSTALLATION FAILED "
    } else {
        " ✓ INSTALLATION COMPLETE "
    };
    let title_style = if is_error {
        theme::error_style().add_modifier(Modifier::BOLD)
    } else {
        theme::success_style().add_modifier(Modifier::BOLD)
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
            format!("Error: {msg}"),
            theme::error_style(),
        )));
        lines.push(Line::from(""));
    }

    if let Some(report) = &app.report {
        lines.push(Line::from(Span::styled(
            "Completed phases:",
            theme::title_style(),
        )));
        for phase in &report.completed_phases {
            lines.push(Line::from(Span::styled(
                format!("  ✓ {phase}"),
                theme::success_style(),
            )));
        }

        if !report.errors.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Errors:", theme::error_style())));
            for err in &report.errors {
                lines.push(Line::from(Span::styled(
                    format!("  ✗ {} — {}", err.phase, err.user_message()),
                    theme::error_style(),
                )));
                if let Some(advice) = &err.advice {
                    lines.push(Line::from(Span::styled(
                        format!("    Advice: {advice}"),
                        theme::warning_style(),
                    )));
                }
            }
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("Staging: {}", report.staging_dir.display()),
            theme::dim_style(),
        )));
    }

    if !is_error {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Post-install notes:",
            theme::title_style(),
        )));
        lines.push(Line::from(
            "  • Log out and back in for docker group to take effect.",
        ));
        lines.push(Line::from(
            "  • Run `mash-setup doctor` to verify everything.",
        ));
        lines.push(Line::from(
            "  • Config lives at ~/.config/mash-installer/config.toml",
        ));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press q or Esc to exit.",
        theme::dim_style(),
    )));

    // Scrollable
    let total_lines = lines.len();
    let visible = inner.height as usize;
    let scroll = app.summary_scroll.min(total_lines.saturating_sub(visible));

    let text = Text::from(lines);
    let para = Paragraph::new(text)
        .style(theme::default_style())
        .scroll((scroll as u16, 0))
        .wrap(Wrap { trim: false });
    f.render_widget(para, inner);
}
