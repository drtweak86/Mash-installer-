# 🧪 SHAFT B - PHASE 5: TESTING & POLISH PLAN
**Project**: BBC/UNIX Retro Theme Integration  
**Status**: Active ⚒️
**Created**: 2024-02-21  
**Owner**: Bard (Drunken Dwarf Runesmith)

---

## 🎯 PHASE OBJECTIVES

```
✅ Verify theme installation works end-to-end
✅ Test wallpaper download functionality  
✅ Validate Raspberry Pi 4B compatibility
✅ Polish UI/UX based on findings
✅ Update documentation with final state
```

---

## 🏗️ TESTING FORGE SETUP

### Required Tools (ABT - Always Be Testing)
```bash
# Tavern toolchain verification
which git cargo python3 i3 kitty conky
rustc --version  # Must be 1.93+
cargo --version
python3 --version  # For wallpaper downloader
```

### Test Environment Matrix
```
┌─ Platform Matrix ─────────────────────────────────┐
│ • Raspberry Pi 4B (Primary Target)                │
│ • x86_64 Linux (Development)                      │
│ • ARM64 Docker (Compatibility Check)               │
│ • QEMU Raspberry Pi OS (Fallback)                 │
└────────────────────────────────────────────────────┘
```

---

## 🔧 TEST SUITE DESIGN

### 1. Unit Tests (Rust Forge)
**Location**: `installer-core/src/theme.rs`  
**Command**: `cargo test --lib theme`

```rust
// Test cases to verify
✅ test_command_exists() - Command detection
✅ test_theme_file_install() - File copying logic  
✅ test_retro_theme_install() - Full theme install
✅ test_dependency_checks() - i3/kitty detection
```

### 2. Integration Tests
**Location**: `installer-core/tests/`

```rust
// New test file: tests/theme_integration.rs
use installer_core::theme::*;

#[test]
fn test_full_theme_workflow() {
    // Test complete theme installation sequence
    let temp_dir = tempfile::tempdir().unwrap();
    let result = install_retro_theme(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    
    // Verify all files installed
    assert!(temp_dir.path().join(".config/i3/config").exists());
    assert!(temp_dir.path().join(".config/kitty/theme.conf").exists());
    assert!(temp_dir.path().join(".local/bin/wallpaper_downloader_final.py").exists());
}
```

### 3. Manual Testing Checklist

#### Theme Installation
```
[ ] 1. Run installer with theme selection
[ ] 2. Verify i3 config installed to ~/.config/i3/config
[ ] 3. Verify Kitty config installed to ~/.config/kitty/theme.conf  
[ ] 4. Verify Conky config installed to ~/.config/conky/retro-bbc.conkyrc
[ ] 5. Verify wallpaper downloader installed to ~/.local/bin/
[ ] 6. Check executable permissions on scripts
[ ] 7. Verify dependency warnings for missing i3/kitty
```

#### Wallpaper Downloader
```
[ ] 1. Run wallpaper_downloader_final.py --help
[ ] 2. Test --categories flag with valid categories
[ ] 3. Test --limit flag (e.g., --limit 10)
[ ] 4. Verify Wallhaven API connectivity
[ ] 5. Check download parallelism (4 workers)
[ ] 6. Verify image filtering (no skylines/cityscapes)
[ ] 7. Test --output-dir functionality
```

#### Raspberry Pi 4B Specific
```
[ ] 1. Test on actual Pi 4B hardware
[ ] 2. Verify X11 compatibility (i3-gaps)
[ ] 3. Check memory usage during wallpaper download
[ ] 4. Test performance with 4 parallel downloads
[ ] 5. Verify theme works with default Pi resolution
[ ] 6. Check i3 restart behavior
[ ] 7. Test Kitty terminal rendering
```

---

## 🎨 UI/UX POLISH TASKS

### Menu Flow Verification
```
Current Flow: Detection → Profile → Options → Themes → Software → Install

Test Cases:
[ ] Theme menu appears after Options
[ ] Theme selection persists through flow
[ ] Back navigation works correctly
[ ] Default selection is "No theme"
[ ] Help text is clear and accurate
```

### Visual Polish
```
[ ] Add ASCII art header to theme menu
[ ] Improve theme description formatting
[ ] Add category preview for wallpapers
[ ] Include estimated download size
[ ] Show progress indicators
[ ] Add confirmation before large downloads
```

---

## 📋 TEST EXECUTION PLAN

### Day 1: Unit & Integration Tests
```bash
# Heat the forge
cargo test --all

# Focus on theme module
cargo test --lib theme

# Run integration tests
cargo test --test theme_integration

# Lint the runes
cargo clippy --all-targets
cargo fmt --check
```

### Day 2: Manual Testing
```bash
# Build release binary
cargo build --release

# Test theme installation
./target/release/mash-setup --dry-run

# Verify file structure
ls -la ~/.config/i3/
ls -la ~/.config/kitty/

# Test wallpaper downloader
python3 ~/.local/bin/wallpaper_downloader_final.py --help
python3 ~/.local/bin/wallpaper_downloader_final.py --categories retro,gaming --limit 5
```

