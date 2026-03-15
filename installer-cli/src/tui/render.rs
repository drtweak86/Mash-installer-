//! 1984 Retro-Station Rendering — Single-pane command flow.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::tui::app::{Screen, TuiApp};
use crate::tui::info_box;
use crate::tui::menus;
use crate::tui::theme;

#[allow(dead_code)]
const BANNER: &str = r"
  __  __    _    ____  _   _ 
 |  \/  |  / \  / ___|| | | |
 | |\/| | / _ \ \___ \| |_| |
 | |  | |/ ___ \ ___) |  _  |
 |_|  |_/_/   \_\____/|_| |_|
 _______________________________
/                               \
|  MASH INSTALLER v0.2.3        |
|  (C) 1984 MYTHIC ASSEMBLY     |
|  SYSTEM READY.                |
\_______________________________/
";

pub fn draw(f: &mut Frame, app: &TuiApp) {
    f.render_widget(Block::default().style(theme::default_style()), f.area());

    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::outer_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(
            " STATION_01 : SYSTEM_INITIALIZATION ",
            theme::title_style(),
        ))
        .style(theme::default_style());
    let chrome_area = f.area();
    f.render_widget(&outer, chrome_area);
    let inner_chrome = outer.inner(chrome_area);

    let main_layout_area = info_box::draw_info_box(f, inner_chrome, app);

    let root_v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(main_layout_area);

    let top_h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(40)])
        .split(root_v_chunks[0]);

    let right_v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .split(top_h_chunks[1]);

    let main_area = top_h_chunks[0];
    let stats_area = right_v_chunks[0];
    let intel_area = right_v_chunks[1];
    let bbs_area = root_v_chunks[1];

    match app.screen {
        Screen::Welcome => menus::draw_welcome(f, main_area, app),
        Screen::SystemScan => menus::draw_system_scan(f, main_area, app),
        Screen::Landing => menus::draw_landing(f, main_area, app),
        Screen::DistroSelect => menus::draw_distro_select(f, main_area, app),
        Screen::ProfileSelect => menus::draw_profile_select(f, main_area, app),
        Screen::ThemeSelect => menus::draw_theme_select(f, main_area, app),
        Screen::SoftwareMode => menus::draw_software_mode_select(f, main_area, app),
        Screen::SoftwareCategorySelect => menus::draw_software_category_select(f, main_area, app),
        Screen::SoftwareSelect => menus::draw_software_select(f, main_area, app),
        Screen::DeSelect => menus::draw_de_select(f, main_area, app),
        Screen::ProtocolSelect => menus::draw_protocol_select(f, main_area, app),
        Screen::DeConfirm => menus::draw_de_confirm(f, main_area, app),
        Screen::Confirm => menus::draw_pre_install_confirm(f, main_area, app),
        Screen::FontPrep => menus::draw_font_prep(f, main_area, app),
        Screen::Wardrobe => menus::draw_wardrobe(f, main_area, app),
        Screen::ArgonConfig => menus::draw_argon_config(f, main_area, app),
        Screen::DockerConfig => menus::draw_docker_config(f, main_area, app),
        Screen::ChezmoiConfig => menus::draw_chezmoi_config(f, main_area, app),
        Screen::SystemSummary => menus::draw_system_summary(f, main_area, app),
        Screen::Password => menus::draw_password_screen(f, main_area, app),
        Screen::Authorization => menus::draw_auth_screen(f, main_area, app),
        Screen::Installing => menus::draw_installing(f, main_area, app),
        Screen::Done => menus::draw_done(f, main_area, app),
        Screen::Error => menus::draw_error(f, main_area, app),
    }

    draw_sys_status(f, stats_area, app);
    draw_station_intel(f, intel_area, app);
    draw_bbs_console(f, bbs_area, app);
}

fn draw_sys_status(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" SYS_STATUS ", theme::title_style()));
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let stats = vec![
        Line::from(vec![
            Span::styled("CPU: ", theme::dim_style()),
            Span::styled(
                format!("{:.1}%", app.sys_stats.cpu_pct),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("RAM: ", theme::dim_style()),
            Span::styled(
                format!("{} MB", app.sys_stats.ram_used_mb),
                theme::success_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("NET: ", theme::dim_style()),
            Span::styled(
                format!("{:.1} KB/s", app.sys_stats.net_rx_kbps),
                theme::success_style(),
            ),
        ]),
    ];
    f.render_widget(Paragraph::new(stats), inner);
}

fn draw_station_intel(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" STATION_INTEL ", theme::title_style()));
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let mut intel = vec![
        Line::from(vec![
            Span::styled("STATION: ", theme::dim_style()),
            Span::styled("01", theme::success_style()),
        ]),
        Line::from(vec![
            Span::styled("USER:    ", theme::dim_style()),
            Span::styled(whoami::username(), theme::success_style()),
        ]),
        Line::from(""),
        Line::from(Span::styled("CURRENT_SCREEN:", theme::dim_style())),
        Line::from(Span::styled(
            format!("{:?}", app.screen).to_uppercase(),
            theme::accent_style(),
        )),
    ];

    if let Some(driver) = app.drivers.get(app.selected_driver_idx) {
        intel.push(Line::from(""));
        intel.push(Line::from(Span::styled(
            "DRIVER_LOADED:",
            theme::dim_style(),
        )));
        intel.push(Line::from(Span::styled(
            driver.name().to_uppercase(),
            theme::success_style(),
        )));
    }

    f.render_widget(Paragraph::new(intel), inner);
}

fn draw_bbs_console(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" BBS_CONSOLE ", theme::title_style()));
    f.render_widget(&block, area);
    let inner = block.inner(area);

    f.render_widget(
        Paragraph::new(app.bbs_msg.clone()).style(theme::accent_style()),
        inner,
    );
}
