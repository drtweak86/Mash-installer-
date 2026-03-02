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
    detect_platform, DistroDriver, InstallOptions, InstallationReport, PhaseEvent, ProfileLevel,
    SoftwareTierPlan, ThemePlan,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::tui::bbs::spawn_bbs_cycler;
use crate::tui::confirmation::LongProcessState;
use crate::tui::observer::RatatuiPhaseObserver;
use crate::tui::render;
use crate::tui::sysinfo_poller::spawn_sysinfo_poller;
use installer_core::catalog::Program;
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
    AuthRequest {
        auth_type: installer_core::AuthType,
        reply: Sender<bool>,
    },

    Done(Box<InstallationReport>),
    InstallError(String),
}

// ── Screen state machine ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Welcome,
    ArchDetected,
    DistroSelect,
    ProfileSelect,
    ModuleSelect,
    ThemeSelect,
    SoftwareMode,
    SoftwareSelect,
    Confirm,
    DeSelect,
    ProtocolSelect,
    DeConfirm,
    FontPrep,
    Wardrobe,
    SystemSummary,
    #[allow(dead_code)]
    Password,
    Authorization,
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

// ── Auth prompt state ────────────────────────────────────────────────────────

pub struct AuthState {
    pub auth_type: installer_core::AuthType,
    pub reply: Sender<bool>,
    pub selected: bool,
}

// ── Long process confirmation state ─────────────────────────────────────────

