# Macro Complexity Analysis

## 🎯 Status: ✅ Task 2 Complete

Analysis of procedural macro complexity and identification of optimization opportunities.

## ⚖️ High-Complexity Targets (> 5 Derives)

The following structs/enums have dense derive blocks, which increases compile time and metadata bloat.

| File | Symbol | Derives |
|------|--------|---------|
| `installer-model/src/config/mod.rs` | `Config` | `Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default` |
| `installer-model/src/options/mod.rs` | `InstallOptions` | `Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize` |
| `installer-core/src/pi_overlord.rs` | `PiModel` | `Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize` |
| `installer-core/src/advice.rs` | `Advice` | `Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord` |

## 🛠️ Recommended Optimizations

### 1. Prune Serde Usage
*   **Target**: Internal state structs that are never persisted or returned as JSON.
*   **Action**: Remove `Serialize` and `Deserialize` from structs used only during phase execution.

### 2. Dependency Removal: `async-trait`
*   **Status**: Confirmed unused across all traits.
*   **Action**: Remove from `installer-core` and `mash-system` Cargo manifests.

### 3. Manual Error Impls
*   **Target**: Simple error enums with 1-2 variants.
*   **Action**: Replace `thiserror` with manual `Display` and `Error` trait implementations to reduce proc-macro server overhead.

### 4. Consolidated Features
*   **Action**: Ensure `serde` is used with `default-features = false` where `derive` is the only requirement.

---
"*Simplicity is the final rune of mastery.*" — Bard 🍺⚒️
