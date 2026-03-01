# ⚒️ The Miner's Active Maps: Current Shaft
> *"The blade is hardened. The crown is on the table."* — Bard 🍺

---

## 🏗️ SHAFT I: Software Catalog & Installation Flow Overhaul — ACTIVE
> *Branch*: `work-shaft-i-catalog` (to be created)
> *Risk*: MEDIUM (catalog restructuring, UI changes)
> *Reward*: HIGH (significantly improved user experience, logical organization)
> *Status*: 🔨 IMPLEMENTATION IN PROGRESS (Phase 1 Complete)

### OVERVIEW
Complete reorganization of MASH software catalog and installation flow to create a more logical, user-friendly, and efficient system with curated S-tier applications, multiple installation modes, and optimized dependency handling.

### PHASE 1: Software Catalog Curation ✅ COMPLETE
**Objective**: Create comprehensive software catalog with S-tier applications
- [x] Define Category Structure (10 main categories)
- [x] Curate S-Tier Applications (5 per category)
- [x] Include All Programming Languages
- [x] Create TOML Catalog Structure (`s-tier_catalog.toml`, `full_catalog.toml`, `programming_languages.toml`)
**Status**: ✅ COMPLETE 2026-03-01

### PHASE 2: Installation Modes
**Objective**: Implement Manual, Auto, and Bard's Recommendations modes
- [ ] Manual Mode: Individual program selection with search/filter
- [ ] Auto Mode: One-click category installation
- [ ] Bard's Recommendations: Opinionated S-tier selection
**Status**: ⏳ PENDING

### PHASE 3: Menu Restructuring
**Objective**: Organize software into logical categories
- [ ] Category/Subcategory Hierarchy implementation
- [ ] UI Implementation in `installer-cli`
**Status**: ⏳ PENDING

### PHASE 4: Installation Flow Optimization
**Objective**: Ensure proper installation order
- [ ] Dependency Resolution (ccache/sccache before heavy builds)
- [ ] Parallel Installation implementation
**Status**: ⏳ PENDING

### PHASE 5: UI Integration
**Objective**: Connect catalog to installer UI
- [ ] Software Selection Screens
- [ ] Visual Progress Tracking
**Status**: ⏳ PENDING

---

## 🏗️ SHAFT S: THE ALL-SEEING EYE (Auto-Detection & System Profiling) — PLANNING COMPLETE
> *Branch*: `work-shaft-s-profiler` (to be created)
> *Risk*: MEDIUM (complex system detection)
> *Reward*: HIGH (intelligent installer, context-aware decisions)
> *Status*: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING

### OVERVIEW
Implementation of a comprehensive auto-detection system that builds a `SystemProfile` of the machine's hardware, OS, and storage landscape.

### PHASE 1: System Profile Model
- **File**: `docs/mining-projects/shaft-s/EX_S01_System_Profile_Model.md`
- **Objective**: Define `SystemProfile` and sub-models with Serde support.
- **Status**: ⏳ PENDING

### PHASE 2: Hardware & OS Detection
- **File**: `docs/mining-projects/shaft-s/EX_S02_Hardware_OS_Detection.md`
- **Objective**: Scry CPU, RAM, Distro, and identify Raspberry Pi vs PC.
- **Status**: ⏳ PENDING

### PHASE 3: Storage & Filesystem Audit
- **File**: `docs/mining-projects/shaft-s/EX_S03_Storage_Analysis.md`
- **Objective**: Map block devices, partitions, and deep Btrfs runes.
- **Status**: ⏳ PENDING

### PHASE 4: TUI Summary & Persistence
- **File**: `docs/mining-projects/shaft-s/EX_S04_TUI_Display_Persistence.md`
- **Objective**: Visual summary in TUI and save to `system_profile.json`.
- **Status**: ⏳ PENDING

---

## 🏗️ SHAFT T: THE BARD'S WISDOM (Intelligent Advice Engine) — PLANNING COMPLETE
> *Branch*: `work-shaft-t-advice` (to be created)
> *Risk*: LOW (pure logic engine)
> *Reward*: MAXIMUM (user-friendly, expert-level system optimization)
> *Status*: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING

### OVERVIEW
Implementation of an intelligent "Advice Engine" that translates the `SystemProfile` into actionable wisdom, warnings, and performance tuning for the smith.

