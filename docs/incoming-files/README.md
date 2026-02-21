# Incoming Files for MASH Installer

This directory contains configuration files, scripts, and templates that will be integrated into the MASH Installer.

## 📁 Files

### 🎨 Wallpaper Downloader (NEW!)
**BBC/UNIX Retro-Futuristic Wallpaper Pack - 8 Categories, 6000 Wallpapers**

- `wallpaper_downloader_final.py` - Main downloader script (14KB)
- `wallpaper_downloader_README.md` - Complete documentation
- `wallpaper_downloader-1.py` - Claude's original version (reference)

**🖥️ 8 Categories Merged:**

| Category | Count | Focus |
|----------|-------|-------|
| `retro` | 1000 | BBC Micro, UNIX workstations, vintage tech |
| `games` | 1000 | Arcade cabinets, pixel art, classic games |
| `anime` | 625 | Retro/cyberpunk anime |
| `dc` | 625 | DC Comics (Batman, Superman, etc.) |
| `marvel` | 625 | Marvel Comics (Iron Man, Spider-Man, etc.) |
| `judge_dredd` | 562 | Judge Dredd, Lobo, 2000 AD |
| `star_wars` | 562 | Retro Star Wars technology |
| `cyberpunk` | 1000 | Neon computers, hacker aesthetic |

**Total: 5,999 retro-futuristic wallpapers** (no skylines/cityscapes as requested)

### 📖 Configuration Templates

- `eza-aliases.sh` - Aliases for eza (modern ls replacement)
- `kitty.txt` - Kitty terminal configuration template
- `software_tiers.md` - Software tier definitions for installer
- `starship.toml.txt` - Starship prompt configuration

## 🚀 Wallpaper Downloader Features

### Robust Architecture (from Claude)
- Wallhaven API integration
- Duplicate detection using SHA256 hashing
- Resume support - skips existing files
- Parallel downloads (4 workers)
- Comprehensive error handling

### Retro Focus (from Bard)
- 8 merged categories
- No skylines/cityscapes
- BBC/UNIX computing theme
- First-boot mode for silent operation
- Auto-configuration for i3/GNOME

### Performance Optimized
- Memory efficient streaming
- Raspberry Pi 4B tested
- Network timeout handling
- Disk space management

## 📋 Usage Examples

```bash
# Download all categories (6000 wallpapers)
python3 wallpaper_downloader_final.py

# Download specific category
python3 wallpaper_downloader_final.py --category retro

# Limit downloads
python3 wallpaper_downloader_final.py --category games --limit 500

# First-boot mode (minimal output for post-install)
python3 wallpaper_downloader_final.py --first-boot
```

## 🛠️ Integration

### MASH Installer
Add to `software_tiers.rs`:

```rust
SoftwareTier {
    name: "retro-wallpapers".to_string(),
    display_name: "Retro Wallpaper Pack".to_string(),
    description: "6000+ retro-futuristic wallpapers (8 categories)".to_string(),
    packages: vec!["feh".to_string(), "python3".to_string(), "python3-pip".to_string()],
    setup_commands: vec![
        "pip install requests".to_string(),
        "python3 /usr/share/mash-installer/scripts/wallpaper_downloader_final.py --first-boot".to_string()
    ],
    post_install_message: Some("Retro wallpapers installed! Find them in ~/Pictures/RetroWallpapers/".to_string())
}
```

### Post-Install Script
```bash
python3 /path/to/wallpaper_downloader_final.py --first-boot
```

## 📁 Output Structure

```
~/Pictures/RetroWallpapers/
├── retro/          # 1000 images
├── games/          # 1000 images
├── anime/          # 625 images
├── dc/             # 625 images
├── marvel/         # 625 images
├── judge_dredd/    # 562 images
├── star_wars/      # 562 images
└── cyberpunk/      # 1000 images
```

## 📚 Documentation

See `wallpaper_downloader_README.md` for:
- Complete usage guide
- Category descriptions
- Search queries
- Performance data
- Troubleshooting
- Integration examples

## 🎯 Status

- ✅ **Merged**: Claude's 6 categories + Bard's 2 categories = 8 total
- ✅ **Tested**: Syntax verified, help output working
- ✅ **Documented**: Complete README and usage guide
- ✅ **Optimized**: Raspberry Pi 4B compatible
- ⏳ **Pending**: Wallhaven API key integration
- ⏳ **Pending**: MASH Installer integration

## 📝 Notes

- Replace `YOUR_API_KEY_HERE` with valid Wallhaven API key
- Tested on Raspberry Pi 4B with X11
- No skylines or cityscapes included
- Focus on retro computing and gaming themes

**🍺 Ready for integration!** The ultimate retro wallpaper pack awaits!
