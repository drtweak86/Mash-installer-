//! Data types for the wallpaper downloader

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::path::PathBuf;

/// Wallpaper metadata from Wallhaven API
#[derive(Debug, Deserialize, Clone)]
pub struct Wallpaper {
    pub id: String,
    pub url: String,
    pub thumbnail: String,
    pub resolution: String,
    pub views: u32,
    pub favorites: u32,
    pub category: String,
}

/// Wallhaven API response
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<Wallpaper>,
    pub meta: ApiMeta,
}

/// Wallhaven API metadata
#[derive(Debug, Deserialize)]
pub struct ApiMeta {
    pub current_page: u32,
    pub last_page: u32,
    pub per_page: u32,
    pub total: u32,
}

/// Category definition
#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub display_name: String,
    pub queries: Vec<String>,
    pub count: usize,
}

/// Download statistics
#[derive(Debug)]
pub struct DownloadStats {
    pub success_count: usize,
    pub fail_count: usize,
    pub start_time: std::time::Instant,
}

impl Default for DownloadStats {
    fn default() -> Self {
        Self {
            success_count: 0,
            fail_count: 0,
            start_time: std::time::Instant::now(),
        }
    }
}

impl DownloadStats {
    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
}

/// Wallpaper category definitions (8 categories, 5999 total images)
pub static CATEGORIES: Lazy<Vec<Category>> = Lazy::new(|| {
    vec![
        Category {
            name: "retro".to_string(),
            display_name: "Retro Computing".to_string(),
            queries: vec![
                "retro computer".to_string(),
                "bbc micro".to_string(),
                "unix workstation".to_string(),
                "vintage tech".to_string(),
                "old computer".to_string(),
                "80s computer".to_string(),
                "90s computer".to_string(),
                "amiga".to_string(),
                "commodore 64".to_string(),
                "apple ii".to_string(),
                "terminal".to_string(),
                "command line".to_string(),
                "text mode".to_string(),
                "green screen".to_string(),
            ],
            count: 1000,
        },
        Category {
            name: "games".to_string(),
            display_name: "Video Games".to_string(),
            queries: vec![
                "retro video games".to_string(),
                "arcade cabinets".to_string(),
                "pixel art".to_string(),
                "8-bit games".to_string(),
                "16-bit games".to_string(),
                "classic video games".to_string(),
                "retro gaming".to_string(),
                "videogame art".to_string(),
                "game consoles".to_string(),
                "nintendo".to_string(),
                "sega".to_string(),
                "atari".to_string(),
                "playstation 1".to_string(),
            ],
            count: 1000,
        },
        Category {
            name: "anime".to_string(),
            display_name: "Anime".to_string(),
            queries: vec![
                "retro anime".to_string(),
                "cyberpunk anime".to_string(),
                "80s anime".to_string(),
                "90s anime".to_string(),
                "anime computers".to_string(),
                "anime technology".to_string(),
            ],
            count: 625,
        },
        Category {
            name: "dc".to_string(),
            display_name: "DC Comics".to_string(),
            queries: vec![
                "dc comics".to_string(),
                "batman".to_string(),
                "superman".to_string(),
                "justice league".to_string(),
                "dc retro".to_string(),
                "dc computers".to_string(),
                "dc technology".to_string(),
            ],
            count: 625,
        },
        Category {
            name: "marvel".to_string(),
            display_name: "Marvel Comics".to_string(),
            queries: vec![
                "marvel comics".to_string(),
                "iron man".to_string(),
                "spider man".to_string(),
                "avengers".to_string(),
                "marvel retro".to_string(),
                "marvel computers".to_string(),
                "marvel technology".to_string(),
            ],
            count: 625,
        },
        Category {
            name: "judge_dredd".to_string(),
            display_name: "Judge Dredd/Lobo".to_string(),
            queries: vec![
                "judge dredd".to_string(),
                "lobo".to_string(),
                "2000 ad".to_string(),
                "mega city one".to_string(),
                "judge dredd computer".to_string(),
                "lobo comic".to_string(),
                "2000 ad retro".to_string(),
            ],
            count: 562,
        },
        Category {
            name: "star_wars".to_string(),
            display_name: "Star Wars".to_string(),
            queries: vec![
                "star wars retro".to_string(),
                "star wars computers".to_string(),
                "star wars technology".to_string(),
                "droids".to_string(),
                "star wars terminals".to_string(),
                "star wars retro tech".to_string(),
            ],
            count: 562,
        },
        Category {
            name: "cyberpunk".to_string(),
            display_name: "Cyberpunk".to_string(),
            queries: vec![
                "cyberpunk".to_string(),
                "cyberpunk computers".to_string(),
                "cyberpunk terminals".to_string(),
                "neon computers".to_string(),
                "retro futurism".to_string(),
                "cyberpunk technology".to_string(),
                "hacker aesthetic".to_string(),
                "terminal aesthetic".to_string(),
            ],
            count: 1000,
        },
    ]
});

/// Default output directory
pub fn default_output_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not determine home directory")
        .join("Pictures")
        .join("RetroWallpapers")
}
