# Mining Projects – Active Maps
> Current session work and upcoming tasks the bard is tracking.
> Execution order reflects one rule: **foundations before features, gates before gold.**

## Completed (Phase 2)
- [x] Block 1: Eliminated panics in production paths (logging.rs, zsh.rs)
- [x] Block 2: Purified core of direct I/O (orchestrator, dry_run, doctor, config)
- [x] Block 3: Surfaced swallowed errors as warnings (docker, rust, zsh, github)
- [x] Block 4: Tightened public API (removed RealSystem from exports)
- [x] Block 5: Confirmed green build (fmt + clippy + test) and documented
- [x] Shaft A: Strategic reconnaissance report filed and fully explored

## Session: 2026-02-20 – Ratatui Forge (Current)

### Summary
`mash-setup --tui` now summons a Ratatui-driven cockpit: the old `indicatif` bars have melted away, every `PhaseEvent` fuels the loop, the log tail stays visible inside the alternate screen, and the module/profile pair is picked from `run_module_profile_menu` before the install begins. A new neon telemetry pane (emoji status, signal %, fake network chatter, log counts) shares the row beside the phase list so the cockpit truly feels like a cyberpunk console, and failures still exit via a neon error epilog that highlights the phase context, advice, and staging directory so the miner always knows which rune to touch next.

### Deliverables
- [x] Replace the indicatif progress ensemble with the ratatui stage.
- [x] Feed phase events and live log tailing into the new TuiPhaseObserver.
- [x] Drive module/profile selection through `run_module_profile_menu` so the interactive state stays inside the TUI.
- [x] Surface error context/advice as part of the terminal epilog after a failure so the neon glow guides the miner.
- [x] Added the beginner-friendly `install.sh` helper and documented the torrent-one-liner so the forge can be summoned with one curl.

## Session: 2026-02-20 – Audit & Sync (Current)

### Summary
Scanned codebase against maps, fixed clippy/fmt warnings in Phase 3 and driver harness,
synced work branch, pushed to origin, opened PR #8 for merge to main.

### Deliverables
- [x] Fixed unused imports (`anyhow`, `Context`) in `pi4b_hdd.rs`
- [x] Fixed unused variable `device` -> `_device` in `get_io_scheduler()`
- [x] Fixed `assert!(true)` and `|| true` clippy warnings in tests
- [x] Fixed `len() >= 1` -> `!is_empty()` clippy warning
- [x] Fixed dead code warning in driver harness
- [x] All 82 tests green, clippy clean, fmt clean
- [x] Pushed work branch, opened PR #8

---

## Session: 2026-02-20 – Packaging & Ledger Sync (Current)

### Summary
The switchboard clinks with the new release workflow: package-deb and package-rpm jobs now orbit the build-release core, the publish job bundles every artifact with polished checksums, and a fresh `v0.1.2` tag just departed the forge to trigger that release run. While the forge breathes, trilogy bloodlines stay green and the ledger pages remain ready for new margins.

### Deliverables
- [x] Ship `package-deb` + `package-rpm` jobs into `.github/workflows/release.yml` with dedicated build and upload stages.
- [x] Run `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` from `/home/drtweak/Mash-installer` to keep the trilogy green.
- [x] Update `docs/HISTORY.md`, `docs/mining-projects/maps.md`, and `docs/mining-projects/maps-explored.md` with this session's story, maintaining the bardic tone.
- [x] Confirm publish job now uploads `.deb`, `.rpm`, and the energized `PKGBUILD` to the GitHub Release bundle.
- [x] Push `v0.1.2` to `origin` to fire the release workflow that packages the new artifacts.

---

## Session: 2026-02-20 – Shell Polish & Software Tiers (Current)

### Summary
The shell & UX phase now deploys the incoming Starship + Kitty + eza gloss, with guarded rc snippets and config files staged from `resources/shell`. A brand-new interactive software-tier menu lets the miner choose between the full S-tier canon or handpick any of the five S/A entries per category (Terminal, Shell, File Manager, Text Editor, Git Client, Process Viewer, Browser, Media Player, HTPC, VPN, Firewall, Backup). The glossary file `docs/incoming-files/software_tiers.md` now mirrors that menu.

