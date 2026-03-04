# Dependency Consolidation Plan

**Created**: 2026-03-03
**Status**: Draft Plan (Phase 3)

## Summary

This document outlines the strategy for consolidating and optimizing dependencies across the MASH workspace to improve compile times, reduce binary sizes, and simplify maintenance.

## Identified Version Conflicts

| Dependency | Crates Affected | Version Conflict | Impact |
|------------|-----------------|-------------------|--------|
| `dirs` | `mash-system` (5.0) vs `installer-core`, `installer-model` (6.0.0) | Duplicate `dirs` build and potential logic inconsistencies. | Medium |
| `thiserror` | `mash-system` (1.0) vs `installer-core`, `installer-model` (2.0.18) | Dual compilation of `thiserror` macro and library. | Low |
| `indicatif` | `installer-core` (0.18) vs `installer-cli` (0.18.4) | Minor version mismatch. | Low |
| `nix` | `mash-system` (0.31.2) vs `installer-core` (0.31.2) | Consistent but should be managed centrally. | Low |

## Consolidation Strategy

### 1. Root Workspace Management
Migrate all shared dependencies to the `[workspace.dependencies]` section in the root `Cargo.toml`. This ensures version alignment across all crates.

### 2. Feature Unification
Standardize feature usage for heavy crates like `tokio` and `reqwest` to avoid multiple compilations with slightly different feature sets.

### 3. Migration Roadmap

#### Phase 1: Align Versions (Immediate)
Update `Cargo.toml` in all crates to use the latest agreed-upon versions:
- `dirs`: Standardize on `6.0.0`
- `thiserror`: Standardize on `2.0.18`
- `indicatif`: Standardize on `0.18.4`
- `tokio`: Standardize on `1.0` with consistent features.

#### Phase 2: Root Cargo.toml Refactoring
1. Add `[workspace.dependencies]` to root `Cargo.toml`.
2. Move `anyhow`, `serde`, `thiserror`, `tracing`, `tokio`, `dirs`, `sysinfo`, `clap` to the workspace level.
3. Update individual crates to use `{ workspace = true }`.

#### Phase 3: Unused Dependency Removal
Run a manual audit and/or use `cargo-udeps` to identify and remove truly unused dependencies. (Current candidates: `num_cpus` in `installer-core` if not used directly).

## Success Criteria

- **Single Build**: `cargo tree -d` returns zero duplicate dependencies.
- **Faster Compile**: Reduction in total build time for a clean workspace build.
- **Centralized Versions**: All major versions are controlled from the root `Cargo.toml`.

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Breaking changes in `dirs` v5 -> v6 | Perform full integration tests across all distros (Arch, Debian, Fedora). |
| Compatibility issues with `thiserror` v2 | Verify that custom error types still compile and behave correctly. |

"*A single version of the truth is better than a dozen conflicting scrolls.*" — Bard 🍺⚒️
