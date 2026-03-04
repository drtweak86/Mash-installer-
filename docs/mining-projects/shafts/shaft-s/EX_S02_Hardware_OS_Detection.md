# EX_S02: Hardware & OS Detection
> *"Scrying the machine's true pedigree."* — Bard 🍺

## 🎯 OBJECTIVE
Implement the core detection logic for CPU, RAM, OS, and hardware identity.

## 📋 DETAILED STEPS

### 1. Identify Hardware (The Pi vs. PC Detection)
- [ ] Implement `PlatformInfo::detect()`:
  - Check `/proc/device-tree/model` for "Raspberry Pi".
  - If Pi, extract version (4, 5, etc.).
  - Check for WiFi presence (`/sys/class/net/wlan0` or `brcmfmac` hints).
  - Fallback to `GenericArm` or `PC`.

### 2. Scry CPU & Memory
- [ ] Use `sysinfo` or direct `/proc` parsing to populate `CpuInfo`.
- [ ] Use `/proc/meminfo` and `/proc/swaps` to extract RAM, Available RAM, and Swap.
- [ ] Detect ZRAM by checking device names in `/proc/swaps` or `/sys/class/zram-control`.

### 3. Identify the OS & Distro
- [ ] Parse `/etc/os-release` for `ID`, `VERSION_ID`, and `PRETTY_NAME`.
- [ ] Detect the init system (check if `/sbin/init` points to `systemd`).
- [ ] Extract kernel version from `uname -r`.

### 4. Detect the Session Landscape
- [ ] Check `XDG_SESSION_TYPE` (Wayland vs. X11).
- [ ] Check `XDG_CURRENT_DESKTOP` (GNOME, KDE, COSMIC, etc.).
- [ ] Infer WM if desktop is unknown (check running processes for `i3`, `sway`, `hyprland`).

## ✅ VERIFICATION
- [ ] `cargo test -p installer-core profile` passes.
- [ ] Detection works on both a PC and a Raspberry Pi (if available for testing).
- [ ] `system_profile.json` correctly identifies the host system.
