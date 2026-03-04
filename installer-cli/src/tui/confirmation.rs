//! Long Process Confirmation — Advisory dialogs for operations > 2 minutes

use std::time::Duration;

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::app::TuiApp;
use crate::tui::theme;

/// Draw long process confirmation dialog
#[allow(dead_code)]
pub fn draw_long_process_confirm(f: &mut ratatui::Frame, area: Rect, app: &TuiApp) {
    let Some(state) = &app.long_process_state else {
        return;
    };

    if !state.should_show_confirmation() {
        return;
    }

    let popup = centered_rect(70, 20, area);

    // Draw background overlay
    let overlay = Block::default().style(theme::dim_style());
    f.render_widget(overlay, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::outer_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(
            " LONG OPERATION ADVISORY ",
            theme::title_style(),
        ))
        .style(theme::default_style());

    let inner = block.inner(popup);
    f.render_widget(block, popup);

    // Split into header, content, and footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(inner);

    // Header - Operation name
    let header_text = Text::from(vec![Line::from(vec![
        Span::styled("OPERATION: ", theme::accent_style()),
        Span::styled(&state.operation_name, theme::success_style()),
    ])]);
    let header_para = Paragraph::new(header_text)
        .style(theme::default_style())
        .alignment(Alignment::Center);
    f.render_widget(header_para, chunks[0]);

    // Content - Advisory message and details
    let duration_minutes = state.estimated_duration.as_secs() / 60;
    let content_text = Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(
            "THIS OPERATION WILL TAKE APPROXIMATELY:",
            theme::accent_style(),
        )),
        Line::from(vec![Span::styled(
            format!("{} minutes", duration_minutes),
            theme::warning_style(),
        )]),
        Line::from(""),
        Line::from(Span::styled(
            "PLEASE ENSURE YOU HAVE SUFFICIENT TIME BEFORE PROCEEDING.",
            theme::dim_style(),
        )),
        Line::from(Span::styled(
            "THIS IS A GOOD TIME TO GRAB A BEVERAGE. 🍺",
            theme::dim_style(),
        )),
    ]);
    let content_para = Paragraph::new(content_text)
        .style(theme::default_style())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(content_para, chunks[1]);

    // Footer - Countdown and instructions
    let footer_lines = if let Some(countdown) = state.countdown {
        vec![
            Line::from(vec![
                Span::styled("AUTO-PROCEED IN: ", theme::dim_style()),
                Span::styled(format!("{}s", countdown), theme::success_style()),
            ]),
            Line::from(vec![
                Span::styled("PRESS ", theme::dim_style()),
                Span::styled("ENTER", theme::accent_style()),
                Span::styled(" TO PROCEED NOW OR ", theme::dim_style()),
                Span::styled("ESC", theme::accent_style()),
                Span::styled(" TO CANCEL", theme::dim_style()),
            ]),
        ]
    } else {
        vec![Line::from(vec![
            Span::styled("PRESS ", theme::dim_style()),
            Span::styled("ENTER", theme::accent_style()),
            Span::styled(" TO PROCEED OR ", theme::dim_style()),
            Span::styled("ESC", theme::accent_style()),
            Span::styled(" TO CANCEL", theme::dim_style()),
        ])]
    };

    let footer_text = Text::from(footer_lines);
    let footer_para = Paragraph::new(footer_text)
        .style(theme::default_style())
        .alignment(Alignment::Center);
    f.render_widget(footer_para, chunks[2]);
}

/// Helper function to center a rectangle within an area
#[allow(dead_code)]
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width) / 2),
            Constraint::Percentage(width),
            Constraint::Percentage((100 - width) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height) / 2),
            Constraint::Percentage(height),
            Constraint::Percentage((100 - height) / 2),
        ])
        .split(horizontal[1])[1]
}

/// Check if an operation should trigger long process confirmation
#[allow(dead_code)]
pub fn should_show_long_confirmation(estimated_duration: Duration) -> bool {
    estimated_duration.as_secs() > 120 // > 2 minutes
}

/// Get advisory message for long operations
#[allow(dead_code)]
pub fn get_long_operation_advisory(operation_name: &str, duration: Duration) -> String {
    let minutes = duration.as_secs() / 60;
    format!(
        "ADVISORY: Operation '{}' will take approximately {} minutes. Please ensure you have sufficient time.",
        operation_name, minutes
    )
}
