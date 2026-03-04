# Shaft W: THE AESTHETIC GUILD (Presets, Themes & Dotfiles)

**Shaft Title**: THE AESTHETIC GUILD (Presets, Themes & Dotfiles)
**Status**: 🔨 ACTIVE
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️

## 🎯 Objective
Empower the user with high-quality, opinionated configurations and aesthetic presets. Transition the installer from providing "bare binaries" to providing "living, beautiful environments." We will build a system that safely manages dotfiles, themes, and "Presets" (curated combinations of software and aesthetics).

## 📦 Core Deliverables

### 1. The Preset Engine
- **Concept**: A "Preset" is a named configuration that selects specific software tiers, themes, and shell settings.
- **Examples**: "Cyberpunk Neon", "Minimalist Zen", "Retro Terminal".
- **Implementation**: A new `Preset` struct and logic to apply it to the `UserOptionsContext`.

### 2. Dotfile & Theme Management (The "Stow" Protocol)
- **Problem**: We currently copy files. We need a robust way to manage symlinks or copies that handles conflicts (clobbering) safely.
- **Solution**: A "Stow-like" functionality within `installer-core` that:
    - Backs up existing config files (with timestamps).
    - Symlinks or copies new configs from the `resources/` directory.
    - specialized `dotfile_manager` module.

### 3. The Wardrobe (TUI Selector)
- **Concept**: An interactive screen to browse and select themes/presets.
- **UI**: A TUI list/grid showing available presets with descriptions.
- **Preview**: (Optional/Future) ASCII art preview of the theme.

## ⚠️ Risks & Challenges
- **Dotfile Conflicts**: Overwriting user configs is the cardinal sin. We MUST have 100% reliable backup logic.
- **Platform Paths**: Config paths vary between distros (though `XDG_CONFIG_HOME` helps).
- **Complexity**: Presets touch almost every other system (software selection, fonts, shell).

## 🗓️ Phased Approach

### Phase 1: The Preset Engine
Define the data structures for `Preset` and `Theme`. Create the logic to load them from TOML/JSON resources.

### Phase 2: Dotfile Manager
Build the `dotfile` module with `backup`, `link`, and `restore` capabilities. Unit test heavily for collision handling.

### Phase 3: The Wardrobe UI
Create the TUI screen `Screen::Wardrobe` to let the user pick their style.

"*A sharp blade is good, but a beautiful blade inspires the soul.*" — Bard 🍺
