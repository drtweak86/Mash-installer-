# 📟 SHAFT E: The Station Interface
> **Strategic Mining Plan**
> *“The screen is the anvil where the user meets the code. It must be solid, informative, and slightly nostalgic.”* — Bard 🍺

## 📜 Project Summary
Implement a robust Ratatui/Crossterm hybrid architecture, simplify error handling to the "KISS" principle, and ink the ancient texts (User Manual).

## 🛠️ Technical Plan (Tasks 2, 3, 13)

### 1. Hybrid Ratatui/Crossterm System
- **Objective**: Use the best tool for the job.
- **Files**: `installer-cli/src/tui/`
- **What**:
    - **Ratatui**: Handle layout, widgets (lists, gauges), and rendering loop.
    - **Crossterm**: Handle low-level raw mode, input events, and alternate screen switching.
    - **Why**: Ratatui is great for "painting," Crossterm is great for "controlling."

### 2. KISS Error Handling
- **Objective**: Errors should be actionable, not cryptic.
- **Files**: `installer-core/src/error.rs`
- **What**:
    - Format: `WHAT` (The error), `WHERE` (The phase/log), `FIX` (Suggestion).
    - Example: "HALTED: GIT_CLONE_FAILED. Log: ~/.mash.log. Fix: Check internet connection."

### 3. The Ancient Manual (`MANUAL.md`)
- **Objective**: An old-school BBS guide.
- **Files**: `MANUAL.md` (root or docs)
- **What**:
    - Written in the style of a 90s text file.
    - Usage instructions, flags, and troubleshooting.
    - Referenced in `README.md`.

## 🏗️ Technical Dependencies
- `ratatui`
- `crossterm`

## ⚠️ Risks
- **Terminal Compatibility**: Ensure raw mode exits cleanly on panic (panic hooks required).

---
**Status**: Planned ⏳
**Owner**: Bard
