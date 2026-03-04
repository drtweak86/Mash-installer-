# Shaft H: New Installer Flow Design

## 🎯 Flow Overview

The new installer flow is designed to be human-friendly, logical, and informative. It breaks down the installation process into clear, manageable screens with proper information and confirmation at each step.

## 📱 Visual Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    MASH INSTALLER                          │
│                 "Retro-Futuristic Forging"                │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│                    WELCOME SCREEN                           │
│  "Welcome to the MASH Forge! Let's build something great."│
│  [Start Installation]  [Advanced Options]  [Quit]          │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│                  SYSTEM COMPATIBILITY CHECK                 │
│  ✓ Distribution: Fedora 40 (detected)                       │
│  ✓ Architecture: x86_64                                    │
│  ✓ Disk Space: 45GB available                              │
│  ✓ Internet Connection: Active                             │
│  [Continue]  [View Details]  [Back]                        │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│                    FONT SELECTION                           │
│  "Choose your terminal font - the foundation of your forge"│
│  ▼ Nerd Fonts Available (35)                                 │
│  • JetBrains Mono Nerd Font (Recommended)                   │
│  • Fira Code Nerd Font                                      │
│  • Cascadia Code Nerd Font                                 │
│  • Terminus Nerd Font                                       │
│  • Hack Nerd Font                                           │
│  [Search: ______]  [Preview]  [Continue]  [Back]           │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│               DESKTOP ENVIRONMENT (Optional)                 │
│  "Select a desktop environment for graphical interface"     │
│  ⚠️  Raspberry Pi detected - X11 recommended for performance │
│  ▼ Available Desktop Environments:                           │
│  • KDE Plasma (X11) - Full-featured desktop                │
│  • GNOME (X11) - Modern and user-friendly                   │
│  • Xfce (X11) - Lightweight (Recommended for Pi)           │
│  • MATE (X11) - Traditional and stable                     │
│  • Hyprland (Wayland) - Tiling window manager               │
│  • None - Server/CLI only                                    │
│  [Show Wayland Options]  [Continue]  [Back]                  │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│                  CORE COMPONENTS                            │
│  "Essential tools for any modern system"                    │
│  ✓ Terminal Tools (kitty, tmux, neovim)                     │
│  ✓ Shell Utilities (eza, bat, ripgrep)                      │
│  ✓ Development Tools (git, cargo, python)                    │
│  ✓ System Utilities (htop, btop, ncdu)                     │
│  [Customize]  [Continue]  [Back]                             │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│                  WALLPAPER PACKS (Optional)                  │
│  "Retro-futuristic wallpapers for your desktop"             │
│  ✓ Retro Computing (1000 wallpapers)                        │
│  ✓ Classic Games (1000 wallpapers)                          │
│  ✓ Anime (625 wallpapers)                                   │
│  □ DC Comics (625 wallpapers)                                │
│  □ Marvel (625 wallpapers)                                  │
│  □ Judge Dredd (562 wallpapers)                             │
│  □ Star Wars (562 wallpapers)                               │
│  □ Cyberpunk (1000 wallpapers)                              │
│  [Select All]  [Select None]  [Continue]  [Back]              │
│  ⚠️  This will download ~2.5GB of wallpapers                │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│                  CONFIRMATION                                │
│  "Ready to forge your system!"                              │
│  • Font: JetBrains Mono Nerd Font                           │
│  • Desktop Environment: Xfce (X11)                           │
│  • Core Components: Standard set                             │
│  • Wallpapers: Retro, Games, Anime (2625 total)             │
│  • Estimated Time: 12-18 minutes                             │
│  • Estimated Disk Space: ~3.2GB                              │
│  [Start Installation]  [Back]                                │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│                  INSTALLATION IN PROGRESS                    │
│  "Forging your system..."                                    │
│  [■■■■■■■■■■■■■■■■□□□□□□□□] 65% Complete              │
│                                                                      │
│  📦 Currently: Installing Xfce packages (4/12)                │
│  ⏱️  Estimated time remaining: 7 minutes                     │
│  💾 Disk space used: 1.8GB / 3.2GB                            │
│  🔧 Next: Configuring display manager                         │
│                                                                      │
│  [Pause]  [View Log]  [Cancel]                               │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│                  LONG PROCESS CONFIRMATION                   │
│  ⚠️  Wallpaper download will take approximately 8-12 minutes   │
│  "This is a good time to grab a beverage! 🍺"                │
│  [Start Download]  [Skip]  [Cancel Installation]             │
│  Auto-continue in: 10 seconds                                │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│                  INSTALLATION COMPLETE                       │
│  "✅ Your system has been successfully forged!"              │
│                                                                      │
│  🎉 Success! All components installed                        │
│  ⏱️  Total time: 14 minutes 27 seconds                       │
│  💾 Total disk space used: 2.9GB                             │
│                                                                      │
│  📋 Summary:                                                  │
│  • Font: JetBrains Mono Nerd Font installed                  │
│  • Desktop: Xfce with SDDM display manager                   │
│  • Wallpapers: 2625 retro-futuristic wallpapers installed    │
│  • Tools: 47 packages installed and configured                │
│                                                                      │
│  💡 Next Steps:                                              │
│  1. Reboot your system to start the desktop environment      │
│  2. Run 'mash-doctor' to verify your installation            │
│  3. Explore the retro wallpapers in ~/Pictures/RetroWallpapers/│
│                                                                      │
│  [Reboot Now]  [View Log]  [Exit to Shell]                   │
└─────────────────────────────────────────────────────────────┘
```

## 🎨 Key Design Principles

### 1. **Logical Progression**
- Start with fundamental choices (fonts)
- Move to major components (DEs)
- Then to core utilities
- Finally to optional extras (wallpapers)

### 2. **Information Hierarchy**
- **Primary Info**: Large, clear text at top
- **Secondary Info**: Bullet points with details
- **Tertiary Info**: Bottom info box with real-time updates
- **Warnings**: Yellow highlight with ⚠️ icon

### 3. **User Control**
- Always provide Back button (except where it doesn't make sense)
- Clear Continue/Start buttons
- Explicit confirmation for destructive or long operations
- Progress tracking and cancellation options

### 4. **Transparency**
- Show what's being installed
- Provide time estimates
- Display progress percentages
- Show disk space usage
- Provide next steps

### 5. **Personality**
- Friendly, approachable language
- Occasional humor ("grab a beverage")
- Retro-futuristic theme consistent throughout
- Encouraging messages ("Let's build something great")

## 🔧 Technical Implementation

### Screen Structure
```rust
struct InstallerScreen {
    title: String,
    description: String,
    options: Vec<MenuOption>,
    info_box: InfoBox,
    navigation: NavigationButtons,
    status: ScreenStatus,
}

