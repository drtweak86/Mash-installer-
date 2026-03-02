//! 1984 Retro-Station Rendering — Single-pane command flow.

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::tui::app::{LogLevel, Screen, TuiApp};
use crate::tui::confirmation;
use crate::tui::info_box;
use crate::tui::menus;
use crate::tui::theme;

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

// ── Top-level dispatch ────────────────────────────────────────────────────────

pub fn draw(f: &mut Frame, app: &TuiApp) {
    // Fill background with pure black
    f.render_widget(Block::default().style(theme::default_style()), f.area());

    // Outer Chrome
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

    // Draw info box and get remaining area
    let main_layout_area = info_box::draw_info_box(f, inner_chrome, app);

    // ── 4-Tile Layout Implementation ──────────────────────────────────────────

    // Split Vertical: (Top Content + Sidebars) and (Bottom BBS)
    let root_v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Top section
            Constraint::Length(3), // BBS Console (Panel 4) - Spans width
        ])
        .split(main_layout_area);

    // Split Horizontal: (Main Content) and (Stats + Intel)
    let top_h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left: Main Content (Panel 1)
            Constraint::Length(40), // Right: Stats + Intel (widened for better hardware display)
        ])
        .split(root_v_chunks[0]);

    // Split Right Vertical: (Stats) and (Intel)
    let right_v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // System Status (Panel 2) - slightly taller
            Constraint::Min(0),     // Station Intel (Panel 3)
        ])
        .split(top_h_chunks[1]);

    let main_area = top_h_chunks[0];
    let stats_area = right_v_chunks[0];
    let intel_area = right_v_chunks[1];
    let bbs_area = root_v_chunks[1];

    // ── Render Panels ─────────────────────────────────────────────────────────

    // Panel 1: Main Content
    match app.screen {
        Screen::Welcome => menus::draw_welcome(f, main_area, app),
        Screen::ArchDetected => menus::draw_arch_detected(f, main_area, app),
        Screen::DistroSelect => menus::draw_distro_select(f, main_area, app),
        Screen::ProfileSelect => menus::draw_profile_select(f, main_area, app),
        Screen::ModuleSelect => menus::draw_module_select(f, main_area, app),
        Screen::ThemeSelect => menus::draw_theme_select(f, main_area, app),
        Screen::SoftwareMode => menus::draw_software_mode_select(f, main_area, app),
        Screen::SoftwareSelect => menus::draw_software_select(f, main_area, app),
        Screen::DeSelect => menus::draw_de_select(f, main_area, app),
        Screen::ProtocolSelect => menus::draw_protocol_select(f, main_area, app),
        Screen::DeConfirm => menus::draw_de_confirm(f, main_area, app),
        Screen::Confirm => menus::draw_pre_install_confirm(f, main_area, app),
        Screen::FontPrep => menus::draw_font_prep(f, main_area, app),
        Screen::Wardrobe => menus::draw_wardrobe(f, main_area, app),
        Screen::SystemSummary => menus::draw_system_summary(f, main_area, app),
        Screen::Installing | Screen::Password | Screen::Authorization => draw_terminal_buffer(f, main_area, app),
        Screen::Done => draw_summary(f, app, false),
        Screen::Error => draw_summary(f, app, true),
    }

    // Panel 2: System Status
    draw_stats_panel(f, stats_area, app);

    // Panel 3: Station Intel
    draw_intel_panel(f, intel_area, app);

    // Panel 4: BBS Console
    draw_bbs_panel(f, bbs_area, app);

    // ── Overlay Modals (Visible on any screen) ────────────────────────────────

    // Long process confirmation (highest priority overlay)
    if app.long_process_state.is_some() {
        confirmation::draw_long_process_confirm(f, f.area(), app);
    }

    if let Some(state) = &app.password_state {
        menus::draw_password_prompt(f, f.area(), app, state);
    }

    if let Some(state) = &app.auth_state {
        menus::draw_auth_prompt(f, f.area(), app, state);
    }

    if let Some(_state) = &app.confirm_state {
        if app.screen == Screen::Installing || app.screen == Screen::Password {
            menus::draw_mid_install_confirm(f, f.area(), app);
        }
    }
}

#[allow(dead_code)]
pub fn draw_installing(_f: &mut Frame, _app: &TuiApp) {
    // Deprecated in favor of universal 4-tile draw
}

// ── BBS Panel (Panel 4) ──────────────────────────────────────────────────────

