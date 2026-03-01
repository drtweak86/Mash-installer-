//! Wallpaper Harvest — Rust transmogrification of mash-wallpaper-harvest.py
//!
//! Features:
//! - SQLite state tracking (resume, dedup, retry)
//! - SHA-256 fingerprint deduplication
//! - Concurrent downloads with rate limiting
//! - Streaming downloads (constant memory)
//! - Resolution validation via image header parsing
//! - Exponential backoff retries
//! - Pi 4B friendly (low resource usage)

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use futures_util::stream::StreamExt;
use reqwest::Client;
use rusqlite::{Connection, OptionalExtension};
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::context::PhaseContext;

// ── Configuration ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct HarvestConfig {
    pub dest: PathBuf,
    pub workers: usize,
    #[allow(dead_code)] // Kept for future expansion
    pub target: usize,
    pub min_width: u32,
    pub min_height: u32,
    pub min_size_kb: u64,
    pub max_size_mb: u64,
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
    pub retry_max: usize,
    pub retry_delay: Duration,
    pub rate_limit: Duration,
    #[allow(dead_code)] // Kept for future expansion
    pub chunk_size: usize,
    pub fingerprint_bytes: usize,
}

impl Default for HarvestConfig {
    fn default() -> Self {
        Self {
            dest: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("wallpapers")
                .join("mash"),
            workers: 4, // Keep ≤ 4 on Pi 4B
            target: 5000,
            min_width: 1280,
            min_height: 720,
            min_size_kb: 100,
            max_size_mb: 25,
            connect_timeout: Duration::from_secs(15),
            read_timeout: Duration::from_secs(60),
            retry_max: 3,
            retry_delay: Duration::from_secs(2),
            rate_limit: Duration::from_secs_f32(1.0),
            chunk_size: 65536,        // 64 KB
            fingerprint_bytes: 65536, // 64 KB for SHA-256
        }
    }
}

// ── SQLite State Store ──────────────────────────────────────────────────────
#[derive(Debug)]
pub struct StateDB {
    conn: RwLock<Connection>,
}

unsafe impl Send for StateDB {}
unsafe impl Sync for StateDB {}

