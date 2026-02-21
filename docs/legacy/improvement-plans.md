# Mash-Installer Improvement Plans
> **Neon Chronicle (Technical polish)**: This ledger keeps Phase 2 clear, records the D-03 dry-run gate, and notes exactly which lanes closed before the Pi 4B saga begins. ⚙️🛡️

## Phase Overview
| Phase | Focus | Status |
| --- | --- | --- |
| Phase 1 – Deduplication | Shared helpers, downloads, and system calls were unified. D-03 waited until `PhaseContext` could hold the gate. | ✅ Complete |
| Phase 2 – Refactoring | `lib.rs` split, `PhaseRunner` forged, `PhaseContext` hardened, `InstallationReport` shaped, registry and CLI wiring stabilized. | ✅ Complete |
| Phase 3 – Pi 4B HDD | Preflight, USB 3.0, and HDD tuning; paused until this refactor stays calm. | ❄️ Paused |
| Phase 4 – Hardening | TLS, rollback, locks, signal handling; gated on Phase 2 stability. | 🛡️ Blocked |

## Phase 2 Revised Priority (Decision + Order)
The queue honors one gate: `PhaseContext::run_or_record()` in `PhaseRunner` handles every dry run. The entries below show why each lane fired when it did.
1. **D-03 – Dry-run gate** — inserted in `PhaseRunner` to keep all helpers from scattering `if dry_run` checks. 🛡️  
2. **R-02 – Split `lib.rs`** — `runner` and `registry` wrappers now isolate the execution loop from the orchestration helpers. 🧱  
3. **R-01 – Harden `PhaseContext` helpers** — metadata (`record_action`, `register_rollback_action`, warnings, dry-run log) lives on the shared context without dragging the entire monolith. 🔩  
4. **R-03 – Structured `PhaseOutput` & `InstallationReport`** — phases report `actions_taken`, `rollback_actions`, `dry_run`, and `status`; the runner emits `InstallationReport` for CLI/TUI consumers. 🧾  
5. **R-08 – Typed `PackageSpec`** — packages carry intent and profile gates so registry wiring can make decisions without stringly tickets. 📦  
6. **R-05 – ConfigService depth** — configuration errors keep path/context so `InstallerError` advice is rich. ⚠️  
7. **R-04 – PhaseRegistry metadata** — each entry knows its gate (`Profile`, module alias, etc.), localization, and `PackageSpec` hooks after the contexts and reports settled. 🗂️  
8. **R-07 – Pi detection helpers** — `PlatformContext` now exposes `is_pi`, `pi_generation`, `is_pi_4b`, and `supports_usb3` for the Argon/Hyprland flows. 🐧  
9. **R-09 – Flatten RunSummary** — the runner now keeps completed-phase lists and errors inside `InstallationReport`, feeding the CLI/TUI without duplication. 📜  
10. **R-06 – DriverTestHarness** — harness-style tests exercise each registry/runner combination once the API stabilized. 🧪  
11. **R-10 – CLI/TUI contract** — the CLI listens to the structured report/event stream rather than printing from the core once the data flows are stable. 🎛️

## Reordered 10-Point Plan (with D-03)
The original PlanA ten points now ride this queue after D-03 so dependency edges stay intact.
1. **D-03 – Dry-run gate first** — every action flows through `PhaseContext::run_or_record()` before we touch contexts or emit metadata.  
2. **Plan Point 1 – Refactor `InstallContext`** — break the god object into focused contexts (`PlatformContext`, `UserOptionsContext`, trimmed `PhaseContext`).  
3. **Plan Point 2 – Introduce a Phase trait** — the trait now lives on the lean contexts so the runner can parse metadata.  
4. **Plan Point 4 – Forge a flexible PhaseRunner** — this runner iterates traits, enforces the gate, and returns structured outputs.  
5. **Plan Point 7 – Centralize configuration** — `ConfigService` handles defaults, validation, and richer error context.  
6. **Plan Point 8 – Abstract system operations** — commands, downloads, and services live in helper modules that observe dry runs and logging.  
7. **Plan Point 5 – Enhance error handling** — once helpers stabilize, richer `InstallerError` variants surface to the report.  
8. **Plan Point 9 – Externalize UI strings** — strings live in localization modules so CLI/TUI can reuse them.  
9. **Plan Point 3 – Decouple UI** — the UI reacts to events/reports rather than forcing prints inside the core.  
10. **Plan Point 10 – Improve the library API** — `installer-core` returns a fully formed `InstallationReport`; external interfaces orchestrate presentation.  
11. **Plan Point 6 – Comprehensive tests** — once contracts are stable, the harness validates drivers without hitting real systems.

## D-03 Decision Record
- **Why it was deferred:** Phase 1 lacked a consolidated `PhaseContext`, and early gate insertion risked scattering dry-run checks.  
- **Why it is now live:** `PhaseContext::run_or_record()` sits in `PhaseRunner`, every helper calls it, and the ledger keeps the gate visible. Balanced docs, tests, and logging now reference this single portal.

## Next Steps
1. Keep `docs/HISTORY.md`, `docs/QA/PlanA.md`, and the README in sync with this ledger; every change earns a Whimsical or Technical polish note.  
2. Trust the `InstallationReport`/`PhaseOutput` contract for CLI, TUI, and registry wiring before touching Phase 3.  
3. Run `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` from `/work/Mash-installer`, log the runs in `.logs`, and only promote green builds to `main`.  
4. Phase 3 (Pi 4B HDD tuning) waits until these signals stay calm; the ledger will flip when the forge is ready.
