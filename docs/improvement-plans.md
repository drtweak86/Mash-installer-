# Mash-Installer Improvement Plans
> **Neon Chronicle (Technical polish)**: this ledger keeps the beats precise so readers can see the prioritized order, the deferred D-03 decision, and why each refinement follows the last. ⚙️

## Phase Overview
| Phase | Focus | Status |
| --- | --- | --- |
| Phase 1 – Deduplication | Extract shared helpers, unify downloads, and remove duplicate system calls across the phases. | ✅ Completed (D-03 dry-run gate deferred until the new `PhaseContext` could document it). |
| Phase 2 – Refactoring | Split `lib.rs`, formalize `PhaseRunner`, harden `PhaseContext`, standardize reporting, theory-craft a registry, and make the core library-grade. | ✅ Complete (module split, PhaseContext, PhaseRunner, registry, CLI wiring done) |
| Phase 3 – Pi 4B HDD | Harden Pi 4B preflight, optimize USB 3.0 staging, and tune HDD/Rust homes for the blue ports. | ❄️ Pending Phase 2 stabilization. |
| Phase 4 – Hardening | Add parking locks, TLS wiggles, signal handling, rollback safety, and lock files. | 🛡️ Blocked on Phase 2 API stability. |

## Phase 2 Revised Priority (Decision + Order)
The Phase 2 queue now honors a single source of truth: the dry-run gate must exist before anything else touches context, modules, or reporting. Each entry below lists why it runs when it does.
1. **D-03 – Dry-run gating (`PhaseContext::run_or_record`)** — this gate is in place inside `PhaseRunner`. Recording the decision keeps the ledger honest and prevents duplicate `if dry_run` checks. 🛡️
2. **R-02 – Split `lib.rs`** — creating module boundaries reduces coupling and lets the rest of Phase 2 work within scoped crates (orchestrator, runner, options, sudo). 🧱
3. **R-01 – Harden `PhaseContext` helpers** — with the codebase split, the shared context can natively host downloader, package, and service helpers without dragging in the entire `lib.rs` monolith. 🔩
4. **R-03 – Structured `PhaseOutput`** — a clear metadata schema (`actions_taken`, `rollback_registered`, `errors`, `dry_run`) must exist before registries or reports rely on what the runner produces. 🧾
5. **R-08 – Typed `PackageSpec`** — packages signal whether they're required, optional, or profile-gated, so registry and phases can programmatically decide what to install. 📦
6. **R-05 – `ConfigService` error fidelity** — richer configuration errors travel through `PhaseContext`, keeping failure stories visible before registries or drivers run. ⚠️
7. **R-04 – PhaseRegistry** — with structured outputs, typed specs, and a hardened context, the registry can honor profiles, metadata, and feature gates without guessing. 🗂️
8. **R-07 – Pi detection helpers** — PlatformContext earns clean helpers after the registry can accept the metadata it produces. 🐧
9. **R-09 – Flatten `RunSummary` into `InstallationReport`** — once reporting pillars are solid, flattening removes duplication and simplifies CLI/TUI wiring. 📜
10. **R-06 – DriverTestHarness** — tests can safely exercise each distro driver once the core runner/context/report contract is stable. 🧪
11. **R-10 – CLI/TUI split** — defer until data contracts are stable and the driver harness has exercised the new surfaces. 🎛️

## Reordered 10-Point Plan (with D-03)
The 10 points from `docs/QA/PlanA.md` nest into the Phase 2 queue after `D-03` so long as dependency edges stay intact. Here is the practical execution order and the reasoning behind it:
1. **D-03 – Dry-run gate** — run through `PhaseContext::run_or_record()` before anything touches the contexts or iterates phases so each helper bases dry-run behavior on one source of truth. 🛡️
2. **Plan Point 1 – Refactor `InstallContext`** — split the god object into targeted context slices (`PlatformContext`, `UserOptionsContext`, `PhaseContext` helpers) so traits and runners only ask for what they need. 🔩
3. **Plan Point 2 – Introduce a Phase trait** — trait objects built on the lean contexts describe the state machine (name, description, `execute`). That structure preps the runner for metadata capture. 🧭
4. **Plan Point 4 – Build a more flexible PhaseRunner** — now the runner can iterate trait objects, enforce the dry-run gate, and return structured reports that the CLI/TUI and registry can trust. 🧱
5. **Plan Point 7 – Centralize configuration management** — experience with the new contexts shows where config values belong; a `ConfigService` keeps defaults, validation, and error reporting consistent. ⚙️
6. **Plan Point 8 – Abstract system operations** — shell commands, file ops, downloaders, and service helpers move into shared modules that respect dry-run, logging, and caching concerns. 🧰
7. **Plan Point 5 – Enhance error handling** — rich `ConfigError` variants and phase-level contexts arrive once the helper surfaces settle so the runner can keep failures visible without stopping the entire program. ⚠️
8. **Plan Point 9 – Externalize UI strings** — once core helpers stop printing, move labels and messaging into config so the CLI can reuse them with different renderers. 📘
9. **Plan Point 3 – Decouple UI from core logic** — the CLI/TUI now react to events and reports instead of printing from `installer-core`, and the core runs cleanly even without a terminal. 🎛️
10. **Plan Point 10 – Improve the library API design** — the core now returns structured `InstallationReport` data, and drivers observe outcomes through typed metadata instead of raw prints. 📜
11. **Plan Point 6 – Implement a comprehensive test suite** — once the core API is stable, a harness exercises each distro driver to lock the refactor in place. 🧪

## D-03 Decision Record
- **Why it was deferred:** Phase 1 lacked the consolidated `PhaseContext`, so early gate insertion would have scattered `dry_run` checks across helpers.
- **Why it is now complete:** `PhaseContext::run_or_record()` lives in `PhaseRunner`, every new helper invokes it, and the dry-run visualizations go through one gate. Recording this decision in the improvement plan ensures future explorers know the gate went live once the context could sustain it.

## What Remains / Next Steps
1. Keep the Phase 2 ledger alive: every doc (README, HISTORY, improvement plans) retains its polish tag so readers know the Canon of Phase 2 is closed.  
2. Stay on the new `PhaseOutput`/`InstallationReport` contract: every CLI/TUI/driver change must lean on those metadata surfaces, plus `PackageSpec` gating, before Phase 3 even prints a ticket.  
3. Run `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` from `/work`, log the results in `.logs`, and only push to `main` after a green trilogy.  
4. Once these signals stay calm, begin drafting Phase 3 (Pi 4B HDD planning) with this refactored base instead of reshuffling Phase 2 again.

> The ledger notes that Phase 2 still breathes fire. Keep the rustfmt/clippy flames alive and the `sccache` cache warm. 
