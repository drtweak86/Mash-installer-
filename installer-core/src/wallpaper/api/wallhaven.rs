use serde::{Deserialize, Serialize};
use ureq::Agent;

use crate::wallpaper::api::WallpaperApi;
use crate::wallpaper::error::WallpaperError;
use crate::wallpaper::types::{WallpaperImage, WallpaperResult};

/// Wallhaven API client
#[derive(Debug, Clone)]
pub struct WallhavenApi {
    agent: Agent,
    api_key: String,
}

impl WallhavenApi {
    /// Creates a new Wallhaven API client
    pub fn new(api_key: String) -> Self {
        Self {
            agent: Agent::new(),
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

impl WallpaperApi for WallhavenApi {
    fn search(
        &self,
        query: &str,
        count: usize,
        page: usize,
    ) -> Result<WallpaperResult, WallpaperError> {
        let url = format!(
            "https://wallhaven.cc/api/v1/search?api_key={}&q={}&page={}&per_page={}",
            self.api_key, query, page, count
        );

        let response = self.agent.get(&url).call().map_err(|e| {
            WallpaperError::ApiError(format!("Wallhaven request failed: {}", e).into())
        })?;

        if response.status() != 200 {
            return Err(WallpaperError::ApiError(
                format!("Wallhaven API error: {}", response.status()).into(),
            ));
        }

        let wallhaven_response: WallhavenResponse = response.into_json().map_err(|e| {
            WallpaperError::ApiError(format!("Wallhaven JSON parsing failed: {}", e).into())
        })?;

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
