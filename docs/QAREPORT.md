# QA Report: `mash-installer` Architecture Review

This report provides a QA analysis of the `mash-installer` project, focusing on its architecture, crate boundaries, responsibility leaks, and code duplication.

## What is solid

*   **Single-crate structure:** The project is a single-crate project, which is appropriate for its current size and scope. This simplifies the build process and makes the project easy to manage.
*   **Clear entry point:** The `main.rs` file is a clear entry point to the application. It is responsible for parsing command-line arguments, handling user interaction, and orchestrating the installation process.
*   **Modular design:** The installation logic is broken down into modules (e.g., `pkg`, `rust`, `docker`), which is a good design practice. This makes the code easier to understand, maintain, and test.
*   **Good separation of concerns (mostly):** There is a good separation of concerns between the UI and the installation logic. The `main.rs` file acts as a controller, and the individual modules are responsible for the actual work.

## What is risky

*   **Distribution-specific logic:** The distribution-specific logic is handled within the `pkg` module. While this is acceptable for now, it could become a problem as the project grows and supports more distributions. The `translate_for_arch` function is a "code smell" that indicates a potential for future problems.
*   **Responsibility leak:** The `ensure_dialog_available` function in `main.rs` calls `crate::pkg::ensure_packages(&["dialog"], false)`. This is a responsibility leak, as the UI code is directly calling the package installation logic. The UI should not be responsible for installing packages.

## What should be refactored

*   **`pkg` module:** The `pkg` module could be refactored to improve the separation of concerns. The logic for translating package names and detecting the package manager could be moved to a separate module.
*   **Code duplication:** The duplication in the `apt_ensure` and `pacman_ensure` functions in the `pkg` module could be refactored. A more generic approach could be used to install packages, with the specific commands for each package manager being passed as arguments.
*   **`ensure_dialog_available` function:** The `ensure_dialog_available` function in `main.rs` should be refactored to remove the direct call to the package installation logic. The UI should instead check if the `dialog` command is available and, if not, inform the user that they need to install it.

<<<<<<< HEAD
## Recommendations

*   **Create a `distro` module:** Create a `distro` module to encapsulate all the distribution-specific logic. This module would be responsible for detecting the distribution, translating package names, and providing a common interface for installing packages.
*   **Use a trait for package installation:** Create a `PackageInstaller` trait that defines a common interface for installing packages. Then, create separate implementations of this trait for each supported distribution (e.g., `AptInstaller`, `PacmanInstaller`). This would eliminate the code duplication in the `pkg` module and make it easier to add support for new distributions.
*   **Refactor the UI:** Refactor the UI to remove the responsibility leak. The UI should not be responsible for installing packages. Instead, it should check for the availability of required commands and inform the user if they are missing.

## Follow-up QA status

### Tests

`cargo test` (with `CARGO_HOME=/home/larry/.cargo`) now exercises the CLI/database unit tests covering driver package translation and module aliases. The distro crates still report zero tests, but the targeted new cases prove the core driver hooks and CLI menu logic can be verified automatically.

**Recommendation:** Broaden the coverage to include the installer phases (pkg, Docker, Argon, etc.) and driver repo/service hooks to keep regressions from slipping in.

### Linter

`CARGO_HOME=/home/larry/.cargo cargo clippy --all-targets --all-features -- -D warnings` runs clean, including the new tests and module-selection helpers.

### Code Formatting

`cargo fmt -- --check` passes across the workspace.

### `bootstrap.sh` Script

`/tmp/shellcheck-v0.9.0/shellcheck bootstrap.sh` now succeeds using the locally downloaded binary; keep rerunning it whenever you touch the bootstrapper.

## Additional Risk Highlights

- Remote scripts (rustup, Argon40 OEM, Docker/GitHub CLI repositories) remain unpinned, so a compromised upstream resource could still expose the installer.
- `cargo maelstrom` consistently fails on this environment with `ENOSYS` while trying to clone the gen-1 process; a kernel/runtime with zygote/unprivileged-clone support is required for CI coverage.
- Docker data-root adjustments still assume systemd and unlimited disk space, so validating the target mount before rewriting `/etc/docker/daemon.json` is necessary to avoid corruption.

## Additional Refactor Opportunities

- The driver metadata already lives in `installer-core::DistroDriver`; consider splitting the repo/service helpers into testable modules so each distro driver stays minimal.
- The CLI module menu now wraps alias-based toggles with tests; extracting that behavior to a reusable component would help reuse the same flow across future UIs.
- Service handling for Docker/Argon could gracefully fall back when systemd is absent, and optional online scripts should be gated by configuration to maintain determinism.

## Additional Recommendations

- Run `cargo maelstrom` on supported kernels before any major release to keep the optional Maelstrom harness exercised.
- Keep growing automated test coverage for installer phases and repo/service logic as the workspace expands.
- Continue linting `bootstrap.sh` with ShellCheck whenever the bootstrapper changes to catch shell scripting issues early.
=======
The `cargo test` command (with `CARGO_HOME=/home/larry/.cargo`) now covers the new CLI/unit tests introduced for the driver hooks and module-selection helpers. The suite still reports 0 tests for the distro crates, but the installer CLI and core crate now exercise small, focused unit tests so the QA gap noted previously is closing.

**Recommendation:** Expand coverage beyond the handful of driver/menu tests so the installer phases, repo configuration, and module toggling logic remain verifiable.

### Linter

The `cargo clippy` command was run to check for lints and potential bugs in the codebase. The command finished successfully, which means there are no linting errors.

### Code Formatting

The `cargo fmt --check` command was run to check the code formatting. The command finished successfully, which means the code is correctly formatted.

### `bootstrap.sh` Script

`shellcheck bootstrap.sh` now passes using the locally downloaded ARM64 binary, covering the lint recommendation from the original QA pass.

**Recommendation:** Keep rerunning `shellcheck` whenever the bootstrapper changes to avoid shell-script regressions.

## Risk Highlights
- Remote scripts (e.g., `rustup`, `oh-my-zsh`, Argon40 OEM, Docker/GitHub CLI repos) still execute unpinned content, so the attack surface remains if upstream hosts are compromised.
- `cargo maelstrom` cannot run in this environment (`ENOSYS` from zygote clone); CI coverage must include Maelstrom runs on hosts/kernels that expose the required primitives before release.
- `docker data-root` rewrites still assume systemd and unlimited disk, so failure to validate free space before writing could leave `/etc/docker/daemon.json` misconfigured.

## Refactor Opportunities
- The new workspace now centralizes distro-specific metadata inside `installer-core::DistroDriver`, but shared helpers (package translations, repo wiring) can be further split into dedicated modules/tests so Arch/Fedora/Debian implementations stay minimal and traceable.
- The CLI module menu now exposes alias-driven toggles with tests; consider extracting that logic into a reusable UI helper if other interfaces (GUI/web) surface the same options.
- Continued refactors: ensure Docker repo/Argon One service handling gracefully falls back on non-systemd systems and adopt configuration guards for optional online scripts.

## Recommendations
- Run `cargo maelstrom` on a compatible runtime (zygote/unprivileged-clone support) prior to release to keep Maelstrom coverage green.
- Increase unit/integration test coverage for the installer phases (pkg, docker, argon, etc.) with driver-aware mocks to catch regressions in repo/service hooking.
- Keep `shellcheck bootstrap.sh` in the QA checklist; rerun after modifying the bootstrapper steps to ensure shell hygiene.
>>>>>>> 87b0025 (docs: expand QA report)
