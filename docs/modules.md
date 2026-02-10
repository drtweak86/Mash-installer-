# Canonical Module Inventory

This document is a straight extraction of what `mash-setup install` currently does. Every installed package, script, and feature below is enumerated with its purpose, the distros that the distro drivers push it to, and whether it is implicit (profile/phase driven) or surfaced via a modal toggle. No behavior is changed yet; the goal is documentation-only inventory.

## Profiles

### Minimal profile (core system packages)
- **ca-certificates** – supplies TLS trust anchors so curl/wget/gh can verify HTTPS endpoints. Distros: Debian/Ubuntu (apt), Arch/Manjaro (pacman), Fedora (pacman); Visibility: implicit (minimal profile).  
- **curl** – downloads artefacts from GitHub, starship, Argon installs, etc. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **wget** – serves as a conservative download tool for scripts that expect it. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **xz-utils** – unpacks `.xz` archives from releases. Distros: Debian/Ubuntu, Arch/Manjaro (installs `xz`), Fedora; Visibility: implicit.  
- **tar** – unpacks tarballs as part of staging. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **coreutils** – provides basic Unix commands the installer relies on. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **jq** – lightweight JSON parser used by config helpers. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **git** – version control needed for local clones that other modules perform. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **software-properties-common** – provides `add-apt-repository` for Debian/Ubuntu before adding extra repos. Distros: Debian/Ubuntu only (Arch/Fedora drivers translate it to `None`); Visibility: implicit.  
- **gnupg** – imports GPG keys for Docker/GitHub CLI repos. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **lsb-release** – reads distro codenames for apt repo lines. Distros: Debian/Ubuntu only; Visibility: implicit.  
- **apt-transport-https** – enables HTTPS-based sources on Debian/Ubuntu. Distros: Debian/Ubuntu only; Visibility: implicit.  
- **build-essential** – installs GCC/make tooling on Debian/Ubuntu; Arch/Manjaro swaps it for `base-devel`, Fedora installs the canonical package. Distros: Debian/Ubuntu, Arch/Manjaro (via `base-devel`), Fedora; Visibility: implicit.  
- **pkg-config** – required by native builds; Arch/Manjaro installs `pkgconf`, Fedora uses the canonical name. Distros: Debian/Ubuntu, Arch/Manjaro (pkgconf), Fedora; Visibility: implicit.  
- **clang** – LLVM C compiler needed by some builds. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **lld** – LLVM linker that the toolchain assumes exists. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **cmake** – cross-platform build system generator. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **ninja-build** – high‑speed build backend; Arch/Manjaro installs `ninja`, Fedora keeps `ninja-build`. Distros: Debian/Ubuntu, Arch/Manjaro (ninja), Fedora; Visibility: implicit.  
- **gcc** – GNU compiler required by countless build scripts. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **g++** – GNU C++ compiler; Arch/Manjaro drops this entry (the driver returns `None`), while Debian/Ubuntu and Fedora install it by name. Distros: Debian/Ubuntu, Fedora; Visibility: implicit.  
- **gdb** – debugger for native debugging. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **make** – traditional build orchestrator. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  

### Developer profile additions (Dev & Full profiles)
- **python3** – Python interpreter required by scripts such as Argon One and buildroot helpers; Arch/Manjaro maps this to `python`, Fedora keeps `python3`. Distros: Debian/Ubuntu, Arch/Manjaro (python), Fedora; Visibility: implicit (Dev/Full).  
- **python3-pip** – `pip` installer for Python toolchains (`python-pip` on Arch/Manjaro); Fedora uses the canonical name. Distros: Debian/Ubuntu, Arch/Manjaro (python-pip), Fedora; Visibility: implicit (Dev/Full).  
- **python3-venv** – `venv` support for local virtual environments; Arch/Manjaro skips it entirely, Fedora leaves the canonical name unchanged. Distros: Debian/Ubuntu, Fedora (Arch/Manjaro drops it via translation); Visibility: implicit (Dev/Full).  
- **ripgrep** – fast recursive search used heavily in dotfiles and CLI workflows. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **fd-find** – `fd` search helper (`fd` on Arch/Fedora). Distros: Debian/Ubuntu, Arch/Manjaro (fd), Fedora (fd); Visibility: implicit (Dev/Full).  
- **fzf** – fuzzy finder for terminal interactions. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **tmux** – terminal multiplexer for resilient shells. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **htop** – interactive process monitor. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **ncdu** – CLI disk usage explorer. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **neovim** – modern editor provided for CLI editing sessions. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  

