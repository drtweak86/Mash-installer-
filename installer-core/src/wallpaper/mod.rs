/// Wallpaper downloader module for retro-futuristic wallpapers
///
/// Downloads wallpapers from multiple sources (Wallhaven, Pexels, Pixabay)
/// with concurrent downloads limited by a semaphore to avoid API bans.
pub mod api;
pub mod config;
pub mod download;
pub mod error;
pub mod types;

#[cfg(test)]
mod tests;

pub use config::WallpaperConfig;
pub use download::download_wallpapers;
pub use error::WallpaperError;
