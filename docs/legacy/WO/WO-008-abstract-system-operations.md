# WO-008-abstract-system-operations
> **Neon Chronicle (Technical polish)**: WO-008-abstract-system-operations keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Abstract common system operations (e.g., file system interactions, shell command execution, network requests) into dedicated utility modules or services within `installer-core` to promote code reuse, testability, and enable features like dry-runs or logging.

## Problem
Across different installation phases and modules, there is likely duplicated or inconsistently implemented logic for interacting with the underlying system. This direct and scattered invocation of low-level APIs for tasks like running shell commands, reading/writing files, or performing network operations leads to:
- **Code Duplication:** Reinvention of the wheel for common tasks.
- **Poor Testability:** Difficult to test code that directly calls system APIs without side effects.
- **Limited Extensibility:** Hard to introduce cross-cutting concerns like logging, permission checks, or dry-run modes without modifying numerous call sites.

## Proposal
- **Utility Modules/Services:** Create a set of specialized utility modules or services within `installer-core` (or a dedicated `installer-utils` crate if appropriate) for common system interactions:
    - `FileSystem`: Abstracting file creation, reading, writing, deletion, directory operations, etc.
    - `ShellCommandExecutor`: Providing a consistent and robust way to execute external commands, handle their output, and check exit codes.
    - `NetworkClient` (or `Downloader`): Centralizing HTTP requests, downloads, and network-related checks.
- **Interface-Based Abstraction:** Define traits for these services where appropriate, allowing for different implementations (e.g., a "real" implementation for live runs and a "mock" implementation for testing or dry-runs).
- **Dependency Injection:** Inject these abstract services into the installation phases and modules that require them, rather than having them directly call low-level system APIs.

## Rationale
Abstracting system operations is a crucial step towards a more modular, testable, and maintainable installer. It eliminates code duplication, makes it significantly easier to unit test components that interact with the system (by injecting mock implementations), and provides a central point to introduce cross-cutting concerns. For instance, a "dry-run" mode could be implemented by providing alternative service implementations that log intended actions without performing them. This approach also reduces opinionated defaults by allowing different underlying implementations for system operations, such as choosing between `curl` or `wget` for downloads via configuration.