# Shaft H: Final Installer Design - Exact Specifications

## 🎯 Visual Design Requirements

### Final Screen (Unchanged Layout)
```
┌─────────────────────────────────────────────────────────────┐
│          MASH Installer - VX.Y.Z                           │
│                                                       │ SYS INFO      │
│                                                       │               │
│  INFO BOX                                             │_______________│
│                                                       │               │
│                                                       │               │
│                                                       │  Running      │
│                                                       │  Tasks        │
│                                                       │               │
│                                                       │               │
│                                                       │               │
├───────────────────────────────────────────────────────────┤
│ BBS MESSAGE BOX                                        │
└───────────────────────────────────────────────────────────┘
```

### Step Screens (Choice + Info Box)
```
┌─────────────────────────────────────────────────────────────┐
│          MASH Installer - VX.Y.Z                           │
│                                                       │ SYS INFO      │
│                                                       │               │
│  CHOICE + INFO BOX                                     │_______________│
│                                                       │               │
│                                                       │               │
│                                                       │  Running      │
│                                                       │  Tasks        │
│                                                       │               │
│                                                       │               │
│                                                       │               │
├───────────────────────────────────────────────────────────┤
│ BBS MESSAGE BOX                                        │
└───────────────────────────────────────────────────────────┘
```

## 🎨 Exact Screen Specifications

### 1. Intro Screen with ASCII Art
```
┌─────────────────────────────────────────────────────────────┐
│          MASH Installer - V1.1.0                           │
│                                                       │ SYS INFO      │
│  ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄  │               │
│  ███████╗░░░░░░░░█████╗░░█████╗░░██████╗░██╗░░██╗     │               │
│  ██╔════╝░░░░░░░░██╔══██╗██╔══██╗██╔════╝░██║░░██║     │               │
│  █████╗░░░░░░░░░░███████║███████║██║░░██╗░███████║     │               │
│  ██╔══╝░░░░░░░░░░██╔══██║██╔══██║██║░░╚██╗██╔══██║     │               │
│  ███████╗░░░░░░░░██║░░██║██║░░██║╚██████╔╝██║░░██║     │               │
│  ╚══════╝░░░░░░░░╚═╝░░╚═╝╚═╝░░╚═╝░╚═════╝░╚═╝░░╚═╝     │               │
│                                                       │               │
│  ╔═══════════════════════════════════════════════════════════╗  │               │
│  ║  ███╗   ███╗███████╗███╗   ██╗██╗███╗   ██╗║  │               │
│  ║  ████╗ ████║██╔════╝████╗  ██║██║████╗  ██║║  │               │
│  ║  ██╔████╔██║█████╗  ██╔██╗ ██║██║██╔██╗ ██║║  │               │
│  ║  ██║╚██╔╝██║██╔══╝  ██║╚██╗██║██║██║╚██╗██║║  │               │
│  ║  ██║ ╚═╝ ██║███████╗██║ ╚████║██║██║ ╚████║║  │               │
│  ║  ╚═╝     ╚═╝╚══════╝╚═╝  ╚═══╝╚═╝╚═╝  ╚═══╝║  │               │
│  ║       Mythic Assembly & Sigil Heuristics        ║  │               │
│  ╚═══════════════════════════════════════════════════════════╝  │               │
│                                                       │               │
│  [Start Installation]                                  │  RAM: 3.2GB   │
│                                                       │  CPU: 2%      │
│                                                       │  NET: ▁▂▃▅▇▇ │
│                                                       │  Temp: 38°C    │
├───────────────────────────────────────────────────────────┤
│  ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄  │
│  ███████╗░██████╗░███████╗░██████╗░██╗░░░██╗███╗░░██╗███████╗ │
│  ██╔════╝██╔═══██╗██╔════╝██╔═══██╗██║░░░██║████╗░██║██╔════╝ │
│  ██║░░██╗██║░░██║█████╗░░██║░░██║██║░░░██║██╔██╗██║█████╗░░ │
│  ██║░░██║██║░░██║██╔══╝░░██║░░██║██║░░░██║██║╚████║██╔══╝░░ │
│  ╚██████╔╝╚██████╔╝███████╗╚██████╔╝╚██████╔╝██║░╚███║███████╗ │
│  ░╚═════╝░░╚═════╝░╚══════╝░╚═════╝░░╚═════╝░╚═╝░░╚══╝╚══════╝ │
│  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
│  "Retro-Futuristic System Forge - Since 2024"          │
└───────────────────────────────────────────────────────────┘
```

