# QA Report for mash-installer

## Executive Summary

The `mash-installer` project is an ambitious and largely well-structured idempotent installer. The existing `docs/REPORT.md` provides a commendable, detailed audit, which forms a strong foundation for this review.

Overall, the installer demonstrates robust logic for platform detection, package management abstraction, and a clear phase-based installation process. However, critical vulnerabilities exist in the chain of trust for external dependencies, and there are several areas where idempotency and platform compatibility are compromised. The project exhibits significant determinism issues due to relying on unpinned external resources.

The primary directive for this review is to ensure correctness, safety, quality, and release discipline. Based on this, several findings necessitate immediate attention to mitigate significant risks.

## Findings

### High Severity

1.  **Weak Bootstrap Chain of Trust:**
    *   **Description:** `bootstrap.sh` downloads the latest release binary and its `.sha256` checksum (if present) without mandatory verification. It proceeds if the `.sha256` is missing. This introduces a critical supply chain risk; a compromised GitHub release or CDN could lead to arbitrary code execution. The unauthenticated GitHub API for fetching the latest tag is also a rate-limit risk.
    *   **Impact:** Critical vulnerability, potential for arbitrary code execution during installation.

2.  **Execution of Unpinned Remote Scripts:**
    *   **Description:** Multiple installation phases (`rustup`, `oh-my-zsh`, `starship`, `rclone`, Argon40 OEM script) and Git clones (`powerlevel10k`, `argononed`) fetch and execute scripts or repositories from `master`/HEAD or unpinned latest versions.
    *   **Impact:** Non-deterministic installations, potential for upstream breaking changes, security vulnerabilities introduced via unreviewed third-party code.

3.  **Staging Directory Safety Bypass:**
    *   **Description:** The `staging.rs` module's root filesystem free space check (500 MiB) is only performed if the *parent* directory of the resolved staging path already exists. If a user-provided `--staging-dir` points to a path where a parent mount point does not exist (e.g., `/mnt/data/mash-installer` when `/mnt/data` is missing), `fs::create_dir_all` will create `/mnt/data` on the root filesystem, bypassing the safeguard and potentially filling the root partition.
    *   **Impact:** System instability, potential for root filesystem exhaustion.

4.  **No Transactional Rollbacks:**
    *   **Description:** The installer lacks any explicit rollback mechanism or transactional behavior. If a phase fails mid-installation, the system is left in an inconsistent, partially configured state.
    *   **Impact:** System instability, difficult recovery from failed installations, non-idempotent behavior on failures.

### Medium Severity

1.  **Docker Apt Repository Hardcoding:**
    *   **Description:** `src/docker.rs` hardcodes the Docker APT repository URL to `https://download.docker.com/linux/ubuntu` irrespective of the detected Debian-family distribution (e.g., pure Debian).
    *   **Impact:** Debian users may receive incompatible packages, experience installation failures, or receive older/untested Docker versions.

2.  **`daemon.json` Overwrite Risk:**
    *   **Description:** In `src/docker.rs`, when configuring `data-root`, if `/etc/docker/daemon.json` exists but cannot be parsed as valid JSON (e.g., due to comments or malformation), it is silently overwritten with a new `{"data-root": ...}` configuration. Existing valid settings will be lost.
    *   **Impact:** Data loss of Docker daemon configuration, unexpected Docker behavior.

3.  **`pacman -Sy` Partial Upgrade Risk:**
    *   **Description:** `src/pkg.rs` uses `pacman -Sy` to synchronize package databases for Arch-based systems. While `pacman -S --needed` ensures idempotency, running `-Sy` without a subsequent full system upgrade (`-Syu`) can lead to partial upgrade scenarios and system instability.
    *   **Impact:** System breakage or unexpected behavior for Arch users.

4.  **Misleading `--enable-ollama` Flag:**
    *   **Description:** The `--enable-ollama` flag is accepted by the CLI, but `REPORT.md` indicates no corresponding installation phase exists.
    *   **Impact:** User confusion, false expectation of functionality, incomplete installations.

5.  **Assumptions about Systemd Availability:**
    *   **Description:** The `docker.rs` module explicitly uses `systemctl enable --now docker.service`. `REPORT.md` notes this will fail in non-systemd environments (e.g., containers, WSL, minimal systems).
    *   **Impact:** Installation failures in specific environments; limits portability.

