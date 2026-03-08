use crate::tui::app::SoftwareMode;
use crate::tui::state::{Screen, TuiApp};
use installer_core::desktop::DesktopEnvironment;

impl TuiApp {
    pub fn advance_from_list(&mut self) {
        let screen = self.screen;
        match screen {
            Screen::Welcome => {
                self.navigate_to(Screen::SystemScan, "Active Scrying...");
            }
            Screen::DistroSelect => {
                self.selected_driver_idx = self.menu_cursor;
                self.navigate_to(Screen::ProfileSelect, "Profile Selection");
                self.menu_cursor = 1; // Default to Dev
            }
            Screen::ProfileSelect => {
                self.profile_idx = self.menu_cursor;
                self.navigate_to(Screen::SystemSummary, "System Results & Wisdom");
                self.menu_cursor = 0;
            }
            Screen::SystemSummary => {
                self.navigate_to(Screen::DeSelect, "Desktop Environment Selection");
                self.menu_cursor = 0;
            }
            Screen::DeSelect => {
                self.desktop_environment = Some(match self.menu_cursor {
                    0 => DesktopEnvironment::Gnome,
                    1 => DesktopEnvironment::Kde,
                    2 => DesktopEnvironment::Xfce,
                    3 => DesktopEnvironment::Lxqt,
                    4 => DesktopEnvironment::Mate,
                    5 => DesktopEnvironment::Cinnamon,
                    6 => DesktopEnvironment::Budgie,
                    7 => DesktopEnvironment::Enlightenment,
                    8 => DesktopEnvironment::Lxde,
                    9 => DesktopEnvironment::Cosmic,
                    10 => DesktopEnvironment::Hyprland,
                    11 => DesktopEnvironment::None,
                    _ => DesktopEnvironment::None,
                });
                self.navigate_to(Screen::ProtocolSelect, "Display Protocol Selection");
                self.menu_cursor = 0;
            }
            Screen::ProtocolSelect => {
                self.display_protocol = match self.menu_cursor {
                    0 => installer_core::desktop::DisplayProtocol::Auto,
                    1 => installer_core::desktop::DisplayProtocol::Wayland,
                    2 => installer_core::desktop::DisplayProtocol::X11,
                    _ => installer_core::desktop::DisplayProtocol::Auto,
                };
                self.navigate_to(Screen::DeConfirm, "Desktop Environment Confirmation");
                self.menu_cursor = 0;
            }
            Screen::DeConfirm => {
                if self.menu_cursor == 0 {
                    // YES
                    self.navigate_to(Screen::ThemeSelect, "Aesthetic Calibration");
                    self.menu_cursor = 0;
                } else {
                    // NO
                    self.go_back();
                }
            }
            Screen::ThemeSelect => {
                self.theme_plan = match self.menu_cursor {
                    0 => installer_core::ThemePlan::RetroOnly,
                    1 => installer_core::ThemePlan::RetroWithWallpapers,
                    2 => installer_core::ThemePlan::None,
                    _ => installer_core::ThemePlan::None,
                };

                // If Pi 4B detected, show Argon Config
                if self.platform_info.pi_model.is_some() {
                    self.navigate_to(Screen::ArgonConfig, "Argon One Configuration");
                } else {
                    self.navigate_to(Screen::FontPrep, "Font Curation");
                }
                self.menu_cursor = 0;
            }
            Screen::ArgonConfig => {
                self.navigate_to(Screen::FontPrep, "Font Curation");
                self.menu_cursor = 0;
            }
            Screen::FontPrep => {
                self.navigate_to(Screen::SoftwareMode, "Software Selection Mode");
                self.menu_cursor = 0;
            }
            Screen::SoftwareMode => {
                self.software_mode = match self.menu_cursor {
                    0 => SoftwareMode::BardsRecommendations,
                    1 => SoftwareMode::Auto,
                    2 => SoftwareMode::Manual,
                    _ => SoftwareMode::BardsRecommendations,
                };
                if self.software_mode == SoftwareMode::Manual {
                    self.navigate_to(Screen::SoftwareCategorySelect, "Software Categories");
                    self.menu_cursor = 0;
                } else {
                    self.navigate_to(Screen::ChezmoiConfig, "Dotfile Restoration");
                    self.menu_cursor = 0;
                }
            }
            Screen::SoftwareCategorySelect => {
                // If 'Done' (last option) is selected, move to Docker or Chezmoi
                let categories_len = self.catalog.categories.len();
                if self.menu_cursor >= categories_len {
                    // Check if Docker was selected
                    let docker_selected = self
                        .software_picks
                        .values()
                        .any(|picks| picks.iter().any(|p| p.to_lowercase().contains("docker")));

                    if docker_selected {
                        self.navigate_to(Screen::DockerConfig, "Docker Configuration");
                    } else {
                        self.navigate_to(Screen::ChezmoiConfig, "Dotfile Restoration");
                    }
                    self.menu_cursor = 0;
                } else {
                    // Navigate to selection for this category
                    self.software_category_idx = self.menu_cursor;
                    self.navigate_to(Screen::SoftwareSelect, "Component Selection");
                    self.menu_cursor = 0;
                }
            }
            Screen::SoftwareSelect => {
                // Back to category select
                self.navigate_to(Screen::SoftwareCategorySelect, "Software Categories");
                // Restore cursor to the category we just came from
                self.menu_cursor = self.software_category_idx;
            }
            Screen::DockerConfig => {
                self.navigate_to(Screen::ChezmoiConfig, "Dotfile Restoration");
                self.menu_cursor = 0;
            }
            Screen::ChezmoiConfig => {
                self.navigate_to(Screen::Confirm, "Final Provisioning Summary");
                self.menu_cursor = 0;
            }
            _ => {}
        }
    }

