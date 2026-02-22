# 🗺️ SHAFT J: WALLPAPER DOWNLOADER RUST CONVERSION

## 🎭 By the Bard, Drunken Dwarf Runesmith
*Mythic Assembly & Sigil Heuristics*
*Forge Tavern, Neon District*

---

## 🏺 SHAFT OVERVIEW

**Objective**: Convert the Python wallpaper downloader to Rust, eliminating Python dependencies and aligning with the project's Rust vision.

**Status**: ✅ PLANNING COMPLETE | 🔨 EXECUTION PENDING

**Timeline**: 7 days (2024-02-23 to 2024-02-29)

**Risk Level**: MEDIUM (mitigated with phased approach)

**Reward Level**: HIGH (long-term maintainability, performance, alignment)

**Dependencies**: None

---

## 📜 DETAILED PLAN

### Phase 1: Analysis and Preparation (Day 1)

#### 1.1 Analyze Current Python Implementation
**Files to Touch**:
- `docs/incoming-files/wallpaper_downloader_final.py`
- `docs/incoming-files/wallpaper_downloader_README.md`

**Changes to Make**:
- Document current functionality
- Identify all API endpoints
- Map Python libraries to Rust equivalents
- Document error handling patterns
- Document configuration options

**Sub-steps**:
1. **1.1.1**: Create functionality matrix
   - List all functions and their purposes
   - Document input/output patterns
   - Note error handling approaches
   
2. **1.1.2**: Map Python libraries to Rust
   - `requests` → `reqwest`
   - `json` → `serde_json`
   - `argparse` → `clap` or `structopt`
   - `os.path` → `std::path`
   - `shutil` → `std::fs`
   
3. **1.1.3**: Document API endpoints
   - List all HTTP endpoints used
   - Document authentication if any
   - Note rate limiting behavior

#### 1.2 Set Up Rust Project Structure
**Files to Create**:
- `wallpaper-downloader/Cargo.toml`
- `wallpaper-downloader/src/main.rs`
- `wallpaper-downloader/src/lib.rs`
- `wallpaper-downloader/src/config.rs`
- `wallpaper-downloader/src/api.rs`
- `wallpaper-downloader/src/download.rs`
- `wallpaper-downloader/src/error.rs`

**Changes to Make**:
- Create new workspace member
- Set up dependencies
- Configure build settings
- Set up basic project structure

**Sub-steps**:
1. **1.2.1**: Create workspace member
   - Add to root `Cargo.toml`
   - Create `wallpaper-downloader` directory
   
2. **1.2.2**: Set up dependencies
   - `reqwest` for HTTP requests
   - `tokio` for async runtime
   - `serde`, `serde_json` for JSON handling
   - `clap` or `structopt` for CLI parsing
   - `anyhow` or `thiserror` for error handling
   - `log` and `env_logger` for logging
   
3. **1.2.3**: Configure build settings
   - Set up release profile
   - Configure target platforms
   - Set up feature flags if needed

---

### Phase 2: Core Implementation (Days 2-4)

#### 2.1 Implement Configuration Handling
**Files to Touch**:
- `wallpaper-downloader/src/config.rs`

**Changes to Make**:
- Parse command-line arguments
- Handle configuration files if any
- Validate inputs
- Provide sensible defaults

**Sub-steps**:
1. **2.1.1**: Define configuration struct
   ```rust
   #[derive(Debug, Clone, clap::Parser)]
   pub struct Config {
       /// Category of wallpapers
       #[clap(long, default_value = "retro")]
       pub category: String,
       
       /// Number of wallpapers to download
       #[clap(long, default_value_t = 10)]
       pub limit: usize,
       
       /// Output directory
       #[clap(long, default_value = "./wallpapers")]
       pub output_dir: PathBuf,
       
       /// First boot mode
       #[clap(long)]
       pub first_boot: bool,
   }
   ```

2. **2.1.2**: Implement validation logic
   - Validate category names
   - Validate output directory
   - Check filesystem permissions

3. **2.1.3**: Add error handling
   - Define configuration errors
   - Provide helpful error messages

#### 2.2 Implement API Client
**Files to Touch**:
- `wallpaper-downloader/src/api.rs`
- `wallpaper-downloader/src/error.rs`

**Changes to Make**:
- Create HTTP client
- Implement API endpoints
- Handle authentication if needed
- Implement rate limiting
- Add retry logic

**Sub-steps**:
1. **2.2.1**: Define API error types
   ```rust
   #[derive(Debug, thiserror::Error)]
   pub enum ApiError {
       #[error("HTTP error: {0}")]
       Http(#[from] reqwest::Error),
       
       #[error("API error: {0}")]
       Api(String),
       
       #[error("Rate limited")]
       RateLimited,
   }
   ```

