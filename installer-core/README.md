# 📦 installer-core
**The Engine Room**

This crate houses the core logic for the MASH Installer. It defines the `Phase` trait, the `PhaseRunner`, and the `PhaseRegistry`. All side effects are routed through the `SystemOps` trait to ensure dry-run capability and cross-distro support.

### Key Components:
- **`Phase`**: The atomic unit of work (e.g., `RustPhase`, `ZshPhase`).
- **`SystemOps`**: Trait-bound system operations (FS, Pkg, Cmd) that gate all side effects.
- **`InstallContext`**: The shared state of the forge, containing options and detected platform info.
- **`Advice`**: The engine that translates system profiles into actionable wisdom.
