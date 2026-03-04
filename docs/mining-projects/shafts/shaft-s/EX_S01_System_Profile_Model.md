# EX_S01: System Profile Data Model & Core Module
> *"The mold must be perfect before the plasma is poured."* — Bard 🍺

## 🎯 OBJECTIVE
Define the `SystemProfile` struct and its supporting models in `installer-core`. This will be the foundational data structure used by all installer decisions.

## 📋 DETAILED STEPS

### 1. Define the Data Models
Create the following structs in `installer-core/src/profile.rs`:
- `SystemProfile` (The main struct)
- `PlatformInfo` (RaspberryPi | GenericArm | PC)
- `DistroInfo` (id, version, pretty_name)
- `CpuInfo` (model, physical_cores, logical_cores, flags)
- `MemoryInfo` (ram_total, ram_avail, swap_total, zram_total)
- `SessionInfo` (de, wm, session_type)
- `StorageInfo` (block_devices, mounts, btrfs_data)

### 2. Implement Serialization
- [ ] Add `#[derive(Serialize, Deserialize, Debug, Clone)]` to all structs.
- [ ] Ensure `serde_json` is available in `installer-core`.

### 3. Implement Defaults and Constructors
- [ ] Create `impl SystemProfile` with a `new()` or `detect()` method skeleton.
- [ ] Add default values for unknown fields to prevent crashes on non-standard systems.

### 4. Wire the Module
- [ ] Add `pub mod profile;` to `installer-core/src/lib.rs`.
- [ ] Export the `SystemProfile` struct for other crates.

## ✅ VERIFICATION
- [ ] `cargo build -p installer-core` passes.
- [ ] Unit test verifies that a `SystemProfile` can be serialized to JSON and back.