### Full profile additions
- **nodejs** – JavaScript runtime for optional frontend tooling and scripting helpers. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Full profile).  
- **npm** – Node package manager accompanying `nodejs`. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Full profile).  

### Automatically attempted optional packages
- **btop** – terminal-based resource monitor that the installer “tries optional” on Dev/Full profiles. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full, best-effort).  
- **bat** – syntax-highlighting `cat` alternative attempted on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **eza** – `ls` replacement attempted on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **yq** – YAML CLI helper attempted on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **lldb** – LLVM debugger tried every install (even minimal) via `try_optional`; Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (all profiles).  

## Buildroot dependencies (Dev & Full)
- **bison** – parser generator required for buildroot builds. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **flex** – lexical analyzer helper. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **gawk** – GNU Awk used by build scripts. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **texinfo** – document builder called by bootstraps. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **libncurses-dev** – terminal UI headers (`ncurses` on Arch, `ncurses-devel` on Fedora); Visibility: implicit (Dev/Full). Distros: Debian/Ubuntu, Arch/Manjaro (ncurses), Fedora (ncurses-devel).  
- **libssl-dev** – OpenSSL headers (`openssl` on Arch, `openssl-devel` on Fedora). Distros: Debian/Ubuntu, Arch/Manjaro (openssl), Fedora (openssl-devel); Visibility: implicit (Dev/Full).  
- **bc** – calculator used in build scripts. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **rsync** – mirrors artifacts into staging. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **cpio** – archive helper consumed by buildroot. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **unzip** – extracts ZIP payloads. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **file** – determines file formats; used in staging checks. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **patch** – applies diffs before building. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **python3**, **python3-pip**, **python3-venv** – reconfirmed here for buildroot even though they appear above (same distro applicability). Visibility: implicit (Dev/Full).  

## Rust toolchain & tooling (all profiles + Dev/Full extras)
- **rustup + stable toolchain** – installs Rust and keeps it updated for every profile. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **rustfmt** – formatting component added via `rustup component add`. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **clippy** – lint component added via `rustup component add`. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **rust-src** – source component for tooling and analysis. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **cargo-edit** (`cargo add`) – installed via `cargo install` on Dev/Full profiles. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **cargo-watch** – runs builds/tests on file changes; installed via `cargo install` on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **cargo-audit** – security scanner installed via `cargo install` on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **bacon** – workspace helper installed via `cargo install` on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **just** – command runner installed via `cargo install` on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **sccache** – distributed cache installed via `cargo install` on Dev/Full. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **flamegraph** – perf-based flamegraphs via `cargo install` on Full profile only. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Full).  

## Git, GitHub CLI & SSH
- **git** – source control backbone needed by nearly every module. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **git-lfs** – enables Git LFS workflows and runs `git lfs install` after installation. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **gh** (GitHub CLI) – installs via apt (Debian/Ubuntu with the configured repo) or via pacman (`github-cli` on Arch/Manjaro, `gh` on Fedora). Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **openssh-client** – SSH tooling required by GitHub CLI; Arch/Manjaro uses `openssh`, Fedora uses `openssh-clients`. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit.  
- **GitHub CLI apt repository (RepoKind::GitHubCli)** – Debian/Ubuntu-only repo that installs the GPG key at `/etc/apt/keyrings/githubcli-archive-keyring.gpg` and writes `/etc/apt/sources.list.d/github-cli-stable.list` before installing `gh`. Distros: Debian/Ubuntu; Visibility: implicit.  

