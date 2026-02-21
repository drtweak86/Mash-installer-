# 🚀 SHAFT H: The Expansion
> **Strategic Mining Plan**
> *“The mine grows deeper. We seek AI spirits, snapshot magic, and new hardware horizons.”* — Bard 🍺

## 📜 Project Summary
Expand the installer's capabilities to include AI agent installation, filesystem snapshots, and support for new SBCs.

## 🛠️ Technical Plan (Tasks 10, 11, 12, 16)

### 1. AI Agents
- **Objective**: Offer installation of Vibe, Gemini, and Claude.
- **Action**: Add a new `Phase` for "AI Assistants". Check for binaries or install scripts.

### 2. Filesystem Snapshots
- **Objective**: Safety net before changes.
- **Action**: Detect `btrfs` (Timeshift/Snapper) or `ext4`. If `btrfs`, create a pre-install snapshot.

### 3. The Python Script
- **Objective**: Optimize the wallpaper downloader.
- **Action**: Evaluate if `wallpaper_downloader.py` can be rewritten in Rust (`reqwest` + `tokio`). If not, ensure the Python environment is robust (venv).

### 4. SBC Expansion (Pi 5 / Odroid)
- **Objective**: Run on new hardware.
- **Action**: Update `platform.rs` to detect Pi 5. Mark these paths as "EXPERIMENTAL/UNTESTED" in the UI.

## 🏗️ Technical Dependencies
- `sys-info` (for FS detection).
- `snapper` or `timeshift` binaries.

## ⚠️ Risks
- **Scope Creep**: AI agent installation methods change frequently.
- **Python-Rust Interop**: Calling Python scripts can be fragile.

---
**Status**: Planned ⏳
**Owner**: Bard