#[allow(dead_code, clippy::readonly_write_lock)] // Methods kept for future expansion, SQLite uses write locks
impl StateDB {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)
            .with_context(|| format!("Failed to open database at {:?}", db_path))?;

        // Create schema
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS urls (
                url       TEXT PRIMARY KEY,
                status    TEXT NOT NULL DEFAULT 'pending',
                attempts  INTEGER NOT NULL DEFAULT 0,
                filename  TEXT,
                added_at  REAL NOT NULL DEFAULT (unixepoch('now')),
                updated_at REAL NOT NULL DEFAULT (unixepoch('now'))
            );
            CREATE TABLE IF NOT EXISTS hashes (
                fingerprint TEXT PRIMARY KEY,
                filename    TEXT NOT NULL,
                added_at    REAL NOT NULL DEFAULT (unixepoch('now'))
            );
            CREATE TABLE IF NOT EXISTS meta (
                key   TEXT PRIMARY KEY,
                value TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_urls_status ON urls(status);
            ",
        )?;

        Ok(Self {
            conn: RwLock::new(conn),
        })
    }

    // URL helpers
    pub fn add_urls(&self, urls: &[String]) -> Result<usize> {
        let mut conn = self.conn.write().unwrap();
        let tx = conn.transaction()?;
        let mut stmt = tx.prepare("INSERT OR IGNORE INTO urls (url) VALUES (?1)")?;

        let mut count = 0;
        for url in urls {
            stmt.execute([url])?;
            count += 1;
        }

        stmt.finalize()?;
        tx.commit()?;
        Ok(count)
    }

    pub fn url_status(&self, url: &str) -> Result<Option<String>> {
        let conn = self.conn.read().unwrap();
        Ok(conn
            .query_row("SELECT status FROM urls WHERE url=?1", [url], |row| {
                row.get(0)
            })
            .optional()?)
    }

    pub fn mark_done(&self, url: &str, filename: &str) -> Result<()> {
        let conn = self.conn.write().unwrap();
        conn.execute(
            "UPDATE urls SET status='done', filename=?1, attempts=attempts+1, updated_at=unixepoch('now') WHERE url=?2",
            (filename, url),
        )?;
        Ok(())
    }

    pub fn mark_failed(&self, url: &str) -> Result<()> {
        let conn = self.conn.write().unwrap();
        conn.execute(
            "UPDATE urls SET status='failed', attempts=attempts+1, updated_at=unixepoch('now') WHERE url=?1",
            [url],
        )?;
        Ok(())
    }

    pub fn mark_skip(&self, url: &str) -> Result<()> {
        let conn = self.conn.write().unwrap();
        conn.execute(
            "UPDATE urls SET status='skip', updated_at=unixepoch('now') WHERE url=?1",
            [url],
        )?;
        Ok(())
    }

    pub fn pending_urls(&self) -> Result<Vec<String>> {
        let conn = self.conn.read().unwrap();
        let mut stmt =
            conn.prepare("SELECT url FROM urls WHERE status='pending' ORDER BY added_at")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut urls = Vec::new();
        for row in rows {
            urls.push(row?);
        }

        Ok(urls)
    }

    pub fn counts(&self) -> Result<std::collections::HashMap<String, i64>> {
        let conn = self.conn.read().unwrap();
        let mut stmt = conn.prepare("SELECT status, COUNT(*) FROM urls GROUP BY status")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;

        let mut counts = std::collections::HashMap::new();
        for row in rows {
            let (status, count) = row?;
            counts.insert(status, count);
        }

        Ok(counts)
    }

    // Hash helpers
    pub fn has_hash(&self, fingerprint: &str) -> Result<bool> {
        let conn = self.conn.read().unwrap();
        let exists: Option<bool> = conn
            .query_row(
                "SELECT 1 FROM hashes WHERE fingerprint=?1",
                [fingerprint],
                |row| row.get(0),
            )
            .optional()?;
        Ok(exists.unwrap_or(false))
    }

    pub fn add_hash(&self, fingerprint: &str, filename: &str) -> Result<()> {
        let conn = self.conn.write().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO hashes (fingerprint, filename) VALUES (?1, ?2)",
            (fingerprint, filename),
        )?;
        Ok(())
    }

    pub fn hash_count(&self) -> Result<i64> {
        let conn = self.conn.read().unwrap();
        Ok(conn.query_row("SELECT COUNT(*) FROM hashes", [], |row| row.get(0))?)
    }

    // Meta helpers
    pub fn set_meta(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.write().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO meta(key,value) VALUES(?1,?2)",
            (key, value),
        )?;
        Ok(())
    }

    pub fn get_meta(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.read().unwrap();
        Ok(conn
            .query_row("SELECT value FROM meta WHERE key=?1", [key], |row| {
                row.get(0)
            })
            .optional()?)
    }

    pub fn reset(&self) -> Result<()> {
        let conn = self.conn.write().unwrap();
        conn.execute_batch("DELETE FROM urls; DELETE FROM hashes; DELETE FROM meta;")?;
        Ok(())
    }
}

// ── Image Header Parser ────────────────────────────────────────────────────
#[derive(Debug)]
pub struct ImageInfo;

impl ImageInfo {
    pub fn dimensions(data: &[u8]) -> Option<(u32, u32)> {
        if data.len() < 24 {
            return None;
        }

        // PNG: 8-byte sig + IHDR chunk
        if data.starts_with(b"\x89PNG\r\n\x1a\n") && data.len() >= 24 {
            let w = u32::from_be_bytes(data[16..20].try_into().ok()?);
            let h = u32::from_be_bytes(data[20..24].try_into().ok()?);
            return Some((w, h));
        }

        // JPEG: scan for SOF markers
        if data.starts_with(b"\xff\xd8") {
            return Self::_jpeg_dims(data);
        }

        // WebP: RIFF....WEBPVP8
        if data.starts_with(b"RIFF") && data.get(8..12) == Some(b"WEBP".as_slice()) {
            return Self::_webp_dims(data);
        }

        None
    }

