# Mining Projects – Explored Maps
> Archive of completed work, closed at the end of each session.

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
- [ ] Set branch protection on `main` (GitHub UI — requires manual action)
- [ ] Verify: PR from `work` → `main` triggers full pipeline

### Build Status
- cargo fmt: clean
- cargo clippy --all-targets --all-features -- -D warnings: clean
- cargo test: 68 tests passing
