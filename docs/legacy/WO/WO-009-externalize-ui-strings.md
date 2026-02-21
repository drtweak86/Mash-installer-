# WO-009-externalize-ui-strings
> **Neon Chronicle (Technical polish)**: WO-009-externalize-ui-strings keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Externalize all user-facing strings (e.g., phase labels, descriptive messages, error prompts) from the Rust source code into a separate, centralized configuration or resource file.

## Problem
Currently, all UI strings are hardcoded directly within the Rust source files of `mash-installer`. This approach presents several issues:
- **Difficult to Change:** Modifying even a minor piece of text requires recompiling the entire application.
- **Maintenance Overhead:** Translating the application into other languages (internationalization, i18n) becomes a monumental task, as strings are scattered throughout the codebase.
- **Developer Focus Shift:** Developers might inadvertently alter user-facing text during code changes, potentially introducing inconsistencies or grammatical errors.

## Proposal
- **Resource File Format:** Choose a suitable format for externalizing strings, such as TOML, JSON, YAML, or a dedicated `.ini`-like file. TOML is often a good choice for Rust projects due to its native support and readability.
- **Centralized Storage:** Create a dedicated directory (e.g., `resources/strings/`) within the project to store these string files.
- **String Loading Mechanism:** Implement a mechanism within `installer-core` (or a new dedicated `installer-localization` crate) to load these strings at runtime. This mechanism should:
    - Load the default set of strings.
    - Potentially allow for loading locale-specific string files based on user configuration or system settings.
- **Access API:** Provide a simple and consistent API (e.g., a function like `get_string("key_name")`) for retrieving strings throughout the application.
- **Placeholder Management:** For strings requiring dynamic content (e.g., "Installation failed for {phase_name}"), implement a templating or placeholder replacement system.

## Rationale
Externalizing UI strings is a critical step towards improving the maintainability, flexibility, and global reach of the `mash-installer`. It decouples content from code, allowing for easier updates to text without code changes, and lays essential groundwork for future internationalization efforts. This aligns with reducing opinionated defaults by allowing the text presented to the user to be fully configurable and adaptable to different linguistic or regional requirements, without altering the core logic. It also supports modularization by separating presentational concerns from functional concerns.