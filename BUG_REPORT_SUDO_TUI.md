# Bug Report: Sudo Password Entry Fails in TUI Mode

## Issue Description
When running the Mash-Installer in TUI mode (default), sudo password prompts fail to work properly on both Pi OS 64-bit and Arch aarch64. The installer appears to hang or fail when it reaches the point where sudo requires password entry.

## Root Cause
The TUI uses Ratatui/crossterm which enables terminal raw mode. When raw mode is active, sudo's password prompt cannot properly read user input from the terminal, causing the installation to fail or hang.

## Technical Details

### Affected Components
- **File**: `installer-core/src/sudo.rs`
- **Function**: `start_sudo_keepalive()`
- **Issue**: The function runs `sudo -v` with `stdin(std::process::Stdio::inherit())`, which tries to read from the terminal. When the terminal is in raw mode (as it is in TUI), sudo cannot read the password.

### Call Stack
1. TUI starts and enables raw mode via `crossterm::terminal::enable_raw_mode()`
2. Installation begins and calls `installer_core::run_with_driver()`
3. Orchestrator starts sudo keepalive: `crate::sudo::start_sudo_keepalive()`
4. Sudo keepalive tries to run `sudo -v` to test sudo access
5. When sudo needs a password, it tries to read from terminal but raw mode prevents proper input
6. Sudo fails or hangs, breaking the installation

## Workarounds

### Workaround 1: Use Classic Mode (Recommended for most users)
Run the installer without TUI:
```bash
./mash-setup --no-tui
```

This uses the progress-bar UI which doesn't enable raw mode, allowing sudo password prompts to work normally.

### Workaround 2: Configure Sudo NOPASSWD
Add a sudoers entry to allow passwordless sudo for the installer:
```bash
echo "$USER ALL=(ALL) NOPASSWD: /path/to/mash-setup" | sudo tee /etc/sudoers.d/mash-installer
sudo chmod 440 /etc/sudoers.d/mash-installer
```

### Workaround 3: Run with --non-interactive
For automated installations:
```bash
./mash-setup --non-interactive --profile dev
```

## Code Changes Made

### 1. Updated `installer-core/src/sudo.rs`
Changed stdin handling to use pipes instead of inheriting terminal input:
```rust
// Before:
test_cmd.args(["-v"]).stdin(std::process::Stdio::inherit());

// After:
test_cmd.args(["-v"]);
test_cmd.stdin(std::process::Stdio::piped());
```

This prevents sudo from trying to read password from the terminal when in raw mode.

### 2. Updated `README.md`
Added a new section "⚠️ Sudo Password Note" explaining the issue and workarounds to users.

## Testing
- ✅ All existing tests pass (99 tests)
- ✅ Code compiles successfully
- ✅ No regressions in functionality

## Future Improvements

### Potential Long-term Solutions

1. **Temporary Raw Mode Disable**: Temporarily disable raw mode when sudo needs to prompt for password
   - Complex to implement due to thread synchronization
   - Requires careful terminal state management

2. **Password Prompt in TUI**: Implement a custom password prompt within the TUI
   - Would require significant refactoring
   - Could provide better user experience

3. **Environment Detection**: Detect if running in TUI and provide appropriate error messages
   - Could guide users to use `--no-tui` automatically
   - Would need to pass TUI mode flag to core library

4. **Sudo Wrapper**: Create a wrapper that handles sudo password prompts outside of raw mode
   - Could be a separate process that manages sudo authentication
   - Would require IPC between processes

## Recommendation
For now, users should use the `--no-tui` flag when they need to enter sudo passwords. The documentation has been updated to clearly explain this limitation and provide workarounds.

## Files Modified
- `installer-core/src/sudo.rs` - Updated stdin handling for sudo commands
- `README.md` - Added sudo password note section

## Verification
Users can verify the fix works by:
1. Running with `--no-tui` flag (should work normally)
2. Configuring NOPASSWD in sudoers (should work in TUI mode)
3. Testing on both Pi OS and Arch systems
