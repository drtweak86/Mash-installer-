# 🐉 SHAFT C: TUI Aesthetic Transformation
> **The Bard's 10-Point Plan to Excavate the Retro Ruin**
> *“From cyberpunk neon to BBC Micro green-on-black, we crash 1984 into 2026 with dwarven precision.”* — Bard 🍺

---

## 🎯 Objective

Transform the TUI aesthetic from cyberpunk to BBC Micro/UNIX terminal style (1984 crashed into 2026) while maintaining all existing functionality. The goal is to create an authentic retro computing experience that honors the heritage of UNIX terminals and BBC Micro computers.

## 🏗️ Foundation Principles

### 1. Function > Form
- **Rule**: All existing functionality must work exactly as before
- **Approach**: Visual changes only, no functional modifications
- **Verification**: All tests must pass before any visual changes

### 2. Test Before Extend
- **Rule**: Verify all functionality works before making aesthetic changes
- **Approach**: Create theme abstraction layer first
- **Verification**: Run full test suite before visual implementation

### 3. User Needs > Architecture
- **Rule**: Focus on user experience and accessibility
- **Approach**: Ensure retro aesthetic doesn't hinder usability
- **Verification**: User testing on various terminal types

### 4. Simple > Clever
- **Rule**: Straightforward solutions over clever hacks
- **Approach**: Use existing Ratatui capabilities
- **Verification**: Code review for simplicity

### 5. Document Before Code
- **Rule**: ABD - Always Be Documenting
- **Approach**: Complete documentation before implementation
- **Verification**: Documentation review before coding

---

## 🗺️ 10-Point Excavation Plan

### 1. Foundation: Audit and Abstraction

**Objective**: Ensure all existing functionality works before aesthetic changes

**Files to Modify**:
- `installer-cli/src/tui/app.rs`
- `installer-cli/src/tui/render.rs`
- `installer-cli/src/tui/theme.rs`

**Changes**:
```rust
// Create theme abstraction layer
pub enum TuiTheme {
    Cyberpunk,  // Current theme
    RetroBBC,   // New theme
    RetroUNIX,  // Alternative theme
}

// Theme manager
pub struct ThemeManager {
    current: TuiTheme,
    themes: HashMap<String, ThemeConfig>,
}
```

**Verification**:
- ✅ All 99 tests passing
- ✅ No functional changes
- ✅ Theme switching works

---

### 2. Terminal Emulation: BBC Micro Style

**Objective**: Create authentic BBC Micro terminal feel

**Files to Modify**:
- `installer-cli/src/tui/theme.rs`

**Changes**:
```rust
// BBC Micro color palette
pub const BBC_GREEN: Color = Color::Rgb(0, 255, 0);
pub const BBC_BLACK: Color = Color::Rgb(0, 0, 0);
pub const BBC_YELLOW: Color = Color::Rgb(255, 255, 0);
pub const BBC_BLUE: Color = Color::Rgb(0, 0, 255);
pub const BBC_RED: Color = Color::Rgb(255, 0, 0);

// BBC Micro theme
pub fn bbc_micro_theme() -> Theme {
    Theme {
        background: BBC_BLACK,
        text: BBC_GREEN,
        highlight: BBC_YELLOW,
        error: BBC_RED,
        warning: BBC_YELLOW,
        success: BBC_BLUE,
    }
}
```

**Verification**:
- ✅ Green-on-black color scheme
- ✅ Teletext font support
- ✅ Blocky character rendering
- ✅ Mode 7 display emulation

---

### 3. ASCII Art: Loading Screens

**Objective**: Replace cyberpunk graphics with retro ASCII art

**Files to Modify**:
- `resources/themes/retro-bbc/loading.txt` (new)
- `installer-cli/src/tui/render.rs`

**Changes**:
```rust
// BBC Micro loading screen
pub const BBC_LOADING: &str = r#"
  ____  _____ _   _ _____ ____  _____
 |  _ \| ____| \ | |_   _|  _ \| ____|
 | |_) |  _||  \| | | | | |_) |  _|
 |  _ <| |___| |\  | | | |  _ <| |___
 |_| \_\_____|_| \_| |_| |_| \_\_____|
"#;

// UNIX terminal art
pub const UNIX_TERMINAL: &str = r#"
  _____ _   _ _____ ____ _____ ____
 |_   _| \ | |_   _|  _ \_   _|  _ \
   | | |  \| | | | | |_) || | | |_) |
   | | | |\  | | | |  _ < | | |  _ <
   |_| |_| \_| |_| |_| \_\|_| |_| \_\
"#;
```

**Verification**:
- ✅ Loading screens display correctly
- ✅ ASCII art fits terminal dimensions
- ✅ Multiple terminal types supported

---

### 4. Layout: UNIX Terminal Style

