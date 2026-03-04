# 🌋 Idea Percolation: Post-Consolidation Era

**Date**: 2026-03-04
**Context**: Following the completion of Shaft Y (Optimization) and Shaft Z (Consolidation).

## 💎 Retrospective: The Loot

We have successfully transmuted a sprawling 11-crate workspace into a hardened 5-crate diamond.

*   **Metric**: Binary Size `13.7MB` -> `4.5MB` (-67%)
*   **Metric**: Build Time (Incremental) `4m` -> `~45s` (Subjective feel: immediate)
*   **Metric**: Dependency Tree Depth: Significantly shallower (Removed `reqwest`, `hyper`, `tokio-rustls`, `sysinfo`).

**Key Learning**: For a system installer, **Control > Concurrency**. Removing async complexity in favor of blocking calls (`ureq`, `std::thread`) resulted in more readable, linear, and debuggable code.

## 🔭 The Horizon: Ideas for Shaft A & Beyond

### 1. The Grimoire Architecture (Shaft A Strategy)
Shaft A requires ingesting the "Software Grimoire" (hundreds of packages).
*   **Current State**: Packages are hardcoded or loosely defined.
*   **Percolation**: We should define the Catalog as a **Structurally Typed Data Set**.
    *   Use `enums` for Categories (Editors, Terminals, etc.) to enforce exhaustive matching in the UI.
    *   Use `installer-drivers` to enforce that every S-Tier package *must* have a mapping for Arch, Debian, and Fedora. The compiler should fail if a mapping is missing.

### 2. Dynamic Heuristics ("Bard Recommends")
The "Bard Recommends" profile shouldn't just be a static list. It should be context-aware.
*   **Hardware Awareness**:
    *   *High RAM (>32GB)*: Enable heavy caching services / in-memory databases by default.
    *   *Pi 4B*: Default to `zram` swap, disable heavy animations in desktop configs.
*   **Network Awareness**:
    *   If `ping` is high latency, prefer mirrors or smaller binaries.

### 3. The "Zero-HTTP" Ambition (Future Optimization)
We are currently static linking `rustls`. This is great for portability but heavy.
*   **Hypothesis**: We rely on `curl` for the bootstrap (`install.sh`).
*   **Experiment**: Create a `mash-curl` module in `mash-system` that wraps `std::process::Command("curl")`.
*   **Potential Gain**: Drop `ureq`, `rustls`, `webpki-roots`. Binary size could drop to ~1.5MB - 2.0MB.
*   **Tradeoff**: External dependency on `curl` (already required by bootstrap).

### 4. Telemetry & "The Ghost in the Shell"
We built a robust `PhaseObserver` trait.
*   **Idea**: Implement a `WebsocketObserver`.
*   **Usage**: Run the installer on a headless server (Pi), watch the TUI progress on your laptop via a web CLI.
*   **Fun Factor**: High.

---
*“We do not just build tools; we forge artifacts.”*
