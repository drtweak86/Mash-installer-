# `mash-installer` - Analysis and Improvement Plan (Plan B)
> **Neon Chronicle (Technical polish)**: PlanB keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


This document provides a detailed analysis of the `mash-installer` project, highlighting existing strengths, identifying areas for immediate fixes, and proposing further improvements.

### 1. What's Missing or Wrong

*   **Limited Error Recovery:** The current error handling approach immediately halts the installation on the first failure. There's no mechanism to collect multiple errors, attempt recovery for non-critical issues, or suggest granular recovery steps to the user beyond re-running `mash-setup doctor`.
*   **`InstallContext` as a "God Object":** The `InstallContext` struct is very large and passed to every phase. This tight coupling makes individual phases harder to reason about, test in isolation, and maintain, as any phase could theoretically access or modify any part of the global state.
*   **Implicit Phase Contracts:** While phases are functions, their exact responsibilities and side effects are implicit. Without formal contracts or interfaces, it's challenging to ensure consistency and prevent unintended interactions between phases.
*   **Lack of Dedicated Test Suite:** There's no apparent comprehensive test suite (unit, integration) for `installer-core`. This significantly increases the risk associated with refactoring or adding new features, as changes can introduce regressions that are only caught during manual installation attempts.
*   **Inconsistent Input Validation (Inferred):** Given the nature of an installer interacting with diverse system components and user inputs, there's a potential for insufficient validation and sanitization of external data, leading to unexpected behavior or security vulnerabilities.

### 2. What's Correct

*   **Modular, Phase-Based Structure:** The division of the installation process into distinct, named phases (e.g., "System packages", "Rust toolchain") is a robust and understandable architectural pattern. It improves code organization and conceptual clarity.
*   **`DistroDriver` for Platform Abstraction:** The `DistroDriver` trait is an excellent design choice for abstracting away distribution-specific details (package managers, service names, repo configurations). This promotes extensibility and simplifies adding support for new Linux distributions.
*   **Effective Use of `anyhow::Result`:** The consistent use of `anyhow::Result` for error propagation throughout the codebase is a good practice, simplifying error handling and providing rich context for failures.
*   **User-Friendly Progress Feedback with `indicatif`:** The integration of `indicatif` for multi-progress bars provides clear, professional, and real-time visual feedback to the user, enhancing the overall user experience during potentially long installation processes.
*   **Clear Component Separation (Modules):** The individual modules within `installer-core` (e.g., `pkg.rs`, `docker.rs`, `github.rs`) correctly encapsulate logic for specific installation components, promoting better organization.

### 3. Fixes Needed for Program to Function as Intended

*   **Implement Robust External Command Error Reporting:** Ensure that all shell commands executed by the installer capture and log both `stdout` and `stderr` on failure, providing crucial diagnostic information when a phase fails.
*   **Enhance `mash-setup doctor` for Pre-flight Validation:** Expand the `doctor` module to perform a more comprehensive suite of pre-installation checks, including but not limited to: available disk space, network connectivity to critical endpoints, user permissions for directories, and necessary package manager availability. This should ideally prevent common failures before the installation begins.
*   **Ensure Idempotency for Key Operations:** Modify installation phases where applicable to ensure that re-running them does not cause errors or redundant operations. For instance, package installation phases should check if a package is already installed before attempting to install it again.
*   **Implement Transactional/Rollback Capabilities for Critical Steps:** For phases involving significant system changes (e.g., partitioning, critical service configuration), investigate and implement mechanisms to revert or rollback changes if the phase fails, to prevent leaving the system in a broken or inconsistent state.
*   **Clear Definition of User Interaction Points:** Explicitly define where and when user interaction (e.g., prompts for confirmation, password input) is expected, and ensure these interactions are handled gracefully, especially in interactive vs. non-interactive modes.

### 4. Suggested Improvements to the Program

*   **Decouple UI from `installer-core`:** Transition all UI rendering (progress bars, `println!`) from `installer-core` to `installer-cli`. `installer-core` should become a pure library that returns structured events or results, allowing `installer-cli` to interpret and present them. This enhances modularity and allows for alternative UIs.
*   **Introduce a Formal `Phase` Trait or Interface:** Define a `Phase` trait that each installation step implements. This would enforce a common structure, clarify responsibilities, and make it easier to add, remove, or reorder phases programmatically.
*   **Implement a Dedicated Logging Framework:** Beyond `tracing::info!` and `error!`, integrate a more robust logging framework (e.g., `log4rs` or `tracing-subscriber` with file appenders) to create detailed, structured logs for every installation run. This is invaluable for debugging and support.
*   **Refactor `InstallContext` into Smaller Contexts:** Break down the monolithic `InstallContext` into smaller, more granular context objects. Each phase would then only receive the specific data it requires, reducing coupling and improving testability.
*   **Expand `dry_run` Fidelity:** Enhance the `dry_run` mode to provide a more accurate simulation of the full installation process, including detailed reports of what shell commands would have been run, what files would have been modified, and what services would have been started. This provides invaluable pre-execution validation.