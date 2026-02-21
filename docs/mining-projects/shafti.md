# 🔐 SHAFT I: The Sudo Plumbing
> **Strategic Mining Plan**
> *“The rune of power must flow through the veins of the system, silent and secure.”* — Bard 🍺

## 📜 Project Summary
The final connection for the Sudo password system. Wiring the `InteractionService` to the TUI and injecting credentials into command execution.

## 🛠️ Technical Plan (Task 15)

### 1. The Wire (Interaction -> TUI)
- **Objective**: When `InteractionService` needs a password, the TUI must wake up.
- **Files**: `installer-core/src/interaction.rs`, `installer-cli/src/tui/observer.rs`.
- **Action**: Implement a callback/channel that triggers the `PasswordPrompt` TUI screen.

### 2. Credential Injection
- **Objective**: Commands needing sudo must use the stored password.
- **Files**: `installer-core/src/cmd.rs`, `installer-core/src/sudo.rs`.
- **Action**:
    - Modify `cmd::run` to check if `sudo` is required.
    - If yes, use `sudo -S` and pipe the stored password into stdin.
    - Ensure the password is never logged.

## 🏗️ Technical Dependencies
- `std::process::Stdio::piped()`

## ⚠️ Risks
- **Security**: Leaking passwords in logs. (Mitigation: Strict masking in `cmd.rs`).
- **Race Conditions**: TUI must block while waiting for input.

---
**Status**: Planned ⏳
**Owner**: Bard