    pub fn navigate_to(&mut self, new_screen: Screen, context: &str) {
        if self.screen != new_screen {
            self.navigation_history.push(self.screen);
        }
        self.screen = new_screen;
        self.navigation_context = context.to_string();

        if new_screen == Screen::SystemScan {
            self.spawn_system_scan();
        }
    }

    pub fn navigate_back(&mut self) {
        if let Some(previous_screen) = self.navigation_history.pop() {
            self.screen = previous_screen;
            self.navigation_context = self.context_for_screen(previous_screen).to_string();
        }
    }

    fn context_for_screen(&self, screen: Screen) -> &'static str {
        match screen {
            Screen::Welcome => "Welcome to MASH Installer",
            Screen::SystemScan => "Active Scrying...",
            Screen::DistroSelect => "Distribution Selection",
            Screen::ProfileSelect => "Profile Selection",
            Screen::ThemeSelect => "Theme Selection",
            Screen::SoftwareMode => "Software Selection Mode",
            Screen::SoftwareCategorySelect => "Software Categories",
            Screen::SoftwareSelect => "Software Selection",
            Screen::Confirm => "Final Provisioning Summary",
            Screen::DeSelect => "Desktop Environment Selection",
            Screen::ProtocolSelect => "Display Protocol Selection",
            Screen::DeConfirm => "Desktop Environment Confirmation",
            Screen::FontPrep => "Font Curation",
            Screen::Wardrobe => "The Wardrobe (Presets)",
            Screen::ArgonConfig => "Argon One Configuration",
            Screen::DockerConfig => "Docker Configuration",
            Screen::ChezmoiConfig => "Dotfile Restoration",
            Screen::SystemSummary => "System Results & Wisdom",
            Screen::Password => "Password Prompt",
            Screen::Authorization => "Interactive Authorization",
            Screen::Installing => "Installation Forge",
            Screen::Done => "Installation Complete",
            Screen::Error => "Error Encountered",
        }
    }

    pub fn get_navigation_context(&self) -> &str {
        &self.navigation_context
    }

    pub fn go_back(&mut self) {
        if !self.navigation_history.is_empty() {
            self.navigate_back();
            return;
        }

        match self.screen {
            Screen::SystemScan => self.screen = Screen::Welcome,
            Screen::SystemSummary => self.screen = Screen::ProfileSelect,
            Screen::ProfileSelect => self.screen = Screen::DistroSelect,
            Screen::DistroSelect => self.screen = Screen::SystemScan,
            Screen::DeSelect => self.screen = Screen::SystemSummary,
            Screen::DeConfirm => self.screen = Screen::ProtocolSelect,
            Screen::ThemeSelect => self.screen = Screen::DeConfirm,
            Screen::ArgonConfig => self.screen = Screen::ThemeSelect,
            Screen::FontPrep => {
                if self.platform_info.pi_model.is_some() {
                    self.screen = Screen::ArgonConfig;
                } else {
                    self.screen = Screen::ThemeSelect;
                }
            }
            Screen::SoftwareMode => self.screen = Screen::FontPrep,
            Screen::SoftwareCategorySelect => self.screen = Screen::SoftwareMode,
            Screen::SoftwareSelect => self.screen = Screen::SoftwareCategorySelect,
            Screen::DockerConfig => self.screen = Screen::SoftwareCategorySelect,
            Screen::ChezmoiConfig => {
                if self.software_mode == SoftwareMode::Manual {
                    self.screen = Screen::SoftwareCategorySelect;
                } else {
                    self.screen = Screen::SoftwareMode;
                }
            }
            Screen::Confirm => {
                self.screen = Screen::ChezmoiConfig;
                self.menu_cursor = 0;
            }
            _ => {}
        }

        self.navigation_context = self.context_for_screen(self.screen).to_string();
    }

    pub fn update_long_process_confirmation(&mut self) -> bool {
        let Some(state) = &mut self.long_process_state else {
            return false;
        };

        if state.update_countdown() {
            state.user_confirmed = true;
            return true;
        }

        false
    }
}