// ── Module selection (mirrors menu::ModuleSelection) ─────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SoftwareMode {
    #[default]
    BardsRecommendations,
    Auto,
    Manual,
}

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
    // Navigation state for enhanced install flow
    pub navigation_history: Vec<Screen>,
    pub navigation_context: String, // Human-readable description of current context
    // Menu navigation
    pub menu_cursor: usize,
    // Available drivers
    pub drivers: Vec<&'static dyn DistroDriver>,
    pub selected_driver_idx: usize,
    // Module selection
    pub modules: ModuleState,
    // Profile selection  (0=Minimal, 1=Dev, 2=Full)
    pub profile_idx: usize,
    // Desktop environment selection
    pub desktop_environment: Option<installer_core::desktop::DesktopEnvironment>,
    pub display_protocol: installer_core::desktop::DisplayProtocol,
    // Theme selection
    pub theme_plan: ThemePlan,
    // Software tiers
    pub software_mode: SoftwareMode,
    pub catalog: installer_core::catalog::Catalog,
    pub software_picks: BTreeMap<String, String>, // Category Name -> Program ID
    pub software_category_idx: usize,
    // Dry-run flag (passed in from CLI)
    pub dry_run: bool,
    pub continue_on_error: bool,
    pub platform_info: installer_core::platform::PlatformInfo,
    pub system_profile: Option<installer_core::SystemProfile>,
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
    // Arch detection timer
    pub arch_timer: Option<Instant>,
    // Presets
    pub available_presets: Vec<installer_core::preset::Preset>,
    pub selected_preset_idx: usize,
    // Confirm prompt (mid-install)
    pub confirm_state: Option<ConfirmState>,
    // Long process confirmation
    pub long_process_state: Option<LongProcessState>,
    // Password prompt (mid-install)
    pub password_state: Option<PasswordState>,
    // Auth prompt (mid-install)
    pub auth_state: Option<AuthState>,
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
            navigation_history: Vec::new(),
            navigation_context: String::from("Welcome to MASH Installer"),
            menu_cursor: 0,
            drivers,
            selected_driver_idx: 0,
            modules: ModuleState::default(),
            profile_idx: 1, // Dev by default
            desktop_environment: None,
            display_protocol: installer_core::desktop::DisplayProtocol::Auto,
            theme_plan: ThemePlan::None,
            software_mode: SoftwareMode::BardsRecommendations,
            catalog: installer_core::catalog::Catalog::load_s_tier().unwrap_or_default(),
            software_picks: BTreeMap::new(),
            software_category_idx: 0,
            dry_run: false,
            continue_on_error: false,
            platform_info: installer_core::PlatformInfo {
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
            system_profile: installer_core::SystemProfile::detect(&installer_core::REAL_SYSTEM).ok(),
            phases: Vec::new(),
            current_phase: 0,
            total_phases: 0,
            start_time: Instant::now(),
            log: VecDeque::with_capacity(500),
            sys_stats: SysStats::default(),
            bbs_msg: "⚡ Initialising the forge...".to_string(),
            arch_timer: None,
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

    pub fn tick(&mut self) {
        if self.screen == Screen::ArchDetected {
            if let Some(start) = self.arch_timer {
                if start.elapsed().as_secs() >= 15 {
                    self.screen = Screen::DistroSelect;
                    self.arch_timer = None;
                }
            }
        }

        // Update long process confirmation countdown
        if self.long_process_state.is_some() {
            let auto_proceed = self.update_long_process_confirmation();
            if auto_proceed {
                // Countdown complete, auto-proceed with the operation
                // The operation will proceed on the next tick
            }
        }
    }

    pub fn handle_auto_arch(&mut self, arch: String) {
        self.screen = Screen::ArchDetected;
        self.bbs_msg = format!("STATION_01: ARCH_SIGIL_{} identified.", arch.to_uppercase());
        self.arch_timer = Some(Instant::now());
    }

    pub fn profile_level(&self) -> ProfileLevel {
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
        let picks = match self.software_mode {
            SoftwareMode::BardsRecommendations => {
                let mut picks = BTreeMap::new();
                for category in &self.catalog.categories {
                    for subcategory in &category.subcategories {
                        if let Some(recommended) =
                            subcategory.programs.iter().find(|p| p.recommended)
                        {
                            picks.insert(category.display_name.clone(), recommended.id.clone());
                        } else if let Some(first) = subcategory.programs.first() {
                            picks.insert(category.display_name.clone(), first.id.clone());
                        }
                    }
                }
                picks
            }
            SoftwareMode::Auto => {
                let mut picks = BTreeMap::new();
                for category in &self.catalog.categories {
                    for subcategory in &category.subcategories {
                        if let Some(first) = subcategory.programs.first() {
                            picks.insert(category.display_name.clone(), first.id.clone());
                        }
                    }
                }
                picks
            }
            SoftwareMode::Manual => self.software_picks.clone(),
        };
        SoftwareTierPlan::new(
            matches!(self.software_mode, SoftwareMode::BardsRecommendations),
            picks,
            self.theme_plan.clone(),
            None, // No preset_id by default from manual selection
        )
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
        // Handle long process confirmation first (highest priority)
        if self.long_process_state.is_some() {
            let proceed = self.handle_long_process_key(code);
            if proceed {
                // User confirmed - the operation will proceed
                return;
            }
            // If user cancelled (Esc), long_process_state is cleared
            if self.long_process_state.is_none() {
                return;
            }
        }

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
            Screen::ArchDetected => {
                if code == KeyCode::Enter || code == KeyCode::Char(' ') {
                    self.screen = Screen::DistroSelect;
                    self.arch_timer = None;
                }
                if code == KeyCode::Char('c') || code == KeyCode::Char('C') || code == KeyCode::Esc
                {
                    // Go to manual select (for now just DistroSelect)
                    self.screen = Screen::DistroSelect;
                    self.arch_timer = None;
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
            Screen::DeSelect => self.handle_list_key(code, 10),
            Screen::ProtocolSelect => self.handle_list_key(code, 3),
            Screen::DeConfirm => self.handle_confirm_key(code),
            Screen::FontPrep => self.handle_font_prep_key(code),
            Screen::Wardrobe => self.handle_wardrobe_key(code),
            Screen::SystemSummary => self.handle_system_summary_key(code),
            Screen::Authorization => self.handle_auth_key(code),
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

    fn handle_auth_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                if let Some(ref mut s) = self.auth_state {
                    s.selected = !s.selected;
                }
            }
            KeyCode::Enter => {
                if let Some(s) = self.auth_state.take() {
                    let _ = s.reply.send(s.selected);
                    self.screen = Screen::Installing;
                }
            }
            KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                if let Some(s) = self.auth_state.take() {
                    let _ = s.reply.send(false);
                    self.screen = Screen::Installing;
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let Some(s) = self.auth_state.take() {
                    let _ = s.reply.send(true);
                    self.screen = Screen::Installing;
                }
            }
            _ => {}
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
            Screen::DeSelect => {
                if idx < 10 {
                    self.menu_cursor = idx;
                    self.select_desktop_environment();
                }
            }
            Screen::ProtocolSelect => {
                if idx < 3 {
                    self.menu_cursor = idx;
                    self.select_display_protocol();
                }
            }
            Screen::SoftwareMode => {
                if idx < 3 {
                    self.menu_cursor = idx;
                    self.advance_from_list();
                }
            }
            Screen::SoftwareSelect => {
                if let Some(category) = self.catalog.categories.get(self.software_category_idx) {
                    let all_programs: Vec<&Program> = category
                        .subcategories
                        .iter()
                        .flat_map(|sc| &sc.programs)
                        .collect();

                    if idx < all_programs.len() {
                        self.menu_cursor = idx;
                        let chosen = all_programs[self.menu_cursor];
                        self.software_picks
                            .insert(category.display_name.clone(), chosen.id.clone());

                        if self.software_category_idx + 1 >= self.catalog.categories.len() {
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
                self.screen = Screen::DeSelect;
                self.menu_cursor = 0;
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
            // Check which confirm screen we're in
            if self.screen == Screen::DeConfirm {
                // Desktop environment confirmation
                match code {
                    KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                        self.advance_from_list();
                    }
                    KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                        self.go_back();
                    }
                    _ => {}
                }
                return;
            } else {
                // Pre-install confirm screen
                match code {
                    KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                        // Trigger detection before showing summary
                        if self.system_profile.is_none() {
                            self.bbs_msg = "SCRYING: ANALYSING MACHINE PEDIGREE...".to_string();
                            if let Ok(profile) =
                                installer_core::SystemProfile::detect(&installer_core::REAL_SYSTEM)
                            {
                                let _ = profile.save_to_config();
                                self.system_profile = Some(profile);
                            }
                        }
                        self.screen = Screen::SystemSummary;
                    }
                    KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                        self.screen = Screen::SoftwareMode;
                        self.menu_cursor = match self.software_mode {
                            SoftwareMode::BardsRecommendations => 0,
                            SoftwareMode::Auto => 1,
                            SoftwareMode::Manual => 2,
                        };
                    }
                    _ => {}
                }
                return;
            }
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

    fn handle_system_summary_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.screen = Screen::FontPrep;
            }
            KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Backspace => {
                self.screen = Screen::DeConfirm;
            }
            _ => {}
        }
    }

    fn handle_font_prep_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                // User wants nerd fonts
                self.push_log("User requested Nerd Font installation.", LogLevel::Info);
                self.screen = Screen::Wardrobe;
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                // User skips nerd fonts
                self.push_log("Nerd Font installation skipped by user.", LogLevel::Warning);
                self.screen = Screen::Wardrobe;
            }
            KeyCode::Backspace => {
                self.screen = Screen::SystemSummary;
            }
            _ => {}
        }
    }

    fn handle_wardrobe_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_preset_idx > 0 {
                    self.selected_preset_idx -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_preset_idx + 1 < self.available_presets.len() {
                    self.selected_preset_idx += 1;
                }
            }
            KeyCode::Enter => {
                self.apply_preset();
                self.start_install();
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.screen = Screen::FontPrep;
            }
            _ => {}
        }
    }

    fn apply_preset(&mut self) {
        if let Some(preset) = self.available_presets.get(self.selected_preset_idx).cloned() {
            self.push_log(format!("Applying preset: {}", preset.name), LogLevel::Info);
            
            // Apply software selections
            for (category, program_id) in &preset.software_plan.selections {
                self.software_picks
                    .insert(category.clone(), program_id.clone());
            }

            // Update Theme Plan based on theme_id (simple mapping for now)
            if preset.theme_id == "neon-night" || preset.theme_id == "retro-bbc" {
                self.theme_plan = ThemePlan::RetroWithWallpapers;
            }

            // Apply tweaks
            for tweak in &preset.tweaks {
                match tweak.as_str() {
                    "enable_p10k" => self.modules.enable_p10k = true,
                    "argon_one" => self.modules.enable_argon = true,
                    "docker_data_root" => self.modules.docker_data_root = true,
                    _ => {}
                }
            }
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
                self.menu_cursor = match self.software_mode {
                    SoftwareMode::BardsRecommendations => 0,
                    SoftwareMode::Auto => 1,
                    SoftwareMode::Manual => 2,
                };
            }
            Screen::SoftwareMode => {
                self.software_mode = match self.menu_cursor {
                    0 => SoftwareMode::BardsRecommendations,
                    1 => SoftwareMode::Auto,
                    _ => SoftwareMode::Manual,
                };

                match self.software_mode {
                    SoftwareMode::BardsRecommendations | SoftwareMode::Auto => {
                        self.software_picks.clear();
                        self.screen = Screen::Confirm;
                        self.menu_cursor = 0;
                    }
                    SoftwareMode::Manual => {
                        self.software_picks.clear();
                        self.software_category_idx = 0;
                        self.menu_cursor = 0;
                        self.screen = Screen::SoftwareSelect;
                    }
                }
            }
            _ => {}
        }
    }

    fn select_desktop_environment(&mut self) {
        use installer_core::desktop::DesktopEnvironment;

        let de = match self.menu_cursor {
            0 => DesktopEnvironment::Gnome,
            1 => DesktopEnvironment::Kde,
            2 => DesktopEnvironment::Xfce,
            3 => DesktopEnvironment::Lxqt,
            4 => DesktopEnvironment::Mate,
            5 => DesktopEnvironment::Cinnamon,
            6 => DesktopEnvironment::Budgie,
            7 => DesktopEnvironment::Enlightenment,
            8 => DesktopEnvironment::Lxde,
            9 => DesktopEnvironment::None,
            _ => DesktopEnvironment::None,
        };

        self.desktop_environment = Some(de);
        self.screen = Screen::ProtocolSelect;
        self.menu_cursor = 0; // Default to Auto
    }

    fn select_display_protocol(&mut self) {
        use installer_core::desktop::DisplayProtocol;

        let protocol = match self.menu_cursor {
            0 => DisplayProtocol::Auto,
            1 => DisplayProtocol::X11,
            2 => DisplayProtocol::Wayland,
            _ => DisplayProtocol::Auto,
        };

        self.display_protocol = protocol;
        self.screen = Screen::DeConfirm;
        self.menu_cursor = 0;
    }

    fn handle_software_key(&mut self, code: KeyCode) {
        let category = match self.catalog.categories.get(self.software_category_idx) {
            Some(category) => category,
            None => {
                self.screen = Screen::Confirm;
                self.menu_cursor = 0;
                return;
            }
        };

        // Flatten all programs in the category across all subcategories
        let all_programs: Vec<&Program> = category
            .subcategories
            .iter()
            .flat_map(|sc| &sc.programs)
            .collect();

        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_cursor > 0 {
                    self.menu_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_cursor + 1 < all_programs.len() {
                    self.menu_cursor += 1;
                }
            }
            KeyCode::Enter => {
                let chosen = all_programs[self.menu_cursor];
                self.software_picks
                    .insert(category.display_name.clone(), chosen.id.clone());

                if self.software_category_idx + 1 >= self.catalog.categories.len() {
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
        let category = self.catalog.categories.get(category_idx)?;
        let picked = self.software_picks.get(&category.display_name)?;

        let all_programs: Vec<&Program> = category
            .subcategories
            .iter()
            .flat_map(|sc| &sc.programs)
            .collect();

        all_programs.iter().position(|p| p.id == *picked)
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
        match self.software_mode {
            SoftwareMode::BardsRecommendations => "Bard's Recommendations (S-tier)".to_string(),
            SoftwareMode::Auto => "Automatic (Baseline)".to_string(),
            SoftwareMode::Manual => format!(
                "Manual ({}/{})",
                self.software_picks.len(),
                self.catalog.categories.len()
            ),
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
            TuiMessage::AuthRequest { auth_type, reply } => {
                self.auth_state = Some(AuthState {
                    auth_type,
                    reply,
                    selected: true,
                });
                self.screen = Screen::Authorization;
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

    // If exactly one driver matches the detected platform, auto-select it and skip
    // the 15-second ArchDetected banner. On error or ambiguous match, fall through
    // to handle_auto_arch() so the user can confirm manually.
    match detect_platform().ok().and_then(|info| {
        let matches: Vec<usize> = app
            .drivers
            .iter()
            .enumerate()
            .filter_map(|(i, d)| if d.matches(&info) { Some(i) } else { None })
            .collect();
        if matches.len() == 1 {
            Some((matches[0], info.arch))
        } else {
            None
        }
    }) {
        Some((idx, arch)) => {
            app.selected_driver_idx = idx;
            app.screen = Screen::DistroSelect;
            app.bbs_msg = format!(
                "STATION_01: ARCH_SIGIL_{} — {} auto-selected.",
                arch.to_uppercase(),
                app.drivers[idx].name()
            );
        }
        None => {
            let arch = std::env::consts::ARCH.to_string();
            app.handle_auto_arch(arch);
        }
    }

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        app.tick();
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

// ── Long Process Confirmation Methods ──────────────────────────────────────

impl TuiApp {
    /// Start a long process confirmation
    #[allow(dead_code)]
    pub fn start_long_process_confirmation(
        &mut self,
        operation_name: String,
        estimated_duration: std::time::Duration,
    ) {
        if crate::tui::confirmation::should_show_long_confirmation(estimated_duration) {
            self.long_process_state = Some(crate::tui::confirmation::LongProcessState::new(
                operation_name,
                estimated_duration,
            ));
        }
    }

    /// Cancel long process confirmation
    #[allow(dead_code)]
    pub fn cancel_long_process_confirmation(&mut self) {
        self.long_process_state = None;
    }

    /// Navigate to a new screen with history tracking
    #[allow(dead_code)]
    pub fn navigate_to(&mut self, new_screen: Screen, context: &str) {
        // Push current screen to history before navigating away
        if self.screen != new_screen {
            self.navigation_history.push(self.screen);
        }
        self.screen = new_screen;
        self.navigation_context = context.to_string();
    }

    /// Navigate back using history
    pub fn navigate_back(&mut self) {
        if let Some(previous_screen) = self.navigation_history.pop() {
            self.screen = previous_screen;
            self.navigation_context = match previous_screen {
                Screen::Welcome => "Welcome to MASH Installer",
                Screen::ArchDetected => "Architecture Detection",
                Screen::DistroSelect => "Distribution Selection",
                Screen::ProfileSelect => "Profile Selection",
                Screen::ModuleSelect => "Module Selection",
                Screen::ThemeSelect => "Theme Selection",
                Screen::SoftwareMode => "Software Mode Selection",
                Screen::SoftwareSelect => "Software Selection",
                Screen::Confirm => "Installation Confirmation",
                Screen::DeSelect => "Desktop Environment Selection",
                Screen::ProtocolSelect => "Display Protocol Selection",
                Screen::DeConfirm => "Desktop Environment Confirmation",
                Screen::FontPrep => "Font Preparation",
                Screen::Wardrobe => "The Wardrobe (Presets)",
                Screen::SystemSummary => "System Pedigree Summary",
                Screen::Password => "Password Prompt",
                Screen::Authorization => "Interactive Authorization",
                Screen::Installing => "Installation in Progress",
                Screen::Done => "Installation Complete",
                Screen::Error => "Error Encountered",
            }
            .to_string();
        }
    }

    /// Get current navigation context
    pub fn get_navigation_context(&self) -> &str {
        &self.navigation_context
    }

    /// Handle key input for long process confirmation
    pub fn handle_long_process_key(&mut self, code: crossterm::event::KeyCode) -> bool {
        let Some(state) = &mut self.long_process_state else {
            return false;
        };

        match code {
            crossterm::event::KeyCode::Enter => {
                // User confirmed - proceed immediately
                state.user_confirmed = true;
                true
            }
            crossterm::event::KeyCode::Esc => {
                // User cancelled
                self.long_process_state = None;
                false
            }
            _ => false,
        }
    }

    /// Enhanced go_back with navigation history support
    fn go_back(&mut self) {
        // Try to use navigation history first
        if !self.navigation_history.is_empty() {
            self.navigate_back();
            return;
        }

        // Fallback to original hardcoded navigation with cursor positioning
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
            Screen::DeSelect => {
                self.screen = Screen::ModuleSelect;
                self.menu_cursor = 0;
            }
            Screen::ProtocolSelect => {
                self.screen = Screen::DeSelect;
                self.menu_cursor = self
                    .desktop_environment
                    .map(|de| match de {
                        installer_core::desktop::DesktopEnvironment::Gnome => 0,
                        installer_core::desktop::DesktopEnvironment::Kde => 1,
                        installer_core::desktop::DesktopEnvironment::Xfce => 2,
                        installer_core::desktop::DesktopEnvironment::Lxqt => 3,
                        installer_core::desktop::DesktopEnvironment::Mate => 4,
                        installer_core::desktop::DesktopEnvironment::Cinnamon => 5,
                        installer_core::desktop::DesktopEnvironment::Budgie => 6,
                        installer_core::desktop::DesktopEnvironment::Enlightenment => 7,
                        installer_core::desktop::DesktopEnvironment::Lxde => 8,
                        installer_core::desktop::DesktopEnvironment::None => 9,
                    })
                    .unwrap_or(0);
            }
            Screen::DeConfirm => {
                // When confirmed, advance to ThemeSelect
                self.screen = Screen::ThemeSelect;
                self.menu_cursor = self.theme_menu_index();
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
                    self.menu_cursor = match self.software_mode {
                        SoftwareMode::BardsRecommendations => 0,
                        SoftwareMode::Auto => 1,
                        SoftwareMode::Manual => 2,
                    };
                } else {
                    self.software_category_idx = self.software_category_idx.saturating_sub(1);
                    self.menu_cursor = self
                        .selected_option_index(self.software_category_idx)
                        .unwrap_or(0);
                }
            }
            Screen::Confirm => match self.software_mode {
                SoftwareMode::BardsRecommendations | SoftwareMode::Auto => {
                    self.screen = Screen::SoftwareMode;
                    self.menu_cursor = match self.software_mode {
                        SoftwareMode::BardsRecommendations => 0,
                        SoftwareMode::Auto => 1,
                        _ => 0,
                    };
                }
                SoftwareMode::Manual => {
                    self.screen = Screen::SoftwareSelect;
                    self.menu_cursor = self
                        .selected_option_index(self.software_category_idx)
                        .unwrap_or(0);
                }
            },
            Screen::FontPrep => {
                self.screen = Screen::SystemSummary;
                self.menu_cursor = 0;
            }
            Screen::Wardrobe => {
                self.screen = Screen::FontPrep;
                self.menu_cursor = 0;
            }
            Screen::SystemSummary => {
                self.screen = Screen::Confirm;
                self.menu_cursor = 0;
            }
            Screen::Password | Screen::Authorization => {
                // No-op for mid-install screens
            }
            Screen::Installing
            | Screen::Done
            | Screen::Error
            | Screen::Welcome
            | Screen::ArchDetected => {
                // No-op for terminal screens
            }
        }

        // Update navigation context for the new screen
        self.navigation_context = match self.screen {
            Screen::Welcome => "Welcome to MASH Installer",
            Screen::ArchDetected => "Architecture Detection",
            Screen::DistroSelect => "Distribution Selection",
            Screen::ProfileSelect => "Profile Selection",
            Screen::ModuleSelect => "Module Selection",
            Screen::ThemeSelect => "Theme Selection",
            Screen::SoftwareMode => "Software Mode Selection",
            Screen::SoftwareSelect => "Software Selection",
            Screen::Confirm => "Installation Confirmation",
            Screen::DeSelect => "Desktop Environment Selection",
            Screen::ProtocolSelect => "Display Protocol Selection",
            Screen::DeConfirm => "Desktop Environment Confirmation",
            Screen::FontPrep => "Font Preparation",
            Screen::Wardrobe => "The Wardrobe (Presets)",
            Screen::SystemSummary => "System Pedigree Summary",
            Screen::Password => "Password Prompt",
            Screen::Authorization => "Interactive Authorization",
            Screen::Installing => "Installation in Progress",
            Screen::Done => "Installation Complete",
            Screen::Error => "Error Encountered",
        }
        .to_string();
    }

    /// Update long process confirmation state (call this in tick())
    pub fn update_long_process_confirmation(&mut self) -> bool {
        let Some(state) = &mut self.long_process_state else {
            return false;
        };

        if state.update_countdown() {
            // Countdown complete - auto-proceed
            state.user_confirmed = true;
            return true;
        }

        false
    }

    /// Check if long process is confirmed
    #[allow(dead_code)]
    pub fn is_long_process_confirmed(&self) -> bool {
        self.long_process_state
            .as_ref()
            .map(|s| s.user_confirmed)
            .unwrap_or(false)
    }
}
