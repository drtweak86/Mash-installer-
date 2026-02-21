# Canonical Module Inventory
> **Neon Chronicle (Whimsical polish)**: modules invites the bored bard to braid cyberpunk sigils with Tolkien smoke, so the instructions sing with neon and dice. 🌌🎲

This document is a straight extraction of what `mash-setup install` currently does. Every installed package, script, and feature below is enumerated with its purpose, the distros that the distro drivers push it to, and whether it is implicit (profile/phase driven) or surfaced via a modal toggle. No behavior is changed yet; the goal is documentation-only inventory.

## UI Layer (installer-cli)

As of 2026-02-21, the **Ratatui TUI** is the primary UI layer. The legacy `indicatif`/stdio
path remains available via `--no-tui` and is used automatically for non-interactive mode.

### TUI modules (`installer-cli/src/tui/`)

| Module | Role |
|--------|------|
| `mod.rs` | Module root, re-exports `run()` |
| `app.rs` | `TuiApp` state machine, `Screen` enum, `TuiMessage` bus, event loop |
| `render.rs` | 4-pane draw pipeline (Main, ActionLog, SysStats, BBS strip) |
| `menus.rs` | Welcome / DistroSelect / ModuleSelect / ProfileSelect / Confirm screens |
| `theme.rs` | Cyberpunk palette (cyan, magenta, matrix-green, red, gold) |
| `bbs.rs` | 44-entry BBS message bank + 4-second cycler thread |
| `sysinfo_poller.rs` | CPU%/RAM via `sysinfo 0.33`; NET/IO from `/proc`; 1-second poll |
| `observer.rs` | `RatatuiPhaseObserver` — sends `PhaseEvent` → `TuiMessage` via mpsc |

## Profiles

### Minimal profile (core system packages)
- **ca-certificates** – supplies TLS trust anchors so curl/wget/gh can verify HTTPS endpoints. Distros: Debian/Ubuntu (apt), Arch/Manjaro (pacman), Fedora (pacman); Visibility: implicit (minimal profile).  
- **curl** – downloads artefacts from GitHub, starship, Argon installs, etc. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **wget** – serves as a conservative download tool for scripts that expect it. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **xz-utils** – unpacks `.xz` archives from releases. Distros: Debian/Ubuntu, Arch/Manjaro (installs `xz`), Fedora; Visibility: implicit.  
- **tar** – unpacks tarballs as part of staging. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **coreutils** – provides basic Unix commands the installer relies on. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **jq** – lightweight JSON parser used by config helpers. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **git** – version control needed for local clones that other modules perform. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **software-properties-common** – provides `add-apt-repository` for Debian/Ubuntu before adding extra repos. Distros: Debian/Ubuntu only (Arch/Fedora drivers translate it to `None`); Visibility: implicit.  
- **gnupg** – imports GPG keys for Docker/GitHub CLI repos. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **lsb-release** – reads distro codenames for apt repo lines. Distros: Debian/Ubuntu only; Visibility: implicit.  
- **apt-transport-https** – enables HTTPS-based sources on Debian/Ubuntu. Distros: Debian/Ubuntu only; Visibility: implicit.  
- **build-essential** – installs GCC/make tooling on Debian/Ubuntu; Arch/Manjaro swaps it for `base-devel`, Fedora installs the canonical package. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **pkg-config** – required by native builds; Arch/Manjaro installs `pkgconf`, Fedora uses the canonical name. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **clang** – LLVM C compiler needed by some builds. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **lld** – LLVM linker that the toolchain assumes exists. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **cmake** – cross-platform build system generator. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **ninja-build** – high‑speed build backend; Arch/Manjaro installs `ninja`, Fedora keeps `ninja-build`. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **gcc** – GNU compiler required by countless build scripts. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **g++** – GNU C++ compiler; Arch/Manjaro drops this entry (the driver returns `None`), while Debian/Ubuntu and Fedora install it by name. Distros: Debian/Ubuntu, Fedora; Visibility: implicit.  
- **gdb** – debugger for native debugging. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **make** – traditional build orchestrator. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  

