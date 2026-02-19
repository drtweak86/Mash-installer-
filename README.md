# mash-installer

Idempotent mega-installer for Raspberry Pi 4 and Linux dev machines with a beautiful TUI.

**Features:**
- 🎨 Beautiful terminal UI built with ratatui
- 🐈 Kitty terminal included
- ⚡ Powerlevel10k prompt (starship removed)
- 📦 Modular installation profiles

**Supported distros:**
- Ubuntu 22.04+ / Debian (apt)
- Manjaro / Arch Linux (aarch64) / EndeavourOS (pacman)
- amd64 and arm64 architectures

## Overview

A two-layer installer:

1. **`bootstrap.sh`** – tiny bash script that installs minimal prerequisites,
   downloads the prebuilt `mash-setup` binary from GitHub Releases, and runs it.
2. **`mash-setup`** – Rust binary with ratatui TUI that performs the full idempotent installation.

The installer auto-detects your package manager (`apt` or `pacman`) and
translates package names automatically.

> **Note:** The CLI previously accepted `--enable-ollama`, but no install
> phase ever existed for it. That flag has been removed to avoid the
> misleading experience described in `docs/QAREPORT.md` (Medium 4).

## Quick start

Option 1 — One-liner (downloads latest release and runs it)
```bash
curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer-/master/bootstrap.sh | bash
```

Option 2 — One-liner with installer options (pass options to the bootstrap script)
```bash
curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer-/master/bootstrap.sh \
  | bash -s -- --profile dev --staging-dir /mnt/data/mash-installer
```

The bootstrapper now *requires* a `.sha256` checksum file for the downloaded `mash-setup` binary; the script exits if the checksum is missing or fails to verify, matching the QA report's High Priority 1 recommendation (see `docs/QAREPORT.md`).

You can also download the bootstrap script first and inspect or run it locally if you prefer:
```bash
curl -fsSL -o bootstrap.sh https://raw.githubusercontent.com/drtweak86/Mash-installer-/master/bootstrap.sh
less bootstrap.sh   # inspect
bash bootstrap.sh --profile dev --staging-dir /mnt/data/mash-installer
```

## Commands

```
mash-setup install [OPTIONS]
    --profile <dev|minimal|full>   Installation profile (default: dev)
    --staging-dir <PATH>           Override staging directory
    --dry-run                      Print what would happen without executing
    --interactive                  Enable interactive prompts
    --enable-argon                 Install Argon One fan scripts
    --docker-data-root             Set Docker data-root to staging_dir/docker

mash-setup doctor
    Print diagnostic information about the system state.

mash-setup config init
    Write default config to ~/.config/mash-installer/config.toml

mash-setup config show
    Print the current configuration.
```

## Profiles

| Profile   | What it installs |
|-----------|-----------------|
| `minimal` | Core build tools, git, Rust toolchain |
| `dev`     | Everything in minimal + buildroot deps, Docker, zsh/kitty/powerlevel10k, fonts, rclone, AI/scripting tools |
| `full`    | Everything in dev + Node.js/npm, flamegraph, optional extras |

## What gets installed

Package names below use Debian conventions; on Arch-based distros the
installer translates names automatically (e.g. `build-essential` → `base-devel`,
`fd-find` → `fd`, `python3` → `python`).

### All profiles
- **Build tools**: build-essential / base-devel, pkg-config, clang, lld, cmake, ninja-build, gcc/g++, gdb, make
- **Rust**: rustup + stable toolchain, rustfmt, clippy, rust-src
- **Git**: git, git-lfs, gh (GitHub CLI), openssh-client

### Dev profile and above
- **Cargo tools**: cargo-edit, cargo-watch, cargo-audit, bacon, just, sccache
- **Buildroot deps**: bison, flex, gawk, texinfo, libncurses-dev, libssl-dev, bc, rsync, cpio, etc.
- **Docker**: docker-ce / docker (Arch), docker-buildx, docker-compose
- **Shell/UX**: zsh, oh-my-zsh (unattended), starship prompt
- **Fonts**: Terminus, Noto Color Emoji
- **AI/scripting tools**: python3 + venv + pip, ripgrep, fd-find, fzf, jq, yq
- **Terminal**: tmux, htop, btop, ncdu, neovim, bat, eza
- **rclone**: via package manager or official script

### Full profile
- Node.js + npm
- flamegraph (cargo)

### Optional (flag-gated)
- **Argon One**: `--enable-argon` (Raspberry Pi 4 Argon One case fan control)
- **Docker data-root**: `--docker-data-root`

## Distro-specific notes

### Manjaro / Arch

- Docker is installed from community repos (`docker`, `docker-buildx`, `docker-compose`).
  No GPG key or third-party repo setup needed.
