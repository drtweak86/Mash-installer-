# ⚒️ SHAFT K: FORGE HARDENING
> *"The blade is only as strong as the steel it's forged from."* — Bard 🍺

**Objective**: Clean the forge floor — commit the baseline, eliminate dead weight, fold thin
shims, consolidate duplicate crates, audit remaining non-Rust artifacts, and leave the
codebase lean and ready for final release.

**Branch**: `work-shaftj-phase1` → new branch `work-shaftk-hardening`
**Risk Level**: LOW (no logic changes — structural only)
**Reward Level**: HIGH (cleaner build, less confusion, faster CI, smaller binary surface)
**Status**: PENDING

---

## PHASE 1: BASELINE COMMIT — Checkpoint α
*"Stamp what exists before changing anything."*

### Step 1: Commit current Shaft J + scripts work
- 1.1 Stage all modified tracked files
  - installer-core/src/{doctor,lib,phase_registry,system}.rs
  - wallpaper-downloader/src/*.rs
  - docs/forge-tavern/{bard-bbs-profile,bard-quick-ref,maps,maps-explored}.md
  - .gitignore
- 1.2 Stage all new untracked files
  - installer-core/src/phases/
  - installer-core/src/wallpaper/
  - scripts/{branch-prune,auto_bump,check_docs,document_hygiene,release_checklist,test_infrastructure,test_theme}.rs
- 1.3 Stage deletions
  - scripts/{auto_bump.py,check-docs.py,release-checklist.sh,test-infrastructure.sh,test-theme-manual.sh,rustify.rs}
  - docs/forge-tavern/{COMPLETION_ORDER_ANALYSIS,FINAL_SUMMARY,REPO_STATUS,shaftj}.md
- 1.4 Run build trinity before commit
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --workspace`
- 1.5 Commit with message: `refactor: rustify scripts/, clean forge-tavern, complete Shaft J`

### ✅ MILESTONE α — Baseline committed, working tree clean

---

## PHASE 2: LEGACY ARTIFACT PURGE — Checkpoint β
*"Dead weight sinks the ship."*

### Step 2: Remove legacy Python files
- 2.1 Delete `resources/themes/retro-bbc/wallpaper_downloader_final.py`
  - Reason: Superseded by Shaft J's Rust implementation in installer-core/src/wallpaper/
  - Risk: NONE — not compiled, not referenced by Rust code
- 2.2 Delete `docs/incoming-files/wallpaper_downloader_final.py`
  - Reason: Staging artifact from before Shaft J, no longer needed
  - Risk: NONE
- 2.3 Delete `docs/incoming-files/eza-aliases.sh`
  - Reason: Staging copy — the canonical version lives in resources/shell/eza_aliases.sh
  - Risk: NONE — the resources/ copy is what matters

### Step 3: Evaluate resources/shell/eza_aliases.sh
- 3.1 Read current content and confirm it matches what zsh.rs deploys
- 3.2 Decision: KEEP as resource file (data, not build tool)
  - Justification: Function > Form — shell alias files are config/data, not scripts
  - Editing them in plain text is cleaner than modifying Rust const strings and recompiling
  - POSIX sh syntax is inert; it is not executed during build
  - This is the `install.sh` equivalence principle: the file is data that runs on the
    user's machine, not in our build pipeline
- 3.3 No action required — document the decision in this shaft

### Step 4: Verify install.sh status
- 4.1 Confirm install.sh is the lone legitimate .sh file in the project root
- 4.2 Verify it is POSIX-compliant and shellcheck-clean
  - `shellcheck install.sh`
- 4.3 No changes expected

### ✅ MILESTONE β — All legacy non-Rust artifacts purged

---

## PHASE 3: THIN SHIM ELIMINATION — Checkpoint γ
*"A re-export of a re-export is noise."*

### Step 5: Delete registry.rs (1-line shim)
- 5.1 Read installer-core/src/registry.rs
  - Content: `pub use crate::phase_registry::PhaseRegistry;`
  - 1 line. A thin re-export layer with no added value.
- 5.2 Check all uses of `crate::registry` across the codebase
  - `grep -r "crate::registry\|use.*registry::" installer-core/src/`
- 5.3 Update lib.rs: replace `pub use registry::PhaseRegistry;` with
  `pub use phase_registry::PhaseRegistry;`
- 5.4 Delete installer-core/src/registry.rs
- 5.5 Remove `mod registry;` from lib.rs
- 5.6 Build check: `cargo build --workspace`

### Step 6: Delete runner.rs (4-line shim)
- 6.1 Read installer-core/src/runner.rs
  - Content: re-exports from phase_runner (4 lines)
- 6.2 Check all uses of `crate::runner` across the codebase
  - `grep -r "crate::runner\|use.*runner::" installer-core/src/`
- 6.3 Update lib.rs: re-export directly from phase_runner
- 6.4 Delete installer-core/src/runner.rs
- 6.5 Remove `mod runner;` from lib.rs
- 6.6 Build check: `cargo build --workspace`

### ✅ MILESTONE γ — Thin shims eliminated, lib.rs cleaner

---

## PHASE 4: CRATE CONSOLIDATION AUDIT — Checkpoint δ
*"Two anvils for the same job is one anvil too many."*

### Step 7: Audit wallpaper-downloader standalone crate
- 7.1 Compare wallpaper-downloader/ vs installer-core/src/wallpaper/
  - Diff: deps (log/env_logger vs tracing), binary vs library, standalone vs phase
  - wallpaper-downloader/ has: reqwest 0.11, tokio, serde, clap, log, env_logger, indicatif,
    sha2, tokio-util, dirs, once_cell, mockito
  - installer-core/wallpaper/ uses: reqwest 0.11, tokio, serde, anyhow, thiserror (shared)
- 7.2 Decision matrix:
  - Option A: KEEP as standalone utility binary (users can run standalone)
    - Pro: useful as standalone wallpaper tool; separate concerns
    - Con: two implementations to maintain; dep duplication (log vs tracing)
  - Option B: FOLD into installer-core fully, remove standalone crate
    - Pro: single implementation; no dep duplication; simpler workspace
    - Con: users lose standalone binary
  - Option C: KEEP standalone but have it delegate to installer-core/wallpaper/ as a lib
    - Pro: single implementation, standalone binary preserved
    - Con: circular dependency risk (wallpaper-downloader depends on installer-core)
  - RECOMMENDATION: Option C — make wallpaper-downloader a thin CLI wrapper around
    installer-core/src/wallpaper/ public API. This is LOW RISK HIGH REWARD.
- 7.3 If Option C chosen:
  - 7.3.1 Add `installer-core` as dependency to wallpaper-downloader/Cargo.toml
  - 7.3.2 Replace wallpaper-downloader/src/*.rs with thin CLI wrappers
  - 7.3.3 Remove duplicate api.rs, config.rs, download.rs, error.rs, types.rs from wallpaper-downloader/src/
  - 7.3.4 wallpaper-downloader/src/main.rs becomes a ~50-line CLI entry point
  - 7.3.5 wallpaper-downloader/src/lib.rs re-exports from installer-core
- 7.4 Align versions: wallpaper-downloader is 0.1.0, rest of workspace is 0.2.3
  - Bump wallpaper-downloader to 0.2.3 for consistency

### Step 8: CI workflow deduplication
- 8.1 Read .github/workflows/rust.yml (23 lines)
  - Contains: basic build + test (subset of ci.yml)
- 8.2 Read .github/workflows/ci.yml (204 lines)
  - Contains: everything rust.yml does + much more
- 8.3 Decision: DELETE rust.yml — it is fully subsumed by ci.yml
  - Risk: NONE — ci.yml runs on every push/PR already
- 8.4 Delete .github/workflows/rust.yml
- 8.5 Verify ci.yml triggers cover all cases rust.yml covered

### ✅ MILESTONE δ — Crate consolidation and CI dedup done

---

## PHASE 5: DEPENDENCY HYGIENE — Checkpoint ε
*"Every dep is a promise to maintain."*

### Step 9: Dependency audit
- 9.1 Run `cargo audit` — check for known vulnerabilities
- 9.2 reqwest version: pinned at 0.11 throughout
  - reqwest 0.12 has breaking changes (http 1.0 upgrade, rustls by default)
  - Decision: DEFER upgrade to a dedicated shaft (HIGH EFFORT, MEDIUM RISK)
  - Note in maps for future shaft
- 9.3 `which` crate: installer-cli uses version 4, should check installer-core uses 7
  - Align to the same version
- 9.4 `indicatif`: installer-cli uses 0.18, installer-core uses 0.17
  - Align to 0.18 across the workspace
- 9.5 `once_cell`: installer-core uses 1.19, wallpaper-downloader uses 1.18
  - Align to 1.19 (or consider std::sync::OnceLock — stabilized in Rust 1.70)
- 9.6 Run `cargo update` after alignment
- 9.7 Run `cargo test --workspace` to confirm green

### Step 10: Toolchain assessment
- 10.1 Current: Rust 1.93.1 (pinned in rust-toolchain.toml)
- 10.2 Rust 1.93.1 is stable — evaluate upgrade to 1.85.0 (latest stable as of 2026-02)
  - Key improvements available: std::sync::LazyLock (1.80), OnceLock stable (1.70)
  - RECOMMENDATION: Upgrade to 1.85.0 in this shaft — low risk given CI validates
  - 10.2.1 Edit rust-toolchain.toml channel to "1.85.0"
  - 10.2.2 Run full build + clippy + test
  - 10.2.3 If green → commit; if red → investigate specific failures
- 10.3 Consider pinning to `stable` channel instead of specific version
  - Pro: always gets security patches automatically
  - Con: build reproducibility reduced
  - Decision: DEFER — keep pinned version for now, note for v1.0 shaft

### ✅ MILESTONE ε — Dependencies aligned, toolchain evaluated

---

## PHASE 6: FINAL SHAFT K COMMIT — Checkpoint ζ
*"Stamp the steel, ship the blade."*

- 6.1 Run final build trinity
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --workspace`
  - `cargo audit`
- 6.2 `shellcheck install.sh` — confirm clean
- 6.3 Commit all Shaft K changes
  - Separate commits per phase for clean history:
    - `refactor: remove legacy Python and duplicate shell artifacts`
    - `refactor: eliminate registry.rs and runner.rs thin shims`
    - `refactor: consolidate wallpaper-downloader as thin CLI over installer-core`
    - `chore: remove redundant rust.yml CI workflow`
    - `chore: align dep versions across workspace`
- 6.4 Open PR from `work-shaftk-hardening` to `main`
- 6.5 Wait for CI green
- 6.6 Merge

### ✅ MILESTONE ζ — Shaft K COMPLETE

---

## RISK / REWARD SUMMARY

| Task | Risk | Reward | Priority |
|------|------|--------|----------|
| Commit baseline | NONE | HIGH (clean state) | P0 |
| Delete legacy Python files | NONE | LOW | P1 |
| Keep eza_aliases.sh as resource | NONE | NONE (correct as-is) | P0 |
| Delete registry.rs shim | LOW | MEDIUM (cleaner) | P1 |
| Delete runner.rs shim | LOW | MEDIUM (cleaner) | P1 |
| Fold wallpaper-downloader | MEDIUM | HIGH (single impl) | P1 |
| Delete rust.yml | NONE | LOW (CI simplicity) | P1 |
| Align dep versions | LOW | MEDIUM (consistency) | P2 |
| Toolchain upgrade 1.85.0 | LOW | MEDIUM (security) | P2 |
| reqwest 0.12 upgrade | HIGH | MEDIUM | DEFER |

---

**Last Updated**: 2026-02-22
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
