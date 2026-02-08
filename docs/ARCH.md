# mash-installer workspace architecture

- `installer-cli` is the thin binary crate that bootstraps the installer: it simply links against `installer-core`, prints errors, and keeps CLI-specific wiring (menus, autodetect, user prompts) outside the reusable core logic.
- `installer-core` contains the former `mash-setup` logic, refactored into a reusable library that owns ABB backups, deterministic downloads, module/profile types, and exposes a single `run()` entrypoint for the CLI.
- `installer-arch`, `installer-fedora`, and `installer-debian` are dedicated distro-driver crates that will implement the `DistroDriver` trait (planned in `installer-core`) to encapsulate package-manager commands, repository configuration, and distro-specific quirks.

## Goals
1. Keep the CLI thin: it should only worry about argument parsing, menu flow, and selecting the appropriate distro driver (auto-detection vs. explicit selection).
2. Let the core crate own installation invariants, logging, and module composition so we can reuse it from other interfaces in the future.
3. Distro drivers expose traits consumed by `installer-core`, ensuring Arch/Manjaro remain fully supported while Fedora/Debian crates can be incrementally implemented.

## Next steps
- Implement the `DistroDriver` trait in `installer-core` and wire the CLI to select between installed distro crates.
- Move `pkg`, `docker`, and other modules into core/internal traits so distro-specific behavior can be injected cleanly.
- Expand `README.md` and other docs to describe the multi-crate workspace layout.
