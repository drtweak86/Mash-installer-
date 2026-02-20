# Refactoring and Improvement Plan for Mash-Installer
> **Neon Chronicle (Technical polish)**: PlanA keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


This document outlines a 10-point plan to refactor and improve the `mash-installer` codebase. The goal is to enhance modularity, testability, and maintainability.

### 1. Refactor the `InstallContext` God Object

**Problem:** The `InstallContext` struct holds all state and is passed to every installation phase. This creates tight coupling and makes it difficult to test phases in isolation.

**Proposal:**
- Break down `InstallContext` into smaller, more focused context structs (e.g., `PlatformContext`, `UserOptionsContext`, `UIContext`).
- Each phase function should only receive the specific context it needs.
- Use a dependency injection pattern to provide the necessary contexts to the phases.

### 2. Introduce a Formal Phase Trait

**Problem:** Phases are currently defined as simple functions (`fn(&InstallContext) -> Result<()`), which is not very descriptive.

**Proposal:**
- Define a `Phase` trait with methods like `name()`, `description()`, `execute(&mut PhaseContext)`, and `should_run(&AppContext)`.
- Each installation phase would implement this trait.
- This provides a more structured and extensible way to define phases.

### 3. Decouple UI from Core Logic

**Problem:** The `run_with_driver` function in `installer-core` is responsible for both orchestrating the installation and managing the `indicatif` progress bars. This violates the separation of concerns.

**Proposal:**
- Move all UI-related code (progress bars, `println!` statements) out of `installer-core` and into `installer-cli`.
- `installer-core` should return events or results (e.g., `PhaseStarted`, `PhaseCompleted`, `PhaseFailed`), and the CLI should be responsible for rendering the UI based on these events.

### 4. Create a more Flexible Phase Runner

**Problem:** The phase execution loop is a hardcoded `for` loop within `run_with_driver`.

**Proposal:**
- Create a `PhaseRunner` struct responsible for executing a list of `Phase` objects.
- This runner would handle the logic for progressing through phases, handling errors, and collecting results.
- This would also make the `run_with_driver` function much simpler.

### 5. Enhance Error Handling and Reporting

**Problem:** The installer currently stops on the first error.

**Proposal:**
- Implement an error handling strategy that allows the installer to continue and report multiple errors if possible.
- For failed phases, provide more detailed error messages and suggest potential solutions or cleanup steps.
- Create a dedicated error type for the installer that can encapsulate more context.

### 6. Implement a Comprehensive Test Suite

**Problem:** There is a lack of tests, which makes refactoring risky.

**Proposal:**
- Add unit tests for individual functions and modules, especially in `installer-core`.
- Use mocking (e.g., with the `mockall` crate) to test phases in isolation without needing a live system.
- Create integration tests that run the installer against a mocked `DistroDriver`.

### 7. Centralize Configuration Management

**Problem:** Configuration is loaded in a single place, but access to it is spread throughout the `InstallContext`.

**Proposal:**
- Create a dedicated `ConfigService` or `ConfigRepository` that is responsible for providing configuration values.
- This service can handle loading, validation, and providing default values.
- This will make it easier to manage configuration and add new options in the future.

### 8. Abstract System Operations

**Problem:** There is likely duplicated logic for running shell commands, managing files, and making network requests across the different phase modules.

**Proposal:**
- Create a set of utility modules or services for common system operations (e.g., `FileSystem`, `ShellCommand`, `Downloader`).
- Use these abstractions in the phases instead of directly calling low-level APIs.
- This will improve code reuse and make it easier to add features like logging or dry-run support for these operations.

### 9. Externalize UI Strings

**Problem:** All UI strings (phase labels, error messages, etc.) are hardcoded in the Rust source.

**Proposal:**
- Move all user-facing strings into a separate configuration file (e.g., a TOML or JSON file).
- This will make it easier to change wording, and lays the groundwork for future internationalization (i18n).

### 10. Improve Library API Design

**Problem:** As noted in point 3, `installer-core` acts more like an application than a library by printing directly to the console.

**Proposal:**
- Redesign the public API of `installer-core` to be more programmatic. `run_with_driver` should return a `Result` with detailed information about the installation, not just `Ok(())`.
- The CLI should then be responsible for interpreting this result and presenting it to the user.

## Phase 2 Priority (with D-03)
Phase 2 now begins with the dry-run gate (D-03) before the remaining R-items:

1. **D-03 – Dry-run gating (`PhaseContext::run_or_record()`)** – implemented and already used inside `PhaseRunner` to avoid redundant dry-run checks.  
2. **R-02 – Split `lib.rs`** – carved out orchestrator, runner, options, and sudo helpers.  
3. **R-08 – Typed `PackageSpec`** – type-safe package lists ready for the context helpers.  
4. **R-01 – PhaseContext surface** – expand `ctx.*` so phases only touch the context API.  
5. **R-03 – Structured `PhaseOutput`** – each phase returns metadata for richer reporting.  
6. **R-04 – PhaseRegistry** – metadata-driven registration replaces the `build_phase_list()` chain.  
7. **R-05 – ConfigService errors** – explicit `ConfigError` variants and active-path reporting.  
8. **R-07 – Pi detection helpers** – add Raspberry Pi detection helpers to `PlatformContext`.  
9. **R-09 – Flatten `RunSummary`** – merge summary fields into `InstallationReport`.  
10. **R-06 – DriverTestHarness** – run once the API stabilizes.  
11. **R-10 – CLI/TUI split** – finish after the report/event shapes are final.

The same sequence is mirrored in `docs/improvement-plans.md` so QA reviewers can trace the path from priority to implementation.

## Rationale for the Execution Order
The dry-run gate (D-03) remains the throne room guard: every action now passes through `PhaseContext::run_or_record()` inside `PhaseRunner`, so the order listed above avoids premature dependencies and duplicated `dry_run` checks.
1. **D-03 – Dry-run gate first** keeps simulation stable before any helper slices are touched.  
2. **Refactor `InstallContext` (Plan Point 1)** splits state so traits and runners only request what they actually need.  
3. **Introduce a Phase trait (Plan Point 2)** once the contexts are lean enough to describe the installation state machine.  
4. **Build a flexible PhaseRunner (Plan Point 4)** that iterates trait objects, captures structured reporting, and enforces the gate.  
5. **Centralize configuration (Plan Point 7)** so the new context slices consume consistent defaults and validation.  
6. **Abstract common system operations (Plan Point 8)** and let them respect dry-run, logging, and caching concerns.  
7. **Enhance error handling (Plan Point 5)** after the helpers stabilize so rich failures bubble through the runner.  
8. **Externalize UI strings (Plan Point 9)** once the core stops printing, keeping messaging in config and freeing localization.  
9. **Decouple UI from core logic (Plan Point 3)** so the CLI/TUI react to events instead of forcing prints into the library.  
10. **Improve the library API design (Plan Point 10)** by returning structured `InstallationReport` data and letting the UI orchestrate presentation.  
11. **Implement the comprehensive test suite (Plan Point 6)** once the contracts are solid, exercising every distro driver through the harness.