## Docker & container runtime (Dev & Full)
- **Docker apt packages** (`docker-ce`, `docker-ce-cli`, `containerd.io`, `docker-buildx-plugin`, `docker-compose-plugin`) – installed after the Docker apt repo is configured. Distros: Debian/Ubuntu; Visibility: implicit (Dev/Full).  
- **Docker pacman packages** (`docker`, `docker-buildx`, `docker-compose`) – installed on Arch/Manjaro via translation; Fedora’s driver remaps the canonical names to `docker`, `containerd`, `docker-buildx`, and `docker-compose` before invoking pacman. Distros: Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **Docker apt repository (RepoKind::Docker)** – Debian/Ubuntu-only repo (key `/etc/apt/keyrings/docker.asc`, source `/etc/apt/sources.list.d/docker.list`) added before installing Docker packages. Distros: Debian/Ubuntu; Visibility: implicit.  
- **Docker group membership** – adds the invoking user to the `docker` group for socket access. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **Docker service enablement** – runs `systemctl enable --now` on the distro’s Docker unit. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **Docker data root management (module alias D)** – user-facing module (also exposed via `--docker-data-root`) that rewrites `/etc/docker/daemon.json` to point data-root to the staging directory or CLI-configured path, ensures the directory exists, and restarts Docker. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: user-facing (module D).  

## Shell & UX polish (Dev & Full)
- **zsh** – installs Z shell via the package manager. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **oh-my-zsh** – unattended `RUNZSH=no CHSH=no` install of the framework in the user’s home directory. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **starship prompt** – installs via the official script and appends `eval "$(starship init zsh)"` to `.zshrc`. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  
- **Powerlevel10k prompt (module alias P)** – user-facing toggle; Arch/Manjaro first tries to install the `zsh-theme-powerlevel10k` package, and other distros clone https://github.com/romkatv/powerlevel10k into `/usr/share/powerlevel10k` before adding a guarded `source` block to `.zshrc`. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: user-facing (module P / `--enable-p10k`).  

## Fonts (Dev & Full)
- **fonts-terminus** – installs the monospace Terminus font. Distros: Debian/Ubuntu, Arch/Manjaro (terminus-font), Fedora; Visibility: implicit (Dev/Full).  
- **fonts-noto-color-emoji** – installs Noto Color Emoji (Arch/Manjaro → `noto-fonts-emoji`). Distros: Debian/Ubuntu, Arch/Manjaro (noto-fonts-emoji), Fedora; Visibility: implicit (Dev/Full).  
- **xfonts-terminus** – legacy X11 Terminus glyphs (Arch/Manjaro drops it via translation). Distros: Debian/Ubuntu, Fedora; Visibility: implicit (Dev/Full).  
- **Nerd Fonts note** – the installer logs that patched fonts are not shipped automatically and points users to https://www.nerdfonts.com/ for manual installation; Visibility: implicit message (Dev/Full).  

## Sync & utility tools
- **rclone** – tries to install via the package manager and falls back to `curl -fsSL https://rclone.org/install.sh | sudo bash` if the package is unavailable. Distros: Debian/Ubuntu, Arch/Manjaro, Fedora; Visibility: implicit (Dev/Full).  

## Raspberry Pi / hardware-specific modules
- **Argon One fan control (module alias A)** – user-facing, Pi-only option (skips entirely if `pi_model` is `None`). Debian/Ubuntu runs Argon40’s OEM script (`curl -fsSL https://download.argon40.com/argon1.sh | bash`), while Arch/Manjaro installs the `dtc` dependency, clones https://gitlab.com/DarkElvenAngel/argononed.git into `/usr/local/src/argononed`, builds it, and enables the distro’s Argon One service (via `systemctl enable argononed.service`). Distros: Debian/Ubuntu, Arch/Manjaro; Visibility: user-facing (module A / `--enable-argon`).  

## Interactive module toggles
- **Module aliases A/P/D** – the interactive “Select modules” menu explains `A` for Argon One, `P` for Powerlevel10k, and `D` for Docker data-root management. Non-interactive installs default to `ModuleSelection::default()` (all toggles off) while the “full install” choice flips all of them on. The CLI flags `--enable-argon`, `--enable-p10k`, and `--docker-data-root` expose the same toggles in scripts. Distros: affects whichever distro driver is selected; Visibility: user-facing toggles.  
