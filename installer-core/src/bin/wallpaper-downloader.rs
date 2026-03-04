//! BBC/UNIX Retro-Futuristic Wallpaper Downloader
//!
//! A Rust implementation for downloading 6000+ retro-futuristic wallpapers
//! from 8 categories: retro, games, anime, dc, marvel, judge_dredd, star_wars, cyberpunk.

use std::path::PathBuf;
use std::process;

use anyhow::Result;
use clap::Parser;
use installer_core::{harvest_wallpapers, HarvestConfig, PhaseEvent, PhaseObserver};

/// CLI arguments for the wallpaper downloader
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output directory for wallpapers
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Specific category to download (default: all)
    #[arg(short, long)]
    category: Option<String>,

    /// Number of concurrent downloads
    #[arg(short, long, default_value_t = 3)]
    workers: usize,
}

struct ConsoleObserver;

impl PhaseObserver for ConsoleObserver {
    fn on_event(&mut self, event: PhaseEvent) {
        match event {
            PhaseEvent::Warning { message } => {
                println!("{}", message);
            }
            PhaseEvent::Started { phase, .. } => {
                println!("🚀 Starting phase: {}", phase);
            }
            PhaseEvent::Completed { phase, .. } => {
                println!("✅ Completed phase: {}", phase);
            }
            PhaseEvent::Failed { phase, error, .. } => {
                eprintln!("❌ Phase {} failed: {}", phase, error);
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    let args = Args::parse();

    // Determine output directory
    let output_dir = args.output.unwrap_or_else(|| {
        dirs::picture_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("RetroWallpapers")
    });

    println!("🎨 MASH Retro-Futuristic Wallpaper Downloader");
    println!("📂 Target: {}", output_dir.display());
    println!("⚙️  Workers: {}", args.workers);
    println!("-------------------------------------------");

    // Create harvest config
    let config = HarvestConfig {
        dest: output_dir,
        workers: args.workers,
        ..Default::default()
    };

    // Run harvester
    let mut observer = ConsoleObserver;
    match harvest_wallpapers(config, &mut observer) {
        Ok(_) => {
            println!("\n✨ Mission accomplished! All wallpapers have been harvested.");
            process::exit(0);
        }
        Err(e) => {
            eprintln!("\n❌ Download failed: {}", e);
            process::exit(1);
        }
    }
}
