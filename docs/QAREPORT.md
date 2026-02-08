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

## Recommendations

*   **Create a `distro` module:** Create a `distro` module to encapsulate all the distribution-specific logic. This module would be responsible for detecting the distribution, translating package names, and providing a common interface for installing packages.
*   **Use a trait for package installation:** Create a `PackageInstaller` trait that defines a common interface for installing packages. Then, create separate implementations of this trait for each supported distribution (e.g., `AptInstaller`, `PacmanInstaller`). This would eliminate the code duplication in the `pkg` module and make it easier to add support for new distributions.
*   **Refactor the UI:** Refactor the UI to remove the responsibility leak. The UI should not be responsible for installing packages. Instead, it should check for the availability of required commands and inform the user if they are missing.
