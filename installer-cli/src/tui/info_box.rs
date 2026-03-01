//! Info Box - Bottom information display showing installation progress, time estimates, and context help

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::app::{Screen, TuiApp};
use crate::tui::theme;

pub const INFO_BOX_HEIGHT: u16 = 3;

pub fn draw_info_box(f: &mut ratatui::Frame, area: Rect, app: &TuiApp) -> Rect {
    // Create a layout that reserves space for the info box at the bottom
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),                  // Main content area
            Constraint::Length(INFO_BOX_HEIGHT), // Info box at bottom
        ])
        .split(area);

    let main_area = vertical[0];
    let info_area = vertical[1];

    // Draw the info box
    draw_info_content(f, info_area, app);

    // Return the main area for the rest of the UI to use
    main_area
}

fn draw_info_content(f: &mut ratatui::Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::TOP) // Only top border to separate from main content
        .border_type(theme::inner_border_type())
        .border_style(theme::border_style())
        .style(theme::default_style());

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Build info lines based on current screen and state
    let info_lines = build_info_lines(app);
    let paragraph = Paragraph::new(info_lines).style(theme::default_style());
    f.render_widget(paragraph, inner);
}

fn build_info_lines(app: &TuiApp) -> Vec<Line<'static>> {
    match app.screen {
        Screen::Welcome => vec![
            Line::from(Span::styled(
                "MASH Installer v0.2.3 | Press Enter to begin",
                theme::dim_style(),
            )),
            Line::from(Span::styled(
                "System ready for configuration",
                theme::success_style(),
            )),
        ],

        Screen::ArchDetected => vec![
            Line::from(Span::styled(
                "Architecture detection complete",
                theme::success_style(),
            )),
            Line::from(Span::styled(
                "Proceeding to distribution selection...",
                theme::dim_style(),
            )),
        ],

        Screen::DistroSelect => vec![
            Line::from(Span::styled(
                "Select target distribution",
                theme::accent_style(),
            )),
            Line::from(Span::styled(
                "Use arrow keys to navigate, Enter to select",
                theme::dim_style(),
            )),
        ],

        Screen::ProfileSelect => vec![
            Line::from(Span::styled(
                "Choose installation profile",
                theme::accent_style(),
            )),
            Line::from(Span::styled(
                "Minimal/Dev/Full - determines software scope",
                theme::dim_style(),
            )),
        ],

        Screen::ModuleSelect => vec![
            Line::from(Span::styled(
                "Select optional modules",
                theme::accent_style(),
            )),
            Line::from(Span::styled(
                "Toggle with Space, navigate with arrows",
                theme::dim_style(),
            )),
        ],

        Screen::DeSelect => vec![
            Line::from(Span::styled(
                "Desktop Environment Selection",
                theme::accent_style(),
            )),
            Line::from(Span::styled(
                "Choose your preferred desktop environment",
                theme::dim_style(),
            )),
        ],

        Screen::ProtocolSelect => vec![
            Line::from(Span::styled(
                "Display Protocol Selection",
                theme::accent_style(),
            )),
            Line::from(Span::styled(
                "X11, Wayland, or Auto detection",
                theme::dim_style(),
            )),
        ],

        Screen::DeConfirm => vec![
            Line::from(Span::styled(
                "Desktop Environment Confirmation",
                theme::accent_style(),
            )),
            Line::from(Span::styled(
                "Review your selection before proceeding",
                theme::dim_style(),
            )),
        ],

        Screen::ThemeSelect => vec![
            Line::from(Span::styled("Theme Selection", theme::accent_style())),
            Line::from(Span::styled(
                "Choose visual theme for your system",
                theme::dim_style(),
            )),
        ],

        Screen::SoftwareMode => vec![
            Line::from(Span::styled(
                "Software Installation Mode",
                theme::accent_style(),
            )),
            Line::from(Span::styled(
                "Full or Custom software selection",
                theme::dim_style(),
            )),
        ],

        Screen::SoftwareSelect => vec![
            Line::from(Span::styled(
                "Custom Software Selection",
                theme::accent_style(),
            )),
            Line::from(Span::styled(
                "Select software categories and packages",
                theme::dim_style(),
            )),
        ],

        Screen::Confirm => vec![
            Line::from(Span::styled(
                "Pre-Installation Confirmation",
                theme::accent_style(),
            )),
            Line::from(Span::styled(
                "Review all settings before proceeding",
                theme::dim_style(),
            )),
        ],

        Screen::FontPrep => vec![
            Line::from(Span::styled("Font Preparation", theme::accent_style())),
            Line::from(Span::styled(
                "Downloading and installing fonts",
                theme::dim_style(),
            )),
        ],

        Screen::Installing => {
            let progress = if app.total_phases == 0 {
                0
            } else {
                (app.current_phase as f32 / app.total_phases as f32 * 100.0) as u8
            };

            let elapsed = app.start_time.elapsed().as_secs();
            let estimated_remaining = if progress > 0 {
                (elapsed * 100 / progress as u64) - elapsed
            } else {
                0
            };

            vec![
                Line::from(vec![
                    Span::styled("Installation Progress: ", theme::dim_style()),
                    Span::styled(format!("{}% ", progress), theme::success_style()),
                    Span::styled(
                        format!("Phase {} of {}", app.current_phase, app.total_phases),
                        theme::dim_style(),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Time: ", theme::dim_style()),
                    Span::styled(format!("{}s elapsed", elapsed), theme::success_style()),
                    Span::styled(
                        format!(" ~{}s remaining", estimated_remaining),
                        theme::dim_style(),
                    ),
                ]),
            ]
        }

        Screen::Done => vec![
            Line::from(Span::styled(
                "Installation Complete!",
                theme::success_style(),
            )),
            Line::from(Span::styled("Press Q to exit to shell", theme::dim_style())),
        ],

        Screen::Error => vec![
            Line::from(Span::styled(
                "Installation Error Encountered",
                theme::error_style(),
            )),
            Line::from(Span::styled(
                "Review error details above",
                theme::dim_style(),
            )),
        ],

        Screen::Password => vec![
            Line::from(Span::styled("Password Required", theme::accent_style())),
            Line::from(Span::styled(
                "Enter password to continue installation",
                theme::dim_style(),
            )),
        ],
    }
}

pub fn get_main_area_with_info_box(area: Rect) -> (Rect, Rect) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(INFO_BOX_HEIGHT)])
        .split(area);
    (vertical[0], vertical[1])
}
