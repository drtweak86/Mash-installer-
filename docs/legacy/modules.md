# 📦 Module Directory: MASH-installer
> **Status**: Legacy (Maintained for historical reference)  
> **Summary**: Overview of the crate and module structure within the MASH-installer workspace.

## 📋 Workspace Structure
MASH-installer is organized as a Rust workspace to maintain clean separation between the core logic and platform-specific implementations.

### ⚙️ `installer-core`
The primary engine for system provisioning and phase orchestration.

- `apt_repo.rs`: Debian/Ubuntu repository management.
- `argon.rs`: Argon One hardware-specific scripts (Raspberry Pi).
- `backend.rs`: Trait for system operations (`SystemOps`).
- `buildroot.rs`: Buildroot dependency management.
- `cmd.rs`: Command execution and pipe handling.
- `config.rs`: Configuration file loading and parsing.
- `context.rs`: Contextual state management for the installer run.
- `distro.rs`: Distribution detection and driver registration.
- `docker.rs`: Docker Engine installation and configuration.
- `doctor.rs`: Pre-flight system health checks.
- `driver.rs`: Traits for distribution-specific drivers.
- `dry_run.rs`: Logic for simulating installation runs.
- `error.rs`: Centralized error handling and reporting.
- `interaction.rs`: Interface for user prompts and decisions.
- `lockfile.rs`: Exclusive execution management.
- `logging.rs`: Tracing and event logging setup.
- `options.rs`: User options parsing and management.
- `orchestrator.rs`: Main entry point for the core installation flow.
- `package_manager.rs`: Abstractions for `apt`, `pacman`, and `dnf`.
- `phase_registry.rs`: Registry of all available installation phases.
- `phase_runner.rs`: Orchestration of phase execution.
- `pi4b_hdd.rs`: Raspberry Pi 4B HDD-specific optimizations.
- `rollback.rs`: Management of undo operations.
- `signal.rs`: Handling of SIGINT/SIGTERM interruptions.
- `sudo.rs`: Management of sudo access and keepalive.
- `sudo_password.rs`: Secure memory storage for sudo credentials.
- `system.rs`: Low-level system abstractions.
- `theme.rs`: TUI theme definitions and palette management.
- `verify.rs`: Filesystem verification and sync logic.
- `zsh.rs`: Shell configuration (zsh, oh-my-zsh, starship).

### 🖥️ `installer-cli`
The CLI and Ratatui-based TUI implementation.

- `main.rs`: Application entry point.
- `menu.rs`: Traditional CLI menu implementation.
- `ui.rs`: Modern TUI rendering logic using Ratatui.
- `tui/`: Subdirectory for modular TUI components.
  - `app.rs`: State machine and event loop.
  - `render.rs`: Widget drawing and layout logic.
  - `observer.rs`: Integration with the core's `PhaseObserver`.

### 🚛 Platform Drivers
- `installer-arch`: Implementation for Arch Linux.
- `installer-debian`: Implementation for Debian-based systems.
- `installer-fedora`: Implementation for Fedora-based systems.

---
**Technical Lead:**  
*Bard, Drunken Dwarf Runesmith*
