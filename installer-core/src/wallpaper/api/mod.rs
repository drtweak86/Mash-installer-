use super::error::WallpaperError;
use super::types::WallpaperResult;

pub mod pexels;
pub mod pixabay;
pub mod wallhaven;

/// Trait for wallpaper API clients
pub trait WallpaperApi: Send + Sync {
    /// Search for wallpapers
    fn search(
        &self,
        query: &str,
        count: usize,
        page: usize,
    ) -> Result<WallpaperResult, WallpaperError>;
}
