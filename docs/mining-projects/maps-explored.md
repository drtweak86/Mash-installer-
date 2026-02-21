# Mining Projects – Maps Explored
> Historical ledger of completed shafts and sessions with technical diff analysis.

---

## SHAFT A <COMPLETED> ✅

### Summary
Completed strategic reconnaissance of the codebase. Audited architecture, identified integration points, and created comprehensive strategic plan for retro theme and wallpaper integration.

### Technical Changes Analyzed

#### Codebase Audit Findings
- **Pure Rust Analysis**: ~95% pure Rust core
  - 6,355 lines across 38 modules in `installer-core`
  - `installer-cli` fully in Rust
  - Cross-compilation support for x86_64 and aarch64

#### Shell Boundaries Identified
- **Font installation**: Shell required (no Rust-native package management)
- **Docker setup**: Shell required (systemctl integration)
- **Rust toolchain**: Shell required (rustup install script)
- **Desktop environment**: Shell required (package manager integration)
- **Git LFS**: Shell required (no Rust-native equivalent)
- **oh-my-zsh**: Shell required (third-party install script)
- **cargo tools**: Shell required (binary installation)

#### Hard Blockers (Unavoidable Shell Dependencies)
1. **sudo**: No Rust-native privilege escalation (Linux kernel boundary)
2. **Package managers**: System binaries, no Rust wrappers at needed level
3. **systemctl**: D-Bus API exists but significantly more complex than shell
4. **git clone**: libgit2 bindings add C dependency and 500KB+ to binary

#### Architecture Improvements
- **SystemOps trait**: Abstracted filesystem operations
- **PhaseContext::run_or_record()**: Central gate for all side effects
- **InstallationReport**: Captures all actions for audit
- **dry_run flag**: Gates every write operation

### Files Touched
- `installer-core/src/system_ops.rs` (new)
- `installer-core/src/phase_context.rs` (updated)
- `installer-core/src/installation_report.rs` (updated)
- `installer-core/src/orchestrator.rs` (updated)
- `installer-core/src/lib.rs` (exports updated)

### Verification
- ✅ All 82 tests passing
- ✅ cargo fmt clean
- ✅ cargo clippy --all-targets --all-features -- -D warnings clean
- ✅ Cross-compilation verified for both targets

---

## SHAFT B <NOT_COMPLETED> ⏳

### Summary
Integration of BBC/UNIX retro-futuristic theme (i3-gaps + Kitty) and wallpaper downloader into MASH Installer main flow.

### Technical Changes Planned

#### Phase 2 - Core Integration
- **Wallpaper downloader integration**: Add to software tiers
  - 8 categories, 6000 images
  - Wallhaven API integration
  - First-boot mode support
  - Error handling for API failures

- **Retro theme integration**: Add to software tiers
  - i3-gaps configuration
  - Kitty terminal with Terminus 14px
  - Conky system monitor
  - Retro color schemes

#### Phase 3 - Theme Integration
- **Dependency checking**: Auto-install i3/Kitty
  - Package manager detection
  - Version compatibility checks
  - Fallback mechanisms

- **Configuration deployment**: Theme installation logic
  - File permissions management
  - Environment variable setup
  - Service configuration

- **Hyprland retirement**: Remove references
  - Configuration files cleanup
  - Documentation updates
  - Menu system updates

#### Phase 4 - TUI Reorganization
- **New theme selection menu**: Ratatui implementation
  - Menu navigation
  - Preview functionality
  - Selection confirmation

- **Menu reordering**: Logical flow
  - Detection → Profile → Options → Themes → Software → Install
  - Backward compatibility maintained
  - User experience improvements

#### Phase 5 - Testing & Polish
- **Raspberry Pi 4B testing**: Hardware verification
  - Performance metrics
  - Memory usage analysis
  - Thermal testing

- **Wallpaper download testing**: API integration
  - Error handling verification
  - Fallback mechanisms
  - User feedback loops

- **Documentation updates**: Complete documentation
  - User guides
  - Troubleshooting sections
  - Changelog entries

### Files to be Touched
- `installer-core/src/software_tiers.rs` (updated)
- `installer-cli/src/tui/menus.rs` (updated)
- `installer-cli/src/tui/app.rs` (updated)
- `resources/themes/retro-bbc/` (new)
- `docs/mining-projects/shaftb.md` (updated)
- `README.md` (updated)

### Blockers
- 🔴 **Wallhaven API Key**: Required for production deployment
- 🟡 **Integration Time**: 5 days estimated
- 🟡 **Pi 4B Testing**: Hardware availability

### Status
- ✅ Planning Complete
- ⏳ Integration Pending
- 📅 Target Completion: 2026-02-27

---

## SHAFTC <NOT_COMPLETED> ⏳

### The Bard Begins His Quest in a New Tunnel Below

**Objective**: Transform TUI aesthetic from cyberpunk to BBC Micro/UNIX terminal (1984 crashed into 2026)

### 10 Point Plan to Safely Excavate the Ruin

#### 1. Foundation: Core Functionality First
- **Priority**: Ensure all existing functionality works before aesthetic changes
- **Files**: `installer-cli/src/tui/app.rs`, `installer-cli/src/tui/render.rs`
- **Changes**: No functional changes, only visual updates

#### 2. Terminal Emulation: BBC Micro Style
- **Objective**: Create authentic BBC Micro terminal feel
- **Files**: `installer-cli/src/tui/theme.rs`
- **Changes**:
  - Green-on-black color scheme
  - Teletext font support
  - Blocky character rendering
  - Mode 7 display emulation

