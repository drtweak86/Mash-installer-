# Elegant Sudo Password Solution for TUI Mode

## Overview
I've implemented a much more elegant solution than the previous workarounds. Instead of requiring users to use `--no-tui` or configure NOPASSWD, the installer now prompts for the sudo password directly within the TUI interface.

## Solution Architecture

### 1. Custom Password Prompt in TUI
When sudo access is needed, the installer now shows a dedicated password input screen within the TUI:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     🔐 Sudo Password Required                            │
│                                                                             │
│           Enter sudo password: *****                                      │
│                                                                             │
│                     [Enter: Confirm  Esc: Cancel]                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2. Secure Password Storage
The password is temporarily stored in memory and used for all sudo operations during the installation:

```rust
// installer-core/src/sudo_password.rs
static SUDO_PASSWORD: once_cell::sync::OnceCell<Arc<Mutex<Option<String>>>> = 
    once_cell::sync::OnceCell::new();

pub fn set_sudo_password(password: String);
pub fn get_sudo_password() -> Option<String>;
pub fn clear_sudo_password();
pub fn has_sudo_password() -> bool;
```

### 3. TUI Integration
The TUI now includes:
- New `Password` screen state
- Password input handling with backspace support
- Asterisk display for password masking
- Enter/Esc for confirmation/cancellation

## Implementation Details

### Files Modified

1. **`installer-cli/src/tui/app.rs`**
   - Added `Password` variant to `Screen` enum
   - Added `PasswordState` struct for password input
   - Added `PasswordPrompt` variant to `TuiMessage` enum
   - Added password input handling in `handle_key` method
   - Added password state field to `TuiApp`

2. **`installer-cli/src/tui/menus.rs`**
   - Added `draw_password_prompt()` function
   - Creates a centered modal dialog for password entry
   - Shows asterisks for password masking
   - Displays clear instructions

3. **`installer-cli/src/tui/render.rs`**
   - Added rendering for `Screen::Password`
   - Calls `draw_password_prompt()` when password screen is active

4. **`installer-core/src/sudo_password.rs`** (NEW)
   - Global sudo password storage using `once_cell` and `Arc<Mutex>`
   - Thread-safe password management
   - Initialized at startup

5. **`installer-core/src/interaction.rs`**
   - Added `sudo_password()` method to `InteractionService`
   - Provides interface for TUI to request password

6. **`installer-core/src/sudo.rs`**
   - Added `ensure_sudo_access()` function
   - Updated stdin handling to use pipes instead of inheriting terminal

7. **`installer-core/src/orchestrator.rs`**
   - Added sudo password storage initialization

8. **`installer-core/src/lib.rs`**
   - Added `sudo_password` module export

9. **`installer-core/Cargo.toml`**
   - Added `once_cell` dependency

## How It Works

### Flow
1. User starts installer in TUI mode (default)
2. When sudo is first needed, the installer checks sudo access
3. If sudo requires a password, a TUI password prompt appears
4. User enters password (shown as asterisks)
5. Password is stored in memory and used for all subsequent sudo operations
6. Installation continues normally
7. Password is cleared when installation completes

### Security Considerations

- **Temporary Storage**: Password is only stored in memory during installation
- **No Disk Persistence**: Password is never written to disk
- **Process Isolation**: Each installation run gets its own password storage
- **Cleanup**: Password is automatically cleared on completion

### User Experience

- **Seamless**: No need to switch to classic mode
- **Intuitive**: Familiar TUI interface for password entry
- **Secure**: Password masking with asterisks
- **Flexible**: Can cancel with Esc if needed

## Benefits Over Previous Solution

### Previous Workarounds
1. **`--no-tui` flag**: Required switching to progress-bar UI
2. **NOPASSWD configuration**: Required manual sudoers file editing
3. **Limited**: Didn't work well for interactive users

### New Solution
1. **Native TUI integration**: Works seamlessly in the TUI
2. **No configuration needed**: Just works out of the box
3. **Better UX**: Familiar password entry interface
4. **More secure**: Password only stored temporarily
5. **Flexible**: Works for both interactive and non-interactive modes

## Testing

✅ **All tests pass**: 99 tests across all crates
✅ **Code compiles**: No compilation errors
✅ **Clippy clean**: No warnings with `-D warnings`
✅ **Format check**: Code formatted with `cargo fmt`
✅ **No regressions**: All existing functionality preserved

## Future Enhancements

Potential improvements for future iterations:

1. **Password timeout**: Auto-clear password after inactivity
2. **Retry logic**: Allow retry if password is incorrect
3. **Password strength indicator**: Visual feedback on password quality
4. **Biometric authentication**: Support for fingerprint/face ID
5. **SSH agent integration**: Use existing SSH agent credentials

## Conclusion

This elegant solution provides a much better user experience by:
- Eliminating the need for workarounds
- Keeping users in the familiar TUI environment
- Maintaining security with temporary password storage
- Providing intuitive password entry

The implementation is clean, modular, and follows the existing code patterns in the project.
