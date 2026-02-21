# 🚪 SHAFT D: The Gate & Guardian
> **Strategic Mining Plan**
> *“The door must open for all who knock, provided they know the secret knock is just `curl | sh`.”* — Bard 🍺

## 📜 Project Summary
Upgrade the entry points of the application. `install.sh` must be a smart guardian, detecting architecture and dependencies before the forge even lights up. The README must reflect this simplicity.

## 🛠️ Technical Plan (Tasks 1 & 6)

### 1. Upgrade `install.sh` (The Guardian)
- **Objective**: Make the shell script the first line of intelligent defense.
- **Files**: `install.sh`
- **What**:
    - **A. Architecture Detection**: Auto-detect `x86_64` vs `aarch64`. Pass this knowledge to the TUI to skip the selection screen.
    - **B. Dependency Checks**: Verify `curl`, `tar`, `git` exist. If missing, fail gracefully with a suggestion.
    - **C. Integration**: Pass flags (e.g., `--arch detected_arch`) to the binary.
- **UX**: If detection is unsure, show a "Detected: [Arch]. Cancel in 15s to manual select" countdown.

### 2. Update README.md (The Signpost)
- **Objective**: Clear, visual instructions for the weary traveler.
- **Files**: `README.md`
- **What**:
    - Add the one-liner: `sh <(curl -L https://raw.githubusercontent.com/drtweak86/Mash-installer/main/install.sh)`
    - Add screenshots of the 1984 station TUI (placeholders for now).
    - Ensure it references the new `MANUAL.md` (from Shaft E).

## 🏗️ Technical Dependencies
- Bash scripting (for `install.sh`).
- `installer-cli` argument parsing (to accept detected architecture).

## ⚠️ Risks
- **False Positives**: `uname -m` can be tricky on some obscure SBCs.
- **Permissions**: Ensure the script remains executable when piped.

---
**Status**: Planned ⏳
**Owner**: Bard
