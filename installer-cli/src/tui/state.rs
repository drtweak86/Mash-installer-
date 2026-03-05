use std::collections::{BTreeMap, VecDeque};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

pub use installer_core::catalog::Catalog;
pub use installer_core::desktop::{DesktopEnvironment, DisplayProtocol};
pub use installer_core::platform::PlatformInfo;
pub use installer_core::preset::Preset;
pub use installer_core::SystemProfile;
pub use installer_core::{
    AuthType, DistroDriver, InstallationReport, PhaseEvent, SoftwareCategory, ThemePlan,
};

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
        auth_type: AuthType,
        reply: Sender<bool>,
    },

    ScanComplete {
        platform: PlatformInfo,
        profile: SystemProfile,
    },

    Done(Box<InstallationReport>),
    InstallError(String),
}

// ── Screen state machine ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Welcome,
    SystemScan,
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

// ── Sys stats snapshot ───────────────────────────────────────────────────────

#[derive(Default, Clone)]
pub struct SysStats {
    pub cpu_pct: f32,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub net_rx_kbps: f32,
}

// ── Confirm prompt state ─────────────────────────────────────────────────────

#[allow(dead_code)]
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
    pub auth_type: AuthType,
    pub reply: Sender<bool>,
    pub selected: bool,
}

// ── Long process confirmation state ─────────────────────────────────────────

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LongProcessState {
    pub operation_name: String,
    pub estimated_duration: Duration,
    pub start_time: Instant,
    pub countdown: Option<u64>,
    pub user_confirmed: bool,
}

#[allow(dead_code)]
impl LongProcessState {
    pub fn new(operation_name: String, estimated_duration: Duration) -> Self {
        Self {
            operation_name,
            estimated_duration,
            start_time: Instant::now(),
            countdown: if estimated_duration.as_secs() > 120 {
                Some(30)
            } else {
                None
            },
            user_confirmed: false,
        }
    }

    pub fn should_show_confirmation(&self) -> bool {
        !self.user_confirmed && self.start_time.elapsed() < Duration::from_secs(30)
    }

    pub fn update_countdown(&mut self) -> bool {
        let elapsed = self.start_time.elapsed();
        if elapsed >= Duration::from_secs(30) {
            self.user_confirmed = true;
            return true;
        }
        self.countdown = Some(30 - elapsed.as_secs());
        false
    }
}

// ── Software Selection Mode ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SoftwareMode {
    #[default]
    BardsRecommendations,
    Auto,
    Manual,
}

// ── Module selection ─────────────────────────────────────────────────────────

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
    // Navigation state
    pub navigation_history: Vec<Screen>,
    pub navigation_context: String,
    // Menu navigation
    pub menu_cursor: usize,
    // Available drivers
    pub drivers: Vec<&'static dyn DistroDriver>,
    pub selected_driver_idx: usize,
    // Module selection
    pub modules: ModuleState,
    // Profile selection
    pub profile_idx: usize,
    // Desktop environment selection
    pub desktop_environment: Option<DesktopEnvironment>,
    pub display_protocol: DisplayProtocol,
    // Theme selection
    pub theme_plan: ThemePlan,
    // Software tiers
    pub software_mode: SoftwareMode,
    pub catalog: Catalog,
    pub software_picks: BTreeMap<SoftwareCategory, Vec<String>>,
    pub software_category_idx: usize,
    // Dry-run flag
    pub dry_run: bool,
    pub continue_on_error: bool,
    pub platform_info: PlatformInfo,
    pub system_profile: Option<SystemProfile>,
    // Installing phase state
    pub phases: Vec<PhaseRow>,
    pub current_phase: usize,
    pub total_phases: usize,
    pub start_time: Instant,
    pub progress_pct: f32,
    // Log
    pub log: VecDeque<LogEntry>,
    // Stats
    pub sys_stats: SysStats,
    // BBS message
    pub bbs_msg: String,
    // Presets
    pub available_presets: Vec<Preset>,
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
    // Scry state
    pub scry: bool,
    pub scry_port: u16,
    pub environment: installer_core::model::options::EnvironmentTag,
    // Should quit
    pub should_quit: bool,
}
