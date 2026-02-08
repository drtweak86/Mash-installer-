# mash-installer workspace architecture

 - `installer-cli` is now the thin binary that bootstraps the interactive menu flow, autodetects the distro via `/etc/os-release`, and calls into `installer-core` with the selected driver and options.
 - `installer-core` is the reusable library that owns ABB backups, deterministic downloads, logging helpers, and module/profile orchestration; it exposes `run_with_driver` along with the `InstallOptions`/`ProfileLevel` types.
 - `installer-arch`, `installer-fedora`, and `installer-debian` are the distro driver crates implementing the `DistroDriver` trait, allowing the CLI to swap in distro-specific behaviors while keeping the core invariant-safe.

## Goals
1. Keep the CLI thin: it handles argument parsing, tracing setup, the three-stage menu (auto vs select distro, modules, profile), and driver selection.
2. Let `installer-core` own installation invariants so other UIs (for example a GUI or tests) can reuse the same phases and logging.
3. Distro drivers implement `DistroDriver` so each platform exposes metadata and backend hints without coupling to the core crate.

## Next steps
- Continue fleshing out distro drivers (pkg repos, service helpers) so Fedora and Debian can exercise their own backends.
- Expose `DistroDriver`-specific hooks inside `installer-core` (pkg repositories, service helpers) so the core phases use the selected driver when needed.
- Document the workspace layout, menu flow, and driver expectations in README and release notes.
