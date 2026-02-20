# WO-015-define-user-interaction-points
> **Neon Chronicle (Technical polish)**: WO-015-define-user-interaction-points keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Explicitly define and centralize the handling of all user interaction points within `mash-installer`, ensuring consistency, clarity, and graceful management across interactive and non-interactive modes.

## Problem
Currently, user interactions such as prompts for confirmation, password input, or selection choices might be scattered throughout various installation phases. This decentralized approach can lead to:
- **Inconsistent UX:** Different phases might present prompts in varying styles or with inconsistent messaging.
- **Difficulty in Automation:** Non-interactive modes (e.g., for scripting or CI/CD environments) become challenging to manage if interaction points are not clearly defined and bypassable.
- **Testing Complexity:** Automating tests that involve user input becomes more complex.
- **Security Concerns:** Sensitive information like passwords might be handled inconsistently or insecurely.

## Proposal
- **Centralized Interaction Service:** Introduce a dedicated `InteractionService` (or similar) within `installer-core` that acts as the sole gateway for all user input and output that requires direct user attention. This service would offer methods like:
    - `confirm(prompt: &str) -> bool`
    - `get_text_input(prompt: &str, sensitive: bool) -> String`
    - `select_option(prompt: &str, options: &[&str]) -> usize`
- **Interactive vs. Non-Interactive Modes:** The `InteractionService` will implement logic to differentiate between interactive and non-interactive modes:
    - **Interactive:** Present prompts to the user and await input.
    - **Non-Interactive:** Fallback to defaults, read from pre-configured answers (e.g., from a configuration file, environment variables), or return an error if interaction is mandatory and cannot be automated.
- **Integration with Configuration:** Allow default answers for interaction points to be specified via the centralized configuration management (WO-007), making it easier to script the installer.
- **UI Decoupling:** The `InteractionService` will abstract *what* to ask from *how* to ask it. The actual rendering of prompts and reading of input will still be handled by `installer-cli` (as per WO-003) or other frontends, based on events or calls from the `InteractionService`.

## Rationale
Centralizing and explicitly defining user interaction points significantly enhances the usability, maintainability, and automation capabilities of the `mash-installer`. It guarantees a consistent user experience, simplifies non-interactive operations, and makes the installer more testable. This aligns with reducing opinionated defaults by providing clear mechanisms for users and automation scripts to control the installer's behavior, rather than being forced into a specific interaction flow. It is a key step towards making the installer more flexible and adaptable to diverse deployment scenarios.