# Macro Inventory

**Created**: 2026-03-03
**Status**: Comprehensive Analysis (Phase 2)

## Summary

Comprehensive macro inventory for the MASH installer codebase. This report documents both declarative and procedural macro usage patterns across the workspace.

## Declarative Macros (macro_rules!)

### Analysis Results
- No custom declarative macros found in the current codebase.
- The project follows a strategy of using standard Rust functions and traits over custom macros for better maintainability and code clarity.

## Procedural Macros (Derive)

The codebase heavily utilizes procedural macros from the standard ecosystem to simplify serialization, CLI parsing, and error handling.

### Core Derive Macros
| Macro | Crates Providing | Usage in MASH | Purpose |
|-------|------------------|---------------|---------|
| `Debug` | `std` | Workspace-wide | Standardized debug representation. |
| `Clone` | `std` | Workspace-wide | Facilitating explicit deep copying of models. |
| `Default` | `std` | Workspace-wide | Providing canonical default values for configurations. |
| `PartialEq` | `std` | `installer-model` | Enabling comparison for tests and UI states. |
| `Serialize` | `serde` | `installer-model`, `installer-core` | Serialization to JSON/TOML for configuration persistence. |
| `Deserialize` | `serde` | `installer-model`, `installer-core` | Deserialization from configuration files and API responses. |
| `Parser` | `clap` | `installer-cli`, `wallpaper-downloader` | Command-line argument parsing and validation. |
| `ValueEnum` | `clap` | `installer-core` | Mapping enums to CLI arguments (e.g., install modes). |
| `ThisError` | `thiserror` | `installer-core`, `mash-system` | Derived implementation of the `std::error::Error` trait. |

### Usage Patterns
- **Serialization Patterns**: `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]` is the most common pattern for configuration models in `installer-model`.
- **Error Patterns**: `#[derive(ThisError, Debug)]` is used to define structured error hierarchies in `installer-core`.

## Procedural Macros (Attribute)

### Core Attribute Macros
| Macro | Crates Providing | Usage in MASH | Purpose |
|-------|------------------|---------------|---------|
| `#[tokio::main]` | `tokio` | `installer-cli` | Entry point for the asynchronous runtime. |
| `#[tokio::test]` | `tokio` | `installer-core`, `mash-system` | Running asynchronous tests with the tokio executor. |
| `#[async_trait]` | `async-trait` | `installer-core`, `mash-system` | Enabling async methods in traits (e.g., `Phase` and `SystemOps`). |
| `#[test]` | `std` | Workspace-wide | Standard unit test runner. |

## Function-like Macros (Standard Library)

| Macro | Purpose | Usage Frequency |
|-------|---------|-----------------|
| `vec![]` | Vector initialization | High (30+ occurrences) |
| `format!()` | String formatting | High (15+ occurrences) |
| `println!()` | Console output | Low (primarily in CLI main) |
| `hash!()` | Hash map initialization | Low |

## Recommendations

### Usage Guidelines
1. **Prefer Functions**: Only use macros when a function or trait implementation cannot provide the required behavior (e.g., DSLs, boilerplate reduction like `serde`).
2. **Standard Crates**: Stick to well-vetted procedural macros (`serde`, `clap`, `thiserror`, `tokio`).
3. **Avoid Complexity**: If a custom macro is required, keep it under 20 lines and document its pattern matching thoroughly.

## Next Steps
- Periodically audit `Cargo.toml` for new proc-macro dependencies.
- Ensure all custom enums that are exposed to the CLI use `clap::ValueEnum` for consistency.

"*Runes should be used sparingly; let the logic flow like clear ale.*" — Bard 🍺⚒️
