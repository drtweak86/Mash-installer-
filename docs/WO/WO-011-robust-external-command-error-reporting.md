# WO-011-robust-external-command-error-reporting
> **Neon Chronicle (Technical polish)**: WO-011-robust-external-command-error-reporting keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Implement robust error reporting for external shell commands executed by the `mash-installer`, ensuring comprehensive capture and logging of `stdout` and `stderr` upon failure.

## Problem
Currently, when an external shell command executed by the installer fails, the diagnostic information provided is often insufficient. This lack of detail, especially the absence of captured `stdout` and `stderr` streams, makes it extremely difficult to pinpoint the exact cause of a command failure, leading to extended debugging times and a poor user experience during troubleshooting.

## Proposal
- **Centralized Command Execution:** Enhance the `ShellCommandExecutor` (as proposed in WO-008: Abstract System Operations) to automatically capture and store the `stdout` and `stderr` of every executed command, regardless of success or failure.
- **Error Context Enrichment:** When a command returns a non-zero exit code (indicating failure), incorporate the captured `stdout`, `stderr`, and the exit code directly into the error message or the dedicated `InstallerError` type (from WO-005: Enhance Error Handling and Reporting).
- **Logging Integration:** Ensure that these detailed command execution failures, including `stdout` and `stderr`, are properly integrated with the logging framework (as proposed in WO-014: Implement Dedicated Logging Framework) to provide comprehensive logs for post-mortem analysis.
- **User-Friendly Presentation:** When presenting command failures to the user via the `installer-cli` (as per WO-003: Decouple UI from Core Logic), format the captured output in a clear and readable manner, highlighting relevant error messages from the external command.

## Rationale
This work order directly addresses a critical need for improved diagnostics and error handling, making the installer significantly more user-friendly and maintainable. By providing rich, actionable context for external command failures, developers can quickly identify and fix issues, and users can better understand why an installation failed. This enhancement aligns with the goal of providing more informative error reporting and contributes to the overall robustness of the installer. It also refines the capabilities of the abstracted system operations, making that abstraction more valuable.