#### 3. ASCII Art: Loading Screens
- **Objective**: Replace cyberpunk graphics with retro ASCII art
- **Files**: `resources/themes/retro-bbc/`
- **Changes**:
  - BBC Micro loading screens
  - UNIX terminal art
  - 8-bit style graphics
  - Animated loading indicators

#### 4. Layout: UNIX Terminal Style
- **Objective**: Recreate classic UNIX terminal layout
- **Files**: `installer-cli/src/tui/render.rs`
- **Changes**:
  - Single pane terminal
  - Command prompt style
  - Status bar at bottom
  - Logical menu flow

#### 5. Typography: Retro Fonts
- **Objective**: Use authentic retro terminal fonts
- **Files**: `installer-core/src/fonts.rs`
- **Changes**:
  - Terminus font (14px)
  - Fixed-width only
  - No anti-aliasing
  - Pixel-perfect rendering

#### 6. Color Scheme: BBC Micro Palette
- **Objective**: Implement authentic BBC Micro colors
- **Files**: `installer-cli/src/tui/theme.rs`
- **Changes**:
  - Green text on black background
  - Yellow highlights
  - Blue for active elements
  - Red for errors/warnings

#### 7. Sound: Optional 8-bit Beeps
- **Objective**: Add retro sound effects (optional)
- **Files**: `installer-cli/src/tui/app.rs`
- **Changes**:
  - Keyboard beep on Enter
  - Menu navigation sounds
  - Completion chime
  - Configurable sound volume

#### 8. Animation: Teletext Style
- **Objective**: Create teletext-style animations
- **Files**: `installer-cli/src/tui/render.rs`
- **Changes**:
  - Scrolling text effects
  - Page transitions
  - Loading animations
  - Progress indicators

#### 9. Error Messages: UNIX Style
- **Objective**: Format errors like classic UNIX
- **Files**: `installer-core/src/error.rs`
- **Changes**:
  - Error codes
  - Brief descriptions
  - No fancy formatting
  - Clear action items

#### 10. Documentation: Retro Style Guide
- **Objective**: Document the retro aesthetic
- **Files**: `docs/mining-projects/shaftc.md`
- **Changes**:
  - Design principles
  - Color palette reference
  - Typography guide
  - Screenshots of final design

### Technical Implementation

#### Phase 1: Foundation (1 day)
- [ ] Audit current TUI code
- [ ] Identify all rendering points
- [ ] Create theme abstraction layer
- [ ] Verify all functionality works

#### Phase 2: BBC Micro Theme (2 days)
- [ ] Implement green-on-black color scheme
- [ ] Add teletext font support
- [ ] Create ASCII art loading screens
- [ ] Test on various terminal types

#### Phase 3: UNIX Layout (1 day)
- [ ] Redesign menu structure
- [ ] Implement single-pane layout
- [ ] Add status bar
- [ ] Test navigation flow

#### Phase 4: Polish (1 day)
- [ ] Add sound effects (optional)
- [ ] Create animations
- [ ] Finalize error messages
- [ ] Complete documentation

### Files to be Modified
1. `installer-cli/src/tui/theme.rs` - Color schemes and styling
2. `installer-cli/src/tui/render.rs` - Layout and rendering
3. `installer-cli/src/tui/app.rs` - Navigation and interaction
4. `installer-core/src/fonts.rs` - Font handling
5. `installer-core/src/error.rs` - Error formatting
6. `resources/themes/retro-bbc/` - Assets (new)
7. `docs/mining-projects/shaftc.md` - Documentation (new)

### Risk Assessment
- **Low Risk**: Visual changes only, no functional impact
- **Medium Risk**: Font rendering on different terminals
- **High Risk**: Sound effects compatibility
- **Mitigation**: Optional sound, fallback fonts, extensive testing

### Timeline
- **Start Date**: 2026-03-01 (after Shaft B completion)
- **Duration**: 5 days
- **Completion**: 2026-03-06

---

## Guiding Principles

- **Function > Form**: Working code over perfect aesthetics
- **Test Before Extend**: Verify functionality before visual changes
- **User Needs > Architecture**: Focus on user experience
- **Simple > Clever**: Straightforward solutions
- **Document Before Code**: ABD - Always Be Documenting

---

## Status Dashboard

### Active Projects
| Project | Status | Timeline |
|---------|--------|----------|
| Shaft B - Retro Integration | ✅ Planning Complete | 2026-02-22 to 2026-02-27 |
| Shaft C - TUI Aesthetic | ⏳ Planned | 2026-03-01 to 2026-03-06 |

### Completed Projects
| Project | Status | Completion Date |
|---------|--------|-----------------|
| Shaft A - Reconnaissance | ✅ Complete | 2026-02-20 |
| Phase 1-5 - Core | ✅ Complete | 2026-02-20 |
| Release v0.1.2 | ✅ Complete | 2026-02-20 |

### Upcoming Projects
| Project | Status | Estimated Start |
|---------|--------|-----------------|
| Shaft C - TUI Aesthetic | ⏳ Planned | 2026-03-01 |

---

## Source of Truth

**Primary Documents:**
- `maps.md` - Current status and active projects
- `shafta.md` - Shaft A technical report
- `shaftb.md` - Shaft B integration plan
- `shaftc.md` - Shaft C aesthetic plan (future)

**Supporting Documents:**
- `DOCUMENT_HYGIENE_SUMMARY.md` - Documentation organization
- `scratch/README.md` - Archived content guide

---

**Last Updated**: 2026-02-21
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️

*Document Status: ACTIVE* 🟢
*Version: 2.0* (Updated with Shaft C plan)
