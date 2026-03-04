use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::debug;

use crate::model::phase::{PhaseEvent, PhaseObserver};
use crate::wallpaper::config::HarvestConfig;

// --- State DB ---
#[derive(Debug)]
pub struct StateDB {
    conn: std::sync::Mutex<Connection>,
}

impl StateDB {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path).context("opening state database")?;

        // Initialize schema
        conn.execute(
            "CREATE TABLE IF NOT EXISTS urls (
                url TEXT PRIMARY KEY,
                status TEXT DEFAULT 'pending',
                attempts INTEGER DEFAULT 0,
                last_attempt INTEGER
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS hashes (
                hash TEXT PRIMARY KEY,
                path TEXT,
                url TEXT
            )",
            [],
        )?;

        Ok(Self {
            conn: std::sync::Mutex::new(conn),
        })
    }

    pub fn add_urls(&self, urls: &[String]) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        {
            let mut stmt = tx.prepare_cached("INSERT OR IGNORE INTO urls (url) VALUES (?)")?;
            for url in urls {
                stmt.execute(params![url])?;
            }
        }

        tx.commit().context("committing urls")?;
        Ok(())
    }

    pub fn pending_urls(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT url FROM urls WHERE status = 'pending' LIMIT 500")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut urls = Vec::new();
        for url in rows {
            urls.push(url?);
        }
        Ok(urls)
    }

    pub fn mark_done(&self, url: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE urls SET status = 'done' WHERE url = ?",
            params![url],
        )?;
        Ok(())
    }

    pub fn mark_failed(&self, url: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE urls SET status = 'failed', attempts = attempts + 1, last_attempt = ? WHERE url = ?",
            params![
                SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs() as i64,
                url
            ],
        )?;
        Ok(())
    }

    pub fn add_hash(&self, hash: &str, path: &Path, url: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let res = conn.execute(
            "INSERT OR IGNORE INTO hashes (hash, path, url) VALUES (?, ?, ?)",
            params![hash, path.to_string_lossy(), url],
        );

        match res {
            Ok(1) => Ok(true),
            Ok(_) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    pub fn hash_count(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM hashes", [], |row| row.get(0))?;
        Ok(count as usize)
    }
}

// ── Downloader ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Downloader {
    config: HarvestConfig,
    db: Arc<StateDB>,
    agent: ureq::Agent,
    pub stop_flag: Arc<AtomicBool>,
}

impl Downloader {
    pub fn new(config: HarvestConfig, db: Arc<StateDB>) -> Self {
        Self {
            config,
            db,
            agent: ureq::AgentBuilder::new()
                .user_agent("MASH-Wallpaper-Harvester/1.0")
                .timeout(Duration::from_secs(30))
                .build(),
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn download(&self, url: String) -> Result<bool> {
        if self.stop_flag.load(Ordering::SeqCst) {
            return Ok(false);
        }

        let file_name = url
            .split('/')
            .next_back()
            .unwrap_or("unknown.jpg")
            .split('?')
            .next()
            .unwrap_or("unknown.jpg");

        let dest_path = self.config.dest.join(file_name);
        let tmp_path = dest_path.with_extension("tmp");

        // 1. Download to temp file
        let response = match self.agent.get(&url).call() {
            Ok(res) => res,
            Err(e) => {
                debug!("Request failed for {}: {}", url, e);
                self.db.mark_failed(&url)?;
                return Ok(false);
            }
        };

        if response.status() != 200 {
            self.db.mark_failed(&url)?;
            return Ok(false);
        }

        let mut file = std::fs::File::create(&tmp_path)?;
        let mut hasher = Sha256::new();
        let mut total_bytes = 0;
        let mut header_bytes = Vec::new();

        let mut reader = response.into_reader();
        let mut buffer = [0; 8192];
        loop {
            if self.stop_flag.load(Ordering::SeqCst) {
                drop(file);
                std::fs::remove_file(&tmp_path).ok();
                return Ok(false);
            }

            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }

            let chunk = &buffer[..n];
            file.write_all(chunk)?;
            total_bytes += n as u64;

            // Capture header for validation
            if header_bytes.len() < 512 {
                let to_copy = std::cmp::min(512 - header_bytes.len(), n);
                header_bytes.extend_from_slice(&chunk[..to_copy]);
            }

            hasher.update(chunk);
        }

        file.flush()?;
        drop(file);

        // 2. Validate (basic check if it's an image)
        if total_bytes < 1024 || !is_image_header(&header_bytes) {
            std::fs::remove_file(&tmp_path).ok();
            self.db.mark_failed(&url)?;
            return Ok(false);
        }

        // 3. Deduplicate
        let hash = format!("{:x}", hasher.finalize());
        if self.db.add_hash(&hash, &dest_path, &url)? {
            std::fs::rename(&tmp_path, &dest_path)?;
            self.db.mark_done(&url)?;
            Ok(true)
        } else {
            // Duplicate
            std::fs::remove_file(&tmp_path).ok();
            self.db.mark_done(&url)?;
            Ok(false)
        }
    }
}

fn is_image_header(header: &[u8]) -> bool {
    // JPEG
    if header.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return true;
    }
    // PNG
    if header.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        return true;
    }
    // WEBP
    if header.len() >= 12 && &header[0..4] == b"RIFF" && &header[8..12] == b"WEBP" {
        return true;
    }
    false
}