### Developer profile additions (Dev & Full profiles)
- **python3** – Python interpreter required by scripts such as Argon One and buildroot helpers; Arch/Manjaro maps this to `python`, Fedora keeps `python3`. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **python3-pip** – `pip` installer for Python toolchains (`python-pip` on Arch/Manjaro); Fedora uses the canonical name. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **python3-venv** – `venv` support for local virtual environments; Arch/Manjaro skips it entirely, Fedora leaves the canonical name unchanged. Distros: Debian/Ubuntu, Fedora; Visibility: implicit (Dev/Full).  
- **ripgrep** – fast recursive search used heavily in dotfiles and CLI workflows. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **fd-find** – `fd` search helper (`fd` on Arch/Fedora). Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **fzf** – fuzzy finder for terminal interactions. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **tmux** – terminal multiplexer for resilient shells. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **htop** – interactive process monitor. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **ncdu** – CLI disk usage explorer. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **neovim** – modern editor provided for CLI editing sessions. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  

### Full profile additions
- **nodejs** – JavaScript runtime for optional frontend tooling and scripting helpers. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Full profile).  
- **npm** – Node package manager accompanying `nodejs`. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Full profile).  

### Automatically attempted optional packages
- **btop** – terminal-based resource monitor the installer “tries optional” on Dev/Full profiles. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **bat** – syntax-highlighting `cat` alternative optional on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **eza** – `ls` replacement optional on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **yq** – YAML CLI helper optional on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **lldb** – LLVM debugger tried in every profile via `try_optional`; Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.

## Buildroot dependencies (Dev & Full)
- **bison** – parser generator required for buildroot builds. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **flex** – lexical analyzer helper. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **gawk** – GNU Awk used by build scripts. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **texinfo** – document builder called by bootstraps. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **libncurses-dev** – terminal UI headers; Arch/Manjaro uses `ncurses`, Fedora uses `ncurses-devel`; Visibility: implicit (Dev/Full).  
- **libssl-dev** – OpenSSL headers; Arch/Manjaro uses `openssl`, Fedora `openssl-devel`; Visibility: implicit (Dev/Full).  
- **bc** – calculator used in build scripts. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **rsync** – mirrors artifacts into staging. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **cpio** – archive helper consumed by buildroot. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **unzip** – extracts ZIP payloads. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **file** – determines file formats; used in staging checks. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **patch** – applies diffs before building. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **python3 / python3-pip / python3-venv** – restated here for buildroot builds even though they appear above; Visibility: implicit (Dev/Full).

## Rust toolchain & tooling (all profiles + Dev/Full extras)
- `rustup` + stable toolchain – installs Rust in every profile.  
- `rustfmt`, `clippy`, `rust-src` – added via `rustup component add`.  
- `cargo-edit`, `cargo-watch`, `cargo-audit`, `bacon`, `just`, `sccache` – installed via `cargo install` on Dev/Full profiles.  
- `flamegraph` – `cargo install` on Full profile only.

## Git, GitHub CLI & SSH
- `git`, `git-lfs`, `gh`, `openssh-client`, and the GitHub CLI apt repository are configured per distro, ensuring SSH, not HTTP, rules the roost.

## Docker & container runtime (Dev & Full)
- Docker packages, repo setup, group membership, and service enablement happen after the Docker repo is configured; the Docker data-root module rewrite is optional (module D).  

## Shell & UX polish (Dev & Full)
- `zsh`, `oh-my-zsh`, starship prompt, and Powerlevel10k (module P) come online per profile.  

## Fonts & sync utilities
- Fonts, `rclone`, and other helpers follow the Dev/Full toggles.

## Core runners & helpers
- `PhaseContext` is the shared context; `run_or_record()`, `record_action()`, and `register_rollback_action()` log metadata and maintain the dry-run gate.  
- `PlatformContext` wraps `ConfigService`, `PlatformInfo`, and now exposes Pi helpers so R-07 keeps reliant on a single source of truth.  
- The new `runner` module centralizes `PhaseRunner` exports and emits `PhaseOutput`; the `registry` module wraps `PhaseRegistry`.  
