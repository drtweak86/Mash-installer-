use anyhow::Result;

use crate::context::PhaseContext;
use crate::sys_ops::RealSystem;
use crate::wallpaper::{download_wallpapers, HarvestConfig, WallpaperConfig, WallpaperHarvester};
use crate::PhaseResult;

/// Wallpaper installation phase
pub fn install_phase(ctx: &mut PhaseContext<'_>) -> Result<PhaseResult> {
    // Whimsical starting message
    ctx.record_action("🚀  Launching plasma ore collection mission...");
    ctx.record_action("🎨  Painting your desktop with neon runes...");

    let config = WallpaperConfig::default();

    // Check for API keys for traditional download
    let has_api_keys = config.api_keys.wallhaven.is_some()
        || config.api_keys.pexels.is_some()
        || config.api_keys.pixabay.is_some();

    // First, try the harvest method (no API keys required)
    ctx.record_action("🌾  Attempting wallpaper harvest (no API keys required)...");

    let harvest_config = HarvestConfig::default();
    let harvester = WallpaperHarvester::new(harvest_config)?;

    // We need to pass the observer from ctx
    let harvest_result = harvester.run(ctx.observer);

    match harvest_result {
        Ok(_) => {
            ctx.record_action("✅  Wallpaper harvest completed successfully.");
        }
        Err(e) => {
            ctx.record_warning(format!("⚠️  Wallpaper harvest failed: {}. Falling back to traditional download if keys are present.", e));
        }
    }

    // If we have API keys, also run the traditional search/download
    if has_api_keys {
        ctx.record_action("📡  API keys detected. Searching for specific high-res runes...");
        let stats = download_wallpapers(&config, &RealSystem, ctx.observer)?;

        if stats.success > 0 || stats.failed > 0 {
            ctx.record_action(format!(
                "📊  Download mission results: {} successes, {} failures",
                stats.success, stats.failed
            ));

            if stats.success > 0 {
                ctx.record_action("✨  Your desktop now has retro-futuristic wallpapers!");
            }
        }
    }

    Ok(PhaseResult::Success)
}
