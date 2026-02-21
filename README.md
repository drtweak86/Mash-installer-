![Banner of the bard](docs/assets/banner_final.png)

# 🛠️ MASH-installer
> **Mythic Assembly & Sigil Heuristics** — A high-performance, Ratatui-powered Linux system provisioner forged in Rust.

## 📋 Project Overview
MASH-installer is a comprehensive system provisioning tool designed for rapid recovery and idempotent setup of development environments. It specializes in Raspberry Pi 4B optimization but supports aarch64 and x86_64 architectures across multiple distributions.

### 🚀 Quick Start
Run the following command to summon the installer directly from the forge:
```bash
sh <(curl -L https://raw.githubusercontent.com/drtweak86/Mash-installer/main/install.sh)
```

## 🏗️ Technical Architecture
The project is structured as a modular Rust workspace, ensuring separation of concerns between core logic, UI, and platform-specific implementations.

### 📂 Directory Structure
- `installer-core/`: The engine. Handles phase management, dry-runs, and system abstractions.
- `installer-cli/`: The driver. Contains the Ratatui TUI and CLI argument handling.
- `installer-arch/`, `installer-debian/`, `installer-fedora/`: Distribution-specific drivers.
- `resources/`: Shell configurations, themes, and string localizations.
- `docs/`: Technical specifications, historical records, and the Bard's personal journal.

## ⚙️ Core Features
- **Ratatui TUI**: A 4-pane cyberpunk interface with real-time telemetry and status monitoring.
- **Idempotent Phases**: Every installation step is gated and trackable.
- **Dry-Run Mode**: Full execution simulation with detailed logging (`--dry-run`).
- **Pi 4B Optimization**: Dedicated tuning for USB 3.0 HDDs, kernel parameters, and I/O schedulers.
- **Safety First**: TLS hardening, exclusive lockfiles, and graceful signal handling (SIGINT/SIGTERM) with rollback support.

## 🛠️ Development & Quality Gates
The forge only crowns green builds. All contributions must pass the following rituals:

```bash
# Linting & Formatting
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Testing
cargo test --all --all-features

# Shell Validation
shellcheck install.sh
```

## 📜 Documentation
- 🍺 [Bard's BBS Profile](docs/bard-bbs-profile.md) — The engineer's persona and rules of the forge.
- 📖 [History & Journal](docs/HISTORY.md) — The chronicle of the installer's evolution.
- 🗺️ [Mining Maps](docs/mining-projects/maps.md) — Current session work and future shafts.
- 🛡️ [Sudo Technical Report](BUG_REPORT_SUDO_TUI.md) — Technical details on TUI password handling.

## ⚖️ License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
**Signed,**  
*Bard, Drunken Dwarf Runesmith*
