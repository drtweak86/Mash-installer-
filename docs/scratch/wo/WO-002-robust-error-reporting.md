
> **Neon Chronicle (Technical polish)**: WO-002-robust-error-reporting keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️

# Title
WO-002: Add Robust External Command Error Reporting

# Source
- PlanB.md - Section 3: "Fixes Needed for Program to Function as Intended"
- PlanA.md - Section 5: "Enhance Error Handling and Reporting"

# Goal
To improve debugging of failed installations by ensuring that whenever an external shell command fails, its `stdout` and `stderr` are captured and displayed to the user.

# Scope
- This change may affect multiple phase files in `installer-core/src/` where external commands are used (e.g., `pkg.rs`, `git.rs`, `docker.rs`).
- A new utility function for running commands should be created, perhaps in a new `installer-core/src/cmd.rs` module.

# Non-goals
- Do not implement a full logging framework. This WO is focused only on capturing output from failed commands.
- Do not change the overall error handling strategy (e.g., stopping on first error).
- Do not refactor the `InstallContext`.

# Steps
1.  Analyze the codebase to identify all places where `std::process::Command` is used to run external commands.
2.  Create a new utility function (e.g., `run_command_with_output`) that wraps `std::process::Command`.
3.  This function should:
    -   Execute the command.
    -   If the command succeeds, return `Ok(())`.
    -   If the command fails, capture its `exit code`, `stdout`, and `stderr`.
    -   Format a detailed error message including the captured output.
    -   Return an `anyhow::Result` containing this detailed error.
4.  Refactor all existing calls to `std::process::Command` to use the new utility function.
5.  Ensure the captured output is printed when a phase fails due to a command error.

# Success criteria
-   When an installation phase fails due to an external command (e.g., `apt-get install` fails because of a broken package), the error message printed to the console now includes the `stdout` and `stderr` from the failed command.
-   Phases that use successful commands are unaffected.

# Tests
-   No automated test framework currently exists.
-   **Verification must be performed manually:**
    1.  Identify a phase that runs an external command (e.g., `pkg::install_phase`).
    2.  Temporarily modify the command to be one that is guaranteed to fail (e.g., `apt-get install non-existent-package-xyz`).
    3.  Run the installer.
    4.  Verify that the installer fails at that phase and that the error output from `apt-get` is displayed in the terminal.
    5.  Revert the modification.

# Risk
Low. This change improves error reporting and is unlikely to affect the success path of the installation.

# Commit message
```
feat(errors): capture stdout/stderr from failed commands

Implements a new command execution utility to ensure that when any
external shell command fails during an installation phase, its stdout
and stderr are captured and included in the error message.

This provides crucial diagnostic information for debugging failed
installations, addressing a key item in the "Fixes Needed" section
of Plan B.

Verified by intentionally causing a package installation to fail and
confirming that the command's output was displayed in the final
error message.
```