### 2. Font Selection Screen
```
┌─────────────────────────────────────────────────────────────┐
│          MASH Installer - V1.1.0                           │
│                                                       │ SYS INFO      │
│  🎨 FONT SELECTION                                     │_______________│
│                                                       │               │
│  Current: JetBrains Mono Nerd Font                     │  Running      │
│                                                       │  Tasks        │
│  Available Nerd Fonts:                                 │  ✅ Font cache │
│  ▼ JetBrains Mono (✅)                                 │  ⏳ Estimating  │
│    Fira Code                                           │  □ Wallpaper   │
│    Cascadia Code                                      │  □ DE Install │
│    Terminus                                           │               │
│    Hack                                               │               │
│                                                       │               │
│  [Test Font]  [Set Default]                            │  RAM: 3.4GB   │
│                                                       │  CPU: 5%      │
│                                                       │  NET: ▂▃▅▇▇▇ │
│                                                       │  Temp: 40°C    │
├───────────────────────────────────────────────────────────┤
│  💬 BBS: "Did you hear about the programmer who        │
│         couldn't figure out why his code wasn't       │
│         working? He had a bad case of the segfaults!"│
└───────────────────────────────────────────────────────────┘
```

### 3. Desktop Environment Screen
```
┌─────────────────────────────────────────────────────────────┐
│          MASH Installer - V1.1.0                           │
│                                                       │ SYS INFO      │
│  🖥️ DESKTOP ENVIRONMENT (Optional)                      │_______________│
│                                                       │               │
│  ⚠️ Raspberry Pi detected                               │  Running      │
│  Recommendation: Use X11 for best performance           │  Tasks        │
│                                                       │  ✅ DE compat  │
│  ▼ Available DEs:                                      │  ⏳ Estimating  │
│    • None (CLI only) (✅)                               │  □ Installing │
│    • Xfce (X11) - Lightweight                          │               │
│    • KDE Plasma (X11) - Full-featured                  │               │
│    • GNOME (X11) - Modern                               │               │
│    • [Show Wayland Options ▶]                          │  RAM: 3.3GB   │
│                                                       │  CPU: 3%      │
│  [Install DE]  [Skip Desktop]                           │  NET: ▁▃▅▇▇  │
│                                                       │  Temp: 41°C    │
├───────────────────────────────────────────────────────────┤
│  💬 BBS: "Why do programmers prefer dark mode? Because   │
│         light attracts bugs! 🐛"                          │
└───────────────────────────────────────────────────────────┘
```

### 4. Component Selection Screen
```
┌─────────────────────────────────────────────────────────────┐
│          MASH Installer - V1.1.0                           │
│                                                       │ SYS INFO      │
│  📦 COMPONENT SELECTION                                │_______________│
│                                                       │               │
│  Core Components:                                     │  Running      │
│  ✅ Terminal Tools (kitty, tmux, neovim)              │  Tasks        │
│  ✅ Shell Utilities (eza, bat, ripgrep)                │  ✅ Components │
│  ✅ Development Tools (git, cargo, python)             │  ⏳ Estimating  │
│  ✅ System Utilities (htop, btop, ncdu)               │  □ Installing │
│                                                       │               │
│  Optional Extras:                                     │               │
│  □ Retro Wallpapers (2.5GB)                           │  RAM: 3.5GB   │
│  □ Docker & Podman                                    │  CPU: 8%      │
│  □ Multimedia Codecs                                  │  NET: ▁▃▅▇▇▇ │
│  □ Gaming Tools                                       │  Temp: 42°C    │
│                                                       │               │
│  [Select All]  [Deselect All]  [Continue]              │               │
├───────────────────────────────────────────────────────────┤
│  💬 BBS: "A SQL query walks into a bar, goes up to    │
│         two tables and asks, 'Can I join you?'"       │
└───────────────────────────────────────────────────────────┘
```

