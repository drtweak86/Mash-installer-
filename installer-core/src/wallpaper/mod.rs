/// Wallpaper downloader module for retro-futuristic wallpapers
///
/// Downloads wallpapers from multiple sources (Wallhaven, Pexels, Pixabay)
/// with concurrent downloads limited by a semaphore to avoid API bans.
pub mod api;
pub mod config;
pub mod download;
pub mod error;
pub mod harvest;
pub mod types;

pub use config::{HarvestConfig, WallpaperConfig};
pub use download::download_wallpapers;
pub use error::WallpaperError;
pub use harvest::{harvest_wallpapers, WallpaperHarvester};

#[cfg(test)]
mod tests {
    use super::config::WallpaperConfig;
    use std::path::PathBuf;

    #[test]
    fn test_config_validation() {
        let config = WallpaperConfig::default();
        assert!(config.validate().is_ok());

        // Test invalid output directory
        let invalid_config = WallpaperConfig {
            output_dir: PathBuf::from(""),
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());

        // Test zero max concurrent
        let invalid_config = WallpaperConfig {
            max_concurrent: 0,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());

        // Test empty categories
        let invalid_config = WallpaperConfig {
            categories: vec![],
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());

        // Test wrong total count
        let mut config = WallpaperConfig::default();
        config.categories[0].target_count = 100; // Change one category
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_category_counts() {
        let config = WallpaperConfig::default();
        let total: usize = config.categories.iter().map(|c| c.target_count).sum();
        assert_eq!(total, 6500); // Match Rust implementation
    }
}
