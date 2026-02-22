use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::wallpaper::api::WallpaperApi;
use crate::wallpaper::error::WallpaperError;
use crate::wallpaper::types::{WallpaperImage, WallpaperResult};

/// Pixabay API client
#[derive(Debug, Clone)]
pub struct PixabayApi {
    client: Client,
    api_key: String,
}

impl PixabayApi {
    /// Creates a new Pixabay API client
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

/// Pixabay API response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct PixabayResponse {
    pub total: usize,
    pub total_hits: usize,
    pub hits: Vec<PixabayHit>,
}

/// Pixabay hit structure
#[derive(Debug, Serialize, Deserialize)]
pub struct PixabayHit {
    pub id: usize,
    pub webformat_url: String,
    pub large_image_url: String,
    pub image_width: usize,
    pub image_height: usize,
}

#[async_trait]
impl WallpaperApi for PixabayApi {
    async fn search(
        &self,
        query: &str,
        count: usize,
        page: usize,
    ) -> Result<WallpaperResult, WallpaperError> {
        let url = format!(
            "https://pixabay.com/api/?key={}&q={}&per_page={}&page={}&image_type=photo",
            self.api_key, query, count, page
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(WallpaperError::ApiError(
                format!("Pixabay API error: {}", response.status()).into(),
            ));
        }

        let pixabay_response: PixabayResponse = response.json().await?;

        let images = pixabay_response
            .hits
            .into_iter()
            .map(|hit| WallpaperImage {
                id: hit.id.to_string(),
                url: hit.large_image_url, // Use large size for better quality
                source: "pixabay".to_string(),
                resolution: format!("{}x{}", hit.image_width, hit.image_height),
                category: "".to_string(), // Will be set by caller
            })
            .collect();

        Ok(WallpaperResult { images })
    }
}
