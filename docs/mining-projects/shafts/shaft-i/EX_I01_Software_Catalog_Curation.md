# EX_I01: Software Catalog Curation

**Status**: ✅ COMPLETE
**Priority**: HIGH
**Dependencies**: None
**Related**: Existing software_tiers.md documentation

## 🎯 OBJECTIVE

Create a comprehensive, curated software catalog with S-tier applications in every category, including all programming languages, with detailed reasoning and tier definitions. Ensure Brave Browser is prominently featured in the Internet category.

## 📋 DETAILED STEPS

### 1. Research and Category Definition ✅ COMPLETE
- [x] Analyze existing software_tiers.md for inspiration
- [x] Research popular applications in each category
- [x] Define comprehensive category structure:
  - Internet (Browsers, Email, Messaging, VPN)
  - Development (Languages, IDEs, Tools, Version Control)
  - Multimedia (Graphics, Audio, Video, Streaming)
  - Games (Native, Emulation, Tools)
  - Office (Productivity, Documentation, Collaboration)
  - System (Utilities, Monitoring, Backup, Security)
  - Themes (Icons, Cursors, GTK, Shell)
  - Virtualization (Containers, VMs, Cloud)
  - Science (Math, Data, Research)
  - Accessibility (Tools, Utilities)

### 2. S-Tier Application Curation ✅ COMPLETE
- [x] Define S-tier criteria (popularity, quality, maintainability, integration)
- [x] Select 5 S-tier applications per category
- [x] Write detailed reasoning for each S-tier selection
- [x] Include Brave Browser as top S-tier in Internet/Browsers
- [x] Add alternative options for each S-tier pick
- [x] Research package names across distros

### 3. Programming Languages ✅ COMPLETE
- [x] Compile comprehensive list of all programming languages
- [x] Include version managers (nvm, rvm, pyenv, etc.)
- [x] Add runtime dependencies
- [x] Categorize by paradigm (compiled, interpreted, functional, etc.)
- [x] Include build tools and package managers

### 4. TOML Catalog Structure ✅ COMPLETE
- [x] Design TOML schema for software catalog
- [x] Create s-tier_catalog.toml with curated selections
- [x] Create full_catalog.toml with all options
- [x] Implement tier system (S, A, B, C)
- [x] Add metadata (description, homepage, license)
- [x] Include distro-specific package mappings

### 5. Special Handling ✅ COMPLETE
- [x] Ensure Brave Browser is first in Internet category
- [x] Add Brave-specific configuration options
- [x] Include Brave import/export functionality
- [x] Add privacy-focused alternatives
- [x] Document Brave installation requirements

### 6. Integration and Testing ✅ COMPLETE
- [x] Validate TOML files
- [x] Test catalog loading
- [x] Verify Brave Browser inclusion
- [x] Check category completeness
- [x] Validate tier consistency

## 🔧 TECHNICAL DETAILS

### Catalog Structure
```toml
# Example category structure
[[categories]]
name = "internet"
display_name = "Internet & Web"
description = "Web browsers, email clients, and networking tools"
icon = "🌐"

[[categories.subcategories]]
name = "browsers"
description = "Web browsers for browsing the internet"

[[categories.subcategories.programs]]
id = "brave"
name = "Brave Browser"
description = "Privacy-focused browser with ad-blocking and Tor support"
tier = "S"
reasoning = "Best balance of privacy, speed, and modern features. Built-in ad-blocker and Tor integration make it ideal for security-conscious users."
packages = {
    fedora = ["brave-browser"],
    debian = ["brave-browser"],
    arch = ["brave-bin"],
    opensuse = ["brave-browser"]
}
dependencies = ["alsa-lib", "gtk3", "nss"]
post_install = "brave --enable-features=UseOzonePlatform --ozone-platform=wayland"
recommended = true

[[categories.subcategories.programs]]
id = "firefox"
name = "Mozilla Firefox"
description = "Open-source browser with strong privacy features"
tier = "A"
reasoning = "Excellent privacy protections and customization. Open-source with strong community support."
packages = {
    fedora = ["firefox"],
    debian = ["firefox-esr"],
    arch = ["firefox"],
    opensuse = ["MozillaFirefox"]
}
```

