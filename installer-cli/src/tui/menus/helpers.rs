use crate::tui::app::{ModuleState, TuiApp};
use crate::tui::theme;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

#[allow(dead_code)]
pub fn centered_rect(width_pct: u16, height_pct: u16, area: Rect) -> Rect {
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

pub fn draw_navigation_info(f: &mut Frame, area: Rect, app: &TuiApp) {
    let nav_block = Block::default()
        .borders(Borders::TOP)
        .style(theme::default_style());
    let nav_area = Rect::new(area.x, area.y + area.height - 3, area.width, 3);
    f.render_widget(nav_block, nav_area);

    let nav_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(nav_area);

    let back_text = if app.navigation_history.is_empty() {
        Span::styled("[BACK: Disabled]", theme::dim_style())
    } else {
        Span::styled("[BACK: Esc]", theme::success_style())
    };
    f.render_widget(Paragraph::new(back_text), nav_chunks[0]);

    let context_text = Span::styled(app.get_navigation_context(), theme::default_style());
    f.render_widget(
        Paragraph::new(context_text).alignment(Alignment::Center),
        nav_chunks[1],
    );

    let help_text = Span::styled("[HELP: F1]", theme::warning_style());
    f.render_widget(
        Paragraph::new(help_text).alignment(Alignment::Right),
        nav_chunks[2],
    );
}

pub fn station_block<'a>(title: &'a str) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(theme::outer_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(format!(" {title} "), theme::title_style()))
        .style(theme::default_style())
}

pub fn command_prompt_line(
    label: impl Into<String>,
    index: usize,
    selected: bool,
) -> ratatui::widgets::ListItem<'static> {
    use ratatui::style::Modifier;
    let label = label.into();
    let style = if selected {
        theme::selected_style().add_modifier(Modifier::BOLD)
    } else {
        theme::default_style()
    };

    let prefix = if selected { "> " } else { "  " };
    let content = format!("{}{} [{}]", prefix, label.to_uppercase(), index);

    ratatui::widgets::ListItem::new(Line::from(vec![Span::styled(content, style)]))
}

pub fn module_checks(m: &ModuleState) -> [bool; 3] {
    [m.enable_argon, m.enable_p10k, m.docker_data_root]
}