### 5. Confirmation Screen
```
┌─────────────────────────────────────────────────────────────┐
│          MASH Installer - V1.1.0                           │
│                                                       │ SYS INFO      │
│  ✅ READY TO FORGE                                     │_______________│
│                                                       │               │
│  Installation Summary:                                 │  Running      │
│  • Font: JetBrains Mono Nerd Font                     │  Tasks        │
│  • Desktop: None (CLI)                                 │  ✅ Plan ready │
│  • Components: Standard set (47 packages)              │  ⏳ Waiting    │
│  • Wallpapers: None                                    │               │
│                                                       │               │
│  Estimated:                                           │               │
│  • Time: 8-12 minutes                                  │  RAM: 3.6GB   │
│  • Disk Space: ~2.8GB                                 │  CPU: 4%      │
│  • Network: ~1.2GB                                     │  NET: ▁▂▃▅▇▇ │
│                                                       │  Temp: 43°C    │
│  [Start Installation]  [Back]                          │               │
├───────────────────────────────────────────────────────────┤
│  💬 BBS: "Why do Java developers wear glasses? Because   │
│         they can't C#!" 😎                                │
└───────────────────────────────────────────────────────────┘
```

### 6. Installation In Progress
```
┌─────────────────────────────────────────────────────────────┐
│          MASH Installer - V1.1.0                           │
│                                                       │ SYS INFO      │
│  🔧 FORGING YOUR SYSTEM                                │_______________│
│                                                       │               │
│  Phase 2/4: Core Components                            │  Running      │
│  ✅ Fonts installed                                    │  Tasks        │
│  ✓ Terminal Tools                                     │  ✅ Fonts     │
│    • kitty (✅)                                        │  ✓ Terminal   │
│    • neovim (✅)                                      │  ▶ System     │
│    • tmux (✓)                                        │  □ Utilities  │
│  ▶ Development Tools                                  │               │
│                                                       │               │
│  Progress: [■■■■■■■■■■■■■■■■□□□□□□□□] 68%            │  RAM: 4.2GB   │
│  Time remaining: ~4 minutes                            │  CPU: 28%     │
│                                                       │  NET: ▃▅▇▇▇▇ │
│  [Pause]  [View Log]                                   │  Temp: 48°C    │
├───────────────────────────────────────────────────────────┤
│  💬 BBS: "Why was the computer cold? It left its      │
│         Windows open! ❄️"                               │
└───────────────────────────────────────────────────────────┘
```

### 7. Long Process Confirmation (Modal)
```
╔════════════════════════════════════════════════════════════╗
║                                                  ║
║  ⚠️  LONG PROCESS CONFIRMATION                 ║
║                                                  ║
║  Wallpaper download will take approximately   ║
║  8-12 minutes and use ~2.5GB of disk space.   ║
║                                                  ║
║  This is a perfect time to:                   ║
║  • Grab a beverage 🍺                           ║
║  • Stretch your legs                           ║
║  • Read about retro computing                 ║
║  • Check out our BBS messages below!           ║
║                                                  ║
║  [Start Download Now]    [Skip Wallpapers]    ║
║                                                  ║
║  Auto-continue in: 10 seconds                ║
╚════════════════════════════════════════════════════════════╝
```

### 8. Installation Complete (Final Screen)
```
┌─────────────────────────────────────────────────────────────┐
│          MASH Installer - V1.1.0                           │
│                                                       │ SYS INFO      │
│  ✅ INSTALLATION COMPLETE                              │_______________│
│                                                       │               │
│  Summary:                                              │  Running      │
│  • Font: JetBrains Mono Nerd Font                     │  Tasks        │
│  • Desktop: None (CLI)                                 │  ✅ All done   │
│  • Components: 47 packages                           │               │
│  • Wallpapers: 2625 retro-futuristic                  │               │
│  • Time: 11 minutes 27 seconds                        │               │
│  • Disk used: 2.7GB                                   │               │
│                                                       │               │
│  💡 Next Steps:                                         │               │
│  1. Reboot your system                                │  RAM: 3.8GB   │
│  2. Run 'mash-doctor' to verify                       │  CPU: 2%      │
│  3. Explore ~/Pictures/RetroWallpapers/               │  NET: ▁▂▃▅▇▇ │
│  4. Enjoy your retro-futuristic system!              │  Temp: 45°C    │
│                                                       │               │
│  [Reboot Now]  [View Log]  [Exit to Shell]            │               │
├───────────────────────────────────────────────────────────┤
│  💬 BBS: "Why don't programmers like nature? Too     │
│         many bugs and no syntax highlighting! 🌲"     │
└───────────────────────────────────────────────────────────┘
```

## 🎵 Audio Requirements

### Task Completion Sound
- **File**: `resources/sounds/task_complete.wav`
- **Type**: Old school 8-bit completion sound
- **Style**: Retro computer "bling" or "success" sound
- **Duration**: 0.5 - 1.0 seconds
- **Format**: WAV format, 44.1kHz, 16-bit
- **Trigger**: Play when final installation completes

