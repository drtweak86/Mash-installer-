//! Wallpaper Downloader Library
//!
//! A Rust implementation for downloading retro-futuristic wallpapers from Wallhaven API.
//!
//! # Features
//! - Download wallpapers from 8 categories (retro, games, anime, etc.)
//! - Parallel downloads with progress reporting
//! - Duplicate detection using SHA256 hashing
//! - Resume support for existing downloads
//! - First-boot mode for silent operation
//!
//! # Example
//! ```no_run
//! use wallpaper_downloader::{Config, Downloader};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::parse();
//!     let mut downloader = Downloader::new(&config)?;
//!     downloader.download_all().await?;
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod config;
pub mod download;
pub mod error;
pub mod types;

pub use config::Config;
pub use download::Downloader;
pub use error::DownloadError;
