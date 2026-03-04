use crate::tui::app::SoftwareMode;
use crate::tui::state::{Screen, TuiApp};
use installer_core::desktop::DesktopEnvironment;
use installer_core::ThemePlan;

impl TuiApp {
    pub fn advance_from_list(&mut self) {
        let screen = self.screen;
        match screen {
            Screen::DistroSelect => {
                self.selected_driver_idx = self.menu_cursor;
                self.navigate_to(Screen::ProfileSelect, "Profile Selection");
                self.menu_cursor = 1; // Default to Dev
            }
            Screen::ProfileSelect => {
                self.profile_idx = self.menu_cursor;
                self.navigate_to(Screen::ModuleSelect, "Module Selection");
                self.menu_cursor = 0;
            }
            Screen::ThemeSelect => {
                self.theme_plan = match self.menu_cursor {
                    0 => ThemePlan::RetroOnly,
                    1 => ThemePlan::RetroWithWallpapers,
                    2 => ThemePlan::None,
                    _ => ThemePlan::None,
                };
                self.navigate_to(Screen::SoftwareMode, "Software Mode Selection");
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
                    self.navigate_to(Screen::SoftwareSelect, "Software Selection");
                    self.software_category_idx = 0;
                    self.menu_cursor = 0;
                } else {
                    self.navigate_to(Screen::Confirm, "Installation Confirmation");
                    self.menu_cursor = 0;
                }
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
                    9 => DesktopEnvironment::None,
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
            _ => {}
        }
    }

    pub fn navigate_to(&mut self, new_screen: Screen, context: &str) {
        if self.screen != new_screen {
            self.navigation_history.push(self.screen);
        }
        self.screen = new_screen;
        self.navigation_context = context.to_string();
    }

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

    pub fn get_navigation_context(&self) -> &str {
        &self.navigation_context
    }

    pub fn go_back(&mut self) {
        if !self.navigation_history.is_empty() {
            self.navigate_back();
            return;
        }

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
                self.menu_cursor = 0;
            }
            Screen::SoftwareSelect => {
                if self.software_category_idx == 0 {
                    self.screen = Screen::SoftwareMode;
                    self.menu_cursor = 2;
                } else {
                    self.software_category_idx = self.software_category_idx.saturating_sub(1);
                }
            }
            Screen::Confirm => {
                self.screen = Screen::SoftwareMode;
                self.menu_cursor = 0;
            }
            _ => {}
        }

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
