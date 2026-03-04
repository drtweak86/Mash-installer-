# ⚒️ Shaft Z: The Great Consolidation
> *"Too many shards make a weak blade. We bring the steel back to the center."* — Bard 🍺

## 🎯 OBJECTIVE
Reduce the workspace crate count from **10** down to **5 or 6** to improve developer experience, simplify dependency management, and reduce the overhead of cross-crate maintenance.

## 📋 SCOPE
- **Crates Affected**: `installer-arch`, `installer-fedora`, `installer-debian`, `installer-model`, `mash-system`, `wallpaper-downloader`, `mash-wallpaper`, `workspace-hack`.
- **Target Architecture**: A unified Core (with integrated system and wallpaper logic), a thin UI, a consolidated Driver crate, and essential automation.

## 🛠️ METHODOLOGY
1. **Driver Consolidation**: Merge the three distro-specific crates into a single `installer-drivers` crate using feature flags or module-level isolation.
2. **Core Re-unification**: Merge `installer-model` and `mash-system` back into `installer-core`. These were split to break circular dependencies; we will now use internal modules and shared interfaces to maintain hygiene without the crate overhead.
3. **Feature Integration**: Move `mash-wallpaper` logic and the `wallpaper-downloader` binary into `installer-core` as internal modules/bins.
4. **Workspace Hygiene**: Evaluate and likely remove `workspace-hack` if the reduced crate count makes it redundant for build performance.

## 📦 DELIVERABLES
- **Target Workspace (4-5 Crates)**:
  1. `installer-cli` (The TUI/CLI interface)
  2. `installer-core` (The Heart: Logic, Models, System Types, Wallpapers)
  3. `installer-drivers` (Consolidated: Arch, Fedora, Debian)
  4. `xtask` (Automation & Maintenance)
  5. *Optional*: `workspace-hack` (Only if required for build speed)

---

## 🔧 VERIFICATION
- **Green Build**: `cargo build --workspace` must pass with zero warnings.
- **Full Test Suite**: All 110+ tests must pass across the new consolidated boundaries.
- **Binary Integrity**: `mash-setup` must retain full functionality for all supported distros.

---
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Status**: ✅ COMPLETE
**Last Updated**: 2026-03-04
