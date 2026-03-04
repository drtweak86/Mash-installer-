# 📟 MASH INSTALLER v1.0.0 — OFFICIAL USER MANUAL
> **STATION:** FORGE_TAVERN_BBS
> **DATE:** 22-FEB-1984 (RE-SYNCED 2026)
> **FROM:** BARD (RUNESMITH_STATION_01)
> **VERSION:** 1.0.0

---

## 📜 INTRODUCTION
Welcome, traveler. You have successfully accessed the **MASH** (Mythic Assembly & Sigil Heuristics)
system installer. MASH provisions a full development environment on Arch/Manjaro, Debian/Ubuntu,
or Fedora — with first-class support for Raspberry Pi 4B.

---

## 🚀 QUICK START

### One-Line Install (Recommended)
```bash
bash <(curl -L https://raw.githubusercontent.com/drtweak86/Mash-installer/main/install.sh)
```

### TUI Mode (Default)
Launches the interactive cyberpunk cockpit — arrow keys, Space to toggle, Enter to confirm.

### No-TUI Mode (Headless / SSH)
```bash
mash-setup --no-tui
```
Falls back to indicatif progress bars for remote or minimal terminals.

### Dry-Run Mode
```bash
mash-setup --dry-run
```
Simulates all phases and logs what *would* happen — no packages installed, no files written.

---

## 🖱️ TUI CONTROLS

| Key        | Action                                       |
|------------|----------------------------------------------|
| `↑` / `↓` | Navigate menu items                          |
| `Space`    | Toggle module selection (multi-select menus) |
| `Enter`    | Confirm selection / advance screen           |
| `Esc`      | Return to previous screen                   |
| `q`        | Abort and return to shell                    |

---

## 🛠️ INSTALLATION PROFILES

| Profile     | Description                                      |
|-------------|--------------------------------------------------|
| **Minimal** | Core packages only — git, curl, essential tools  |
| **Developer** | Full forge suite — Rust, Docker, shell polish  |
| **Archive** | Complete system + wallpapers + Pi tuning         |

---

## 🖼️ WALLPAPER DOWNLOADER

MASH can download a curated library of retro/cyberpunk wallpapers from three sources:
- **Wallhaven** — largest anime/wallpaper community
- **Pexels** — high-quality CC0 photography
- **Pixabay** — broad creative commons images

### API Keys (Optional)
Without keys, MASH skips wallpaper sources that require authentication.
Set keys as environment variables before running:

```bash
export MASH_WALLHAVEN_KEY=your_key_here
export MASH_PEXELS_KEY=your_key_here
export MASH_PIXABAY_KEY=your_key_here
mash-setup
```

| Variable            | Source    | Register at                           |
|---------------------|-----------|---------------------------------------|
| `MASH_WALLHAVEN_KEY` | Wallhaven | https://wallhaven.cc/settings/account |
| `MASH_PEXELS_KEY`   | Pexels    | https://www.pexels.com/api/new/       |
| `MASH_PIXABAY_KEY`  | Pixabay   | https://pixabay.com/api/docs/#api_key |

Wallpapers are downloaded to `/usr/share/backgrounds/retro/` with up to 3 concurrent connections.

---

## 🥧 RASPBERRY PI 4B TUNING

When MASH detects a Raspberry Pi 4B, it applies targeted HDD/SSD optimisations:

- **Mount options**: `noatime,commit=60` to reduce SD/HDD writes
- **Swap placement**: moves swapfile to external HDD if detected
- **Kernel parameters**: tuned `vm.swappiness`, `vm.dirty_ratio`, `vm.dirty_background_ratio`
- **I/O scheduler**: `mq-deadline` for USB-attached storage
- **USB 3.0 detection**: identifies and reports controller capabilities
- **Preflight checks**: reports disk health, scheduler, and partition layout

These changes are logged to `~/mash-install.log` and respect `--dry-run`.

---

## 🩺 DOCTOR MODE

Run a full system diagnostic before installing:

```bash
mash-setup doctor
```

Output includes:
- Pre-flight checks (commands, disk space, memory, connectivity)
- System info (OS, kernel, Pi model if applicable)
- Package manager detection
- Installed tools inventory
- Cargo tools status
- Wallpaper API keys status (PASS / WARN + setup URLs)
- SSH key inventory
- Config file location

JSON output for machine parsing:
```bash
mash-setup doctor --format json
```

---

## 🤖 AI SPIRITS

MASH optionally installs AI coding assistants via npm:

| Spirit      | Package                        |
|-------------|--------------------------------|
| Claude Code | `@anthropic-ai/claude-code`    |
| Gemini CLI  | `@google/gemini-cli`           |
| Mistral Vibe | `@mistral-ai/vibe`            |

Select "AI Spirits" in the software tier menu. MASH also injects a GitHub MCP server entry
into any detected Claude Desktop, Zed, Cursor, or VS Code configuration.

