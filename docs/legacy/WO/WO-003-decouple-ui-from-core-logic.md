# WO-003-decouple-ui-from-core-logic
> **Neon Chronicle (Technical polish)**: WO-003-decouple-ui-from-core-logic keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Decouple the User Interface (UI) logic, specifically progress reporting and console output, from the core installation logic within `installer-core`.

## Problem
The `run_with_driver` function in `installer-core` currently handles both the orchestration of the installation process and the management of UI elements, such as `indicatif` progress bars and direct `println!` statements. This violates the principle of separation of concerns, making `installer-core` less reusable as a library and complicating testing.

## Proposal
- **UI Code Relocation:** Move all UI-related code, including progress bar updates and direct console output (`println!`), out of `installer-core` and into `installer-cli`.
- **Event-Driven Communication:** `installer-core` should be modified to emit events or return structured results that describe the progress and outcomes of the installation phases. Examples of such events could include:
    - `PhaseStarted(PhaseName)`
    - `PhaseCompleted(PhaseName, ResultDetails)`
    - `PhaseFailed(PhaseName, ErrorDetails)`
- **CLI as UI Renderer:** `installer-cli` will be responsible for subscribing to these events or interpreting the results from `installer-core` and rendering the appropriate UI feedback to the user (e.g., updating progress bars, displaying success/failure messages).

## Rationale
This decoupling significantly enhances the modularity of the `mash-installer`. It transforms `installer-core` into a true, reusable library that can be integrated into various frontends (e.g., a GUI, a web interface, or other CLI tools) without carrying UI baggage. This also simplifies testing of the core logic, as UI concerns do not need to be mocked or managed. It aligns with the goal of making the installer more flexible and less opinionated about its presentation layer.