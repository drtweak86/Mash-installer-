# 🛡️ Sudo Technical Report: TUI Integration & Fixes
> **Status**: Partial Fix (Infrastructure ready, full integration pending)  
> **Summary**: Technical audit of sudo password handling in Ratatui/crossterm terminal environments.

## 📋 Issue Description
When running `mash-setup` in TUI mode (default), sudo password prompts fail on multiple distributions (Pi OS 64-bit, Arch aarch64). The installer hangs or crashes when sudo requires password entry due to terminal raw mode.

## ⚙️ Root Cause Analysis
The TUI leverages `ratatui` and `crossterm`, which enable terminal **raw mode**. In raw mode, standard input handling is altered, and `sudo` is unable to read password input from the terminal properly.

### 📦 Affected Components
- `installer-core/src/sudo.rs`: Handles keepalive and initial sudo checks.
- `installer-cli/src/tui/app.rs`: Terminal state management and UI loops.
- `installer-core/src/cmd.rs`: Execution of external processes.

## 🛠️ Implemented Fixes (v0.1.4)
The immediate crashing/hanging issue has been addressed by isolating sudo's input stream from the terminal.

### 1. Isolated Sudo Input
In `installer-core/src/sudo.rs`, the stdin handling for `sudo -v` was changed from `Stdio::inherit()` to `Stdio::piped()`. This prevents sudo from attempting to read from the terminal while it's in raw mode.

```rust
// Fix in sudo.rs
let mut test_cmd = Command::new("sudo");
test_cmd.args(["-v"]);
test_cmd.stdin(std::process::Stdio::piped());
```

### 2. Sudo Password Infrastructure
We have implemented a secure, thread-safe memory storage for sudo passwords in `installer-core/src/sudo_password.rs` using `once_cell::sync::OnceCell` and `Arc<Mutex>`.

## 🧭 Future Integration Plan
To provide a seamless experience without requiring `NOPASSWD` configuration, the following integration is planned:

1. **TUI Password Prompt**: The `TuiApp` already has a `Password` screen state and `PasswordPrompt` message.
2. **Interaction Service**: `InteractionService::sudo_password` is ready to be hooked into the TUI message bus.
3. **Orchestrator Linkage**: Update the orchestrator to catch sudo password requests from the core and dispatch them to the TUI.
4. **Credential Injection**: Inject the captured password into subsequent `Command` calls using a custom wrapper or `sudo -S` (stdin password).

## ⚠️ Current Workarounds
Until full TUI integration is finalized, users can:

1. **Use Classic Mode**: Run `./mash-setup --no-tui` to use the progress-bar UI, which does not enable raw mode.
2. **Configure NOPASSWD**: Add a sudoers entry for the installer:
   ```bash
   echo "$USER ALL=(ALL) NOPASSWD: /path/to/mash-setup" | sudo tee /etc/sudoers.d/mash-installer
   sudo chmod 440 /etc/sudoers.d/mash-installer
   ```

---
**Technical Lead:**  
*Bard, Drunken Dwarf Runesmith*
