# Mining Projects – Explored Maps
> Archive of completed work, closed at the end of each session.

## Session: 2026-02-20 – Packaging Sprint & Ledger Update

### Summary
The courier lanes were refitted so `package-deb` and `package-rpm` now trail the build-release job, the publish gate bundles their results plus the ebbing PKGBUILD, and the ledger received a fresh chapter while I watched the neon sky for warnings.

### Deliverables
- [x] Added `.deb/.rpm` packaging jobs and publish-asset bundling to `.github/workflows/release.yml`.
- [x] Ran `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- [x] Logged the sprint in `docs/HISTORY.md`, `docs/mining-projects/maps.md`, and this archive.

### Build Status
- cargo fmt: clean
- cargo clippy --all-targets --all-features -- -D warnings: clean
- cargo test: clean

## Session: 2026-02-20 – Release Trigger

### Summary
`work` just pushed `v0.1.2` into the sky; the release workflow now runs with the new `.deb`, `.rpm`, and checksumed `PKGBUILD` cohorts.

### Deliverables
- [x] Push `v0.1.2` tag to `origin` so `.github/workflows/release.yml` fires the build-release → package-deb/rpm → publish relay.
- [ ] Monitor GitHub Actions for the release job results and confirm the artifacts land before closing the vault.

### Build Status
- release workflow triggered by `v0.1.2` (check GitHub Actions for the green signal).

---

## Session: 2026-02-20 – Shell Polish & Software Tiers

### Summary
Starship, Kitty, and the goblin eza alias palette now land with guarded rc blocks from `resources/shell`. The interactive “software tiers” menu ships a dozen categories with five curated S/A choices each, so the miner can either let the installer pick the S-tier champions or select their own build. The new plan is logged into `InstallOptions`, and the tier doc mirrors the menu.

### Deliverables
- [x] Added Starship + Kitty + eza config deployment to the shell phase with backups and guarded blocks.
- [x] Implemented `SoftwareTierPlan`, wired it through `InstallOptions`, and introduced the CLI menu that surfaces the 12-category, five-option list.
- [x] Updated `docs/incoming-files/software_tiers.md` to document the curated tiers.
- [x] fmt/clippy/test trilogy stayed green while wiring up the new UX.

### Build Status
- cargo fmt: clean
- cargo clippy --all-targets --all-features -- -D warnings: clean
- cargo test: clean

---

## Session: 2026-02-20 – Phase 4: Hardening Complete

### Summary
Sealed the forge against the neon rain. Five hardening measures implemented:
lockfile (concurrent run prevention), TLS hardening (all curl calls pinned to TLS 1.2+),
signal handling (graceful SIGINT/SIGTERM shutdown with rollback), rollback expansion
(zsh, rust, argon phases register actions), and filesystem forensics infrastructure.

### Deliverables
- [x] `lockfile.rs` — `InstallerLock` via `nix::fcntl::Flock`, exclusive non-blocking
- [x] `cmd.rs` — `curl_flags()` helper centralizing TLS enforcement
- [x] TLS hardened: `apt_repo.rs`, `rclone.rs`, `argon.rs`, `zsh.rs`
- [x] `signal.rs` — `SignalGuard` via `signal-hook`, `Arc<AtomicBool>` flag
- [x] `verify.rs` — `verify_file_written()` + `sync_file()` infrastructure
- [x] Rollback: `zsh.rs` (omz dir), `rust.rs` (note), `argon.rs` (config files)
- [x] `orchestrator.rs` — acquires lockfile + signal guard before phases
- [x] `phase_runner.rs` — checks signal between phases, triggers rollback
- [x] 13 new tests (99 total), all green
- [x] `signal-hook = "0.3"` added to Cargo.toml

### Build Status
- cargo fmt: clean
- cargo clippy --all-targets --all-features -- -D warnings: clean
- cargo test: 99 tests passing

---

## Session: 2026-02-20 – Phase 3: Pi 4B HDD Tuning Complete

### Summary
Completed the remaining Phase 3 tasks: mount options optimization, swap configuration,
kernel parameter tuning, and full Phase system integration. The `pi4b_hdd` module now
runs as a proper phase in `PhaseRegistry`, self-skipping on non-Pi4B hardware.

### Deliverables
- [x] `optimize_mount_options()` — reads /proc/mounts, recommends noatime/commit=60 for ext4 HDD
- [x] `configure_swap()` — checks swapon, recommends 2GB swap on external HDD
- [x] `tune_kernel_params()` — reads /proc/sys/vm/, recommends swappiness=10, dirty_ratio=15
- [x] `install_phase()` wired into PhaseRegistry as "Pi 4B HDD Tuning"
- [x] New exports: MountOptimization, SwapConfig, KernelParam
- [x] 12 pi4b_hdd tests (86 total), all green
- [x] Pushed to work branch, PR #8 updated

### Build Status
- cargo fmt: clean
- cargo clippy --all-targets --all-features -- -D warnings: clean
- cargo test: 86 tests passing

---

## Session: 2026-02-20 – Step 2: First Tagged Release v0.1.0

### Summary
Created the first official tagged release v0.1.0 after completing Phase 2 hardening.
The release pipeline successfully built and published binaries for both x86_64 and aarch64
architectures with SHA256 checksums.

### Deliverables
- [x] Verified release.yml workflow triggers on v* tags
- [x] Built locally with --release (successful)
- [x] Confirmed version string reflects 0.1.0 (installer-cli v0.1.0)
- [x] Tagged v0.1.0 on main after merging work branch
- [x] Confirmed GitHub Release v0.1.0 with both binaries + SHA256 checksums
- [x] Verified checksums via sha256sum -c
- [x] Release workflow completed successfully (all jobs green)

### Release Assets
- mash-setup-x86_64-unknown-linux-gnu (3.7MB)
- mash-setup-aarch64-unknown-linux-gnu (3.7MB)
- mash-setup-x86_64-unknown-linux-gnu.sha256
- mash-setup-aarch64-unknown-linux-gnu.sha256

### Build Status
- cargo fmt: clean
- cargo clippy --all-targets --all-features -- -D warnings: clean
- cargo test: all passing
- GitHub Actions: 5/5 checks passing (fmt, clippy, test, build, audit)

---

## Session: 2026-02-20 – Step 3: Retire bootstrap.sh

### Summary
Slimmed down bootstrap.sh from 134 lines to ~20 lines, transforming it from a
full installation script to a lightweight binary downloader. The new version:
- Detects architecture and maps to target triple
- Downloads pre-built binary from GitHub Releases
- Verifies SHA256 checksum before execution
- Executes the downloaded binary

### Deliverables
- [x] Slimmed bootstrap.sh to ~20 lines
- [x] Removed Rust/git/cargo install logic (no longer needed)
- [x] Removed font/Hyprland/makepkg logic (handled by mash-setup)
- [x] Added uname -m → target triple mapping
- [x] Tested on local machine (successfully downloads and runs from GitHub)
- [ ] Document one-liner curl install as primary method
- [ ] Test on clean Pi (no Rust installed)

### New bootstrap.sh Flow
```bash
# Detect arch → download binary → verify SHA256 → exec
```

### Benefits
- Eliminates 10-minute cargo build tax for end users
- Reduces prerequisites (no Rust, git, or cargo needed)
- Faster installation on Pi (download vs compile)
- Smaller attack surface (no shell-out to rustup)

---

## Session: 2026-02-20 – Phase 2 Completion Audit (Hardening)

### Summary
Full audit of installer-core for unwraps, panics, implicit assumptions, leaky
abstractions, stringly-typed errors, and silent fallthrough logic. All findings
addressed across 5 blocks of surgical fixes.

### Deliverables
- logging.rs: Replaced mutex `.unwrap()` with `io::Error::other()`
- zsh.rs: Replaced `writeln!().unwrap()` with `?` propagation
- dry_run.rs: Deleted `print_summary()`, moved rendering to CLI
- orchestrator.rs: Removed all eprintln/stdin, routed through PhaseObserver
- doctor.rs: All output routed through `&mut dyn Write`
- config.rs: `init_config()` and `show_config()` accept `&mut dyn Write`
- phase_runner.rs: Added `PhaseEvent::Warning`, `PhaseObserver::confirm()`, `warnings` on `PhaseOutput`
- context.rs: Added `PhaseContext::record_warning()`, `warnings` on `PhaseMetadata`
- docker.rs, rust.rs, zsh.rs, github.rs: Replaced error swallowing with `record_warning()`
- error.rs: Added `dry_run_log` to `InstallationReport`
- lib.rs: Removed `RealSystem` from public exports
- ui.rs (CLI): Implemented `confirm()` and `Warning` event handling
- main.rs (CLI): Added `print_dry_run_summary()` for report-based dry-run output

### Build Status
- cargo fmt: clean
- cargo clippy --all-targets --all-features -- -D warnings: clean
- cargo test: all passing

---

## Session: 2026-02-20 – Step 1: CI Lockdown

### Summary
Locked down the CI pipeline with proper flags, dependency auditing, toolchain
pinning, and cleanup of legacy workflow duplication.

### Deliverables
- [x] Deleted `.github/workflows/rust.yml` (legacy duplicate of ci.yml)
- [x] Added `--all-features` to clippy and test steps in `ci.yml`
- [x] Added `cargo audit` job for dependency vulnerability scanning
- [x] Pinned Rust 1.93.1 via `rust-toolchain.toml` (deterministic local + CI builds)
- [x] Set branch protection on `main` (via `gh api`)
- [x] Verify: PR from `work` → `main` triggers full pipeline (PR #6 — 5/5 green)
- [x] Fix ShellCheck: removed unused `BOLD` var (SC2034), added source directive (SC1091)

### Build Status
- cargo fmt: clean
- cargo clippy --all-targets --all-features -- -D warnings: clean
- cargo test: 68 tests passing

---

## Session: 2026-02-20 – Phase 2 Closure (Final)

### Summary
The neon ledger now proclaims Phase 2 complete: runner/registry split, InstallationReport contract, PhaseContext helpers, Pi detection lore, and CLI wiring all sealed with the fmt/clippy/test trilogy.

### Deliverables
- [x] Created `.bard-persona.md` with the Drunken Dwarf Bard manifesto.
- [x] Split `installer-core/lib.rs` exports into `runner` & `registry` wrappers and kept the CLI aligned with `InstallationReport`.
- [x] Updated README/HISTORY/ARCH/modules/improvement-plan/QA notes to deck them with the Phase 2 completion story.
- [x] Logged the closure in `docs/mining-projects/maps.md` and noted the paused Phase 3.
- [x] Ran `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` from `/work/Mash-installer` with green results.

### Notes
- Phase 3 (Pi 4B HDD) work awaits the ledger flip; do not start before that.
