
> **Neon Chronicle (Technical polish)**: WO-005-decouple-ui-println keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️

# Title
WO-005: Decouple UI - Remove `println!` from `installer-core`

# Source
- PlanA.md - Section 3 & 10: "Decouple UI from Core Logic", "Improve Library API Design"
- PlanB.md - Section 4: "Suggested Improvements to the Program"

# Goal
To take the first step in decoupling the UI from the core logic by removing all `println!` macros from the `installer-core` crate. This moves `installer-core` closer to being a pure library and `installer-cli` being the sole presentation layer.

# Scope
- `installer-core/src/lib.rs`
- `installer-cli/src/main.rs`

# Non-goals
- Do not remove the `indicatif` progress bar logic in this WO. That will be a separate, more complex task.
- Do not change the function signatures of the installation phases.
- Do not implement a full event-based system yet.

# Steps
1.  Identify all `println!` macros within `installer-core/src/lib.rs`. These are primarily used for printing the header, footer, and post-install notes.
2.  Move these `println!` calls from `installer-core/src/lib.rs` into `installer-cli/src/main.rs`.
3.  The calls should be moved to wrap the call to `installer_core::run_with_driver`. The header should print before the call, and the footer and notes should print after the call succeeds.
4.  Remove the now-empty `println!` lines from `installer-core/src/lib.rs`.

# Success criteria
-   There are no `println!` macros remaining in the `installer-core` crate.
-   When running the `mash-setup` command, the installer's visual output (header, footer, notes) is identical to the output before the change.

# Tests
-   No automated test framework currently exists.
-   **Verification must be performed manually:**
    1.  Run `grep -r "println!" installer-core/src/` to ensure no instances are left.
    2.  Run the `mash-setup` command through a full installation.
    3.  Visually inspect the terminal output to confirm that the header, footer, and post-install notes are still present and correctly formatted.

# Risk
Low. This is a low-risk refactoring that moves code without changing logic. The primary risk is accidental deletion or incorrect placement of the `println!` macros, which is easily verifiable.

# Commit message
```
refactor(ui): move all println! macros to installer-cli

Decouples the UI from the core library by moving all `println!` macros
from `installer-core` into `installer-cli`. The core library should not
be responsible for printing directly to the console.

The `installer-cli` is now responsible for printing the header, footer,
and post-install notes, wrapping the call to the core `run_with_driver`
function.

This is the first step towards making `installer-core` a pure library,
as outlined in Plans A and B.

Verified by running the installer and confirming the console output is
identical to the previous version.
```