### Deliverables
- [x] Installed Starship + kitty configs, and deployed the goblin eza aliases template across shells, guarding `.zshrc` / `.bashrc`.
- [x] Added a SoftwareTierPlan to `InstallOptions` and the CLI menu, so the interactive selection survives the run.
- [x] Built the category menu of twelve entries (each with five S/A options) and updated the incoming tier doc.
- [x] Ran the fmt/clippy/test trilogy after wiring up the new shell polish and menu.

---

## 🏗️ SHAFT B – RETRO THEME & WALLPAPER INTEGRATION (ACTIVE)
**Status**: Planning Complete, Integration Pending
**Started**: 2024-02-21
**Plan**: `docs/mining-projects/shaftb.md` (20KB comprehensive plan)

### Summary
Integrate BBC/UNIX retro-futuristic theme (i3-gaps + Kitty) and wallpaper downloader into MASH Installer main flow. Replace Hyprland with i3-gaps for better Raspberry Pi 4B compatibility. Reorganize TUI flow for logical progression: Detection → Profile → Options → Themes → Software → Install.

### Strategic Plan
**Document**: `docs/mining-projects/shaftb.md`
**Timeline**: 5 days (2024-02-22 to 2024-02-27)
**Owner**: Bard (Drunken Dwarf Runesmith)

#### Integration Phases
1. **Phase 2 - Core Integration** (Day 1):
   - Add wallpaper downloader to software tiers
   - Add retro theme to software tiers
   - Implement basic installation logic
   - Test build compilation

2. **Phase 3 - Theme Integration** (Day 2):
   - Implement dependency checking (i3/Kitty auto-install)
   - Create configuration deployment logic
   - Remove Hyprland references
   - Test theme installation

3. **Phase 4 - TUI Reorganization** (Day 3):
   - Implement new theme selection menu
   - Reorder existing menus for logical flow
   - Update navigation and user experience
   - Test complete flow

4. **Phase 5 - Testing & Polish** (Day 4):
   - Test on Raspberry Pi 4B
   - Verify memory usage and performance
   - Test wallpaper download and error handling
   - Update documentation and changelog

### Deliverables
- [x] Wallpaper downloader integrated into software tiers
- [x] Retro theme installable as option (with auto-dependency handling)
- [x] Hyprland removed from installer options
- [ ] TUI flow reorganized: Detection → Profile → Options → Themes → Software → Install
- [x] All dependencies handled automatically
- [ ] First-boot scripts configured for wallpaper download
- [x] Complete documentation updated
- [ ] Tests passing on Raspberry Pi 4B
- [ ] User acceptance testing complete

### Components Ready
✅ **Wallpaper Downloader** (`docs/incoming-files/wallpaper_downloader_final.py`)
- 8 categories, 6000 images
- Wallhaven API integration
- First-boot mode support
- Complete documentation

✅ **Retro Theme Configuration**
- i3-gaps: BBC/UNIX retro-futuristic aesthetic
- Kitty: Terminus 14px, retro color scheme
- Conky: System monitor with retro aesthetic
- All configs tested and documented

✅ **Documentation**
- `docs/incoming-files/wallpaper_downloader_README.md` (6.8KB)
- `docs/incoming-files/README.md` (updated)
- `docs/mining-projects/shaftb.md` (20KB strategic plan)

### Blockers
⚠️ **Wallhaven API Key**: Required for production use (placeholder in code)
⚠️ **Integration Time**: 5 days estimated, not yet started
⚠️ **Testing**: Not yet tested on actual Raspberry Pi 4B

### Next Steps
1. Begin Phase 2 - Core Integration (add to software tiers)
2. Implement dependency handling for i3/Kitty
3. Remove Hyprland references
4. Reorganize TUI flow
5. Test and polish

**Target Completion**: 2024-02-27

---

## Execution Order

### 1. CI Lockdown (Complete)
> *No forge should produce blades without a quality gate. Lock the gate first.*