### PHASE 1: Advice Engine Core
- **File**: `docs/mining-projects/shaft-t/EX_T01_Advice_Engine_Core.md`
- **Objective**: Define `AdviceEngine` and the `Rule` trait.
- **Status**: ⏳ PENDING

### PHASE 2: Hardware & Resource Wisdom
- **File**: `docs/mining-projects/shaft-s/EX_T02_Hardware_Wisdom.md`
- **Objective**: Rules for RAM, CPU, Platform (Pi/Laptop), and thermals.
- **Status**: ⏳ PENDING

### PHASE 3: Storage & Filesystem Wisdom
- **File**: `docs/mining-projects/shaft-t/EX_T03_Storage_Wisdom.md`
- **Objective**: Rules for Btrfs, SD Cards, and Workspace relocation.
- **Status**: ⏳ PENDING

### PHASE 4: Software Stability & Version Wisdom
- **File**: `docs/mining-projects/shaft-t/EX_T04_Software_Stability_Wisdom.md`
- **Objective**: Rules for ARM64 Node, Firmware hints, and Session stability.
- **Status**: ⏳ PENDING

---

## 🏗️ SHAFT U: THE GREAT REFACTOR (Hardening & Deduplication) — PLANNING COMPLETE
> *Branch*: `work-shaft-u-refactor` (to be created)
> *Risk*: HIGH (codebase-wide changes)
> *Reward*: CRITICAL (structural integrity, idiomatic code, security)
> *Status**: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING

### OVERVIEW
Full codebase purification to ensure structural integrity and professional-grade hardening. This shaft addresses technical debt, eliminates redundancy, and strengthens the security and resilience of the MASH installer.

### PHASE 1: Deduplication
- **Objective**: Identify and eliminate redundant logic across all crates.
- **Status**: ⏳ PENDING

### PHASE 2: Structural Refining
- **Objective**: Refactor for idiomatic Rust patterns (SystemOps, Phase Registry).
- **Status**: ⏳ PENDING

### PHASE 3: Hardening & Security
- **Objective**: Strict input validation and side-effect gating for dry runs.
- **Status**: ⏳ PENDING

---

## 🏗️ SHAFT V: THE INTERACTIVE FORGE (Installation Flow & Authorizations) — PLANNING COMPLETE
> *Branch*: `work-shaft-v-interactive` (to be created)
> *Risk**: MEDIUM (complex flow management)
> *Reward*: HIGH (fully configured environment, user-friendly auth)
> *Status**: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING

### OVERVIEW
Perfecting the installation sequence and enabling "Living Installations" through interactive setup. Moves beyond simple binary installation to fully configured environments.

### PHASE 1: Dependency-First Orchestration
- **Objective**: Topological sorting of installation phases.
- **Status**: ⏳ PENDING

### PHASE 2: Interactive Authorizations
- **Objective**: gh auth, ssh-keygen, and git configuration hooks.
- **Status**: ⏳ PENDING

### PHASE 3: Post-Install Verification
- **Objective**: Automatic doctor runs to verify functional state.
- **Status**: ⏳ PENDING

---

## 🏗️ SHAFT W: THE AESTHETIC GUILD (Presets, Themes & Dotfiles) — PLANNING COMPLETE
> *Branch*: `work-shaft-w-aesthetic` (to be created)
> *Risk*: MEDIUM (UX complexity)
> *Reward*: MAXIMUM (beautiful, opinionated "out-of-the-box" experience)
> *Status**: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING

### OVERVIEW
Empowers the user with high-quality, opinionated configurations and aesthetic presets. Transitions the installer from providing "bare binaries" to providing "living, beautiful environments."

### PHASE 1: The Preset Engine
- **Objective**: Intelligent software/theme combinations based on user selection.
- **Status**: ⏳ PENDING

### PHASE 2: Dotfile & Theme Management
- **Objective**: Clobber-safe symlink management with automatic backups.
- **Status**: ⏳ PENDING

### PHASE 3: The Wardrobe (TUI Selector)
- **Objective**: Interactive TUI screen for fine-tuning themes and icons.
- **Status**: ⏳ PENDING

---

**Next Review**: 2026-03-05 (Implementation kickoff)
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-01
