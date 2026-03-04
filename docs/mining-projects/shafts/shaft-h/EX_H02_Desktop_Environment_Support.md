# EX_H02: Desktop Environment Support

**Status**: ⏳ PENDING
**Priority**: HIGH
**Dependencies**: EX_H01 (Font Management - for font consistency)

## 🎯 OBJECTIVE
Implement comprehensive desktop environment installation support with X11/Wayland options, Raspberry Pi specific recommendations, and cross-distro compatibility.

## 📋 DETAILED STEPS

### 1. Research and Design (1 day)
- [ ] Research package names for each DE across distros
- [ ] Design DE selection UI mockup
- [ ] Create data structures for DE metadata
- [ ] Design installation verification system

### 2. Create desktop_environments.rs Module (3 days)
- [ ] Create new file: `installer-core/src/desktop_environments.rs`
- [ ] Define `DesktopEnvironment` enum with variants for each DE
- [ ] Define `DisplayServer` enum (X11, Wayland)
- [ ] Create `DEPackageMapping` struct for cross-distro support
- [ ] Implement `list_available_des()` function
- [ ] Implement `install_de(de: DesktopEnvironment, display_server: DisplayServer) -> Result<()>`
- [ ] Add Raspberry Pi specific logic and warnings
- [ ] Implement installation verification

### 3. Create Package Mapping Database (2 days)
- [ ] Create `installer-core/src/de_packages.rs`
- [ ] Define package lists for each DE/distro combination
- [ ] Add Fedora package mappings
- [ ] Add Debian/Ubuntu package mappings
- [ ] Add Arch Linux package mappings
- [ ] Add OpenSUSE package mappings
- [ ] Create fallback logic for unknown distros

### 4. Create DE Selection UI (2 days)
- [ ] Add DE selection screen to installer flow
- [ ] Implement DE listing with descriptions
- [ ] Add X11/Wayland toggle with warnings
- [ ] Implement Raspberry Pi specific recommendations
- [ ] Add confirmation dialog with DE details and warnings
- [ ] Implement progress tracking for DE installation

### 5. Implement Installation Logic (2 days)
- [ ] Add pre-installation checks (disk space, existing DE)
- [ ] Implement package installation with progress
- [ ] Add display manager configuration
- [ ] Implement post-installation configuration
- [ ] Add error handling and rollback capability
- [ ] Implement idempotency checks

### 6. Integration and Testing (2 days)
- [ ] Integrate DE selection into main installer flow
- [ ] Add unit tests for desktop_environments.rs
- [ ] Test DE installation on different distros
- [ ] Verify X11/Wayland selection works correctly
- [ ] Test Raspberry Pi specific behavior

## 🔧 TECHNICAL DETAILS

### Data Structures
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DesktopEnvironment {
    Kde,
    Gnome,
    Cosmic,
    Xfce,
    Mate,
    Hyprland,
    // Add more as needed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DisplayServer {
    X11,
    Wayland,
}

struct DesktopEnvironmentInfo {
    name: String,
    display_name: String,
    description: String,
    recommended_display_server: DisplayServer,
    raspberry_pi_recommended: bool,
    package_groups: Vec<String>,
    min_disk_space: u64, // in MB
}

struct DEPackageMapping {
    fedora: Vec<String>,
    debian: Vec<String>,
    arch: Vec<String>,
    opensuse: Vec<String>,
    fallback: Vec<String>,
}
```

### Key Functions
- `list_available_des() -> Vec<DesktopEnvironmentInfo>` - Get available DEs
- `get_de_info(de: DesktopEnvironment) -> DesktopEnvironmentInfo` - Get DE details
- `get_packages_for_de(de: DesktopEnvironment, distro: Distro) -> Vec<String>` - Get packages
- `install_de(de: DesktopEnvironment, display_server: DisplayServer) -> Result<()>` - Install
- `is_de_installed(de: DesktopEnvironment) -> bool` - Check installation
- `configure_display_manager(de: DesktopEnvironment) -> Result<()>` - Configure DM
- `get_raspberry_pi_recommendation(de: DesktopEnvironment) -> Option<String>` - Pi advice

### Raspberry Pi Specific Logic
- Prefer X11 over Wayland for all DEs on Raspberry Pi
- Add performance warnings for Wayland on Pi
- Recommend lightweight DEs (Xfce, MATE) for Pi
- Handle Pi-specific display configuration

### Cross-Distro Package Mapping
```rust
match distro {
    Distro::Fedora => vec!["@kde-desktop", "sddm"],
    Distro::Debian | Distro::Ubuntu => vec!["kde-plasma-desktop", "sddm"],
    Distro::Arch => vec!["plasma", "sddm"],
    Distro::OpenSuse => vec!["-pattern kde kde_plasma", "sddm"],
    _ => vec!["kde-workspace", "sddm"] // fallback
}
```

### Installation Process
1. **Pre-checks**: Disk space, existing DE, distro compatibility
2. **Package Installation**: Install DE packages using distro package manager
3. **Display Server**: Configure X11 or Wayland as selected
4. **Display Manager**: Set up and enable SDDM/LightDM/GDM
5. **Configuration**: Apply DE-specific configurations
6. **Verification**: Check installation success
7. **Post-install**: Cleanup and final configuration

## ✅ VERIFICATION

### Unit Tests
- [ ] Test DE listing and information
- [ ] Test package mapping for different distros
- [ ] Test Raspberry Pi recommendation logic
- [ ] Test display server selection logic
- [ ] Test error scenarios and fallbacks

### Integration Tests
- [ ] DE selection UI works correctly
- [ ] Package mapping returns correct packages for current distro
- [ ] Installation progress is tracked and displayed
- [ ] Raspberry Pi warnings are shown appropriately
- [ ] X11/Wayland selection is preserved

### Manual Testing
- [ ] Install KDE Plasma with X11 on Fedora
- [ ] Install GNOME with Wayland on Ubuntu
- [ ] Install Xfce with X11 on Raspberry Pi (with warnings)
- [ ] Test DE installation idempotency
- [ ] Verify display manager starts correctly
- [ ] Test error handling for failed installations

### Cross-Distro Testing
- [ ] Test on Fedora
- [ ] Test on Ubuntu/Debian
- [ ] Test on Arch Linux
- [ ] Test on OpenSUSE (if possible)
- [ ] Verify package mapping works correctly

## 📝 NOTES

- Wayland may have performance issues on Raspberry Pi - warn users
- Some DEs may not support Wayland on all distros - handle gracefully
- DE installation can take significant time - implement progress tracking
- Consider bandwidth usage for downloading DE packages
- Ensure proper cleanup on installation failure
- Test with both fresh installations and existing DE upgrades

## 🔗 REFERENCES

- Fedora KDE: https://docs.fedoraproject.org/en-US/quick-docs/installing-kde/
- Ubuntu DEs: https://ubuntu.com/download/flavours
- Arch Wiki DEs: https://wiki.archlinux.org/title/Desktop_environment
- Raspberry Pi DE recommendations: https://www.raspberrypi.com/documentation/computers/desktop.html