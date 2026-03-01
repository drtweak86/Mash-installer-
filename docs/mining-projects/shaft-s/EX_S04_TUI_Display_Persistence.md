# EX_S04: TUI Summary & Persistence
> *"The Eye must reveal all to the smith, and record its findings for the archives."* — Bard 🍺

## 🎯 OBJECTIVE
Create a visual summary of the system profile in the TUI and save the detailed findings to a JSON file.

## 📋 DETAILED STEPS

### 1. Persistence Rune (JSON Save)
- [ ] Implement saving the `SystemProfile` to `~/.config/mash-installer/system_profile.json`.
- [ ] Ensure the configuration directory is created if missing.
- [ ] Use `serde_json` for pretty-printed output.

### 2. TUI Visualization
- [ ] Design and implement a new "System Summary" screen in `installer-cli/src/tui/summary.rs`.
- [ ] Show key data:
  - Hardware model (e.g., Raspberry Pi 4 Model B Rev 1.5)
  - CPU (model, cores)
  - Memory (Total/Available RAM, Swap, ZRAM)
  - OS (Distro, Version, Kernel, Session type)
  - Storage (Root FS, Subvolumes, Free Space)

### 3. Integrate into the Forge Flow
- [ ] Call the `detect()` method during the preflight phase of the installer.
- [ ] Display the "System Summary" as the first screen or a selectable "System Info" menu.

### 4. Provide Debugging Info
- [ ] Add a `--profile` or `--doctor` sub-command to `mash-setup` to only show this profile and exit.

## ✅ VERIFICATION
- [ ] `mash-setup --profile` shows the correct system summary.
- [ ] `~/.config/mash-installer/system_profile.json` exists and is valid JSON.
- [ ] The TUI screen is visually consistent with the rest of the MASH interface.
