# WO-017-expand-dry-run-fidelity
> **Neon Chronicle (Technical polish)**: WO-017-expand-dry-run-fidelity keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Objective
Significantly enhance the `dry_run` mode of `mash-installer` to provide a more accurate, comprehensive, and detailed simulation of the full installation process without performing any actual system modifications.

## Problem
A limited `dry_run` mode that only simulates a superficial part of the installation process reduces its utility. If a `dry_run` doesn't accurately reflect what a real installation would do, it cannot reliably predict success or failure, nor can it provide clear insights into the actions that would be taken. This can lead to a false sense of security for users and still result in unexpected issues during a live installation.

## Proposal
- **Comprehensive Simulation:** Modify all relevant installation phases to incorporate `dry_run` logic. This means that instead of executing system commands or modifying files, they should:
    - Log *what* command would have been run (including full arguments).
    - Log *what* files would have been created, modified, or deleted (including their paths and potentially their simulated content).
    - Log *what* services would have been started, stopped, or configured.
    - Log *what* network requests would have been made.
- **Integration with Abstractions:** Leverage the abstracted system operations (WO-008: Abstract System Operations) to facilitate the `dry_run` mode. The `FileSystem`, `ShellCommandExecutor`, and `NetworkClient` services should have a `dry_run` implementation that logs actions instead of executing them. This is key to enabling consistent `dry_run` behavior across the codebase.
- **Detailed Report Generation:** At the end of a `dry_run`, generate a comprehensive report summarizing all intended actions. This report should be easily parsable and include:
    - A chronological list of all simulated operations.
    - Potential warnings or errors that would have occurred during a live run.
    - An estimate of resources that would have been used (e.g., disk space, network bandwidth).
- **User Interface Integration:** Present the `dry_run` report clearly to the user via the `installer-cli` (WO-003: Decouple UI from Core Logic), possibly offering different levels of detail.

## Rationale
Expanding the `dry_run` fidelity dramatically increases its value as a pre-installation validation and planning tool. It empowers users to understand the full impact of an installation before committing to it, catching potential problems and confirming expected behavior without risk. This aligns directly with reducing opinionated defaults by providing transparency into the installer's actions and allowing users to verify its behavior against their expectations, thus giving them more control and confidence in the installation process. It also serves as an excellent debugging aid for developers.