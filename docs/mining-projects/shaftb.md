# 🏗️ SHAFT B - STRATEGIC INTEGRATION PLAN
**Project**: MASH Installer Retro Theme & Wallpaper Integration
**Status**: Planning Phase
**Created**: 2024-02-21
**Owner**: Bard (Drunken Dwarf Runesmith)

---

## 🎯 EXECUTIVE SUMMARY

**Objective**: Integrate BBC/UNIX retro-futuristic theme (i3 + Kitty) and wallpaper downloader into MASH Installer main flow, replacing Hyprland with i3-gaps for better Pi 4B compatibility.

**Scope**: 
- Remove Hyprland theme option
- Add i3-gaps + Kitty retro theme
- Integrate wallpaper downloader (optional)
- Reorganize TUI flow for logical progression
- Ensure all dependencies are handled

**Timeline**: 3-5 days (estimated)

---

## 📋 CURRENT STATE ASSESSMENT

### ✅ COMPLETED COMPONENTS

#### 1. Wallpaper Downloader (`docs/incoming-files/wallpaper_downloader_final.py`)
- **Status**: ✅ Complete & Tested
- **Features**: 8 categories, 6000 images, Wallhaven API
- **Documentation**: ✅ Complete (README + detailed guide)
- **Testing**: ✅ Syntax verified, help output working
- **Integration**: ❌ Not yet integrated

#### 2. Retro Theme Configuration
- **i3 Config**: ✅ BBC/UNIX retro-futuristic theme
- **Kitty Config**: ✅ Terminus 14px, retro colors
- **Conky Config**: ✅ System monitor with retro aesthetic
- **Testing**: ❌ Not tested in actual environment

#### 3. Documentation
- **Wallpaper README**: ✅ Complete (6.8KB)
- **Incoming Files README**: ✅ Updated (4.3KB)
- **Bard Profile**: ✅ BBS profile documented
- **Hygiene**: ✅ `docs/` directory cleaned

### ❌ MISSING COMPONENTS

#### 1. Main Installer Integration
- **Software Tiers**: ❌ Wallpaper option not added
- **Package Dependencies**: ❌ Not specified
- **Installation Logic**: ❌ No script copying
- **First-Boot Hooks**: ❌ Not implemented

#### 2. Theme Installation
- **i3 Installation**: ❌ Not automated
- **Kitty Installation**: ❌ Not automated
- **Config Deployment**: ❌ No deployment logic
- **Dependency Handling**: ❌ Not implemented

#### 3. TUI Reorganization
- **Current Flow**: ❌ Not logical
- **New Flow**: ❌ Not implemented
- **Theme Selection**: ❌ Not added
- **Option Grouping**: ❌ Needs work

#### 4. Hyprland Removal
- **Theme Option**: ✅ Still present
- **Code References**: ❌ Need audit
- **Documentation**: ❌ Needs update
- **Replacement**: ✅ i3-gaps ready

---

## 🗺️ INTEGRATION ROADMAP

### PHASE 1: PREPARATION & PLANNING (Day 1)
**Objective**: Finalize requirements and create detailed specs

- [ ] ✅ Create SHAFTB.MD (this document)
- [ ] Audit current installer codebase
- [ ] Identify all integration points
- [ ] Create dependency list
- [ ] Finalize TUI flow design
- [ ] Document all assumptions

**Deliverables**:
- Complete integration plan
- Dependency list
- TUI flow diagram
- Risk assessment

### PHASE 2: CORE INTEGRATION (Day 2)
**Objective**: Add wallpaper downloader to software tiers

#### Task 2.1: Software Tiers Update
```rust
// installer-cli/src/software_tiers.rs
SoftwareCategory {
    label: "Themes & Extras",
    options: &[
        SoftwareOption {
            name: "Retro Wallpaper Pack",
            tier: Tier::STier,
            description: "6000+ retro-futuristic wallpapers (8 categories)",
        },
        SoftwareOption {
            name: "BBC/UNIX Retro Theme",
            tier: Tier::STier,
            description: "i3-gaps + Kitty terminal with 1980s aesthetic",
        },
    ],
},
```