fn draw_bbs_panel(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::inner_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(" BBS_CONSOLE ", theme::accent_style()))
        .style(theme::default_style());

    let text = Paragraph::new(format!(" > {}", app.bbs_msg))
        .style(theme::success_style())
        .alignment(Alignment::Left)
        .block(block);

    f.render_widget(text, area);
}

// ── Stats Panel (Panel 2) ─────────────────────────────────────────────────────

fn draw_stats_panel(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::inner_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(" SYSTEM_STATUS ", theme::accent_style()))
        .style(theme::default_style());
    let inner = block.inner(area);
    f.render_widget(block, area);

    let s = &app.sys_stats;
    let ram_pct = if s.ram_total_mb > 0 {
        (s.ram_used_mb * 100 / s.ram_total_mb).min(100)
    } else {
        0
    };

    let stats_text = vec![
        Line::from(vec![
            Span::styled(" CPU: ", theme::dim_style()),
            Span::styled(format!("{:3}%", s.cpu_pct), theme::success_style()),
        ]),
        Line::from(vec![
            Span::styled(" RAM: ", theme::dim_style()),
            Span::styled(format!("{:3}%", ram_pct), theme::success_style()),
        ]),
        Line::from(vec![
            Span::styled(" NET: ", theme::dim_style()),
            Span::styled(format!("{:.1}kB/s", s.net_rx_kbps), theme::success_style()),
        ]),
        Line::from(vec![
            Span::styled(" PRG: ", theme::dim_style()),
            if app.screen == Screen::Installing || app.screen == Screen::Password {
                let progress = if app.total_phases > 0 {
                    (app.current_phase as f32 / app.total_phases as f32 * 100.0) as u16
                } else {
                    0
                };
                Span::styled(format!("{:3}%", progress), theme::accent_style())
            } else {
                Span::styled("IDLE", theme::success_style())
            },
        ]),
    ];

    let para = Paragraph::new(stats_text).alignment(Alignment::Left);
    f.render_widget(para, inner);
}

// ── Intel Panel (Panel 3) ─────────────────────────────────────────────────────

fn draw_intel_panel(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::inner_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(" STATION_INTEL ", theme::accent_style()))
        .style(theme::default_style());
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut info = Vec::new();

    if let Some(p) = &app.system_profile {
        info.push(Line::from(vec![
            Span::styled("> OS:   ", theme::dim_style()),
            Span::styled(p.distro.pretty_name.to_uppercase(), theme::success_style()),
        ]));
        info.push(Line::from(vec![
            Span::styled("> ARCH: ", theme::dim_style()),
            Span::styled(p.cpu.arch.to_uppercase(), theme::success_style()),
        ]));
        info.push(Line::from(""));
        
        info.push(Line::from(vec![
            Span::styled("> CPU:  ", theme::dim_style()),
            Span::styled(p.cpu.model.to_uppercase(), theme::accent_style()),
        ]));
        info.push(Line::from(vec![
            Span::styled("> CORES:", theme::dim_style()),
            Span::styled(format!(" {}/{}", p.cpu.physical_cores, p.cpu.logical_cores), theme::success_style()),
        ]));
        
        let ram_gb = p.memory.ram_total_kb as f32 / (1024.0 * 1024.0);
        info.push(Line::from(vec![
            Span::styled("> RAM:  ", theme::dim_style()),
            Span::styled(format!(" {:.1} GB", ram_gb), theme::success_style()),
        ]));
        
        info.push(Line::from(""));
        
        info.push(Line::from(vec![
            Span::styled("> HW:   ", theme::dim_style()),
            Span::styled(p.platform.model.to_uppercase(), theme::accent_style()),
        ]));
    } else {
        info.push(Line::from(vec![
            Span::styled("> STATUS: ", theme::dim_style()),
            Span::styled("SCRYING PENDING...", theme::warning_style()),
        ]));
    }

    let para = Paragraph::new(info)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(para, inner);
}

// ── Terminal Buffer (Action Log + Phases) ─────────────────────────────────────

