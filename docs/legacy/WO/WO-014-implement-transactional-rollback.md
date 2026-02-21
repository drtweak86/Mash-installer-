# WO-014-implement-transactional-rollback
> **Neon Chronicle (Technical polish)**: WO-014-implement-transactional-rollback keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Implement transactional and rollback capabilities for critical installation steps within `mash-installer` to prevent leaving the system in a broken or inconsistent state in case of failure.

## Problem
Currently, if a critical installation phase fails, the installer may leave the system in a partially configured or broken state. This can be difficult for users to recover from, requiring manual intervention, clean-up, or even a system reinstallation. The absence of a rollback mechanism increases the risk associated with running the installer, especially for users installing on production or sensitive systems.

## Proposal
- **Identify Critical Phases:** Categorize installation phases based on their potential impact and reversibility. Focus initially on phases that perform significant system modifications (e.g., creating system users, modifying core configuration files, installing complex services, partitioning).
- **Strategy for Reversibility:** For identified critical phases, design and implement mechanisms to reverse their actions. This could involve:
    - **Pre-action Snapshots:** Before a critical change, create a snapshot of the relevant system state (e.g., backup configuration files, note down installed packages).
    - **Atomic Operations:** Wherever possible, use system utilities or Rust crates that support atomic operations (e.g., rename temporary files into place) to minimize inconsistent states.
    - **Explicit Undo/Cleanup Actions:** For each critical "do" action, define a corresponding "undo" or "cleanup" action.
- **Rollback Manager:** Integrate a rollback manager within the `PhaseRunner` (WO-004). This manager would:
    - Register "undo" actions as phases successfully complete their "do" actions.
    - If a subsequent phase fails, trigger the execution of all registered "undo" actions in reverse order of registration.
- **User Notification:** Clearly inform the user if a rollback is being attempted and the outcome of the rollback process. This should integrate with the enhanced error reporting (WO-005) and UI decoupling (WO-003).

## Rationale
Implementing transactional and rollback capabilities dramatically enhances the safety and reliability of the `mash-installer`. It provides users with confidence that a failed installation will not render their system unusable, reducing fear of experimenting and increasing trust in the tool. This is a significant step towards a more mature and robust installer, aligning with the goal of providing a less opinionated and more resilient installation experience by mitigating the risks associated with system modifications. It also supports modularization by requiring a clear definition of forward and backward actions for each critical component.