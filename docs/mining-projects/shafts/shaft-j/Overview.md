# ⚔️ SHAFT J: The Overlord Protocols
> **Strategic Mining Plan**
> *"The Forge does not adorn itself with runes it cannot read. Every glyph deployed must be backed by steel."* — Bard 🍺

## 📜 Project Summary

Shaft J folds two incoming files into the installer — the **Pi Overlord terminal stack** (Kitty config, Starship config, eza aliases, software tiers ledger) — and corrects two core logic flaws in the installer itself: the **15-second architecture detection wait** (improvement 1) and the **Nerd Font source** (improvement 3). All three work together: the overlord configs require JetBrainsMono Nerd Font, and the user should not wait 15 seconds before reaching the install flow.

---

## 🗂️ Incoming Files Being Integrated

| Incoming File | What It Is | Deploy Target |
|---|---|---|
| `docs/incoming-files/kitty.txt` | BBC Acorn Kitty terminal config | `resources/shell/kitty.conf` |
| `docs/incoming-files/starship.toml.txt` | Goblin Starship prompt config | `resources/shell/starship.toml` |
| `docs/incoming-files/eza-aliases.sh` | Goblin mega eza + git alias set | `resources/shell/eza_aliases.sh` |
| `docs/incoming-files/software_tiers.md` | S-tier/A-tier software ledger | Reference only — already reflected in `installer-cli/src/software_catalog.rs` |

---

## 🛠️ Technical Plan

---

### 1. Promote the Overlord Configs into `resources/shell/`

#### Why These Files Need to Change

`installer-core/src/zsh.rs` uses `include_str!` to embed three config files at compile time:

```
// zsh.rs lines 21–23
const STARSHIP_CONFIG: &str = include_str!("../../resources/shell/starship.toml");
const KITTY_CONFIG:    &str = include_str!("../../resources/shell/kitty.conf");
const EZA_ALIASES_SCRIPT: &str = include_str!("../../resources/shell/eza_aliases.sh");
```

The current files in `resources/shell/` are placeholder versions. The incoming files from `docs/incoming-files/` are the Bard-curated final editions. Promoting them updates the compile-time embed with no Rust code change required.

#### 1.1 — Update `resources/shell/kitty.conf`

**File:** `resources/shell/kitty.conf`
**Why:** Current file is a placeholder. The incoming `kitty.txt` is the canonical BBC Acorn-themed Kitty config with correct JetBrainsMono Nerd Font reference.

**Exact change:** Replace the entire contents of `resources/shell/kitty.conf` with the content of `docs/incoming-files/kitty.txt`.

Key fields in the incoming version:
```
font_family      JetBrainsMono Nerd Font   ← drives the fonts.rs requirement (see §3)
background       #000000                    ← BBC Acorn black
foreground       #FFFFFF                    ← BBC Acorn white
cursor           #FF5555                    ← BBC Acorn red cursor
```

**Verification after change:** `grep "font_family" resources/shell/kitty.conf` must return `JetBrainsMono Nerd Font`.

#### 1.2 — Update `resources/shell/starship.toml`

**File:** `resources/shell/starship.toml`
**Why:** Current file is a placeholder. The incoming `starship.toml.txt` is the Goblin Starship config with Rust, Git, memory, and time modules correctly wired.

**Exact change:** Replace the entire contents of `resources/shell/starship.toml` with the content of `docs/incoming-files/starship.toml.txt`.

Key additions in the incoming version:
- `$memory_usage` module with 75% threshold
- `$time` module always shown
- `$cmd_duration` for slow-command tracking
- Proper `$rust`, `$git_branch`, `$git_status` modules

**Verification after change:** `grep "memory_usage" resources/shell/starship.toml` must return the `[memory_usage]` block.

#### 1.3 — Update `resources/shell/eza_aliases.sh`

**File:** `resources/shell/eza_aliases.sh`
**Why:** Current file is a placeholder. The incoming `eza-aliases.sh` is the Goblin mega alias set, including eza variants, git shortcuts, system ops, and the `goblin`, `cheat`, and `weather` easter eggs.

**Exact change:** Replace the entire contents of `resources/shell/eza_aliases.sh` with the content of `docs/incoming-files/eza-aliases.sh`.

Key additions in the incoming version:
```bash
alias lsg='eza -lah --git --icons'       # git-aware listing
alias lss='eza -lah --sort=size --icons' # sort by size
alias clean='sudo pacman -Rns $(pacman -Qdtq)'  # clean orphans
alias edit='helix'                        # editor alias
alias goblin='cmatrix -a'                 # easter egg
```

**Note:** The `clean` alias provides pacman orphan removal. This is a shell-level clean op, not a phase-level one. The clean and polish phases in `phase_registry.rs` are separate (see Shaft K §1).

