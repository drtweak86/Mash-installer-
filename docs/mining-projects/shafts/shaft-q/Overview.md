# Shaft Q Design Document: Wallpaper Consolidation

**Status**: AWAITING APPROVAL — Phase 1 complete, no code written
**Author**: Bard 🍺
**Date**: 2026-02-23

---

## 1. The Question

> Should `wallpaper-downloader/` share code with `installer-core/src/wallpaper/`,
> or stay separate with a common CLI entry point?

---

## 2. Audit Findings

### 2.1 installer-core/src/wallpaper/ (phase-integrated)

**Purpose**: Downloads wallpapers as part of a MASH install phase.

**API surface**:
```rust
pub async fn download_wallpapers(
    config: &WallpaperConfig,
    _system_ops: &dyn SystemOps,
    phase_ctx: &mut PhaseContext<'_>,  // ← THE COUPLING
) -> Result<DownloadStats, WallpaperError>
```

**PhaseContext usage** (the coupling):
- `phase_ctx.record_action(msg)` — 2 calls (start + completion messages)
- `phase_ctx.record_warning(msg)` — 4 calls (when no API keys are set)

**Provider coverage**: Wallhaven + Pexels + Pixabay (via `WallpaperApi` trait)

**File writes**: Through `RealSystem` (SystemOps abstraction) → temp→rename
**Output**: `/usr/share/backgrounds/retro/` (system-wide, requires sudo)
**Deduplication**: None
**reqwest**: 0.11

---

### 2.2 wallpaper-downloader/ (standalone binary)

**Purpose**: Standalone CLI to bulk-download a curated retro wallpaper library.

**API surface**:
```
wallpaper-downloader [--category all|retro|games|...] [--limit N] [--api-key KEY]
```

**Provider coverage**: Wallhaven ONLY (`ApiClient` — concrete, not trait-based)

**Features NOT present in installer-core**:
- SHA256 hash deduplication (avoids re-downloading identical images)
- URL cache (skips already-processed URLs in-session)
- i3 config auto-injection (`~/.config/i3/config`)
- GNOME gsettings auto-set
- `--first-boot` flag (minimal output for scripted/install-time use)
- Per-category filter (`--category`)
- Download limit (`--limit`)
- Configurable timeout (`--timeout`)
- Category-per-subdirectory output layout

**File writes**: Raw `tokio::fs` — no SystemOps abstraction
**Output**: `~/Pictures/RetroWallpapers/` (user-local, no sudo)
**reqwest**: 0.11
**Environment variable**: `WALLHAVEN_API_KEY` (does NOT match `MASH_WALLHAVEN_KEY`)

---

### 2.3 What Is Actually Shared?

| Concept          | installer-core version | wallpaper-downloader version | Divergence |
|------------------|------------------------|------------------------------|------------|
| Category type    | `WallpaperCategory` (name, display_name, queries, target_count) | `Category` (name, display_name, queries, count) | Field name `target_count` vs `count` |
| Stats type       | `DownloadStats` (success, failed, total) | `DownloadStats` (success_count, fail_count, start_time) | Different fields |
| Download pattern | temp→rename via RealSystem | temp→rename via tokio::fs | Different abstraction |
| Semaphore limit  | `Arc<Semaphore>` | `Arc<Semaphore>` | Identical pattern |
| API key env var  | `MASH_WALLHAVEN_KEY` | `WALLHAVEN_API_KEY` | **MISMATCH** |

**Verdict**: ~50 lines of structural overlap. No shared business logic worth extracting.

---

## 3. Options Evaluated

### Option A: Extract a shared `wallpaper-core` sub-crate

Create `wallpaper-core/` with shared types, traits, and download logic. Both crates depend on it.

**Pros**: True single source of truth for types
**Cons**:
- Adds a 3rd wallpaper crate to the workspace
- The actual shareable code is ~50 lines of types
- Still requires solving the PhaseContext coupling
- Violates KISS for marginal gain

**Verdict**: ❌ Over-engineered for the problem size.

---

### Option B: Thin CLI delegates to installer-core

`wallpaper-downloader` depends on `installer-core` and calls `download_wallpapers()` directly.

**Problem**: `download_wallpapers()` requires `&mut PhaseContext<'_>`. PhaseContext carries
all of installer-core's context (options, platform, UI, interaction, localization, rollback,
dry-run log). Constructing one outside a MASH install phase is artificial and fragile.

