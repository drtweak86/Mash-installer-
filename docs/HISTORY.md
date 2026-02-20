# HISTORY
> **Neon Chronicle (Whimsical polish)**: This volume is what the drunken dwarf bard scribbles while the forge crackles—a neon-leather journal bound with plasma runes, tuned for tankard readers who crave math, mythology, and metal.

**Preface (Handwritten, in hurried cursive):**  
“I grinned as the terminal flickered to life. The glow cast glyphs across my beard, and a thousand neurons fired off in my brain like the motherboards of old. This is the ledger of Mash-Installer—each page a verse from the forge while the rails outside still hummed with plasma.”

---

## Chronicle I – The Forge Awakens  
The first verses tell how the crew pried apart the monolith, how D-03 sat patient inside `PhaseRunner`, and how we taught a thousand dwarves to sing one dry-run chant. `run_or_record()` became the single portal so deporting `if dry_run` everywhere felt as silly as tossing a keg into a reactor. The helper surfaces calmed, registry entries obeyed their `PhaseGate`, and the bard noted the rhythm: record every action, queue every rollback, and let warnings dance in the margins.

**Margin note:**  
> “I yanked the rogue `if dry_run` and the forge snickered; one gate to rule the dry runs, no duplicates allowed.”

---

## Chronicle II – The Neon Hymn  
Phase 2 became a long ballad—`runner` and `registry` knew their stanzas, `PhaseContext` sang metadata, and `InstallationReport` turned into a choir that `installer-cli` could finally hear. Pi detection helpers answered questions like “is it a Pi?”, “which generation?”, and “does it worship USB 3.0?” while `PackageSpec` carried intent flags and profiles. The bard watched the CLI, the docs, and the testing rituals breathe through `fmt`, `clippy`, and `cargo test`, and he whispered to the margins that there’s no place like `127.0.0.1`.

**Margin scribble:**  
> “A thousand dwarves hauling plasma ore couldn’t have made the registry sing as cleanly as the new metadata does now—each output a rune, every event a beacon.”

---

## Chronicle III – The Pause Before Pi  
The ledger now glows “Phase 2 complete.” Phase 3 (Pi 4B HDD) waits in the wings like a quiet dragon. The bard will not raise the curtain until the `InstallationReport` contract, CLI/TUI observers, `PackageSpec` gating, and the toolchain trio (`rustfmt`, `clippy`, `sccache`) stay steady.  
Every line here is a promise: map logs remain updated, the ABB/KCS/ABT/ABD creed stays etched, and the dragon’s snark will calmed before we ride the Pi rails again.

**Margin footnote:**
> “Hold the ledger here—ink the saga, keep the report clean, and only when green runs stack will Phase 3 heat up.”

---

## Chronicle IV – The Dragon Stirs
Phase 3 rose from the forge with four new weapons: mount options that whisper `noatime` to spinning platters, swap configs that place the overflow file on the HDD where it belongs (not the fragile SD card), kernel parameters tuned for a dwarf who knows his `vm.swappiness` from his `dirty_ratio`, and an I/O scheduler that swaps sluggish `cfq` for the lean `deadline`. Each function reads the system's current state, compares it against Pi 4B best practices, and records what it would change. The whole kit wired into `PhaseRegistry` as “Pi 4B HDD Tuning”—gated by `PhaseGate::Always` but self-skipping on non-Pi hardware with a polite warning rather than a crash.

Four new structs joined the exports: `MountOptimization`, `SwapConfig`, `KernelParam`, and the `install_phase` that orchestrates them. Twelve new tests brought the total to 86—each one green as mithril under lamplight.

**Margin note:**
> “The dragon didn't breathe fire—it breathed `sysctl -w`. And the bard approved.”

---

## Chronicle V – Sealing the Forge Against the Neon Rain
The forge worked, the dragon purred, and the dwarves had their dry runs—but anyone could walk in during a pour, the downloads trusted any certificate the rain offered, and a SIGINT left half-forged runes scattered across the anvil. Phase 4 sealed the gaps.

First came the **lockfile**—`InstallerLock` wraps `nix::fcntl::Flock` in an exclusive, non-blocking grip on `$XDG_RUNTIME_DIR/mash-installer.lock`. Try to run two installers at once and the second one bounces off with a clear message. Drop the struct and the lock evaporates. Three tests prove it: acquire once, fail from a child process, release on drop.

