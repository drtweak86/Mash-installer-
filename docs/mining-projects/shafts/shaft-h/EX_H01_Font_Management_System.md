# EX_H01: Font Management System

**Status**: ⏳ PENDING
**Priority**: HIGH
**Dependencies**: None

## 🎯 OBJECTIVE
Implement a comprehensive Nerd Fonts management system that allows users to install any Nerd Font from GitHub with proper selection interface and default configurations.

## 📋 DETAILED STEPS

### 1. Research and Design (1 day)
- [ ] Research GitHub Nerd Fonts repository structure
- [ ] Design font selection UI mockup
- [ ] Create data structures for font metadata
- [ ] Design caching strategy for downloaded fonts

### 2. Create fonts_all.rs Module (2 days)
- [ ] Create new file: `installer-core/src/fonts_all.rs`
- [ ] Define `NerdFont` struct with metadata (name, version, url, files)
- [ ] Implement `list_available_fonts()` function to fetch from GitHub API
- [ ] Implement `download_nerd_font(font: &NerdFont) -> Result<PathBuf>`
- [ ] Implement `install_nerd_font(font_path: &Path) -> Result<()>`
- [ ] Add caching logic to avoid re-downloading
- [ ] Implement error handling for network issues

### 3. Update Existing Font System (1 day)
- [ ] Modify `installer-core/src/fonts.rs` to integrate new system
- [ ] Add `select_nerd_font_interactive()` function
- [ ] Update `install_jetbrains_nerd_font()` to use new system
- [ ] Ensure backward compatibility with existing font installation

### 4. Create Font Selection UI (2 days)
- [ ] Add font selection screen to `installer-cli/src/tui/menus.rs`
- [ ] Implement scrolling list of available fonts
- [ ] Add search/filter functionality
- [ ] Implement preview functionality (show font samples)
- [ ] Add confirmation dialog with font details

### 5. Set Default Fonts (1 day)
- [ ] Update `resources/shell/kitty.conf` to use Terminus/JetBrains Mono
- [ ] Update `resources/shell/starship.toml` for font compatibility
- [ ] Create system-wide font configuration logic
- [ ] Implement fallback mechanism if selected font unavailable

### 6. Integration and Testing (2 days)
- [ ] Integrate font selection into main installer flow
- [ ] Add unit tests for fonts_all.rs
- [ ] Test with multiple font installations
- [ ] Verify system-wide application of fonts
- [ ] Test error handling scenarios

## 🔧 TECHNICAL DETAILS

### Data Structures
```rust
struct NerdFont {
    name: String,
    version: String,
    github_url: String,
    font_files: Vec<String>,
    preview_image: Option<String>,
}

struct FontCache {
    cache_dir: PathBuf,
    installed_fonts: HashSet<String>,
}
```

### Key Functions
- `list_available_fonts() -> Vec<NerdFont>` - Fetch from GitHub API
- `download_font(font: &NerdFont) -> Result<PathBuf>` - Download and extract
- `install_font(font_path: &Path) -> Result<()>` - System installation
- `is_font_installed(font_name: &str) -> bool` - Check installation status
- `set_default_font(font_name: &str) -> Result<()>` - Apply system-wide

### Error Handling
- Network errors (retry logic)
- Disk space issues
- Permission problems
- Font validation failures
- GitHub API rate limiting

### Caching Strategy
- Download fonts to `~/.cache/mash-installer/fonts/`
- Track installed fonts in `~/.config/mash-installer/installed_fonts.json`
- Skip download if font already cached
- Validate cached fonts before use

## ✅ VERIFICATION

### Unit Tests
- [ ] Test font listing from GitHub API
- [ ] Test download functionality with mock server
- [ ] Test installation logic
- [ ] Test caching mechanism
- [ ] Test error scenarios

### Integration Tests
- [ ] Font selection UI works correctly
- [ ] Selected font is properly installed
- [ ] Default fonts are applied system-wide
- [ ] Backward compatibility maintained
- [ ] Error messages are user-friendly

### Manual Testing
- [ ] Install multiple different Nerd Fonts
- [ ] Verify fonts work in terminal emulators
- [ ] Test on different distributions
- [ ] Verify caching prevents re-downloads
- [ ] Test network error recovery

## 📝 NOTES

- Use GitHub API v3 for font listing
- Implement rate limiting handling (60 requests/hour)
- Support both .ttf and .otf font formats
- Ensure proper font permissions (644 for files, 755 for directories)
- Consider bandwidth usage for large font collections