//! Wallpaper download logic

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use sha2::{Digest, Sha256};

use crate::api::ApiClient;
use crate::config::Config;
use crate::error::{DownloadError, Result};
use crate::types::{Category, DownloadStats};

/// Wallpaper downloader
#[derive(Debug)]
pub struct Downloader {
    config: Config,
    api_client: ApiClient,
    downloaded_hashes: Arc<Mutex<HashSet<String>>>,
    url_cache: Arc<Mutex<HashSet<String>>>,
    stats: Arc<Mutex<DownloadStats>>,
}

impl Downloader {
    /// Create a new downloader
    pub fn new(config: &Config) -> Result<Self> {
        let api_client = ApiClient::new(config.get_api_key());

        let downloader = Self {
            config: config.clone(),
            api_client,
            downloaded_hashes: Arc::new(Mutex::new(HashSet::new())),
            url_cache: Arc::new(Mutex::new(HashSet::new())),
            stats: Arc::new(Mutex::new(DownloadStats::default())),
        };

        // Load existing hashes
        downloader.load_existing_hashes()?;

        Ok(downloader)
    }

    /// Load existing file hashes to avoid re-downloading
    fn load_existing_hashes(&self) -> Result<()> {
        if !self.config.first_boot {
            log::info!("🔍 Scanning for existing files...");
        }

        let output_dir = self
            .config
            .output_dir
            .clone()
            .unwrap_or_else(crate::types::default_output_dir);
        let mut count = 0;

        let mut hashes = self.downloaded_hashes.lock().unwrap();

        // Get all category directories
        for category in &*crate::types::CATEGORIES {
            let category_dir = output_dir.join(&category.name);
            if category_dir.exists() && category_dir.is_dir() {
                let entries = std::fs::read_dir(&category_dir)?;
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        if let Ok(file_hash) = Self::compute_file_hash_static(&path) {
                            hashes.insert(file_hash);
                            count += 1;
                        }
                    }
                }
            }
        }

        if count > 0 && !self.config.first_boot {
            log::info!(
                "✓ Loaded {} existing files ({} unique)",
                count,
                hashes.len()
            );
        }

        Ok(())
    }

    /// Compute SHA256 hash of a file
    fn compute_file_hash_static(filepath: &Path) -> Result<String> {
        let mut file = std::fs::File::open(filepath)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Create output directories
    async fn create_directories(&self) -> Result<()> {
        let output_dir = self
            .config
            .output_dir
            .clone()
            .unwrap_or_else(crate::types::default_output_dir);
        for category in &*crate::types::CATEGORIES {
            let category_dir = output_dir.join(&category.name);
            tokio::fs::create_dir_all(&category_dir).await?;
        }

        if !self.config.first_boot {
            log::info!("📁 Created directories in {}", output_dir.display());
        }

        Ok(())
    }

    /// Download all wallpapers
    pub async fn download_all(&mut self) -> Result<()> {
        // Create directories
        self.create_directories().await?;

        // Log start message
        let output_dir = self
            .config
            .output_dir
            .clone()
            .unwrap_or_else(crate::types::default_output_dir);
        let total_target = std::cmp::min(self.config.limit, 6000);
        if !self.config.first_boot {
            log::info!("🚀 Starting download of {} wallpapers...", total_target);
            log::info!("📥 Category: {}", self.config.category);
            log::info!("📁 Destination: {}", output_dir.display());
        }

        if self.config.category == "all" {
            // Download all categories
            let mut results = std::collections::HashMap::new();

            for category in &*crate::types::CATEGORIES {
                let downloaded = self.download_category(category).await?;
                results.insert(category.name.clone(), downloaded);
            }

            // Print summary per category
            if !self.config.first_boot {
                println!("\n📊 Category Summary:");
                for (category, count) in results {
                    let display_name = crate::types::CATEGORIES
                        .iter()
                        .find(|c| c.name == category)
                        .map(|c| c.display_name.as_str())
                        .unwrap_or(&category);
                    println!("  • {}: {} images", display_name, count);
                }
            }
        } else {
            // Download specific category
            let category_name = self.config.category.clone();
            let category = crate::types::CATEGORIES
                .iter()
                .find(|c| c.name == category_name)
                .ok_or_else(|| {
                    DownloadError::Config(format!("Unknown category: {}", category_name))
                })?;

            self.download_category(category).await?;
        }

        // Print final summary
        if !self.config.first_boot {
            let stats = self.stats.lock().unwrap();
            println!("\n✨ Download complete!");
            println!("  ✅ Success: {}", stats.success_count);
            println!("  ❌ Failed:  {}", stats.fail_count);
            log::info!("📁 Location: {}", output_dir.display());
        }

        // Setup wallpapers
        self.setup_wallpapers().await?;

        Ok(())
    }

    /// Download a specific category
    async fn download_category(&self, category: &Category) -> Result<usize> {
        let images = self
            .api_client
            .get_category_wallpapers(&category.name, &category.queries, category.count)
            .await?;

        let mut downloaded = 0;
        let mut tasks = JoinSet::new();
        let semaphore = Arc::new(Semaphore::new(self.config.max_workers));

        // Limit to the requested count
        let images_to_download = images.into_iter().take(category.count).collect::<Vec<_>>();

        for (i, wallpaper) in images_to_download.iter().enumerate() {
            let permit = semaphore.clone().acquire_owned().await?;
            let url = wallpaper.url.clone();
            let output_dir = self
                .config
                .output_dir
                .clone()
                .unwrap_or_else(crate::types::default_output_dir);
            let filename = format!("{}_{:04}.jpg", category.name, i);
            let filepath = output_dir.join(&category.name).join(filename);

            // Clone shared state for the async task
            let downloaded_hashes = self.downloaded_hashes.clone();
            let url_cache = self.url_cache.clone();
            let reqwest_client = self.api_client.get_client().clone();
            let timeout = self.config.timeout;

            tasks.spawn(async move {
                let _permit = permit;
                Self::download_image_static(
                    reqwest_client,
                    downloaded_hashes,
                    url_cache,
                    url,
                    filepath,
                    timeout,
                )
                .await
            });
        }

        // Wait for all downloads to complete
        while let Some(result) = tasks.join_next().await {
            if let Ok(download_result) = result {
                if download_result {
                    downloaded += 1;
                    if downloaded % 10 == 0 && !self.config.first_boot {
                        log::info!("  ✓ Downloaded {} images...", downloaded);
                    }
                    let mut stats = self.stats.lock().unwrap();
                    stats.success_count += 1;
                } else {
                    let mut stats = self.stats.lock().unwrap();
                    stats.fail_count += 1;
                }
            }
        }

        Ok(downloaded)
    }

    /// Static helper for individual download to avoid self sharing issues
    async fn download_image_static(
        client: reqwest::Client,
        downloaded_hashes: Arc<Mutex<HashSet<String>>>,
        url_cache: Arc<Mutex<HashSet<String>>>,
        url: String,
        filepath: PathBuf,
        timeout: u64,
    ) -> bool {
        // Skip if URL already cached
        {
            if url_cache.lock().unwrap().contains(&url) {
                return false;
            }
        }

        // Skip if file already exists
        if filepath.exists() {
            return false;
        }

        // Create parent directory if needed
        if let Some(parent) = filepath.parent() {
            if !parent.exists() {
                if let Err(e) = tokio::fs::create_dir_all(parent).await {
                    log::error!("Failed to create directory {}: {}", parent.display(), e);
                    return false;
                }
            }
        }

        // Download the image
        let response = match client
            .get(&url)
            .timeout(std::time::Duration::from_secs(timeout))
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                log::error!("✗ Error downloading {}: {}", url, e);
                return false;
            }
        };

        if !response.status().is_success() {
            log::error!("✗ HTTP error {} for {}", response.status(), url);
            return false;
        }

        // Read response bytes
        let bytes = match response.bytes().await {
            Ok(b) => b,
            Err(e) => {
                log::error!("✗ Error reading response for {}: {}", url, e);
                return false;
            }
        };

        // Write to temporary file first
        let temp_path = filepath.with_extension("tmp");
        let mut temp_file = match tokio::fs::File::create(&temp_path).await {
            Ok(f) => f,
            Err(e) => {
                log::error!("✗ Error creating temp file {}: {}", temp_path.display(), e);
                return false;
            }
        };

        if let Err(e) = temp_file.write_all(&bytes).await {
            log::error!("✗ Error writing temp file {}: {}", temp_path.display(), e);
            return false;
        }

        // Compute hash of downloaded file
        let file_hash = match Self::compute_file_hash_static(&temp_path) {
            Ok(h) => h,
            Err(e) => {
                log::error!("✗ Error hashing {}: {}", temp_path.display(), e);
                return false;
            }
        };

        // Check if already downloaded
        let exists = {
            let mut hashes = downloaded_hashes.lock().unwrap();
            if hashes.contains(&file_hash) {
                true
            } else {
                hashes.insert(file_hash);
                false
            }
        };

        if exists {
            let _ = tokio::fs::remove_file(&temp_path).await;
            url_cache.lock().unwrap().insert(url);
            return false;
        }

        // Rename temp file to final location
        if let Err(e) = tokio::fs::rename(&temp_path, &filepath).await {
            log::error!(
                "✗ Error renaming {} to {}: {}",
                temp_path.display(),
                filepath.display(),
                e
            );
            let _ = tokio::fs::remove_file(&temp_path).await;
            return false;
        }

        true
    }

    /// Setup wallpapers for i3 and GNOME
    async fn setup_wallpapers(&self) -> Result<()> {
        // For i3/feh
        let config_file = dirs::home_dir()
            .ok_or_else(|| DownloadError::Config("Could not determine home directory".to_string()))?
            .join(".config")
            .join("i3")
            .join("config");

        if config_file.exists() {
            let mut content = String::new();
            if let Ok(existing) = tokio::fs::read_to_string(&config_file).await {
                content.push_str(&existing);
            }

            content.push_str("\n# Auto-generated wallpaper setting\n");
            content.push('e');
            content.push_str("xec_always feh --bg-scale --randomize ");

            let output_dir = self
                .config
                .output_dir
                .clone()
                .unwrap_or_else(crate::types::default_output_dir);
            for category in &*crate::types::CATEGORIES {
                content.push_str(&format!("{}/* ", output_dir.join(&category.name).display()));
            }
            content.push('\n');

            if let Err(e) = tokio::fs::write(&config_file, content).await {
                log::warn!("⚠️  Could not update i3 config: {}", e);
            } else {
                log::info!("🎨 Configured i3 wallpaper directory");
            }
        }

        // For GNOME
        #[cfg(target_os = "linux")]
        {
            let output_dir = self
                .config
                .output_dir
                .clone()
                .unwrap_or_else(crate::types::default_output_dir);
            if let Err(e) = std::process::Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.background",
                    "picture-uri",
                    &format!(
                        "file://{}",
                        output_dir.join("retro").join("retro_0001.jpg").display()
                    ),
                ])
                .status()
            {
                log::warn!("⚠️  Could not auto-configure GNOME wallpapers: {}", e);
            } else {
                log::info!("🎨 Configured GNOME wallpaper directory");
            }
        }

        Ok(())
    }
}
