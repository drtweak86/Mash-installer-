use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tracing::{info, warn};

use crate::context::PhaseContext;
use crate::system::{RealSystem, SystemOps};
use crate::wallpaper::api::{
    pexels::PexelsApi, pixabay::PixabayApi, wallhaven::WallhavenApi, WallpaperApi,
};
use crate::wallpaper::config::WallpaperConfig;
use crate::wallpaper::error::WallpaperError;
use crate::wallpaper::types::{DownloadStats, WallpaperCategory, WallpaperImage};

/// Downloads wallpapers from multiple sources with concurrent downloads
pub async fn download_wallpapers(
    config: &WallpaperConfig,
    _system_ops: &dyn SystemOps,
    phase_ctx: &mut PhaseContext<'_>,
) -> Result<DownloadStats, WallpaperError> {
    // Validate configuration
    config.validate()?;

    // Create semaphore to limit concurrent downloads (max 3 to avoid API bans)
    let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
    let mut stats = DownloadStats::new();

    // Whimsical message
    phase_ctx.record_action("🖼️  Summoning retro-futuristic wallpapers from the digital void...");

    // Create output directory
    RealSystem
        .create_dir_all(&config.output_dir)
        .map_err(|e| WallpaperError::FsError(std::io::Error::other(e.to_string())))?;

    // Download from each category
    for category in &config.categories {
        let category_stats =
            download_category(category, config, &semaphore, _system_ops, phase_ctx).await?;

        stats.success += category_stats.success;
        stats.failed += category_stats.failed;
    }

    // Whimsical completion message
    phase_ctx.record_action("🎉  Wallpaper collection complete! Your desktop awaits...");

    Ok(stats)
}

/// Downloads wallpapers for a single category
async fn download_category(
    category: &WallpaperCategory,
    config: &WallpaperConfig,
    semaphore: &Arc<Semaphore>,
    _system_ops: &dyn SystemOps,
    phase_ctx: &mut PhaseContext<'_>,
) -> Result<DownloadStats, WallpaperError> {
    let mut stats = DownloadStats::new();
    let mut join_set = JoinSet::new();

    // Create API clients based on available keys
    let mut api_clients: Vec<Box<dyn WallpaperApi>> = Vec::new();

    if let Some(key) = &config.api_keys.wallhaven {
        api_clients.push(Box::new(WallhavenApi::new(key.clone())));
    }

    if let Some(key) = &config.api_keys.pexels {
        api_clients.push(Box::new(PexelsApi::new(key.clone())));
    }

    if let Some(key) = &config.api_keys.pixabay {
        api_clients.push(Box::new(PixabayApi::new(key.clone())));
    }

    // If no API keys are configured, warn and skip
    if api_clients.is_empty() {
        phase_ctx.record_warning("No API keys configured. Please obtain keys from:");
        phase_ctx.record_warning("Wallhaven: https://wallhaven.cc/settings/account");
        phase_ctx.record_warning("Pexels: https://www.pexels.com/api/");
        phase_ctx.record_warning("Pixabay: https://pixabay.com/api/docs/");
        return Ok(stats);
    }

    // Search for images using each API
    for query in &category.queries {
        for api in &api_clients {
            let result = api.search(query, 20, 1).await?;

            for mut image in result.images {
                image.category = category.name.clone();

                // Spawn download task
                let semaphore_clone = semaphore.clone();
                let config_clone = config.clone();
                let image_clone = image.clone();

                join_set.spawn(async move {
                    let result = download_single(
                        image_clone,
                        config_clone.output_dir.clone(),
                        semaphore_clone,
                    )
                    .await;

                    match result {
                        Ok(_) => {
                            let mut stats = DownloadStats::new();
                            stats.add_success();
                            stats
                        }
                        Err(e) => {
                            warn!("Failed to download {}: {}", image.id, e);
                            let mut stats = DownloadStats::new();
                            stats.add_failure();
                            stats
                        }
                    }
                });
            }
        }
    }

    // Wait for all downloads to complete
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(category_stats) => {
                stats.success += category_stats.success;
                stats.failed += category_stats.failed;
            }
            Err(e) => {
                warn!("Download task failed: {}", e);
                stats.failed += 1;
            }
        }
    }

    Ok(stats)
}

/// Downloads a single wallpaper image
async fn download_single(
    image: WallpaperImage,
    output_dir: PathBuf,
    semaphore: Arc<Semaphore>,
) -> Result<(), WallpaperError> {
    // Acquire semaphore permit (limits concurrent downloads)
    let _permit = semaphore
        .acquire()
        .await
        .map_err(|e| WallpaperError::SemaphoreError(e.to_string()))?;

    // Create destination path
    let filename = format!("{}_{}.jpg", image.category, image.id);
    let dest_path = output_dir.join(filename);
    let temp_path = dest_path.with_extension("tmp");

    // Download the image
    let response = reqwest::get(&image.url).await?;
    let bytes = response.bytes().await?;

    // Write using SystemOps (no direct fs writes)
    RealSystem
        .write_file(&temp_path, &bytes)
        .map_err(|e| WallpaperError::FsError(std::io::Error::other(e.to_string())))?;

    // Rename using SystemOps
    RealSystem
        .rename(&temp_path, &dest_path)
        .map_err(|e| WallpaperError::FsError(std::io::Error::other(e.to_string())))?;

    info!("Downloaded wallpaper: {} ({})", image.id, image.source);

    Ok(())
}