**Objective**: Recreate classic UNIX terminal layout

**Files to Modify**:
- `installer-cli/src/tui/render.rs`

**Changes**:
```rust
// Single pane layout
pub fn render_unix_layout(f: &mut Frame, area: Rect) {
    // Main content area (80% of screen)
    let main_area = Rect {
        x: 0,
        y: 0,
        width: area.width,
        height: area.height * 80 / 100,
    };
    
    // Status bar (20% of screen at bottom)
    let status_area = Rect {
        x: 0,
        y: main_area.height,
        width: area.width,
        height: area.height - main_area.height,
    };
    
    // Render main content
    render_main_content(f, main_area);
    
    // Render status bar
    render_status_bar(f, status_area);
}

// Status bar with command prompt style
pub fn render_status_bar(f: &mut Frame, area: Rect) {
    let status = Paragraph::new(Text::from("mash-setup$ "))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Green));
    f.render_widget(status, area);
}
```

**Verification**:
- ✅ Single pane terminal layout
- ✅ Status bar at bottom
- ✅ Command prompt style
- ✅ Logical menu flow

---

### 5. Typography: Retro Fonts

**Objective**: Use authentic retro terminal fonts

**Files to Modify**:
- `installer-core/src/fonts.rs`

**Changes**:
```rust
// Retro font configuration
pub struct RetroFont {
    name: String,
    size: u16,
    path: PathBuf,
}

// Terminus font (14px)
pub fn terminus_font() -> RetroFont {
    RetroFont {
        name: "Terminus".to_string(),
        size: 14,
        path: PathBuf::from("/usr/share/fonts/terminus/terminusbf-14.png"),
    }
}

// Fallback fonts
pub fn fallback_fonts() -> Vec<String> {
    vec![
        "Terminus".to_string(),
        "Fixed".to_string(),
        "Monospace".to_string(),
    ]
}
```

**Verification**:
- ✅ Terminus font (14px) working
- ✅ Fallback fonts configured
- ✅ Fixed-width only
- ✅ No anti-aliasing

---

### 6. Color Scheme: BBC Micro Palette

**Objective**: Implement authentic BBC Micro colors

**Files to Modify**:
- `installer-cli/src/tui/theme.rs`

**Changes**:
```rust
// BBC Micro color palette
pub struct BbcMicroPalette {
    pub background: Color,
    pub text: Color,
    pub highlight: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
}

impl BbcMicroPalette {
    pub fn new() -> Self {
        Self {
            background: Color::Rgb(0, 0, 0),              // Black
            text: Color::Rgb(0, 255, 0),                 // Green
            highlight: Color::Rgb(255, 255, 0),           // Yellow
            error: Color::Rgb(255, 0, 0),                // Red
            warning: Color::Rgb(255, 255, 0),            // Yellow
            success: Color::Rgb(0, 0, 255),               // Blue
        }
    }
}
```

**Verification**:
- ✅ Green text on black background
- ✅ Yellow highlights
- ✅ Blue for active elements
- ✅ Red for errors/warnings

---

### 7. Sound: Optional 8-bit Beeps

**Objective**: Add retro sound effects (optional)

**Files to Modify**:
- `installer-cli/src/tui/app.rs`

**Changes**:
```rust
// Sound configuration
pub struct SoundConfig {
    pub enabled: bool,
    pub volume: u8,
    pub beep_frequency: u32,
    pub beep_duration: u32,
}

impl SoundConfig {
    pub fn new() -> Self {
        Self {
            enabled: false,      // Default: disabled
            volume: 50,          // 0-100
            beep_frequency: 800, // Hz
            beep_duration: 50,   // ms
        }
    }
    
    pub fn play_beep(&self) {
        if self.enabled {
            // Use crossterm's beep or system beep
            #![cfg(target_family = "unix")]
            unsafe {
                libc::beep();
            }
        }
    }
}
```

**Verification**:
- ✅ Sound effects optional
- ✅ Configurable volume
- ✅ Keyboard beep on Enter
- ✅ Menu navigation sounds

---

### 8. Animation: Teletext Style

**Objective**: Create teletext-style animations

**Files to Modify**:
- `installer-cli/src/tui/render.rs`