    fn _jpeg_dims(data: &[u8]) -> Option<(u32, u32)> {
        let mut i = 2;
        while i < data.len().saturating_sub(8) {
            if data[i] != 0xFF {
                break;
            }
            let marker = data.get(i + 1).copied()?;

            // SOF markers: C0-C3, C5-C7, C9-CB, CD-CF
            if ((0xC0..=0xC3).contains(&marker)
                || (0xC5..=0xC7).contains(&marker)
                || (0xC9..=0xCB).contains(&marker)
                || (0xCD..=0xCF).contains(&marker))
                && data.len() > i + 9
            {
                let h = u16::from_be_bytes(data[i + 5..i + 7].try_into().ok()?) as u32;
                let w = u16::from_be_bytes(data[i + 7..i + 9].try_into().ok()?) as u32;
                return Some((w, h));
            }

            let seg_len = u16::from_be_bytes(data[i + 2..i + 4].try_into().ok()?) as usize;
            i = i.saturating_add(2 + seg_len);
        }
        None
    }

    fn _webp_dims(data: &[u8]) -> Option<(u32, u32)> {
        if data.len() < 30 {
            return None;
        }
        let fmt = data.get(12..16)?;

        if fmt == b"VP8 " {
            // Lossy
            let w = u16::from_le_bytes(data[26..28].try_into().ok()?) as u32 & 0x3FFF;
            let h = u16::from_le_bytes(data[28..30].try_into().ok()?) as u32 & 0x3FFF;
            Some((w, h))
        } else if fmt == b"VP8L" {
            // Lossless
            let bits = u32::from_le_bytes(data[21..25].try_into().ok()?);
            let w = (bits & 0x3FFF) + 1;
            let h = ((bits >> 14) & 0x3FFF) + 1;
            Some((w, h))
        } else if fmt == b"VP8X" {
            // Extended
            let w = (u32::from_le_bytes(data[24..28].try_into().ok()?) & 0xFFFFFF) + 1;
            let h = (u32::from_le_bytes(data[27..31].try_into().ok()?) & 0xFFFFFF) + 1;
            Some((w, h))
        } else {
            None
        }
    }

    pub fn mime_from_bytes(data: &[u8]) -> Option<&'static str> {
        if data.starts_with(b"\x89PNG\r\n\x1a\n") {
            Some("image/png")
        } else if data.starts_with(b"\xff\xd8") {
            Some("image/jpeg")
        } else if data.starts_with(b"RIFF") && data.get(8..12) == Some(b"WEBP".as_slice()) {
            Some("image/webp")
        } else {
            None
        }
    }
}

// ── Downloader ──────────────────────────────────────────────────────────────
#[derive(Debug)]
pub struct Downloader {
    config: HarvestConfig,
    db: Arc<StateDB>,
    client: Client,
    stop_flag: Arc<AtomicBool>,
}

unsafe impl Send for Downloader {}
unsafe impl Sync for Downloader {}

impl Downloader {
    pub fn new(config: HarvestConfig, db: Arc<StateDB>) -> Self {
        let client = Client::builder()
            .timeout(config.read_timeout)
            .connect_timeout(config.connect_timeout)
            .user_agent("Mozilla/5.0 (Linux; Android 12; Pixel 6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36 MASH-Harvester/2.0")
            .build()
            .unwrap();

        Self {
            config,
            db,
            client,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    #[allow(dead_code)] // Kept for future expansion
    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::SeqCst);
    }

    fn _ext_from_url(&self, url: &str) -> &str {
        let path = url.to_lowercase();
        if path.ends_with(".jpeg") || path.ends_with(".jpg") {
            return ".jpg";
        } else if path.ends_with(".png") {
            return ".png";
        } else if path.ends_with(".webp") {
            return ".webp";
        }
        ".jpg"
    }

    fn _safe_filename(&self, url: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        let ext = self._ext_from_url(url);
        format!("{}{}", &hash[..12], ext)
    }

    fn _fingerprint(&self, path: &Path) -> Result<String> {
        use sha2::{Digest, Sha256};
        use std::fs::File;
        use std::io::{BufReader, Read};

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; self.config.fingerprint_bytes];
        reader.read_exact(&mut buffer)?;

