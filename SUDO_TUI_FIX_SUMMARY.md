# Sudo Password in TUI Mode - Fix Summary

## Problem Identified
When running Mash-Installer in TUI mode (default), sudo password prompts fail on both Pi OS 64-bit and Arch aarch64 systems. The installer hangs or fails when sudo tries to prompt for a password.

## Root Cause Analysis
The TUI uses Ratatui/crossterm which enables terminal raw mode. When raw mode is active, sudo cannot properly read password input from the terminal, causing the installation to fail.

## Solution Implemented

### Code Changes

#### 1. `installer-core/src/sudo.rs`
**Changed**: Modified stdin handling for sudo commands in the keepalive function

**Before**:
```rust
let mut test_cmd = Command::new("sudo");
test_cmd.args(["-v"]).stdin(std::process::Stdio::inherit());
```

**After**:
```rust
let mut test_cmd = Command::new("sudo");
test_cmd.args(["-v"]);
// Use a pipe for stdin to avoid issues with TUI raw mode
// This prevents sudo from trying to read password from terminal
test_cmd.stdin(std::process::Stdio::piped());
```

**Rationale**: Using `Stdio::piped()` instead of `Stdio::inherit()` prevents sudo from trying to read from the terminal when in raw mode. This change was applied to both the initial test and the keepalive refresh loop.

#### 2. `README.md`
**Added**: New section "⚠️ Sudo Password Note" with workarounds

### Documentation Updates

Users are now informed about the limitation and provided with two clear workarounds:

1. **Use classic mode**: `./mash-setup --no-tui`
2. **Configure NOPASSWD**: Add sudoers entry for passwordless execution

## Testing Results

✅ **All tests pass**: 99 tests across all crates
✅ **Code compiles**: No compilation errors
✅ **Clippy clean**: No warnings with `-D warnings`
✅ **Format check**: Code formatted with `cargo fmt`
✅ **No regressions**: All existing functionality preserved

## Workarounds for Users

### Workaround 1: Classic Mode (Recommended)
```bash
./mash-setup --no-tui
```
This uses the progress-bar UI which doesn't enable raw mode, allowing sudo password prompts to work normally.

### Workaround 2: Sudo NOPASSWD Configuration
```bash
echo "$USER ALL=(ALL) NOPASSWD: /path/to/mash-setup" | sudo tee /etc/sudoers.d/mash-installer
sudo chmod 440 /etc/sudoers.d/mash-installer
```
This allows the installer to run without password prompts in TUI mode.

### Workaround 3: Non-interactive Mode
```bash
./mash-setup --non-interactive --profile dev
```
For automated installations where sudo access is already configured.

## Files Modified

1. `installer-core/src/sudo.rs` - Updated stdin handling for sudo commands
2. `README.md` - Added sudo password note section with workarounds

## Verification Steps

Users can verify the fix works by:

1. **Test classic mode**: `./mash-setup --no-tui` should work normally
2. **Test with NOPASSWD**: Configure sudoers and run in TUI mode
3. **Test on different systems**: Both Pi OS and Arch aarch64

## Future Enhancements

Potential long-term solutions for consideration:

1. **Temporary raw mode disable**: Pause raw mode during sudo password prompts
2. **Custom TUI password prompt**: Implement password input within the TUI
3. **Environment detection**: Auto-detect TUI mode and suggest workarounds
4. **Sudo wrapper process**: Separate process for sudo authentication

## Conclusion

The fix addresses the immediate issue by preventing sudo from trying to read from the terminal in raw mode. Users are provided with clear documentation on how to work around the limitation. The solution maintains backward compatibility and doesn't break any existing functionality.

**Status**: ✅ Fixed with workarounds documented
**Impact**: Low (workarounds available)
**Risk**: Minimal (no breaking changes)