2. **2.2.2**: Implement API client struct
   ```rust
   pub struct ApiClient {
       client: reqwest::Client,
       base_url: String,
   }
   
   impl ApiClient {
       pub fn new(base_url: &str) -> Self {
           Self {
               client: reqwest::Client::new(),
               base_url: base_url.to_string(),
           }
       }
       
       pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<Wallpaper>, ApiError> {
           // Implementation
       }
   }
   ```

3. **2.2.3**: Implement endpoint methods
   - Search wallpapers
   - Get wallpaper details
   - Download wallpaper
   - Handle pagination

#### 2.3 Implement Download Logic
**Files to Touch**:
- `wallpaper-downloader/src/download.rs`

**Changes to Make**:
- Download files from URLs
- Handle file naming
- Create output directory
- Handle file conflicts
- Add progress reporting

**Sub-steps**:
1. **2.3.1**: Implement download function
   ```rust
   pub async fn download_wallpaper(
       &self,
       url: &str,
       output_path: &Path,
   ) -> Result<(), DownloadError> {
       let response = self.client.get(url).send().await?;
       let bytes = response.bytes().await?;
       
       if let Some(parent) = output_path.parent() {
           tokio::fs::create_dir_all(parent).await?;
       }
       
       tokio::fs::write(output_path, bytes).await?;
       Ok(())
   }
   ```

2. **2.3.2**: Add progress reporting
   - Use `indicatif` crate for progress bars
   - Report download speed
   - Show estimated time remaining

3. **2.3.3**: Handle file conflicts
   - Check for existing files
   - Implement overwrite confirmation
   - Add unique naming if needed

#### 2.4 Implement Main Application Logic
**Files to Touch**:
- `wallpaper-downloader/src/main.rs`
- `wallpaper-downloader/src/lib.rs`

**Changes to Make**:
- Parse command-line arguments
- Initialize components
- Orchestrate the workflow
- Handle errors gracefully
- Add logging

**Sub-steps**:
1. **2.4.1**: Implement main function
   ```rust
   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       let config = Config::parse();
       
       env_logger::init();
       log::info!("Starting wallpaper downloader");
       
       let api = ApiClient::new("https://api.wallhaven.cc");
       let downloader = Downloader::new(config.output_dir.clone());
       
       let wallpapers = api.search(&config.category, config.limit).await?;
       
       for wallpaper in wallpapers {
           let output_path = downloader.download(&wallpaper).await?;
           log::info!("Downloaded: {}", output_path.display());
       }
       
       Ok(())
   }
   ```

2. **2.4.2**: Implement library functions
   - Export public API
   - Create reusable components
   - Add documentation

3. **2.4.3**: Add comprehensive error handling
   - Convert errors to user-friendly messages
   - Add recovery suggestions
   - Log detailed error information

---

### Phase 3: Testing (Day 5)

#### 3.1 Unit Tests
**Files to Create**:
- `wallpaper-downloader/tests/config_test.rs`
- `wallpaper-downloader/tests/api_test.rs`
- `wallpaper-downloader/tests/download_test.rs`

**Changes to Make**:
- Test configuration parsing
- Test API client methods
- Test download logic
- Test error handling

**Sub-steps**:
1. **3.1.1**: Test configuration
   - Test default values
   - Test custom values
   - Test validation
   
2. **3.1.2**: Test API client
   - Mock HTTP requests
   - Test error handling
   - Test rate limiting
   
3. **3.1.3**: Test download logic
   - Test file creation
   - Test directory creation
   - Test error cases

#### 3.2 Integration Tests
**Files to Create**:
- `wallpaper-downloader/tests/integration_test.rs`

**Changes to Make**:
- Test full workflow
- Test with real API (mocked)
- Test error scenarios
- Test edge cases

**Sub-steps**:
1. **3.2.1**: Test happy path
   - Search and download wallpapers
   - Verify files are created
   
2. **3.2.2**: Test error scenarios
   - Network failures
   - Invalid inputs
   - Permission errors
   
3. **3.2.3**: Test edge cases
   - Empty results
   - Large downloads
   - Special characters in filenames

#### 3.3 End-to-End Tests
**Files to Create**:
- `wallpaper-downloader/tests/e2e_test.rs`

**Changes to Make**:
- Test CLI interface
- Test all command-line options
- Test output format
- Test logging

**Sub-steps**:
1. **3.3.1**: Test CLI parsing
   - Test all flags
   - Test help output
   - Test version output
   
