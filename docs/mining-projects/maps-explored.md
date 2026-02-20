# Mining Projects – Explored Maps
> Archive of completed work, closed at the end of each session.

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
