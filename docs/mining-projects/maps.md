# Mining Projects – Active Maps
> Current session work and upcoming tasks the bard is tracking.
> Execution order reflects one rule: **foundations before features, gates before gold.**

## Completed (Phase 2)
- [x] Block 1: Eliminated panics in production paths (logging.rs, zsh.rs)
- [x] Block 2: Purified core of direct I/O (orchestrator, dry_run, doctor, config)
- [x] Block 3: Surfaced swallowed errors as warnings (docker, rust, zsh, github)
- [x] Block 4: Tightened public API (removed RealSystem from exports)
- [x] Block 5: Confirmed green build (fmt + clippy + test) and documented
- [x] Shaft A: Strategic reconnaissance report filed and fully explored ✓

## Session: 2026-02-20 – Phase 2 Closure (Current)

### Summary
Fully closed Phase 2: runner/registry split, InstallationReport/PhaseOutput contract, PhaseContext helper polish, Pi detection lore, and CLI wiring all locked down with the fmt/clippy/test trilogy.

### Deliverables
- [x] Split `installer-core/lib.rs` exports into dedicated `runner` and `registry` wrappers (R-02).
- [x] Hardened `PhaseContext` helpers and documented them in `docs/modules.md` (R-01).
- [x] Added Pi helpers (`is_pi`, `pi_generation`, `supports_usb3`) and recorded the lore in `docs/ARCH.md`/`docs/HISTORY.md` (R-07).
- [x] Confirmed `InstallationReport`/`PhaseOutput` flows across CLI and tests while keeping `docs/mining-projects` docs intact (R-03/R-04/R-06).
- [x] Ran `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` from `/work/Mash-installer` with green results.

### Notes
- Phase 3 (Pi 4B HDD) remains on ice until the ledger flips to it.

---

## Execution Order

### 1. CI Lockdown (next session — 30 min)
> *No forge should produce blades without a quality gate. Lock the gate first.*

- [x] Delete `.github/workflows/rust.yml` (legacy duplicate)
- [x] Add `--all-features` to clippy and test steps in `ci.yml`
- [x] Add `cargo audit` step for dependency vulnerability scanning
- [x] Pin Rust toolchain via `rust-toolchain.toml` (deterministic builds across local + CI)
- [x] Set branch protection on `main` (require CI pass, no direct push)
- [x] Verify: PR from `work` → `main` triggers full pipeline (PR #6 — 5/5 green)

**Why first:** Every change after this gets automatic fmt/clippy/test/audit gates.
Without it, regressions sneak in unnoticed. 10 minutes of config saves hours of
debugging later.

### 2. First Tagged Release — `v0.1.0` (✓ Complete — 15 min)
> *You can't distribute what you haven't stamped.*

- [x] Verify `release.yml` triggers on `v*` tags
- [x] Build locally with `--release`
- [x] Confirm version string reflects `0.1.0` (installer-cli v0.1.0)
- [x] Tag `v0.1.0` on `main` after merging current `work`
- [x] Confirm GitHub Release appears with both binaries + SHA256
- [x] Verify checksums: `sha256sum -c mash-setup-*.sha256`
- [ ] Smoke test: download aarch64 binary on Pi, confirm it runs

**Why second:** Unlocks step 3 entirely. Also proves the release pipeline works
before we depend on it.

### 3. Retire `bootstrap.sh` (✓ Complete — 15 min)
> *The scaffolding served its purpose. Replace it with a bridge that doesn't
> require a forge on-site.*

- [x] Slim `bootstrap.sh` to ~20 lines: detect arch, download binary, verify SHA256, exec
- [x] Remove Rust/git/cargo install logic (no longer needed)
- [x] Remove font/Hyprland/makepkg logic (mash-setup handles these)
- [x] Add `uname -m` → target triple mapping (`aarch64` → `aarch64-unknown-linux-gnu`)
- [x] Test on local machine (downloads from GitHub Release)
- [x] Document the one-liner curl install as the primary method
- [x] Test on clean Pi (no Rust installed) - verified working

**Why third:** Depends on tagged releases existing. Removes the 10-minute cargo
build tax for end users. Biggest UX improvement per line of code changed.

### 4. Driver Test Harness (1-2 sessions)
> *Test the walls before you mine deeper.*

- [ ] Create test fixtures for each distro driver (Arch, Debian, Fedora)
- [ ] Mock `SystemOps` + `PhaseContext` for unit-level driver testing
- [ ] Exercise each driver's phase list against the Phase trait contract
- [ ] Verify dry-run mode produces correct `DryRunEntry` logs per driver
- [ ] Add to CI as a required check

**Why fourth:** Before adding new features (Phase 3/4), prove the existing drivers
don't regress. The hardening audit gave us clean interfaces — now test them.

### 5. Phase 3: Pi 4B HDD Tuning (2-3 sessions)
> *The primary hardware gets its dedicated optimization pass.*

- [ ] Preflight checks: USB 3.0 detection, HDD health, partition layout
- [ ] I/O scheduler tuning for external USB 3.0 HDD
- [ ] Mount options optimization (noatime, commit interval)
- [ ] Swap configuration for 8GB RAM + HDD
- [ ] Kernel parameter tuning (vm.swappiness, dirty ratio)
- [ ] All changes gated behind `PhaseContext::run_or_record()`

**Why fifth:** Feature work on the primary target hardware. Test harness (step 4)
catches regressions as we add new phases.

### 6. Phase 4: Hardening (2-3 sessions)
> *Seal the forge against the neon rain.*

- [ ] TLS certificate validation for all downloads
- [ ] Rollback rituals: snapshot before phase, restore on failure
- [ ] Lockfile: prevent concurrent installer runs
- [ ] Signal handling: graceful shutdown on SIGINT/SIGTERM
- [ ] Filesystem forensics: verify writes landed correctly

**Why sixth:** Builds on a stable, tested, CI-gated foundation. These are safety
nets — they matter most when everything else is already working.

### 7. System Packaging — AUR / .deb / .rpm (stretch goal)
> *Let the system's own courier deliver the blade.*

- [ ] AUR PKGBUILD for Arch users
- [ ] `.deb` package for Debian/Ubuntu (via `cargo-deb` or manual)
- [ ] `.rpm` package for Fedora (via `cargo-rpm` or manual)
- [ ] Add package builds to release pipeline

**Why seventh:** Gold-standard distribution, but requires stable releases and
mature feature set. Premature packaging means constant re-packaging.

### 8. TUI Rendering via Ratatui (stretch goal)
> *The forge works. Now make it glow.*

- [ ] Replace indicatif progress bars with ratatui terminal UI
- [ ] Phase-by-phase progress with live log tailing
- [ ] Interactive mode: phase selection, profile picker
- [ ] Error display with context and advice rendering

**Why last:** Polish layer. Everything underneath must be solid before investing
in presentation. A beautiful UI over a broken installer is a painted ruin.

---

## Guiding Principles
- **Gates before gold**: CI lockdown and testing infrastructure come before features
- **Stamp before ship**: Tagged releases before distribution changes
- **Test before extend**: Driver harness before new phases
- **Foundation before facade**: Core stability before TUI polish
- **Green before merge**: `work` branch only merges to `main` when fmt + clippy + test pass
