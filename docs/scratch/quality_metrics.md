# Code Quality Metrics: Standards for the Forge

**Created**: 2026-03-03
**Status**: Adopted Standards (Phase 3)

## Summary

This document defines the quantitative metrics and qualitative standards that all code in the MASH installer must meet before being merged into the `main` branch.

## Quantitative Metrics

| Metric | Target | Requirement | Enforcement |
|--------|--------|-------------|-------------|
| **Function Length** | <50 lines | Mandatory | `clippy::too_many_lines` |
| **Nesting Depth** | <3 levels | Mandatory | `clippy::cognitive_complexity` |
| **Cyclomatic Complexity** | <15 | Highly Recommended | `clippy::cognitive_complexity` |
| **File Length** | <600 lines | Recommended | Manual/Code Review |
| **Test Coverage** | >80% | Mandatory | `cargo tarpaulin` (CI) |
| **Linting** | 0 warnings | Mandatory | `cargo clippy -D warnings` |
| **Formatting** | 0 errors | Mandatory | `cargo fmt --check` |

## Qualitative Standards

### 1. Naming Conventions
- Variables/Functions: `snake_case`.
- Structs/Enums: `PascalCase`.
- Constants: `SCREAMING_SNAKE_CASE`.
- Crates: `kebab-case`.

### 2. Documentation Rules (Always Be Documenting)
- Every public module must have a summary doc comment (`//!`).
- Every public struct, enum, and function must have a clear doc comment (`///`).
- Complex logic within functions should use inline comments explaining the *why*, not the *what*.

### 3. Error Handling
- Never use `unwrap()` or `expect()` in production code (except in tests or unreachable branches).
- Use `InstallerError` or `anyhow::Result` with context for all fallible operations.
- Errors must provide advice or recovery paths for the user.

### 4. Dependency Hygiene
- No circular dependencies between workspace crates.
- Every new dependency requires justification.
- Prefer `std` over external crates for simple utilities.

## Enforcement Mechanism

- **Pre-Commit**: Recommended local execution of `cargo fmt` and `cargo clippy`.
- **CI Pipeline**: The CI will fail on any violation of these standards (Formatting, Clippy warnings, Coverage < 80%).
- **Manual Review**: High-level structural standards (file length, decomposition) are enforced during the PR review process.

"*A standard is a promise you make to your future self.*" — Bard 🍺⚒️
