# BBC/UNIX Retro-Futuristic Wallpaper Downloader

**Final Edition - 8 Categories, 6000 Wallpapers**

## 📋 Overview

This script downloads 6000+ high-quality retro-futuristic wallpapers across 8 categories, combining Claude's robust downloader with Bard's retro computing focus.

## 🎨 Categories (8 Total)

| Category | Display Name | Count | Focus |
|----------|--------------|-------|-------|
| `retro` | Retro Computing | 1000 | BBC Micro, UNIX, vintage tech |
| `games` | Video Games | 1000 | Arcade, pixel art, classic games |
| `anime` | Anime | 625 | Retro/cyberpunk anime |
| `dc` | DC Comics | 625 | Batman, Superman, Justice League |
| `marvel` | Marvel Comics | 625 | Iron Man, Spider-Man, Avengers |
| `judge_dredd` | Judge Dredd/Lobo | 562 | 2000 AD, Mega City One |
| `star_wars` | Star Wars | 562 | Retro Star Wars tech |
| `cyberpunk` | Cyberpunk | 1000 | Neon computers, hacker aesthetic |

**Total: 5,999 wallpapers** (no skylines or cityscapes)

## 🚀 Usage

### Basic Download (All Categories)
```bash
python3 wallpaper_downloader_final.py
```

### Specific Category
```bash
python3 wallpaper_downloader_final.py --category retro
```

### Limit Downloads
```bash
python3 wallpaper_downloader_final.py --category games --limit 500
```

### First-Boot Mode (Minimal Output)
```bash
python3 wallpaper_downloader_final.py --first-boot
```

## 📁 Output Structure

```
~/Pictures/RetroWallpapers/
├── retro/
│   ├── retro_0001.jpg
│   ├── retro_0002.jpg
│   └── ... (1000 images)
├── games/
│   ├── games_0001.jpg
│   └── ... (1000 images)
├── anime/
│   └── ... (625 images)
├── dc/
│   └── ... (625 images)
├── marvel/
│   └── ... (625 images)
├── judge_dredd/
│   └── ... (562 images)
├── star_wars/
│   └── ... (562 images)
└── cyberpunk/
    └── ... (1000 images)
```

## 🎯 Features

### Robust Downloading
- **Wallhaven API** integration (real image source)
- **Duplicate detection** using SHA256 hashing
- **Resume support** - skips existing files
- **Parallel downloads** (4 workers)
- **Error handling** with automatic retries

### Smart Configuration
- **Auto-detects existing files** to avoid re-downloading
- **First-boot mode** for silent post-install operation
- **Auto-configures wallpapers** for i3 and GNOME
- **Progress tracking** with detailed logging

### Performance Optimized
- **Memory efficient** - streams downloads
- **Disk efficient** - temporary files during download
- **Network efficient** - timeout and retry logic
- **Pi-friendly** - tested on Raspberry Pi 4B

## 🔧 Requirements

### Python Packages
```bash
pip install requests
```

### System Tools
- `feh` (for i3 wallpaper management)
- `gsettings` (for GNOME wallpaper management)

### API Key
Edit the script and replace `YOUR_API_KEY_HERE` with a valid Wallhaven API key.

## 🛠️ Integration

### MASH Installer Integration
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
# Add to first-boot setup
python3 /path/to/wallpaper_downloader_final.py --first-boot
```

## 🎨 Wallpaper Management

### Set Random Wallpaper (i3)
```bash
feh --bg-scale --randomize ~/Pictures/RetroWallpapers/*/*
```

### Add to i3 Config
```bash
# Auto-rotate wallpapers every 30 minutes
exec --no-startup-id while true; do \
    feh --bg-scale --randomize ~/Pictures/RetroWallpapers/*/*; \
    sleep 1800; \
done
```

### Set Specific Category
```bash
feh --bg-scale --randomize ~/Pictures/RetroWallpapers/retro/*
```

## 🔍 Search Queries

### Retro Category
- retro computer, bbc micro, unix workstation
- vintage tech, old computer, 80s computer
- 90s computer, amiga, commodore 64
- apple ii, terminal, command line
- text mode, green screen

### Games Category
- retro video games, arcade cabinets
- pixel art, 8-bit games, 16-bit games
- classic video games, retro gaming
- videogame art, game consoles
- nintendo, sega, atari, playstation 1

### Cyberpunk Category
- cyberpunk, cyberpunk computers
- cyberpunk terminals, neon computers
- retro futurism, cyberpunk technology
- hacker aesthetic, terminal aesthetic

## ⚙️ Configuration

### Wallhaven API
Edit these constants in the script:

```python
WALLHAVEN_API_KEY = "YOUR_API_KEY_HERE"
MAX_WORKERS = 4  # Parallel downloads
TIMEOUT = 30  # Seconds per download
```

### Download Limits
Adjust category counts as needed:

```python
CATEGORIES = {
    "retro": {"count": 1000},
    "games": {"count": 1000},
    # ... etc
}
```

## 📊 Performance

### Estimated Download Times
- **100 images**: ~2-5 minutes
- **1000 images**: ~30-60 minutes
- **6000 images**: ~4-8 hours

### System Requirements
- **Disk Space**: ~10-15GB for 6000 wallpapers
- **Memory**: ~500MB during download
- **Bandwidth**: ~5-10GB total

### Raspberry Pi 4B Performance
- **Tested and optimized** for Pi 4B
- **Recommended**: Run overnight or during low-usage
- **Tip**: Use `--limit` for smaller batches

## 🐛 Troubleshooting

### API Key Issues
```
❌ Error: Invalid API key
```
**Solution**: Get a free Wallhaven API key and replace `YOUR_API_KEY_HERE`

### Network Errors
```
❌ Error: Connection timeout
```
**Solution**: Check internet connection, increase `TIMEOUT` value

### Disk Full
```
❌ Error: No space left on device
```
**Solution**: Free up disk space or reduce `--limit`

### Permission Errors
```
❌ Error: Permission denied
```
**Solution**: Run with proper permissions or change `WALLPAPER_DIR`

## 📜 License

MIT License - Free for personal and commercial use.

## 🎯 Roadmap

### Future Improvements
- [ ] Add more niche retro categories
- [ ] Support additional wallpaper APIs
- [ ] Add tag-based filtering
- [ ] Implement GUI selector
- [ ] Add preview functionality

## 🙏 Credits

- **Claude**: Original robust downloader architecture
- **Bard**: Retro computing focus and category merging
- **Wallhaven**: Amazing wallpaper API
- **MASH Installer**: Integration platform

## 📞 Support

For issues or questions:
1. Check the troubleshooting section
2. Verify your API key
3. Test with smaller limits first
4. Open an issue on GitHub

---

**Enjoy your retro-futuristic wallpapers! 🖥️🎮**
