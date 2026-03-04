use crate::tui::app::SoftwareMode;
use crate::tui::state::{LogLevel, Screen, TuiApp};
use crossterm::event::KeyCode;
use installer_core::catalog::Program;
use installer_core::preset::Preset;
use installer_core::{InstallOptions, SoftwareTierPlan, ThemePlan};
use std::collections::BTreeMap;
use std::time::Instant;

impl TuiApp {
    pub fn build_options(&self) -> InstallOptions {
        InstallOptions {
            profile: self.profile_level(),
            staging_dir: None,
            dry_run: self.dry_run,
            interactive: false,
            enable_argon: self.modules.enable_argon,
            enable_p10k: self.modules.enable_p10k,
            docker_data_root: self.modules.docker_data_root,
            continue_on_error: self.continue_on_error,
            software_plan: self.build_software_plan(),
        }
    }

    pub fn build_software_plan(&self) -> SoftwareTierPlan {
        let (picks, is_recommended) = match self.software_mode {
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
                (picks, true)
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
                (picks, false)
            }
            SoftwareMode::Manual => (self.software_picks.clone(), false),
        };

        SoftwareTierPlan::new(is_recommended, picks, self.theme_plan.clone(), None)
    }

    pub fn apply_preset(&mut self, preset: &Preset) {
        self.profile_idx = if preset.software_plan.full_install {
            2
        } else {
            1
        };
        self.theme_plan = preset.software_plan.theme_plan.clone();

        self.modules.enable_p10k = preset.tweaks.iter().any(|t| t == "enable_p10k");
        self.modules.enable_argon = preset.tweaks.iter().any(|t| t == "enable_argon");
        self.modules.docker_data_root = preset.tweaks.iter().any(|t| t == "docker_data_root");

        for (cat, id) in &preset.software_plan.selections {
            self.software_picks.insert(cat.clone(), id.clone());
        }
    }

    pub fn handle_software_key(&mut self, code: KeyCode) {
        let category = match self.catalog.categories.get(self.software_category_idx) {
            Some(category) => category,
            None => {
                self.screen = Screen::Confirm;
                self.menu_cursor = 0;
                return;
            }
        };

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

    pub fn selected_option_index(&self, category_idx: usize) -> Option<usize> {
        let category = self.catalog.categories.get(category_idx)?;
        let picked = self.software_picks.get(&category.display_name)?;

        let all_programs: Vec<&Program> = category
            .subcategories
            .iter()
            .flat_map(|sc| &sc.programs)
            .collect();

        all_programs.iter().position(|p| p.id == *picked)
    }

    #[allow(dead_code)]
    pub fn theme_menu_index(&self) -> usize {
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

    pub fn start_install(&mut self) {
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
}