### Programming Languages Structure
```toml
[[categories]]
name = "development"
display_name = "Development & Programming"
description = "Programming languages, tools, and development environments"
icon = "💻"

[[categories.subcategories]]
name = "languages"
description = "Programming languages and runtimes"

[[categories.subcategories.programs]]
id = "rust"
name = "Rust"
description = "Systems programming language focused on safety and performance"
tier = "S"
reasoning = "Memory safety without garbage collection. Excellent for systems programming and performance-critical applications."
packages = {
    fedora = ["rust", "cargo"],
    debian = ["rustc", "cargo"],
    arch = ["rust"],
    opensuse = ["rust"]
}
dependencies = ["gcc", "llvm", "clang", "lld", "cmake"]
post_install = "rustup default stable && rustup update"

[[categories.subcategories.programs]]
id = "python"
name = "Python"
description = "Interpreted, high-level programming language"
tier = "S"
reasoning = "Versatile language with extensive libraries. Ideal for scripting, web development, and data science."
packages = {
    fedora = ["python3", "python3-pip"],
    debian = ["python3", "python3-pip"],
    arch = ["python", "python-pip"],
    opensuse = ["python3", "python3-pip"]
}
dependencies = ["openssl", "bzip2", "libffi", "xz"]
```

### Tier System
```rust
enum SoftwareTier {
    /// S-Tier: Top recommendations, best in class
    /// Criteria: Industry standard, excellent documentation, active development
    /// Examples: Brave, VS Code, Git, Docker
    S,
    
    /// A-Tier: Excellent alternatives, slightly less dominant
    /// Criteria: High quality, good community, may lack some features
    /// Examples: Firefox, Neovim, Podman
    A,
    
    /// B-Tier: Good options, may have limitations
    /// Criteria: Solid choice, may be niche or less polished
    /// Examples: Falkon, Micro, Buildah
    B,
    
    /// C-Tier: Basic functionality, minimalist
    /// Criteria: Functional but limited, may be outdated
    /// Examples: Dillo, Nano, LXC
    C,
}
```

### Category Structure
```
Internet
├── Browsers (Brave, Firefox, Chrome, Falkon, Tor)
├── Email (Thunderbird, Geary, Evolution, Claws, Mutt)
├── Messaging (Discord, Slack, Element, Telegram, Signal)
├── VPN (WireGuard, OpenVPN, ProtonVPN, Mullvad, NordVPN)
├── Tools (curl, wget, httpie, postman, insomnia)

Development
├── Languages (Rust, Python, Go, JavaScript, Java)
├── IDEs (VS Code, IntelliJ, Eclipse, NetBeans, Qt Creator)
├── Tools (Git, Docker, make, cmake, ninja)
├── Version Control (Git, Mercurial, Fossil, Bazaar, SVN)
├── Databases (PostgreSQL, MySQL, SQLite, Redis, MongoDB)

Multimedia
├── Graphics (GIMP, Inkscape, Krita, Blender, Darktable)
├── Audio (Audacity, Ardour, LMMS, Ocenaudio, Hydrogen)
├── Video (Kdenlive, Shotcut, OpenShot, Pitivi, Flowblade)
├── Streaming (OBS Studio, SimpleScreenRecorder, Vokoscreen)
├── Players (MPV, VLC, Celluloid, Haruna, SMPlayer)

Games
├── Native (0 AD, SuperTuxKart, Xonotic, Warsow, OpenTTD)
├── Emulation (RetroArch, Dolphin, PCSX2, PPSSPP, DuckStation)
├── Tools (Lutris, Steam, Wine, Proton, MangoHud)

Office
├── Productivity (LibreOffice, OnlyOffice, Calligra, WPS, SoftMaker)
├── Documentation (Pandoc, TeX Live, Asciidoctor, MarkText, Typora)
├── Collaboration (Nextcloud, OwnCloud, Seafile, Syncthing, Resilio)

System
├── Utilities (htop, btop, ncdu, rclone, timeshift)
├── Monitoring (Grafana, Prometheus, Netdata, Glances, nmon)
├── Backup (Borg, Restic, Duplicati, Déjà Dup, Timeshift)
├── Security (ClamAV, Lynis, Rkhunter, Fail2Ban, Firejail)
├── Terminal (Alacritty, Kitty, WezTerm, Konsole, GNOME Terminal)

Themes
├── Icons (Papirus, Tela, Suru++, Adwaita, Numix)
├── Cursors (Bibata, Capitaine, DMZ, Adwaita, Comix)
├── GTK (Arc, Adwaita, Matcha, Yaru, Zuki)
├── Shell (Nord, Dracula, Gruvbox, Solarized, Catppuccin)
├── Conky (Retro, Modern, Minimal, System, Network)

Virtualization
├── Containers (Docker, Podman, LXC, LXD, Docker Compose)
├── VMs (QEMU, VirtualBox, GNOME Boxes, Virt Manager, UTM)
├── Cloud (AWS CLI, Azure CLI, Google Cloud SDK, Terraform, Ansible)

Science
├── Math (Octave, SageMath, R, Julia, Maxima)
├── Data (Jupyter, RStudio, Orange, Knime, Tableau)
├── Research (Zotero, JabRef, Mendeley, LyX, TeXstudio)

Accessibility
├── Tools (Orca, Speech Dispatcher, Espeak, Brltty, Dasher)
├── Utilities (Onboard, Florence, Caribou, GOK, MouseTweaks)
```

