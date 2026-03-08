use crate::tui::state::{Screen, TuiApp};
use crossterm::event::{KeyCode, KeyModifiers};

impl TuiApp {
    pub fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        // Handle long process confirmation first (highest priority)
        if self.long_process_state.is_some() {
            let proceed = self.handle_long_process_key(code);
            if proceed {
                return;
            }
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
                if let Some(val) = c.to_digit(10) {
                    self.handle_numeric_input(val as usize);
                }
            }
        }

        let screen = self.screen;
        match screen {
            Screen::Welcome => {
                if code == KeyCode::Enter || code == KeyCode::Char(' ') {
                    self.advance_from_list();
                }
            }
            Screen::SystemScan => {
                if code == KeyCode::Esc {
                    self.go_back();
                }
            }
            Screen::DistroSelect => {
                let len = self.drivers.len();
                self.handle_list_key(code, len);
            }
            Screen::ProfileSelect => self.handle_list_key(code, 3),
            Screen::ThemeSelect => self.handle_list_key(code, 6), // Extended with new themes
            Screen::Password => self.handle_password_key(code),

            Screen::SoftwareMode => self.handle_list_key(code, 3), // Bards, Auto, Manual
            Screen::SoftwareCategorySelect => {
                self.handle_list_key(code, self.catalog.categories.len() + 1)
            }
            Screen::SoftwareSelect => self.handle_software_key(code),
            Screen::Confirm => self.handle_confirm_key(code),
            Screen::DeSelect => self.handle_list_key(code, 12), // Added Cosmic/Hyprland
            Screen::ProtocolSelect => self.handle_list_key(code, 3),
            Screen::DeConfirm => self.handle_confirm_key(code),
            Screen::FontPrep => self.handle_font_prep_key(code),
            Screen::Wardrobe => self.handle_wardrobe_key(code),
            Screen::ArgonConfig => self.handle_argon_key(code),
            Screen::DockerConfig => self.handle_docker_key(code),
            Screen::ChezmoiConfig => self.handle_chezmoi_config_key(code),
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
            Screen::SoftwareMode => {
                if idx < 3 {
                    self.menu_cursor = idx;
                    self.advance_from_list();
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
                } else {
                    self.menu_cursor = list_len.saturating_sub(1);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_cursor < list_len.saturating_sub(1) {
                    self.menu_cursor += 1;
                } else {
                    self.menu_cursor = 0;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.advance_from_list();
            }
            KeyCode::Esc => {
                self.go_back();
            }
            _ => {}
        }
    }

    fn handle_argon_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_cursor > 0 {
                    self.menu_cursor -= 1;
                } else {
                    self.menu_cursor = 3;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_cursor < 3 {
                    self.menu_cursor += 1;
                } else {
                    self.menu_cursor = 0;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => match self.menu_cursor {
                0 => {
                    self.argon.enabled = !self.argon.enabled;
                    self.argon.cooling_profile = "Quiet".to_string();
                }
                1 => {
                    self.argon.enabled = !self.argon.enabled;
                    self.argon.cooling_profile = "Balanced".to_string();
                }
                2 => {
                    self.argon.enabled = !self.argon.enabled;
                    self.argon.cooling_profile = "Performance".to_string();
                }
                3 => self.advance_from_list(),
                _ => {}
            },
            KeyCode::Esc => self.go_back(),
            _ => {}
        }
    }

    fn handle_docker_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Down | KeyCode::Char('j') => {
                self.menu_cursor = if self.menu_cursor == 0 { 1 } else { 0 };
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if self.menu_cursor == 0 {
                    self.docker.enabled = !self.docker.enabled;
                } else {
                    self.advance_from_list();
                }
            }
            KeyCode::Esc => self.go_back(),
            _ => {}
        }
    }

    fn handle_chezmoi_config_key(&mut self, code: KeyCode) {
        let max_cursor = if self.chezmoi_enabled { 3 } else { 1 };

        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_cursor > 0 {
                    self.menu_cursor -= 1;
                } else {
                    self.menu_cursor = max_cursor;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_cursor < max_cursor {
                    self.menu_cursor += 1;
                } else {
                    self.menu_cursor = 0;
                }
            }
            KeyCode::Enter => match self.menu_cursor {
                0 => {
                    self.chezmoi_enabled = !self.chezmoi_enabled;
                }
                1 => {
                    if !self.chezmoi_enabled {
                        self.advance_from_list();
                    }
                }
                3 => {
                    self.advance_from_list();
                }
                _ => {}
            },
            KeyCode::Backspace => match self.menu_cursor {
                1 if self.chezmoi_enabled => {
                    self.chezmoi_repo.pop();
                }
                2 if self.chezmoi_enabled => {
                    self.chezmoi_branch.pop();
                }
                _ => {}
            },
            KeyCode::Char(c) => match self.menu_cursor {
                1 if self.chezmoi_enabled => {
                    self.chezmoi_repo.push(c);
                }
                2 if self.chezmoi_enabled => {
                    self.chezmoi_branch.push(c);
                }
                _ => {}
            },
            KeyCode::Esc => self.go_back(),
            _ => {}
        }
    }

    fn handle_confirm_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Left
            | KeyCode::Right
            | KeyCode::Tab
            | KeyCode::Char('h')
            | KeyCode::Char('l') => {
                self.menu_cursor = if self.menu_cursor == 0 { 1 } else { 0 };
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if self.menu_cursor == 0 {
                    // YES
                    if self.screen == Screen::Confirm {
                        self.start_install();
                    } else {
                        self.advance_from_list();
                    }
                } else {
                    // NO
                    self.go_back();
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if self.screen == Screen::Confirm {
                    self.start_install();
                } else {
                    self.advance_from_list();
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.go_back();
            }
            _ => {}
        }
    }

    fn handle_system_summary_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.advance_from_list();
            }
            KeyCode::Esc => self.go_back(),
            _ => {}
        }
    }

    fn handle_font_prep_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.advance_from_list();
            }
            KeyCode::Esc => self.go_back(),
            _ => {}
        }
    }

    fn handle_wardrobe_key(&mut self, code: KeyCode) {
        let presets_len = self.available_presets.len();
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_cursor > 0 {
                    self.menu_cursor -= 1;
                } else {
                    self.menu_cursor = presets_len; // presets + back
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_cursor < presets_len {
                    self.menu_cursor += 1;
                } else {
                    self.menu_cursor = 0;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if self.menu_cursor < presets_len {
                    // Select preset
                    self.selected_preset_idx = self.menu_cursor;
                    let preset = self.available_presets[self.selected_preset_idx].clone();
                    self.apply_preset(&preset);
                    self.screen = Screen::FontPrep;
                    self.menu_cursor = 0;
                } else {
                    // Back
                    self.go_back();
                }
            }
            KeyCode::Esc => self.go_back(),
            _ => {}
        }
    }

    fn handle_installing_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up => {
                self.log_scroll = self.log_scroll.saturating_sub(1);
            }
            KeyCode::Down => {
                self.log_scroll += 1;
            }
            _ => {}
        }
    }

    pub fn handle_long_process_key(&mut self, code: crossterm::event::KeyCode) -> bool {
        let Some(state) = &mut self.long_process_state else {
            return false;
        };

        match code {
            crossterm::event::KeyCode::Enter => {
                state.user_confirmed = true;
                true
            }
            crossterm::event::KeyCode::Esc => {
                self.long_process_state = None;
                false
            }
            _ => false,
        }
    }
}