6.  **Unsupported ARM Architectures:**
    *   **Description:** `bootstrap.sh` and `platform.rs` explicitly support `aarch64/arm64` and `x86_64/amd64`. 32-bit Raspberry Pi OS (`armv7l`) is unsupported and will lead to a hard failure.
    *   **Impact:** Limited compatibility on older Raspberry Pi models or 32-bit ARM distributions.

### Low Severity

1.  **`bootstrap.sh` Overwrites `/usr/local/bin`:**
    *   **Description:** `bootstrap.sh` installs `mash-setup` into `/usr/local/bin` without checking for existing binaries. It always overwrites.
    *   **Impact:** Potential loss of a pre-existing `mash-setup` or another binary with the same name.

2.  **Incomplete `zshrc` Backup for `oh-my-zsh`:**
    *   **Description:** `REPORT.md` notes that backups are made for `starship`/`p10k` changes to `.zshrc` but not for modifications made by the `oh-my-zsh` script itself.
    *   **Impact:** Potential loss of user's `oh-my-zsh` customizations if the script modifies `.zshrc` significantly.

3.  **Redundant `jq` installation in `bootstrap.sh`:**
    *   **Description:** `bootstrap.sh` checks for `jq` and installs it as a prerequisite if missing, but also includes a fallback `grep`/`sed` method for `latest_release_tag` if `jq` is not found. The `pkg.rs` also includes `jq` in the core packages for `apt` and `pacman`.
    *   **Impact:** Minor inefficiency; `jq` is installed early and later ensured again.

4.  **Inconsistent `sudo` Handling:**
    *   **Description:** `bootstrap.sh` attempts `sudo` and falls back to non-`sudo` commands for prerequisite installation. However, the Rust codebase (`pkg.rs`, `docker.rs`, `github.rs`) directly uses `sudo` for many operations.
    *   **Impact:** Installation failure if `sudo` is not available or if the user cannot authenticate non-interactively within the Rust binary's execution context.

## Recommended Actions

The following actions are prioritized and must be addressed to enhance the security, reliability, and quality of the `mash-installer`.

### High Priority

1.  **Implement Mandatory Integrity Verification:**
    *   **Action:** Modify `bootstrap.sh` to *require* and *verify* a cryptographic signature (e.g., GPG, Cosign) or a mandatory `.sha256` checksum for the downloaded `mash-setup` binary. Fail if verification fails or if the manifest/checksum is missing.
    *   **Rationale:** Eliminates a critical supply chain attack vector.
    *   **Relevant Files:** `bootstrap.sh`, `src/github.rs` (if signing keys are managed here).

2.  **Pin External Dependencies and Scripts:**
    *   **Action:** For all remote scripts, Git clones, and `cargo install` commands (`rustup`, `oh-my-zsh`, `starship`, `rclone`, Argon40 OEM script, `powerlevel10k`, `argononed`), pin them to specific versions, commit SHAs, or tags. Introduce an explicit `--latest` flag for users who accept the risks of unpinned dependencies.
    *   **Rationale:** Ensures deterministic installations and protects against upstream breaking changes or malicious code injections.
    *   **Relevant Files:** `bootstrap.sh`, `src/rust.rs`, `src/zsh.rs`, `src/rclone.rs`, `src/argon.rs`, `src/pkg.rs` (for cargo install).

3.  **Enhance Staging Directory Space Check:**
    *   **Action:** In `src/staging.rs`, before creating any directories for the staging path, resolve the actual filesystem mount point for the *intended* staging directory. If the mount point is the root (`/`) and free space is insufficient, bail *before* `fs::create_dir_all` can create parent directories on the root filesystem.
    *   **Rationale:** Prevents unintended disk space exhaustion on the root partition.
    *   **Relevant Files:** `src/staging.rs`.

4.  **Implement Rollback Mechanism (or clear failure states):**
    *   **Action:** Investigate implementing a basic rollback strategy for critical installation phases, or at minimum, ensure clear error messages and guidance on how to manually clean up or recover from a partial installation. For critical operations, consider using temporary filesystems or atomic operations where possible.
    *   **Rationale:** Improves system stability post-failure and user experience.
    *   **Relevant Files:** `src/main.rs` (error handling), potentially new modules for transactional management.

### Medium Priority

