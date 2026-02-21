# 📼 SHAFT F: The Black Box
> **Strategic Mining Plan**
> *“If a tree falls in the forest and no one logs it, did it really segfault?”* — Bard 🍺

## 📜 Project Summary
Establish rigorous telemetry and verification. Logs must go to a persistent file, and tests must cover the critical paths for both ARM and x86_64.

## 🛠️ Technical Plan (Tasks 4 & 5)

### 1. Persistent Logging
- **Objective**: Logs shouldn't vanish when the TUI closes.
- **Files**: `installer-core/src/logging.rs`
- **What**:
    - Direct `tracing` output to `~/mash-install.log` (or XDG state dir).
    - Ensure it captures debug events that don't appear in the UI.

### 2. Testing Rig
- **Objective**: Verify the blade before battle.
- **Files**: `tests/`
- **What**:
    - **Path Testing**: Mock tests for `aarch64` vs `x86_64` flow execution.
    - **Script Testing**: Use `shellcheck` and logic tests for `install.sh`.

## 🏗️ Technical Dependencies
- `tracing-appender`
- `tracing-subscriber`

## ⚠️ Risks
- **Permissions**: Logging to home dir requires correct user resolution (sudo vs regular).

---
**Status**: Planned ⏳
**Owner**: Bard