        let mut hasher = Sha256::new();
        hasher.update(&buffer);
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub async fn download(&self, url: String) -> Result<bool> {
        if self.stop_flag.load(Ordering::SeqCst) {
            return Ok(false);
        }

        let dest = self.config.dest.join(self._safe_filename(&url));
        let tmp = dest.with_extension("tmp");

        // Resume: file already exists
        if tmp.exists() {
            let metadata = std::fs::metadata(&tmp)?;
            if metadata.len() > self.config.min_size_kb * 1024 {
                let fp = self._fingerprint(&tmp)?;
                if !self.db.has_hash(&fp)? {
                    self.db
                        .add_hash(&fp, dest.file_name().unwrap().to_str().unwrap())?;
                    self.db
                        .mark_done(&url, dest.file_name().unwrap().to_str().unwrap())?;
                    return Ok(true);
                }
                std::fs::remove_file(&tmp)?;
                self.db.mark_skip(&url)?;
                return Ok(false);
            }
        }

        // Download with retries
        for attempt in 1..=self.config.retry_max {
            let result = self._download_attempt(&url, &tmp).await;

            match result {
                Ok(saved) => {
                    if saved {
                        return Ok(true);
                    } else {
                        return Ok(false);
                    }
                }
                Err(e) => {
                    if attempt < self.config.retry_max {
                        tokio::time::sleep(
                            self.config.retry_delay * 2u32.pow((attempt - 1) as u32),
                        )
                        .await;
                    } else {
                        self.db.mark_failed(&url)?;
                        return Err(e);
                    }
                }
            }
        }

        Ok(false)
    }

    async fn _download_attempt(&self, url: &str, tmp_path: &Path) -> Result<bool> {
        let response = self.client.get(url).send().await?;

        // Content-Length check
        if let Some(content_length) = response.content_length() {
            let cl_kb = content_length / 1024;
            if cl_kb < self.config.min_size_kb {
                self.db.mark_skip(url)?;
                return Ok(false);
            }
            if cl_kb > self.config.max_size_mb * 1024 {
                self.db.mark_skip(url)?;
                return Ok(false);
            }
        }

        // Stream to temp file
        let mut file = tokio::fs::File::create(tmp_path).await?;
        let mut stream = response.bytes_stream();
        let mut header_bytes = Vec::new();
        let mut total_bytes = 0u64;

        while let Some(chunk) = stream.next().await {
            if self.stop_flag.load(Ordering::SeqCst) {
                tokio::fs::remove_file(tmp_path).await.ok();
                return Ok(false);
            }

            let chunk: bytes::Bytes = chunk?;
            file.write_all(&chunk).await?;
            total_bytes += chunk.len() as u64;

            // Capture header for validation
            if header_bytes.len() < 512 && total_bytes <= 512 {
                header_bytes.extend_from_slice(&chunk);
            }

            // Size limit check
            if total_bytes > self.config.max_size_mb * 1024 * 1024 {
                break;
            }
        }

        file.flush().await?;
        drop(file);

        // Validate downloaded file
        let metadata = tokio::fs::metadata(tmp_path).await?;
        let size_kb = metadata.len() / 1024;

        if size_kb < self.config.min_size_kb {
            tokio::fs::remove_file(tmp_path).await?;
            self.db.mark_skip(url)?;
            return Ok(false);
        }

        // MIME check
        let mime = ImageInfo::mime_from_bytes(&header_bytes);
        if mime.is_none() {
            tokio::fs::remove_file(tmp_path).await?;
            self.db.mark_skip(url)?;
            return Ok(false);
        }

        // Resolution check
        if let Some((w, h)) = ImageInfo::dimensions(&header_bytes) {
            if w < self.config.min_width || h < self.config.min_height {
                tokio::fs::remove_file(tmp_path).await?;
                self.db.mark_skip(url)?;
                return Ok(false);
            }
        }

        // Deduplication
        let fp = self._fingerprint(tmp_path)?;
        if self.db.has_hash(&fp)? {
            tokio::fs::remove_file(tmp_path).await?;
            self.db.mark_skip(url)?;
            return Ok(false);
        }

        // Commit
        tokio::fs::rename(tmp_path, tmp_path.with_extension("")).await?;
        let final_dest = tmp_path.with_extension("");
        self.db
            .add_hash(&fp, final_dest.file_name().unwrap().to_str().unwrap())?;
        self.db
            .mark_done(url, final_dest.file_name().unwrap().to_str().unwrap())?;

        Ok(true)
    }
}