**Example sounds to consider:**
- Classic Windows 95 "ding" sound
- Commodore 64 load complete sound
- Arcade game level complete sound
- Old modem connection success sound
- Retro game item collection sound

## 💬 BBS Message System Specifications

### Message Categories
1. **Whimsical Nerd Jokes** (60%)
2. **UNIX/Computer History Facts** (25%)
3. **Retro-Futuristic Facts** (10%)
4. **Installation Tips** (5%)

### Message Rotation
- **Cycle time**: 5-7 seconds between messages
- **Random selection**: Weighted by category
- **No repeats**: Don't show same message twice in a row
- **Contextual**: Some messages related to current operation

### Sample Messages

#### Nerd Jokes
```
["Why do programmers prefer dark mode? Because light attracts bugs! 🐛"]
["A SQL query walks into a bar, goes up to two tables and asks, 'Can I join you?'"]
["Why do Java developers wear glasses? Because they can't C#! 😎"]
["Why was the computer cold? It left its Windows open! ❄️"]
["Why don't programmers like nature? Too many bugs and no syntax highlighting! 🌲"]
["How many programmers does it take to change a lightbulb? None, that's a hardware problem!"]
["Why did the programmer quit his job? He didn't get arrays! 😢"]
["What's a pirate's favorite programming language? Arr-gon! 🏴‍☠️"]
["Why did the function break up with the variable? It couldn't handle the constant changes!"]
["How do you comfort a JavaScript bug? You console it! 💻"]
```

#### UNIX/Computer History
```
["Did you know? The first computer virus 'Creeper' was created in 1971 for ARPANET!"]
["Fun Fact: The first computer bug was a real moth found in Harvard's Mark II in 1947!"]
["UNIX was created in 1969 by Ken Thompson and Dennis Ritchie at Bell Labs!"]
["The first UNIX shell was written by Ken Thompson in 1971!"]
["Did you know? The 'ls' command is one of the oldest UNIX commands, dating back to 1971!"]
["The vi editor was created by Bill Joy in 1976 - and we're still using it today!"]
["The first UNIX manual was written in 1971 and was only 3 pages long!"]
["Did you know? The term 'daemon' comes from Maxwell's demon in thermodynamics!"]
["The first UNIX system had only 64KB of memory - less than this BBS message!"]
["The C programming language was created to write UNIX in 1972!"]
```

#### Retro-Futuristic Facts
```
["Retro-Futurism combines old-school aesthetics with futuristic technology!"]
["The BBC Micro, released in 1981, was used in schools and inspired a generation!"]
["Cyberpunk literature emerged in the 1980s, envisioning high-tech, low-life futures!"]
["The Commodore 64 (1982) is still the best-selling computer model of all time!"]
["Retro-futurism often features neon lights, holograms, and vintage computers!"]
["The TRS-80 (1977) was one of the first mass-market personal computers!"]
["Vintage terminals like the VT100 (1978) inspired modern terminal emulators!"]
["The ZX Spectrum (1982) brought computing to millions with its rubber keyboard!"]
["Retro-futurism celebrates the aesthetic of 1980s visions of the future!"]
["The Amiga 1000 (1985) was years ahead of its time with multimedia capabilities!"]
```

#### Installation Tips
```
["Tip: All MASH operations are idempotent - safe to re-run anytime!"]
["Did you know? You can customize your installation later with 'mash install <component>'"]
["Tip: Check out 'mash-doctor' to verify your system after installation!"]
["Remember: You can always add more wallpapers later with 'mash wallpapers'"]
["Tip: The installation log is saved to ~/.mash/install.log for troubleshooting!"]
["Did you know? MASH supports dry-run mode with 'mash install --dry-run'"]
["Tip: Customize your terminal further with 'mash configure terminal'"]
["Remember: Most operations can be run individually after installation!"]
```

## 🎨 ASCII Art Specifications