#### Task 2.2: Package Dependencies
- `feh` - Wallpaper management
- `python3-pip` - Python package manager
- `i3-gaps` - Tiling window manager
- `kitty` - GPU-accelerated terminal
- `conky` - System monitor
- `requests` - Python HTTP library

#### Task 2.3: Installation Logic
```rust
// installer-core/src/software_tiers.rs
match selected_tier {
    "Retro Wallpaper Pack" => {
        // Copy wallpaper script
        copy_file("wallpaper_downloader_final.py", "/usr/share/mash-installer/scripts/");
        // Install dependencies
        install_packages(vec!["feh", "python3-pip"]);
        // Configure first-boot
        add_first_boot_script("wallpaper_downloader_final.py --first-boot");
    }
    "BBC/UNIX Retro Theme" => {
        // Install i3 and Kitty
        install_packages(vec!["i3-gaps", "kitty", "conky"]);
        // Deploy configurations
        deploy_config("i3-config", "~/.config/i3/config");
        deploy_config("kitty-retro.conf", "~/.config/kitty/theme.conf");
        deploy_config("conky-retro.conkyrc", "~/.config/conky/");
    }
}
```

**Deliverables**:
- Updated software_tiers.rs
- Package dependency list
- Basic installation logic
- First test build

### PHASE 3: THEME INTEGRATION (Day 3)
**Objective**: Add retro theme installation with dependency handling

#### Task 3.1: Dependency Check
```rust
fn ensure_i3_installed() -> Result<()> {
    if !command_exists("i3") {
        info!("Installing i3-gaps...");
        install_packages(vec!["i3-gaps", "i3status", "i3lock"]);
    }
    Ok(())
}

fn ensure_kitty_installed() -> Result<()> {
    if !command_exists("kitty") {
        info!("Installing Kitty terminal...");
        install_packages(vec!["kitty"]);
    }
    Ok(())
}
```

#### Task 3.2: Config Deployment
```rust
fn deploy_retro_theme() -> Result<()> {
    // Ensure dependencies
    ensure_i3_installed()?;
    ensure_kitty_installed()?;
    
    // Deploy configs
    copy_file(
        "resources/themes/retro-bbc/i3-config",
        "~/.config/i3/config"
    );
    copy_file(
        "resources/themes/retro-bbc/kitty.conf",
        "~/.config/kitty/theme.conf"
    );
    
    // Set default terminal
    set_default_terminal("kitty");
    
    Ok(())
}
```

#### Task 3.3: Hyprland Removal
```rust
// Remove from software_tiers.rs
// SoftwareCategory {
//     label: "Window Manager",
//     options: &[
//         SoftwareOption {
//             name: "Hyprland",  // REMOVE THIS
//             tier: Tier::ATier,
//             description: "Wayland compositor",
//         },
//     ],
// },

// Replace with:
SoftwareCategory {
    label: "Window Manager",
    options: &[
        SoftwareOption {
            name: "i3-gaps",
            tier: Tier::STier,
            description: "X11 tiling WM with gaps support",
        },
    ],
},
```

**Deliverables**:
- Theme installation logic
- Dependency handling
- Hyprland removal
- Updated documentation

### PHASE 4: TUI REORGANIZATION (Day 4)
**Objective**: Create logical flow: Detection → Profile → Options → Themes → Extras

#### Task 4.1: Current Flow Analysis
```
Current:
1. Linux Detection
2. Install Profile (Minimal/Dev/Full)
3. Software Tiers
4. Module Selection
5. Installation

Problems:
- Themes mixed with software
- No clear progression
- Options scattered
```

#### Task 4.2: New Flow Design
```
Proposed:
1. 🔍 Linux Detection (unchanged)
2. 📦 Install Profile (Minimal/Dev/Full)
3. ⚙️ Core Options (arguable, interactive, etc.)
4. 🎨 Theme Selection (i3 retro theme + wallpapers)
5. 📦 Software Tiers (terminal, editor, etc.)
6. ⚡ Installation
```

