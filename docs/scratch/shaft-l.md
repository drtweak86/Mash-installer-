# ⚒️ SHAFT L: FINAL RELEASE CLEARANCE
> *"The gate opens only after every lock is checked."* — Bard 🍺

**Objective**: Comprehensive release readiness audit. Inventory what is done, identify gaps,
rank all improvement targets, chart the cleanest path to v1.0.0, and update the maps.

**Branch**: follows Shaft K completion
**Risk Level**: PLANNING ONLY in Phase 1-7; execution risk varies per item
**Reward Level**: HIGH (v1.0.0 release quality)
**Status**: PENDING (STARTS AFTER SHAFT K)

---

## ═══ SECTION A: COMPLETED TASKS INVENTORY ═══

### Core Architecture
- [x] `installer-core` library crate — all phase logic, system ops, orchestration
- [x] `installer-cli` binary crate — TUI + `--no-tui` fallback
- [x] `installer-arch`, `installer-debian`, `installer-fedora` — distro driver crates
- [x] `PhaseRegistry` / `PhaseRunner` — metadata-driven phase dispatch
- [x] `PhaseContext::run_or_record()` — dry-run-safe single gate for all side effects
- [x] `InstallContext` — central threaded context for all phases
- [x] `SystemOps` trait + `RealSystem` — testable abstraction over OS calls
- [x] `DryRunLog` — audit trail for all side effects in dry-run mode
- [x] `InstallationReport` / `PhaseOutput` — install result contract
- [x] `PlatformContext` / `detect_platform()` — Pi detection (is_pi, pi_generation, is_pi_4b, supports_usb3)

### TUI Layer (installer-cli/src/tui/)
- [x] 8-screen state machine (Welcome→DistroSelect→ModuleSelect→ProfileSelect→Confirm→Installing→Done/Error)
- [x] 4-pane cyberpunk cockpit layout (65% main / 35% log+stats / BBS strip)
- [x] `TuiMessage` bus — mpsc-based inter-thread communication
- [x] `RatatuiPhaseObserver` — live phase events → TUI updates
- [x] BBS message cycler (44 messages, 4-second cycle)
- [x] sysinfo poller (CPU/RAM/NET/IO, 1-second poll)
- [x] `--no-tui` fallback to indicatif stdio
- [x] Cyberpunk theme palette (CYAN/MAGENTA/GREEN/RED/YELLOW)

### Phase Implementations
- [x] fonts phase — font installation
- [x] system_packages phase — base packages
- [x] software_tiers phase — curated S/A-tier software
- [x] rust_toolchain phase — Rust + cargo tools
- [x] git_cli phase — Git, GitHub CLI, SSH
- [x] buildroot_dependencies phase — Buildroot deps
- [x] docker_engine phase — Docker Engine
- [x] shell_ux phase — zsh, Starship, eza aliases
- [x] rclone phase — rclone installation
- [x] snapshots phase — filesystem snapshots
- [x] ai_spirits phase — AI agent installation
- [x] pi4b_hdd_tuning phase — HDD optimization (mount opts, swap, kernel params, I/O scheduler)
- [x] argon_one phase — Argon One fan script
- [x] wallpapers phase (Shaft J) — async multi-API wallpaper downloader

### Hardening (Phase 4 completions)
- [x] Lockfile (prevents concurrent installations)
- [x] TLS enforcement in network ops
- [x] Signal handling (SIGTERM, SIGINT graceful shutdown)
- [x] Rollback manager — state snapshot + revert capability
- [x] Binary verification (SHA256)

