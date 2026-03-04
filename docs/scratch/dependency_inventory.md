# Dependency Inventory & Reduction Report

## 🎯 Status: ⏳ Phase 3 Implementation In Progress

## 📋 Completed Reductions

| Dependency | Action | Location | Impact |
|------------|--------|----------|--------|
| `slog` | Removed | `installer-core` | Replaced with `tracing`. Reduced 3 dependencies. |
| `slog-term` | Removed | `installer-core` | Part of slog removal. |
| `slog-async` | Removed | `installer-core` | Part of slog removal. |
| `chrono` | Removed | Workspace | Replaced with `std::time::SystemTime` for backups. |
| `lazy_static` | Removed | Workspace | Replaced with `std::sync::LazyLock`. |
| `log` | Removed | `mash-system` | Unused. |
| `num_cpus` | Removed | `installer-core` | Replaced with `std::thread::available_parallelism`. |
| `bytes` | Removed | `installer-core` | Removed from `Cargo.toml` (still transitive). |
| `sysinfo` | Removed | Workspace | Replaced with manual `/proc` parsing. Saved ~500KB. |
| `wallpaper-downloader` | Unified | Workspace | Moved logic to new `mash-wallpaper` crate. Consolidated dependencies. |
| `reqwest` | Replaced | `mash-wallpaper` | Replaced with `ureq` (blocking). Shed heavy transitive deps (hyper, tokio-rustls). |

## 📦 Current Inventory (Key Dependencies)

### Core Dependencies
| Dependency | Version | Used By | Purpose | Necessity |
|------------|---------|---------|---------|-----------|
| `anyhow` | 1.0 | Workspace | Error handling | High |
| `tokio` | 1.50 | Workspace | Async runtime | High |
| `serde` | 1.0 | Workspace | Serialization | High |
| `tracing` | 0.1 | Workspace | Logging | High |
| `ratatui` | 0.30 | `installer-cli` | TUI | High |
| `ureq` | 2.12 | `mash-wallpaper` | HTTP | Low (could be replaced by curl) |
| `rusqlite` | 0.38 | `mash-wallpaper` | State tracking | Medium (could be replaced) |

### Heavy Dependencies (>1MB binary impact)
| Dependency | Est. Size | Used By | Purpose | Alternatives |
|------------|-----------|---------|---------|--------------|
| `rusqlite` | ~1.5MB | Wallpaper | SQLite | JSON/KV store |

## 📊 Phase 3 Impact Summary

*   **Binary Size**: Reduced from **13.7 MiB** to **4.5 MiB** (~67% reduction).
*   **Dependency Count**: 10+ direct dependencies removed.
*   **Transitive Deps**: Hundreds of transitive dependencies shed (hyper, rustls, tokio-rustls, etc.).
*   **Build Time**: Significant improvement in clean build times due to smaller dependency graph.

## 🔍 Remaining Opportunities

1. **Replace rusqlite**:
   - Use a simple JSON file or KV store for wallpaper state if possible.
   - Benefit: Save another ~1.5MB.

2. **curl-based downloads**:
   - Instead of `ureq`, use the system `curl` via `mash-system::cmd`.
   - Benefit: Remove `ureq` and its dependencies entirely.

3. **Optimize Feature Flags**:
   - Review `indicatif` and `tracing-subscriber` features.

---
"*A lean forge is a fast forge.*" — Bard 🍺⚒️
