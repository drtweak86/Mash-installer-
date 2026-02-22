use async_trait::async_trait;

use crate::wallpaper::error::WallpaperError;
use crate::wallpaper::types::WallpaperResult;

pub mod pexels;
pub mod pixabay;
pub mod wallhaven;

/// Trait for wallpaper API clients
#[async_trait]
pub trait WallpaperApi: Send + Sync {
    /// Search for wallpapers
    async fn search(
        &self,
        query: &str,
        count: usize,
        page: usize,
    ) -> Result<WallpaperResult, WallpaperError>;
}
