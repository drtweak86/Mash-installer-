# EX_T02: Hardware & Resource Wisdom
> *"Scrying the physical limits of the machine."* — Bard 🍺

## 🎯 OBJECTIVE
Implement detection for hardware-related bottlenecks and platform-specific quirks.

## 📋 DETAILED STEPS

### 1. Identify Resource Constraints
- [ ] Implement `LowRamRule`:
  - Threshold: < 8GB RAM.
  - Action: Warn and suggest `Minimal` profile.
- [ ] Implement `NoSwapRule`:
  - Trigger: No Swap or ZRAM on < 8GB system.
  - Action: Strongly advise enabling ZRAM.

### 2. Pi-Specific Wisdom
- [ ] Implement `PiWaylandWarning`:
  - Trigger: `profile.platform == RaspberryPi` and `session.type == Wayland`.
  - Action: Warn about performance/stability. Recommend X11 for heavy desktop use.
- [ ] Implement `PiSdCardWarning`:
  - Trigger: Root disk is an SD card (`mmcblk0`).
  - Action: Warn about I/O bottleneck during heavy builds (Rust/C++).

### 3. Thermal & Power Wisdom
- [ ] Implement `LaptopDetectedRule`:
  - Trigger: `profile.platform == PC` and `is_laptop` (check chassis/battery).
  - Action: Recommend `auto-cpufreq` or `TLP`.
- [ ] Implement `HighCoreCountOptimization`:
  - Trigger: CPU Physical Cores > 16.
  - Action: Suggest `sccache` for maximum build speed.

## ✅ VERIFICATION
- [ ] Unit tests for each rule against mock `SystemProfile` objects.
- [ ] `AdviceEngine` correctly aggregates hardware warnings.
