# WO-001-refactor-installcontext
> **Neon Chronicle (Technical polish)**: WO-001-refactor-installcontext keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Refactor the `InstallContext` god object in `installer-core` to improve modularity, testability, and maintainability.

## Problem
The `InstallContext` struct currently holds all state and is passed to every installation phase. This design choice creates tight coupling between components and hinders the ability to test individual phases in isolation effectively.

## Proposal
- **Decomposition:** Break down the monolithic `InstallContext` into smaller, more focused context structs. Examples include:
    - `PlatformContext`: Encapsulating platform-specific details.
    - `UserOptionsContext`: Holding user-defined configuration and choices.
    - `UIContext`: Managing user interface-related states and interactions.
- **Dependency Minimization:** Ensure that each phase function only receives the specific context (or contexts) it absolutely requires to perform its operation. This reduces unnecessary dependencies.
- **Dependency Injection:** Implement a dependency injection pattern to provide the necessary context objects to the phases. This will allow for easier mocking and testing of individual phases.

## Rationale
This refactoring directly addresses the modularisation goals by promoting a more granular and focused approach to state management. It enhances testability by making it easier to isolate components, and improves maintainability by reducing the complexity of the `InstallContext` and its widespread usage.