1.  **Refine Docker APT Repository Selection:**
    *   **Action:** In `src/docker.rs`, dynamically select the Docker APT repository URL based on the detected Debian-family distribution (e.g., `linux/debian` vs `linux/ubuntu`).
    *   **Rationale:** Ensures correct Docker packages for Debian users, improving reliability and compatibility.
    *   **Relevant Files:** `src/docker.rs`.

2.  **Robust `daemon.json` Handling:**
    *   **Action:** In `src/docker.rs`, when modifying `/etc/docker/daemon.json`, implement more robust JSON parsing (e.g., using `jsonc-parser` if Rust has a similar library, or explicitly removing comments before parsing). If parsing fails, do *not* overwrite the file; instead, log an error and instruct the user to manually resolve the `daemon.json` issue.
    *   **Rationale:** Prevents unintended loss of existing Docker daemon configuration.
    *   **Relevant Files:** `src/docker.rs`.

3.  **Address `pacman -Sy` Risk:**
    *   **Action:** In `src/pkg.rs`, consider using `pacman -Syu` (full system upgrade) before installing packages on Arch-based systems, or provide a clear warning to the user to perform a full system upgrade prior to running `mash-setup`.
    *   **Rationale:** Mitigates partial upgrade scenarios and enhances system stability for Arch users.
    *   **Relevant Files:** `src/pkg.rs`.

4.  **Resolve `--enable-ollama` Discrepancy:**
    *   **Action:** Either implement the installation logic for Ollama when `--enable-ollama` is passed, or remove the flag and associated CLI option until the functionality is ready.
    *   **Rationale:** Avoids misleading user expectations and ensures feature parity with advertised options.
    *   **Relevant Files:** `src/main.rs`, relevant feature modules.

5.  **Clarify Systemd Requirement:**
    *   **Action:** Add explicit checks for `systemd` availability in relevant modules (e.g., `src/docker.rs`, `src/argon.rs`) and provide clear error messages or warnings if `systemd` is not found when required.
    *   **Rationale:** Improves portability awareness and informs users about environment constraints.
    *   **Relevant Files:** `src/docker.rs`, `src/argon.rs`.

6.  **Extend ARM Compatibility:**
    *   **Action:** If feasible, add support for 32-bit ARM (e.g., `armv7l`) Raspberry Pi OS, or provide a graceful exit with a clear message indicating unsupported architecture early in `bootstrap.sh` and `src/platform.rs`.
    *   **Rationale:** Broadens the installer's reach and prevents hard failures on common Pi setups.
    *   **Relevant Files:** `bootstrap.sh`, `src/platform.rs`.

### Low Priority

1.  **Prevent `mash-setup` Overwrite:**
    *   **Action:** In `bootstrap.sh`, add a check before `install` to `/usr/local/bin/mash-setup`. If it exists, warn the user and/or prompt for confirmation before overwriting, or provide an option to install to an alternative path.
    *   **Rationale:** Prevents accidental loss of a pre-existing binary.
    *   **Relevant Files:** `bootstrap.sh`.

2.  **Comprehensive `.zshrc` Backup:**
    *   **Action:** In `src/zsh.rs`, ensure that changes made by the `oh-my-zsh` installation script to `.zshrc` are also included in the backup strategy.
    *   **Rationale:** Protects user customizations for `oh-my-zsh` users.
    *   **Relevant Files:** `src/zsh.rs`.

3.  **Streamline `jq` Handling:**
    *   **Action:** Ensure `jq` is installed as a *strict* prerequisite in `bootstrap.sh` before attempting to use it for `latest_release_tag`. Remove the `grep`/`sed` fallback. Rely solely on the Rust `pkg` module for `jq` installation if the `bootstrap.sh` needs to be minimal.
    *   **Rationale:** Reduces code complexity and ensures consistent dependency management.
    *   **Relevant Files:** `bootstrap.sh`, `src/pkg.rs`.

4.  **Standardize `sudo` Usage:**
    *   **Action:** Consolidate `sudo` presence checks at the start of `bootstrap.sh` or `main.rs`. Ensure that if `sudo` is unavailable or non-interactive authentication fails, a clear error is presented to the user early. All subsequent calls to `sudo` should assume its availability.
    *   **Rationale:** Provides a consistent user experience regarding privileges and prevents unexpected failures mid-installation.
    *   **Relevant Files:** `bootstrap.sh`, `src/main.rs`, `src/pkg.rs`, `src/docker.rs`, `src/github.rs`.
