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
            Screen::ModuleSelect => self.handle_module_key(code),
            Screen::ThemeSelect => self.handle_list_key(code, 3),
            Screen::Password => self.handle_password_key(code),

            Screen::SoftwareMode => self.handle_list_key(code, 3), // Bards, Auto, Manual
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

    fn handle_module_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_cursor > 0 {
                    self.menu_cursor -= 1;
                } else {
                    self.menu_cursor = 3; // 3 options + Confirm
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
                0 => self.modules.enable_argon = !self.modules.enable_argon,
                1 => self.modules.enable_p10k = !self.modules.enable_p10k,
                2 => self.modules.docker_data_root = !self.modules.docker_data_root,
                3 => {
                    self.screen = Screen::DeSelect;
                    self.menu_cursor = 0;
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
                    } else if self.screen == Screen::DeConfirm {
                        self.screen = Screen::ThemeSelect;
                        self.menu_cursor = 0;
                    }
                } else {
                    // NO
                    self.go_back();
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if self.screen == Screen::Confirm {
                    self.start_install();
                } else if self.screen == Screen::DeConfirm {
                    self.screen = Screen::ThemeSelect;
                    self.menu_cursor = 0;
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
