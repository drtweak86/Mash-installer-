# Curated Program Options
> **Neon Chronicle (Whimsical polish)**: curated-options invites the bored bard to braid cyberpunk sigils with Tolkien smoke, so the instructions sing with neon and dice. 🌌🎲


This menu recommends one default plus four alternatives per category so you can pick the stack that fits your Pi/SBC workflows.

## Shell & terminal
Default: `zsh` — mature, scriptable, and widely supported by frameworks like Oh My Zsh and Powerlevel10k.
`fish` — smart autocompletion and sane defaults when you want configuration-free modernity.
`bash` — the smallest common denominator for scripting, automation, and rescue shells.
`pwsh` — PowerShell Core brings cross-platform object pipelines if you need Windows-like scripts or Azure tooling.
`elvish` — expressive data structures and shared history make it ideal for tinkering without losing context.

## Editors
Default: `neovim` — fast over SSH, great on Pi, huge ecosystem.
`micro` — “nano but modern,” perfect when you just need to edit a file.
`helix` — modern modal editor with few plugins and sensible defaults out of the box.
`vim` — smallest common denominator, always available, perfect for minimal recoveries.
`code-server` — VS Code in the browser when you want full IDE features on a phone or tablet.

## Dev toolchains
Default: `rustup` — bump-free Rust toolchain plus cargo helpers (watch, fmt, clippy) for any project.
`python3` — ubiquitous interpreter for scripts, build helpers, and automation on Pi and SBCs.
`nodejs` — Web tooling, TypeScript builds, and npm-powered CLIs run natively on modern builds.
`build-essential` / `base-devel` — barebones GCC/make chain for C/C++ projects that still ship on many boards.
`go` — Go compiler is fast to install and handles small cross-platform utilities or networking daemons.

## Containers
Default: `docker` — widest ecosystem, tutorials, and muscle memory.
`podman` — daemonless, rootless-friendly, “Linux-native” vibe with Docker-compatible CLI.
`nerdctl + containerd` — lightweight stack closer to Kubernetes/containerd world.
`lxc`/`lxd` — system containers that feel like mini VMs, great for isolation without full virtualization.
`distrobox` — not a runtime, but fantastic for disposable distro-specific dev environments inside a container.

## Git & GitHub workflows
Default: `git` — rock-solid source control that every workflow assumes is installed.
`gh` — GitHub CLI for PRs, issues, and automation straight from the terminal.
`git-lfs` — manage large media files without bloating the repo, especially useful for offline Pi builds.
`lazygit` — TUI for staging, rebasing and history browsing when you prefer visual helpers.
`forge` — GitHub-native multiplexer for people who need `gh` + `git` in one assisted experience.

## System monitoring
Default: `htop` — simple process tree with CPU/memory bars you can trust on Pi boards.
`btop` — prettier stats, graphs, and widgets when you crave a dashboard feel.
`glances` — cross-platform, exposes metrics via API/HTML for remote monitoring sessions.
`nmon` — lightweight and scriptable, handy when zoned into troubleshooting on older SBCs.
`bpytop` — Python-based, fancier output if you want a slick, configurable console UI.

## File management
Default: `nnn` — insanely fast navigator with optional file previews and plugin hooks.
`ranger` — vim-style interface that shines when you like directory trees and multi-column views.
`mc` — Midnight Commander remains familiar to admins who grew up on console file dialogs.
`lf` — minimal, lua-flagged file manager that leans heavily on shell pipeability.
`tmsu` — tags instead of directories, perfect for messy media collections on SD cards.

## Networking & remote
Default: `openssh` — ubiquitous SSH client/server stack for remote shells and file copies.
`dropbear` — tiny SSH implementation when you need a lightweight footprint on resource-constrained boards.
`mosh` — mobile-optimized remote shells that survive flaky Wi-Fi and roaming IPs.
`tmate` — shareable SSH session for pair programming or remote debugging without complex VPNs.
`sshfs` — mount remote filesystems transparently and work on them like local files.

## Backups & sync
Default: `rsync` — battle-tested sync for SD cards, thumb drives, and over-the-wire copies.
`rclone` — cloud-backed sync that speaks s3, GDrive, and many other remotes when you need offsite storage.
`borg` — deduplicating backup with compression and encryption that protects repeated builds.
`restic` — easy snapshots with built-in verification, ideal for immutable Pi images.
`duplicity` — incremental backups over SSH/FTP when you want GPG-encrypted archives.

## Pi/SBC hardware extras
Default: `argononed` fan control (Argon One module) — tuned fan curves plus service integration tailored to Argon cases.
`pigpio` — background daemon for PWM, hardware PWM, and DMA-driven GPIO toggling.
`gpiozero` — beginner-friendly Python interface for buttons, LEDs, and sensors on a Pi.
`libgpiod` — chardev-based GPIO tooling when you need modern kernel interfaces instead of sysfs.
`RPi.GPIO` — classic Python library kept around for legacy scripts and teaching examples.