- GitHub CLI is `github-cli` in community repos.
- Packages that don't apply (`software-properties-common`, `apt-transport-https`,
  `python3-venv`, etc.) are silently skipped.
- `pacman -Syu` is run to sync the database and avoid partial-upgrade issues.
- `--needed` ensures pacman is idempotent (already-installed packages are skipped).

- Each distro driver implements the shared `DistroDriver` trait, so the core phases
  ask the driver for package translations, apt repo metadata, and service names
  before touching Docker/GitHub CLI resources.

### Ubuntu / Debian

- Docker is installed from Docker's official apt repo (GPG key + sources list).
- GitHub CLI is installed from GitHub's official apt repo.
- All installs use `--install-recommends`.

## Interactive module selection

The installer’s interactive flow still runs three steps (distro selection, module selection, profile selection). If you choose `Select modules`, the CLI describes module toggles using aliases: `A` for Argon One fan control, `P` for Powerlevel10k shell polish, and `D` for managing Docker’s data root inside the staging dir. Those alias choices populate `ModuleSelection`, so the resulting flags (`--enable-argon`, `--enable-p10k`, `--docker-data-root`) remain aligned with the menu output.

## Configuration

Config file: `~/.config/mash-installer/config.toml`

```toml
staging_dir = "/var/tmp/mash-installer"

[agents]
larry = "/home/user/.config/mash-agents/larry"
moe = "/home/user/.config/mash-agents/moe"
claude = "/home/user/.config/mash-agents/claude"

[cache]
installer = "/home/user/.cache/mash-installer"
gh = "/home/user/.cache/gh"
cargo = "/home/user/.cache/cargo"
rustup = "/home/user/.cache/rustup"

[docker]
compose_plugin = true
# data_root = "/mnt/data/mash-installer/docker"  # optional

[git]
enforce_ssh = true
```

If `docker.data_root` is set, the installer will configure Docker to use that path.
The `--docker-data-root` flag overrides the config and uses `staging_dir/docker`.

### Staging directory

The installer needs a staging area for downloads and temporary files. It will
refuse to stage on the root filesystem if free space is below 500 MiB.

Resolution order:
1. `--staging-dir` CLI flag
2. `staging_dir` from config file
3. Auto-detect: `/mnt/data/mash-installer` → `/data/mash-installer` → `/var/tmp/mash-installer`

### SSH-based GitHub auth

This installer enforces SSH-based GitHub authentication. It will **never**
convert your git remotes to HTTPS. Ensure you have an SSH key configured:

```bash
gh auth login  # select SSH when prompted
```

## Idempotency

Every phase checks before acting:
- `dpkg -s` / `pacman -Q` to verify packages are installed
- `which` / file existence checks for binaries
- Config files are backed up before overwriting
- pacman uses `--needed` to skip installed packages

Re-running `mash-setup install` is safe and will skip already-completed steps.

## Building from source

```bash
# Native build
cargo build --release

# Cross-compile for aarch64
cargo install cross --git https://github.com/cross-rs/cross
cross build --release --target aarch64-unknown-linux-gnu
```

## Testing & CI

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all`

CI is gated on running the commands above (see `.github/workflows/ci.yml`), so pull requests should pass them locally before pushing.

`cargo maelstrom` is optional: it requires a compatible host kernel/non-Docker runtime (with the zygote/unprivileged-clone support) and typically cannot run inside Docker-based CI. Run it only when you have a runtime that exposes those primitives.

## Project structure

```
├── bootstrap.sh              # Layer 1: bash bootstrap (apt + pacman)
├── Cargo.toml                # Rust project manifest
├── src/
│   ├── main.rs               # CLI wiring (clap) + phase orchestration
│   ├── config.rs             # TOML config load/save
│   ├── platform.rs           # Distro/arch/family/Pi detection
│   ├── staging.rs            # Staging dir selection + space checks
│   ├── pkg.rs                # Package manager abstraction (apt + pacman)
│   ├── rust.rs               # rustup + cargo tools
│   ├── docker.rs             # Docker Engine install (apt repo or pacman)
│   ├── zsh.rs                # zsh + oh-my-zsh + starship
│   ├── fonts.rs              # Font installation
│   ├── github.rs             # Git, GitHub CLI (apt repo or pacman), SSH
│   ├── buildroot.rs          # Buildroot dependencies
│   ├── rclone.rs             # rclone install
│   ├── argon.rs              # Argon One (optional)
│   └── doctor.rs             # System diagnostics
├── .github/
│   └── workflows/
│       ├── ci.yml            # Build + test + lint + shellcheck
│       └── release.yml       # Release artifacts on tags
└── README.md
```

## License

MIT
