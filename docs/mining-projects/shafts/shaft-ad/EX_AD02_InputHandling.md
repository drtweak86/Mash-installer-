# 📜 EX_AD02: INPUT HANDLING IMPLEMENTATION

> **Shaft**: AD (Menu-Driven Navigation)
> **Status**: ⏳ **PLANNING**
> **Priority**: ✅ **HIGH**
> **Estimated Duration**: 1 day
> **Guild Member**: Runesmith (Input Handling Specialist)

## 🎯 OBJECTIVE

Implement comprehensive input handling for the landing menu screen, including arrow key navigation, number key shortcuts, and screen selection logic. This excavation task will make the menu fully interactive and functional.

## 📋 DETAILED STEPS

### Step 1: Source of Truth Consultation
- [ ] Read `docs/forge-tavern/maps.md` ✅
- [ ] Read `docs/forge-tavern/maps-explored.md` ✅
- [ ] Review `docs/mining-projects/shafts/shaft-ad/Overview.md` ✅
- [ ] Review `docs/mining-projects/shafts/shaft-ad/EX_AD01_MenuSystem.md` ✅

### Step 2: Add Landing Screen to Input Handler
**File**: `installer-cli/src/tui/app/input.rs`

Add landing screen case to the main input handler:

```rust
Screen::Landing => {
    self.handle_landing_key(code);
}
```

**Location**: In the `handle_key()` method, add this case after `Screen::SystemScan`

### Step 3: Implement Landing Key Handler
**File**: `installer-cli/src/tui/app/input.rs`

Add the `handle_landing_key()` method:

```rust
fn handle_landing_key(&mut self, code: KeyCode) {
    match code {
        // Arrow key navigation (up/down)
        KeyCode::Up | KeyCode::Char('k') => {
            self.handle_landing_navigation(-1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            self.handle_landing_navigation(1);
        }
        
        // Number key shortcuts (1-7)
        KeyCode::Char('1') => {
            self.menu_cursor = 0;
            self.handle_landing_selection();
        }
        KeyCode::Char('2') => {
            self.menu_cursor = 1;
            self.handle_landing_selection();
        }
        KeyCode::Char('3') => {
            self.menu_cursor = 2;
            self.handle_landing_selection();
        }
        KeyCode::Char('4') => {
            self.menu_cursor = 3;
            self.handle_landing_selection();
        }
        KeyCode::Char('5') => {
            self.menu_cursor = 4;
            self.handle_landing_selection();
        }
        KeyCode::Char('6') => {
            self.menu_cursor = 5;
            self.handle_landing_selection();
        }
        KeyCode::Char('7') => {
            self.menu_cursor = 6;
            self.handle_landing_selection();
        }
        
        // ENTER/Space for selection
        KeyCode::Enter | KeyCode::Char(' ') => {
            self.handle_landing_selection();
        }
        
        // ESC for back navigation
        KeyCode::Esc => {
            self.go_back();
        }
        
        // Ignore other keys
        _ => {}
    }
}
```

### Step 4: Implement Navigation Helper
**File**: `installer-cli/src/tui/app/input.rs`

Add the `handle_landing_navigation()` helper method:

```rust
fn handle_landing_navigation(&mut self, direction: i32) {
    let menu_items = 7; // 0-6
    
    if direction < 0 {
        // Up navigation
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = menu_items - 1; // Wrap to bottom
        }
    } else {
        // Down navigation
        if self.menu_cursor < menu_items - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0; // Wrap to top
        }
    }
}
```

### Step 5: Implement Selection Logic
**File**: `installer-cli/src/tui/app/input.rs`

Add the `handle_landing_selection()` method:

```rust
fn handle_landing_selection(&mut self) {
    match self.menu_cursor {
        0 => {
            // Distribution Selection
            self.navigate_to(Screen::DistroSelect, "Distribution Selection");
            self.menu_cursor = 0;
        }
        1 => {
            // Profile Selection
            self.navigate_to(Screen::ProfileSelect, "Profile Selection");
            self.menu_cursor = 1; // Default to Dev
        }
        2 => {
            // System Summary
            self.navigate_to(Screen::SystemSummary, "System Results & Wisdom");
            self.menu_cursor = 0;
        }
        3 => {
            // Theme Selection
            self.navigate_to(Screen::ThemeSelect, "Theme Selection");
            self.menu_cursor = 0;
        }
        4 => {
            // Software Selection
            self.navigate_to(Screen::SoftwareMode, "Software Selection Mode");
            self.menu_cursor = 0;
        }
        5 => {
            // Advanced Configuration
            if self.platform_info.pi_model.is_some() {
                self.navigate_to(Screen::ArgonConfig, "Argon One Configuration");
            } else {
                self.navigate_to(Screen::DockerConfig, "Docker Configuration");
            }
            self.menu_cursor = 0;
        }
        6 => {
            // Start Installation
            self.start_install();
        }
        _ => {}
    }
}
```

### Step 6: Verify Compilation
```bash
cargo check --bin mash-setup
cargo clippy --all-targets -- -D warnings
```

**Expected Result**: Clean compilation with no errors or warnings

### Step 7: Basic Testing
1. Launch the TUI: `cargo run --bin mash-setup -- --demo`
2. Navigate to System Scan (let it complete)
3. Verify Landing screen appears
4. Test arrow key navigation
5. Test number key shortcuts (1-7)
6. Test ENTER selection
7. Test ESC back navigation
8. Verify all menu items route correctly

## 🔨 TOOLS & DEPENDENCIES

### Required Tools
- Rust 1.93+ toolchain
- cargo, clippy, rustfmt
- Existing Ratatui/Crossterm dependencies

### Guild Member Specialization
- **Runesmith**: Input handling and navigation logic
- **Task Type**: Code implementation
- **Complexity**: Medium
- **Estimated Time**: 2-3 hours

## ✅ CHECKPOINT CRITERIA

### Functional Checkpoints
- [ ] Landing screen responds to arrow keys
- [ ] Number keys (1-7) work as shortcuts
- [ ] ENTER/SPACE selects menu items
- [ ] ESC returns to SystemScan screen
- [ ] All 7 menu items route to correct screens
- [ ] Menu cursor wraps correctly (top/bottom)
- [ ] No panics or crashes during navigation

### Quality Checkpoints
- [ ] Clean compilation (no errors/warnings)
- [ ] Code follows existing patterns
- [ ] Proper error handling
- [ ] No memory leaks or performance issues
- [ ] All existing tests still pass

## 📝 NOTES

### Design Decisions
- **Vim-style navigation**: Support both arrow keys and hjkl
- **Number shortcuts**: Direct access to any menu item
- **Cursor wrapping**: Smooth navigation between top/bottom
- **Context-aware routing**: Advanced config checks Pi model
- **Reuse existing methods**: `navigate_to()`, `go_back()`, `start_install()`

### Potential Issues
- Menu cursor initialization (should be 0)
- Platform info availability (check for None)
- Screen transition timing
- Input focus conflicts

### Contingency Plans
- Add bounds checking for menu cursor
- Handle missing platform_info gracefully
- Add debug logging for navigation issues
- Test on multiple terminal types

## 🏁 COMPLETION CHECKLIST

- [ ] All input handling implemented
- [ ] All verification steps passed
- [ ] Clean compilation achieved
- [ ] Basic testing completed
- [ ] Documentation updated
- [ ] Ready for EX_AD03 (navigation logic)

*"May your keys be responsive, your cursor steady, and your selections wise."* — Bard 🍺⚒️