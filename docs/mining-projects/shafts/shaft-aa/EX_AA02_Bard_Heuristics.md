# EX_AA02: Bard's Heuristics

**Objective**: Evolve "Bard Recommends" from a static list into a dynamic decision engine.

## 🛠️ Technical Implementation
1.  **Heuristic Engine**:
    *   Create `installer-core/src/heuristics.rs`.
    *   Input: `SystemProfile` (CPU, RAM, Distro).
    *   Output: `Vec<Recommendation>` (Feature flags, package selections).
2.  **Logic Rules**:
    *   **RAM < 4GB**: Recommend `zram` swap, lightweight DEs, `vim` over `code`.
    *   **Cores > 8**: Enable parallel compilation features (`make -j$(nproc)`).
    *   **Pi 4B**: Force `pi4b_hdd` optimizations, disable heavy compositing.
3.  **UI Integration**:
    *   Add a "💡 Recommendation" badge next to options in the TUI that match the heuristics.

## 🧪 Verification
*   Mock `SystemProfile` with low specs matches "Lite" recommendations.
*   Mock `SystemProfile` with high specs matches "Power User" recommendations.