struct InfoBox {
    current_operation: String,
    progress: u8, // 0-100
    time_remaining: Option<Duration>,
    disk_usage: Option<String>,
    next_operation: Option<String>,
    warnings: Vec<String>,
}
```

### Navigation Flow
```rust
enum InstallerState {
    Welcome,
    CompatibilityCheck,
    FontSelection,
    DesktopEnvironment,
    CoreComponents,
    OptionalExtras,
    Confirmation,
    InstallationInProgress,
    LongProcessConfirmation,
    Complete,
}
```

### Time Estimation Logic
```rust
struct TimeEstimator {
    base_estimates: HashMap<InstallationComponent, Duration>,
    current_distro: Distro,
    network_speed: NetworkSpeed,
}

impl TimeEstimator {
    fn estimate_component(&self, component: &InstallationComponent) -> Duration {
        // Adjust base estimate based on distro and network speed
    }
    
    fn update_from_actual(&mut self, component: &InstallationComponent, actual: Duration) {
        // Learn from actual times to improve future estimates
    }
}
```

## 📱 Responsive Design Considerations

### Minimum Terminal Size
- **Width**: 80 characters (show warning if smaller)
- **Height**: 24 lines (scrollable if smaller)

### Adaptive Layout
- Hide secondary info on small screens
- Simplify progress display on narrow terminals
- Use abbreviations when space is limited
- Provide scrollable areas for long lists

### Color Scheme
- **Primary**: Retro green on black (default)
- **Secondary**: White for important info
- **Warnings**: Yellow with ⚠️ icon
- **Errors**: Red with ✗ icon
- **Success**: Green with ✓ icon
- **Progress**: Blue progress bars

## 🎯 User Experience Enhancements

### 1. **Smart Defaults**
- Recommend JetBrains Mono for fonts
- Suggest Xfce for Raspberry Pi
- Pre-select sensible core components
- Remember user choices for re-runs

### 2. **Contextual Help**
- Tooltips on hover (where supported)
- Detailed descriptions for each option
- "What does this do?" explanations
- Link to online documentation

### 3. **Progress Feedback**
- Real-time operation updates
- Accurate time remaining estimates
- Visual progress bars
- Step-by-step breakdown

### 4. **Error Handling**
- Clear error messages (no cryptic codes)
- Suggested solutions
- Option to retry or skip
- Log file generation

### 5. **Accessibility**
- Keyboard-only navigation
- Screen reader friendly
- High contrast mode
- Adjustable font sizes

## 📋 Implementation Checklist

### UI Components to Create
- [ ] Welcome screen with version info
- [ ] System compatibility check screen
- [ ] Font selection with live preview
- [ ] DE selection with X11/Wayland toggle
- [ ] Component selection with categories
- [ ] Confirmation screen with summary
- [ ] Installation progress screen
- [ ] Long process confirmation dialog
- [ ] Completion screen with next steps

### Backend Components to Create
- [ ] Font management system
- [ ] DE installation logic
- [ ] Package mapping database
- [ ] Time estimation engine
- [ ] Progress tracking system
- [ ] Installation logging
- [ ] Error handling framework

### Integration Points
- [ ] Connect font selection to installation
- [ ] Link DE selection to package installer
- [ ] Wire up progress tracking to UI
- [ ] Integrate time estimation
- [ ] Connect confirmation dialogs
- [ ] Implement cancellation logic

## 🎯 Success Metrics for Flow Design

### Usability Metrics
- ✅ Users can complete installation without confusion
- ✅ Navigation is intuitive and logical
- ✅ Information is always available when needed
- ✅ Users feel in control of the process
- ✅ Errors are handled gracefully

### Technical Metrics
- ✅ Flow works on terminals 80x24 and larger
- ✅ Navigation is responsive (<100ms between screens)
- ✅ Progress updates in real-time
- ✅ Time estimates accurate within 25%
- ✅ Memory usage <50MB

### User Satisfaction Metrics
- ✅ Users report feeling informed throughout process
- ✅ Installation feels professional and polished
- ✅ Errors are understandable and actionable
- ✅ Completes successfully on first try
- ✅ Users would recommend to others

## 📝 Conclusion

The new installer flow represents a significant improvement in user experience while maintaining the technical robustness and cross-distro compatibility that MASH is known for. By breaking down the installation into logical screens, providing clear information at each step, and giving users control over the process, we create an installer that is both powerful and approachable.

The design follows modern UX principles while maintaining the retro-futuristic aesthetic that defines MASH. With clear navigation, comprehensive information display, and robust error handling, this flow will set a new standard for Linux system installers.

"*A good installer is like a well-tuned engine - powerful when you need it, smooth when you don't, and always getting you where you want to go.*" — Bard 🍺⚒️