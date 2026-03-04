# ⚒️ MASH INSTALLER: THE RUNESMITH'S GRIMOIRE (GEMINI.md)

Welcome, traveler, to the **MASH (Mythic Assembly & Sigil Heuristics)** forge. You are now assisting the **Bard**, a Drunken Dwarf Runesmith, in maintaining a high-performance, Ratatui-powered Linux system provisioner forged in Rust.

## 📋 PROJECT OVERVIEW
MASH-installer is a modular, idempotent system provisioning tool designed for rapid recovery and setup of development environments. While optimized for Raspberry Pi 4B, it supports `aarch64` and `x86_64` across Arch, Debian, and Fedora distributions.

- **Primary Language**: Rust (pinned at **1.93.1** via `rust-toolchain.toml`)
- **UI Architecture**: Ratatui TUI with a 4-pane cyberpunk aesthetic.
- **Core Principles**: Idempotency, dry-run safety, and distribution-agnostic orchestration.

## 🏗️ WORKSPACE STRUCTURE
The forge is divided into specialized chambers (crates):

| Crate | Role |
| :--- | :--- |
| `installer-cli` | The Front Gate. Handles arg parsing and the TUI/CLI interface. |
| `installer-core` | The Great Engine. Orchestrates phases, dry-runs, and system abstractions. |
| `installer-model` | The Blueprints. Shared data models (phases, events, user context). |
| `mash-system` | The Foundation. Shared system error types and reporting logic. |
| `installer-<distro>` | The Specialized Smiths. Distro-specific drivers (Arch, Debian, Fedora). |
| `xtask` | The Forge Tools. Custom dev automation (`cargo xtask`). |
| `wallpaper-downloader` | The Scavenger. Standalone tool for harvesting aesthetics. |

## 🛠️ THE RITUALS (BUILD & RUN)
All commands must be performed from the root of the forge.

### Building & Running
```bash
# Summon the binary
cargo build --release
# Run with TUI (default)
cargo run -p installer-cli
# Run safely without the screen
cargo run -p installer-cli -- --dry-run --no-tui
```

### Quality Assurance (The Holy Trinity)
The forge only crowns green builds. Every change must pass these rituals:
```bash
# 1. The Polishing (Format)
cargo fmt --all
# 2. The Inspection (Lint)
cargo clippy --all-targets --all-features -- -D warnings
# 3. The Stress Test (Test)
cargo test --workspace
```

### Specialized Forge Tools (`xtask`)
```bash
cargo xtask release-check  # Full pre-release gate (fmt + clippy + tests + docs)
cargo xtask hygiene        # Move old scratch docs (>7 days) to legacy/
cargo xtask bump patch     # Advance the version runes
cargo xtask check-docs     # Find broken links in the library
```

## 📜 LAWS OF THE FORGE (CONVENTIONS)
Adhere strictly to these dwarven mandates:

1.  **ABB (Always Be Backing up)**: Commits are save points. Never perform destructive ops without a backup.
2.  **ABT (Always Be Testing)**: `fmt`, `clippy`, and `tests` must stay green workspace-wide.
3.  **ABD (Always Be Documenting)**: Update `maps.md`, `HISTORY.md`, and README each session.
4.  **KCS (Keep Commits Small)**: One feature, one commit. No "and also" commits.
5.  **KISS (Keep It Simple Stupid)**: Practical solutions over architectural dogma. No scope creep.
6.  **SVR (Semantic Versioning Rule)**: `v-prefix` tags. `v1.0.0` is a sacred stability contract.
7.  **Dry-Run Sovereignty**: Side-effects MUST pass through `PhaseContext::run_or_record()`. Never bypass the dry-run gate.

## 🗺️ NAVIGATING THE ARCHIVES
- `docs/forge-tavern/`: The Four Sources of Truth (`maps.md`, `maps-explored.md`, `bard-bbs-profile.md`, `bard-quick-ref.md`).
- `docs/mining-projects/`: Detailed "Shafts" (tasks) and "Excavation Tasks" (steps).
- `docs/src/`: The `mdBook` source for the user manual.

*"May your builds be green, your tests be comprehensive, and your tankard never empty."*
— **Bard, Drunken Dwarf Runesmith**
