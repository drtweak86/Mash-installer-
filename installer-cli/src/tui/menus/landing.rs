// 📜 GUILD MEMBER: RUNESMITH
// 🎯 Task: Implement landing menu UI for Shaft AD
// 📁 Location: installer-cli/src/tui/menus/landing.rs
// 📅 Date: 2026-03-09

use crate::tui::app::TuiApp;
use crate::tui::menus::helpers::station_block;
use crate::tui::theme;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

/// Landing menu items with descriptions
const LANDING_MENU: &[&str] = &[
    "1) Distribution Selection - Choose your Linux distribution",
    "2) Profile Selection - Select installation profile (Minimal/Dev/Full)",
    "3) System Summary - View hardware analysis and recommendations",
    "4) Theme Selection - Configure aesthetic preferences",
    "5) Software Selection - Choose applications and tools",
    "6) Advanced Configuration - Argon, Docker, Chezmoi settings",
    "7) Start Installation - Begin the installation process",
];

/// Draw the landing menu screen
///
/// # Arguments
///
/// * `f` - Ratatui frame for rendering
/// * `area` - Rect area to render within
/// * `app` - TuiApp state containing menu cursor and system profile
///
/// # Features
///
/// * Cyberpunk terminal aesthetic with station_block
/// * 7 numbered menu items with selection highlighting
/// * System information display at bottom
/// * Navigation instructions in footer
pub fn draw_landing(f: &mut Frame, area: Rect, app: &TuiApp) {
    // Create station block with cyberpunk aesthetic
    let block = station_block("MAIN_FORGE_MENU");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    // Layout: Header (3 lines) + Menu (flexible) + Footer (3 lines)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    // Header with centered title
    let header_text = Paragraph::new("MASH INSTALLER - MAIN MENU")
        .style(theme::success_style())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(header_text, chunks[0]);

    // Menu items with selection highlighting (>> for selected, spaces for others)
    let menu_items: Vec<_> = LANDING_MENU
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let prefix = if i == app.menu_cursor { ">> " } else { "   " };
            format!("{}{}", prefix, item)
        })
        .collect();

    let menu_text = menu_items.join("\n");
    let menu = Paragraph::new(menu_text)
        .style(theme::default_style())
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(menu, chunks[1]);

    // Footer with navigation instructions
    let footer =
        Paragraph::new("Use ↑/↓ arrows or 1-7 keys to navigate | ENTER to select | ESC to go back")
            .style(theme::dim_style())
            .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);

    // System info display (shows after system scan completes)
    if let Some(profile) = &app.system_profile {
        let ram_gb = profile.memory.ram_total_kb as f32 / (1024.0 * 1024.0);
        let sys_info = format!(
            "System: {} | CPU: {} | RAM: {:.1} GB | OS: {}",
            profile.platform.model, profile.cpu.model, ram_gb, profile.distro.pretty_name
        );
        let sys_paragraph = Paragraph::new(sys_info)
            .style(theme::dim_style())
            .alignment(Alignment::Center);
        f.render_widget(sys_paragraph, area);
    }
}

// 📝 RUNESMITH NOTES:
// ✅ Implemented cyberpunk menu UI with 7 options
// ✅ Added selection highlighting with >> prefix
// ✅ Included system info display for user context
// ✅ Added navigation instructions in footer
// ✅ Follows existing theme and station_block patterns
// ✅ No external dependencies beyond existing Ratatui imports
// ✅ Error handling: gracefully handles missing system_profile
