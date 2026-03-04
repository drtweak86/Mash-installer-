# Macro Inventory

## 🎯 Status: ✅ Task 1 Complete

This document catalogs the procedural and declarative macros used across the MASH forge.

## 📦 Crate Macro Dependencies

| Crate | Primary Macros | Purpose |
|-------|----------------|---------|
| `installer-core` | `serde`, `thiserror`, `tracing` | Domain logic, errors, logging |
| `installer-model` | `serde`, `thiserror` | Data structures, serialization |
| `mash-system` | `serde`, `thiserror` | System abstractions, errors |
| `mash-wallpaper` | `serde`, `thiserror` | Wallpaper harvesting logic |
| `installer-cli` | `clap` | CLI argument parsing |

## 📊 Usage Statistics

| Macro | Occurrences | Impact |
|-------|-------------|--------|
| `#[derive(Serialize)]` | 105 | High (transitive deps) |
| `#[derive(Deserialize)]` | 98 | High (transitive deps) |
| `#[derive(ThisError)]` | 12 | Medium |
| `#[async_trait]` | 0 | **REDUNDANT** (Unused in code) |
| `#[tokio::main]` | 1 | Low |

## 🔍 Redundancy Check

*   `async-trait`: Present in `installer-core` and `mash-system` Cargo manifests but **not used** in the current synchronous/decoupled architecture.
*   `serde`: Used extensively for configuration and reporting. Potential for reduction in internal-only structs.

---
"*A clear view of the runes reveals the path to efficiency.*" — Bard 🍺⚒️
