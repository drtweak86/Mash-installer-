//! Error types for the wallpaper downloader

use thiserror::Error;

/// Main error type for the wallpaper downloader
#[derive(Debug, Error)]
pub enum DownloadError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    /// File system error
    #[error("File system error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// API-specific error
    #[error("API error: {0}")]
    Api(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// Download error
    #[error("Download failed: {0}")]
    Download(String),
    
    /// Rate limiting error
    #[error("Rate limited by API")]
    RateLimited,
    
    /// Invalid category error
    #[error("Invalid category: {0}. Available categories: {1}")]
    InvalidCategory(String, String),
    
    /// Semaphore acquisition error
    #[error("Semaphore error: {0}")]
    Semaphore(String),
}

impl From<tokio::sync::AcquireError> for DownloadError {
    fn from(err: tokio::sync::AcquireError) -> Self {
        DownloadError::Semaphore(err.to_string())
    }
}

/// Result type for the wallpaper downloader
pub type Result<T> = std::result::Result<T, DownloadError>;
