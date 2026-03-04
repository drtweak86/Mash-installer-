# EX_Z01: Workspace Consolidation Plan
**Excavation Task**: Crate Reduction & Architecture Flattening
**Status**: ⚒️ PLANNING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-04
**Estimated Duration**: 3 days

## 🎯 OBJECTIVE
Consolidate the workspace from 10 crates down to 5 or 6, achieving architectural simplicity while maintaining code hygiene and test integrity.

## 📋 TASK BREAKDOWN

### 1. The Tri-Distro Forge (Driver Consolidation)
**Objective**: Merge `installer-arch`, `installer-fedora`, and `installer-debian` into a single crate.
- [x] Create `installer-drivers` crate in the workspace.
- [x] Move Arch logic to `installer-drivers/src/arch.rs`.
- [x] Move Fedora logic to `installer-drivers/src/fedora.rs`.
- [x] Move Debian logic to `installer-drivers/src/debian.rs`.
- [x] Implement a unified `Driver` trait or factory in `installer-drivers/src/lib.rs`.
- [x] Update `installer-cli` and `installer-core` to depend on `installer-drivers`.
- [x] Delete original distro crates.

### 2. The Great Re-unification (Core, Model, System Merge)
**Objective**: Merge `installer-model` and `mash-system` back into `installer-core`.
- [x] Move `installer-model/src/` content to `installer-core/src/model/`.
- [x] Move `mash-system/src/` content to `installer-core/src/system/`.
- [x] Resolve internal pathing and imports (using `crate::` where possible).
- [x] Update all workspace crates to use `installer-core` instead of the split crates.
- [x] Delete `installer-model` and `mash-system`.

### 3. The Wallpaper Integration
**Objective**: Integrate `mash-wallpaper` logic and `wallpaper-downloader` as internal components of `installer-core`.
- [x] Move `mash-wallpaper/src/` to `installer-core/src/wallpaper/`.
- [x] Move `wallpaper-downloader/src/main.rs` to `installer-core/src/bin/wallpaper-downloader.rs`.
- [x] Expose wallpaper functionality via `installer-core/src/lib.rs`.
- [x] Update `installer-cli` to call the now-internal wallpaper logic.
- [x] Delete `mash-wallpaper` and `wallpaper-downloader` crates.

### 4. Workspace Hygiene & Root Polish
**Objective**: Cleanup and optimization of the root configuration.
- [x] Update `Cargo.toml` workspace members list.
- [x] Update `[workspace.dependencies]` and remove orphaned references.
- [x] Evaluate `workspace-hack`. Clean build times are fast.
- [x] Finalize `installer-cli` imports to reflect the flattened structure.

## 🧪 VERIFICATION CHECKLIST
- [x] Workspace compiles with `cargo build --workspace`.
- [x] All unit and integration tests pass with `cargo test --workspace`.
- [x] `mash-setup` binary executes and detects the correct distro driver.
- [x] Wallpaper downloading functions as before.
- [x] Crate count is confirmed to be 5.

## 📦 FINAL ARCHITECTURE (5 Crates Target)
1. `installer-cli`
2. `installer-core` (Logic + Models + System + Wallpapers)
3. `installer-drivers` (Consolidated Distros)
4. `xtask`
5. `workspace-hack` (Reviewing necessity)
6. `target` (Hidden, not a crate)

---
*"One crate to rule the logic, one to show the face, one to drive the distro, and one to run the race (xtask)."* — Bard 🍺⚒️
