# Shaft H: Installer Experience Overhaul & Font/Desktop Environment Integration

**Shaft Title**: Installer Experience Overhaul & Font/Desktop Environment Integration
**Status**: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-02-26

## 🎯 SCOPE

This shaft focuses on enhancing the MASH installer within the existing 4-tile UI layout, making it more user-friendly, informative, and comprehensive while maintaining the current visual structure. It includes:

1. **Font Management**: Install all Nerd Fonts from GitHub with user choice (within existing tiles)
2. **Desktop Environment**: Offer DE installation with X11/Wayland options (integrated into flow)
3. **Improved Install Flow**: Human-friendly, logical progression while keeping 4-tile layout
4. **Enhanced Information Display**: Better info box content with installation details and time estimates
5. **Long Process Confirmation**: Explicit user confirmation for lengthy operations (modal dialog)
6. **Integration**: Fold in mash-wallpaper-harvest and pi-overlord-grimoire functionality (backend)
7. **Cross-Distro Support**: Transmogrify Fedora-specific installs for other distros (backend logic)

## 📁 FILES TO BE CREATED OR TOUCHED

### New Files
- `installer-core/src/fonts_all.rs` - Nerd Fonts installation from GitHub
- `installer-core/src/desktop_environments.rs` - DE installation logic
- `installer-core/src/install_info.rs` - Installation information tracking
- `installer-cli/src/tui/info_box.rs` - Bottom info box component
- `installer-cli/src/tui/confirmation.rs` - Long process confirmation dialog
- `scripts/mash-wallpaper-harvest.py` - Integrated wallpaper downloader
- `scripts/pi-overlord-integration.rs` - Transmogrified overlord functionality

### Modified Files
- `installer-core/src/fonts.rs` - Update for Nerd Fonts selection
- `installer-core/src/lib.rs` - Export new modules
- `installer-cli/src/tui/menus.rs` - Multi-screen install flow
- `installer-cli/src/tui/app.rs` - Info box integration
- `installer-cli/src/main.rs` - New CLI options
- `resources/shell/kitty.conf` - Default font configuration
- `resources/shell/starship.toml` - Font-related updates

### Integration Files
- `docs/incoming-files/mash-wallpaper-harvest.py` → `scripts/mash-wallpaper-harvest.py`
- `docs/incoming-files/pi_overlord_grimoire-1.py` → `scripts/pi-overlord-integration.rs` (transmogrified)

## ⚒️ METHODOLOGY

### Technical Strategy
1. **Modular Design**: Each feature as a separate module with clear interfaces
2. **TDD Approach**: Write tests first, then implementation
3. **Cross-Distro Abstraction**: Use distro detection and package mapping
4. **User Experience First**: Clear information, logical flow, explicit confirmations
5. **Idempotency**: All operations safe to re-run

### Ratatui Patterns
- **Preserve existing 4-tile layout**: Information box, BBS message board, task completion box, system info box
- Component-based UI with clear separation of concerns
- Enhanced state management for improved flow
- Event-driven architecture for user interactions
- Responsive layout that works on various terminal sizes

## 📦 DELIVERABLES

### Phase 1: Font Management System ✅ PLANNED
- [ ] All Nerd Fonts available from GitHub
- [ ] User choice interface in installer
- [ ] Default font set to Terminus/JetBrains Mono
- [ ] Font configuration applied system-wide

### Phase 2: Desktop Environment Support ✅ PLANNED
- [ ] DE selection screen in installer flow
- [ ] X11 variants preferred, Wayland available with warnings
- [ ] Supported DEs: KDE, GNOME, COSMIC, Xfce, MATE, Hyprland
- [ ] Raspberry Pi specific recommendations (X11 over Wayland)

### Phase 3: Improved Install Flow ✅ PLANNED
- [ ] Enhanced logical progression within existing 4-tile layout
- [ ] Human-readable descriptions and options
- [ ] Clear separation of components vs full systems
- [ ] Improved navigation while preserving final install screen
- [ ] Maintain existing tile structure: info box, BBS, tasks, system info

### Phase 4: Information Display ✅ PLANNED
- [ ] Bottom info box showing current operation
- [ ] Approximate time estimates for each step
- [ ] Progress indicators for long operations
- [ ] Helpful context about what's being installed

### Phase 5: Long Process Confirmation ✅ PLANNED
- [ ] Explicit confirmation for operations > 2 minutes
- [ ] Advisory messages about duration
- [ ] Option to cancel or proceed
- [ ] Countdown timer for automatic continuation

### Phase 6: Wallpaper Harvest Integration ✅ PLANNED
- [ ] mash-wallpaper-harvest.py integrated as Rust module
- [ ] 8 category selection (retro, games, anime, etc.)
- [ ] First-boot mode for silent installation
- [ ] Wallhaven API key configuration

### Phase 7: Pi Overlord Transmogrification ✅ PLANNED
- [ ] Fedora-specific packages mapped to other distros
- [ ] Cross-distro package installation logic
- [ ] Phased installation approach
- [ ] Idempotent operations

