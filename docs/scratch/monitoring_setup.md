# Continuous Quality Monitoring: Keeping the Forge Clean

**Created**: 2026-03-03
**Status**: Implementation Plan (Phase 3)

## Summary

This plan outlines the strategy for ongoing monitoring of code quality, technical debt, and build health.

## Monitoring Objectives

1. **Trend Tracking**: Monitor the growth of technical debt and file complexity over time.
2. **Alerting**: Early warning on dependency bloat or build performance regressions.
3. **Accountability**: Transparent metrics on test coverage and clippy compliance.

## Monitoring Mechanisms

### 1. Build Performance Dashboard (Phase 3.2.1)
- **Tool**: `hyperfine` integrated into CI logs.
- **Metric**: Total build time for clean and incremental builds.
- **Target**: Clean build < 5 minutes, Incremental < 30 seconds.

### 2. Code Coverage Tracking (Active)
- **Tool**: `cargo tarpaulin` + `Codecov`.
- **Enforcement**: Build failure if coverage drops below the established threshold (80%).

### 3. Regular Quality Audits
- **Cadence**: Once per minor release or at the end of each major Shaft.
- **Tasks**:
    - Run `cargo tree -d` to check for new duplicates.
    - Run `cargo audit` for security vulnerabilities.
    - Check for new "Large Files" (>600 lines).

### 4. Technical Debt Backlog
- **Location**: `docs/scratch/remediation_plan.md`.
- **Process**: New structural issues or refactor requirements must be added to the remediation plan as they are discovered.

## Future Tooling (Roadmap)
- **Bacon**: Integration for local background quality checks.
- **Custom Dashboard**: A TUI-based "Doctor's Dashboard" for the `scry` and `doctor` commands to show workspace health.

"*Vigilance is the price of a green build.*" — Bard 🍺⚒️
