# ⚒️ SHAFT T: THE BARD'S WISDOM (Overview)
> *"Data is but cold iron; wisdom is the forge-fire that shapes it."* — Bard 🍺

## 🎯 OBJECTIVE
Transform the MASH installer into an **Intelligent Advisor**. We are forging an **Advice & Optimization Engine** that consumes the `SystemProfile` (from Shaft S) and provides context-aware warnings, performance tuning, and smart defaults. No more blind installations; the Bard will whisper the best path for every machine.

## 📋 SCOPE
We will implement an extensible "Advice Engine" within `installer-core` that triggers on:
1.  **Hardware Bottlenecks**: RAM < 8GB, SD Card vs. NVMe, thermal concerns.
2.  **Platform Quirks**: Pi-specific Wayland warnings, ARM64 Node.js stability, WiFi firmware hints.
3.  **Storage Optimization**: Btrfs snapshot bootstrapping, GitHub workspace relocation for small root disks.
4.  **Software Sanity**: Version-specific ARM64 conflicts (e.g., Node 22 instability), Laptop-specific power management.
5.  **Smart Defaults**: Auto-selecting the `Minimal` profile on low-resource hardware.

## 🔧 TECHNICAL STRATEGY

### 1. The Advice Engine (`AdviceEngine`)
We will create a new module `installer-core/src/advice.rs`. This will contain a `Rule` trait and a collection of `DwarvenWisdom` objects.
*   **Input**: `SystemProfile` + `InstallOptions`.
*   **Output**: A list of `AdviceEntry` objects (Level: Info, Warning, Critical).

### 2. The Wisdom Bank (Rules)
*   **RAM Threshold**: If `profile.memory.ram_total < 8GB` → Recommend `Minimal` profile.
*   **Pi Wayland**: If `profile.platform == RaspberryPi` and `session.type == Wayland` → Warn of performance/stability issues.
*   **Node/ARM Drift**: If `profile.arch == aarch64` and `distro.version_id == 43` and `nodejs == 22` → "Known instability; recommend LTS."
*   **Small Root / Large Data**: If `mounts['/'].size < 30GB` and `mounts['/data'].size > 100GB` → Suggest moving GitHub/Docker workspaces.
*   **Firmware Scry**: If `brcmfmac` detected but `wlan0` missing → Suggest firmware package installation.

### 3. TUI Integration
*   The **System Summary Screen** (from Shaft S) will now include a "Bard's Counsel" section.
*   **Critical Blockers**: Some advice may require a "Are you sure?" confirmation before proceeding.

### 4. Smart Defaults
The `InstallOptions` will be pre-populated based on the `AdviceEngine` findings before the user even sees the first menu.

## 🏗️ EXCAVATION TASKS
*   **EX_T01**: Advice Engine Core & Rule Trait
*   **EX_T02**: Hardware & Resource Wisdom (RAM, CPU, Platform)
*   **EX_T03**: Storage & Filesystem Optimization (Btrfs, Workspace Relocation)
*   **EX_T04**: Software Stability & Version Conflicts (ARM64 Node, Firmware)

## ✅ DELIVERABLES
1.  `AdviceEngine` capable of running multiple rules against a `SystemProfile`.
2.  20+ initial "Wisdom Rules" covering Pi, ARM64, and low-resource scenarios.
3.  "Bard's Counsel" UI component in the TUI.
4.  Auto-defaulting logic for profiles based on hardware metrics.

*"A wise smith listens to the ring of the anvil."* — Bard 🍺⚒️
