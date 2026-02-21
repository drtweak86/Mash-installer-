# WO-002-introduce-formal-phase-trait
> **Neon Chronicle (Technical polish)**: WO-002-introduce-formal-phase-trait keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Introduce a formal `Phase` trait to provide a structured and extensible way to define installation phases in `installer-core`.

## Problem
Currently, installation phases are defined as simple functions (`fn(&InstallContext) -> Result<()>`), which lacks descriptive power and makes it harder to enforce a consistent structure across different phases. This informal definition limits extensibility and makes it less clear what capabilities a "phase" truly possesses.

## Proposal
- **Define `Phase` Trait:** Create a new `Phase` trait with the following methods:
    - `name()`: Returns a human-readable name for the phase (e.g., "System Pre-checks").
    - `description()`: Provides a brief explanation of what the phase does.
    - `execute(&mut PhaseContext)`: Contains the core logic of the installation phase, operating on a specific `PhaseContext` (which would be derived from the refactored `InstallContext` as per WO-001). This method should return a `Result<()>`.
    - `should_run(&AppContext)`: A method to determine if the phase should be executed based on the overall application context (e.g., user selections, platform detection).
- **Implement Trait for Phases:** Each existing and future installation phase will implement this `Phase` trait, thereby standardizing their interface and behavior.

## Rationale
Implementing a `Phase` trait significantly enhances the modularity and extensibility of the installer. It provides a clear contract for how phases should behave, making it easier to add new phases, reorder existing ones, and manage their lifecycle. This structured approach also improves code readability and maintainability, aligning with the overall goal of creating a more robust and understandable installer.