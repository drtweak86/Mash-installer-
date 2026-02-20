
> **Neon Chronicle (Technical polish)**: WO-001-enhance-doctor-checks keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️

# Title
WO-001: Enhance `doctor` Pre-flight Checks

# Source
- PlanB.md - Section 3: "Fixes Needed for Program to Function as Intended"
- PlanC.md - Section 2: "Execution order (risk-first)"

# Goal
To prevent installation failures by adding more comprehensive pre-flight validation checks to the `mash-setup doctor` command. This will catch common system issues before the installation starts.

# Scope
- `installer-core/src/doctor.rs`
- Potentially reading configuration or platform info from `installer-core/src/lib.rs` and `installer-core/src/platform.rs`.

# Non-goals
- Do not refactor the `InstallContext` struct.
- Do not change the installation logic of any phase.
- Do not add new installation phases.

# Steps
1.  Identify a list of critical pre-requisites for the installer to function. This should include:
    -   Availability of essential shell commands (e.g., `curl`, `git`, `tar`).
    -   Sufficient free disk space on the root partition.
    -   Network connectivity to critical endpoints (e.g., `github.com`, `crates.io`).
    -   User permissions to write to key directories (e.g., the staging directory, user's home directory).
2.  Modify the code in `installer-core/src/doctor.rs` to implement these checks.
3.  For each check, provide clear success or failure messages.
4.  Ensure the `doctor` command exits with a non-zero status code if any check fails.
5.  (Optional but recommended) Add a `--pre-flight` flag to the main `mash-setup` command that runs these checks automatically before starting the installation.

# Success criteria
-   Running `mash-setup doctor` on a system with all dependencies present exits successfully with an exit code of 0.
-   Running `mash-setup doctor` on a system missing a required dependency (e.g., `curl` is uninstalled) reports the missing dependency to stderr and exits with a non-zero exit code.
-   Running `mash-setup doctor` on a system with insufficient disk space reports the issue and exits with a non-zero exit code.

# Tests
-   No automated test framework currently exists.
-   **Verification must be performed manually:**
    1.  Run `mash-setup doctor` on a compliant system to verify it passes.
    2.  Temporarily uninstall a required package (like `curl`) and run `mash-setup doctor` to verify the failure is detected and reported correctly. Reinstall the package after the test.
    3.  If possible, test on a system with low disk space to verify that check.

# Risk
Low. This change is non-destructive and adds checks rather than modifying core installation logic.

# Commit message
```
feat(doctor): enhance pre-flight validation checks

Adds a series of pre-flight checks to the `doctor` command to validate
the system environment before installation begins. This helps prevent
common installation failures.

Checks added for:
- Essential command availability (e.g., curl, git)
- Network connectivity to key services
- User permissions for critical directories

This addresses a key item in the "Fixes Needed" section of Plan B,
improving the overall robustness of the installer.

Verified by manually testing on a compliant system and a system where
'curl' was temporarily uninstalled to ensure correct error reporting.
```