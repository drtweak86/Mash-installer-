
> **Neon Chronicle (Technical polish)**: WO-004-abstract-system-ops keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️

# Title
WO-004: Abstract System Operations (Shell Commands)

# Source
- PlanA.md - Section 8: "Abstract System Operations"

# Goal
To create a centralized utility for running shell commands, improving code reuse and laying the groundwork for better testing, logging, and `dry-run` support. This is the first step in abstracting all system operations.

# Scope
- A new file: `installer-core/src/cmd.rs`.
- All files in `installer-core/src/` that currently use `std::process::Command`.

# Non-goals
- Do not abstract file system or network operations in this WO.
- Do not implement a full logging or `dry-run` framework yet. This WO is about creating the abstraction, not implementing all features on top of it.
- This will *not* replace all command execution, but will create the foundation. The work from WO-002 can be leveraged here.

# Steps
1.  Create a new module `installer-core/src/cmd.rs`.
2.  In this module, define a `Command` struct that wraps `std::process::Command`.
3.  Implement methods on this struct to fluently build a command (e.g., `arg()`, `args()`, `current_dir()`).
4.  Implement an `execute()` method that runs the command. This method should incorporate the error reporting logic from `WO-002`, capturing and returning `stdout`/`stderr` on failure.
5.  Refactor *one* existing phase (e.g., `zsh.rs` or `fonts.rs`) to use the new `cmd` module for its shell command operations instead of `std::process::Command` directly.
6.  Ensure the `InstallContext` is not passed to the new `cmd` module to maintain separation of concerns.

# Success criteria
-   A new `cmd.rs` module exists and provides a fluent interface for running shell commands.
-   At least one installation phase has been successfully refactored to use the new module.
-   The installer's behavior for the refactored phase remains unchanged on a successful run.
-   Error reporting for the refactored phase (if a command fails) is still robust.

# Tests
-   No automated test framework currently exists.
-   **Verification must be performed manually:**
    1.  Run the installer and ensure the refactored phase (e.g., zsh installation) completes successfully.
    2.  Temporarily modify a command in the refactored phase to cause a failure.
    3.  Run the installer again and verify that the failure is caught and reported correctly, including command output.

# Risk
Medium. This is a mechanical refactoring, but it touches the core execution of external commands. Incorrect implementation could have wide-ranging effects. Focusing the refactoring on a single phase initially minimizes this risk.

# Commit message
```
refactor(cmd): abstract shell command execution

Introduces a new `cmd` module to provide a centralized, fluent
interface for running external shell commands. This is the first step
in abstracting system operations, as outlined in Plan A.

The new module wraps `std::process::Command` and integrates the robust
error reporting previously developed. The `zsh` installation phase has
been refactored to use this new utility as an initial adoption.

This will improve code reuse and makes it easier to add features like
centralized logging or enhanced dry-run support in the future.

Verified by running the installer and confirming the refactored `zsh`
phase functions correctly and still reports errors as expected.
```