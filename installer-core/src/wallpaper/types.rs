use serde::{Deserialize, Serialize};

/// Result from a wallpaper API search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperResult {
    pub images: Vec<WallpaperImage>,
}

/// Individual wallpaper image information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperImage {
    pub id: String,
    pub url: String,
    pub source: String,
    pub resolution: String,
    pub category: String,
}

/// Wallpaper category configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperCategory {
    pub name: String,
    pub display_name: String,
    pub queries: Vec<String>,
    pub target_count: usize,
}

/// API keys for wallpaper sources
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiKeys {
    pub wallhaven: Option<String>,
    pub pexels: Option<String>,
    pub pixabay: Option<String>,
}

/// Download statistics
#[derive(Debug, Default, Clone)]
pub struct DownloadStats {
    pub success: usize,
    pub failed: usize,
    pub total: usize,
}

impl DownloadStats {
    /// Creates new download stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates stats with success
    pub fn add_success(&mut self) {
        self.success += 1;
        self.total += 1;
    }

    /// Updates stats with failure
    pub fn add_failure(&mut self) {
        self.failed += 1;
        self.total += 1;
    }
}
