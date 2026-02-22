use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::wallpaper::api::WallpaperApi;
use crate::wallpaper::error::WallpaperError;
use crate::wallpaper::types::{WallpaperImage, WallpaperResult};

/// Wallhaven API client
#[derive(Debug, Clone)]
pub struct WallhavenApi {
    client: Client,
    api_key: String,
}

impl WallhavenApi {
    /// Creates a new Wallhaven API client
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

/// Wallhaven API response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct WallhavenResponse {
    pub data: Vec<WallhavenImage>,
}

/// Wallhaven image structure
#[derive(Debug, Serialize, Deserialize)]
pub struct WallhavenImage {
    pub id: String,
    pub path: String,
    pub resolution: String,
    pub short_url: String,
}

#[async_trait]
impl WallpaperApi for WallhavenApi {
    async fn search(
        &self,
        query: &str,
        count: usize,
        page: usize,
    ) -> Result<WallpaperResult, WallpaperError> {
        let url = format!(
            "https://wallhaven.cc/api/v1/search?api_key={}&q={}&page={}&per_page={}",
            self.api_key, query, page, count
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(WallpaperError::ApiError(
                format!("Wallhaven API error: {}", response.status()).into(),
            ));
        }

        let wallhaven_response: WallhavenResponse = response.json().await?;

        let images = wallhaven_response
            .data
            .into_iter()
            .map(|img| WallpaperImage {
                id: img.id,
                url: img.path,
                source: "wallhaven".to_string(),
                resolution: img.resolution,
                category: "".to_string(), // Will be set by caller
            })
            .collect();

        Ok(WallpaperResult { images })
    }
}
