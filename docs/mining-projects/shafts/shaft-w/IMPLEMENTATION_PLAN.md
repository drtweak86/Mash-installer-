# Shaft W: Implementation Plan

## 🛠️ Phase 1: The Preset Engine
**Goal**: Define the data models for "Presets" that bundle software, themes, and configuration overrides.

### Tasks
1.  **Define `Preset` Struct**:
    - Create `installer-core/src/preset.rs`.
    - Struct `Preset` should contain:
        - `name`: String
        - `description`: String
        - `theme`: String (reference to a theme key)
        - `software_selection`: `HashMap<Category, String>` (overrides)
        - `shell_config`: Enum/Struct for shell preferences.
2.  **Define `Theme` Struct**:
    - Update `installer-core/src/theme.rs` to be more robust.
    - Should handle color schemes (hex codes), font preferences, and wallpaper references.
3.  **Resource Loading**:
    - Create `resources/presets/` directory.
    - Add logic to load presets from TOML files.
    - Create a few default presets:
        - `cyberpunk.toml` (Neon colors, tech-heavy software)
        - `minimal.toml` (Monochrome, lightweight tools)
        - `retro.toml` (Gruvbox/Amber, classic tools)

## 🛠️ Phase 2: Dotfile Manager
**Goal**: A robust, conflict-aware system for placing configuration files.

### Tasks
1.  **Create `installer-core/src/dotfiles.rs`**:
    - Implement `DotfileManager`.
2.  **Backup Logic**:
    - Before writing any file to `~/.config/...`, check if it exists.
    - If it exists and has different content, move it to `~/.config/... .bak.<timestamp>`.
    - Record this action in the rollback log.
3.  **Link/Copy Logic**:
    - Support both Symlinking (for easy updates) and Copying (for detached configs).
    - `installer-core` should default to Copying for stability, but Symlinking is a "Dev" profile feature.
4.  **Integration**:
    - Update `installer-core/src/theme.rs` and other config-writing modules to use `DotfileManager` instead of raw `fs::write`.

## 🛠️ Phase 3: The Wardrobe (TUI)
**Goal**: An interactive screen to select the Preset.

### Tasks
1.  **New Screen**: `Screen::Wardrobe` in `installer-cli/src/tui/app.rs`.
2.  **New Menu**: `draw_wardrobe` in `installer-cli/src/tui/menus.rs`.
3.  **Interaction**:
    - List presets.
    - Show description and details in a side panel.
    - "Apply" button updates the `UserOptionsContext` with the preset's choices.
4.  **Flow Integration**:
    - Insert this screen *before* the Software Selection (or replace it for "Auto" mode).
    - Or make it a top-level choice: "Choose a Preset" vs "Manual Configuration".

## 🧪 Verification Plan
- **Unit Tests**:
    - `Preset` loading from TOML.
    - `DotfileManager` backup logic (mock filesystem).
    - `DotfileManager` conflict detection.
- **Manual Verification**:
    - Run the TUI, select "Cyberpunk", verify that correct apps and themes are queued.
    - Run install, verify `~/.config` files are backed up and replaced.
