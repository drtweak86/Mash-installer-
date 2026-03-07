use crate::tui::app::TuiApp;
use crate::tui::menus::helpers::station_block;
use crate::tui::theme;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
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
