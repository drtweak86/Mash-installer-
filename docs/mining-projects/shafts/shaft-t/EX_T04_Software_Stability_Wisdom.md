# EX_T04: Software Stability & Version Wisdom
> *"Scrying the digital spirits for known curses."* — Bard 🍺

## 🎯 OBJECTIVE
Implement detection for software-specific conflicts, architecture-related stability issues, and missing firmware.

## 📋 DETAILED STEPS

### 1. Identify Architecture-Specific Conflicts
- [ ] Implement `NodeArm64StabilityRule`:
  - Trigger: `profile.arch == aarch64` and `distro.version_id >= 43` and `nodejs == 22`.
  - Action: Warn about known instability; recommend LTS version.
- [ ] Implement `Armv7lCompatibilityWarning`:
  - Trigger: `profile.arch == armv7l`.
  - Action: Warn about reduced software availability compared to `aarch64`.

### 2. Firmware Scrying
- [ ] Implement `BrcmfmacFirmwareHint`:
  - Trigger: `brcmfmac` hardware present but `wlan0` interface missing.
  - Action: Recommend installing the `raspberrypi-kernel-headers` or firmware package.
- [ ] Implement `GpuDriverWarning`:
  - Trigger: Unknown or generic driver on high-resolution display.
  - Action: Suggest installing vendor-specific drivers (Mesa/NVIDIA/Intel).

### 3. Desktop Session Stability
- [ ] Implement `WaylandNvidiaWarning`:
  - Trigger: `session.type == Wayland` and `nvidia` driver detected.
  - Action: Suggest using X11 or checking specific Wayland compatibility settings.

## ✅ VERIFICATION
- [ ] Unit tests for version and architecture-based rules.
- [ ] `AdviceEngine` correctly identifies the Node 22/ARM64 curse.
