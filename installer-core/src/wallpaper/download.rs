use std::io::Read;
use std::path::Path;

use tracing::warn;

use crate::model::phase::{PhaseEvent, PhaseObserver};
use crate::system::system_ops::SystemOps;
use crate::wallpaper::api::{
    pexels::PexelsApi, pixabay::PixabayApi, wallhaven::WallhavenApi, WallpaperApi,
};
use crate::wallpaper::config::WallpaperConfig;
use crate::wallpaper::error::WallpaperError;
use crate::wallpaper::types::{DownloadStats, WallpaperImage};

/// Downloads wallpapers from multiple sources
pub fn download_wallpapers(
    config: &WallpaperConfig,
    _system_ops: &dyn SystemOps,
    observer: &mut dyn PhaseObserver,
) -> Result<DownloadStats, WallpaperError> {
    // Validate configuration
    config.validate()?;

    let mut stats = DownloadStats::new();

    // Whimsical message
    observer.on_event(PhaseEvent::Warning {
        message: "🚀  Launching plasma ore collection mission...".to_string(),
    });

    let agent = ureq::Agent::new();

    for category in &config.categories {
        if category.target_count == 0 {
            continue;
        }

        observer.on_event(PhaseEvent::Warning {
            message: format!("🎨  Painting {} with neon runes...", category.name).to_string(),
        });

        // Use the first query if available
        let query = category
            .queries
            .first()
            .map(|s| s.as_str())
            .unwrap_or(&category.name);

        // Try sources in order
        let mut category_images = Vec::new();

        // 1. Wallhaven
        if let Some(key) = &config.api_keys.wallhaven {
            let api = WallhavenApi::new(key.clone());
            if let Ok(result) = api.search(query, category.target_count, 1) {
                category_images.extend(result.images);
            }
        }

        // 2. Pexels (if still need more)
        if category_images.len() < category.target_count {
            if let Some(key) = &config.api_keys.pexels {
                let api = PexelsApi::new(key.clone());
                let needed = category.target_count - category_images.len();
                if let Ok(result) = api.search(query, needed, 1) {
                    category_images.extend(result.images);
                }
            }
        }

        // 3. Pixabay (if still need more)
        if category_images.len() < category.target_count {
            if let Some(key) = &config.api_keys.pixabay {
                let api = PixabayApi::new(key.clone());
                let needed = category.target_count - category_images.len();
                if let Ok(result) = api.search(query, needed, 1) {
                    category_images.extend(result.images);
                }
            }
        }

        // Download the images sequentially for simplicity and to reduce dependencies
        // (If performance is an issue, this could be threaded)
        for mut image in category_images.into_iter().take(category.target_count) {
            image.category = category.name.clone();

            match download_single_image(&agent, &image, &config.output_dir, &category.name) {
                Ok(_) => stats.add_success(),
                Err(e) => {
                    warn!("Download failed for {}: {}", image.url, e);
                    stats.add_failure();
                }
            }
        }
    }

    Ok(stats)
}

fn download_single_image(
    agent: &ureq::Agent,
    image: &WallpaperImage,
    output_dir: &Path,
    category: &str,
) -> Result<(), WallpaperError> {
    let category_dir = output_dir.join(category);
    if !category_dir.exists() {
        std::fs::create_dir_all(&category_dir)?;
    }

    let file_name = format!("{}.jpg", image.id);
    let dest_path = category_dir.join(file_name);

    if dest_path.exists() {
        return Ok(());
    }

    let response = agent
        .get(&image.url)
        .call()
        .map_err(|e| WallpaperError::ApiError(format!("Download request failed: {}", e).into()))?;

    if response.status() != 200 {
        return Err(WallpaperError::ApiError(
            format!("Download error {}: {}", response.status(), image.url).into(),
        ));
    }

    let mut reader = response.into_reader();
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;

    std::fs::write(dest_path, bytes)?;

    Ok(())
}
