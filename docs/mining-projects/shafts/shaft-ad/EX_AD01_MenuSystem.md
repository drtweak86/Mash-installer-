# 📜 EX_AD01: LANDING MENU SYSTEM IMPLEMENTATION

> **Shaft**: AD (Menu-Driven Navigation)
> **Status**: ⏳ **PLANNING**
> **Priority**: ✅ **HIGH**
> **Estimated Duration**: 1 day

## 🎯 OBJECTIVE

Implement the core landing menu system that replaces the linear navigation flow with a flexible menu-driven approach. This excavation task focuses on the foundational infrastructure and basic menu functionality.

## 📋 DETAILED STEPS

### Step 1: Source of Truth Consultation
- [ ] Read `docs/forge-tavern/maps.md` ✅
- [ ] Read `docs/forge-tavern/maps-explored.md` ✅
- [ ] Read `docs/HISTORY.md` ✅
- [ ] Review `docs/mining-projects/shafts/shaft-ad/Overview.md` ✅

### Step 2: Screen Enum Extension
**File**: `installer-cli/src/tui/state.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Screen {
    Welcome,
    SystemScan,
    Landing,      // ← NEW: Add this line
    DistroSelect,
    // ... rest of existing screens
}
```

**Verification**: 
- [ ] Compile with `cargo check --bin mash-setup`
- [ ] Run `cargo clippy --all-targets` (no new warnings)
- [ ] Run existing tests (`cargo test --package installer-core`)

### Step 3: Menu Module Creation
**File**: `installer-cli/src/tui/menus/landing.rs`

```rust
use crate::tui::app::TuiApp;
use crate::tui::menus::helpers::station_block;
use crate::tui::theme;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

const LANDING_MENU: &[&str] = &[
    "1) Distribution Selection - Choose your Linux distribution",
    "2) Profile Selection - Select installation profile (Minimal/Dev/Full)",
    "3) System Summary - View hardware analysis and recommendations",
    "4) Theme Selection - Configure aesthetic preferences",
    "5) Software Selection - Choose applications and tools",
    "6) Advanced Configuration - Argon, Docker, Chezmoi settings",
    "7) Start Installation - Begin the installation process",
];

pub fn draw_landing(f: &mut Frame, area: Rect, app: &TuiApp) {
    let block = station_block("MAIN_FORGE_MENU");
    f.render_widget(&block, area);
    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    // Header with cyberpunk aesthetic
    let header_text = Paragraph::new("MASH INSTALLER - MAIN MENU")
        .style(theme::success_style())
        .alignment(Alignment::Center);
    f.render_widget(header_text, chunks[0]);

    // Menu items with selection highlighting
    let menu_items: Vec<_> = LANDING_MENU
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let prefix = if i == app.menu_cursor { ">> " } else { "   " };
            format!("{}{}", prefix, item)
        })
        .collect();

    let menu_text = menu_items.join("\n");
    let menu = Paragraph::new(menu_text)
        .style(theme::default_style());
    f.render_widget(menu, chunks[1]);

    // Footer with navigation instructions
    let footer = Paragraph::new(
        "Use ↑/↓ arrows or 1-7 keys to navigate | ENTER to select | ESC to go back",
    )
    .style(theme::dim_style())
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);

    // System info display
    if let Some(profile) = &app.system_profile {
        let sys_info = format!(
            "System: {} | CPU: {} | RAM: {:.1} GB | OS: {}",
            profile.platform.model,
            profile.cpu.model,
            profile.memory.ram_total_kb as f32 / (1024.0 * 1024.0),
            profile.distro.pretty_name
        );
        let sys_paragraph = Paragraph::new(sys_info)
            .style(theme::dim_style())
            .alignment(Alignment::Center);
        f.render_widget(sys_paragraph, area);
    }
}
```

**Verification**:
- [ ] File compiles with `cargo check --bin mash-setup`
- [ ] No clippy warnings
- [ ] Menu displays correctly in TUI

### Step 4: Module Integration
**File**: `installer-cli/src/tui/menus/mod.rs`

```rust
pub mod helpers;
pub mod install;
pub mod landing;    // ← NEW: Add this line
pub mod scan;
pub mod selection;
pub mod software;
pub mod welcome;

pub use install::*;
pub use landing::*;   // ← NEW: Add this line
pub use scan::*;
pub use selection::*;
pub use software::*;
pub use welcome::*;
```

**Verification**:
- [ ] Compile with `cargo check --bin mash-setup`
- [ ] Import works correctly

### Step 5: Navigation Context
**File**: `installer-cli/src/tui/app/navigation.rs`

Add to `context_for_screen()` method:
```rust
Screen::Landing => "Main Menu",
```

Add to `go_back()` method:
```rust
Screen::Landing => self.screen = Screen::SystemScan,
```

**Verification**:
- [ ] Compile with `cargo check --bin mash-setup`
- [ ] Navigation context displays correctly

### Step 6: Render Integration
**File**: `installer-cli/src/tui/render.rs`

Add to render match statement:
```rust
Screen::Landing => menus::draw_landing(f, main_area, app),
```

**Verification**:
- [ ] Compile with `cargo check --bin mash-setup`
- [ ] Landing screen renders without errors

### Step 7: Info Box Integration
**File**: `installer-cli/src/tui/info_box.rs`

Add info box content for landing screen:
```rust
Screen::Landing => vec![
    Line::from(Span::styled(
        "Main Forge Menu",
        theme::success_style(),
    )),
    Line::from(Span::styled(
        "Navigate using arrows or number keys",
        theme::dim_style(),
    )),
],
```

**Verification**:
- [ ] Compile with `cargo check --bin mash-setup`
- [ ] Info box displays correct messages

## 🔨 TOOLS & DEPENDENCIES

### Required Tools
- Rust 1.93+ toolchain
- cargo, clippy, rustfmt
- Ratatui 0.28+
- Crossterm for terminal control

### Verification Commands
```bash
# Check compilation
cargo check --bin mash-setup

# Check style
cargo clippy --all-targets -- -D warnings

# Run tests
cargo test --package installer-core

# Format code
cargo fmt
```

## ✅ CHECKPOINT CRITERIA

### Functional Checkpoints
- [ ] `Screen::Landing` enum variant exists and compiles
- [ ] `landing.rs` file created and compiles
- [ ] Menu module exported correctly
- [ ] Navigation context updated
- [ ] Render integration complete
- [ ] Info box integration complete
- [ ] Basic menu displays in TUI

### Quality Checkpoints
- [ ] No compilation errors
- [ ] No clippy warnings
- [ ] Code follows existing style
- [ ] All existing tests still pass
- [ ] Documentation complete

## 📝 NOTES

### Design Decisions
- Menu uses numbered items (1-7) for direct access
- Follows existing cyberpunk terminal aesthetic
- Preserves all existing navigation patterns
- System info display helps users understand their environment

### Potential Issues
- Menu cursor initialization (should default to 0)
- Screen transition from SystemScan to Landing
- Back navigation behavior

### Contingency Plans
- If compilation fails, check for missing imports
- If rendering fails, verify Ratatui widget usage
- If navigation fails, check screen enum ordering

## 🏁 COMPLETION CHECKLIST

- [ ] All code changes implemented
- [ ] All verification steps passed
- [ ] No new warnings or errors
- [ ] Documentation updated
- [ ] Ready for next excavation task (EX_U02)

*"Step by step, the menu takes shape. May your cursor be steady and your selections wise."* — Bard 🍺⚒️