use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, Paragraph};
use ratatui::Frame;

use crate::tui::app::TuiApp;
use crate::tui::menus::helpers::{command_prompt_line, draw_navigation_info, station_block};
use crate::tui::theme;

pub fn draw_software_mode_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("SOFTWARE_TIER_MODES");
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

    f.render_widget(Paragraph::new("SELECT DEPLOYMENT STRATEGY:"), chunks[0]);

    let modes = [
        ("Bard's Recommendations", "S-tier curated stack"),
        ("Automatic Selection", "Platform-optimized baseline"),
        ("Manual Selection", "Configure every component"),
    ];

    let items: Vec<ListItem> = modes
        .iter()
        .enumerate()
        .map(|(i, (label, _))| command_prompt_line(*label, i + 1, i == app.menu_cursor))
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    draw_navigation_info(f, area, app);
}

pub fn draw_software_select(f: &mut Frame, area: Rect, app: &TuiApp) {
    let category = match app.catalog.categories.get(app.software_category_idx) {
        Some(c) => c,
        None => return,
    };

    let title = format!("SOFTWARE: {}", category.display_name.to_uppercase());
    let block = station_block(&title);
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

    f.render_widget(
        Paragraph::new(format!(
            "STEP {} OF {}: CHOOSE COMPONENT",
            app.software_category_idx + 1,
            app.catalog.categories.len()
        )),
        chunks[0],
    );

    let all_programs: Vec<&installer_core::catalog::Program> = category
        .subcategories
        .iter()
        .flat_map(|sc| &sc.programs)
        .collect();

    let items: Vec<ListItem> = all_programs
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let selected = i == app.menu_cursor;
            let current_pick = app.software_picks.get(&category.display_name);
            let is_picked = current_pick.map(|id| id == &p.id).unwrap_or(false);

            software_status_line(p, i + 1, selected, is_picked)
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    // Help
    if let Some(prog) = all_programs.get(app.menu_cursor) {
        f.render_widget(
            Paragraph::new(format!("INTEL: {}", prog.description)).style(theme::dim_style()),
            chunks[2],
        );
    }

    draw_navigation_info(f, area, app);
}

fn software_status_line(
    p: &installer_core::catalog::Program,
    index: usize,
    selected: bool,
    is_picked: bool,
) -> ListItem<'static> {
    use ratatui::style::Modifier;
    let style = if selected {
        theme::selected_style().add_modifier(Modifier::BOLD)
    } else {
        theme::default_style()
    };

    let prefix = if selected { "> " } else { "  " };
    let status = if is_picked {
        "[SELECTED]"
    } else {
        "[        ]"
    };
    let rec = if p.recommended { " (REC)" } else { "" };

    ListItem::new(Line::from(vec![
        Span::styled(
            format!(
                "{}{:<30} {:<10}{}",
                prefix,
                p.name.to_uppercase(),
                status,
                rec
            ),
            style,
        ),
        Span::styled(format!(" [{}]", index), theme::dim_style()),
    ]))
}
