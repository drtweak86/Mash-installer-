# Mining Projects – Active Maps
> Current session work and upcoming tasks the bard is tracking.
> Execution order reflects one rule: **foundations before features, gates before gold.**

## Completed (Phase 2)
- [x] Block 1: Eliminated panics in production paths (logging.rs, zsh.rs)
- [x] Block 2: Purified core of direct I/O (orchestrator, dry_run, doctor, config)
- [x] Block 3: Surfaced swallowed errors as warnings (docker, rust, zsh, github)
- [x] Block 4: Tightened public API (removed RealSystem from exports)
- [x] Block 5: Confirmed green build (fmt + clippy + test) and documented
- [x] Shaft A: Strategic reconnaissance report filed

---

## Execution Order

### 1. CI Lockdown (next session — 30 min)
> *No forge should produce blades without a quality gate. Lock the gate first.*

- [x] Delete `.github/workflows/rust.yml` (legacy duplicate)
- [x] Add `--all-features` to clippy and test steps in `ci.yml`
- [x] Add `cargo audit` step for dependency vulnerability scanning
- [x] Pin Rust toolchain via `rust-toolchain.toml` (deterministic builds across local + CI)
- [ ] Set branch protection on `main` (require CI pass, no direct push) ← GitHub UI
- [ ] Verify: PR from `work` → `main` triggers full pipeline ← after push

**Why first:** Every change after this gets automatic fmt/clippy/test/audit gates.
Without it, regressions sneak in unnoticed. 10 minutes of config saves hours of
debugging later.

### 2. First Tagged Release — `v0.1.0` (same session — 15 min)
> *You can't distribute what you haven't stamped.*

- [ ] Verify `release.yml` triggers on `v*` tags
- [ ] Build locally with `--release`
- [ ] Run `./target/release/mash-setup --version` — confirm version string reflects `0.1.0`
- [ ] Tag `v0.1.0` on `main` after merging current `work`
- [ ] Confirm GitHub Release appears with both binaries + SHA256
- [ ] Verify checksums: `sha256sum -c mash-setup-*.sha256`
- [ ] Smoke test: download aarch64 binary on Pi, confirm it runs

**Why second:** Unlocks step 3 entirely. Also proves the release pipeline works
before we depend on it.

### 3. Retire `bootstrap.sh` (next session — 30 min)
> *The scaffolding served its purpose. Replace it with a bridge that doesn't
> require a forge on-site.*

- [ ] Slim `bootstrap.sh` to ~20 lines: detect arch, download binary, verify SHA256, exec
- [ ] Remove Rust/git/cargo install logic (no longer needed)
- [ ] Remove font/Hyprland/makepkg logic (mash-setup handles these)
- [ ] Add `uname -m` → target triple mapping (`aarch64` → `aarch64-unknown-linux-gnu`)
- [ ] Document the one-liner curl install as the primary method
- [ ] Test on clean Pi (no Rust installed)

**Why third:** Depends on tagged releases existing. Removes the 10-minute cargo
build tax for end users. Biggest UX improvement per line of code changed.

### 4. Phase 3: Pi 4B HDD Tuning (2-3 sessions)
> *The primary hardware gets its dedicated optimization pass.*

- [ ] Preflight checks: USB 3.0 detection, HDD health, partition layout
- [ ] I/O scheduler tuning for external USB 3.0 HDD
- [ ] Mount options optimization (noatime, commit interval)
- [ ] Swap configuration for 8GB RAM + HDD
- [ ] Kernel parameter tuning (vm.swappiness, dirty ratio)
- [ ] All changes gated behind `PhaseContext::run_or_record()`

**Why fourth:** Feature work on the primary target hardware. The hardening audit
gave us clean interfaces — build on them while they're fresh.

### 5. Driver Test Harness (1-2 sessions)
> *Test the walls after you've mined deeper.*

- [ ] Create test fixtures for each distro driver (Arch, Debian, Fedora)
- [ ] Mock `SystemOps` + `PhaseContext` for unit-level driver testing
- [ ] Exercise each driver's phase list against the Phase trait contract
- [ ] Verify dry-run mode produces correct `DryRunEntry` logs per driver
- [ ] Add to CI as a required check

**Why fifth:** With Phase 3 adding new phases, the test harness locks in the
expanded surface area. Test what you've built, then keep building.

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
