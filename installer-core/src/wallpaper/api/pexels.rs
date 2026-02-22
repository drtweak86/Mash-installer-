use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::wallpaper::api::WallpaperApi;
use crate::wallpaper::error::WallpaperError;
use crate::wallpaper::types::{WallpaperImage, WallpaperResult};

/// Pexels API client
#[derive(Debug, Clone)]
pub struct PexelsApi {
    client: Client,
    api_key: String,
}

impl PexelsApi {
    /// Creates a new Pexels API client
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

/// Pexels API response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct PexelsResponse {
    pub per_page: usize,
    pub pages: usize,
    pub photos: Vec<PexelsPhoto>,
}

/// Pexels photo structure
#[derive(Debug, Serialize, Deserialize)]
pub struct PexelsPhoto {
    pub id: String,
    pub width: usize,
    pub height: usize,
    pub src: PexelsImageSources,
}

/// Pexels image sources
#[derive(Debug, Serialize, Deserialize)]
pub struct PexelsImageSources {
    pub original: String,
    pub large: String,
    pub medium: String,
}

#[async_trait]
impl WallpaperApi for PexelsApi {
    async fn search(
        &self,
        query: &str,
        count: usize,
        page: usize,
    ) -> Result<WallpaperResult, WallpaperError> {
        let url = format!(
            "https://api.pexels.com/v1/search?query={}&per_page={}&page={}",
            query, count, page
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(WallpaperError::ApiError(
                format!("Pexels API error: {}", response.status()).into(),
            ));
        }

        let pexels_response: PexelsResponse = response.json().await?;

        let images = pexels_response
            .photos
            .into_iter()
            .map(|photo| WallpaperImage {
                id: photo.id,
                url: photo.src.large, // Use large size for better quality
                source: "pexels".to_string(),
                resolution: format!("{}x{}", photo.width, photo.height),
                category: "".to_string(), // Will be set by caller
            })
            .collect();

        Ok(WallpaperResult { images })
    }
}