**Verification after change:** `grep "goblin" resources/shell/eza_aliases.sh` must return `alias goblin='cmatrix -a'`.

---

### 2. Fix Arch Detection Skip (Improvement 1)

#### Why This Needs to Change

**File:** `installer-cli/src/tui/app.rs`

The `run()` function (line 919) currently:
1. Reads `std::env::consts::ARCH`
2. Calls `app.handle_auto_arch(arch)` unconditionally
3. `handle_auto_arch()` sets `screen = Screen::ArchDetected` and starts a 15-second timer
4. `tick()` (called every 250ms in the event loop) advances to `Screen::DistroSelect` after 15 seconds

This means **every launch wastes 15 seconds on the ArchDetected banner**, even when the arch is correctly identified and only one driver matches the platform.

The user can press Enter to skip but the default flow forces a wait.

#### 2.1 — Modify `run()` to Detect Single-Driver Match

**File:** `installer-cli/src/tui/app.rs`

**Location:** Lines 938–940 (the detection block):
```rust
// KISS: Internal detection is the source of truth
let arch = std::env::consts::ARCH.to_string();
app.handle_auto_arch(arch);
```

**Change to:**
```rust
// Detect arch and check if exactly one driver matches the platform.
// If so, skip the ArchDetected banner (no 15-second wait) and go
// directly to DistroSelect with the matched driver pre-selected.
let arch = std::env::consts::ARCH.to_string();

let single_match = installer_core::detect_platform()
    .ok()
    .and_then(|plat| {
        let matched: Vec<usize> = app
            .drivers
            .iter()
            .enumerate()
            .filter(|(_, d)| d.matches(&plat))
            .map(|(i, _)| i)
            .collect();
        if matched.len() == 1 { Some(matched[0]) } else { None }
    });

if let Some(idx) = single_match {
    // Exactly one driver matches — skip the arch banner, pre-select the driver,
    // advance directly to DistroSelect.
    app.selected_driver_idx = idx;
    app.menu_cursor = idx;
    app.screen = Screen::DistroSelect;
    app.bbs_msg = format!(
        "STATION_01: ARCH_SIGIL_{} AUTO-CONFIRMED. DRIVER PRE-SELECTED.",
        arch.to_uppercase()
    );
} else {
    // Multiple or zero drivers match — show the ArchDetected banner as usual.
    app.handle_auto_arch(arch);
}
```

**What this requires:**
- `installer_core::detect_platform()` must be re-exported from `installer-core/src/lib.rs`. Check current re-exports:
  - **File:** `installer-core/src/lib.rs`
  - **Verify:** `pub use platform::detect as detect_platform;` is already present (used in `orchestrator.rs` via `use crate::platform::detect as detect_platform;`)
  - If not already public in lib.rs, add: `pub use platform::detect as detect_platform;`

**Import required in `app.rs`:** `detect_platform` is already imported at the top via `use installer_core::{...}` — add it to that import list if not present.

#### 2.2 — Keep `handle_auto_arch()` Unchanged

`handle_auto_arch()` (lines 274–278) remains as-is. It is still used in the fallback case (multiple/zero matches). No change needed there.

#### 2.3 — Keep `tick()` Unchanged

The 15-second timer in `tick()` remains intact. It only fires when `arch_timer` is `Some(...)`, which only happens when `handle_auto_arch()` is called. In the single-match case, `arch_timer` stays `None`, so `tick()` never starts the countdown.

**Verification:** Launch the binary on a system with only one compiled-in driver. The ArchDetected screen must not appear. Launch on a multi-driver binary — ArchDetected must appear and auto-advance after 15 seconds or on Enter.

---

### 3. Fix Nerd Font Source to JetBrainsMono via Git Releases (Improvement 3)

#### Why This Needs to Change

**File:** `installer-core/src/fonts.rs`

Current state (lines 23–27):
```rust
fn install_terminess_nerd_font(ctx: &mut PhaseContext) -> Result<()> {
    let font_dir = ...;
    let target_font = font_dir.join("TerminessNerdFont-Regular.ttf");
    ...
    let version = "v3.2.1";
    let font_name = "Terminus.zip";
```

Two problems:
1. The kitty config (now promoted to `resources/shell/kitty.conf`) specifies `font_family JetBrainsMono Nerd Font`. Terminus is a different font. If Terminus is installed but JetBrainsMono is not, Kitty will silently fall back to a system default.
2. Version is hardcoded to `v3.2.1` and must be manually bumped with each Nerd Fonts release.

#### 3.1 — Change the Font Name Constant and Target File

**File:** `installer-core/src/fonts.rs`

**Change 1:** Rename the function for clarity:

```rust
// OLD:
fn install_terminess_nerd_font(ctx: &mut PhaseContext) -> Result<()> {

// NEW:
fn install_jetbrains_nerd_font(ctx: &mut PhaseContext) -> Result<()> {
```

