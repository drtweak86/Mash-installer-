//! BBC/UNIX Retro-Futuristic Wallpaper Downloader
//!
//! A Rust implementation for downloading 6000+ retro-futuristic wallpapers
//! from 8 categories: retro, games, anime, dc, marvel, judge_dredd, star_wars, cyberpunk.

use wallpaper_downloader::{Config, Downloader};
use std::process;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Parse configuration
    let config = match Config::parse() {
        config => config,
    };

    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("❌ Configuration error: {}", e);
        process::exit(1);
    }

    // Display banner if not in first-boot mode
    if !config.first_boot {
        println!(
            "{}",
            r#"
╔════════════════════════════════════════════════════════════╗
║  🖥️  BBC/UNIX RETRO WALLPAPER DOWNLOADER - 8 CATEGORIES  ║
║  • 1000 Retro Computing  • 1000 Video Games              ║
║  • 625 Anime             • 625 DC Comics                  ║
║  • 625 Marvel            • 562 Judge Dredd/Lobo          ║
║  • 562 Star Wars         • 1000 Cyberpunk                 ║
║  Total: 5999 retro-futuristic wallpapers!               ║
╚════════════════════════════════════════════════════════════╝
"#
        );
    }

    // Create downloader
    let mut downloader = match Downloader::new(&config) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ Failed to create downloader: {}", e);
            process::exit(1);
        }
    };

    // Download wallpapers
    match downloader.download_all().await {
        Ok(_) => {
            if !config.first_boot {
                println!("\n🎉 Wallpaper download complete!");
                let output_dir = config.output_dir.clone().unwrap_or_else(wallpaper_downloader::types::default_output_dir);
                println!("📁 Find your wallpapers in: {}", output_dir.display());
                println!("🎨 Use 'feh --bg-scale --randomize ~/Pictures/RetroWallpapers/*/*' to set random wallpaper");
            }
            process::exit(0);
        }
        Err(e) => {
            eprintln!("\n❌ Download failed: {}", e);
            process::exit(1);
        }
    }
}
