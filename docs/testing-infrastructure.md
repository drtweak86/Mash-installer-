# Testing Infrastructure

## Overview
The `mash-installer` tests are grouped by the environments they require. This document formalizes the available test categories and provides a lightweight orchestration helper that can detect the available tooling and run the right suite automatically. The same script can be used in CI, local development, and specialized validation labs.

## Test Categories

| Category | Description | Execution Strategy |
| --- | --- | --- |
| **maelstrom-compatible** | Rust unit/integration tests that compile and run inside `maelstrom`'s sandbox or any routine `cargo test` invocation. | `scripts/test-infrastructure.sh maelstrom` uses `maelstrom` when installed; otherwise it gracefully falls back to `cargo test` with the shared `CARGO_HOME=/tmp/cargo` configuration. |
| **kernel/hardware-dependent** | Tests that exercise Docker, networking, or other kernel primitives that typically fail under a constrained sandbox. These require real hardware, privileged VMs, or emulation (e.g., QEMU on `aarch64`). | `scripts/test-infrastructure.sh hardware` runs the long-form Rust suite targeted at physical or emulated machines. When automation is not available, document the hardware used and the manual steps performed. |
| **integration/system-level** | End-to-end scenarios that interact with actual services (Docker, systemd, package managers). These tests usually require dedicated VMs or the same environment used by the installer. | Run on a dedicated integration runner or REPL environment, log the commands executed, and capture the output for post-run analysis. |

## Execution Strategies

1. **Automated `maelstrom` validation**
   ```bash
   ./scripts/test-infrastructure.sh maelstrom
   ```
   If `maelstrom` is not installed (e.g., during a local development cycle), the helper runs the standard `cargo test` fallback while exactly reproducing the `CARGO_HOME=/tmp/cargo RUST_TEST_THREADS=1 cargo test -p installer-core` invocation that CI uses.

2. **Hardware/kernel runs**
   ```bash
   ./scripts/test-infrastructure.sh hardware
   ```
   This path logs the intention to run on a specialized machine or emulator and mirrors the expectations of kernel-heavy features. Extend this script with additional hooks (QEMU, VM provisioning) as needed.

3. **Manual or integration-only scenarios**
   When a test cannot be automated, record:
   * The machine or VM used (architecture, kernel version).
   * The commands issued and their outputs (use `scripts/test-infrastructure.sh` for the canonical commands).
   * Any observed variance from `maelstrom` results.

## Reporting & Logging

- Every run writes to `.logs/test-<mode>-<timestamp>.log`. Include these artifacts in CI builds and QA reports.
- Cross-reference this log with the dry-run reporting framework introduced in WO-016 and WO-017: when `dry_run` mode is active, the installer now emits a structured summary of simulated operations, making it easy to align the simulated actions with the tests executed above.
- When aggregating results, annotate each log with the category it belongs to so reviewers can quickly identify which environments have been validated.

## Next Steps

1. Integrate this script into CI pipelines so `maelstrom` runs happen automatically before other suites.
2. Provide a documented checklist for hardware-dependent verifications, including how to provision the VM, what versions must be used, and how to capture deterministic output.
3. Reuse the structured logging (WO-016 and WO-017) to augment test reports with contextual metadata (phase, packages, commands) for easier debugging.
