# WO-010-improve-library-api-design
> **Neon Chronicle (Technical polish)**: WO-010-improve-library-api-design keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Redesign the public API of `installer-core` to be more programmatic and library-centric, ensuring it returns structured results and events rather than directly interacting with the console.

## Problem
As identified in WO-003, `installer-core` currently behaves more like an application than a reusable library, particularly through its direct use of `println!` and `indicatif` progress bars. The `run_with_driver` function, for instance, might simply return `Ok(())` on success, providing minimal structured information about the installation process or its outcomes. This makes it difficult for external consumers (like `installer-cli` or other potential frontends) to programmatically understand and react to the installation's progress and results.

## Proposal
- **Programmatic Return Values:** The primary API function of `installer-core` (e.g., `run_with_driver` or a new `Installer::run` method) should return a rich `Result` type. This `Result` should contain:
    - **Success State:** Detailed information about a successful installation (e.g., installed components, paths, configuration used).
    - **Error State:** A comprehensive error object (as per WO-005) detailing what went wrong, including specific phases that failed, error codes, and suggested remedies.
- **Event-Driven Interface:** Augment the API with an event-driven mechanism where `installer-core` emits events during its execution (e.g., `PhaseStarted`, `PhaseCompleted`, `ProgressUpdate`). Consumers can then subscribe to these events to get real-time updates and react accordingly. This builds upon the UI decoupling proposed in WO-003.
- **Clear Inputs:** The API should clearly define its required inputs, such as configuration objects (from WO-007) and abstract system operation services (from WO-008), making its dependencies explicit.
- **Minimal Side Effects:** `installer-core` functions should minimize direct side effects (like printing to stdout/stderr) and instead communicate state changes and results through return values and events.

## Rationale
Improving the library API design is crucial for making `installer-core` a robust, reusable, and maintainable component. By providing structured outputs and an event-driven interface, it empowers consumers to build diverse frontends and integrate the installer into broader automation workflows without being constrained by an opinionated UI. This aligns perfectly with modularization goals, making `installer-core` a true, flexible library rather than an embedded application. It also reinforces the principle of separation of concerns, ensuring that the core installation logic remains independent of presentation or reporting mechanisms.