---

## 🐚 SHELL POLISH

MASH installs and configures:
- **Zsh** + Oh-My-Zsh
- **Starship** prompt with a custom retro config (`~/.config/starship.toml`)
- **Kitty** terminal with forge-tuned config (`~/.config/kitty/kitty.conf`)
- **eza** aliases — modern `ls` replacements (sourced from `~/.eza_aliases`)
- **Powerlevel10k** (optional, pass `--enable-p10k`)

---

## 📼 LOGS & TELEMETRY

| Output    | Location                        | Purpose                        |
|-----------|---------------------------------|--------------------------------|
| TUI cockpit | Realtime 4-pane display       | CPU, RAM, net, log stream      |
| Log file  | `~/mash-install.log`            | Persistent event record        |
| Dry-run   | stdout                          | What would happen (no writes)  |

---

## ⚠️ TROUBLESHOOTING

If you encounter a **HALTED** status:

1. **READ** the `ERROR` line in the TUI or log.
2. **LOCATE** the trace: `~/mash-install.log`
3. **FOLLOW** the `FIX` suggestion shown by the station.
4. **RETRY** the specific phase with `--dry-run` first to verify.

---

## 🏛️ SUPPORTED DISTROS

| Distro         | Status       | Notes                         |
|----------------|-------------|-------------------------------|
| Arch / Manjaro | ✅ Supported | pacman backend                |
| Debian / Ubuntu | ✅ Supported | apt backend, incl. Debian 13 |
| Fedora         | ✅ Supported | dnf backend                   |
| Raspberry Pi OS | ✅ Supported | Debian base + Pi tuning       |

---

## 📊 STATUS SUBCOMMAND

Quick overview of your system state — instant, no network checks.

```bash
mash-setup status              # pretty output
mash-setup status --format json  # JSON (CI-friendly)
```

Reports:
- **Platform**: distro, architecture, Pi model (if applicable)
- **Configuration**: path + state (loaded / missing / invalid)
- **Wallpaper API keys**: PASS/WARN per provider (Wallhaven, Pexels, Pixabay)
- **Pre-flight summary**: pass/warn/fail counts (fast checks only — no connectivity)

Run `mash-setup doctor` for the full diagnostic report including network checks.

---

## 🛠️ DEVELOPER WORKFLOW

### Daily Tools (`cargo xtask`)

All project tooling lives in the `xtask/` crate. Run with `cargo xtask <subcommand>`:

| Subcommand       | Description                                      |
|------------------|--------------------------------------------------|
| `check-docs`     | Check for broken markdown links in `docs/`       |
| `bump`           | Bump version: `cargo xtask bump <patch\|minor\|major>` |
| `release-check`  | Pre-release gate: fmt + clippy + tests + docs    |
| `hygiene`        | Move old scratch docs (>7 days) to `docs/legacy/`|
| `branch-prune`   | Delete local branches older than 7 days          |
| `test-infra`     | Run tests (maelstrom mode or fallback to cargo)  |
| `test-theme`     | Verify theme resource files and module structure |

### Release Workflow

```bash
# 1. Install once
cargo install cargo-release

# 2. Dry-run (no changes committed or tagged)
cargo release patch

# 3. Execute the release
cargo release patch --execute
```

`cargo release` will:
1. Run `cargo xtask release-check` (fmt + clippy + tests + doc links)
2. Bump all workspace crate versions in sync
3. Update version strings in `docs/MANUAL.md`
4. Commit with `chore: bump version to X.Y.Z`
5. Create and push `vX.Y.Z` tag → release pipeline auto-fires

### Cron Tasks (auto-managed)

| Cron binary               | Schedule         | What it does           |
|---------------------------|------------------|------------------------|
| `mash-doc-hygiene`        | Daily 03:00      | `cargo xtask hygiene`  |
| `mash-branch-prune`       | Sunday 02:00     | `cargo xtask branch-prune` |

---

## 🏗️ ARCHITECTURE NOTES

- Binary: `mash-setup` (single statically-linked binary for aarch64 and x86_64)
- Config: `~/.config/mash/mash.toml`
- Workspace: Consolidated structure with `installer-core`, `installer-cli`, `installer-drivers`, `mash-system`, `mash-wallpaper`, and `xtask`.
- Phases: metadata-driven via `PhaseRegistry` / `PhaseRunner`
- Dry-run: all side effects gated through `PhaseContext::run_or_record()`

---

### 🍻 TAVERN NOTES
- *"Slow is smooth, smooth is fast."*
- *"Always Be Backing up (ABB)."*
- *"The forge only crowns green builds."*
- *"If it compiles without warnings, ship it with pride."*

**END OF TRANSMISSION.**
```
   _______________________________________
  /                                       \
  |  MASH INSTALLER v1.0.0 — (C) 1984     |
  |  SYSTEM READY.                        |
  |  > _                                  |
  \_______________________________________/
```
