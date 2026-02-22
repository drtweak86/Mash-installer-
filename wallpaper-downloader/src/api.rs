//! Wallhaven API client

use reqwest::Client;
use serde_json::Value;
use std::collections::HashSet;
use std::time::Duration;

use crate::error::{DownloadError, Result};
use crate::types::{ApiResponse, Wallpaper};

/// Wallhaven API client
#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: String,
    user_agent: String,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://wallhaven.cc/api/v1/search".to_string(),
            api_key,
            user_agent: "MASH-Retro-Wallpaper-Downloader/1.0".to_string(),
        }
    }

    /// Search for wallpapers by query
    pub async fn search(&self, query: &str, count: usize) -> Result<Vec<Wallpaper>> {
        let mut images = Vec::new();
        let mut page = 1;

        // Wallhaven API parameters
        let mut params = vec![
            ("categories", "111"),  // General, Anime, People
            ("purity", "100"),      // SFW only
            ("sorting", "relevance"),
            ("atleast", "1920x1080"),  // Minimum resolution
            ("apikey", &self.api_key),
            ("q", query),
        ];

        while images.len() < count && page <= 50 {
            // Create a new params vector for each page to avoid lifetime issues
            let page_str = page.to_string();
            let mut page_params = params.clone();
            page_params.push(("page", page_str.as_str()));

            let response = self.client
                .get(&self.base_url)
                .query(&page_params)
                .header("User-Agent", &self.user_agent)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(DownloadError::Api(format!(
                    "API request failed with status {}: {}",
                    status, error_text
                )));
            }

            let json: Value = response.json().await?;

            // Parse the response
            if let Some(data_array) = json.get("data").and_then(|d| d.as_array()) {
                for item in data_array {
                    if images.len() >= count {
                        break;
                    }

                    let wallpaper = self.parse_wallpaper(item)?;
                    images.push(wallpaper);
                }
            }

            // Check for rate limiting
            if let Some(meta) = json.get("meta") {
                if let Some(rate_limit) = meta.get("rate_limit") {
                    if rate_limit.get("remaining").and_then(|r| r.as_u64()) == Some(0) {
                        return Err(DownloadError::RateLimited);
                    }
                }
            }

            page += 1;
        }

        Ok(images)
    }

    /// Parse a wallpaper from JSON
    fn parse_wallpaper(&self, item: &Value) -> Result<Wallpaper> {
        Ok(Wallpaper {
            id: item.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string(),
            url: item.get("path").and_then(|p| p.as_str()).unwrap_or("").to_string(),
            thumbnail: item.get("thumbs")
                .and_then(|t| t.get("large"))
                .and_then(|l| l.as_str())
                .unwrap_or("")
                .to_string(),
            resolution: item.get("resolution").and_then(|r| r.as_str()).unwrap_or("").to_string(),
            views: item.get("views").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            favorites: item.get("favorites").and_then(|f| f.as_u64()).unwrap_or(0) as u32,
            category: item.get("category").and_then(|c| c.as_str()).unwrap_or("").to_string(),
        })
    }

    /// Get all wallpapers for a category
    pub async fn get_category_wallpapers(&self, _category: &str, queries: &[String], count: usize) -> Result<Vec<Wallpaper>> {
        let mut all_images = Vec::new();
        let images_per_query = std::cmp::max(1, count / queries.len());

        for query in queries {
            log::info!("Searching for '{}'...", query);
            let images = self.search(query, images_per_query).await?;
            log::info!("  ✓ Found {} images", images.len());
            all_images.extend(images);
        }

        Ok(all_images)
    }
}
