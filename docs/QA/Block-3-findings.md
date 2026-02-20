## Block 3 Findings (WO-011 to WO-015)
> **Neon Chronicle (Technical polish)**: Block-3-findings keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


### WO-015 (Commit `9f78636`): Centralize Interactions

**Summary:** This commit introduces `InteractionService` and `InteractionConfig` to centralize user interaction logic, allowing for interactive and non-interactive modes. CLI functions are updated to use this service.

**Findings:**
1.  **Resolved:** **Missing Tests.** The commit `15d022a` (fix: centralize interaction config) adds a dedicated test file `installer-core/tests/interaction.rs` with comprehensive tests for `InteractionService`, covering interactive/non-interactive modes and config precedence.
2.  **Resolved:** **Incomplete Configuration Integration.** The commit `15d022a` integrates `InteractionConfig` with `MashConfig` via `ConfigService`, allowing user-defined defaults for interaction points to be loaded from `config.toml`.

### WO-014 (Commits `f362548`, `4ae8684`, and `6f30453`): Add Rollback Manager

**Summary:** These commits introduce `RollbackManager` and integrate it into the `InstallContext`. The `RollbackManager` itself provides mechanisms for registering and executing rollback actions.

**Findings:**
1.  **Resolved:** **Missing Tests for Rollback Functionality.** The commit `6f30453` (`wo-014: add rollback manager tests`) adds a comprehensive test suite in `installer-core/tests/rollback.rs` that verifies `RollbackManager`'s order of execution and error aggregation.
2.  **Outstanding:** **Missing Integration in `PhaseRunner` to Trigger Rollback.** While `RollbackManager` is available in `InstallContext` (via `PhaseContext`), there is no visible integration within `PhaseRunner::run` (or elsewhere in `lib.rs` that calls it) that explicitly calls `ctx.rollback.rollback_all()` when an installation phase fails. Without this integration, the `RollbackManager` is merely a data structure; it does not actively provide transactional capabilities as intended by the WO.

### WO-013 (Commit `3662986`): Make Docker Data-Root Idempotent

**Summary:** This commit successfully makes the Docker `data-root` configuration idempotent by adding checks to avoid re-applying the same configuration and refactoring configuration loading.

**Findings:**
1.  **Missing Tests:** No dedicated unit or integration tests were added to verify the idempotency logic for the Docker `data-root` configuration. This is a critical omission for ensuring the robustness and correctness of this feature.

### WO-012 (Commit `856e9c1`): Expand Doctor Preflight Validation

**Summary:** This commit significantly enhances the `doctor` module with a comprehensive suite of pre-flight validation checks, structured reporting, and JSON output options.

**Findings:**
1.  **Missing Tests:** No new, dedicated test file (e.g., `installer-core/tests/doctor.rs`) was added to cover the extensive new functionality in the `doctor` module. Given its criticality and complexity, dedicated tests are essential to verify each check function, the overall `PreflightReport` generation, and the `display_preflight_checks` mechanism.

### WO-011 (Commit `6882d3d`): Enrich Command Diagnostics

**Summary:** This commit successfully enriches command diagnostics by capturing and reporting stdout/stderr and exit codes in error reports.

**Findings:** None.