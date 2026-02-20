# WO-018-formalize-testing-infrastructure
> **Neon Chronicle (Technical polish)**: WO-018-formalize-testing-infrastructure keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Formalize and enhance the testing infrastructure for `mash-installer` to ensure comprehensive coverage across diverse environments, explicitly addressing `maelstrom` compatibility and providing robust alternative testing mechanisms for kernel-dependent or sandboxed scenarios.

## Problem
The current testing strategy, while acknowledging `maelstrom`, appears to lack a clear, formalized approach for handling tests that cannot be run within `maelstrom`'s sandbox (e.g., due to kernel dependencies or `ENOSYS` errors). This can lead to:
- **Incomplete Test Coverage:** Critical functionalities might remain untested if they don't fit the standard `maelstrom` execution model.
- **Inconsistent Verification:** Reliance on ad-hoc or poorly defined "approved fallback testing mechanisms" introduces variability in QA.
- **Slow Feedback Loops:** Difficulty in quickly and reliably running tests in specialized environments.

## Proposal
- **Categorize Tests:** Clearly categorize tests based on their execution requirements:
    - **`maelstrom`-compatible:** Tests that can run within `maelstrom`'s sandbox.
    - **Kernel/Hardware-dependent:** Tests requiring specific kernel features, hardware (e.g., `aarch64` ARM), or elevated privileges.
    - **Integration/System-level:** Tests that interact with the actual operating system outside a strict sandbox.
- **Define Execution Strategies:** For each test category, establish explicit execution strategies:
    - **`maelstrom` Tests:** Ensure these are run first and automatically in CI.
    - **Kernel/Hardware-Dependent Tests:** Define a dedicated environment (e.g., specific VMs, physical hardware, specialized CI runners) where these tests must be executed. This might involve containerization or virtualization setups (e.g., QEMU for `aarch64` emulation).
    - **Alternative Testing Methods:** Formalize and document "approved fallback testing mechanisms," such as VM-based integration tests, end-to-end scenarios, or specific manual protocols for areas where automation is infeasible.
- **Test Orchestration:** Develop or integrate a test orchestration layer that can selectively run tests based on their category and the available environment. This could be a CI script that intelligently invokes different test commands or environments.
- **Reporting:** Ensure that results from all test categories are aggregated and reported consistently (linking with WO-016: Implement Dedicated Logging Framework).

## Rationale
Formalizing the testing infrastructure is crucial for ensuring the long-term quality, stability, and maintainability of the `mash-installer`. It addresses the current gaps in testing by providing clear guidelines for executing all types of tests, especially those that are difficult to run in standard CI environments. This reduces the risk of regressions, enhances developer confidence, and supports the "Done" criteria. By providing a structured approach to testing, it also makes the testing process less opinionated and more adaptable to the specific needs of different code components, supporting modularization by clearly defining how each module's correctness is verified.