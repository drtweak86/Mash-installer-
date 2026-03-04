# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Commands

All commands run from the workspace root `/home/drtweak/Mash-installer`.

```bash
# Build
cargo build
cargo build --release

# Lint (must stay clean — CI enforces -D warnings)
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt --all

# Run all tests
cargo test --workspace

# Run a single test by name
cargo test <test_name> --workspace

# Run tests in a specific crate
cargo test -p installer-core

# xtask subcommands (dev tooling)
cargo xtask check-docs          # validate markdown links in docs/
cargo xtask bump patch|minor|major   # semver bump
cargo xtask release-check       # full pre-release gate (fmt + clippy + tests + docs)
cargo xtask hygiene             # move old scratch docs to legacy/
cargo xtask branch-prune        # prune local branches older than 7 days
cargo xtask test-infra          # run tests via maelstrom or cargo fallback
cargo xtask test-theme          # verify theme resources + module structure

# Run the binary
cargo run -p installer-cli -- [options]
cargo run -p installer-cli -- --dry-run --no-tui
cargo run -p installer-cli -- --bard         # easter egg
```

## Rust Toolchain

Pinned at **1.93.1** via `rust-toolchain.toml`. Components: `rustfmt`, `clippy`.

## Architecture

### Workspace Crates

| Crate | Role |
|-------|------|
| `installer-cli` | Binary (`mash-setup`): arg parsing, TUI (default) or stdio (`--no-tui`) |
| `installer-core` | Library: phases, orchestration, models, system, and wallpaper logic |
| `installer-drivers` | Consolidated distro drivers (Arch, Debian, Fedora) |
| `mash-system` | Foundational system abstractions and manual /proc parsing |
| `mash-wallpaper` | Standalone wallpaper harvesting library |
| `wallpaper-downloader` | Thin wrapper for standalone wallpaper downloads |
| `xtask` | Dev tooling (`cargo xtask <subcommand>`); not published |

### Two Execution Paths in `installer-cli`

1. **TUI path** (default): Ratatui 4-pane cyberpunk interface. Code lives in `installer-cli/src/tui/` (8 modules: `app`, `render`, `menus`, `theme`, `bbs`, `sysinfo_poller`, `observer`, `mod`).
2. **Stdio path** (`--no-tui` or `--non-interactive`): indicatif progress bars, menu-driven. See `ui_legacy.rs` and `CliPhaseObserver`.

### Key Patterns in `installer-core`

- **`PhaseContext::run_or_record()`** — the single dry-run gate. All side-effecting operations must pass through this. Never bypass it for new phase work.
- **`pub mod verify`** — exposes `verify_file_written()` and `sync_file()` for Pi SD card write safety. Use these after writing config files on Pi targets.
- **`WallpaperConfig::with_env_keys()`** — reads `MASH_WALLHAVEN_KEY`, `MASH_PEXELS_KEY`, `MASH_PIXABAY_KEY` from the environment. Never hardcode API keys.
- **`DistroDriver` trait** — implemented in `installer-drivers`; `installer-core` calls it via dynamic dispatch.

### `install.sh`

POSIX sh bootstrap that runs **before** Rust exists on the target. Must stay strictly POSIX-compliant (no Bash-isms). Downloads the prebuilt `mash-setup` binary for the detected arch (x86_64 or aarch64).

## CI Gates (all must stay green)

`cargo fmt` → `cargo clippy -D warnings` → `cargo test` → `cargo audit` → shellcheck on `install.sh` → distro matrix (Ubuntu/Fedora/Arch) → mdBook build

The `cargo xtask release-check` command runs the same gates locally before any release tag.

## Release Process

Uses `cargo-release`. The pre-release hook (`release.toml`) calls `cargo xtask release-check`. Tags follow `v{MAJOR}.{MINOR}.{PATCH}`. Post-1.0, backward compatibility is law — no breaking changes without a major version bump.

## docs/ Layout

- `docs/forge-tavern/` — four canonical reference files (immutable set): `bard-bbs-profile.md`, `bard-quick-ref.md`, `maps.md`, `maps-explored.md`
- `docs/src/` — mdBook source (28 chapters, deployed to GitHub Pages)
- `docs/scratch/` — temp files (<7 days; auto-moved to `docs/legacy/` by `cargo xtask hygiene`)
- `docs/HISTORY.md`, `docs/MANUAL.md` — permanent project docs

## Forge Laws (non-negotiable)

1. **ABB** — Always back up before destructive ops
2. **ABT** — fmt + clippy + tests must stay green
3. **ABD** — Update `maps.md` + `HISTORY.md` + README each session
4. **KCS** — Keep commits small and focused
5. **KISS** — No over-engineering; no scope creep
6. **SVR** — Semantic versioning, v-prefix tags, workspace-aligned
7. **1.0 Threshold** — Post-1.0: backward compat is law
