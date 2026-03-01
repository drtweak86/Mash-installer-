# Shaft W: THE AESTHETIC GUILD (Presets, Themes & Dotfiles)

**Shaft Title**: THE AESTHETIC GUILD (Presets, Themes & Dotfiles)
**Status**: ⏳ PLANNING COMPLETE | 🌑 IMPLEMENTATION PENDING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-01

## 🎯 SCOPE

This shaft empowers the user with high-quality, opinionated configurations and aesthetic presets. It transitions the installer from providing "bare binaries" to providing "living, beautiful environments."

1. **The Preset Engine**: Offer intelligent software/theme combinations based on user selection.
2. **Dotfile & Theme Repositories**: Integrated, curated configurations for ZSH, Starship, Kitty, and more.
3. **Interactive Configuration Selector**: A TUI "Wardrobe" screen for fine-tuning themes and icons.
4. **Robust Dotfile Management**: Clobber-safe symlink management with automatic backups.

## 📁 FILES TO BE CREATED OR TOUCHED

### New Files
- `installer-core/src/presets.rs` - Preset logic and database
- `installer-core/src/dotfiles.rs` - Symlink and backup management
- `installer-cli/src/tui/wardrobe.rs` - The Aesthetic Selection screen
- `resources/themes/modern_neon.toml` - New neon theme preset
- `resources/themes/retro_dwarf.toml` - New dwarven retro theme preset

### Modified Files
- `installer-core/src/config.rs` - Support for theme/preset configuration
- `installer-cli/src/tui/menus.rs` - Add Aesthetic Selection stage to flow
- `installer-cli/src/tui/app.rs` - Integrate preview/wardrobe logic
- `installer-core/src/lib.rs` - Export new aesthetic modules

## ⚒️ METHODOLOGY

### Technical Strategy
1. **Preset Matching**: Analyze `SoftwarePlan` to suggest relevant aesthetic presets (e.g., if Kitty and ZSH are selected, suggest "Neon Terminal").
2. **Backup Before Write**: Every dotfile write must first backup existing files to `~/.mash-backups/`.
3. **Template Substitution**: Use `handlebars` or similar to inject system-specific values (e.g., font names) into dotfile templates.
4. **Theme Preview**: Provide descriptive summaries and mock-ups in the TUI to guide the user's choice.

## 📦 DELIVERABLES

### Phase 1: The Preset Engine ✅ PLANNED
- [ ] Implement `PresetDetector` that suggests combos based on `SoftwarePlan`.
- [ ] Implement `presets.toml` database.
- [ ] Add "Preset Selection" modal to the installer flow.

### Phase 2: Dotfile & Theme Management ✅ PLANNED
- [ ] Implement `DotfileManager` with backup and symlink logic.
- [ ] Add `Oh My Zsh`, `Powerlevel10k`, and `Pure` theme options.
- [ ] Add `Kitty` color schemes and `Starship` prompt variations.
- [ ] Implement `colorls` and `lsd` configuration support.

### Phase 3: The Wardrobe (TUI Selector) ✅ PLANNED
- [ ] Create the "Wardrobe" screen for interactive theme selection.
- [ ] Implement "Preview Mode" for font and color combinations.
- [ ] Add "Applied Theme" visual confirmation.

## 🔧 VERIFICATION CHECKLIST
- [ ] Correct presets are suggested based on software selection.
- [ ] Existing dotfiles are backed up correctly before being replaced.
- [ ] Symlinks are created and point to the correct internal resources.
- [ ] TUI previews accurately reflect the configurations being applied.

"*A dwarf's home is his forge, but his terminal is his pride. Time to make it shine.*" — Bard 🍺⚒️