// ── URL Sources ────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
#[allow(dead_code)] // Kept for future expansion
pub enum WallpaperSource {
    Wallhaven,
    Reddit,
    DeviantArt,
}

#[derive(Debug)]
pub struct SourceManager {
    config: HarvestConfig,
    #[allow(dead_code)] // Kept for future expansion
    db: Arc<StateDB>,
    #[allow(dead_code)] // Kept for future expansion
    log: slog::Logger,
    last_request: std::sync::Mutex<f64>,
}

impl SourceManager {
    pub fn new(config: HarvestConfig, db: Arc<StateDB>, log: slog::Logger) -> Self {
        Self {
            config,
            db,
            log,
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

    pub async fn get_urls(&self) -> Result<Vec<String>> {
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

            let response = reqwest::get(&url).await?;
            if response.status().is_success() {
                let json: serde_json::Value = response.json().await?;
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
    log: slog::Logger,
    downloader: Arc<Downloader>,
    stop_flag: Arc<AtomicBool>,
}

impl WallpaperHarvester {
    pub fn new(config: HarvestConfig, log: slog::Logger) -> Result<Self> {
        let state_dir = config.dest.join(".state");
        std::fs::create_dir_all(&state_dir)?;

        let db = Arc::new(StateDB::new(&state_dir.join("state.db"))?);
        let downloader = Arc::new(Downloader::new(config.clone(), db.clone()));

        Ok(Self {
            config,
            db,
            log,
            downloader: downloader.clone(),
            stop_flag: downloader.stop_flag.clone(),
        })
    }

    pub async fn run(&self, ctx: &mut PhaseContext<'_>) -> Result<()> {
        ctx.record_action("🌐  Starting wallpaper harvest operation...");

        // Get URLs from sources
        let source_manager =
            SourceManager::new(self.config.clone(), self.db.clone(), self.log.clone());

        let urls = source_manager.get_urls().await?;
        ctx.record_action(format!("🔍  Found {} potential wallpaper URLs", urls.len()));

        // Add URLs to database
        self.db.add_urls(&urls)?;

        // Process URLs concurrently
        let semaphore = Arc::new(Semaphore::new(self.config.workers));
        let mut join_set = JoinSet::new();

        let pending_urls = self.db.pending_urls()?;
        ctx.record_action(format!(
            "📋  Processing {} pending URLs",
            pending_urls.len()
        ));

        for url in pending_urls {
            if self.stop_flag.load(Ordering::SeqCst) {
                break;
            }

            let permit = semaphore.clone().acquire_owned().await?;
            let downloader = self.downloader.clone();
            let url_clone = url.clone();

            join_set.spawn(async move {
                let _permit = permit; // Hold permit for duration
                let result = downloader.download(url_clone).await;
                result
            });
        }

        // Wait for all downloads to complete
        let mut success_count = 0;
        let mut fail_count = 0;

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(saved)) => {
                    if saved {
                        success_count += 1;
                    }
                }
                Ok(Err(e)) => {
                    ctx.record_warning(format!("⚠️  Download failed: {}", e));
                    fail_count += 1;
                }
                Err(e) => {
                    ctx.record_warning(format!("⚠️  Task failed: {}", e));
                    fail_count += 1;
                }
            }
        }

        let unique_count = self.db.hash_count()?;
        ctx.record_action(format!(
            "📊  Harvest complete: {} unique wallpapers, {} success, {} failed",
            unique_count, success_count, fail_count
        ));

        Ok(())
    }

    #[allow(dead_code)] // Kept for future expansion
    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::SeqCst);
    }
}

// ── Public API ──────────────────────────────────────────────────────────────
#[allow(dead_code)] // Kept for future expansion
pub async fn harvest_wallpapers(
    config: HarvestConfig,
    log: slog::Logger,
    ctx: &mut PhaseContext<'_>,
) -> Result<()> {
    let harvester = WallpaperHarvester::new(config, log)?;
    harvester.run(ctx).await
}