**Changes**:
```rust
// Teletext-style scrolling text
pub fn render_scrolling_text(f: &mut Frame, area: Rect, text: &str, speed: u64) {
    let mut scroll_offset = 0;
    let chars: Vec<char> = text.chars().collect();
    
    for y in 0..area.height {
        let line_start = (scroll_offset + y * area.width as usize) % chars.len();
        let line_end = line_start + area.width as usize;
        let line_chars: Vec<char> = chars[line_start..line_end.min(chars.len())]
            .iter()
            .cloned()
            .collect();
        
        let line: String = line_chars.into_iter().collect();
        let paragraph = Paragraph::new(Text::from(line))
            .alignment(Alignment::Left)
            .style(Style::default().fg(Color::Green));
        
        f.render_widget(paragraph, Rect {
            x: area.x,
            y: area.y + y,
            width: area.width,
            height: 1,
        });
        
        scroll_offset = (scroll_offset + 1) % chars.len();
    }
}

// Loading animation
pub fn render_loading_animation(f: &mut Frame, area: Rect) {
    let frames = ["-", "\", "|", "/"];
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let frame_idx = COUNTER.fetch_add(1, Ordering::SeqCst) % frames.len();
    
    let loading_text = format!("Loading {} ", frames[frame_idx]);
    let paragraph = Paragraph::new(Text::from(loading_text))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green));
    
    f.render_widget(paragraph, area);
}
```

**Verification**:
- ✅ Scrolling text effects
- ✅ Page transitions
- ✅ Loading animations
- ✅ Progress indicators

---

### 9. Error Messages: UNIX Style

**Objective**: Format errors like classic UNIX

**Files to Modify**:
- `installer-core/src/error.rs`

**Changes**:
```rust
// UNIX-style error formatting
pub fn format_unix_error(error: &InstallerError) -> String {
    let mut output = String::new();
    
    // Error code
    output.push_str(&format!("Error {}: ", error.code()));
    
    // Brief description
    output.push_str(&error.description());
    
    // Newline
    output.push('\n');
    
    // Clear action items
    if let Some(advice) = error.advice() {
        output.push_str("Try: ");
        output.push_str(advice);
        output.push('\n');
    }
    
    output
}

// Example output:
// Error 101: Failed to install package
// Try: sudo apt-get update && sudo apt-get install -f
```

**Verification**:
- ✅ Error codes
- ✅ Brief descriptions
- ✅ Clear action items
- ✅ No fancy formatting

---

### 10. Documentation: Retro Style Guide

**Objective**: Document the retro aesthetic

**Files to Modify**:
- `docs/mining-projects/shaftc.md` (this document)
- `README.md`

**Changes**:
```markdown
## Retro Aesthetic Guide

### Color Palette
- **Background**: Black (#000000)
- **Text**: Green (#00FF00)
- **Highlight**: Yellow (#FFFF00)
- **Error**: Red (#FF0000)
- **Success**: Blue (#0000FF)

### Typography
- **Primary Font**: Terminus 14px
- **Fallback Fonts**: Fixed, Monospace
- **Line Height**: 1.0 (no spacing)
- **Character Width**: Fixed (monospace)

### Layout
- **Terminal Size**: 80x24 minimum
- **Single Pane**: Full screen
- **Status Bar**: Bottom 20%
- **Content Area**: Top 80%

### Sound
- **Default**: Disabled
- **Optional**: 8-bit beeps
- **Volume**: 0-100%
- **Frequency**: 800Hz

### Animations
- **Loading**: Teletext-style
- **Transitions**: Scrolling text
- **Progress**: Simple indicators
```

**Verification**:
- ✅ Design principles documented
- ✅ Color palette reference
- ✅ Typography guide
- ✅ Screenshots included

---

## 🔨 Technical Implementation

### Phase 1: Foundation (1 day)

**Tasks**:
- [ ] Audit current TUI code
- [ ] Identify all rendering points
- [ ] Create theme abstraction layer
- [ ] Verify all functionality works

**Files**:
- `installer-cli/src/tui/app.rs`
- `installer-cli/src/tui/render.rs`
- `installer-cli/src/tui/theme.rs`

**Verification**:
- ✅ All 99 tests passing
- ✅ No functional changes
- ✅ Theme switching works

---

### Phase 2: BBC Micro Theme (2 days)

**Tasks**:
- [ ] Implement green-on-black color scheme
- [ ] Add teletext font support
- [ ] Create ASCII art loading screens
- [ ] Test on various terminal types

**Files**:
- `installer-cli/src/tui/theme.rs`
- `resources/themes/retro-bbc/loading.txt` (new)

**Verification**:
- ✅ Green-on-black color scheme
- ✅ Teletext font support
- ✅ Loading screens display correctly
- ✅ Multiple terminal types supported

---

### Phase 3: UNIX Layout (1 day)

**Tasks**:
- [ ] Redesign menu structure
- [ ] Implement single-pane layout
- [ ] Add status bar
- [ ] Test navigation flow

**Files**:
- `installer-cli/src/tui/render.rs`

**Verification**:
- ✅ Single pane terminal layout
- ✅ Status bar at bottom
- ✅ Command prompt style
- ✅ Logical menu flow

---

### Phase 4: Polish (1 day)

**Tasks**:
- [ ] Add sound effects (optional)
- [ ] Create animations
- [ ] Finalize error messages
- [ ] Complete documentation

