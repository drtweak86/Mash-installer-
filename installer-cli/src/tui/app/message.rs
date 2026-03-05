use crate::tui::state::{
    AuthState, ConfirmState, LogLevel, PasswordState, PhaseRow, PhaseStatus, Screen, TuiApp,
    TuiMessage,
};
use installer_core::PhaseEvent;

impl TuiApp {
    pub fn handle_message(&mut self, msg: TuiMessage) {
        match msg {
            TuiMessage::Phase(event) => self.handle_phase_event(event),
            TuiMessage::SysStats {
                cpu_pct,
                ram_used_mb,
                ram_total_mb,
                net_rx_kbps,
            } => {
                self.sys_stats.cpu_pct = cpu_pct;
                self.sys_stats.ram_used_mb = ram_used_mb;
                self.sys_stats.ram_total_mb = ram_total_mb;
                self.sys_stats.net_rx_kbps = net_rx_kbps;
            }
            TuiMessage::BbsMessage(text) => {
                self.bbs_msg = text;
            }
            TuiMessage::ConfirmPrompt { prompt, reply } => {
                self.confirm_state = Some(ConfirmState {
                    prompt,
                    reply,
                    selected: true,
                });
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
            TuiMessage::ScanComplete { platform, profile } => {
                self.platform_info = platform;
                self.system_profile = Some(profile);

                // Auto-select driver based on scan results
                let matches: Vec<usize> = self
                    .drivers
                    .iter()
                    .enumerate()
                    .filter_map(|(i, d)| {
                        if d.matches(&self.platform_info) {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();

                if matches.len() == 1 {
                    self.selected_driver_idx = matches[0];
                    self.navigate_to(Screen::SystemSummary, "System Results & Wisdom");
                } else {
                    // Fallback to manual selection if multiple or zero matches
                    self.navigate_to(Screen::DistroSelect, "Distribution Selection");
                }
            }
            TuiMessage::Done(report) => {
                self.report = Some(report);
                self.screen = Screen::Done;
                self.push_log(
                    "STATION_01: INSTALLATION_SEQUENCE_COMPLETE.",
                    LogLevel::Success,
                );
            }
            TuiMessage::InstallError(err) => {
                self.error_msg = Some(err);
                self.screen = Screen::Error;
                self.push_log("STATION_01: CRITICAL_ERROR_ENCOUNTERED.", LogLevel::Error);
            }
        }
    }

    fn handle_phase_event(&mut self, event: PhaseEvent) {
        match event {
            PhaseEvent::Total { total } => {
                self.total_phases = total;
                self.current_phase = 0;
                self.phases.clear();
            }
            PhaseEvent::Started {
                index,
                phase,
                total: _,
            } => {
                self.current_phase = index;
                self.phases.push(PhaseRow {
                    name: phase.clone(),
                    status: PhaseStatus::Running,
                    description: String::new(),
                });
                self.push_log(format!("STARTING: {}", phase), LogLevel::Info);
            }
            PhaseEvent::Completed {
                index: _,
                phase,
                description,
            } => {
                if let Some(row) = self.phases.iter_mut().find(|r| r.name == phase) {
                    row.status = PhaseStatus::Done;
                    row.description = description;
                }
                self.push_log(format!("COMPLETED: {}", phase), LogLevel::Success);
                self.progress_pct = (self.current_phase as f32 / self.total_phases as f32) * 100.0;
            }
            PhaseEvent::Failed {
                index: _,
                phase,
                error,
            } => {
                if let Some(row) = self.phases.iter_mut().find(|r| r.name == phase) {
                    row.status = PhaseStatus::Failed;
                }
                self.push_log(format!("FAILED: {}: {}", phase, error), LogLevel::Error);
            }
            PhaseEvent::Skipped { index: _, phase } => {
                if let Some(row) = self.phases.iter_mut().find(|r| r.name == phase) {
                    row.status = PhaseStatus::Skipped;
                }
                self.push_log(format!("SKIPPED: {}", phase), LogLevel::Info);
            }
            PhaseEvent::Warning { message } => {
                self.push_log(format!("WARNING: {}", message), LogLevel::Warning);
            }
        }
    }
}