fn draw_terminal_buffer(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::inner_border_type())
        .border_style(theme::border_style())
        .style(theme::default_style());
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    // Small banner at the top of the buffer
    for line in BANNER.lines() {
        lines.push(Line::from(Span::styled(line, theme::accent_style())));
    }
    lines.push(Line::from(""));

    // Current phase status
    if let Some(current) = app.phases.get(app.current_phase.saturating_sub(1)) {
        lines.push(Line::from(vec![
            Span::styled("EXECUTING: ", theme::accent_style()),
            Span::styled(&current.name, theme::success_style()),
        ]));
    }

    // Progress bar (as a sequence of blocks)
    let progress = if app.total_phases == 0 {
        0
    } else {
        (app.current_phase as u64 * 100) / app.total_phases as u64
    };
    let bar_text = format!(
        "[{:_<width$}] {}%",
        "",
        progress,
        width = inner.width as usize - 10
    );
    lines.push(Line::from(Span::styled(bar_text, theme::success_style())));
    lines.push(Line::from(""));

    // Log entries
    for entry in &app.log {
        let ts_span = Span::styled(format!("[{}] ", entry.timestamp), theme::dim_style());
        let msg_style = match entry.level {
            LogLevel::Success => theme::success_style(),
            LogLevel::Warning => theme::warning_style(),
            LogLevel::Error => theme::error_style(),
            LogLevel::Info => theme::default_style(),
        };
        let msg_span = Span::styled(entry.text.clone(), msg_style);
        lines.push(Line::from(vec![ts_span, msg_span]));
    }

    let visible_height = inner.height as usize;
    let total_lines = lines.len();
    let start = total_lines.saturating_sub(visible_height);

    let buffer_para = Paragraph::new(Text::from(lines))
        .style(theme::default_style())
        .scroll((start as u16, 0))
        .wrap(Wrap { trim: false });
    f.render_widget(buffer_para, inner);
}

// ── Done / Error summary ──────────────────────────────────────────────────────

pub fn draw_summary(f: &mut Frame, app: &TuiApp, is_error: bool) {
    let area = f.area();

    // Get main area (excluding info box)
    let (main_area, _info_area) = info_box::get_main_area_with_info_box(area);

    let title = if is_error {
        " ! ABORTED ! "
    } else {
        " * SYSTEM CONFIGURED * "
    };
    let title_style = if is_error {
        theme::error_style()
    } else {
        theme::title_style()
    };

    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(theme::outer_border_type())
        .border_style(theme::border_style())
        .title(Span::styled(title, title_style))
        .style(theme::default_style());
    let inner = outer.inner(main_area);
    f.render_widget(outer, main_area);

    let mut lines: Vec<Line> = Vec::new();

    if let Some(msg) = &app.error_msg {
        lines.push(Line::from(Span::styled(
            "FATAL ERROR DETECTED",
            theme::error_style(),
        )));
        lines.push(Line::from(Span::styled(
            format!("MSG: {msg}"),
            theme::error_style(),
        )));
        lines.push(Line::from(""));
    }

    if let Some(report) = &app.report {
        let mut installed = Vec::new();
        let mut configured = Vec::new();
        let mut tweaked = Vec::new();

        for output in &report.outputs {
            installed.extend(output.actions_taken.clone());
            configured.extend(output.configured_actions.clone());
            tweaked.extend(output.tweaked_actions.clone());
        }

        if !installed.is_empty() {
            lines.push(Line::from(Span::styled("INSTALLED:", theme::accent_style())));
            for item in installed {
                lines.push(Line::from(vec![
                    Span::styled("  + ", theme::success_style()),
                    Span::styled(item, theme::default_style()),
                ]));
            }
            lines.push(Line::from(""));
        }

        if !configured.is_empty() {
            lines.push(Line::from(Span::styled("CONFIGURED:", theme::accent_style())));
            for item in configured {
                lines.push(Line::from(vec![
                    Span::styled("  * ", theme::success_style()),
                    Span::styled(item, theme::default_style()),
                ]));
            }
            lines.push(Line::from(""));
        }

        if !tweaked.is_empty() {
            lines.push(Line::from(Span::styled("TWEAKED:", theme::accent_style())));
            for item in tweaked {
                lines.push(Line::from(vec![
                    Span::styled("  > ", theme::success_style()),
                    Span::styled(item, theme::default_style()),
                ]));
            }
            lines.push(Line::from(""));
        }

        if !report.errors.is_empty() {
            lines.push(Line::from(Span::styled("INCIDENTS:", theme::error_style())));
            for err in &report.errors {
                lines.push(Line::from(Span::styled(
                    format!("  [ERR] {} : {}", err.phase, err.user_message()),
                    theme::error_style(),
                )));
            }
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "READY > _",
        theme::success_style(),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "PRESS Q TO RETURN TO SHELL",
        theme::dim_style(),
    )));

    let text = Text::from(lines);
    let para = Paragraph::new(text)
        .style(theme::default_style())
        .wrap(Wrap { trim: false });
    f.render_widget(para, inner);
}
