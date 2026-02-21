//! TuiApp state machine, TuiMessage enum, and the main run() event loop.

use std::collections::VecDeque;
use std::io;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use installer_core::{
    DistroDriver, InstallOptions, InstallationReport, PhaseEvent, ProfileLevel, SoftwareTierPlan,
    ThemePlan,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::software_catalog::SOFTWARE_CATEGORIES;
use crate::tui::bbs::spawn_bbs_cycler;
use crate::tui::observer::RatatuiPhaseObserver;
use crate::tui::render;
use crate::tui::sysinfo_poller::spawn_sysinfo_poller;
use std::collections::BTreeMap;

// ── Message bus ──────────────────────────────────────────────────────────────

pub enum TuiMessage {
    Phase(PhaseEvent),
    SysStats {
        cpu_pct: f32,
        ram_used_mb: u64,
        ram_total_mb: u64,
        net_rx_kbps: f32,
    },
    BbsMessage(String),
    ConfirmPrompt {
        prompt: String,
        reply: Sender<bool>,
    },
    #[allow(dead_code)]
    PasswordPrompt {
        reply: Sender<String>,
    },

    Done(Box<InstallationReport>),
    InstallError(String),
}

// ── Screen state machine ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Welcome,
    DistroSelect,
    ProfileSelect,
    ModuleSelect,
    ThemeSelect,
    SoftwareMode,
    SoftwareSelect,
    Confirm,
    #[allow(dead_code)]
    Password,
    Installing,
    Done,
    Error,
}

// ── Phase row (for the phase list in Main pane) ──────────────────────────────

#[derive(Debug, Clone)]
pub enum PhaseStatus {
    #[allow(dead_code)]
    Pending,
    Running,
    Done,
    Failed,
    Skipped,
}

#[derive(Debug, Clone)]
pub struct PhaseRow {
    pub name: String,
    pub status: PhaseStatus,
    pub description: String,
}

// ── Log entry ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub text: String,
    pub level: LogLevel,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

fn now_stamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let h = (secs / 3600) % 24;
    let m = (secs / 60) % 60;
    let s = secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

// ── Sys stats snapshot ───────────────────────────────────────────────────────

#[derive(Default, Clone)]
pub struct SysStats {
    pub cpu_pct: f32,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub net_rx_kbps: f32,
}

// ── Confirm prompt state ─────────────────────────────────────────────────────

pub struct ConfirmState {
    pub prompt: String,
    pub reply: Sender<bool>,
    pub selected: bool, // true = Yes, false = No
}

// ── Password prompt state ────────────────────────────────────────────────────

pub struct PasswordState {
    pub reply: Sender<String>,
    pub password: String,
}

// ── Module selection (mirrors menu::ModuleSelection) ─────────────────────────

#[derive(Debug, Clone, Default)]
pub struct ModuleState {
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
}

pub const MODULE_LABELS: &[(&str, &str)] = &[
    (
        "Argon One fan control",
        "Install Argon One scripts (Pi only)",
    ),
    ("Powerlevel10k prompt", "Enable p10k + zsh polish modules"),
    ("Docker data-root", "Manage Docker data-root inside staging"),
];

// ── App state ────────────────────────────────────────────────────────────────