// ── Source Manager ───────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct SourceManager {
    config: HarvestConfig,
    #[allow(dead_code)] // Kept for future expansion
    db: Arc<StateDB>,
    last_request: std::sync::Mutex<f64>,
}

impl SourceManager {
    pub fn new(config: HarvestConfig, db: Arc<StateDB>) -> Self {
        Self {
            config,
            db,
            last_request: std::sync::Mutex::new(0.0),
        }
    }

    fn _throttle(&self) {
        let mut last = self.last_request.lock().unwrap();
        let elapsed = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
            - *last;

        if elapsed < self.config.rate_limit.as_secs_f64() {
            std::thread::sleep(Duration::from_secs_f64(
                self.config.rate_limit.as_secs_f64() - elapsed,
            ));
        }

        *last = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
    }

    pub fn get_urls(&self) -> Result<Vec<String>> {
        let mut urls = Vec::new();

        // Wallhaven queries
        let wallhaven_queries = vec![
            "mass effect character",
            "cyberpunk 2077 character art",
            "halo spartan",
            "destiny hunter warlock titan",
            "metroid samus aran",
            "dead space isaac clarke",
            "doom slayer",
            "doom eternal",
            "half life gordon freeman",
            "bioshock big daddy",
            "alien isolation xenomorph",
            "system shock shodan",
            "prey 2017 typhon",
            "outriders character",
            "returnal selene",
            "control alan wake",
            "akira kaneda tetsuo",
            "ghost in the shell motoko",
            "berserk guts griffith",
            "neon genesis evangelion unit01",
            "cowboy bebop spike",
            "trigun vash stampede",
            "fullmetal alchemist edward",
            "attack on titan eren levi",
            "one punch man saitama",
            "demon slayer tanjiro",
            "jojo bizarre adventure",
            "vinland saga thorfinn",
            "berserker fate",
            "madoka magica",
            "chainsaw man denji",
            "hunter x hunter killua gon",
            "batman dark knight",
            "batman arkham knight",
            "batman comic art",
            "batman beyond",
            "batman versus joker",
            "batman hush",
            "joker dc comics art",
            "joker joaquin phoenix",
            "joker heath ledger",
            "joker batman villain",
            "harley quinn comics",
            "harley quinn suicide squad",
            "harley quinn animated",
            "harley quinn birds of prey",
            "punisher frank castle",
            "punisher skull marvel",
            "punisher comic art",
            "punisher netflix",
            "deadpool comic art",
            "deadpool marvel",
            "deadpool wolverine",
            "deadpool mercenary",
            "star wars darth vader",
            "star wars mandalorian",
            "star wars clone trooper",
            "star wars sith lord",
            "star wars boba fett",
            "star wars stormtrooper",
            "star wars kylo ren",
            "star wars darth maul",
            "judge dredd helmet",
            "judge dredd mega city",
            "dredd 2012 karl urban",
            "judge dredd comic",
            "lobo dc comics",
            "lobo main man",
            "frank miller sin city",
            "frank miller batman",
            "frank miller 300 spartan",
            "frank miller daredevil",
            "sin city noir art",
            "miller comic noir",
        ];

        for query in wallhaven_queries {
            self._throttle();
            let params = [
                ("q", query),
                ("categories", "111"),
                ("purity", "100"),
                ("sorting", "relevance"),
                ("order", "desc"),
                ("page", "1"),
                (
                    "atleast",
                    &format!("{}x{}", self.config.min_width, self.config.min_height),
                ),
            ];

            let url = format!(
                "https://wallhaven.cc/api/v1/search?{}",
                serde_urlencoded::to_string(params)?
            );

            let response = match ureq::get(&url).call() {
                Ok(res) => res,
                Err(_) => continue,
            };

            if response.status() == 200 {
                let json: serde_json::Value = response.into_json()?;
                if let Some(data) = json.get("data") {
                    if let Some(items) = data.as_array() {
                        for item in items {
                            if let Some(path) = item.get("path").and_then(|v| v.as_str()) {
                                urls.push(path.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(urls)
    }
}

// ── Harvester ────────────────────────────────────────────────────────────────
#[derive(Debug)]
pub struct WallpaperHarvester {
    config: HarvestConfig,
    db: Arc<StateDB>,
    downloader: Arc<Downloader>,
    stop_flag: Arc<AtomicBool>,
}

impl WallpaperHarvester {
    pub fn new(config: HarvestConfig) -> Result<Self> {
        let state_dir = config.dest.join(".state");
        std::fs::create_dir_all(&state_dir)?;

        let db = Arc::new(StateDB::new(&state_dir.join("state.db"))?);
        let downloader = Arc::new(Downloader::new(config.clone(), db.clone()));

        Ok(Self {
            config,
            db,
            downloader: downloader.clone(),
            stop_flag: downloader.stop_flag.clone(),
        })
    }

    pub fn run(&self, observer: &mut dyn PhaseObserver) -> Result<()> {
        observer.on_event(PhaseEvent::Warning {
            message: "🌐  Starting wallpaper harvest operation...".to_string(),
        });

        // Get URLs from sources
        let source_manager = SourceManager::new(self.config.clone(), self.db.clone());

        let urls = source_manager.get_urls()?;
        observer.on_event(PhaseEvent::Warning {
            message: format!("🔍  Found {} potential wallpaper URLs", urls.len()),
        });

        // Add URLs to database
        self.db.add_urls(&urls)?;

        let pending_urls = self.db.pending_urls()?;
        observer.on_event(PhaseEvent::Warning {
            message: format!("📋  Processing {} pending URLs", pending_urls.len()),
        });

        // Wait for all downloads to complete
        let mut success_count = 0;
        let mut fail_count = 0;

        for url in pending_urls {
            if self.stop_flag.load(Ordering::SeqCst) {
                break;
            }

            match self.downloader.download(url) {
                Ok(saved) => {
                    if saved {
                        success_count += 1;
                    }
                }
                Err(e) => {
                    observer.on_event(PhaseEvent::Warning {
                        message: format!("⚠️  Download failed: {}", e),
                    });
                    fail_count += 1;
                }
            }
        }

        let unique_count = self.db.hash_count()?;
        observer.on_event(PhaseEvent::Warning {
            message: format!(
                "📊  Harvest complete: {} unique wallpapers, {} success, {} failed",
                unique_count, success_count, fail_count
            ),
        });

        Ok(())
    }

    #[allow(dead_code)] // Kept for future expansion
    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::SeqCst);
    }
}

// ── Public API ──────────────────────────────────────────────────────────────
#[allow(dead_code)] // Kept for future expansion
pub fn harvest_wallpapers(config: HarvestConfig, observer: &mut dyn PhaseObserver) -> Result<()> {
    let harvester = WallpaperHarvester::new(config)?;
    harvester.run(observer)
}