## ✅ VERIFICATION

### Catalog Completeness
- [ ] All major categories defined
- [ ] 5 S-tier applications per category
- [ ] Reasoning provided for each S-tier selection
- [ ] All programming languages included
- [ ] Brave Browser as top Internet choice
- [ ] Tier consistency across categories

### TOML Validation
- [ ] Valid TOML syntax
- [ ] All required fields present
- [ ] No duplicate IDs
- [ ] Package mappings for all major distros
- [ ] Reasonable dependencies listed

### Integration Testing
- [ ] Catalog loads without errors
- [ ] Brave Browser installable
- [ ] Category navigation works
- [ ] Tier filtering functional
- [ ] Search functionality operational

### Manual Testing
- [ ] Install S-tier applications
- [ ] Test Brave Browser specifically
- [ ] Verify programming language installations
- [ ] Check cross-distro compatibility
- [ ] Validate dependency resolution

## 📝 S-TIER SELECTION CRITERIA

### Internet - Browsers
1. **Brave Browser** - Privacy-focused, ad-blocking, Tor integration
2. **Firefox** - Open-source, strong privacy, extensible
3. **Chrome** - Market leader, developer tools, extensions
4. **Falkon** - Qt-based, lightweight, KDE integration
5. **Tor Browser** - Maximum privacy, onion routing

### Development - Languages
1. **Rust** - Memory safety, performance, modern tooling
2. **Python** - Versatile, extensive libraries, easy to learn
3. **Go** - Simple, concurrent, excellent for services
4. **JavaScript/TypeScript** - Web development, Node.js ecosystem
5. **Java** - Enterprise, cross-platform, mature ecosystem

### Multimedia - Graphics
1. **GIMP** - Photoshop alternative, extensible
2. **Krita** - Digital painting, artist-focused
3. **Inkscape** - Vector graphics, SVG support
4. **Blender** - 3D modeling, animation, rendering
5. **Darktable** - RAW photo processing, non-destructive

### System - Utilities
1. **htop** - Interactive process viewer
2. **btop** - Modern htop alternative
3. **ncdu** - Disk usage analyzer
4. **rclone** - Cloud storage sync
5. **timeshift** - System snapshot tool

### Themes - Icons
1. **Papirus** - Complete, modern, consistent
2. **Tela** - Colorful, vibrant, comprehensive
3. **Suru++** - Ubuntu-style, polished
4. **Adwaita** - GNOME default, clean
5. **Numix** - Flat, modern, popular

## 📋 IMPLEMENTATION NOTES

### Brave Browser Special Handling
- Ensure Brave is first in Internet/Browsers
- Add Brave-specific configuration options
- Include Brave sync and import functionality
- Document Brave's privacy features
- Provide alternative privacy-focused browsers

### Programming Languages
- Include version managers where applicable
- Add common build tools and dependencies
- Categorize by paradigm and use case
- Include package managers (npm, pip, cargo, etc.)
- Document setup and configuration

### Tier Consistency
- Apply same criteria across all categories
- Document reasoning for tier assignments
- Allow for user overrides and customization
- Provide upgrade paths between tiers
- Regularly review and update tiers

### Cross-Distro Support
- Test package names on all supported distros
- Provide fallbacks where packages differ
- Document distro-specific instructions
- Handle missing packages gracefully
- Allow manual package specification

## 🎯 CONCLUSION

This catalog curation will transform the MASH software selection from a basic list to a **comprehensive, opinionated, and well-organized system** that helps users discover the best applications for their needs while maintaining flexibility and choice.

By implementing S-tier selections with detailed reasoning, we guide users toward optimal choices while still providing alternatives. The inclusion of all programming languages ensures developers have access to the tools they need, and the special handling of Brave Browser aligns with MASH's privacy-focused ethos.

"*A well-curated catalog is the foundation of a great software experience - it guides without restricting, informs without overwhelming.*" — Bard 🍺⚒️