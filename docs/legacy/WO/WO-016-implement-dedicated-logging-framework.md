# WO-016-implement-dedicated-logging-framework
> **Neon Chronicle (Technical polish)**: WO-016-implement-dedicated-logging-framework keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Integrate a dedicated and robust logging framework into `mash-installer` to provide detailed, structured, and configurable logs for every installation run, significantly aiding in debugging, support, and post-mortem analysis.

## Problem
While `tracing::info!` and `error!` are used, relying solely on basic logging macros might not provide the depth, structure, or configurability required for a production-grade installer. Key issues include:
- **Limited Detail:** Generic log messages might lack sufficient context for complex issues.
- **Unstructured Logs:** Difficult to parse and analyze automatically, especially across many runs.
- **Lack of Centralized Configuration:** How logs are collected, filtered, and outputted (e.g., to console, file, syslog) might not be easily configurable.
- **Performance Overhead:** Inefficient logging can impact installer performance.

## Proposal
- **Choose a Robust Framework:** Select a suitable Rust logging framework. Given the current use of `tracing`, extending its capabilities with `tracing-subscriber` for more advanced sinks (file appenders, structured output) is a natural progression. Alternatives like `log4rs` could also be considered for more traditional log configuration.
- **Structured Logging:** Implement structured logging to include key-value pairs (e.g., `phase="SystemPackages"`, `command="apt install"`, `exit_code=1`) with log messages. This makes logs easily parsable by tools and more informative.
- **Configurable Sinks:** Allow users (via WO-007: Centralize Configuration Management) to configure:
    - **Log Levels:** `debug`, `info`, `warn`, `error`.
    - **Output Destinations:** Console, file (with rotation), potentially syslog or other external services.
    - **Format:** Human-readable or machine-readable (e.g., JSON).
- **Contextual Logging:** Integrate logging with the `InstallContext` (or its refactored components) to automatically include relevant context with each log message (e.g., current phase name, operating system, user options).
- **Error Logging Enhancement:** Ensure that detailed error information, including `stdout`/`stderr` from failed commands (WO-011), is comprehensively logged.

## Rationale
A dedicated logging framework is indispensable for the long-term maintainability, debuggability, and supportability of the `mash-installer`. It transforms raw output into a valuable diagnostic asset, allowing developers and support personnel to quickly understand the state of the system at any point during installation. This enhancement significantly reduces the "opinionated" nature of logging by providing extensive configuration options to the user, making it adaptable to various operational environments and debugging needs. It also supports modularization by centralizing logging concerns rather than scattering print statements throughout the codebase.