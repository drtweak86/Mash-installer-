# ⚒️ SHAFT AC: CHEZMOI INTEGRATION

**Objective**: Integrate `chezmoi` as a first-class dotfile management option in the MASH flow. This empowers the user to restore their personal configurations (dotfiles) as part of the initial provisioning ritual, ensuring the forge is instantly familiar.

## 📋 THE ENHANCED RITUAL (MENU FLOW)

1.  **Welcome Screen** (`Welcome`)
2.  **System Scan** (`SystemScan`)
3.  **Scan Results & Wisdom** (`SystemSummary`)
4.  **Desktop Environment** (`DeSelect`)
5.  **Font Curation** (`FontPrep`)
6.  **Software Selection** (`SoftwareSelect`)
7.  **Dotfile Restoration** (`ChezmoiConfig` - *New*): Configure `chezmoi` repository and branch.
8.  **Final Provisioning Summary** (`Confirm`)
9.  **Installation Forge** (`Installing`)

## 🏗️ EXCAVATION PHASES

### Phase 1: Core Foundation (installer-core)
- **Model Expansion**: Add `ChezmoiOptions` to `InstallOptions` and `UserOptionsContext` in `installer-core/src/model/options/mod.rs`.
- **The Chezmoi Module**: Create `installer-core/src/chezmoi.rs` to handle `chezmoi` installation, initialization (`chezmoi init <repo>`), and application (`chezmoi apply`).
- **Registry Registration**: Add the `chezmoi` phase to `PhaseRegistry` in `installer-core/src/phase_registry.rs`, depending on `git_cli` and `system_packages`.

### Phase 2: TUI Integration (installer-cli)
- **State Machine**: Add `ChezmoiConfig` to the `Screen` enum in `installer-cli/src/tui/state.rs`.
- **Navigation Flow**: Update `installer-cli/src/tui/app/navigation.rs` to insert the `ChezmoiConfig` step between software selection and the final summary.
- **Config UI**: Implement `menus::draw_chezmoi_config` in `installer-cli/src/tui/menus/selection.rs` (or a new module) to capture the repository URL and optional branch.

### Phase 3: Verification & Bardic Wisdom
- **Dry-Run Safety**: Ensure `chezmoi` operations respect the `dry_run` flag via `PhaseContext::run_or_record()`.
- **Heuristics**: Add an Advice Engine rule in `installer-core/src/advice.rs` to suggest `chezmoi` if the user is a "Dev" profile but hasn't configured dotfiles.
- **Verification**: Add unit tests for the `chezmoi` command generation and option validation.

---

**Plan Status**: ✅ EXCAVATION SEALED
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Completed**: 2026-03-06