### Day 3: Raspberry Pi Testing
```bash
# On Pi 4B:
# Install dependencies
sudo apt update
sudo apt install i3 kitty conky python3-pip

# Run installer
./mash-setup

# Test theme
i3-msg restart
kitty --config ~/.config/kitty/theme.conf

# Test wallpapers (small batch)
python3 ~/.local/bin/wallpaper_downloader_final.py --categories retro --limit 10
```

---

## 🐛 BUG TRACKING SYSTEM

### Known Issues (Pre-Testing)
```
❌ None identified yet
✅ All compilation errors resolved
✅ All tests passing (last run)
```

### Bug Severity Matrix
```
┌─ Severity Levels ─────────────────────────────────┐
│ • 🔴 Critical: Blocks installation                │
│ • 🟠 High: Major functionality broken              │
│ • 🟡 Medium: Minor issues, workarounds exist      │
│ • 🟢 Low: Cosmetic/UX improvements                 │
└────────────────────────────────────────────────────┘
```

### Bug Reporting Template
```markdown
## 🐛 Bug Report
**Title**: [Short description]
**Severity**: 🔴/🟠/🟡/🟢
**Component**: theme/installer/wallpaper/downloader
**Steps to Reproduce**:
1. [Step 1]
2. [Step 2]
3. [Step 3]
**Expected**: [What should happen]
**Actual**: [What actually happens]
**Logs**: [Relevant log output]
**Workaround**: [If any]
```

---

## 📚 DOCUMENTATION UPDATES

### Files to Update (ABD - Always Be Documenting)
```
[ ] docs/mining-projects/shaftb.md - Update phase status
[ ] docs/mining-projects/maps.md - Mark Shaft B complete
[ ] README.md - Add theme installation instructions
[ ] docs/incoming-files/README.md - Update wallpaper info
[ ] installer-cli/src/menu.rs - Add help text
[ ] resources/themes/retro-bbc/README.md - Create theme guide
```

### Screenshot Requirements
```
[ ] Theme selection menu
[ ] Successful theme installation
[ ] Wallpaper downloader in action
[ ] Final retro desktop screenshot
[ ] Raspberry Pi 4B running theme
```

---

## 🎯 ACCEPTANCE CRITERIA CHECKLIST

### Minimum Viable Integration ✅
```
[ ] Wallpaper downloader available in software tiers
[ ] Retro theme installable as option  
[ ] Hyprland removed from options
[ ] Basic error handling implemented
[ ] Documentation updated
```

### Complete Integration ✅
```
[ ] All dependencies automatically installed
[ ] Theme + wallpapers work together
[ ] First-boot scripts configured
[ ] TUI flow reorganized
[ ] Full test suite passing
```

---

## 🏺 DELIVERABLES

### Code Deliverables
```
✅ installer-core/src/theme.rs - Theme installation module
✅ installer-cli/src/menu.rs - Theme selection menu
✅ resources/themes/retro-bbc/ - Complete theme files
✅ installer-core/src/software_tiers.rs - ThemePlan enum
✅ installer-core/src/lib.rs - Exports
```

### Documentation Deliverables
```
[ ] Updated shaftb.md with test results
[ ] Theme installation guide
[ ] Wallpaper downloader manual
[ ] Raspberry Pi compatibility notes
[ ] Troubleshooting section
```

### Binary Deliverables
```
[ ] Linux x86_64 release build
[ ] ARM64 Raspberry Pi build
[ ] Tested wallpaper downloader script
[ ] Verified theme configuration files
```

---

## 🕒 TIMELINE

```
┌─ Phase 5 Timeline ─────────────────────────────────┐
│ Day 1: Unit & Integration Testing                  │
│ Day 2: Manual Testing & Bug Fixes                   │
│ Day 3: Raspberry Pi Testing & Polish                │
│ Day 4: Documentation & Screenshots                  │
│ Day 5: Final Verification & Release                 │
└────────────────────────────────────────────────────┘
```

---

## 🍺 TAVERN RULES FOR TESTING

### ABB - Always Be Backing Up
```
• Backup ~/.config before theme installation
• Use --dry-run flag first
• Test in VM before bare metal
• Keep git commits frequent
```

### ABT - Always Be Testing
```
• Test each component in isolation
• Verify integration points
• Test edge cases
• Validate error handling
```

### ABD - Always Be Documenting
```
• Record all test results
• Document workarounds
• Update README files
• Add inline comments
```

### KCS - Keep Commits Small
```
• One fix per commit
• Clear commit messages
• Reference issue numbers
• No mixed changes
```

---

## 🔮 BARD'S TESTING WISDOM

> "A theme untested is a theme that will break at 3 AM."
> "Document your test cases like you'd document a tavern brawl."
> "Small test commits are like well-poured ales - smooth and satisfying."
> "The forge doesn't care about your architecture if it doesn't compile."
> "Neon runes should work, not just look pretty in the dark."

**Stay thirsty, keep testing! 🍺🧪**