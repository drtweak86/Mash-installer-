use serde::{Deserialize, Serialize};
use ureq::Agent;

use crate::wallpaper::api::WallpaperApi;
use crate::wallpaper::error::WallpaperError;
use crate::wallpaper::types::{WallpaperImage, WallpaperResult};

/// Pixabay API client
#[derive(Debug, Clone)]
pub struct PixabayApi {
    agent: Agent,
    api_key: String,
}

impl PixabayApi {
    /// Creates a new Pixabay API client
    pub fn new(api_key: String) -> Self {
        Self {
            agent: Agent::new(),
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

impl WallpaperApi for PixabayApi {
    fn search(
        &self,
        query: &str,
        count: usize,
        page: usize,
    ) -> Result<WallpaperResult, WallpaperError> {
        let url = format!(
            "https://pixabay.com/api/?key={}&q={}&per_page={}&page={}&image_type=photo",
            self.api_key, query, count, page
        );

        let response = self.agent.get(&url).call().map_err(|e| {
            WallpaperError::ApiError(format!("Pixabay request failed: {}", e).into())
        })?;

        if response.status() != 200 {
            return Err(WallpaperError::ApiError(
                format!("Pixabay API error: {}", response.status()).into(),
            ));
        }

        let pixabay_response: PixabayResponse = response.into_json().map_err(|e| {
            WallpaperError::ApiError(format!("Pixabay JSON parsing failed: {}", e).into())
        })?;

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