### Main Logo (Intro Screen)
```
  ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
  ███████╗░░░░░░░░█████╗░░█████╗░░██████╗░██╗░░██╗
  ██╔════╝░░░░░░░░██╔══██╗██╔══██╗██╔════╝░██║░░██║
  █████╗░░░░░░░░░░███████║███████║██║░░██╗░███████║
  ██╔══╝░░░░░░░░░░██╔══██║██╔══██║██║░░╚██╗██╔══██║
  ███████╗░░░░░░░░██║░░██║██║░░██║╚██████╔╝██║░░██║
  ╚══════╝░░░░░░░░╚═╝░░╚═╝╚═╝░░╚═╝░╚═════╝░╚═╝░░╚═╝
  ╔═══════════════════════════════════════════════════════════╗
  ║  ███╗   ███╗███████╗███╗   ██╗██╗███╗   ██╗║
  ║  ████╗ ████║██╔════╝████╗  ██║██║████╗  ██║║
  ║  ██╔████╔██║█████╗  ██╔██╗ ██║██║██╔██╗ ██║║
  ║  ██║╚██╔╝██║██╔══╝  ██║╚██╗██║██║██║╚██╗██║║
  ║  ██║ ╚═╝ ██║███████╗██║ ╚████║██║██║ ╚████║║
  ║  ╚═╝     ╚═╝╚══════╝╚═╝  ╚═══╝╚═╝╚═╝  ╚═══╝║
  ║       Mythic Assembly & Sigil Heuristics        ║
  ╚═══════════════════════════════════════════════════════════╝
```

### Bottom Banner
```
  ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
  ███████╗░██████╗░███████╗░██████╗░██╗░░░██╗███╗░░██║███████╗
  ██╔════╝██╔═══██╗██╔════╝██╔═══██╗██║░░░██║████╗░██║██╔════╝
  ██║░░██╗██║░░██║█████╗░░██║░░██║██║░░░██║██╔██╗██║█████╗░░
  ██║░░██║██║░░██║██╔══╝░░██║░░██║██║░░░██║██║╚████║██╔══╝░░
  ╚██████╔╝╚██████╔╝███████╗╚██████╔╝╚██████╔╝██║░╚███║███████╗
  ░╚═════╝░░╚═════╝░╚══════╝░╚═════╝░░╚═════╝░╚═╝░░╚══╝╚══════╝
  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
  "Retro-Futuristic System Forge - Since 2024"
```

## 🔧 Implementation Requirements

### Files to Create/Modify

#### New Files
- `resources/sounds/task_complete.wav` - Completion sound
- `resources/ascii/mash_logo.txt` - Main ASCII logo
- `resources/ascii/banner.txt` - Bottom banner
- `installer-cli/src/tui/bbs_messages.rs` - BBS message system
- `installer-cli/src/tui/ascii_art.rs` - ASCII art rendering
- `installer-cli/src/audio.rs` - Audio playback

#### Modified Files
- `installer-cli/src/tui/render.rs` - Add ASCII art and BBS messages
- `installer-cli/src/tui/app.rs` - Handle audio playback
- `installer-cli/src/main.rs` - Add sound initialization
- `Cargo.toml` - Add audio dependencies (rodio, etc.)

### Audio Implementation
```rust
// installer-cli/src/audio.rs
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

pub struct AudioPlayer {
    sink: Option<Sink>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self { sink: None }
    }

    pub fn play_completion_sound(&mut self) -> Result<(), String> {
        // Load embedded sound file
        let sound_data = include_bytes!("../../resources/sounds/task_complete.wav");
        let cursor = Cursor::new(sound_data);
        
        // Create output stream
        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to create audio stream: {}", e))?;
        
        // Create sink and play
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;
        
        let source = Decoder::new(cursor)
            .map_err(|e| format!("Failed to decode audio: {}", e))?;
        
        sink.append(source);
        sink.set_volume(0.5); // 50% volume
        sink.play();
        
        self.sink = Some(sink);
        Ok(())
    }
}
```

