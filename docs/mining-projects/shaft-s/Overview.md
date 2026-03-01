# ⚒️ SHAFT S: THE ALL-SEEING EYE (Overview)
> *"A smith who does not know his anvil is a smith who breaks his hammer."* — Bard 🍺

## 🎯 OBJECTIVE
Transform the MASH installer from a passive tool into an intelligent, context-aware engine. We are forging an **Auto-Detection & System Profiling System** that extracts the very essence of the hardware and OS, allowing every installer decision to be guided by reality rather than assumptions.

## 📋 SCOPE
We will implement a comprehensive system profiling suite within `installer-core` that detects:
1.  **CPU & Architecture**: Model, cores, threads, and instruction sets.
2.  **Memory (Plasma)**: RAM, Swap, and ZRAM status.
3.  **OS & Session**: Distro identification, kernel version, init system, and the X11/Wayland/DE landscape.
4.  **Storage Hoard**: Block devices, partitions, filesystems, and Btrfs specifics.
5.  **Hardware Identity**: Identifying Raspberry Pi vs. Generic PC vs. Other ARM stations.

## 🔧 TECHNICAL STRATEGY

### 1. Data Model (`SystemProfile`)
We will create a central `SystemProfile` struct in `installer-core/src/profile.rs` (new module). This struct will be the **Single Source of Truth** for all installer logic.

### 2. Detection Sources
We will tap into the machine's own scrolls:
*   `/proc/cpuinfo`, `/proc/meminfo`, `/proc/swaps`
*   `/etc/os-release` (for the Distro's pedigree)
*   `/proc/device-tree/model` (The Pi Gold Standard)
*   `lsblk --json` (The storage inventory)
*   `findmnt --json` (The mountpoint map)
*   `btrfs` CLI (for the deep Btrfs subvolume runes)

### 3. Hardware Fingerprinting
*   **Pi Detection**: If `/proc/device-tree/model` contains "Raspberry Pi", we extract the exact model (4 vs 5).
*   **WiFi Check**: Scrying for `brcmfmac` in the device tree or `dmesg`.

### 4. Persistence & Display
*   **JSON Persistence**: Save to `~/.config/mash-installer/system_profile.json` using `serde_json`.
*   **TUI Visualization**: A new `SystemSummary` screen in `installer-cli` to show the smith exactly what the Eye has seen.

## 🏗️ EXCAVATION TASKS
*   **EX_S01**: System Profile Data Model & Core Module
*   **EX_S02**: CPU, Memory, and Distro Detection
*   **EX_S03**: Storage, Filesystem, and Btrfs Audit
*   **EX_S04**: TUI Summary Screen & JSON Persistence

## ✅ DELIVERABLES
1.  `SystemProfile` struct with full serialization support.
2.  Auto-detection logic for all 5 target areas.
3.  Integration into the main installer flow.
4.  Pretty TUI summary screen.
5.  `system_profile.json` saved to the config hoard.

*"May the Eye see what the hammer must hit."* — Bard 🍺⚒️
