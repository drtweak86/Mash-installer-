# mash-installer workspace architecture
> **Neon Chronicle (Technical polish)**: ARCH keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


 - `installer-cli` is now the thin binary that bootstraps the interactive menu flow, autodetects the distro via `/etc/os-release`, and calls into `installer-core` with the selected driver and options.
 - `installer-core` is the reusable library that owns ABB backups, deterministic downloads, logging helpers, and module/profile orchestration; it exposes `run_with_driver` along with the `InstallOptions`/`ProfileLevel` types.
 - `installer-arch`, `installer-fedora`, and `installer-debian` are the distro driver crates implementing the `DistroDriver` trait, allowing the CLI to swap in distro-specific behaviors while keeping the core invariant-safe.

## Goals
1. Keep the CLI thin: it handles argument parsing, tracing setup, the three-stage menu (auto vs select distro, modules, profile), and driver selection.
2. Let `installer-core` own installation invariants so other UIs (for example a GUI or tests) can reuse the same phases and logging.
3. Distro drivers implement `DistroDriver` so each platform exposes metadata and backend hints without coupling to the core crate.
4. Module toggles are aliased (A=Argon, P=Powerlevel10k, D=Docker data-root) in the CLI menu, and each `DistroDriver` provides package translations, repository metadata, and service names so the core phases can run the same pipeline across Debian/Arch/Fedora.

## PlatformContext helpers

- `PlatformContext::is_pi`, `::pi_generation`, `::is_pi_4b`, and `::supports_usb3` codify Raspberry Pi detection so phases (Argon One, Hyprland, Docker data-root) can ask for clean boolean answers instead of re-parsing strings. These helpers are part of R-07 and keep the context ready for Pi 4B-specific wiring before Phase 3 starts.

## Next steps
- Add automated tests that verify driver translations/repo hooks and the module selection alias flow, keeping the pipeline safe and reproducible.
- Keep README/docs/ARCH updated whenever new module toggles or driver hooks are introduced.
