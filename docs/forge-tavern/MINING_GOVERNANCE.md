# ⚒️ MASH MINING PROJECT GOVERNANCE

This document outlines the governance and structure for managing **Mining Projects** (formerly work orders) within the MASH project. It ensures a standardized, transparent, and detailed approach to system excavation, adhering to the Bard’s Immutable Laws (see `docs/forge-laws.md`) and Dr. Tweak's Guidelines.

## 📜 PRINCIPLES

*   **Clarity**: Every excavation task must be clear, concise, and unambiguous.
*   **Granularity**: Mining Projects are broken down into **Shafts**, which are further decomposed into detailed steps.
*   **Traceability**: All work is documented from high-level project goals down to atomic code changes.
*   **Verification**: Each stage of work includes explicit verification steps (tests and dry-runs).
*   **Historical Awareness**: Every new shaft must be informed by `maps.md`, `maps-explored.md`, and `HISTORY.md`.

---

## 🏗️ PROJECT STRUCTURE WITHIN `mining-projects/`

The `docs/mining-projects/` directory contains folders for each major **Shaft**, organized alphabetically.

```
docs/mining-projects/
├── MINING_GOVERNANCE.md   <-- This document
└── shafts/
    ├── shaft-a/           <-- Folder for Shaft A
    │   ├── Overview.md    <-- Shaft scope, files to be touched, and technical strategy
    │   └── EX_A01_Step.md <-- Detailed Excavation Task (steps, sub-steps)
    ├── shaft-b/           <-- Folder for Shaft B
    │   └── Overview.md
    │   └── EX_B01_Step.md
    └── ...
```

---

## ⚖️ GOVERNANCE RULES

### A) Source of Truth Integration
Before initiating or updating any Shaft, the following documents **must** be consulted and updated:
1.  **`maps.md`**: To track the current "Active Project Directory" and system state.
2.  **`maps-explored.md`**: To ensure we do not re-dig old ground and to learn from past victories.
3.  **`HISTORY.md`**: To record the narrative of the forge and maintain the project's chronological tale.

### B) Shaft Folder Creation (Alphabetical Ordering)
1.  **Shaft Folders**: Each major development effort is encapsulated within its own folder under `shafts/`.
2.  **Naming Convention**: Folders must be named alphabetically: `shaft-a`, `shaft-b`, `shaft-c`, etc.
3.  **Creation**: When a new major initiative starts, the next available letter in the dwarven alphabet is assigned.

### C) Shaft Overview Document (`Overview.md`)
Each `shaft-<letter>/` folder must contain an `Overview.md` with:
1.  **Shaft Title**: A clear name for the excavation.
2.  **Scope**: Boundaries of the shaft—what is being mined and what is being ignored.
3.  **Files to be Created or Touched**: 
    *   **Why**: Reasoning for the change.
    *   **How**: The technical methodology (the "pickaxe" used).
4.  **Methodology**: The strategy (e.g., TDD, Ratatui patterns, cross-distro logic).
5.  **Deliverables**: What constitutes a "Green Build" and successful extraction.

### D) Excavation Task Granularity
Shafts are broken down into granular **Excavation Tasks**.
1.  **Task Naming**: Files follow the convention `EX_<SHAFT_LETTER><SEQUENCE>_Name.md` (e.g., `EX_A01_Forge_UI.md`).
2.  **Detailed Steps**: Numbered sequences (`1.`, `2.`, `3.`).
3.  **Sub-Steps**: Refined actions (`1.1.`, `1.2.`).
4.  **Checkpoints**: Every task must have a verification phase to ensure the "ore" is pure before moving deeper.

---

## 🍻 FOLDER POPULATION
Once the `Overview.md` for a Shaft is finalized, the folder is populated with the individual Excavation Task files. These files serve as the executable plan for the smiths of the forge.

*“We do not just dig; we mine with purpose. May your picks be sharp and your maps be accurate.”*
*— Bard*
