# WO-007-centralize-configuration-management
> **Neon Chronicle (Technical polish)**: WO-007-centralize-configuration-management keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Centralize the management of configuration within `mash-installer` by introducing a dedicated configuration service, thereby improving consistency, maintainability, and extensibility.

## Problem
While configuration might be loaded in a single location, its access and manipulation are currently spread throughout the `InstallContext` and potentially various modules. This decentralized approach can lead to:
- **Inconsistent Access Patterns:** Different parts of the code might access configuration values in varying ways.
- **Difficulty in Validation:** Ensuring all configuration values are valid and correctly typed becomes challenging.
- **Lack of Defaults Management:** Providing and overriding default values can be cumbersome.
- **Limited Extensibility:** Adding new configuration options or changing existing ones requires modifications in multiple places.

## Proposal
- **`ConfigService` or `ConfigRepository`:** Create a dedicated service or repository (e.g., `ConfigService` in `installer-core`) responsible solely for handling configuration.
- **Loading and Validation:** This service will manage:
    - Loading configuration from various sources (e.g., files, environment variables, command-line arguments).
    - Validating configuration values against defined schemas or rules.
    - Providing default values for missing configuration items.
- **Consistent Access:** All parts of the application needing configuration will access it exclusively through this `ConfigService`. This can be achieved by:
    - Passing the `ConfigService` instance to components that require it (dependency injection).
    - Potentially using a singleton pattern if appropriate for the application's architecture.
- **Layered Configuration:** Support layered configuration, allowing users to override default settings with their own preferences (e.g., system-wide defaults overridden by user-specific configuration, further overridden by command-line arguments).

## Rationale
Centralizing configuration management significantly enhances the maintainability and robustness of the installer. It provides a single, consistent point of truth for all configuration-related concerns, making it easier to audit, modify, and extend. This approach also aligns with reducing opinionated defaults by making it explicit how configurations are loaded and applied, offering clear avenues for users to customize behavior. It supports modularization by abstracting away the specifics of configuration handling from the core logic of installation phases.