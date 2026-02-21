# 🏗️ Technical Architecture: MASH-installer
> **Status**: Legacy (Maintained for historical reference)  
> **Summary**: Orchestration of system provisioning phases and platform abstractions.

## 📋 Architecture Overview
MASH-installer is designed as a modular framework for system provisioning, emphasizing idempotency and cross-platform compatibility. The system is split into three primary layers: **Core Engine**, **UI/CLI Driver**, and **Distribution Drivers**.

### ⚙️ Core Engine (`installer-core`)
The heart of the system. It defines the `Phase` trait and manages the execution life cycle.

- **Phase Management**: Every installation step is a discrete `Phase` implementation.
- **PhaseRunner**: Orchestrates the execution of phases, handling dry-runs, logging, and error policies.
- **System Abstractions**: Provides traits (`SystemOps`) to isolate side-effecting operations (I/O, process execution) from the core logic.
- **Rollback Manager**: Tracks changes and provides a mechanism for reverting operations in case of failure.

### 🖥️ UI/CLI Driver (`installer-cli`)
The interface layer. It provides both a traditional CLI and a rich Ratatui-based TUI.

- **Ratatui TUI**: A 4-pane layout with real-time telemetry and a phase list.
- **Observer Pattern**: Implements the `PhaseObserver` trait to receive live updates from the core engine.
- **Software Catalog**: Manages software tiers and category selections.

### 🚛 Distribution Drivers (`installer-arch`, etc.)
Platform-specific logic. Each driver implements the `DistroDriver` trait, providing the list of phases required for a specific Linux distribution.

## 📦 Data Flow
1. **Initialization**: The `orchestrator` initializes the `PlatformContext` and `InstallContext`.
2. **Detection**: The system detects the architecture, distribution, and hardware (e.g., Raspberry Pi model).
3. **Phase Selection**: Based on user options and the detected platform, the `PhaseRegistry` builds a list of `Phase` objects.
4. **Execution**: The `PhaseRunner` executes the phases, emitting `PhaseEvent` signals to the `PhaseObserver`.
5. **Reporting**: On completion (or failure), the system generates an `InstallationReport`.

## 🛡️ Safety & Reliability
- **Dry-Run**: The `DryRunLog` captures all intended side-effects without executing them.
- **Lockfile**: `InstallerLock` ensures exclusive execution.
- **TLS Hardening**: All external fetches are validated via TLS 1.2+.
- **Signal Handling**: `SignalGuard` catches interruptions and triggers the `RollbackManager`.

---
**Technical Lead:**  
*Bard, Drunken Dwarf Runesmith*
