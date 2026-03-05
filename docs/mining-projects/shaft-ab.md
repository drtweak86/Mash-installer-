# ⚒️ SHAFT AB: THE ASCENDED FLOW

**Objective**: Overhaul the TUI menu sequence to provide a more logical, scry-first user experience. Transition from a fragmented selection process to a streamlined linear flow guided by system intelligence.

## 📋 THE NEW RITUAL (MENU FLOW)

1.  **Welcome Screen** (`Welcome`): The grand entrance.
2.  **System Scan** (`SystemScan` - *New*): An active scrying process that detects hardware and network pedigree.
3.  **Scan Results & Wisdom** (`SystemSummary`): Displays the system pedigree and Bard's recommendations (Advice Engine).
4.  **Desktop Environment** (`DeSelect`): Choose the visual gateway (i3, GNOME, KDE, etc.).
5.  **Font Curation** (`FontPrep`): Select the runes (Nerd Fonts).
6.  **Software Selection** (`SoftwareSelect`): Choose the tools for the forge (Hierarchical catalog).
7.  **Final Provisioning Summary** (`Confirm`): A complete audit of all planned actions before execution.
8.  **Installation Forge** (`Installing`): The actual payload delivery.

## 🏗️ EXCAVATION PHASES

### Phase 1: Navigation Core Refactor
- Update `Screen` enum in `installer-cli/src/tui/state.rs` to include `SystemScan`.
- Refactor `installer-cli/src/tui/app/navigation.rs` to implement the new linear sequence.
- Ensure `navigate_back` correctly unwinds the new flow.

### Phase 2: Active Scrying (System Scan)
- Create `menus::draw_system_scan` to provide a visual representation of the detection process.
- Implement the scanning logic that populates `SystemProfile` and `PlatformInfo` if not already present.
- Auto-transition to `SystemSummary` upon completion.

### Phase 3: Wisdom Integration (Results & Recommendations)
- Update `SystemSummary` to display results from the `AdviceEngine`.
- Ensure hardware-specific tweaks (like ZRAM for Pi 4B) are highlighted as "Automated Wisdom."

### Phase 4: Simplified Selection sequence
- Harmonize the transition from DE selection -> Font selection -> Software selection.
- Fold `ThemeSelect`, `ProfileSelect`, and `ModuleSelect` into the appropriate steps or the final summary to reduce menu fatigue.

### Phase 5: Verification & Polish
- Stress test the "Back" button across all screens.
- Ensure all states (selections, environment tags) are preserved correctly.
- Verify the transition to the installation phase remains robust.

---

**Plan Status**: ⚒️ EXCAVATION INITIATED
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Target**: 2026-03-07