pub struct TuiApp {
    // Screen state
    pub screen: Screen,
    // Menu navigation
    pub menu_cursor: usize,
    // Available drivers
    pub drivers: Vec<&'static dyn DistroDriver>,
    pub selected_driver_idx: usize,
    // Module selection
    pub modules: ModuleState,
    // Profile selection  (0=Minimal, 1=Dev, 2=Full)
    pub profile_idx: usize,
    // Theme selection
    pub theme_plan: ThemePlan,
    // Software tiers
    pub software_full: bool,
    pub software_picks: BTreeMap<&'static str, &'static str>,
    pub software_category_idx: usize,
    // Dry-run flag (passed in from CLI)
    pub dry_run: bool,
    pub continue_on_error: bool,
    // Installing phase state
    pub phases: Vec<PhaseRow>,
    pub current_phase: usize,
    pub total_phases: usize,
    pub start_time: Instant,
    // Log
    pub log: VecDeque<LogEntry>,
    // Stats
    pub sys_stats: SysStats,
    // BBS message
    pub bbs_msg: String,
    // Confirm prompt (mid-install)
    pub confirm_state: Option<ConfirmState>,
    // Password prompt (mid-install)
    pub password_state: Option<PasswordState>,
    // Final report
    pub report: Option<Box<InstallationReport>>,
    pub error_msg: Option<String>,
    // Channel for background messages
    pub tx: Sender<TuiMessage>,
    // Log scroll offset
    pub log_scroll: usize,
    // Summary scroll
    pub summary_scroll: usize,
    // Should quit
    pub should_quit: bool,
}

impl TuiApp {
    pub fn new(tx: Sender<TuiMessage>, drivers: Vec<&'static dyn DistroDriver>) -> Self {
        Self {
            screen: Screen::Welcome,
            menu_cursor: 0,
            drivers,
            selected_driver_idx: 0,
            modules: ModuleState::default(),
            profile_idx: 1, // Dev by default
            theme_plan: ThemePlan::None,
            software_full: true,
            software_picks: BTreeMap::new(),
            software_category_idx: 0,
            dry_run: false,
            continue_on_error: false,
            phases: Vec::new(),
            current_phase: 0,
            total_phases: 0,
            start_time: Instant::now(),
            log: VecDeque::with_capacity(500),
            sys_stats: SysStats::default(),
            bbs_msg: "⚡ Initialising the forge...".to_string(),
            confirm_state: None,
            password_state: None,
            report: None,
            error_msg: None,
            tx,
            log_scroll: 0,
            summary_scroll: 0,
            should_quit: false,
        }
    }

    pub fn push_log(&mut self, text: impl Into<String>, level: LogLevel) {
        if self.log.len() >= 500 {
            self.log.pop_front();
        }
        self.log.push_back(LogEntry {
            timestamp: now_stamp(),
            text: text.into(),
            level,
        });
        // Auto-scroll to bottom
        self.log_scroll = self.log.len().saturating_sub(1);
    }

    fn profile_level(&self) -> ProfileLevel {
        match self.profile_idx {
            0 => ProfileLevel::Minimal,
            1 => ProfileLevel::Dev,
            _ => ProfileLevel::Full,
        }
    }

    fn build_options(&self) -> InstallOptions {
        InstallOptions {
            profile: self.profile_level(),
            staging_dir: None,
            dry_run: self.dry_run,
            interactive: false, // TUI handles interaction
            enable_argon: self.modules.enable_argon,
            enable_p10k: self.modules.enable_p10k,
            docker_data_root: self.modules.docker_data_root,
            continue_on_error: self.continue_on_error,
            software_plan: self.build_software_plan(),
        }
    }

    fn build_software_plan(&self) -> SoftwareTierPlan {
        let picks = if self.software_full {
            let mut picks = BTreeMap::new();
            for category in SOFTWARE_CATEGORIES {
                if let Some(recommended) = category.options.first() {
                    picks.insert(category.label, recommended.name);
                }
            }
            picks
        } else {
            self.software_picks.clone()
        };
        SoftwareTierPlan::new(self.software_full, picks, self.theme_plan.clone())
    }

    fn spawn_installer(&self, driver: &'static dyn DistroDriver) {
        let options = self.build_options();
        let tx = self.tx.clone();
        thread::spawn(move || {
            let mut observer = RatatuiPhaseObserver::new(tx.clone());
            match installer_core::run_with_driver(driver, options, &mut observer) {
                Ok(report) => {
                    let _ = tx.send(TuiMessage::Done(Box::new(report)));
                }
                Err(err) => {
                    let msg = format!("{}", err);
                    // err.report is already Box<InstallationReport>
                    let _ = tx.send(TuiMessage::Done(err.report));
                    let _ = tx.send(TuiMessage::InstallError(msg));
                }
            }
        });
    }

