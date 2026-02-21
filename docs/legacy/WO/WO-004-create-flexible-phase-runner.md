# WO-004-create-flexible-phase-runner
> **Neon Chronicle (Technical polish)**: WO-004-create-flexible-phase-runner keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Design and implement a flexible `PhaseRunner` struct in `installer-core` to manage the execution, error handling, and results collection of installation phases.

## Problem
The current phase execution logic is tightly coupled within the `run_with_driver` function, typically implemented as a hardcoded `for` loop. This rigid structure makes it difficult to dynamically alter the installation workflow, introduce conditional phase execution, or implement advanced error recovery strategies without modifying the core orchestration logic.

## Proposal
- **`PhaseRunner` Struct:** Create a dedicated `PhaseRunner` struct. This struct will encapsulate the logic for iterating through a collection of `Phase` objects (as defined in WO-002).
- **Execution Logic:** The `PhaseRunner` will be responsible for:
    - Invoking each `Phase`'s `execute` method.
    - Handling transitions between phases.
    - Implementing logic for `should_run` checks (from WO-002) to conditionally skip phases.
- **Error Management:** Integrate robust error handling within the `PhaseRunner`. This could involve:
    - Catching errors from individual phases.
    - Deciding whether to stop execution or attempt to continue (as per WO-005).
    - Collecting and summarizing all encountered errors.
- **Result Collection:** The `PhaseRunner` will collect results or events from each phase, which can then be passed back to the UI layer (as per WO-003) for reporting.

## Rationale
Introducing a `PhaseRunner` significantly enhances the flexibility and maintainability of the installation process. It centralizes the control flow, making the installation process more declarative and easier to understand. This abstraction allows for easier modification of the phase execution order, dynamic inclusion/exclusion of phases, and more sophisticated error recovery mechanisms, without cluttering the main `run_with_driver` function. This supports the modularization goals by creating a clear responsibility for phase orchestration.