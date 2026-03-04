# Remediation Plan: Addressing Technical Debt

**Created**: 2026-03-03
**Status**: Draft Plan (Phase 4)

## Summary

This plan outlines prioritized actions to address technical debt and structural issues identified during the Shaft Y codebase analysis.

## Prioritized Action Items

| Priority | Issue | Description | Effort | Risk | Actionable Step |
|----------|-------|-------------|--------|------|-----------------|
| **1 (Critical)** | **Circular Dependencies** | `installer-core` -> `installer-arch` -> `installer-cli` -> `installer-core`. | High | High | Define a clear, unidirectional flow for driver registration and execution. |
| **2 (High)** | **File Bloat** | `app.rs` (1558 lines), `menus.rs` (1351 lines). | Medium | Medium | Break down UI logic into smaller, per-screen sub-modules in `installer-cli/src/tui/`. |
| **3 (Medium)** | **Dependency Duplication** | Multiple `dirs` and `thiserror` versions. | Low | Low | Migrate to `[workspace.dependencies]` in root `Cargo.toml`. |
| **4 (Medium)** | **Macro Complexity** | 55 derive macros across model files. | Low | Low | (Task 2) Document and establish usage guidelines. (Partially Complete) |
| **5 (Low)** | **Clippy Timeout** | Complex modules causing clippy analysis timeouts. | High | Medium | Modularize complex functions and simplify generics in `installer-core`. |

## Refactoring Targets: File Bloat

| File | Current Lines | Target Lines | Strategy |
|------|---------------|--------------|----------|
| `app.rs` | 1558 | <500 | Extract `EventHandler`, `MenuController`, and `State` into separate modules. |
| `menus.rs` | 1351 | <400 | Create a registry for menu items and extract screen definitions into their own files. |
| `phase_runner.rs` | 926 | <500 | Extract individual phase execution logic into specialized sub-modules. |

## Remediation Strategy

### 1. Address Circular Dependencies (Top Priority)
- Introduce a trait-based registry for distro-specific drivers that `installer-core` can call without depending on the concrete types from `installer-arch/debian/fedora`.
- Move driver-related models into `installer-model`.

### 2. Decompose Monolithic Files
- Utilize the `tui/` subdirectory in `installer-cli` to better organize UI components.
- Standardize on 300-500 lines per file where possible.

### 3. Implement Quality Metrics
- Enforce function length limits (50 lines) via `clippy` and code reviews.

## Estimated Timeline

- **Phase 1: Quick Wins** (Dependencies, Macro Documentation): 1-2 days.
- **Phase 2: UI Decomposition** (app.rs, menus.rs): 3-5 days.
- **Phase 3: Structural Refactoring** (Circular Dependencies): 5+ days.

## Success Criteria

- **No Cycles**: Zero circular dependencies between crates.
- **Lighter Modules**: No file exceeds 600 lines.
- **Green Analysis**: `cargo clippy` completes workspace-wide in under 300s.

"*A clean forge is a fast forge; clear the soot before you start the next big job.*" — Bard 🍺⚒️