**Files**:
- `installer-cli/src/tui/app.rs`
- `installer-core/src/error.rs`
- `docs/mining-projects/shaftc.md`

**Verification**:
- ✅ Sound effects optional
- ✅ Animations working
- ✅ Error messages formatted
- ✅ Documentation complete

---

## 📁 Files to be Modified

### Core TUI Files
1. `installer-cli/src/tui/theme.rs` - Color schemes and styling
2. `installer-cli/src/tui/render.rs` - Layout and rendering
3. `installer-cli/src/tui/app.rs` - Navigation and interaction

### Core Functionality Files
4. `installer-core/src/fonts.rs` - Font handling
5. `installer-core/src/error.rs` - Error formatting

### Resource Files
6. `resources/themes/retro-bbc/` - Assets (new directory)
   - `loading.txt` - BBC Micro loading screen
   - `unix_terminal.txt` - UNIX terminal art
   - `animations/` - Teletext animations

### Documentation Files
7. `docs/mining-projects/shaftc.md` - This document (new)
8. `README.md` - Update with retro aesthetic info

---

## ⚠️ Risk Assessment

### Low Risk (✅)
- **Visual changes only**: No functional impact
- **Theme abstraction**: Easy to revert
- **Optional features**: Sound can be disabled
- **Fallback fonts**: Multiple fallback options

### Medium Risk (⚠️)
- **Font rendering**: Different terminals may render differently
- **Color accuracy**: Terminal color support varies
- **Animation performance**: May be slow on old hardware
- **Mitigation**: Test on multiple terminal types, provide fallbacks

### High Risk (🔴)
- **Sound effects**: Compatibility issues across systems
- **Terminal limitations**: Some terminals may not support features
- **Mitigation**: Make sound optional, provide feature detection

---

## 📅 Timeline

### Start Date
- **After Shaft B completion**: 2026-03-01

### Duration
- **Total**: 5 days

### Phase Breakdown
| Phase | Duration | Start Date | End Date |
|-------|----------|------------|----------|
| 1. Foundation | 1 day | 2026-03-01 | 2026-03-01 |
| 2. BBC Micro Theme | 2 days | 2026-03-02 | 2026-03-03 |
| 3. UNIX Layout | 1 day | 2026-03-04 | 2026-03-04 |
| 4. Polish | 1 day | 2026-03-05 | 2026-03-05 |
| **Total** | **5 days** | **2026-03-01** | **2026-03-06** |

### Completion Date
- **Target**: 2026-03-06

---

## 🎯 Success Criteria

### Functional Requirements
- ✅ All existing functionality works exactly as before
- ✅ All 99 tests passing
- ✅ No regressions
- ✅ Backward compatibility maintained

### Aesthetic Requirements
- ✅ Green-on-black color scheme implemented
- ✅ BBC Micro aesthetic achieved
- ✅ UNIX terminal style recreated
- ✅ ASCII art loading screens working

### Usability Requirements
- ✅ Clear navigation
- ✅ Readable text
- ✅ Intuitive interface
- ✅ Accessible to all users

### Documentation Requirements
- ✅ Complete technical documentation
- ✅ User guides updated
- ✅ Screenshots included
- ✅ Design principles documented

---

## 🔮 Future Enhancements

### Potential Future Work
1. **Additional Retro Themes**: Amiga, Atari, C64
2. **Theme Preview**: Show preview before applying
3. **Wallpaper Management**: GUI for wallpaper selection
4. **Community Themes**: User-submitted themes
5. **Advanced Customization**: Custom color schemes
6. **Theme Versioning**: Update themes over time
7. **User Submissions**: Theme repository

### Timeline
- **Prerequisite**: Shaft C completion
- **Estimated Start**: 2026-03-07
- **Duration**: Ongoing

---

## 📋 Summary

### What We're Building
- Authentic BBC Micro/UNIX terminal aesthetic
- Green-on-black color scheme
- Teletext-style animations
- Optional 8-bit sound effects
- UNIX-style error messages
- Single-pane terminal layout

### Why We're Building It
- Honor the heritage of retro computing
- Create authentic user experience
- Improve readability and focus
- Provide nostalgic appeal

### How We're Building It
- Theme abstraction layer
- Incremental implementation
- Extensive testing
- Clear documentation

### When We're Building It
- After Shaft B (2026-03-01)
- 5-day implementation
- Target completion: 2026-03-06

---

**Last Updated**: 2026-02-21
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️

*Document Status: PLANNED* ⏳
*Version: 1.0* (Initial draft)

---

> *“May your builds be green,
> Your tests be comprehensive,
> Your documentation complete,
> And your commits atomic.
> Raise a tankard to the forge!
> — Bard, Drunken Dwarf Runesmith”* 🍺🔥