### Pi 4B Tuning (Phase 3 completions)
- [x] Mount options optimization (relatime, noatime, discard)
- [x] Swap configuration tuning
- [x] Kernel parameter tuning (/proc/sys/*)
- [x] I/O scheduler optimization (mq-deadline for USB HDD)
- [x] USB 3.0 controller detection

### Packaging & Release
- [x] `.deb` package (cargo-deb, package.metadata.deb configured)
- [x] `.rpm` package (cargo-generate-rpm, package.metadata.generate-rpm configured)
- [x] `PKGBUILD` for Arch (pkg/aur/PKGBUILD)
- [x] GitHub Actions CI (ci.yml — fmt, clippy, test, audit, coverage, docker, nightly, docs, shellcheck)
- [x] GitHub Actions release (release.yml — cross-compile, .deb, .rpm, PKGBUILD)
- [x] Bootstrap install.sh (POSIX-compliant one-liner)

### Scripts (100% Rust)
- [x] auto_bump.rs — version bumper
- [x] branch-prune.rs — branch hygiene
- [x] check_docs.rs — doc link validator
- [x] document_hygiene.rs — doc folder management
- [x] release_checklist.rs — pre-release gate
- [x] test_infrastructure.rs — test runner (maelstrom/hardware modes)
- [x] test_theme.rs — theme integration checker

### Quality Assurance
- [x] Code coverage > 80% (Tarpaulin + Codecov)
- [x] Docker image build (drtweak86/mash-installer:latest)
- [x] Integration tests (dry-run in Ubuntu container)
- [x] Nightly Rust checks (scheduled)
- [x] Documentation build (mdBook)

---

## ═══ SECTION B: GAP ANALYSIS ═══
*Must Do | Could Do | Cosmetic | Minor Improvement*

### 🔴 MUST DO (Blocking v1.0.0)
| # | Item | Where | Why |
|---|------|-------|-----|
| B1 | Commit baseline + open PR | Branch work-shaftj-phase1 | Nothing in scope without a clean working tree |
| B2 | Resolve `wallpaper-downloader` version (0.1.0 vs 0.2.3) | wallpaper-downloader/Cargo.toml | Version mismatch in workspace |
| B3 | Consolidate wallpaper-downloader as thin CLI over installer-core | wallpaper-downloader/ | Two implementations of the same feature |
| B4 | Delete registry.rs + runner.rs shims | installer-core/src/ | Dead noise in the module tree |
| B5 | Remove redundant rust.yml | .github/workflows/ | Duplicates ci.yml completely |
| B6 | Remove legacy Python files from resources/ and docs/incoming-files/ | resources/themes/retro-bbc/ + docs/incoming-files/ | These have been superseded by Rust |
| B7 | Align dep versions (indicatif 0.17/0.18, which 4/7, once_cell 1.18/1.19) | All Cargo.toml files | Version drift causes confusion |
| B8 | Final build trinity green on main branch | CI | Release gate |

### 🟡 COULD DO (Strongly recommended before v1.0.0)
| # | Item | Where | Why |
|---|------|-------|-----|
| C1 | Upgrade toolchain 1.93.1 → 1.85.0 (latest stable) | rust-toolchain.toml | Security patches, newer stdlib features |
| C2 | Replace `once_cell` with `std::sync::OnceLock` (stable since 1.70) | installer-core deps | Remove dependency |
| C3 | reqwest 0.11 → 0.12 evaluation | installer-core/Cargo.toml | reqwest 0.12 is the current version |
| C4 | Doctor mode output for wallpaper API key status | installer-core/src/doctor.rs | Users need to know if API keys are missing |
| C5 | MASH_WALLHAVEN_KEY, MASH_PEXELS_KEY, MASH_PIXABAY_KEY env var support | wallpaper/config.rs | Better UX for API key configuration |
| C6 | `catalog` subcommand completeness audit | installer-cli/src/catalog.rs | Verify JSON output matches actual phases |
| C7 | `--dry-run` output validation against real run | installer-core/src/dry_run.rs | Ensure dry-run is truly representative |

### 🔵 COSMETIC (Nice, but not blocking)
| # | Item | Where | Why |
|---|------|-------|-----|
| D1 | BBS message bank expansion (44 → 60+ messages) | installer-cli/src/tui/bbs.rs | Richer tavern atmosphere |
| D2 | TUI progress bar for wallpaper download per-file | installer-core/src/wallpaper/download.rs | Better visual feedback |
| D3 | Install complete screen with summary stats | installer-cli/src/tui/render.rs | Satisfying end-of-install feedback |
| D4 | Docs refresh: MANUAL.md updated to reflect current features | docs/MANUAL.md | User-facing docs |
| D5 | HISTORY.md bardic entry for Shaft J | docs/HISTORY.md | ABD compliance |

### 🟢 MINOR IMPROVEMENT (Future shafts)
| # | Item | Where | Why |
|---|------|-------|-----|
| E1 | `cargo-release` integration for version bumping | Replace auto_bump.rs | Industry standard tool |
| E2 | Theme preview in TUI (show i3-gaps screenshot) | installer-cli/src/tui/ | Better UX |
| E3 | Multi-distro parallel test matrix in CI | .github/workflows/ci.yml | Broader coverage |
| E4 | Dependabot configuration for automated dep updates | .github/ | Maintenance automation |
| E5 | mdBook documentation expansion | docs/ | Better user onboarding |
| E6 | `mash-setup status` subcommand (show install state) | installer-cli/src/main.rs | Useful for support |

---

## ═══ SECTION C: IMPROVEMENT TARGETS ═══
*What? Why? Where? How?*

### C-1: Wallpaper API Key UX
- **What**: Surface clear guidance when API keys are missing
- **Why**: Currently the phase silently skips wallpapers if no keys. Users don't know WHY.
- **Where**: `installer-core/src/wallpaper/config.rs` + `installer-core/src/doctor.rs`
- **How**:
  1. Add env var support: `MASH_WALLHAVEN_KEY`, `MASH_PEXELS_KEY`, `MASH_PIXABAY_KEY`
  2. In `WallpaperConfig::default()`, check env vars before returning None
  3. In `doctor.rs`, add wallpaper API key check section showing present/missing status
  4. In phase `install_phase()`, emit a clear warning with setup URLs when no keys found

### C-2: OnceLock Migration
- **What**: Replace `once_cell = "1.19"` dependency with `std::sync::OnceLock`
- **Why**: `OnceLock` was stabilised in Rust 1.70. Removes one dep.
- **Where**: installer-core/Cargo.toml + any `once_cell::sync::OnceCell` usage in source
- **How**:
  1. `grep -r "once_cell" installer-core/src/`
  2. Replace `use once_cell::sync::OnceCell` with `use std::sync::OnceLock`
  3. API is nearly identical (`set()` / `get()` / `get_or_init()`)
  4. Remove `once_cell` from Cargo.toml
  5. Build check

### C-3: reqwest 0.11 → 0.12
- **What**: Upgrade reqwest from 0.11 to 0.12
- **Why**: reqwest 0.12 uses http 1.0, rustls default, improved async body API
- **Where**: installer-core/Cargo.toml, wallpaper-downloader/Cargo.toml
- **How**: HIGH RISK — breaking changes in response body API
  1. Read reqwest 0.12 migration guide
  2. Update Cargo.toml
  3. Fix all compilation errors (body streaming API changed)
  4. Full test suite
  5. DEFER to post-v1.0 unless critical

### C-4: Doctor Mode — Wallpaper API Check
- **What**: Add wallpaper API key check to `mash-setup doctor`
- **Why**: Operators need visibility into configuration completeness
- **Where**: `installer-core/src/doctor.rs`
- **How**:
  1. Add `check_wallpaper_keys()` function
  2. Check env vars MASH_WALLHAVEN_KEY, MASH_PEXELS_KEY, MASH_PIXABAY_KEY
  3. Emit PASS/WARN per key with setup URLs for missing keys
  4. Include in `run_doctor()` output

---

## ═══ SECTION D: REFACTORING TARGETS ═══

### D-1: phase_runner.rs (747 lines) — Complexity Watch
- **What**: 747 lines in a single file
- **Risk**: MEDIUM (central to everything)
- **Reward**: MEDIUM (readability)
- **Action**: Read carefully — if it has natural split points (Phase trait, PhaseRunner struct,
  observer protocol, error types), split into phase_runner/{mod,runner,observer,error}.rs
- **Gate**: Only refactor if a clear, clean split exists. Do NOT force it.
- **Decision deferred** until Shaft K Phase 3 is read in full

### D-2: installer-cli/src/software_tiers.rs — Correct Location?
- **What**: 102-line CLI interaction layer for software tier selection
- **Concern**: The CLI has its own software_tiers.rs AND the core has one (527 lines)
- **These are NOT duplicates** — cli version is UI interaction, core version is data/logic
- **Action**: Add a doc comment at the top of each clarifying the boundary
  - cli/software_tiers.rs: "UI layer — menus and user prompts for software tier selection"
  - core/software_tiers.rs: "Data layer — SoftwareTierPlan, ThemePlan, category definitions"
- **Risk**: LOW | **Reward**: LOW (clarity only)

### D-3: ai_agents.rs — Audit
- **What**: Module named `ai_agents.rs` in installer-core
- **Why**: Unclear what this does at a glance
- **Action**: Read the file, determine if:
  - It installs actual AI tools (ollama, etc.) → rename to tools_ai.rs for clarity
  - It's placeholder → remove or stub clearly
- **Gate**: Must be read before deciding

### D-4: `verify.rs` — Dead Code Warning
- **What**: `#[allow(dead_code)]` annotation on the entire module in lib.rs
- **Why**: Dead code suppress = something isn't being used
- **Action**: Read verify.rs, determine:
  - If used: remove #[allow(dead_code)]
  - If not used: either connect it to the install flow or delete it
- **Risk**: LOW | **Reward**: MEDIUM (eliminates allow dead_code)

---

## ═══ SECTION E: FOLD INTO MAIN RUST FLOW ═══
*Items that should become dependencies or modules of the main Rust file and flow*

### E-1: wallpaper-downloader crate → fold as thin CLI
- **Current**: Standalone crate with duplicate implementation
- **Target**: wallpaper-downloader/src/main.rs becomes a thin `clap` CLI that calls
  `installer_core::wallpaper::*` public API
- **Steps**:
  1. Add `installer-core` to wallpaper-downloader/Cargo.toml
  2. Remove duplicate api.rs, config.rs, download.rs, error.rs, types.rs from standalone crate
  3. Replace with ~50-line main.rs: parse args, build WallpaperConfig, call download_wallpapers()
  4. wallpaper-downloader becomes purely a CLI wrapper
- **Benefit**: Single source of truth for wallpaper logic

### E-2: scripts/*.rs → xtask crate (Future)
- **Current**: Standalone .rs files run via rustc/rust-script
- **Target**: `xtask` crate in workspace (`cargo xtask bump`, `cargo xtask hygiene`, etc.)
- **When**: Post v1.0.0 — this is a quality-of-life improvement, not blocking
- **Steps** (when ready):
  1. Create `xtask/` directory, add to workspace members
  2. Move scripts/*.rs into xtask/src/commands/
  3. Use clap to dispatch subcommands
  4. cargo xtask <command> replaces rustc scripts/foo.rs

### E-3: resources/shell/eza_aliases.sh → embed as const OR keep as resource
- **Current**: Shell file deployed to user's machine by zsh.rs phase
- **Option**: Embed as `const EZA_ALIASES: &str = include_str!("../resources/shell/eza_aliases.sh")`
  in zsh.rs, then write to disk during install
- **Benefit**: Reduces number of .sh files, consolidates into Rust binary
- **Risk**: LOW | **Reward**: LOW (it's data, not a script)
- **Decision**: COULD DO — `include_str!()` is the clean Rust way to embed resource files
- **Steps**:
  1. In `installer-core/src/zsh.rs`, add `const EZA_ALIASES: &str = include_str!("../../resources/shell/eza_aliases.sh");`
  2. Write EZA_ALIASES to `~/.eza_aliases` during the shell_ux phase
  3. The .sh file remains as the source of truth (edited by humans), compiled in by Rust

---

## ═══ SECTION F: RISK / REWARD MATRIX ═══

| Item | Risk | Reward | Verdict |
|------|------|--------|---------|
| Commit baseline | NONE | HIGH | DO NOW |
| Delete legacy Python files | NONE | LOW | DO NOW |
| Delete registry.rs + runner.rs shims | LOW | MEDIUM | DO - Shaft K |
| Delete rust.yml CI | NONE | LOW | DO - Shaft K |
| Align dep versions | LOW | MEDIUM | DO - Shaft K |
| Fold wallpaper-downloader | MEDIUM | HIGH | DO - Shaft K |
| Toolchain 1.93.1 → 1.85.0 | LOW | MEDIUM | DO - Shaft K |
| once_cell → OnceLock | LOW | MEDIUM | DO - Shaft K |
| Wallpaper API key UX | LOW | HIGH | DO - Shaft L |
| Doctor mode wallpaper check | LOW | HIGH | DO - Shaft L |
| include_str! eza_aliases | LOW | LOW | COULD DO - Shaft L |
| phase_runner.rs split | MEDIUM | MEDIUM | EVALUATE |
| verify.rs dead code | LOW | MEDIUM | DO - Shaft L |
| ai_agents.rs audit | LOW | MEDIUM | DO - Shaft L |
| reqwest 0.11 → 0.12 | HIGH | MEDIUM | DEFER post-v1.0 |
| xtask crate | LOW | MEDIUM | DEFER post-v1.0 |
| MANUAL.md refresh | LOW | MEDIUM | DO before release |
| HISTORY.md bardic entry | NONE | LOW | DO - ABD compliance |
| BBS message expansion | NONE | LOW | Cosmetic |
| cargo-release integration | LOW | MEDIUM | Post-v1.0 |

---

## ═══ SECTION G: LOGICAL PATH TO COMPLETION ═══

```
work-shaftj-phase1 (current)
│
├─ COMMIT BASELINE (Phase 1, Shaft K)
│   └─ All Shaft J + script work committed, PR opened
│
├─ SHAFT K: FORGE HARDENING
│   ├─ Phase 2: Legacy artifact purge (Python files, duplicate sh)
│   ├─ Phase 3: Shim elimination (registry.rs, runner.rs)
│   ├─ Phase 4: Crate consolidation (wallpaper-downloader, rust.yml)
│   ├─ Phase 5: Dependency hygiene (versions, toolchain)
│   └─ MERGE → main (CI green required)
│
├─ SHAFT L: FINAL RELEASE CLEARANCE
│   ├─ Phase 1: Code quality (verify.rs, ai_agents audit, dead code)
│   ├─ Phase 2: UX improvements (API key guidance, doctor mode)
│   ├─ Phase 3: Doc updates (MANUAL.md, HISTORY.md)
│   ├─ Phase 4: Final release gate (release_checklist.rs, cargo audit)
│   └─ MERGE → main
│
└─ VERSION BUMP TO v1.0.0
    ├─ scripts/auto_bump.rs minor (0.2.3 → 1.0.0)  [major bump]
    ├─ HISTORY.md bardic release entry
    ├─ git tag v1.0.0 && git push --tags
    └─ GitHub Actions release pipeline fires → .deb, .rpm, PKGBUILD
```

**Total estimated work**:
- Shaft K: ~6-8 focused sessions
- Shaft L: ~4-6 focused sessions
- Version release: ~1 session

**Critical path**: Shaft K → Shaft L → v1.0.0

---

## PHASE 1: CODE QUALITY PASS — Checkpoint α

### Step 1: verify.rs audit
- 1.1 Read installer-core/src/verify.rs in full
- 1.2 Find all callers: `grep -r "verify::" installer-core/src/ installer-cli/src/`
- 1.3.a If called: remove `#[allow(dead_code)]` from lib.rs declaration
- 1.3.b If unused: connect to install flow OR delete
- 1.4 Build check

### Step 2: ai_agents.rs audit
- 2.1 Read installer-core/src/ai_agents.rs in full
- 2.2 Confirm what it installs (AI tools: ollama, aider, etc.)
- 2.3 If clear purpose: add doc comment at top explaining what it installs
- 2.4 If placeholder: mark clearly with TODO or remove
- 2.5 Ensure it is properly gated (PhaseGate::Always may be wrong for optional AI tools)

### Step 3: OnceLock migration
- 3.1 `grep -r "once_cell" installer-core/src/`
- 3.2 Replace OnceCell/Lazy with OnceLock/LazyLock (std equivalents)
- 3.3 Remove `once_cell` from installer-core/Cargo.toml
- 3.4 Build check

### ✅ MILESTONE α — Code quality clean

---

## PHASE 2: UX IMPROVEMENTS — Checkpoint β

### Step 4: Wallpaper API key env var support
- 4.1 Edit installer-core/src/wallpaper/config.rs
- 4.2 In WallpaperConfig::default() or a new from_env() constructor:
  ```rust
  api_keys: ApiKeys {
      wallhaven: std::env::var("MASH_WALLHAVEN_KEY").ok(),
      pexels: std::env::var("MASH_PEXELS_KEY").ok(),
      pixabay: std::env::var("MASH_PIXABAY_KEY").ok(),
  }
  ```
- 4.3 Update phases/wallpapers.rs install_phase() to emit actionable URLs when no keys found

### Step 5: Doctor mode — wallpaper API check
- 5.1 Edit installer-core/src/doctor.rs
- 5.2 Add check_wallpaper_keys() function that inspects env vars
- 5.3 Emit PASS/WARN per API source with setup URLs
- 5.4 Include in run_doctor() output

### Step 6: eza_aliases.sh include_str! embedding
- 6.1 Edit installer-core/src/zsh.rs
- 6.2 Add: `const EZA_ALIASES: &str = include_str!("../../resources/shell/eza_aliases.sh");`
- 6.3 Ensure install phase writes EZA_ALIASES to `~/.eza_aliases`
- 6.4 Build check — `include_str!` is compile-time, will fail if path wrong

### ✅ MILESTONE β — UX improvements done

---

## PHASE 3: DOCUMENTATION — Checkpoint γ

### Step 7: HISTORY.md bardic entry (Shaft J)
- 7.1 Add Shaft J entry to docs/HISTORY.md
- 7.2 Bardic style: wallpaper downloader Rust conversion tale

### Step 8: MANUAL.md refresh
- 8.1 Read docs/MANUAL.md in full
- 8.2 Update to reflect:
  - Current feature set (wallpapers, Pi 4B tuning, TUI, etc.)
  - API key setup instructions
  - `--no-tui` flag documentation
  - `doctor` subcommand documentation
  - `catalog` subcommand documentation

### Step 9: doc link validation
- 9.1 Run `rustc --edition 2021 scripts/check_docs.rs -o /tmp/check_docs && /tmp/check_docs`
- 9.2 Fix any broken links found

### ✅ MILESTONE γ — Documentation current

---

## PHASE 4: RELEASE GATE — Checkpoint δ

### Step 10: Final release gate
- 10.1 Run release_checklist.rs:
  ```bash
  rustc --edition 2021 scripts/release_checklist.rs -o /tmp/release_checklist
  /tmp/release_checklist
  ```
- 10.2 `cargo audit` — zero vulnerabilities
- 10.3 `shellcheck install.sh` — clean
- 10.4 Version consistency check: all Cargo.toml files at same version
- 10.5 Confirm CI is green on main branch
- 10.6 Commit Shaft L changes
- 10.7 Open PR → wait for CI green → merge

### ✅ MILESTONE δ — Shaft L COMPLETE, v1.0.0 ready

---

**Last Updated**: 2026-02-22
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
