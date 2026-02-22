use anyhow::Result;
use tokio::runtime::Runtime;

use crate::context::PhaseContext;
use crate::system::RealSystem;
use crate::wallpaper::{download_wallpapers, WallpaperConfig};

/// Wallpaper installation phase
pub fn install_phase(ctx: &mut PhaseContext<'_>) -> Result<()> {
    // Whimsical starting message
    ctx.record_action("🚀  Launching plasma ore collection mission...");
    ctx.record_action("🎨  Painting your desktop with neon runes...");

    let config = WallpaperConfig::default();

    // Check for API keys
    if config.api_keys.wallhaven.is_none()
        && config.api_keys.pexels.is_none()
        && config.api_keys.pixabay.is_none()
    {
        ctx.record_warning("🔑  No API keys configured for wallpaper sources.");
        ctx.record_warning("📋  Please obtain API keys from:");
        ctx.record_warning("🌐  Wallhaven: https://wallhaven.cc/settings/account");
        ctx.record_warning("🌐  Pexels: https://www.pexels.com/api/");
        ctx.record_warning("🌐  Pixabay: https://pixabay.com/api/docs/");
        ctx.record_warning("💡  Wallpapers will be skipped without API keys.");
        return Ok(());
    }

    // Create tokio runtime to run async code
    let rt = Runtime::new()?;

    // Download wallpapers
    let stats = rt.block_on(async { download_wallpapers(&config, &RealSystem, ctx).await })?;

    ctx.record_action(format!(
        "📊  Download complete: {} success, {} failed",
        stats.success, stats.failed
    ));

    if stats.success > 0 {
        ctx.record_action("✨  Your desktop now has retro-futuristic wallpapers!");
    }

    Ok(())
}