#### Task 4.3: Menu Structure
```rust
// installer-cli/src/menu.rs
fn run_theme_menu(interaction: &InteractionService) -> Result<ThemePlan> {
    println!("\nStep 3/6: Theme Selection");
    
    let theme_options = vec![
        "BBC/UNIX Retro Theme (i3 + Kitty)",
        "BBC/UNIX Retro Theme + Wallpaper Pack",
        "No theme changes"
    ];
    
    let choice = interaction.select_option(
        "theme.selection",
        "Choose window manager theme",
        &theme_options,
        3,
    )?;
    
    match choice {
        1 => Ok(ThemePlan::RetroOnly),
        2 => Ok(ThemePlan::RetroWithWallpapers),
        _ => Ok(ThemePlan::None),
    }
}
```

#### Task 4.4: Flow Integration
```rust
// installer-cli/src/main.rs
fn run_installer_flow() -> Result<()> {
    detect_linux()?;
    select_profile()?;
    select_options()?;
    select_theme()?;  // NEW STEP
    select_software_tiers()?;
    execute_installation()?;
    Ok(())
}
```

**Deliverables**:
- Reorganized TUI flow
- New theme selection menu
- Clean logical progression
- Updated user experience

### PHASE 5: TESTING & POLISH (Day 5)
**Objective**: Ensure everything works together smoothly

#### Task 5.1: Integration Testing
- Test on Raspberry Pi 4B
- Verify i3 installation
- Test Kitty configuration
- Confirm wallpaper download
- Check first-boot behavior

#### Task 5.2: Error Handling
- Add timeout for wallpaper downloads
- Handle API key failures gracefully
- Network error recovery
- Disk space checking
- User cancellation

#### Task 5.3: Documentation Update
- Update main README.md
- Add installation screenshots
- Create user guide
- Update troubleshooting

#### Task 5.4: Final Polish
- Code formatting (rustfmt)
- Linting (clippy)
- Test suite updates
- Changelog entry

**Deliverables**:
- Fully tested integration
- Robust error handling
- Complete documentation
- Production-ready code

---

## 🎨 VISUAL FLOW DIAGRAM

```
╔════════════════════════════════════════════════════════════╗
║               MASH INSTALLER - NEW FLOW                 ║
╚════════════════════════════════════════════════════════════╝

┌─────────────────────────────────────────────────────┐
│  Step 1/6: Linux Detection                            │
│  • Platform detection                                │
│  • Architecture verification                         │
│  • Dependency checking                               │
└─────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────┐
│  Step 2/6: Install Profile                           │
│  • Minimal (core tools)                              │
│  • Dev (full workstation)                            │
│  • Full (everything + extras)                       │
└─────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────┐
│  Step 3/6: Core Options                              │
│  • Argon One fan control (Pi only)                   │
│  • Powerlevel10k theme                               │
│  • Docker data-root                                  │
│  • Interactive mode                                  │
└─────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────┐
│  Step 4/6: Theme Selection  ←←← NEW!                │
│  • BBC/UNIX Retro Theme (i3 + Kitty)                 │
│  • BBC/UNIX Retro Theme + Wallpaper Pack (6000+)     │
│  • No theme changes                                 │
└─────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────┐
│  Step 5/6: Software Tiers                           │
│  • Terminal (Kitty, Alacritty, etc.)                 │
│  • Shell (Zsh, Fish, etc.)                          │
│  • Editor (Helix, Neovim, etc.)                     │
│  • ... (unchanged)                                  │
└─────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────┐
│  Step 6/6: Installation                              │
│  • Dependency installation                           │
│  • Configuration deployment                          │
│  • First-boot script setup                          │
│  • Summary & completion                              │
└─────────────────────────────────────────────────────┘
```

---

## 📦 DEPENDENCY LIST

