use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::tui::app::{LogLevel, TuiApp};
use crate::tui::menus::helpers::station_block;
use crate::tui::theme;

pub fn draw_installing(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("FORGE_SEQUENCING_ACTIVE");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    // Progress bar
    let gauge_area = chunks[0];
    let gauge_block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::dim_style())
        .title(Span::styled(" PROGRESS_LINK ", theme::dim_style()));
    f.render_widget(&gauge_block, gauge_area);

    let gauge_inner = gauge_block.inner(gauge_area);
    let width = gauge_inner.width as usize;
    let filled = (app.progress_pct / 100.0 * width as f32) as usize;
    let gauge_text = format!(
        "{}{}",
        "█".repeat(filled),
        " ".repeat(width.saturating_sub(filled))
    );
    f.render_widget(
        Paragraph::new(Span::styled(gauge_text, theme::success_style())),
        gauge_inner,
    );

    // Log
    let items: Vec<ListItem> = app
        .log
        .iter()
        .map(|entry| {
            let style = match entry.level {
                LogLevel::Info => theme::default_style(),
                LogLevel::Success => theme::success_style(),
                LogLevel::Warning => theme::warning_style(),
                LogLevel::Error => theme::error_style(),
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("[{}] ", entry.timestamp), theme::dim_style()),
                Span::styled(&entry.text, style),
            ]))
        })
        .collect();

    let list = List::new(items).style(theme::default_style()).block(
        Block::default()
            .borders(Borders::LEFT)
            .border_style(theme::dim_style()),
    );

    // Manual scroll handling for log
    f.render_widget(list, chunks[1]);

    // Current phase
    if let Some(row) = app.phases.get(app.current_phase) {
        let phase_info = Paragraph::new(format!("EXECUTING: {}", row.name.to_uppercase()))
            .style(theme::warning_style())
            .alignment(Alignment::Center);
        f.render_widget(phase_info, chunks[2]);
    }
}

pub fn draw_done(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("INSTALLATION_COMPLETE");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let mut lines = vec![
        Line::from(Span::styled(
            "SUCCESS: SYSTEM_COORDINATES_STABILIZED",
            theme::success_style(),
        )),
        Line::from(""),
    ];

    if let Some(ref report) = app.report {
        lines.push(Line::from(format!(
            "COMPLETED PHASES: {}",
            report.completed_phases.len()
        )));
        lines.push(Line::from(""));

        if !report.errors.is_empty() {
            lines.push(Line::from(Span::styled(
                "NON-FATAL INCIDENTS:",
                theme::warning_style(),
            )));
            for err in &report.errors {
                lines.push(Line::from(format!(
                    "  - {}: {}",
                    err.phase, err.description
                )));
            }
            lines.push(Line::from(""));
        }
    }

    lines.push(Line::from(Span::styled(
        "READY > _",
        theme::success_style(),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "PRESS Q TO RETURN TO SHELL",
        theme::dim_style(),
    )));

    f.render_widget(Paragraph::new(lines).alignment(Alignment::Center), inner);
}

pub fn draw_error(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("FATAL_SEQUENCE_ERROR");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let mut lines = vec![
        Line::from(Span::styled(
            "CRITICAL FAILURE DETECTED",
            theme::error_style(),
        )),
        Line::from(""),
    ];

    if let Some(ref msg) = app.error_msg {
        lines.push(Line::from(msg.clone()));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "STATION_HALTED",
        theme::error_style(),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "PRESS Q TO ABORT TO SHELL",
        theme::warning_style(),
    )));

    f.render_widget(Paragraph::new(lines).alignment(Alignment::Center), inner);
}
