# Shaft U: THE GREAT REFACTOR (Hardening & Deduplication)

**Shaft Title**: THE GREAT REFACTOR (Hardening & Deduplication)
**Status**: ⏳ PLANNING COMPLETE | 🌑 IMPLEMENTATION PENDING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-01

## 🎯 SCOPE

This shaft focuses on a full codebase purification to ensure structural integrity and professional-grade hardening. It addresses technical debt, eliminates redundancy, and strengthens the security and resilience of the MASH installer.

1. **Codebase Audit & Deduplication**: Identify and eliminate redundant logic across `installer-core`, `installer-cli`, and distro-specific crates.
2. **Structural Refactoring**: Top-to-bottom refactor for idiomatic Rust, moving from deep inheritance-like structures to composition using the `SystemOps` trait.
3. **Hardening & Security**: Enhance resilience with strict input validation, zero-side-effect dry runs, and "Advice Strings" for all failure points.
4. **Error Handling Standardization**: Unified use of `thiserror` for library modules and `anyhow` for CLI-level context.

## 📁 FILES TO BE CREATED OR TOUCHED

### Modified Files
- `installer-core/src/package_manager.rs` - Consolidate common package patterns
- `installer-cli/src/ui.rs` - Unify UI components to prevent drift
- `installer-core/src/lib.rs` - Flatten thin shims and clean exports
- `installer-core/src/phase_runner.rs` - Refactor for robust state transitions
- `installer-core/src/dry_run.rs` - Harden side-effect gating
- `installer-core/src/logging.rs` - Enhance Advice String system
- `Cargo.toml` - Workspace-wide dependency alignment

## ⚒️ METHODOLOGY

### Technical Strategy
1. **Composition over Inheritance**: Leverage `SystemOps` and traits to decouple logic from specific implementations.
2. **Atomic Refactoring**: Small, verifiable changes following the KCS (Keep Commits Small) rule.
3. **Static Analysis**: Heavy use of `clippy` and `cargo-audit` to identify hardening opportunities.
4. **Advice-Driven Errors**: Every error must guide the user/miner toward a solution.

## 📦 DELIVERABLES

### Phase 1: Deduplication ✅ COMPLETE (2026-03-01)
- [x] Centralize package management logic in `installer-core`.
- [x] Remove duplicate TUI widget implementations in `installer-cli`.
- [x] Align all workspace crate versions and dependencies.
- [x] Unify `CliPhaseObserver` and delete legacy UI code.
- [x] Enhanced `cmd::Command` with `sudo()` and `dry_run()` support.

### Phase 2: Structural Refining ✅ PLANNED
- [ ] Refactor `Phase` execution registry for better error recovery.
- [ ] Implement `SystemOps` fully across all distro drivers.
- [ ] Clean up public API surface in `installer-core`.

### Phase 3: Hardening ✅ PLANNED
- [ ] Implement `Validator` trait for all configuration inputs.
- [ ] Audit all `unsafe` blocks (if any) and document invariants.
- [ ] Enhance `dry_run` to provide a complete "Pre-flight Audit Report".

## 🔧 VERIFICATION CHECKLIST
- [ ] `cargo test --workspace` passes with 100% success.
- [ ] `cargo clippy --all-targets -- -D warnings` is silent.
- [ ] `cargo audit` reports zero vulnerabilities.
- [ ] Dry-run report correctly identifies all planned actions without execution.

"*The blade is only as good as the steel it's forged from. Time to purify the ore.*" — Bard 🍺⚒️
