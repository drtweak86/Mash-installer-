use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::error::WallpaperError;
use super::types::{ApiKeys, WallpaperCategory};

/// Wallpaper downloader configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperConfig {
    pub output_dir: PathBuf,
    pub max_concurrent: usize,
    pub api_keys: ApiKeys,
    pub categories: Vec<WallpaperCategory>,
}

impl Default for WallpaperConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("/usr/share/backgrounds/retro"),
            max_concurrent: 3, // Limited to avoid API bans
            api_keys: ApiKeys::default(),
            categories: vec![
                // 8 categories from Python implementation (5999 total wallpapers)
                WallpaperCategory {
                    name: "retro".to_string(),
                    display_name: "Retro Computing".to_string(),
                    queries: vec![
                        "bbc micro".to_string(),
                        "unix workstation".to_string(),
                        "vintage computer".to_string(),
                        "retro terminal".to_string(),
                    ],
                    target_count: 1000,
                },
                WallpaperCategory {
                    name: "games".to_string(),
                    display_name: "Video Games".to_string(),
                    queries: vec![
                        "arcade cabinet".to_string(),
                        "pixel art".to_string(),
                        "retro console".to_string(),
                        "classic games".to_string(),
                    ],
                    target_count: 1000,
                },
                WallpaperCategory {
                    name: "anime".to_string(),
                    display_name: "Anime".to_string(),
                    queries: vec![
                        "cyberpunk anime".to_string(),
                        "retro anime".to_string(),
                        "anime wallpaper".to_string(),
                    ],
                    target_count: 1000,
                },
                WallpaperCategory {
                    name: "dc".to_string(),
                    display_name: "DC Comics".to_string(),
                    queries: vec![
                        "batman wallpaper".to_string(),
                        "superman wallpaper".to_string(),
                        "justice league".to_string(),
                    ],
                    target_count: 1000,
                },
                WallpaperCategory {
                    name: "marvel".to_string(),
                    display_name: "Marvel Comics".to_string(),
                    queries: vec![
                        "iron man wallpaper".to_string(),
                        "spider man wallpaper".to_string(),
                        "avengers wallpaper".to_string(),
                    ],
                    target_count: 1000,
                },
                WallpaperCategory {
                    name: "judge_dredd".to_string(),
                    display_name: "Judge Dredd".to_string(),
                    queries: vec![
                        "mega city one".to_string(),
                        "judge dredd".to_string(),
                        "2000 ad retro".to_string(),
                    ],
                    target_count: 500,
                },
                WallpaperCategory {
                    name: "star_wars".to_string(),
                    display_name: "Star Wars".to_string(),
                    queries: vec![
                        "star wars droids".to_string(),
                        "star wars terminal".to_string(),
                        "retro star wars".to_string(),
                    ],
                    target_count: 500,
                },
                WallpaperCategory {
                    name: "cyberpunk".to_string(),
                    display_name: "Cyberpunk".to_string(),
                    queries: vec![
                        "cyberpunk computer".to_string(),
                        "hacker aesthetic".to_string(),
                        "neon tech".to_string(),
                    ],
                    target_count: 500,
                },
            ],
        }
    }
}

impl WallpaperConfig {
    /// Override API keys from environment variables.
    ///
    /// Reads `MASH_WALLHAVEN_KEY`, `MASH_PEXELS_KEY`, and `MASH_PIXABAY_KEY`.
    /// Any non-empty value found in the environment takes precedence over the
    /// current config value.
    ///
    /// # Example
    /// ```no_run
    /// use installer_core::WallpaperConfig;
    /// let config = WallpaperConfig::default().with_env_keys();
    /// ```
    pub fn with_env_keys(mut self) -> Self {
        if let Ok(key) = std::env::var("MASH_WALLHAVEN_KEY") {
            if !key.trim().is_empty() {
                self.api_keys.wallhaven = Some(key);
            }
        }
        if let Ok(key) = std::env::var("MASH_PEXELS_KEY") {
            if !key.trim().is_empty() {
                self.api_keys.pexels = Some(key);
            }
        }
        if let Ok(key) = std::env::var("MASH_PIXABAY_KEY") {
            if !key.trim().is_empty() {
                self.api_keys.pixabay = Some(key);
            }
        }
        self
    }

    /// Validates the configuration
    pub fn validate(&self) -> Result<(), WallpaperError> {
        // Validate output directory path
        if self.output_dir.as_os_str().is_empty() {
            return Err(WallpaperError::config_error(
                "Output directory cannot be empty",
            ));
        }

        // Validate max concurrent downloads
        if self.max_concurrent == 0 {
            return Err(WallpaperError::config_error(
                "Max concurrent downloads must be at least 1",
            ));
        }

        // Validate categories
        if self.categories.is_empty() {
            return Err(WallpaperError::config_error(
                "At least one category must be configured",
            ));
        }

        // Validate total count matches expected (6500)
        let total: usize = self.categories.iter().map(|c| c.target_count).sum();
        if total != 6500 {
            return Err(WallpaperError::config_error(format!(
                "Expected 6500 total wallpapers, got {}",
                total
            )));
        }

        Ok(())
    }
}