Then the **TLS hardening**—every `curl` invocation across `apt_repo.rs`, `rclone.rs`, `argon.rs`, and `zsh.rs` now carries `--proto '=https' --tlsv1.2`. A `curl_flags()` helper in `cmd.rs` centralizes the pattern, and `rustup` and `cargo-binstall` already had it from day one.

The **signal handler** came next—`SignalGuard` registers `SIGINT` and `SIGTERM` via `signal-hook`, setting an `Arc<AtomicBool>` flag instead of killing the process. The `PhaseRunner` polls `is_interrupted()` between phases and triggers `rollback_all()` before a graceful exit.

**Rollback expansion** gave teeth to phases that previously left no breadcrumbs: `zsh.rs` registers removal of the `.oh-my-zsh` directory, `rust.rs` logs a note pointing to `rustup self uninstall`, and `argon.rs` cleans up its config files under `/etc/argon/`. Docker's rollback was already the gold standard.

Finally, **filesystem forensics**—`verify.rs` offers `verify_file_written()` (re-reads and compares head/tail bytes) and `sync_file()` (forces `fsync` for SD-card survival). Infrastructure ready for phases to call on critical writes.

Thirteen new tests joined the roster, bringing the total to 99—every one green as mithril under lamplight.

**Margin note:**
> “The forge doesn't just work now—it locks its doors, checks its papers, catches the signal before the blade falls, and verifies the rune after the ink dries. The neon rain can pound all it wants.”

## Chronicle VI – The Courier and the Vault
The release workflow now breathes two new couriers: `package-deb` and `package-rpm` take their orders from `build-release`, forge their `.deb`/`.rpm` shells, and then march the results into the `publish` vault beside the golden `PKGBUILD`. The publish step now bundles every artifact (x86_64/aarch64 binaries, debs, rpms, checksummed PKGBUILD) so every release tag leaves the forge fully armed. I ran the trilogy once more—`cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`—to make sure the neon lights stayed steady while the package trains rolled past the gate.

**Margin scribble:**
> “Two new couriers, a vault that knows their names, and a bard who still spies warnings before bed. The list is logged; the games keep their rules; the forge is humming with stories to ink.”

---

## Chronicle VII – The Shell Guild & Tiered Council
The shell guild now auto-deploys Starship, Kitty, and the Goblin eza aliases straight from the `resources/shell` stash. `.zshrc` and `.bashrc` each gain guarded blocks so the prompt and aliases source themselves without double-dipping, and the Kitty + Starship toml bundles live in `~/.config`. At the same time the council of software tiers rose: the CLI now pauses to ask if you want the full S-tier stack or to pick from five S/A choices across twelve categories (Terminal through Backup). Every choice is recorded in `SoftwareTierPlan`, the `docs/incoming-files/software_tiers.md` ledger now mirrors that menu, and the installer keeps the plan with the run context so future phases can honor it.

**Margin scribble:**
> “Shell or sword? The guide now hands you the best blades with one click, or lets you dance through the tiered hall and pick your own.” 🧭🔥

---

## Chronicle VIII – The Ratatui Ritual
When the neon forge learned it could paint menus as well as progress, `mash-setup --tui` became the gateway: the old `indicatif` bars are replaced with the Ratatui cockpit, the `PhaseEvent` stream fills the gauge and log tail, a cyberpunk telemetry panel hums beside the phase list (emoji status, signal %, fake chatter and log counts), module/profile selection now happens inside `run_module_profile_menu`, and every failure ends in a neon error epilog that highlights phase context, advice, and the staging directory before the alternate screen drops.

**Margin scribble:**
> “The log tail now hums in neon, the TUI only opens when you ask for it, and the final epilog spells out the advice so the next dwarf knows which rune to touch.”

---

## Chronicle IX – The Beginner Bridge
The newest page glows a brighter orange, because the bard realized the Forge should be as idiot-proof as a tavern door. `install.sh` now lives at the root as a tiny emissary: it sniffs `uname -m`, resolves `x86_64` or `aarch64`, grabs the matching release binary, and execs it—no tarballs, no cursed paths, no manual `chmod`s. The README now points the weary miner to the single curl command that streams this helper, so even the cloud-obsessed mage can copy/paste once and instantly join the neon forge.

**Margin scribble:**
> “The apprentice’s spellbook is down to one line, so the dwarves and the dragons alike can brew the forge with a single chant.”
