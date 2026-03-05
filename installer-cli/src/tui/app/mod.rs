pub mod input;
pub mod message;
pub mod navigation;
pub mod software;

use std::collections::VecDeque;
use std::io;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::tui::bbs::spawn_bbs_cycler;
use crate::tui::observer::RatatuiPhaseObserver;
use crate::tui::render;
use crate::tui::sysinfo_poller::spawn_sysinfo_poller;
use installer_core::{detect_platform, DistroDriver, ProfileLevel, SystemProfileExt};
use std::collections::BTreeMap;

pub use crate::tui::state::{
    LogEntry, LogLevel, ModuleState, Screen, SoftwareMode, SysStats, TuiApp, TuiMessage,
    MODULE_LABELS,
};

impl TuiApp {
    pub fn new(tx: Sender<TuiMessage>, drivers: Vec<&'static dyn DistroDriver>) -> Self {
        Self {
            screen: Screen::Welcome,
            navigation_history: Vec::new(),
            navigation_context: String::from("Welcome to MASH Installer"),
            menu_cursor: 0,
            drivers,
            selected_driver_idx: 0,
            modules: ModuleState::default(),
            profile_idx: 1, // Dev by default
            desktop_environment: None,
            display_protocol: installer_core::desktop::DisplayProtocol::Auto,
            theme_plan: installer_core::ThemePlan::None,
            software_mode: SoftwareMode::BardsRecommendations,
            catalog: installer_core::catalog::Catalog::load_s_tier().unwrap_or_default(),
            software_picks: BTreeMap::new(),
            software_category_idx: 0,
            dry_run: false,
            continue_on_error: false,
            platform_info: installer_core::platform::PlatformInfo {
                arch: std::env::consts::ARCH.to_string(),
                distro: "unknown".to_string(),
                distro_version: "unknown".to_string(),
                distro_codename: "unknown".to_string(),
                distro_family: "unknown".to_string(),
                pi_model: None,
                cpu_model: "Unknown".to_string(),
                cpu_cores: 0,
                ram_total_gb: 0.0,
            },
            system_profile: None,
            phases: Vec::new(),
            current_phase: 0,
            total_phases: 0,
            start_time: Instant::now(),
            progress_pct: 0.0,
            log: VecDeque::with_capacity(500),
            sys_stats: SysStats::default(),
            bbs_msg: "⚡ Initialising the forge...".to_string(),
            available_presets: installer_core::preset::PresetRegistry::load_all()
                .map(|r| r.presets.into_values().collect())
                .unwrap_or_default(),
            selected_preset_idx: 0,
            confirm_state: None,
            long_process_state: None,
            password_state: None,
            auth_state: None,
            report: None,
            error_msg: None,
            tx,
            log_scroll: 0,
            summary_scroll: 0,
            scry: false,
            scry_port: 3030,
            environment: installer_core::model::options::EnvironmentTag::Home,
            should_quit: false,
        }
    }

    pub fn push_log(&mut self, text: impl Into<String>, level: LogLevel) {
        if self.log.len() >= 500 {
            self.log.pop_front();
        }
        self.log.push_back(LogEntry {
            timestamp: crate::tui::app::now_stamp(),
            text: text.into(),
            level,
        });
        // Auto-scroll to bottom
        self.log_scroll = self.log.len().saturating_sub(1);
    }

    pub fn tick(&mut self) {
        // Update long process confirmation countdown
        if self.long_process_state.is_some() {
            let _ = self.update_long_process_confirmation();
        }
    }

    pub fn profile_level(&self) -> ProfileLevel {
        match self.profile_idx {
            0 => ProfileLevel::Minimal,
            1 => ProfileLevel::Dev,
            _ => ProfileLevel::Full,
        }
    }

    pub fn environment(&self) -> installer_core::model::options::EnvironmentTag {
        self.environment
    }

    pub fn spawn_system_scan(&self) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            // Give the UI a moment to show the scan screen
            thread::sleep(Duration::from_millis(1500));

            let platform = detect_platform().unwrap_or_else(|_| installer_core::platform::PlatformInfo {
                arch: std::env::consts::ARCH.to_string(),
                distro: "unknown".to_string(),
                distro_version: "unknown".to_string(),
                distro_codename: "unknown".to_string(),
                distro_family: "unknown".to_string(),
                pi_model: None,
                cpu_model: "Unknown".to_string(),
                cpu_cores: 0,
                ram_total_gb: 0.0,
            });

            let profile = installer_core::SystemProfile::detect(&installer_core::REAL_SYSTEM)
                .unwrap_or_default();

            let _ = tx.send(TuiMessage::ScanComplete { platform, profile });
        });
    }

    pub fn spawn_installer(&self, driver: &'static dyn DistroDriver) {
        let options = self.build_options();
        let tx = self.tx.clone();
        let scry = self.scry;
        let scry_port = self.scry_port;

        thread::spawn(move || {
            let mut composite = installer_core::CompositeObserver::new();

            // 1. TUI Observer
            let tui_observer = RatatuiPhaseObserver::new(tx.clone());
            composite.add(tui_observer);

            // 2. Remote Scrying (Websocket)
            if scry {
                let scryer = installer_core::WebsocketObserver::new(scry_port);
                composite.add(scryer);
            }

            match installer_core::run_with_driver(driver, options, &mut composite) {
                Ok(report) => {
                    let _ = tx.send(TuiMessage::Done(Box::new(report)));
                }
                Err(err) => {
                    let msg = format!("{}", err);
                    let _ = tx.send(TuiMessage::Done(err.report));
                    let _ = tx.send(TuiMessage::InstallError(msg));
                }
            }
        });
    }
}

pub fn now_stamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let h = (secs / 3600) % 24;
    let m = (secs / 60) % 60;
    let s = secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn enter() -> io::Result<Self> {
        use crossterm::execute;
        use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        use crossterm::execute;
        use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

pub fn run(
    drivers: Vec<&'static dyn DistroDriver>,
    dry_run: bool,
    continue_on_error: bool,
    scry: bool,
    scry_port: u16,
) -> anyhow::Result<()> {
    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel::<TuiMessage>();

    // Spawn background threads
    spawn_bbs_cycler(tx.clone());
    spawn_sysinfo_poller(tx.clone());

    let mut app = TuiApp::new(tx, drivers);
    app.dry_run = dry_run;
    app.continue_on_error = continue_on_error;
    app.scry = scry;
    app.scry_port = scry_port;

    // Start at Welcome screen
    app.screen = Screen::Welcome;

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        app.tick();
        terminal.draw(|f| render::draw(f, &app))?;

        loop {
            match rx.try_recv() {
                Ok(msg) => app.handle_message(msg),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => break,
            }
        }

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key.code, key.modifiers);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