### Runtime Dependencies
- `i3-gaps` - Tiling window manager
- `i3status` - Status bar for i3
- `i3lock` - Screen locker
- `kitty` - GPU-accelerated terminal
- `conky` - System monitor
- `feh` - Image viewer/wallpaper setter
- `python3` - Scripting language
- `python3-pip` - Python package manager
- `requests` - Python HTTP library

### Build Dependencies
- `libxcb-xinerama0-dev` - i3 build dependency
- `libxcb-keysyms1-dev` - i3 build dependency
- `libpango1.0-dev` - i3 build dependency
- `libxcb-util0-dev` - i3 build dependency
- `libxcb-icccm4-dev` - i3 build dependency

### Optional Dependencies
- `rofi` - Application launcher (recommended)
- `dunst` - Notification daemon (recommended)
- `picom` - Compositor (optional)
- `lxappearance` - GTK theme manager (optional)

---

## ⚠️ RISK ASSESSMENT

### High Risk 🔴
- **Wallhaven API Rate Limiting**: Could fail without proper API key
- **Disk Space**: 6000 wallpapers need ~10-15GB
- **Network Failures**: Large download could fail on unstable connections
- **Pi 4B Performance**: i3 + Kitty + Conky memory usage

### Medium Risk 🟡
- **Dependency Conflicts**: i3 vs existing WM
- **Configuration Overwrites**: Existing i3/Kitty configs
- **First-Boot Timing**: Script execution order
- **User Cancellation**: Mid-download interruption

### Low Risk 🟢
- **Font Availability**: Terminus is widely available
- **Color Scheme**: Tested on multiple terminals
- **TUI Flow**: Based on existing patterns
- **Documentation**: Already complete

---

## 📅 TIMELINE & MILESTONES

| Phase | Duration | Start Date | End Date | Status |
|-------|----------|------------|----------|--------|
| 1. Planning | 1 day | 2024-02-21 | 2024-02-21 | ✅ Complete |
| 2. Core Integration | 1 day | 2024-02-22 | 2024-02-22 | ⏳ Pending |
| 3. Theme Integration | 1 day | 2024-02-23 | 2024-02-23 | ⏳ Pending |
| 4. TUI Reorganization | 1 day | 2024-02-24 | 2024-02-24 | ⏳ Pending |
| 5. Testing & Polish | 1 day | 2024-02-25 | 2024-02-25 | ⏳ Pending |

**Total Estimated**: 5 days
**Buffer**: 2 days
**Target Completion**: 2024-02-27

---

## ✅ ACCEPTANCE CRITERIA

### Minimum Viable Integration
- [ ] Wallpaper downloader available in software tiers
- [ ] Retro theme installable as option
- [ ] Hyprland removed from options
- [ ] Basic error handling implemented
- [ ] Documentation updated

### Complete Integration
- [ ] All dependencies automatically installed
- [ ] Theme + wallpapers work together
- [ ] First-boot scripts configured
- [ ] TUI flow reorganized
- [ ] Full test suite passing
- [ ] Pi 4B performance verified

### Gold Standard
- [ ] User can select theme independently
- [ ] Wallpaper categories selectable
- [ ] Progress tracking during download
- [ ] Resume support for interrupted downloads
- [ ] Comprehensive error recovery
- [ ] Professional polish

---

## 🛠️ IMPLEMENTATION CHECKLIST

### Before Starting
- [x] Create SHAFTB.MD (this document)
- [x] Audit current codebase
- [x] Review wallpaper downloader
- [x] Test retro theme configs
- [ ] Get Wallhaven API key
- [ ] Backup current installer

### Phase 2: Core Integration
- [ ] Add wallpaper to software tiers
- [ ] Add theme to software tiers
- [ ] Create package dependency list
- [ ] Implement basic installation logic
- [ ] Test build compilation
- [ ] Verify menu selection works

### Phase 3: Theme Integration
- [ ] Implement dependency checking
- [ ] Create config deployment logic
- [ ] Remove Hyprland references
- [ ] Add i3 as default WM option
- [ ] Test theme installation
- [ ] Verify config files deployed

