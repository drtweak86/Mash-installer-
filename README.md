![Banner of the bard](docs/assets/banner_final.png)
# MASH Installer
> Forged beneath the ruins of older systems, polished by neon rain, and narrated in a mix of Tolkien questcraft, cyberpunk grit, and Matrix rhythm.

## 🌌 Mash-Installer: Cyber-Loom & Dwarven Forge Reborn
You step off the mag-rail into the canyon of **Mash-Installer**, where George R.R. Martin scheming meets the lush detail of Tolkien and the terse Matrix code drops. The CLI is a `ratatui` glyph-grid battle station, `phase_runner` plays the dungeon master, and every log entry becomes an enchanted ledger. The ledger—`docs/improvement-plans.md`—is the single source of truth for the phases and their ordering, and the creed carved into this repo is still alive: `Always Be Backing up`, `Keep Commits Small`, and `Always Be Testing`. We build and test inside `/work/Mash-installer`, leaving `main` for the drop-tested crown.

> **Neon Chronicle (Whimsical + Technical polish)**: This README thunders the saga, describes the reordered 10-point plan, records the deferred D-03 decision, and keeps the lore aligned with the ledger.

## 🕹 Quick Invocation (Fast Path)
Invoke the ritual in one breath:
```
curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh | bash
```
Prefer scoping the scroll first:
```
curl -fsSL -o bootstrap.sh https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh
less bootstrap.sh
bash bootstrap.sh
```
Every run chants `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` from `/work/Mash-installer` before anything touches `main`.

## 🧱 Act Structure (Phases & Status)
| Act | Focus | Status |
| --- | --- | --- |
| Phase 1 – Deduplication | Helpers untangled, downloads unified, and duplicate system calls cleared. D-03 deferred until `PhaseContext` was ready. | ✅ Complete |
| Phase 2 – Refactoring | `lib.rs` split, `PhaseRunner` forged, `PhaseContext` hardened, registry drafted, data contracts shaped, CLI/TUI teased apart. | ✅ Complete (R-02 through R-10 settled) |
| Phase 3 – Pi 4B HDD | Preflight, USB 3.0, and HDD tuning for the blue ports. | ❄️ Paused until Phase 2 stabilizes |
| Phase 4 – Hardening | TLS shepherding, rollback rituals, lockfiles, and signal forensics. | 🛡️ Blocked on Phase 2 API stability |

## 🔁 Phase 2 Constellation (Reordered 10-Point Plan + D-03)
The 10-point plan from `docs/QA/PlanA.md` now rides the rails behind a single gate: `PhaseContext::run_or_record()` (`D-03`). The saga continues with the explainable order below so each refactor builds on hardened foundations.
1. **D-03 – Dry-run gate**: Phase actions now pass through `PhaseRunner`’s single portal; no duplicate `if dry_run`. This is recorded in the ledger so the deferral is transparent. 🛡️
2. **Refactor `InstallContext`** (Plan point 1): Break the god object into focused contexts (`PlatformContext`, `UserOptionsContext`, `PhaseContext` slices) so phases only receive what they need. Tight coupling drops, testability rises. 🔧
3. **Formal `Phase` trait** (Plan point 2): Define `name()`, `description()`, `execute()`, `should_run()`, and let each phase implement it. The trait rests on the slimmed contexts and lets the runner reason about metadata. 🧭
4. **Phase runner redesign** (Plan point 4): Build `PhaseRunner` to iterate trait objects, capture structured reporting, and enforce the dry-run gate. It also becomes the public API that the CLI and TUI can consume. 🧱
5. **Centralize configuration** (Plan point 7): `ConfigService` validates, surfaces defaults, and feeds every context slice, keeping config errors consistent. 🗂️
6. **Abstract system helpers** (Plan point 8): Commands, downloads, file ops, and services move into shared helpers that master logging and dry-run behavior. 🛠️
7. **Structured error handling** (Plan point 5): `ConfigService` and every phase wrap outcomes in rich error enums so regulators can triage without rerunning the phase stack. ⚠️
8. **Externalize strings** (Plan point 9): UI text leaves the source and lands in config (TOML/JSON) so phases stay agnostic and localization becomes possible. 🗣️
9. **CLI/TUI decoupling** (Plan point 3): The CLI consumes events and reports instead of printing directly; the core returns metadata, letting the interface stay declarative. 🎛️
10. **Library API cleanup** (Plan point 10): `installer-core` returns structured `InstallationReport` data; the UI layers play conductor with those rich results. 📜
11. **Driver test harness** (Plan point 6): Once the data contracts stabilize, fire up the harness to exercise each distro driver against the new surfaces and catch regressions early. 🧪

## ❓ D-03: Dry-run Gate Decision
The deduplication phase deferred D-03 because the `PhaseContext` was still forming and dry-run checks were scattered. With `PhaseContext::run_or_record()` now living squarely in `PhaseRunner`, every action flows through one gate, one log, and one simulation path. This decision is logged here and mirrors the ledger entry in `docs/improvement-plans.md`.

## 🛠 Rules, Tooling, and Testing Rituals
- `Always Be Backing up` — snapshot the world before major refactors.
- `Keep Commits Small` — each logical change deserves a single hammer strike.
- `Always Be Testing` — run `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` from `/work/Mash-installer`.
- The toolbelt: `rustfmt`, `clippy`, and `sccache` keep builds fast and code tidy. Keep the cache warm between runs.
- The workflow: build/test in the `work` branch, only merge to `main` when every log in `.logs` tells a green story.

## 📚 Docs & Lore
- `docs/improvement-plans.md` is the canonical ledger of phase order, D-03 status, and the 10-point plan rationale. It now captures the ordering above plus the deferral log.
- `docs/QA/PlanA.md` still outlines the original 10 points but now also narrates why our reordered flow keeps dependencies satisfied and reduces churn.
- `docs/HISTORY.md` is the drunken dwarf bard in the neon tavern; credit him for every build artifact, and the tale is updated with the current tooling and ordering.
- Every doc in `/docs` now carries either a Whimsical or Technical polish note so you know the tone before you read.

## 🔮 Next Steps (Execute, Record, Repeat)
1. Maintain the finished Phase 2 ledger: any follow-up is documentation, tests, or tooling, not another API shift.  
2. Keep using the `PhaseOutput` metadata, `PackageSpec` gating, and the `PhaseRunner`/`PhaseRegistry` pairing as the stable contract for CLI/TUI observers and future driver work.  
3. Run `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` from `/work/Mash-installer` for each change, capture the `.logs`, and only promote green runs to `main`.  
4. Once these signals stay calm, the ledger will flip Phase 3 from ❄️ to 🛠️ and the Pi 4B HDD ritual can finally begin.
