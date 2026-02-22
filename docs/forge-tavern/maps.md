# ⚒️ The Miner's Active Maps: Current Shaft
> *"The blade is hardened. The crown is on the table."* — Bard 🍺

---

## ✅ SHAFT J: WALLPAPER DOWNLOADER RUST CONVERSION — COMPLETE

All previous shafts (A through J) are complete. See `maps-explored.md` for full history.

---

## ✅ SHAFT K: FORGE HARDENING — COMPLETE
> *Branch*: `work-shaftj-phase1`
> *PR*: [#35](https://github.com/drtweak86/Mash-installer/pull/35) — awaiting CI green + merge

### PHASE 1: BASELINE COMMIT ✅ COMPLETE — Checkpoint α
- [x] K1.1 Run build trinity: fmt clean | clippy clean | 107 tests passing
- [x] K1.2 Staged and committed 49 files (commit f89d203)
- [x] K1.3 PR #35 opened on `work-shaftj-phase1` → CI → merge

### PHASE 2: LEGACY ARTIFACT PURGE ✅ COMPLETE — Checkpoint β
- [x] K2.1 Deleted `resources/themes/retro-bbc/wallpaper_downloader_final.py`
- [x] K2.2 Deleted `docs/incoming-files/wallpaper_downloader_final.py`
- [x] K2.3 Deleted `docs/incoming-files/eza-aliases.sh` (staging duplicate)
- [x] K2.4 `resources/shell/eza_aliases.sh` confirmed kept as resource data file

### PHASE 3: THIN SHIM ELIMINATION ✅ COMPLETE — Checkpoint γ
- [x] K3.1 Deleted `installer-core/src/registry.rs` (was 1-line re-export)
- [x] K3.2 Updated `lib.rs`: `pub use phase_registry::PhaseRegistry`
- [x] K3.3 Deleted `installer-core/src/runner.rs` (was 4-line re-export)
- [x] K3.4 Updated `lib.rs`: `pub use phase_runner::{Phase, PhaseRunner, ...}`
- [x] K3.5 Build check: green

### PHASE 4: CRATE CONSOLIDATION ✅ COMPLETE — Checkpoint δ
- [x] K4.1.d Bumped wallpaper-downloader version to 1.0.0 (workspace aligned)
  - NOTE: Full thin-CLI fold deferred post-v1.0.0 (PhaseContext coupling architectural decision)
- [x] K4.2 Deleted `.github/workflows/rust.yml` (subsumed by ci.yml)

### PHASE 5: DEPENDENCY HYGIENE ✅ COMPLETE — Checkpoint ε
- [x] K5.1 Aligned `indicatif`: core 0.17 → 0.18 (matches installer-cli)
- [x] K5.2 Removed dead `which = "4"` from installer-cli
- [x] K5.3 `once_cell` → `std::sync::OnceLock` — COMPLETE (sudo_password.rs; dep removed)
- [x] K5.4 Toolchain — CONFIRMED current (1.93.1 IS stable tip)
- [x] K5.5 Build: fmt clean | clippy clean | build green

### PHASE 6: SHAFT K COMMIT ✅ COMPLETE — Checkpoint ζ
- [x] K6.1 Commits: f89d203 | 3a7b7e0 | e4430b2 | dfbfe16 | d73bec3
- [x] K6.2 PR #35 opened `work-shaftj-phase1` → main

**Risk**: LOW | **Reward**: HIGH

---

## ✅ SHAFT L: FINAL RELEASE CLEARANCE — COMPLETE (pending PR merge + tag)
> *PR*: #35 (includes all Shaft L commits)
> *Next action*: CI green → merge → `git tag v1.0.0 && git push --tags`

**Objective**: Code quality pass + UX improvements + docs + release gate → v1.0.0 tag.

### PHASE 1: CODE QUALITY ✅ COMPLETE
- [x] L1.1 verify.rs: changed `#[allow(dead_code)] mod` → `pub mod` (exposed as public API)
- [x] L1.2 ai_agents.rs: confirmed wired in phase_registry.rs:129; added module doc comment
- [x] L1.3 once_cell → OnceLock — DONE in K5.3
- [x] L1.4 software_tiers boundary: doc comments added to both core (data+install) and cli (UI)

### PHASE 2: UX IMPROVEMENTS ✅ COMPLETE
- [x] L2.1 WallpaperConfig::with_env_keys() — reads MASH_WALLHAVEN_KEY, MASH_PEXELS_KEY, MASH_PIXABAY_KEY
- [x] L2.2 Doctor mode: "Wallpaper API keys" section — PASS/WARN per key + setup URLs
- [x] L2.3 include_str!() eza_aliases.sh — CONFIRMED already done in zsh.rs:23

### PHASE 3: DOCUMENTATION ✅ COMPLETE
- [x] L3.1 HISTORY.md: bardic entry for Shaft J (wallpaper Rust conversion)
- [x] L3.2 HISTORY.md: bardic entry for Shaft K (forge hardening, cron, laws)
- [x] L3.3 MANUAL.md: full refresh (TUI, wallpapers, Pi tuning, doctor, API keys, AI Spirits)
- [x] L3.4 check_docs.rs: 4 broken links fixed in QA_SUMMARY.md — passes clean

### PHASE 4: FINAL RELEASE GATE ✅ COMPLETE (local)
- [x] L4.1 release_checklist.rs: fmt clean | clippy clean | 110 tests passing | docs clean
- [x] L4.2 cargo audit: via CI (not installed locally on Pi)
- [x] L4.3 shellcheck install.sh: clean (zero warnings)
- [x] L4.4 Version consistency: all 6 crates bumped to 1.0.0
- [ ] L4.5 Merge to main — awaiting CI green on PR #35
- [ ] L4.7 `git tag v1.0.0 && git push --tags` ← after merge
- [ ] L4.8 GitHub Actions release pipeline fires → .deb, .rpm published ← auto on tag

**Risk**: LOW | **Reward**: MAXIMUM (v1.0.0 shipped)

---

## AWAITING MERGE → TAG → RELEASE

```
PR #35 CI ──► MERGE ──► git tag v1.0.0 ──► git push --tags ──► RELEASE PIPELINE
```

After merge completes, from main:
```bash
git tag v1.0.0
git push --tags
```

---

## DEFERRED (Post v1.0.0)
- reqwest 0.11 → 0.12 upgrade (HIGH RISK, medium reward)
- wallpaper-downloader thin-CLI fold (PhaseContext decoupling required first)
- scripts/*.rs → `xtask` crate (LOW RISK, medium reward, quality-of-life)
- cargo-release integration (replaces auto_bump.rs)
- Dependabot configuration for automated dep updates
- Multi-distro parallel CI test matrix
- BBS message bank expansion (44 → 60+)
- mash-setup status subcommand
- mdBook documentation expansion

---

**Last Updated**: 2026-02-22
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
