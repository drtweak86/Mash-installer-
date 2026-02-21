# ⚒️ The Miner's Active Maps: Current Shaft
> *“Focus the mind, strike the vein. One rune at a time.”* — Bard 🍺

## 🔐 SHAFT I: The Sudo Plumbing <ACTIVE> ⛏️
**Status**: Excavation Commencing.
**Objective**: Fix the TUI sudo interaction failure by wiring `InteractionService` to the Ratatui message bus and implementing `sudo -S` credential injection.

### 🛠️ Execution Plan (Shaft I)
1.  **The Wire**: Establish an mpsc channel between the `InteractionService` and the TuiApp to trigger the `PasswordPrompt` screen.
2.  **Blocking Ritual**: Ensure the installation thread waits for the TUI to return the password rune.
3.  **Credential Storage**: Utilize the `sudo_password` memory storage safely.
4.  **Injection Logic**: Modify `cmd::run` to detect `sudo` requirements and inject the stored password via stdin.
5.  **Security Masking**: Ensure password runes never hit the `tracing` logs or the stdout buffer.
6.  **Verification**: Test against a local sudo-check to ensure the ritual holds.

---
**Last Updated**: 2026-02-22  
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