2. **3.3.2**: Test full workflow
   - Run with various options
   - Verify output
   - Verify files

---

### Phase 4: Integration (Day 6)

#### 4.1 Integrate with Main Project
**Files to Touch**:
- `installer-core/src/software_tiers.rs`
- `installer-core/src/catalog/mod.rs`
- `installer-cli/src/catalog.rs`

**Changes to Make**:
- Add wallpaper downloader to software catalog
- Update installation logic
- Add configuration options
- Update documentation

**Sub-steps**:
1. **4.1.1**: Add to software catalog
   ```rust
   pub enum SoftwareTier {
       // ... existing tiers
       WallpaperDownloader {
           category: String,
           limit: usize,
           first_boot: bool,
       },
   }
   ```

2. **4.1.2**: Update installation logic
   - Add wallpaper downloader phase
   - Handle dependencies
   - Add configuration options
   
3. **4.1.3**: Update documentation
   - Add to user manual
   - Update configuration reference
   - Add troubleshooting section

#### 4.2 Update Installation Scripts
**Files to Touch**:
- `install.sh`

**Changes to Make**:
- Update to use Rust version
- Remove Python dependencies
- Add Rust version check
- Update error handling

**Sub-steps**:
1. **4.2.1**: Update dependency installation
   - Remove Python and pip
   - Add Rust toolchain check
   
2. **4.2.2**: Update installation logic
   - Build wallpaper downloader
   - Install binary
   - Set up configuration

3. **4.2.3**: Update error handling
   - Add Rust-specific checks
   - Update error messages

---

### Phase 5: Documentation (Day 7)

#### 5.1 Update User Documentation
**Files to Touch**:
- `docs/MANUAL.md`
- `docs/incoming-files/README.md`

**Changes to Make**:
- Add Rust version usage
- Update configuration examples
- Add troubleshooting
- Update FAQ

**Sub-steps**:
1. **5.1.1**: Add usage instructions
   - Command-line options
   - Configuration examples
   - Output format
   
2. **5.1.2**: Add troubleshooting
   - Common errors
   - Solutions
   - Debugging tips
   
3. **5.1.3**: Update FAQ
   - Add Rust-specific questions
   - Update existing answers

#### 5.2 Update Developer Documentation
**Files to Touch**:
- `docs/mining-projects/shaftj.md` (this file)
- `docs/HISTORY.md`

**Changes to Make**:
- Add implementation details
- Document design decisions
- Add migration guide
- Update contribution guidelines

**Sub-steps**:
1. **5.2.1**: Add implementation notes
   - Architecture decisions
   - Performance considerations
   - Security considerations
   
2. **5.2.2**: Document migration path
   - From Python to Rust
   - Backward compatibility
   - Rollback procedure
   
3. **5.2.3**: Update contribution guidelines
   - Testing requirements
   - Code style
   - Review process

---

## 📊 RISK MITIGATION

### Risk 1: API Changes
**Mitigation**:
- Implement adaptive API client
- Add version detection
- Provide fallback behavior
- Monitor API changes

### Risk 2: Performance Issues
**Mitigation**:
- Profile before and after
- Add benchmarks
- Optimize critical paths
- Monitor real-world usage

### Risk 3: User Resistance
**Mitigation**:
- Provide clear migration guide
- Offer fallback to Python version
- Highlight benefits
- Gather feedback

### Risk 4: Testing Gaps
**Mitigation**:
- Comprehensive test coverage
- Integration testing
- User acceptance testing
- Monitor production usage

---

## 🔮 BARD'S WISDOM

> "The path to Rust is clear, but the journey must be safe."
> "Test as you build, or regret it later."
> "Documentation is the map that guides the next miner."
> "If it's not tested, it's not done."
> "The forge demands safety, the tavern demands wisdom."

---

## 🍻 TAVERN VERDICT

The tavern has spoken. The plan is clear:

```bash
🍺 SAFE, PHASED, DOCUMENTED RUST CONVERSION 🔥
```

**Risk**: MEDIUM (mitigated)
**Reward**: HIGH
**Net Value**: ✅ EXCELLENT

---

*Signed*,
Bard, Drunken Dwarf Runesmith
Mythic Assembly & Sigil Heuristics
Forge Tavern, Neon District

**Shaft Status**: ✅ PLANNING COMPLETE
**Execution Status**: 🔨 PENDING
**Last Updated**: 2024-02-22
**Version**: 1.0
**Alignment**: Pragmatic Zen

---

**The plan is drawn. The shaft is ready. The journey begins.** ⛏️🔥