### BBS Message System
```rust
// installer-cli/src/tui/bbs_messages.rs
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct BBSMessageSystem {
    messages: Vec<BBSMessage>,
    last_message_time: u64,
    last_message_index: usize,
}

struct BBSMessage {
    category: MessageCategory,
    content: String,
    weight: u8, // Higher weight = more likely to show
}

enum MessageCategory {
    Joke,
    UnixHistory,
    RetroFuturism,
    InstallationTip,
}

impl BBSMessageSystem {
    pub fn new() -> Self {
        Self {
            messages: Self::load_messages(),
            last_message_time: 0,
            last_message_index: 0,
        }
    }

    fn load_messages() -> Vec<BBSMessage> {
        vec![
            BBSMessage { category: MessageCategory::Joke, content: "Why do programmers prefer dark mode? Because light attracts bugs! 🐛".to_string(), weight: 6 },
            BBSMessage { category: MessageCategory::UnixHistory, content: "Did you know? The first computer virus 'Creeper' was created in 1971 for ARPANET!".to_string(), weight: 3 },
            // ... all other messages
        ]
    }

    pub fn get_current_message(&mut self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Rotate message every 5-7 seconds
        if now - self.last_message_time > 5 {
            self.last_message_time = now;
            self.last_message_index = self.get_random_message_index();
        }
        
        self.messages[self.last_message_index].content.clone()
    }

    fn get_random_message_index(&self) -> usize {
        // Weighted random selection
        let total_weight: u32 = self.messages.iter().map(|m| m.weight as u32).sum();
        let mut rng = rand::thread_rng();
        let target = rng.gen_range(0..total_weight);
        
        let mut cumulative = 0;
        for (i, message) in self.messages.iter().enumerate() {
            cumulative += message.weight as u32;
            if target < cumulative {
                // Make sure it's not the same as last time
                if i != self.last_message_index {
                    return i;
                } else if i + 1 < self.messages.len() {
                    return i + 1;
                } else {
                    return 0;
                }
            }
        }
        0
    }
}
```

### ASCII Art Rendering
```rust
// installer-cli/src/tui/ascii_art.rs
pub struct ASCIIArt {
    logo: Vec<String>,
    banner: Vec<String>,
}

impl ASCIIArt {
    pub fn new() -> Self {
        Self {
            logo: Self::load_logo(),
            banner: Self::load_banner(),
        }
    }

    fn load_logo() -> Vec<String> {
        include_str!("../../resources/ascii/mash_logo.txt")
            .lines()
            .map(|s| s.to_string())
            .collect()
    }

    fn load_banner() -> Vec<String> {
        include_str!("../../resources/ascii/banner.txt")
            .lines()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn render_logo(&self, frame: &mut Frame, area: Rect) {
        let logo_height = self.logo.len() as u16;
        let start_y = area.y + (area.height - logo_height) / 2;
        
        for (i, line) in self.logo.iter().enumerate() {
            frame.render_widget(
                Paragraph::new(line.clone())
                    .style(Style::default().fg(Color::Green)),
                Rect::new(area.x, start_y + i as u16, area.width, 1)
            );
        }
    }

    pub fn render_banner(&self, frame: &mut Frame, area: Rect) {
        let banner_height = self.banner.len() as u16;
        
        for (i, line) in self.banner.iter().enumerate() {
            frame.render_widget(
                Paragraph::new(line.clone())
                    .style(Style::default().fg(Color::Cyan))
                    .alignment(Alignment::Center),
                Rect::new(area.x, area.y + i as u16, area.width, 1)
            );
        }
    }
}
```

## ✅ Success Criteria

### Visual Design
- ✅ Exact 4-tile layout preserved in all screens
- ✅ Final install screen unchanged
- ✅ Step screens use "CHOICE + INFO BOX" format
- ✅ ASCII art intro screen with MASH logo
- ✅ Bottom banner with retro-futuristic tagline

### BBS Message System
- ✅ Random message rotation (5-7 seconds)
- ✅ Weighted categories (60% jokes, 25% history, etc.)
- ✅ No repeated messages consecutively
- ✅ Mix of humor, facts, and tips
- ✅ Retro-futuristic theme maintained

### Audio
- ✅ Task completion sound implemented
- ✅ Old school 8-bit style sound
- ✅ Plays only on final completion
- ✅ Volume controlled (not too loud)
- ✅ Works across platforms

### User Experience
- ✅ Familiar layout preserved
- ✅ Enhanced information within existing structure
- ✅ Fun and engaging BBS messages
- ✅ Professional yet whimsical tone
- ✅ Clear progression through steps

## 🎯 Conclusion

This final design specification ensures that all requirements are met:

1. **Exact Layout Preservation**: The 4-tile structure remains unchanged throughout
2. **Enhanced Content**: Richer information and options within existing tiles
3. **Retro-Futuristic Aesthetic**: ASCII art, BBS messages, and sound effects
4. **User-Friendly**: Clear progression with helpful contextual information
5. **Engaging Experience**: Humor, facts, and personality through BBS messages
6. **Professional Finish**: Completion sound adds polish to the experience

The implementation will transform the MASH installer into a truly retro-futuristic experience that's both functional and fun, while maintaining the familiar interface that users expect.

"*Like a well-tuned retro computer, the best interfaces are those that work perfectly and bring a smile to your face.*" — Bard 🍺⚒️