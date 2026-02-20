# WO-005-enhance-error-handling-and-reporting
> **Neon Chronicle (Technical polish)**: WO-005-enhance-error-handling-and-reporting keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Implement a more robust error handling strategy for `mash-installer` that allows for continued execution after non-fatal errors and provides detailed, actionable error reports.

## Problem
The current installer design typically halts execution upon encountering the first error. This "fail-fast" approach can be problematic for users, as it prevents the discovery of multiple issues in a single run and offers limited diagnostic information when an error occurs. The generic nature of current error messages may also hinder troubleshooting.

## Proposal
- **Graceful Error Recovery:** Modify the `PhaseRunner` (as introduced in WO-004) to allow for configurable error handling. For certain types of errors, or if specified by a user option, the installer should attempt to continue with subsequent phases rather than stopping immediately.
- **Dedicated Error Type:** Create a custom, dedicated error type (e.g., `InstallerError`) for the `installer-core` crate. This error type should be able to encapsulate:
    - **Contextual Information:** The phase where the error occurred, relevant input parameters, and the state of the system at the time of the error.
    - **User-Friendly Messages:** Clear, concise messages intended for the end-user.
    - **Developer-Focused Details:** More technical details, stack traces, or internal error codes for debugging.
    - **Actionable Advice:** Suggestions for how the user might resolve the issue or where to find more information (e.g., "Check network connectivity," "Ensure required packages are installed").
- **Aggregated Error Reporting:** If execution continues after errors, the `PhaseRunner` should collect all encountered errors throughout the installation process. At the end of the run, a comprehensive summary report of all failures should be presented to the user.
- **Reporting Mechanism:** Integrate this enhanced error reporting with the UI decoupling (WO-003), allowing the `installer-cli` to present these errors in a well-formatted and helpful manner.

## Rationale
This enhancement directly improves the user experience by providing more informative feedback and potentially allowing for partial installations or the identification of multiple issues in one go. It also significantly aids in debugging and maintenance by providing developers with richer error contexts. By making error messages more actionable and less opinionated in their presentation (allowing the CLI to format), it reduces frustration for both end-users and developers, aligning with the goal of a more user-friendly and maintainable installer.