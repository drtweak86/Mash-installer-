//! Configuration handling for the wallpaper downloader

use clap::Parser;
use std::path::PathBuf;
use crate::error::{DownloadError, Result};
use crate::types::default_output_dir;

/// Command-line configuration
#[derive(Debug, Parser, Clone)]
#[command(name = "wallpaper-downloader")]
#[command(about = "BBC/UNIX Retro-Futuristic Wallpaper Downloader - 8 Categories", long_about = None)]
pub struct Config {
    /// Category to download (all, retro, games, anime, dc, marvel, judge_dredd, star_wars, cyberpunk)
    #[clap(long, default_value = "all")]
    pub category: String,

    /// Maximum number of wallpapers to download (max 6000)
    #[clap(long, default_value_t = 6000)]
    pub limit: usize,

    /// Output directory for wallpapers
    #[clap(long)]
    pub output_dir: Option<PathBuf>,

    /// Run in first-boot mode (minimal output)
    #[clap(long)]
    pub first_boot: bool,

    /// Wallhaven API key
    #[clap(long)]
    pub api_key: Option<String>,

    /// Maximum number of parallel downloads
    #[clap(long, default_value_t = 4)]
    pub max_workers: usize,

    /// Timeout for downloads in seconds
    #[clap(long, default_value_t = 30)]
    pub timeout: u64,
}

impl Config {
    /// Parse command-line arguments
    pub fn parse() -> Self {
        let mut config = <Self as Parser>::parse();
        
        // Read API key from environment variable if not provided
        if config.api_key.is_none() {
            if let Ok(api_key) = std::env::var("WALLHAVEN_API_KEY") {
                config.api_key = Some(api_key);
            }
        }
        
        config
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate category
        if self.category != "all" && !crate::types::CATEGORIES.iter()
            .any(|c| c.name == self.category)
        {
            let valid_categories = crate::types::CATEGORIES.iter()
                .map(|c| c.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(DownloadError::InvalidCategory(
                self.category.clone(),
                valid_categories,
            ));
        }

        // Validate limit
        if self.limit > 6000 {
            return Err(DownloadError::Config(
                "Maximum limit is 6000 wallpapers".to_string(),
            ));
        }

        // Validate output directory
        let output_dir = self.output_dir.clone().unwrap_or_else(default_output_dir);
        if !output_dir.exists() {
            std::fs::create_dir_all(&output_dir)
                .map_err(|e| DownloadError::Config(format!(
                    "Could not create output directory: {}",
                    e
                )))?;
        }

        // Validate API key
        if self.api_key.as_deref() == Some("YOUR_API_KEY_HERE") || self.api_key.is_none() {
            log::warn!("No API key provided. Using placeholder. Downloads may fail.");
        }

        Ok(())
    }

    /// Get the API key, using a placeholder if not provided
    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
            .unwrap_or_else(|| "YOUR_API_KEY_HERE".to_string())
    }
}
