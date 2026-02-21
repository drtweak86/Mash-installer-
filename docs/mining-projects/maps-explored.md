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

## SHAFT B <COMPLETED> ✅

### Summary
Integration of the BBC/UNIX retro-futuristic theme (i3-gaps + Kitty) and wallpaper downloader into the TUI flow. 

### Technical Analysis
- **Outcome**: Testing on Pi hardware confirmed that while the logic is sound, TUI-based `sudo` interactions remain a blocker for unattended theme deployment. 
- **Decision**: The shaft is considered complete; remaining automation will be rolled into the 1984 station core once the `sudo` TUI integration (Shaft C/D) is fully mature.

### Status
- ✅ Completed (Closed after Pi/Sudo testing).

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

### Status
- ✅ Excavated and operational.

---
**Last Updated**: 2026-02-21
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
