use anyhow::Result;
use tokio::runtime::Runtime;

use crate::context::PhaseContext;
use crate::system::RealSystem;
use crate::wallpaper::harvest::WallpaperHarvester;
use crate::wallpaper::{download_wallpapers, HarvestConfig, WallpaperConfig};
use crate::PhaseResult;

/// Create a basic logger for wallpaper operations
fn create_wallpaper_logger() -> slog::Logger {
    use slog::{Drain, Logger};

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    Logger::root(drain, slog::o!("module" => "wallpaper"))
}

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

    // Create tokio runtime to run async code
    let rt = Runtime::new()?;

    // First, try the harvest method (no API keys required)
    ctx.record_action("🌾  Attempting wallpaper harvest (no API keys required)...");

    let harvest_config = HarvestConfig::default();
    let logger = create_wallpaper_logger();
    let harvester = WallpaperHarvester::new(harvest_config, logger)?;

    let harvest_result = rt.block_on(harvester.run(ctx));

    match harvest_result {
        Ok(_) => {
            ctx.record_action("🎉  Wallpaper harvest completed successfully!");
        }
        Err(e) => {
            ctx.record_warning(format!("⚠️  Wallpaper harvest failed: {}", e));
            ctx.record_action("🔄  Falling back to traditional API-based download...");

            // Fall back to traditional download if harvest fails
            if !has_api_keys {
                ctx.record_warning("🔑  No API keys configured for wallpaper sources.");
                ctx.record_warning("📋  Please obtain API keys from:");
                ctx.record_warning("🌐  Wallhaven: https://wallhaven.cc/settings/account");
                ctx.record_warning("🌐  Pexels: https://www.pexels.com/api/");
                ctx.record_warning("🌐  Pixabay: https://pixabay.com/api/docs/");
                ctx.record_warning("💡  Traditional wallpaper download skipped without API keys.");
                return Ok(PhaseResult::Success);
            }

            // Traditional download
            let stats =
                rt.block_on(async { download_wallpapers(&config, &RealSystem, ctx).await })?;

            ctx.record_action(format!(
                "📊  Download complete: {} success, {} failed",
                stats.success, stats.failed
            ));

            if stats.success > 0 {
                ctx.record_action("✨  Your desktop now has retro-futuristic wallpapers!");
            }
        }
    }

    Ok(PhaseResult::Success)
}
