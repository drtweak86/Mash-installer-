use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
use ratatui::Frame;

use crate::tui::app::TuiApp;
use crate::tui::menus::helpers::station_block;
use crate::tui::theme;

pub fn draw_system_scan(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("ACTIVE_STATION_SCRYING");
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

    let text = vec![
        Line::from("STATION_01 IS ANALYZING HOST PEDIGREE..."),
        Line::from(""),
        Line::from(Span::styled(
            "SCANNIG HARDWARE / NETWORK / STORAGE",
            theme::accent_style(),
        )),
        Line::from(""),
        Line::from("PLEASE STAND BY FOR WISDOM..."),
    ];

    f.render_widget(Paragraph::new(text).alignment(Alignment::Center), chunks[0]);

    // Simple animated gauge based on ticks or time
    let progress = (app.start_time.elapsed().as_millis() % 2000) as f64 / 2000.0;
    let label = format!("{:.0}%", progress * 100.0);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(theme::accent_style())
        .ratio(progress)
        .label(label);

    // Wait, I need to import Block and Borders if I use them, but Gauge has its own.
    // Actually theme might have some styles for gauges.

    f.render_widget(gauge, chunks[1]);

    let hint = Paragraph::new("ANALYSIS IN PROGRESS")
        .style(theme::dim_style())
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[2]);
}
