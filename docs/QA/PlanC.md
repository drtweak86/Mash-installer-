# `mash-installer` - Development Workflow and Quality Assurance Plan (Plan C)

This document outlines the operational plan for developing and maintaining the `mash-installer`, focusing on "Done" criteria, execution order, commit strategy, branching, testing, rollback, and release processes.

### 1. Define "Done" Criteria

A task or pull request is considered "Done" and ready for integration when all the following conditions are met:

*   **Builds on aarch64:** The project successfully compiles for the `aarch64` target, typically verified via `cargo build --target=aarch64-unknown-linux-gnu` or continuous integration.
*   **`cargo fmt --check` passes:** All Rust code adheres to the project's formatting standards as enforced by `cargo fmt`.
*   **`cargo clippy --all-targets --all-features -- -D warnings` passes:** The codebase is free of compiler warnings, indicating high code quality and adherence to best practices.
*   **Tests pass:** All automated tests, executed via `maelstrom` where supported, return success. If `maelstrom` cannot be used (e.g., due to kernel dependencies or `ENOSYS` errors), approved fallback testing mechanisms must pass.
*   **Dry-run mode/non-destructive checks (where relevant):** For changes involving potentially destructive or system-altering operations, a `dry-run` mode or equivalent non-destructive checks must be executed and verified to ensure intended behavior without actual system modification.

### 2. Execution Order (Risk-First Approach)

Changes and tasks should be executed and prioritized in the following order to minimize risk and ensure stability:

1.  **Safety Rails First:** Implement and thoroughly verify all critical checks, input validation, and mechanisms that prevent unintended destructive operations. This phase focuses on building robust guardrails and improving error handling.
2.  **Correctness Fixes:** Address any identified bugs, defects, or functional inaccuracies. Ensure the program performs its intended function reliably and correctly.
3.  **Refactor/Mechanics:** Apply structural improvements, modularity enhancements, and underlying mechanical changes that improve maintainability, extensibility, and performance without altering external behavior.
4.  **UX Polish:** Focus on user experience improvements, clearer user feedback, enhanced messaging, and aesthetic refinements.

### 3. Commit Plan (Atomic, Branch-Based - ABB)

*   **1 Change = 1 Commit:** Each commit must represent a single, atomic logical change. This makes code reviews easier, simplifies reverting changes, and improves the clarity of the project history.
*   **Each Commit States: Why + What + How Verified:**
    *   **Why:** Clearly articulate the motivation or the problem that the commit aims to solve.
    *   **What:** Describe the specific code changes implemented in this commit.
    *   **How Verified:** Detail the steps taken to verify the correctness of the change (e.g., "Unit tests for module X pass," "Manually tested on a Raspberry Pi 4 with Debian Bookworm," "Integration tests for Docker installation succeeded").

### 4. Branch Strategy

*   **Larry/Claude Push to Feature/Fix Branches:** Developers ("Larry/Claude") will work on dedicated feature branches (e.g., `feat/new-module`) or fix branches (e.g., `fix/bug-in-phase-x`). These branches are pushed to the remote repository.
*   **Moe Merges to `main` Only When Gates Pass:** I ("Moe") am responsible for reviewing pull requests from these branches. A pull request will only be merged into the `main` branch once all "Done" criteria have been successfully met and verified.

### 5. Test Matrix

A comprehensive test matrix is crucial for ensuring broad compatibility and stability:

*   **Platform/Kernel Specificity:**
    *   Identify which tests are specific to the Raspberry Pi kernel/hardware (e.g., `argon.rs` tests) and must be run on an actual ARM `aarch64` device or highly accurate emulator.
    *   Identify which tests can be run on a generic Linux host (x86_64 or aarch64) without specific hardware dependencies.
*   **Maelstrom Compatibility:**
    *   Document tests or functionalities that Maelstrom's sandbox cannot reliably execute due to `ENOSYS` errors (unsupported system calls) or other sandboxing limitations.
    *   For such cases, define and approve alternative testing methods (e.g., VM-based integration tests, manual test protocols).

### 6. Manual Smoke Tests

A series of essential manual smoke tests must be performed before any release:

*   **"Happy Path" Installation:**
    *   Execute a full, default installation on a clean target system.
    *   First, run in `dry-run` mode (if available) to verify all proposed actions.
    *   Then, perform a full installation and verify all expected components (packages, services, configurations) are correctly installed and functional.
*   **Distro Detection Path:** Verify that the installer correctly identifies the target Linux distribution and applies the appropriate `DistroDriver` logic for package management, service control, etc.
*   **Module Selection Paths:** Test various combinations of optional module installations (e.g., "minimal install," "install with Docker," "install with Argon One," "install with Zsh and Fonts").

### 7. Rollback Plan

A clear rollback strategy is essential for managing unexpected issues post-deployment:

*   **How to Revert a Broken Commit:** Provide instructions for developers on using `git revert <commit-hash>` to undo problematic commits, along with any necessary manual cleanup steps on target systems if the broken commit deployed changes.
*   **How to Tag Last-Known-Good:** Establish a process for tagging stable, functional commits in the `main` branch (e.g., `git tag -a vX.Y.Z -m "Release vX.Y.Z"`) to provide easily identifiable points for reverting to a stable state.

### 8. Release Checklist

Prior to any new release, the following checklist must be completed:

*   **Update Documentation in `/docs`:** Ensure all relevant documentation files within the `docs/` directory are up-to-date. This includes updating installation guides, architectural overviews, and explicitly noting any "Known limitations" (e.g., "Maelstrom on this kernel version requires X workaround").
*   **Version Bump:** Update the `Cargo.toml` file with the new version number, adhering to Semantic Versioning (e.g., `major.minor.patch`).
*   **Changelog Notes:** Add detailed release notes to `docs/CHANGELOG.md` (or equivalent), summarizing new features, bug fixes, breaking changes, and any upgrade considerations for users.