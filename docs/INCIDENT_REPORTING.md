# ⚔️ Incident Classification & Bug Reporting

Errors in the mine are categorized to ensure appropriate response levels.

## 📁 INCIDENT CATEGORIES

### 🦟 Cave Vermin
- **Classification**: Small Bug
- **Description**: Minor UI glitches, typos, or non-critical issues that do not affect the main installation flow.
- **Priority**: Low
- **Assignment**: **Runesmith** or **Refiner**.

### 🕷️ Tunnel Spider
- **Classification**: Tricky Issue
- **Description**: Unexpected behavior in specific edge cases, logic errors that require careful trace, or intermittent failures.
- **Priority**: Medium
- **Assignment**: **Bard** (Architect) or **Refiner**.

### 📉 Collapsed Tunnel
- **Classification**: Build Failure
- **Description**: The forge fire has rejected the blade. `cargo build` or `cargo test` fails. No progress is possible until cleared.
- **Priority**: High (Blocker)
- **Assignment**: **Bard** or **Runesmith**.

### 🐉 Deep Dragon
- **Classification**: Architectural Threat
- **Description**: Issues that compromise the security, integrity, or long-term maintainability of the forge. Circular dependencies, security vulnerabilities, or fundamental design flaws.
- **Priority**: Critical
- **Assignment**: **Bard** (Architect).

---

## 🧭 HOW TO REPORT AN INCIDENT

When an incident is discovered, a report must be filed with the following:

1. **Title**: Prepend category (e.g., `[Tunnel Spider] Broken Arch override`).
2. **Depth**: Where in the mine was this found? (Crate/Module).
3. **Observation**: What did you see? (Error message/logs).
4. **Reproduction**: Steps to encounter the same beast.
5. **Severity**: Classification from the list above.

---
*"A reported bug is a beast halfway slain."* — Bard 🍺⚒️