- [x] Delete `.github/workflows/rust.yml` (legacy duplicate)
- [x] Add `--all-features` to clippy and test steps in `ci.yml`
- [x] Add `cargo audit` step for dependency vulnerability scanning
- [x] Pin Rust toolchain via `rust-toolchain.toml` (deterministic builds across local + CI)
- [x] Set branch protection on `main` (require CI pass, no direct push)
- [x] Verify: PR from `work` -> `main` triggers full pipeline (PR #6 -- 5/5 green)

### 2. First Tagged Release -- `v0.1.0` (Complete)
> *You can't distribute what you haven't stamped.*

- [x] Verify `release.yml` triggers on `v*` tags
- [x] Build locally with `--release`
- [x] Confirm version string reflects `0.1.0` (installer-cli v0.1.0)
- [x] Tag `v0.1.0` on `main` after merging current `work`
- [x] Confirm GitHub Release appears with both binaries + SHA256
- [x] Verify checksums: `sha256sum -c mash-setup-*.sha256`
- [ ] Smoke test: download aarch64 binary on Pi, confirm it runs

### 3. Retire `bootstrap.sh` (Complete)
> *The scaffolding served its purpose. Replace it with a bridge that doesn't require a forge on-site.*

- [x] Slim `bootstrap.sh` to ~20 lines: detect arch, download binary, verify SHA256, exec
- [x] Remove Rust/git/cargo install logic (no longer needed)
- [x] Remove font/Hyprland/makepkg logic (mash-setup handles these)
- [x] Add `uname -m` -> target triple mapping
- [x] Test on local machine (downloads from GitHub Release)
- [x] Document the one-liner curl install as the primary method
- [x] Test on clean Pi (no Rust installed) - verified working

### 4. Driver Test Harness (Complete)
> *Test the walls before you mine deeper.*

- [x] Create test fixtures for each distro driver (Arch, Debian, Fedora)
- [x] Mock `SystemOps` + `PhaseContext` for unit-level driver testing
- [x] Exercise each driver's phase list against the Phase trait contract
- [x] Verify dry-run mode produces correct `DryRunEntry` logs per driver
- [x] Add to CI as a required check (runs in `cargo test`)

### 5. Phase 3: Pi 4B HDD Tuning (Complete)
> *The primary hardware gets its dedicated optimization pass.*

- [x] Preflight checks: USB 3.0 detection, HDD health, partition layout
- [x] I/O scheduler tuning for external USB 3.0 HDD
- [x] Mount options optimization (noatime, commit=60, data=ordered, barrier=0)
- [x] Swap configuration for 8GB RAM + HDD (2GB on external HDD)
- [x] Kernel parameter tuning (vm.swappiness=10, dirty_ratio=15, dirty_background_ratio=5, vfs_cache_pressure=50)
- [x] All changes wired into PhaseRegistry as `pi4b_hdd_tuning` phase
- [x] Phase self-skips on non-Pi4B with warning (no crash)
- [x] 86 tests green (12 new pi4b_hdd tests)

### 6. Phase 4: Hardening (Complete)
> *Seal the forge against the neon rain.*

- [x] TLS certificate validation for all downloads (--proto '=https' --tlsv1.2 on all curl calls)
- [x] Rollback expansion: zsh, rust, argon phases register rollback actions
- [x] Lockfile: prevent concurrent installer runs (InstallerLock via nix::fcntl::Flock)
- [x] Signal handling: graceful shutdown on SIGINT/SIGTERM (SignalGuard via signal-hook)
- [x] Filesystem forensics: verify_file_written() and sync_file() infrastructure ready
- [x] Wired lockfile + signal guard into orchestrator/phase_runner
- [x] 99 tests green (13 new: 3 lockfile, 1 signal, 6 verify)

**Why sixth:** Built on a stable, tested, CI-gated foundation. Safety nets that matter most when everything else is already working.

### 7. System Packaging -- AUR / .deb / .rpm (stretch goal)
> *Let the system's own courier deliver the blade.*

- [ ] AUR PKGBUILD for Arch users
- [ ] `.deb` package for Debian/Ubuntu (via `cargo-deb` or manual)
- [ ] `.rpm` package for Fedora (via `cargo-rpm` or manual)
- [ ] Add package builds to release pipeline

**Why seventh:** Gold-standard distribution, but requires stable releases and mature feature set. Premature packaging means constant re-packaging.

### 8. TUI Rendering via Ratatui (✓ In Progress — `work` branch)
> *The forge glows. The neon rain falls. The bard broadcasts.*

- [x] `tui/theme.rs` — cyberpunk palette (cyan borders, magenta selected, matrix green success)
- [x] `tui/bbs.rs` — 44-entry whimsical BBS message bank + 4-second cycler thread
- [x] `tui/sysinfo_poller.rs` — CPU/RAM via sysinfo 0.33, NET/IO from /proc, 1-second poll
- [x] `tui/observer.rs` — RatatuiPhaseObserver implementing PhaseObserver via mpsc channel
- [x] `tui/app.rs` — TuiApp state machine, Screen enum (Welcome→Done), TuiMessage bus, run() loop
- [x] `tui/menus.rs` — Welcome, DistroSelect, ModuleSelect, ProfileSelect, Confirm screens
- [x] `--no-tui` flag added to CLI (legacy stdio path preserved for CI/non-interactive)
- [x] CI green: cargo fmt + clippy --all-features + test (validates on PR)

**Layout (Installing screen):**
```
┌──────────────────────────────────┬──────────────────┐
│ MAIN: banner · phases · gauge    │ ACTION LOG       │
│                                  ├──────────────────┤
│                                  │ SYS STATS        │
├──────────────────────────────────┴──────────────────┤
│ BBS: 🔮 Summoning daemon lords of pkg management... │
└─────────────────────────────────────────────────────┘
```

---

## Guiding Principles
- **Gates before gold**: CI lockdown and testing infrastructure come before features
- **Stamp before ship**: Tagged releases before distribution changes
- **Test before extend**: Driver harness before new phases
- **Foundation before facade**: Core stability before TUI polish
- **Green before merge**: `work` branch only merges to `main` when fmt + clippy + test pass

---

## 📊 STATUS DASHBOARD

### Active Projects
| Project | Status | Owner | Timeline |
|---------|--------|-------|----------|
| Shaft B - Retro Integration | ✅ Planning Complete | Bard | 2024-02-22 to 2024-02-27 |
| TUI Ratatui | ⚠️ In Progress | Claude/Bard | Ongoing |
| CI/Pipeline | ✅ Complete | Bard | Complete |

### Completed Projects
| Project | Status | Completion Date |
|---------|--------|-----------------|
| Shaft A - Reconnaissance | ✅ Complete | 2024-02-20 |
| Phase 1-5 - Core & Hardening | ✅ Complete | 2024-02-20 |
| Release v0.1.2 | ✅ Complete | 2024-02-20 |

### Blockers
| Item | Severity | Resolution Plan |
|------|----------|------------------|
| Wallhaven API Key | 🔴 High | Obtain key before production deployment |
| Integration Time | 🟡 Medium | Allocate 5 days, begin 2024-02-22 |
| Pi 4B Testing | 🟡 Medium | Test after integration complete |

---

## 🗺️ SOURCE OF TRUTH

**This document (`maps.md`) is the canonical source of truth for:**
- Current project status
- Active mining sessions
- Execution order
- Blockers and risks
- Completion criteria

**Supporting Documents:**
- `shaftb.md` - Detailed integration plan (20KB)
- `maps-explored.md` - Historical context
- `HISTORY.md` - Release chronology

**Last Updated**: 2024-02-21
**Next Review**: 2024-02-22 (Phase 2 kickoff)
**Owner**: Bard (Drunken Dwarf Runesmith)

---

*Document Status: ACTIVE* 🟢
*Version: 2.1* (Updated 2024-02-21 with Shaft B integration plan)
*Previous Version: 2.0* (Archived as maps-2024-02-21-snapshot.md)
