# Mining Projects – Maps Explored
> Historical ledger of completed shafts and sessions with technical diff analysis.

---

## SHAFT A <COMPLETED> ✅

### Summary
Strategic reconnaissance of the forge. Audited architecture, identified shell boundaries, and created the comprehensive strategic plan for retro integration.

### Technical Analysis
- **Architecture**: ~95% pure Rust core. 
- **Shell Boundaries**: Font installation, Docker setup, and Rust toolchain identified as hard shell-out points.
- **Improvements**: Abstracted filesystem operations via `SystemOps` trait.

### Files Touched
- `installer-core/src/system_ops.rs` (new)
- `installer-core/src/phase_context.rs` (updated)
- `installer-core/src/installation_report.rs` (updated)
- `installer-core/src/orchestrator.rs` (updated)
- `installer-core/src/lib.rs` (exports updated)

### Verification
- ✅ 82 tests passing.
- ✅ Clippy & Fmt clean.
- ✅ Cross-compilation verified for x86_64 and aarch64.

---

## SHAFT C <COMPLETED> ✅

### Summary
Complete aesthetic transformation from cyberpunk neon to 1984 BBC Micro/UNIX terminal station.

### Technical Analysis
- **Palette Shift**: Implemented Green/Amber phosphor theme (#00FF00, #FFBF00).
- **Layout Refactor**: Transitioned from a 4-pane cockpit to a single-pane scrolling terminal buffer with a dedicated status bar.
- **Interaction Model**: Implemented numbered command prompts and direct numeric key selection (1-9).
- **Animation logic**: Adjusted station heartbeat (tick rate) to 250ms for authentic retro response times.
- **Error formatting**: Re-engineered `InstallerError` to use industrial uppercase "HALTED_WITH_ERROR" patterns.

### Files Touched
- `installer-cli/src/tui/theme.rs` (palette and style overhaul)
- `installer-cli/src/tui/render.rs` (complete layout and widget rewrite)
- `installer-cli/src/tui/menus.rs` (menu widget and prompt overhaul)
- `installer-cli/src/tui/app.rs` (keyboard handling and tick rate adjustment)
- `installer-core/src/error.rs` (industrial error formatting)
- `docs/mining-projects/shaftc.md` (completed plan)

### Verification
- ✅ cargo check clean.
- ✅ Logic remains intact (function > form).
- ✅ Numeric selection verified.

---

## SHAFT B <NOT_COMPLETED> ⏳

### Summary
Integration of the BBC/UNIX retro-futuristic theme (i3-gaps + Kitty) and wallpaper downloader into the TUI flow.

### Technical Analysis
- **Design**: Reordered the TUI flow: Detection → Profile → Options → Themes → Software → Install.
- **Aesthetics**: Preparation for i3-gaps and Kitty integration.

### Files Touched
- `installer-core/src/software_tiers.rs` (updated)
- `installer-cli/src/tui/menus.rs` (updated)
- `installer-cli/src/tui/app.rs` (updated)
- `docs/mining-projects/shaftb.md` (new)

### Status
- ⏳ Integration Pending...

---
**Last Updated**: 2026-02-21
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
