# ⚒️ The Miner's Active Maps: Current Shaft
> *"Two shafts remain. One to harden the blade. One to ship it."* — Bard 🍺

---

## ✅ SHAFT J: WALLPAPER DOWNLOADER RUST CONVERSION — COMPLETE

All previous shafts (A through J) are complete. See `maps-explored.md` for full history.

---

## ⛏️ SHAFT K: FORGE HARDENING — ACTIVE
> *Branch*: `work-shaftj-phase1` → new `work-shaftk-hardening`
> *Detailed plan*: `docs/scratch/shaft-k.md`

**Objective**: Structural cleanup — commit baseline, purge legacy artifacts, fold thin shims,
consolidate duplicate crate logic, align deps.

### PHASE 1: BASELINE COMMIT ✅ COMPLETE — Checkpoint α
- [x] K1.1 Run build trinity: fmt clean | clippy clean | 107 tests passing
- [x] K1.2 Staged and committed 49 files (commit f89d203)
- [ ] K1.3 Open PR on `work-shaftj-phase1` → CI green → merge

### PHASE 2: LEGACY ARTIFACT PURGE ✅ COMPLETE — Checkpoint β
- [x] K2.1 Deleted `resources/themes/retro-bbc/wallpaper_downloader_final.py`
- [x] K2.2 Deleted `docs/incoming-files/wallpaper_downloader_final.py`
- [x] K2.3 Deleted `docs/incoming-files/eza-aliases.sh` (staging duplicate)
- [x] K2.4 `resources/shell/eza_aliases.sh` confirmed kept as resource data file
- Note: docs/scratch/wallpaper_downloader-1.py is scratch — will age out via hygiene

### PHASE 3: THIN SHIM ELIMINATION ✅ COMPLETE — Checkpoint γ
- [x] K3.1 Deleted `installer-core/src/registry.rs` (was 1-line re-export)
- [x] K3.2 Updated `lib.rs`: `pub use phase_registry::PhaseRegistry`
- [x] K3.3 Deleted `installer-core/src/runner.rs` (was 4-line re-export)
- [x] K3.4 Updated `lib.rs`: `pub use phase_runner::{Phase, PhaseRunner, ...}`
- [x] K3.5 Build check: green (2m 40s, 0 errors)

### PHASE 4: CRATE CONSOLIDATION ✅ COMPLETE — Checkpoint δ
- [x] K4.1 wallpaper-downloader consolidation — PARTIAL (design decision required)
  - [x] K4.1.d Bumped wallpaper-downloader version to 0.2.3 (workspace aligned)
  - NOTE: Full thin-CLI fold deferred to Shaft L Phase 1 — download_wallpapers() takes
    &mut PhaseContext which must be decoupled first (architectural decision needed)
  - Declared in shaft-l.md as design-first task
- [x] K4.2 Deleted `.github/workflows/rust.yml` (subsumed by ci.yml)

### PHASE 5: DEPENDENCY HYGIENE
- [ ] K5.1 Align `indicatif`: 0.17 → 0.18 everywhere
- [ ] K5.2 Align `which`: v4 (cli) + v7 (core) → pick one
- [ ] K5.3 Align `once_cell`: evaluate → migrate to `std::sync::OnceLock` (stable since 1.70)
- [ ] K5.4 Assess toolchain upgrade: 1.93.1 → 1.85.0
- [ ] K5.5 `cargo update` + `cargo audit` — confirm zero vulnerabilities
- [ ] K5.6 Run build trinity — confirm green

### PHASE 6: SHAFT K COMMIT
- [ ] K6.1 Commit per phase (atomic commits, descriptive messages)
- [ ] K6.2 Open PR `work-shaftk-hardening` → CI green → merge

**Risk**: LOW | **Reward**: HIGH

---

## ⬜ SHAFT L: FINAL RELEASE CLEARANCE — PENDING
> *Starts after Shaft K is merged*
> *Detailed plan*: `docs/scratch/shaft-l.md`

**Objective**: Code quality pass + UX improvements + docs + release gate → v1.0.0 tag.

### PHASE 1: CODE QUALITY
- [ ] L1.1 Audit `verify.rs` (marked `#[allow(dead_code)]`) — connect or delete
- [ ] L1.2 Audit `ai_agents.rs` — confirm purpose, add doc comment or refactor gate
- [ ] L1.3 Migrate `once_cell` usages to `std::sync::OnceLock` (if K5.3 deferred)
- [ ] L1.4 Clarify boundary between `installer-cli/src/software_tiers.rs` (UI)
       and `installer-core/src/software_tiers.rs` (data) with doc comments

### PHASE 2: UX IMPROVEMENTS
- [ ] L2.1 Wallpaper API key env var support (MASH_WALLHAVEN_KEY, MASH_PEXELS_KEY, MASH_PIXABAY_KEY)
- [ ] L2.2 Doctor mode wallpaper API key check (PASS/WARN + setup URLs)
- [ ] L2.3 `include_str!()` eza_aliases.sh into zsh.rs (embed resource, remove .sh from flow)

### PHASE 3: DOCUMENTATION
- [ ] L3.1 HISTORY.md — bardic entry for Shaft J (wallpaper downloader Rust conversion)
- [ ] L3.2 HISTORY.md — bardic entry for Shaft K (forge hardening)
- [ ] L3.3 MANUAL.md — refresh for current feature set (wallpapers, Pi tuning, TUI, API keys)
- [ ] L3.4 Run `check_docs.rs` — fix all broken links

### PHASE 4: FINAL RELEASE GATE
- [ ] L4.1 Run `release_checklist.rs` — all gates green
- [ ] L4.2 `cargo audit` — zero vulnerabilities
- [ ] L4.3 `shellcheck install.sh` — clean
- [ ] L4.4 Version consistency: all crates at 0.2.3 (or bump to 1.0.0)
- [ ] L4.5 Merge to main — CI green
- [ ] L4.6 Run `auto_bump.rs major` → 1.0.0 (0.x.x → 1.0.0)
- [ ] L4.7 `git tag v1.0.0 && git push --tags`
- [ ] L4.8 GitHub Actions release pipeline fires → .deb, .rpm, PKGBUILD published

**Risk**: LOW | **Reward**: MAXIMUM (v1.0.0 shipped)

---

## DEFERRED (Post v1.0.0)
- reqwest 0.11 → 0.12 upgrade (HIGH RISK, medium reward)
- scripts/*.rs → `xtask` crate (LOW RISK, medium reward, quality-of-life)
- cargo-release integration (replaces auto_bump.rs)
- Dependabot configuration for automated dep updates
- Multi-distro parallel CI test matrix
- BBS message bank expansion (44 → 60+)
- mash-setup status subcommand
- mdBook documentation expansion

---

## CRITICAL PATH

```
SHAFT K ──► SHAFT L ──► v1.0.0 TAG
(hardening)  (quality)   (ship it)
```

---

**Last Updated**: 2026-02-22
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
