# Refactoring and Improvement Plan for Mash-Installer

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
