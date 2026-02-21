# WO-012-enhance-doctor-preflight-validation
> **Neon Chronicle (Technical polish)**: WO-012-enhance-doctor-preflight-validation keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Significantly enhance the `mash-setup doctor` module to perform a comprehensive suite of pre-installation validation checks, proactively identifying and reporting potential issues before the main installation process begins.

## Problem
The current `mash-setup doctor` functionality may be limited in its scope, potentially missing crucial environmental prerequisites or configurations that are vital for a successful installation. This can lead to installations failing midway through due to unaddressed system conditions, resulting in wasted time and a frustrating user experience.

## Proposal
- **Expand Validation Checks:** Broaden the range of checks performed by `mash-setup doctor` to include:
    - **System Resources:** Verification of available disk space, memory, and CPU capabilities against minimum requirements.
    - **Network Connectivity:** Testing connectivity to critical external endpoints (e.g., package repositories, GitHub, update servers).
    - **User Permissions:** Checking for appropriate permissions in target installation directories and for executing system-level commands.
    - **Required Utilities/Packages:** Ensuring necessary system utilities (e.g., `git`, `curl`, `docker`) and package managers are installed and accessible.
    - **OS/Distribution Compatibility:** Confirming the operating system and its version are supported.
    - **Existing Software Conflicts:** Detecting potentially conflicting software installations.
- **Detailed Reporting:** For each check, provide clear and actionable feedback:
    - **Success:** Indicate that the check passed.
    - **Warning:** Highlight non-critical issues that might impact the installation but don't necessarily prevent it.
    - **Error:** Clearly state critical issues that *will* prevent a successful installation and suggest immediate remedies.
- **Integration with Installer:** The main `mash-installer` should optionally (or mandatorily for certain modes) invoke `doctor` checks before proceeding with the installation.
- **Structured Output:** Provide options for both human-readable and machine-readable output formats for the `doctor` report, facilitating automation and integration with other tools.

## Rationale
Enhancing the `doctor` command is a critical step in improving the robustness and user experience of the `mash-installer`. By catching potential problems early, it minimizes mid-installation failures and reduces the need for manual troubleshooting. This proactive validation aligns with reducing opinionated defaults by clearly outlining system requirements and dependencies, and provides users with explicit guidance on how to prepare their environment for a successful installation. It also contributes to the overall reliability and perceived quality of the installer.