### Phase 4: TUI Reorganization
- [ ] Design new flow diagram
- [ ] Implement theme selection menu
- [ ] Reorder existing menus
- [ ] Update menu navigation
- [ ] Test flow progression
- [ ] Verify all options accessible

### Phase 5: Testing & Polish
- [ ] Test on Raspberry Pi 4B
- [ ] Verify memory usage
- [ ] Test wallpaper download
- [ ] Check error handling
- [ ] Update documentation
- [ ] Run full test suite

---

## 📚 REFERENCE DOCUMENTS

- **Wallpaper Downloader**: `docs/incoming-files/wallpaper_downloader_README.md`
- **Retro Theme Configs**: `docs/incoming-files/kitty.txt`, `docs/incoming-files/software_tiers.md`
- **Bard's Profile**: `docs/bard-bbs-profile.md`
- **Installer Architecture**: `docs/ARCH.md` (legacy)

---

## 🎯 SUCCESS METRICS

### Quantitative
- **Integration Time**: ≤ 5 days
- **Code Changes**: ≤ 500 lines
- **Test Coverage**: ≥ 90%
- **Disk Usage**: ≤ 15GB for full install
- **Memory Usage**: ≤ 500MB during download

### Qualitative
- **User Experience**: "Intuitive and logical flow"
- **Performance**: "Smooth on Raspberry Pi 4B"
- **Reliability**: "Downloads complete without errors"
- **Aesthetics**: "Authentic retro-futuristic feel"
- **Documentation**: "Clear and comprehensive"

---

## 🔮 FUTURE ENHANCEMENTS

### Post-Launch Features
- **Theme Preview**: Show screenshots before installation
- **Category Selection**: Choose specific wallpaper categories
- **Download Progress**: Real-time progress bar
- **API Key Management**: GUI for API key entry
- **Theme Customization**: Color scheme editor

### Long-Term Roadmap
- **Additional Themes**: More retro styles
- **Wallpaper Management**: GUI for browsing/selecting
- **Cloud Sync**: Backup wallpaper collections
- **Community Themes**: User-submitted configurations
- **Theme Marketplace**: Download additional themes

---

## 📝 DECISION LOG

### 2024-02-21: Initial Planning
**Decision**: Create comprehensive integration plan before coding
**Rationale**: Ensure all requirements captured, avoid rework
**Owner**: Bard

### 2024-02-21: Category Selection
**Decision**: Merge all 8 categories (Claude's 6 + Bard's 2)
**Rationale**: Maximize user choice while maintaining retro focus
**Owner**: Bard

### 2024-02-21: Hyprland Removal
**Decision**: Replace Hyprland with i3-gaps as default
**Rationale**: Better Pi 4B compatibility, X11 stability
**Owner**: Bard

### 2024-02-21: TUI Flow
**Decision**: Add dedicated theme selection step
**Rationale**: Clear separation of concerns, better UX
**Owner**: Bard

---

## 🙏 STAKEHOLDERS

- **Bard**: Lead developer, documentation
- **Claude**: Original wallpaper downloader architecture
- **MASH Users**: Target audience
- **Raspberry Pi Community**: Primary platform
- **Retro Computing Enthusiasts**: Theme inspiration

---

## 🎉 COMPLETION CRITERIA

This project will be considered complete when:

1. ✅ Wallpaper downloader integrated into software tiers
2. ✅ Retro theme installable as option
3. ✅ Hyprland removed from installer
4. ✅ TUI flow reorganized logically
5. ✅ All dependencies handled automatically
6. ✅ First-boot scripts configured
7. ✅ Documentation complete and accurate
8. ✅ Tests passing on Raspberry Pi 4B
9. ✅ User acceptance testing complete
10. ✅ Merged into main branch

---

**🍺 PLAN APPROVED & READY FOR EXECUTION!**
*Next Step: Begin Phase 2 - Core Integration*
*Estimated Start: 2024-02-22*
*Target Completion: 2024-02-27*

---

*Document Status: ACTIVE* 🟢
*Last Updated: 2024-02-21*
*Version: 1.0*
