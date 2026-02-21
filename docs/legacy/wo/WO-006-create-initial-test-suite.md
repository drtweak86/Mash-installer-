
> **Neon Chronicle (Technical polish)**: WO-006-create-initial-test-suite keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️

# Title
WO-006: Create Initial Test Suite for `config` module

# Source
- PlanA.md - Section 6: "Implement a Comprehensive Test Suite"
- PlanB.md - Section 1: "What's Missing or Wrong"

# Goal
To establish a foundation for automated testing within the project by creating the first unit tests for a simple, self-contained module. The `config.rs` module is a good candidate as it has minimal dependencies.

# Scope
- `installer-core/src/config.rs`
- A new test block (`#[cfg(test)]`) within `config.rs`.

# Non-goals
- Do not write tests for any other module.
- Do not set up a full integration test framework.
- Do not introduce complex mocking libraries at this stage.

# Steps
1.  In `installer-core/src/config.rs`, create a `mod tests` block annotated with `#[cfg(test)]`.
2.  Inside the `tests` module, import the necessary items from the parent module (`use super::*;`).
3.  Write the first unit test, `#[test] fn test_load_or_default_creates_default()`.
    -   This test should simulate a scenario where the config file does not exist.
    -   It will need to operate in a temporary, controlled directory to avoid interfering with the real user config. Libraries like `tempfile` can be useful here, but for a first pass, operating on a known (and cleaned up) path is also acceptable if `tempfile` is not yet a dependency.
    -   The test should call `load_or_default` (or a testable version of it).
    -   Assert that the returned `MashConfig` is equal to `MashConfig::default()`.
4.  Write a second unit test, `#[test] fn test_load_or_default_loads_existing()`.
    -   This test should first create a dummy `config.toml` file with a non-default value (e.g., a different `staging_dir`).
    -   It should then call `load_or_default` pointing to this dummy file's location.
    -   Assert that the loaded `MashConfig` contains the non-default value.

# Success criteria
-   Running `cargo test -p installer-core` executes the new tests.
-   Both tests in `config.rs` pass successfully.
-   The "Done" criteria from Plan C (`cargo fmt`, `cargo clippy`) are still met.

# Tests
-   This task *is* the creation of tests.
-   The tests are executed via `cargo test -p installer-core`.

# Risk
Low. This change adds new test code that does not run in a production build. It has no impact on the functionality of the installer itself.

# Commit message
```
test(config): create initial unit tests for config module

Establishes the foundation for automated testing by adding the first
unit tests to the project for the `installer-core::config` module.

The tests cover two primary scenarios:
1.  A default config is created when none exists.
2.  An existing config file is loaded correctly.

This addresses the "Lack of Dedicated Test Suite" noted in Plans A and B,
and provides a safety net for future refactoring.

Tests were executed and passed using `cargo test -p installer-core`.
```