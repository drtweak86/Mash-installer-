# 🎮 installer-cli
**The Cockpit**

The Ratatui-powered terminal interface for the MASH Installer. This crate provides the interactive menus, the telemetry panels, and the BBS-style feedback system that brings the forge to life.

### Key Components:
- **`ui/`**: The multi-pane Ratatui interface logic.
- **`menu.rs`**: Hierarchical selection logic for software and modes.
- **`software_catalog.rs`**: CLI-side mapping of the Software Grimoire.
- **`tui/`**: Specialized widgets for the 1984 Station aesthetic.