To make this work, installer-core's wallpaper module would need to be refactored to accept
a simpler progress callback instead of PhaseContext.

**Additionally**: wallpaper-downloader has features installer-core lacks (deduplication, i3
integration, user-local output). The thin CLI would lose these or have to re-add them.

**Verdict**: ❌ High coupling cost, feature regression risk, non-trivial refactor.

---

### Option C: Keep them separate, fix the env var mismatch (RECOMMENDED)

Acknowledge that these are **two different tools serving different use cases**:

| Dimension          | installer-core/wallpaper    | wallpaper-downloader          |
|--------------------|-----------------------------|-------------------------------|
| Invoked by         | MASH install phase          | User / first-boot script      |
| API providers      | Wallhaven + Pexels + Pixabay| Wallhaven only                |
| Output dir         | `/usr/share/backgrounds/` (system) | `~/Pictures/RetroWallpapers/` (user) |
| Deduplication      | None                        | SHA256 + URL cache            |
| Desktop integration| No (separate phases)        | Yes (i3 config, gsettings)    |
| Progress reporting | PhaseContext (TUI/dry-run)  | log + stdout                  |
| Dry-run aware      | Yes (via PhaseContext)       | No                            |

These are **complementary**, not competing. The installer-core version integrates with MASH's
phase system; the standalone version is a power tool for bulk downloads.

**One real bug to fix**: `wallpaper-downloader` reads `WALLHAVEN_API_KEY` but the documented
env var is `MASH_WALLHAVEN_KEY`. This is a one-line fix that aligns the two tools.

**Verdict**: ✅ KISS wins. Keep them separate. Fix the env var mismatch.

---

### Option D: Adapter pattern (de-couple PhaseContext from wallpaper)

Define a `WallpaperProgress` trait in installer-core:

```rust
pub trait WallpaperProgress {
    fn record_action(&self, msg: &str);
    fn record_warning(&self, msg: &str);
}
```

`PhaseContext` implements `WallpaperProgress`. `wallpaper-downloader` provides `StdioProgress`.
`download_wallpapers` accepts `&dyn WallpaperProgress` instead of `&mut PhaseContext<'_>`.

**Pros**: Breaks the coupling cleanly; enables future sharing
**Cons**:
- Still doesn't solve the feature gap (deduplication, i3 integration)
- Medium blast radius (changes installer-core's public API)
- wallpaper-downloader would then depend on installer-core (new dependency edge)
- For 6 lines of `record_action/record_warning` usage, the cost exceeds the benefit

**Verdict**: ❌ Valid architecture, but the cost-benefit ratio is poor at this scale.
Could be reconsidered post-reqwest 0.12 upgrade when both codebases are touched anyway.

---

## 4. Recommendation: Option C

**Keep them separate. Fix the env var mismatch.**

### Scope (minimal — 2 files max)

1. **`wallpaper-downloader/src/config.rs`** — change `WALLHAVEN_API_KEY` → `MASH_WALLHAVEN_KEY`
   (the downloader should read the same key as the installer does)

2. **`docs/src/features/wallpapers.md`** — update the `wallpaper-downloader` section to note
   that both tools use `MASH_WALLHAVEN_KEY`

### What stays the same

- Both codebases remain independent — no new dependency edges
- No refactor of PhaseContext or WallpaperApi trait
- No new crates
- No feature regressions in either tool

### Why this is correct

The 1.0 Threshold law (Law #8) applies: we have a shipped, working system. The consolidation
would not add user-visible value proportional to its risk. The only real user-facing problem
is the env var mismatch — and that's a one-liner.

---

## 5. Implementation (post-approval)

If Option C is approved, Phase 2 is:

- [ ] Q2.1 `wallpaper-downloader/src/config.rs`: read `MASH_WALLHAVEN_KEY` (was `WALLHAVEN_API_KEY`)
- [ ] Q2.2 `docs/src/features/wallpapers.md`: note the unified env var
- [ ] Q2.3 Build + test
- [ ] Q2.4 maps.md — Shaft Q complete

**Risk**: VERY LOW (one env var name change, no logic changes)
**Reward**: Removes the only real user-facing inconsistency between the two tools

---

*Bard, Drunken Dwarf Runesmith — Forge Tavern, 2026-02-23*