### Phase 8: Zsh Enhancement ✅ PLANNED
- [ ] colorls integration with Ruby dependency management
- [ ] Essential Zsh plugins (autosuggestions, syntax-highlighting)
- [ ] Comprehensive alias management system
- [ ] ALIAS.md documentation generation
- [ ] User customization hooks

### Phase 9: Testing & Verification ✅ PLANNED
- [ ] Unit tests for all new modules
- [ ] Integration tests for installer flow
- [ ] Dry-run mode verification
- [ ] Cross-distro compatibility testing

## 🔧 VERIFICATION CHECKLIST

### Build Verification
- [ ] `cargo build --workspace` passes
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes

### Test Verification
- [ ] `cargo test --workspace` passes (all existing tests)
- [ ] New unit tests for fonts_all.rs pass
- [ ] New unit tests for desktop_environments.rs pass
- [ ] Integration tests for multi-screen flow pass

### Runtime Verification
- [ ] Font selection screen displays correctly
- [ ] DE selection screen works with all options
- [ ] Info box shows relevant information during installation
- [ ] Long process confirmation appears when expected
- [ ] Wallpaper downloader integrates successfully
- [ ] Cross-distro package mapping works

### CI/CD Verification
- [ ] All GitHub Actions checks pass
- [ ] Documentation builds successfully
- [ ] Release pipeline works with new features

## 📝 PHASE BREAKDOWN

### EX_H01_Font_Management_System.md
1. Create `fonts_all.rs` module
2. Implement GitHub Nerd Fonts downloader
3. Add font selection UI
4. Set Terminus/JetBrains Mono as default
5. Update system font configurations

### EX_H02_Desktop_Environment_Support.md
1. Create `desktop_environments.rs` module
2. Implement DE detection and installation
3. Add X11/Wayland option logic
4. Create DE selection UI
5. Add Raspberry Pi specific warnings

### EX_H03_Enhanced_Install_Flow.md
1. Redesign installer menu structure
2. Implement multi-screen navigation
3. Add human-readable descriptions
4. Create logical component grouping
5. Implement state preservation

### EX_H04_Information_Display.md
1. Create info box component
2. Implement time estimation logic
3. Add context help system
4. Integrate with main UI
5. Test with various operations

### EX_H05_Long_Process_Confirmation.md
1. Create confirmation dialog component
2. Implement duration detection
3. Add advisory messages
4. Implement cancel/proceed logic
5. Add automatic continuation timer

### EX_H06_Wallpaper_Harvest_Integration.md
1. Translate Python script to Rust
2. Implement Wallhaven API integration
3. Add category selection UI
4. Implement first-boot mode
5. Add API key configuration

### EX_H07_Pi_Overlord_Transmogrification.md
1. Analyze Fedora-specific components
2. Create cross-distro package mapping
3. Implement phased installation
4. Ensure idempotency
5. Integrate with main installer

### EX_H08_Zsh_Enhancement.md
1. Create zsh_enhancement.rs module
2. Implement colorls with Ruby dependency management
3. Add Zsh plugin installation framework
4. Create alias management system
5. Generate ALIAS.md documentation

### EX_H09_Testing_and_Verification.md
1. Write unit tests for all new modules
2. Create integration tests
3. Test dry-run mode
4. Verify cross-distro compatibility
5. Update documentation

## 🎯 SUCCESS CRITERIA (GREEN BUILD)

1. **Code Quality**: All code passes fmt, clippy, and tests
2. **Functionality**: All features work as specified
3. **User Experience**: Installer is intuitive and informative
4. **Cross-Distro**: Works on Fedora, Debian, Arch, and derivatives
5. **Performance**: No significant performance regressions
6. **Documentation**: All new features properly documented
7. **Integration**: Wallpaper and overlord functionality seamlessly integrated

## 📋 DEPENDENCIES

### Internal Dependencies
- Existing font installation system (`fonts.rs`)
- Current TUI framework and menu system
- Distro detection system
- Package installation infrastructure

### External Dependencies
- GitHub API for Nerd Fonts
- Wallhaven API for wallpapers
- System package managers (dnf, apt, pacman)

## ⚠️ RISKS & MITIGATIONS

| Risk | Mitigation |
|---|---|
| GitHub rate limiting for Nerd Fonts | Implement caching and retry logic |
| Wallhaven API changes | Versioned API integration with fallback |
| DE installation failures | Comprehensive error handling and rollback |
| Cross-distro package differences | Extensive package mapping database |
| Performance issues with large operations | Progress tracking and user feedback |
| UI complexity overwhelming users | Clear documentation and tooltips |

## 📚 REFERENCE DOCUMENTS

- `docs/forge-tavern/MINING_GOVERNANCE.md` - Governance rules
- `docs/forge-tavern/maps.md` - Current shaft status
- `docs/forge-tavern/maps-explored.md` - Historical context
- `docs/incoming-files/README.md` - Wallpaper downloader documentation
- `docs/incoming-files/pi_overlord_grimoire-1.py` - Overlord reference

"*A well-crafted installer is like a fine ale - it should be smooth, satisfying, and leave the user wanting more.*" — Bard 🍺