    // ── Keyboard handling ────────────────────────────────────────────────────

    pub fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        // Global quit
        if code == KeyCode::Char('q')
            && (modifiers == KeyModifiers::NONE || modifiers == KeyModifiers::SHIFT)
            && self.screen != Screen::Installing
        {
            self.should_quit = true;
            return;
        }
        if code == KeyCode::Char('c') && modifiers == KeyModifiers::CONTROL {
            self.should_quit = true;
            return;
        }

        // Numeric selection support
        if let KeyCode::Char(c) = code {
            if c.is_ascii_digit() && c != '0' {
                let val = c.to_digit(10).unwrap() as usize;
                self.handle_numeric_input(val);
            }
        }

        // Copy screen so we don't hold a borrow into self while calling &mut self methods
        let screen = self.screen;
        match screen {
            Screen::Welcome => {
                if code == KeyCode::Enter || code == KeyCode::Char(' ') {
                    self.screen = Screen::DistroSelect;
                    self.menu_cursor = 0;
                }
            }
            Screen::DistroSelect => {
                let len = self.drivers.len();
                self.handle_list_key(code, len);
            }
            Screen::ProfileSelect => self.handle_list_key(code, 3),
            Screen::ModuleSelect => self.handle_module_key(code),
            Screen::ThemeSelect => self.handle_list_key(code, 3),
            Screen::Password => self.handle_password_key(code),

            Screen::SoftwareMode => self.handle_list_key(code, 2),
            Screen::SoftwareSelect => self.handle_software_key(code),
            Screen::Confirm => self.handle_confirm_key(code),
            Screen::Installing => self.handle_installing_key(code),
            Screen::Done | Screen::Error => match code {
                KeyCode::Up => {
                    self.summary_scroll = self.summary_scroll.saturating_sub(1);
                }
                KeyCode::Down => {
                    self.summary_scroll += 1;
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.should_quit = true;
                }
                _ => {}
            },
        }
    }

    fn handle_password_key(&mut self, code: KeyCode) {
        if let Some(ref mut s) = self.password_state {
            match code {
                KeyCode::Char(c) => {
                    s.password.push(c);
                }
                KeyCode::Backspace => {
                    s.password.pop();
                }
                KeyCode::Enter => {
                    if let Some(s) = self.password_state.take() {
                        let _ = s.reply.send(s.password);
                        self.screen = Screen::Installing;
                    }
                }
                KeyCode::Esc => {
                    if let Some(s) = self.password_state.take() {
                        let _ = s.reply.send(String::new());
                        self.screen = Screen::Installing;
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_numeric_input(&mut self, val: usize) {
        let idx = val.saturating_sub(1);
        match self.screen {
            Screen::DistroSelect => {
                if idx < self.drivers.len() {
                    self.menu_cursor = idx;
                    self.advance_from_list();
                }
            }
            Screen::ProfileSelect => {
                if idx < 3 {
                    self.menu_cursor = idx;
                    self.advance_from_list();
                }
            }
            Screen::ThemeSelect => {
                if idx < 3 {
                    self.menu_cursor = idx;
                    self.advance_from_list();
                }
            }
            Screen::SoftwareMode => {
                if idx < 2 {
                    self.menu_cursor = idx;
                    self.advance_from_list();
                }
            }
            Screen::SoftwareSelect => {
                if let Some(cat) = SOFTWARE_CATEGORIES.get(self.software_category_idx) {
                    if idx < cat.options.len() {
                        self.menu_cursor = idx;
                        let chosen = &cat.options[self.menu_cursor];
                        self.software_picks.insert(cat.label, chosen.name);
                        if self.software_category_idx + 1 >= SOFTWARE_CATEGORIES.len() {
                            self.screen = Screen::Confirm;
                            self.menu_cursor = 0;
                        } else {
                            self.software_category_idx += 1;
                            self.menu_cursor = self
                                .selected_option_index(self.software_category_idx)
                                .unwrap_or(0);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_list_key(&mut self, code: KeyCode, list_len: usize) {
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_cursor > 0 {
                    self.menu_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_cursor + 1 < list_len {
                    self.menu_cursor += 1;
                }
            }
            KeyCode::Enter => {
                self.advance_from_list();
            }
            KeyCode::Esc => {
                self.go_back();
            }
            _ => {}
        }
    }

    fn handle_module_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_cursor > 0 {
                    self.menu_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_cursor + 1 < MODULE_LABELS.len() {
                    self.menu_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                self.toggle_module(self.menu_cursor);
            }
            KeyCode::Enter => {
                self.screen = Screen::ThemeSelect;
                self.menu_cursor = self.theme_menu_index();
            }
            KeyCode::Esc => {
                self.go_back();
            }
            _ => {}
        }
    }

    fn toggle_module(&mut self, idx: usize) {
        match idx {
            0 => self.modules.enable_argon = !self.modules.enable_argon,
            1 => self.modules.enable_p10k = !self.modules.enable_p10k,
            2 => self.modules.docker_data_root = !self.modules.docker_data_root,
            _ => {}
        }
    }

    fn handle_confirm_key(&mut self, code: KeyCode) {
        if self.confirm_state.is_none() {
            // Pre-install confirm screen
            match code {
                KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.start_install();
                }
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                    self.screen = Screen::SoftwareMode;
                    self.menu_cursor = if self.software_full { 0 } else { 1 };
                }
                _ => {}
            }
            return;
        }
        // Mid-install confirm prompt — borrows are scoped to each arm
        match code {
            KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                if let Some(ref mut s) = self.confirm_state {
                    s.selected = !s.selected;
                }
            }
            KeyCode::Enter => {
                if let Some(s) = self.confirm_state.take() {
                    let _ = s.reply.send(s.selected);
                    // Return to Installing screen after user answers
                    self.screen = Screen::Installing;
                }
            }
            KeyCode::Esc => {
                if let Some(s) = self.confirm_state.take() {
                    let _ = s.reply.send(false);
                    self.screen = Screen::Installing;
                }
            }
            _ => {}
        }
    }

    fn handle_installing_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up => {
                self.log_scroll = self.log_scroll.saturating_sub(1);
            }
            KeyCode::Down => {
                self.log_scroll = self.log_scroll.saturating_add(1);
            }
            _ => {}
        }
    }

    fn advance_from_list(&mut self) {
        match self.screen {
            Screen::DistroSelect => {
                self.selected_driver_idx = self.menu_cursor;
                self.screen = Screen::ProfileSelect;
                self.menu_cursor = self.profile_idx;
            }
            Screen::ProfileSelect => {
                self.profile_idx = self.menu_cursor;
                self.screen = Screen::ModuleSelect;
                self.menu_cursor = 0;
            }
            Screen::ThemeSelect => {
                self.theme_plan = match self.menu_cursor {
                    0 => ThemePlan::RetroOnly,
                    1 => ThemePlan::RetroWithWallpapers,
                    _ => ThemePlan::None,
                };
                self.screen = Screen::SoftwareMode;
                self.menu_cursor = if self.software_full { 0 } else { 1 };
            }
            Screen::SoftwareMode => {
                if self.menu_cursor == 0 {
                    self.software_full = true;
                    self.software_picks.clear();
                    self.screen = Screen::Confirm;
                    self.menu_cursor = 0;
                } else {
                    self.software_full = false;
                    self.software_picks.clear();
                    self.software_category_idx = 0;
                    self.menu_cursor = 0;
                    self.screen = Screen::SoftwareSelect;
                }
            }
            _ => {}
        }
    }

    fn go_back(&mut self) {
        match self.screen {
            Screen::DistroSelect => self.screen = Screen::Welcome,
            Screen::ProfileSelect => {
                self.screen = Screen::DistroSelect;
                self.menu_cursor = self.selected_driver_idx;
            }
            Screen::ModuleSelect => {
                self.screen = Screen::ProfileSelect;
                self.menu_cursor = self.profile_idx;
            }
            Screen::ThemeSelect => {
                self.screen = Screen::ModuleSelect;
                self.menu_cursor = 0;
            }
            Screen::SoftwareMode => {
                self.screen = Screen::ThemeSelect;
                self.menu_cursor = self.theme_menu_index();
            }
            Screen::SoftwareSelect => {
                if self.software_category_idx == 0 {
                    self.screen = Screen::SoftwareMode;
                    self.menu_cursor = if self.software_full { 0 } else { 1 };
                } else {
                    self.software_category_idx = self.software_category_idx.saturating_sub(1);
                    self.menu_cursor = self
                        .selected_option_index(self.software_category_idx)
                        .unwrap_or(0);
                }
            }
            Screen::Confirm => {
                self.screen = Screen::SoftwareMode;
                self.menu_cursor = if self.software_full { 0 } else { 1 };
            }
            _ => {}
        }
    }

    fn handle_software_key(&mut self, code: KeyCode) {
        let category = match SOFTWARE_CATEGORIES.get(self.software_category_idx) {
            Some(category) => category,
            None => {
                self.screen = Screen::Confirm;
                self.menu_cursor = 0;
                return;
            }
        };
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_cursor > 0 {
                    self.menu_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_cursor + 1 < category.options.len() {
                    self.menu_cursor += 1;
                }
            }
            KeyCode::Enter => {
                let chosen = &category.options[self.menu_cursor];
                self.software_picks.insert(category.label, chosen.name);
                if self.software_category_idx + 1 >= SOFTWARE_CATEGORIES.len() {
                    self.screen = Screen::Confirm;
                    self.menu_cursor = 0;
                } else {
                    self.software_category_idx += 1;
                    self.menu_cursor = self
                        .selected_option_index(self.software_category_idx)
                        .unwrap_or(0);
                }
            }
            KeyCode::Esc => self.go_back(),
            _ => {}
        }
    }

    fn selected_option_index(&self, category_idx: usize) -> Option<usize> {
        let category = SOFTWARE_CATEGORIES.get(category_idx)?;
        let picked = self.software_picks.get(category.label)?;
        category.options.iter().position(|opt| opt.name == *picked)
    }

    fn theme_menu_index(&self) -> usize {
        match self.theme_plan {
            ThemePlan::RetroOnly => 0,
            ThemePlan::RetroWithWallpapers => 1,
            ThemePlan::None => 2,
        }
    }

    pub fn theme_plan_label(&self) -> &'static str {
        match self.theme_plan {
            ThemePlan::RetroOnly => "BBC/UNIX Retro Theme",
            ThemePlan::RetroWithWallpapers => "Retro Theme + Wallpapers",
            ThemePlan::None => "No theme changes",
        }
    }

    pub fn software_plan_label(&self) -> String {
        if self.software_full {
            "Full S-tier".to_string()
        } else {
            format!(
                "Custom ({}/{})",
                self.software_picks.len(),
                SOFTWARE_CATEGORIES.len()
            )
        }
    }

    fn start_install(&mut self) {
        let driver = self.drivers[self.selected_driver_idx];
        self.screen = Screen::Installing;
        self.start_time = Instant::now();
        self.push_log("Installation started", LogLevel::Info);
        self.push_log(
            format!("Driver: {} — {}", driver.name(), driver.description()),
            LogLevel::Info,
        );
        self.push_log(
            format!("Profile: {:?}", self.profile_level()),
            LogLevel::Info,
        );
        self.push_log(
            format!("Theme: {}", self.theme_plan_label()),
            LogLevel::Info,
        );
        self.push_log(
            format!("Software plan: {}", self.software_plan_label()),
            LogLevel::Info,
        );
        self.spawn_installer(driver);
    }

    // ── Message dispatch ─────────────────────────────────────────────────────

    pub fn handle_message(&mut self, msg: TuiMessage) {
        match msg {
            TuiMessage::Phase(event) => self.handle_phase_event(event),
            TuiMessage::SysStats {
                cpu_pct,
                ram_used_mb,
                ram_total_mb,
                net_rx_kbps,
            } => {
                self.sys_stats = SysStats {
                    cpu_pct,
                    ram_used_mb,
                    ram_total_mb,
                    net_rx_kbps,
                };
            }
            TuiMessage::BbsMessage(msg) => {
                self.bbs_msg = msg;
            }
            TuiMessage::ConfirmPrompt { prompt, reply } => {
                self.confirm_state = Some(ConfirmState {
                    prompt,
                    reply,
                    selected: true,
                });
                self.screen = Screen::Confirm;
            }
            TuiMessage::PasswordPrompt { reply } => {
                self.password_state = Some(PasswordState {
                    reply,
                    password: String::new(),
                });
                self.screen = Screen::Password;
            }

            TuiMessage::Done(report) => {
                self.report = Some(report);
                if self.error_msg.is_none() {
                    self.screen = Screen::Done;
                    self.push_log("Installation complete!", LogLevel::Success);
                }
            }
            TuiMessage::InstallError(msg) => {
                self.error_msg = Some(msg.clone());
                self.screen = Screen::Error;
                self.push_log(format!("ERROR: {msg}"), LogLevel::Error);
            }
        }
    }

    fn handle_phase_event(&mut self, event: PhaseEvent) {
        match &event {
            PhaseEvent::Total { total } => {
                self.total_phases = *total;
                self.phases = Vec::with_capacity(*total);
                self.push_log(format!("Total phases: {total}"), LogLevel::Info);
            }
            PhaseEvent::Started {
                index,
                total,
                phase,
            } => {
                self.current_phase = *index;
                self.total_phases = *total;
                // Update or add phase row
                if let Some(row) = self.phases.iter_mut().find(|r| r.name == *phase) {
                    row.status = PhaseStatus::Running;
                } else {
                    self.phases.push(PhaseRow {
                        name: phase.clone(),
                        status: PhaseStatus::Running,
                        description: String::new(),
                    });
                }
                self.push_log(
                    format!("[{index}/{total}] Starting: {phase}"),
                    LogLevel::Info,
                );
            }
            PhaseEvent::Completed {
                phase, description, ..
            } => {
                if let Some(row) = self.phases.iter_mut().find(|r| r.name == *phase) {
                    row.status = PhaseStatus::Done;
                    row.description = description.clone();
                }
                self.push_log(format!("✓ {phase}: {description}"), LogLevel::Success);
            }
            PhaseEvent::Failed { phase, error, .. } => {
                if let Some(row) = self.phases.iter_mut().find(|r| r.name == *phase) {
                    row.status = PhaseStatus::Failed;
                }
                self.push_log(format!("✗ {phase}: {error}"), LogLevel::Error);
            }
            PhaseEvent::Skipped { phase, .. } => {
                if let Some(row) = self.phases.iter_mut().find(|r| r.name == *phase) {
                    row.status = PhaseStatus::Skipped;
                } else {
                    self.phases.push(PhaseRow {
                        name: phase.clone(),
                        status: PhaseStatus::Skipped,
                        description: "skipped".to_string(),
                    });
                }
                self.push_log(format!("– {phase} (skipped)"), LogLevel::Info);
            }
            PhaseEvent::Warning { message } => {
                self.push_log(format!("⚠ {message}"), LogLevel::Warning);
            }
        }
    }
}

// ── Terminal guard ────────────────────────────────────────────────────────────

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Run the full TUI. Returns when the user quits or installation finishes.
pub fn run(
    drivers: Vec<&'static dyn DistroDriver>,
    dry_run: bool,
    continue_on_error: bool,
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

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        // Draw
        terminal.draw(|f| render::draw(f, &app))?;

        // Drain messages from background threads / installer
        loop {
            match rx.try_recv() {
                Ok(msg) => app.handle_message(msg),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => break,
            }
        }

        // Poll keyboard with remaining tick time
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
