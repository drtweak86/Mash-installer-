use crate::tui::app::TuiApp;
use crate::tui::menus::helpers::station_block;
use crate::tui::theme;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

const WELCOME_BANNER: &str = r"
  __  __    _    ____  _   _ 
 |  \/  |  / \  / ___|| | | |
 | |\/| | / _ \ \___ \| |_| |
 | |  | |/ ___ \ ___) |  _  |
 |_|  |_/_/   \_\____/|_| |_|
 _______________________________________
/                                       
|  * * *  MASH BOOT LOADER v1.0  * * *  |
|                                       |
|  STATION STATUS: READY                |
|  IO MODULES:     LOADED               |
|  PHOSPHOR:       STABILIZED           |
\_______________________________________/
";

pub fn draw_welcome(f: &mut Frame, area: Rect, _app: &TuiApp) {
    let block = station_block("STATION_BOOT_SEQUENCE");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner);

    let banner = Paragraph::new(WELCOME_BANNER)
        .style(theme::success_style())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    f.render_widget(banner, chunks[0]);

    let prompt = Paragraph::new("PRESS [ENTER] TO INITIALIZE LOCAL FORGE")
        .style(theme::warning_style())
        .alignment(Alignment::Center);
    f.render_widget(prompt, chunks[1]);
}

pub fn draw_arch_detected(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("AUTO_PROBE_RESULTS");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    let title = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("STATION_01 PROBE: ", theme::default_style()),
            Span::styled("HARDWARE_SIGIL_DETECTED", theme::success_style()),
        ]),
        Line::from(""),
    ])
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let arch = std::env::consts::ARCH.to_uppercase();
    let info = Paragraph::new(vec![
        Line::from(Span::styled("TARGET_ARCHITECTURE:", theme::dim_style())),
        Line::from(Span::styled(format!("  {} ", arch), theme::title_style())),
        Line::from(""),
        Line::from(Span::styled("STATUS: ", theme::dim_style())),
        Line::from(Span::styled("  STABILIZED ", theme::success_style())),
        Line::from(""),
    ])
    .alignment(Alignment::Center);
    f.render_widget(info, chunks[1]);

    let elapsed = app
        .arch_timer
        .map(|t: std::time::Instant| t.elapsed().as_secs())
        .unwrap_or(0);
    let remaining = 15u64.saturating_sub(elapsed);

    let prompt = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("PROCEEDING TO DRIVER CONFIGURATION IN ", theme::dim_style()),
            Span::styled(format!("{}s", remaining), theme::warning_style()),
        ]),
        Line::from(Span::styled(
            "PRESS [C] TO CONFIGURE MANUALLY",
            theme::dim_style(),
        )),
    ])
    .alignment(Alignment::Center);
    f.render_widget(prompt, chunks[2]);
}