**Change 2:** Update the target font path and the download parameters:

```rust
// OLD:
let target_font = font_dir.join("TerminessNerdFont-Regular.ttf");
...
let version = "v3.2.1";
let font_name = "Terminus.zip";

// NEW:
let target_font = font_dir.join("JetBrainsMonoNerdFont-Regular.ttf");
...
const NERD_FONT_VERSION: &str = "v3.3.0";
let font_name = "JetBrainsMono.zip";
let url = format!(
    "https://github.com/ryanoasis/nerd-fonts/releases/download/{}/{}",
    NERD_FONT_VERSION, font_name
);
```

Move `NERD_FONT_VERSION` to a module-level constant at the top of `fonts.rs` (after the `use` block):

```rust
// Nerd Fonts release to download from https://github.com/ryanoasis/nerd-fonts
// Bump this constant when a new Nerd Fonts release is tested and confirmed.
const NERD_FONT_VERSION: &str = "v3.3.0";
```

**Change 3:** Update the `install_phase()` call:

```rust
// OLD:
install_terminess_nerd_font(ctx)?;

// NEW:
install_jetbrains_nerd_font(ctx)?;
```

**Change 4:** Update the `run_or_record()` description string:

```rust
// OLD:
Some("Downloading from GitHub Nerd Fonts release".into()),

// NEW:
Some(format!("Downloading JetBrainsMono from github.com/ryanoasis/nerd-fonts @ {}", NERD_FONT_VERSION)),
```

#### 3.2 — Keep Terminus Base Packages

The call to `package_manager::ensure_packages` for `fonts-terminus`, `fonts-noto-color-emoji`, `xfonts-terminus` remains. These are system packages, not Nerd Font variants. They are independent of the JetBrainsMono download.

#### 3.3 — File Filter for Zip Extraction

The current extraction loop filters for `.ttf`:
```rust
if path.extension().and_then(|s| s.to_str()) == Some("ttf") {
```

JetBrainsMono.zip contains both `.ttf` and `.otf` files. The filter is correct — we only want `.ttf`. No change needed.

**Verification after change:** After install, `ls ~/.local/share/fonts/ | grep JetBrains` must return `JetBrainsMonoNerdFont-Regular.ttf` (and variants). Running `kitty` must use the correct font without fallback warnings.

---

## 🏗️ File Touch Summary

| File | Section | Nature of Change |
|---|---|---|
| `resources/shell/kitty.conf` | §1.1 | Full replacement with BBC Acorn config |
| `resources/shell/starship.toml` | §1.2 | Full replacement with Goblin Starship config |
| `resources/shell/eza_aliases.sh` | §1.3 | Full replacement with Goblin mega aliases |
| `installer-cli/src/tui/app.rs` | §2.1 | Replace 3-line arch detection block with 20-line single-match logic |
| `installer-core/src/fonts.rs` | §3.1 | Add `NERD_FONT_VERSION` const; rename fn; change font name and target file |
| `installer-core/src/lib.rs` | §2.1 | Verify `detect_platform` is re-exported; add if missing |

---

## ⚠️ Risks & Mitigations

| Risk | Mitigation |
|---|---|
| `detect_platform()` fails early in `run()` before terminal is ready | Wrap in `.ok().and_then(...)` — on `Err`, fall through to `handle_auto_arch()` as before |
| JetBrainsMono.zip URL changes between Nerd Fonts releases | `NERD_FONT_VERSION` is a single constant to bump; document in CHANGELOG |
| `detect_platform()` call in `run()` adds startup latency | `detect_platform()` reads `/etc/os-release` and `/proc` — sub-millisecond; acceptable |
| Existing Terminus font users lose their font | Terminus base packages still installed via `ensure_packages`; only the Nerd Font variant changes |
| `include_str!` compile-time embed fails if resource file missing | Files are promoted in-place — no new paths introduced; existing build path unchanged |

---

## ⚙️ Test Checklist

- [ ] `cargo build --workspace` passes after resource file updates
- [ ] `cargo test --workspace` passes — especially `phase_runner` and `driver_harness` tests
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] TUI launched on single-driver binary: ArchDetected screen does NOT appear
- [ ] TUI launched on multi-driver binary: ArchDetected screen DOES appear, auto-advances at 15s or on Enter
- [ ] `install_phase` for fonts: `JetBrainsMonoNerdFont-Regular.ttf` present after dry-run log; URL is correct
- [ ] Kitty launched after install: `font_family` resolves to JetBrainsMono (check `kitty +list-fonts | grep JetBrains`)
- [ ] Starship prompt shows memory module when RAM > 75%
- [ ] `source ~/.eza_aliases` in zsh: `ls` invokes eza, `goblin` invokes cmatrix

---

**Status**: Planned ⏳
**Owner**: Bard
