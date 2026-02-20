
> **Neon Chronicle (Technical polish)**: WO-003-ensure-idempotency keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️

# Title
WO-003: Ensure Idempotency for Package Installation

# Source
- PlanB.md - Section 3: "Fixes Needed for Program to Function as Intended"

# Goal
To make the system package installation phase idempotent, preventing errors or unnecessary work when re-running the installer. The installer should check if a package is already installed before attempting to install it.

# Scope
- `installer-core/src/pkg.rs`
- May require small additions to the `DistroDriver` trait in `installer-core/src/driver.rs` to query package status.

# Non-goals
- Do not make any other phases idempotent at this time.
- Do not change the package list.
- Do not refactor the `InstallContext`.

# Steps
1.  Add a new method to the `DistroDriver` trait, such as `is_package_installed(&self, package_name: &str) -> bool`.
2.  Implement this method for each existing `DistroDriver` implementation (e.g., for Debian/Apt, Arch/Pacman).
    -   For Apt, this can be done using `dpkg-query -W -f='${Status}' <package> | grep "install ok installed"`.
    -   For Pacman, this can be done using `pacman -Q <package>`.
3.  In `installer-core/src/pkg.rs`, before attempting to install the list of packages, iterate through the list.
4.  For each package, use the new `is_package_installed` method to check if it is already present.
5.  Filter the list of packages to only include those that are not already installed.
6.  Proceed with the installation using this filtered list. If the list is empty, the phase can complete immediately.

# Success criteria
-   Running the installer on a system where all system packages are already installed results in the "System packages" phase completing quickly and successfully without calling the package manager's install command.
-   Running the installer on a system with only some packages installed results in the installer only attempting to install the missing packages.
-   Running the installer on a system with no packages installed works as before.

# Tests
-   No automated test framework currently exists.
-   **Verification must be performed manually:**
    1.  Run the installer once on a clean system to install all packages.
    2.  Run the installer a second time on the same system.
    3.  Verify that the "System packages" phase is skipped or completes instantly, and that no `apt-get install` (or equivalent) command is executed (this can be checked via process monitoring or by observing installer output).

# Risk
Medium. This change modifies core installation logic. While it's intended to be safer, incorrect implementation of the package status check could cause packages to be skipped when they are needed.

# Commit message
```
feat(pkg): make system package installation idempotent

Modifies the system package installation phase to check if each package
is already installed before attempting to install it. This prevents
unnecessary work and potential errors when re-running the installer.

A new method `is_package_installed` has been added to the `DistroDriver`
trait to abstract the platform-specific check.

This makes the installer safer and more robust, addressing an item
in the "Fixes Needed" section of Plan B.

Verified by running the installer twice on the same system and
confirming that the package installation step was skipped on the
second run.
```