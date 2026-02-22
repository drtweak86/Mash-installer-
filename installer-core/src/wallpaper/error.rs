use thiserror::Error;

/// Errors that can occur during wallpaper operations
#[derive(Error, Debug)]
pub enum WallpaperError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("API error: {0}")]
    ApiError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Download error: {0}")]
    DownloadError(String),

    #[error("Filesystem error: {0}")]
    FsError(#[from] std::io::Error),

    #[error("Missing API key for {0}")]
    MissingApiKey(&'static str),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Semaphore error: {0}")]
    SemaphoreError(String),
}

impl WallpaperError {
    /// Creates a download error with context
    pub fn download_error(msg: impl Into<String>) -> Self {
        WallpaperError::DownloadError(msg.into())
    }

    /// Creates a config error with context
    pub fn config_error(msg: impl Into<String>) -> Self {
        WallpaperError::ConfigError(msg.into())
    }
}
