#!/usr/bin/env python3
"""
╔══════════════════════════════════════════════════════════════════════════╗
║                                                                          ║
║   🧠  PI OVERLORD GRIMOIRE  —  Complete One-Shot Installer               ║
║       FrankenPi Overlord Mode  ×  The Green Grimoire                    ║
║                                                                          ║
║   Raspberry Pi 4/5  ·  Fedora ARM  ·  KDE Plasma X11  ·  Btrfs         ║
║   Phosphor Green Terminal  ·  Rust-first  ·  Full Dev Stack             ║
║                                                                          ║
╚══════════════════════════════════════════════════════════════════════════╝

  WHAT THIS DOES
  ──────────────
  Fire-and-forget, idempotent, single-shot installer.
  Run it once. Walk away. Come back to a fully configured machine.

    Phase  0  Preflight   — detect user, check internet, dnf upgrade
    Phase  1  Core tools  — monitors, search, compression, networking CLI
    Phase  2  KDE Plasma  — KDE Plasma + SDDM, X11 forced (Pi 4B stable)
    Phase  3  Kitty       — install terminal emulator
    Phase  4  Fonts       — Nerd Fonts (git clone + install.sh), JetBrains, Fira
    Phase  5  Zsh shell   — Oh-My-Zsh + Powerlevel10k, set as default shell
    Phase  6  Rust        — rustup, stable toolchain, clippy, rustfmt
    Phase  7  Cargo       — cargo-watch, cargo-edit, cargo-audit, just, bacon
    Phase  8  Build deps  — gcc, clang, cmake, ninja, make, pkg-config, dev libs
    Phase  9  Git forge   — gh, git-lfs, lazygit, delta, tig
    Phase 10  Containers  — Podman, Buildah, Skopeo, podman-compose
    Phase 11  Filesystem  — btrfs-progs, Snapper config, golden snapshot
    Phase 12  Network     — nmap, iperf3, WireGuard, Tailscale, firewalld, fail2ban
    Phase 13  Multi-lang  — Python3, Node.js, Go, Lua, shellcheck
    Phase 14  Performance — earlyoom, tuned (balanced), irqbalance, zram, sysctl
    Phase 15  Wayland     — wl-clipboard, grim, slurp, direnv, flatpak, helix
    Phase 16  Appearance  — papirus icons, neovim, fastfetch, eza, atuin, zoxide
    Phase 17  Dotfiles    — write all config files (kitty, zshrc, nvim, fastfetch)
    Phase 18  Argon One   — argononed daemon (fan + power button for Argon One case)
    Phase 19  Services    — enable SDDM + all systemd units, fix ownership

  HOW TO RUN
  ──────────
    sudo python3 pi_overlord_grimoire.py
    sudo python3 pi_overlord_grimoire.py --dry-run
    sudo python3 pi_overlord_grimoire.py --phases 1,2,3
    sudo python3 pi_overlord_grimoire.py --skip-cargo

  Safe to re-run — already installed/configured things are skipped.
"""

import argparse
import os
import pwd as _pwd
import shutil
import subprocess
import sys
import time
from pathlib import Path

# =============================================================================
#  COLOUR HELPERS
# =============================================================================

RESET  = "\033[0m"
BOLD   = "\033[1m"
DIM    = "\033[2m"
RED    = "\033[31m"
GREEN  = "\033[32m"
YELLOW = "\033[33m"
CYAN   = "\033[36m"
WHITE  = "\033[37m"

# =============================================================================
#  GLOBALS  (populated in phase_0_preflight)
# =============================================================================

DRY_RUN   = False
REAL_USER = ""
REAL_HOME = Path("/root")
LOG_FILE  = Path("/var/log/pi_overlord_grimoire.log")

# =============================================================================
#  OUTPUT / LOGGING
# =============================================================================

def _log(msg: str) -> None:
    try:
        with LOG_FILE.open("a") as f:
            f.write(f"[{time.strftime('%H:%M:%S')}] {msg}\n")
    except Exception:
        pass


def banner(text: str) -> None:
    print(f"\n{BOLD}{CYAN}{'━' * 66}{RESET}")
    print(f"{BOLD}{CYAN}  ▸  {text}{RESET}")
    print(f"{BOLD}{CYAN}{'━' * 66}{RESET}")
    _log(f"=== {text} ===")


def ok(msg: str)    -> None: print(f"  {GREEN}✔{RESET}  {msg}");             _log(f"OK   {msg}")
def info(msg: str)  -> None: print(f"  {CYAN}→{RESET}  {msg}");              _log(f"INFO {msg}")
def warn(msg: str)  -> None: print(f"  {YELLOW}⚠{RESET}  {msg}");            _log(f"WARN {msg}")
def skip(msg: str)  -> None: print(f"  {DIM}⊘  {msg} — already done{RESET}"); _log(f"SKIP {msg}")
def err(msg: str)   -> None: print(f"  {RED}✘{RESET}  {msg}", file=sys.stderr); _log(f"ERR  {msg}")
def fatal(msg: str) -> None: err(msg); sys.exit(1)

# =============================================================================
#  EXECUTION HELPERS
# =============================================================================

def run(
    cmd: list,
    *,
    check: bool = True,
    capture: bool = False,
    as_user: bool = False,
    env_extra: dict | None = None,
) -> subprocess.CompletedProcess:
    """Execute cmd, honouring --dry-run and optional user-switching."""
    display = " ".join(str(c) for c in cmd)

    if DRY_RUN:
        print(f"  {DIM}[dry]{RESET} {display}")
        return subprocess.CompletedProcess(cmd, 0, stdout="", stderr="")

    env = {**os.environ, **(env_extra or {})}

    actual_cmd = cmd
    if as_user and REAL_USER and REAL_USER != "root":
        actual_cmd = ["sudo", "-u", REAL_USER, "--"] + list(cmd)

    result = subprocess.run(
        actual_cmd,
        check=False,
        capture_output=capture,
        text=True,
        env=env,
    )

    if check and result.returncode != 0:
        err(f"Command failed (rc={result.returncode}): {display}")
        if result.stderr:
            print(result.stderr.strip(), file=sys.stderr)
        sys.exit(result.returncode)

    return result


def shell(script: str, *, check: bool = True) -> subprocess.CompletedProcess:
    """Run a bash script string as root."""
    return run(["bash", "-c", script], check=check)


def cmd_exists(name: str) -> bool:
    return shutil.which(name) is not None


# =============================================================================
#  DNF HELPERS
# =============================================================================

def dnf_install(packages: list, *, label: str = "") -> None:
    if not packages:
        return
    desc = label or f"{len(packages)} package(s)"
    info(f"dnf install → {desc}")
    run([
        "dnf", "install", "-y",
        "--skip-unavailable",
        "--setopt=install_weak_deps=False",
    ] + packages)


def dnf_copr(repo: str) -> None:
    info(f"Enabling COPR: {repo}")
    run(["dnf", "copr", "enable", "-y", repo], check=False)

# =============================================================================
#  SYSTEMD / SYSCTL HELPERS
# =============================================================================

def systemctl_enable(unit: str) -> None:
    r = run(["systemctl", "is-enabled", unit], check=False, capture=True)
    if r.returncode == 0 and "enabled" in (r.stdout or ""):
        skip(f"unit: {unit}")
        return
    run(["systemctl", "enable", "--now", unit], check=False)
    ok(f"Enabled + started: {unit}")


def sysctl_set(key: str, value: str, conf: str) -> None:
    path = Path(conf)
    entry = f"{key}={value}\n"
    if path.exists() and entry in path.read_text():
        skip(f"sysctl {key}")
        return
    if not DRY_RUN:
        path.parent.mkdir(parents=True, exist_ok=True)
        # Append to the file so multiple calls don't overwrite each other
        with path.open("a") as f:
            f.write(entry)
    run(["sysctl", "--system"], check=False)
    ok(f"sysctl {key}={value}")

# =============================================================================
#  FILE WRITE HELPERS
# =============================================================================

def write_file(path: Path, content: str, *, fix_owner: bool = True) -> None:
    """Write dedented content to path. Creates parent dirs. Idempotent."""
    if DRY_RUN:
        info(f"[dry] would write: {path}")
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    # Dedent so we can use indented triple-quoted strings in code
    import textwrap
    path.write_text(textwrap.dedent(content))
    if fix_owner and REAL_USER and REAL_USER != "root":
        run(["chown", f"{REAL_USER}:{REAL_USER}", str(path)], check=False)
    ok(f"Wrote: {path}")


def append_once(path: Path, marker: str, content: str, *, fix_owner: bool = True) -> None:
    """Append content only if marker is not already in the file."""
    if DRY_RUN:
        info(f"[dry] would patch: {path}")
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    existing = path.read_text() if path.exists() else ""
    if marker in existing:
        skip(f"patch already present: {path.name}")
        return
    import textwrap
    with path.open("a") as f:
        f.write("\n" + textwrap.dedent(content) + "\n")
    if fix_owner and REAL_USER and REAL_USER != "root":
        run(["chown", f"{REAL_USER}:{REAL_USER}", str(path)], check=False)
    ok(f"Patched: {path}")


def git_clone_or_pull(url: str, dest: Path, *, depth: int = 1, as_user: bool = False) -> None:
    depth_args = ["--depth", str(depth)] if depth else []
    if dest.exists():
        info(f"Updating: {dest.name}")
        run(["git", "-C", str(dest), "pull", "--rebase", "--autostash"],
            check=False, as_user=as_user)
    else:
        info(f"Cloning: {url}")
        run(["git", "clone"] + depth_args + [url, str(dest)], as_user=as_user)

# =============================================================================
#  PHASE 0 — PREFLIGHT
# =============================================================================

def phase_0_preflight() -> None:
    banner("Phase 0 — Preflight: User Detection · Checks · System Upgrade")

    global REAL_USER, REAL_HOME

    # ── Must be root ──────────────────────────────────────────────────────────
    if not DRY_RUN and os.geteuid() != 0:
        print(f"""
  {RED}{BOLD}This script needs administrator privileges.{RESET}

  Please re-run it like this:

      {BOLD}sudo python3 pi_overlord_grimoire.py{RESET}

  (sudo means "run as administrator" — it will ask for your password once)
""")
        sys.exit(1)

    # ── Detect the real human user (not root) ─────────────────────────────────
    sudo_user = os.environ.get("SUDO_USER", "")
    if sudo_user and sudo_user != "root":
        try:
            pw = _pwd.getpwnam(sudo_user)
            REAL_USER = pw.pw_name
            REAL_HOME = Path(pw.pw_dir)
        except KeyError:
            REAL_USER = sudo_user
            REAL_HOME = Path(f"/home/{sudo_user}")
    else:
        pw = _pwd.getpwuid(os.getuid())
        REAL_USER = pw.pw_name
        REAL_HOME = Path(pw.pw_dir)

    info(f"Installing for user : {BOLD}{REAL_USER}{RESET}")
    info(f"Home directory      : {REAL_HOME}")

    # ── Must be Fedora ────────────────────────────────────────────────────────
    if not DRY_RUN:
        os_release = Path("/etc/os-release").read_text() if Path("/etc/os-release").exists() else ""
        if "fedora" not in os_release.lower():
            warn("This script targets Fedora. Other distros may need package name changes.")
        else:
            ok("Fedora detected")

    # ── Internet check ────────────────────────────────────────────────────────
    info("Checking internet connectivity...")
    r = run(["curl", "-sf", "--max-time", "10", "https://fedoraproject.org"],
            check=False, capture=True)
    if r.returncode != 0 and not DRY_RUN:
        fatal("No internet connection detected. Please connect and retry.")
    ok("Internet: OK")

    # ── DNF plugins (needed for copr) ─────────────────────────────────────────
    info("Ensuring dnf-plugins-core is present...")
    run(["dnf", "install", "-y", "--skip-unavailable", "dnf-plugins-core"], check=False)

    # ── Enable RPM Fusion free + nonfree ─────────────────────────────────────
    info("Enabling RPM Fusion repositories...")
    shell(
        'dnf install -y --skip-unavailable '
        '"https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm" '
        '"https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm"',
        check=False,
    )
    ok("RPM Fusion: enabled")

    # ── Full system upgrade ───────────────────────────────────────────────────
    info("Refreshing DNF metadata...")
    run(["dnf", "makecache", "--refresh"])

    info("Upgrading all packages — this may take several minutes...")
    run(["dnf", "upgrade", "-y"])
    ok("System fully up to date")

    ok("Phase 0 complete")

# =============================================================================
#  PHASE 1 — CORE CLI TOOLS
# =============================================================================

def phase_1_core_tools() -> None:
    banner("Phase 1 — Core CLI Tools")
    dnf_install([
        # Monitors
        "btop", "htop", "nvtop", "iotop", "powertop",
        # Hardware inspection
        "lm_sensors", "smartmontools", "usbutils", "pciutils",
        # Diagnostics
        "lsof", "strace", "tcpdump", "bind-utils",
        # Modern file tools
        "ncdu", "tree", "ripgrep", "fd-find", "bat", "fzf", "eza",
        # Download / transfer
        "wget", "curl", "aria2", "rsync",
        # Multiplexers
        "tmux", "screen",
        # Compression
        "zstd", "unzip", "p7zip", "p7zip-plugins",
        # Shell utilities
        "which", "util-linux-user", "pv", "jq",
    ], label="core CLI tools")
    ok("Phase 1 complete")

# =============================================================================
#  PHASE 2 — KDE PLASMA (X11, Pi 4B STABLE)
# =============================================================================

def phase_2_kde() -> None:
    banner("Phase 2 — KDE Plasma Desktop (X11)")

    # Install the full KDE Plasma group — this is the cleanest way on Fedora
    # and pulls in Plasma shell, KWin (X11), Dolphin, Konsole, etc.
    info("Installing KDE Plasma group via dnf...")
    run(["dnf", "group", "install", "-y", "--skip-unavailable",
         "KDE Plasma Workspaces"], check=False)

    dnf_install([
        # ── Display manager ────────────────────────────────────────────────
        "sddm",                    # KDE's native DM — lighter than GDM
        # ── X11 stack (explicit, avoids accidental Wayland pull-in) ────────
        "xorg-x11-server-Xorg",
        "xorg-x11-xinit",
        "xorg-x11-utils",          # xrandr, xdpyinfo
        # ── KDE extras not always in the group ─────────────────────────────
        "plasma-workspace",
        "plasma-desktop",
        "kwin",                    # window manager
        "kdeplasma-addons",        # extra widgets / applets
        "plasma-nm",               # Network Manager KDE applet
        "plasma-pa",               # PulseAudio KDE applet
        "kscreen",                 # display configuration
        "powerdevil",              # power management
        "bluedevil",               # Bluetooth (optional but expected)
        "kate",                    # KDE advanced text editor
        "dolphin",                 # file manager
        "ark",                     # archive manager
        "spectacle",               # screenshot tool
        "gwenview",                # image viewer
        "okular",                  # document viewer
        "kcalc",                   # calculator
        # ── Useful extras ─────────────────────────────────────────────────
        "playerctl",               # media key / polybar support
        "xdg-utils",
        "xdg-user-dirs",
        "network-manager-applet",  # nm-applet fallback in non-Plasma contexts
    ], label="KDE Plasma + X11 stack")

    # Force SDDM to use X11 session (prevent accidental Wayland on Pi 4B)
    sddm_conf = Path("/etc/sddm.conf.d/99-pi-x11.conf")
    if not sddm_conf.exists():
        write_file(sddm_conf, """\
            [General]
            DisplayServer=x11

            [X11]
            SessionDir=/usr/share/xsessions
        """, fix_owner=False)
        ok("SDDM forced to X11 session")
    else:
        skip("SDDM X11 config")

    ok("Phase 2 complete — SDDM enabled in Phase 18")



# =============================================================================
#  PHASE 3 — KITTY
# =============================================================================

def phase_3_kitty() -> None:
    banner("Phase 3 — Kitty Terminal")
    dnf_install(["kitty"], label="kitty terminal")
    ok("Phase 3 complete — config written in Phase 17")

# =============================================================================
#  PHASE 4 — FONTS (NERD FONTS via git clone + install.sh)
# =============================================================================

def phase_4_fonts() -> None:
    banner("Phase 4 — Fonts (Nerd Fonts + Distro Fonts)")

    # Distro packaged fonts
    dnf_install([
        "jetbrains-mono-fonts",
        "fira-code-fonts",
        "fontconfig",
    ], label="distro fonts")

    # Nerd Fonts — official repo install.sh method
    # Uses --filter=blob:none + sparse checkout so we only pull what we need
    # install.sh copies fonts to /usr/local/share/fonts when run as root
    nf_dir = Path("/opt/nerd-fonts")

    if not (nf_dir / "install.sh").exists():
        info("Cloning Nerd Fonts repository (sparse, shallow)...")
        run(["git", "clone",
             "--depth", "1",
             "--filter=blob:none",
             "--sparse",
             "https://github.com/ryanoasis/nerd-fonts.git",
             str(nf_dir),
        ])
    else:
        info("Nerd Fonts repo already cloned — pulling latest...")
        run(["git", "-C", str(nf_dir), "pull", "--rebase", "--autostash"], check=False)

    # The install.sh in the Nerd Fonts repo installs ALL fonts when called
    # with no arguments, or specific fonts with font names as arguments.
    # Running as root installs to /usr/local/share/fonts (system-wide).
    install_sh = nf_dir / "install.sh"
    if install_sh.exists():
        info("Running Nerd Fonts install.sh — this installs all fonts system-wide...")
        run(["bash", str(install_sh)], check=False)
        ok("Nerd Fonts installed via install.sh")
    else:
        warn("install.sh not found in cloned repo — falling back to zip downloads")
        _nerd_fonts_zip_fallback()

    info("Rebuilding font cache...")
    run(["fc-cache", "-fv"], check=False)
    ok("Phase 4 complete")


def _nerd_fonts_zip_fallback() -> None:
    """Download individual Nerd Font zips as a fallback."""
    fonts_dir = Path("/usr/local/share/fonts/nerd-fonts")
    fonts_dir.mkdir(parents=True, exist_ok=True)
    base = "https://github.com/ryanoasis/nerd-fonts/releases/latest/download"
    for name in ["Terminus", "JetBrainsMono", "FiraCode", "Hack", "SourceCodePro"]:
        dest_zip  = fonts_dir / f"{name}.zip"
        dest_dir  = fonts_dir / name
        if dest_dir.exists():
            skip(f"Nerd Font: {name}")
            continue
        info(f"Downloading {name}.zip...")
        run(["curl", "-fLo", str(dest_zip), f"{base}/{name}.zip"], check=False)
        run(["unzip", "-o", str(dest_zip), "-d", str(dest_dir)], check=False)
        dest_zip.unlink(missing_ok=True)

# =============================================================================
#  PHASE 5 — ZSH + OH-MY-ZSH + POWERLEVEL10K
# =============================================================================

def phase_5_zsh() -> None:
    banner("Phase 5 — Zsh + Oh-My-Zsh + Powerlevel10k")

    dnf_install(["zsh", "git", "curl"], label="zsh + git + curl")

    # Set Zsh as default shell
    try:
        current_shell = _pwd.getpwnam(REAL_USER).pw_shell
    except KeyError:
        current_shell = ""

    if "zsh" not in current_shell:
        zsh_path = run(["which", "zsh"], capture=True, check=False).stdout.strip() or "/bin/zsh"
        run(["chsh", "-s", zsh_path, REAL_USER])
        ok(f"Default shell → {zsh_path}")
    else:
        skip(f"default shell (already zsh)")

    # Oh-My-Zsh
    omz_dir = REAL_HOME / ".oh-my-zsh"
    if omz_dir.exists():
        skip("Oh-My-Zsh")
    else:
        info("Installing Oh-My-Zsh (unattended)...")
        shell(
            f'sudo -u {REAL_USER} env RUNZSH=no CHSH=no HOME={REAL_HOME} '
            f'sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"'
        )
        ok("Oh-My-Zsh installed")

    # Powerlevel10k
    p10k_dir = REAL_HOME / ".oh-my-zsh/custom/themes/powerlevel10k"
    git_clone_or_pull("https://github.com/romkatv/powerlevel10k.git", p10k_dir,
                      depth=1, as_user=True)

    # zsh-autosuggestions
    zsh_as = REAL_HOME / ".oh-my-zsh/custom/plugins/zsh-autosuggestions"
    git_clone_or_pull("https://github.com/zsh-users/zsh-autosuggestions.git", zsh_as,
                      depth=1, as_user=True)

    # zsh-syntax-highlighting
    zsh_hl = REAL_HOME / ".oh-my-zsh/custom/plugins/zsh-syntax-highlighting"
    git_clone_or_pull("https://github.com/zsh-users/zsh-syntax-highlighting.git", zsh_hl,
                      depth=1, as_user=True)

    ok("Phase 5 complete — .zshrc written in Phase 17")

# =============================================================================
#  PHASE 6 — RUST TOOLCHAIN
# =============================================================================

def phase_6_rust() -> None:
    banner("Phase 6 — Rust Toolchain (rustup)")

    # Build prerequisites
    dnf_install([
        "gcc", "gcc-c++",
        "openssl-devel", "sqlite-devel", "zlib-devel", "libffi-devel",
    ], label="Rust build prerequisites")

    rustup_bin = REAL_HOME / ".cargo/bin/rustup"

    cargo_env = {
        "HOME": str(REAL_HOME),
        "PATH": f"{REAL_HOME}/.cargo/bin:{os.environ.get('PATH', '')}",
        "RUSTUP_HOME": str(REAL_HOME / ".rustup"),
        "CARGO_HOME":  str(REAL_HOME / ".cargo"),
    }

    if rustup_bin.exists():
        info("rustup already present — updating stable toolchain...")
        run([str(rustup_bin), "update", "stable"], as_user=True, env_extra=cargo_env)
    else:
        info(f"Installing rustup for {REAL_USER}...")
        shell(
            f'sudo -u {REAL_USER} env HOME={REAL_HOME} '
            f'RUSTUP_HOME={REAL_HOME}/.rustup CARGO_HOME={REAL_HOME}/.cargo '
            f'sh -c "curl https://sh.rustup.rs -sSf | sh -s -- -y '
            f'--default-toolchain stable --no-modify-path"'
        )
        ok("rustup installed")

    # Components
    if rustup_bin.exists() or DRY_RUN:
        rbin = str(rustup_bin) if not DRY_RUN else "rustup"
        run([rbin, "default", "stable"], as_user=True, check=False, env_extra=cargo_env)
        run([rbin, "component", "add", "clippy", "rustfmt", "rust-analyzer"],
            as_user=True, check=False, env_extra=cargo_env)
        ok("Components: clippy, rustfmt, rust-analyzer")

    ok("Phase 6 complete")

# =============================================================================
#  PHASE 7 — CARGO TOOLS
# =============================================================================

def phase_7_cargo_tools(skip_cargo: bool = False) -> None:
    banner("Phase 7 — Cargo Tools")

    if skip_cargo:
        skip("all cargo tools (--skip-cargo)")
        return

    cargo_bin = REAL_HOME / ".cargo/bin/cargo"
    if not cargo_bin.exists() and not DRY_RUN:
        warn("cargo not found — skipping. Re-run after Phase 6 succeeds.")
        return

    cargo_env = {
        "HOME": str(REAL_HOME),
        "PATH": f"{REAL_HOME}/.cargo/bin:{os.environ.get('PATH', '')}",
        "RUSTUP_HOME": str(REAL_HOME / ".rustup"),
        "CARGO_HOME":  str(REAL_HOME / ".cargo"),
    }

    cbin = str(cargo_bin) if not DRY_RUN else "cargo"

    installed = run([cbin, "install", "--list"],
                    check=False, capture=True, as_user=True, env_extra=cargo_env).stdout or ""

    tools = {
        "cargo-watch":   "cargo-watch",
        "cargo-edit":    "cargo-edit",
        "cargo-audit":   "cargo-audit",
        "cargo-nextest": "cargo-nextest",
        "just":          "just",
        "bacon":         "bacon",
        "starship":      "starship",
    }

    for binary, crate in tools.items():
        if binary in installed:
            skip(f"cargo: {crate}")
        else:
            info(f"cargo install {crate}...")
            run([cbin, "install", crate],
                as_user=True, check=False, env_extra=cargo_env)
            ok(f"{crate} installed")

    ok("Phase 7 complete")

# =============================================================================
#  PHASE 8 — BUILD TOOLS & DEV LIBRARIES
# =============================================================================

def phase_8_build_tools() -> None:
    banner("Phase 8 — Build Tools & Dev Libraries")
    dnf_install([
        # Compilers
        "clang", "llvm", "lldb",
        # Build systems
        "cmake", "ninja-build", "meson", "make",
        "autoconf", "automake", "libtool",
        # pkg-config (Fedora name)
        "pkgconf-pkg-config",
        # Pi / ARM specific
        "uboot-tools", "dtc",
        # Filesystem tools
        "btrfs-progs", "parted", "dosfstools", "cryptsetup", "lvm2",
        # Dev headers
        "openssl-devel", "sqlite-devel", "zlib-devel", "libffi-devel",
        "libxml2-devel", "libcurl-devel", "readline-devel",
    ], label="build tools + dev libs")
    ok("Phase 8 complete")

# =============================================================================
#  PHASE 9 — GIT FORGE TOOLS
# =============================================================================

def phase_9_git_forge() -> None:
    banner("Phase 9 — Git Forge Tools")

    dnf_install([
        "git", "git-lfs", "gh",
        "tig",
        "git-delta",   # Fedora package name for 'delta' diff pager
    ], label="git tools")

    # lazygit — via COPR (not in default Fedora repos)
    if cmd_exists("lazygit"):
        skip("lazygit")
    else:
        info("Installing lazygit via COPR atim/lazygit...")
        dnf_copr("atim/lazygit")
        dnf_install(["lazygit"], label="lazygit")

    # Initialise git-lfs system-wide
    run(["git", "lfs", "install", "--system"], check=False)
    ok("Phase 9 complete")

# =============================================================================
#  PHASE 10 — CONTAINERS (PODMAN)
# =============================================================================

def phase_10_containers() -> None:
    banner("Phase 10 — Containers (Podman Stack)")
    dnf_install([
        "podman",
        "buildah",
        "skopeo",
        "podman-compose",   # Fedora's docker-compose compatible replacement
        "toolbox",
        "aardvark-dns",     # Podman network DNS
    ], label="Podman stack")

    systemctl_enable("podman.socket")

    # Allow rootless Podman to persist for REAL_USER
    if REAL_USER and REAL_USER != "root":
        run(["loginctl", "enable-linger", REAL_USER], check=False)
        ok(f"Podman rootless linger enabled for {REAL_USER}")

    ok("Phase 10 complete")

# =============================================================================
#  PHASE 11 — BTRFS SNAPSHOTS (SNAPPER)
# =============================================================================

def phase_11_snapshots() -> None:
    banner("Phase 11 — Btrfs Snapshots (Snapper)")

    dnf_install([
        "snapper",
        "snapper-plugins",
        "python3-dnf-plugin-snapper",  # auto-snapshot before dnf transactions
    ], label="Snapper + dnf hook")

    # Create root config (idempotent)
    r = run(["snapper", "-c", "root", "list"], check=False, capture=True)
    if r.returncode == 0:
        skip("Snapper root config")
    else:
        info("Creating Snapper root config...")
        run(["snapper", "-c", "root", "create-config", "/"], check=False)
        ok("Snapper root config created")

    systemctl_enable("snapper-timeline.timer")
    systemctl_enable("snapper-cleanup.timer")

    # Golden baseline snapshot
    info("Creating Golden Baseline snapshot...")
    run(["snapper", "create",
         "--description", "Golden Baseline — Pi Overlord Grimoire Install"],
        check=False)
    ok("Golden Baseline snapshot taken")

    ok("Phase 11 complete")

# =============================================================================
#  PHASE 12 — NETWORK TOOLS
# =============================================================================

def phase_12_network() -> None:
    banner("Phase 12 — Network Tools + Security")

    dnf_install([
        "nmap", "iperf3",
        "nethogs", "iftop", "bandwhich",
        "wireguard-tools",
        "firewalld", "fail2ban",
        "openssh-server", "openssh-clients",
    ], label="network + security tools")

    systemctl_enable("firewalld")
    systemctl_enable("sshd")
    systemctl_enable("NetworkManager")
    systemctl_enable("fail2ban")

    # Tailscale via official repo
    if cmd_exists("tailscale"):
        skip("tailscale")
    else:
        info("Adding official Tailscale repo for Fedora...")
        shell(
            'curl -fsSL https://pkgs.tailscale.com/stable/fedora/tailscale.repo '
            '-o /etc/yum.repos.d/tailscale.repo',
            check=False,
        )
        dnf_install(["tailscale"], label="tailscale")
        systemctl_enable("tailscaled")

    ok("Phase 12 complete")

# =============================================================================
#  PHASE 13 — MULTI-LANGUAGE DEV STACK
# =============================================================================

def phase_13_languages() -> None:
    banner("Phase 13 — Multi-Language Dev Stack")
    dnf_install([
        # Python
        "python3", "python3-pip", "python3-virtualenv", "python3-devel",
        # Node.js
        "nodejs", "npm",
        # Go
        "golang",
        # Lua
        "lua", "lua-devel",
        # Shell tools
        "shellcheck", "shfmt",
    ], label="Python, Node.js, Go, Lua, shell linting")
    ok("Phase 13 complete")

# =============================================================================
#  PHASE 14 — PERFORMANCE TUNING
# =============================================================================

def phase_14_performance() -> None:
    banner("Phase 14 — Performance Tuning (Pi 4/5 Optimised)")

    dnf_install([
        "earlyoom",
        "tuned",
        "irqbalance",
        "zram-generator",
    ], label="performance daemons")

    systemctl_enable("earlyoom")
    systemctl_enable("irqbalance")
    systemctl_enable("tuned")
    run(["tuned-adm", "profile", "balanced"], check=False)
    ok("tuned profile: balanced")

    # sysctl tweaks — Pi-optimised
    conf = "/etc/sysctl.d/99-pi-overlord.conf"
    sysctl_set("vm.swappiness",         "10",   conf)
    sysctl_set("vm.vfs_cache_pressure", "50",   conf)
    sysctl_set("net.core.somaxconn",    "1024", conf)

    # zram config
    zram_conf = Path("/etc/systemd/zram-generator.conf")
    if zram_conf.exists():
        skip("zram config")
    else:
        write_file(zram_conf, """\
            [zram0]
            zram-size = ram * 0.4
            compression-algorithm = zstd
            swap-priority = 100
        """, fix_owner=False)

    ok("Phase 14 complete")

# =============================================================================
#  PHASE 15 — WAYLAND EXTRAS
# =============================================================================

def phase_15_wayland() -> None:
    banner("Phase 15 — Wayland & Display Extras")
    dnf_install([
        "wl-clipboard", "grim", "slurp",
        "direnv",
        "flatpak",
        "helix",
        "neovim",
        "alacritty",
    ], label="Wayland tools + editors")

    # Flathub
    r = run(["flatpak", "remotes"], capture=True, check=False)
    if "flathub" not in (r.stdout or "").lower():
        run([
            "flatpak", "remote-add", "--if-not-exists", "flathub",
            "https://dl.flathub.org/repo/flathub.flatpakrepo",
        ], check=False)
        ok("Flathub remote added")
    else:
        skip("Flathub remote")

    ok("Phase 15 complete")

# =============================================================================
#  PHASE 16 — WORKFLOW & APPEARANCE
# =============================================================================

def phase_16_workflow() -> None:
    banner("Phase 16 — Workflow & Appearance Tools")
    dnf_install([
        "fastfetch",
        "neofetch",
        "papirus-icon-theme",
        "atuin",
        "zoxide",
    ], label="workflow enhancers")
    ok("Phase 16 complete")

# =============================================================================
#  PHASE 17 — ALL CONFIG FILES
# =============================================================================

def phase_17_dotfiles() -> None:
    banner("Phase 17 — Writing All Config Files")

    cfg = REAL_HOME / ".config"

    # ─────────────────────────────────────────────────────────────────────────
    # 17.1  KITTY — phosphor green 1984 terminal
    # ─────────────────────────────────────────────────────────────────────────
    kitty_conf = cfg / "kitty" / "kitty.conf"
    if kitty_conf.exists():
        skip("kitty.conf")
    else:
        write_file(kitty_conf, """\
            # ============================================================
            # PI OVERLORD GRIMOIRE — Kitty: 1984 Phosphor Green Terminal
            # ============================================================

            # Terminess Nerd Font (installed via Nerd Fonts install.sh)
            font_family      TerminessNerdFontMono
            bold_font        TerminessNerdFontMono Bold
            italic_font      TerminessNerdFontMono Italic
            bold_italic_font TerminessNerdFontMono Bold Italic
            font_size        14.0

            # Nerd Font icon fallback
            symbol_map U+E000-U+F8FF Symbols Nerd Font Mono

            # ── Phosphor Green palette ────────────────────────────────
            background       #000000
            foreground       #33FF33

            color0   #000000
            color1   #005500
            color2   #00FF00
            color3   #00CC00
            color4   #007700
            color5   #009900
            color6   #00DD00
            color7   #33FF33
            color8   #1a1a1a
            color9   #00AA00
            color10  #00FF00
            color11  #00FF00
            color12  #00BB00
            color13  #00CC00
            color14  #00EE00
            color15  #44FF44

            # ── Cursor ───────────────────────────────────────────────
            cursor                #00FF00
            cursor_text_color     #000000
            cursor_shape          block
            cursor_blink_interval 0

            # ── Selection ────────────────────────────────────────────
            selection_background  #00FF00
            selection_foreground  #000000

            # ── Scrollback ────────────────────────────────────────────
            scrollback_lines      10000

            # ── Window ───────────────────────────────────────────────
            window_padding_width  4
            confirm_os_window_close 0

            # ── Bell ─────────────────────────────────────────────────
            enable_audio_bell     no
            visual_bell_duration  0.0

            # ── Tab bar ──────────────────────────────────────────────
            tab_bar_style           powerline
            tab_powerline_style     slanted
            active_tab_foreground   #000000
            active_tab_background   #00FF00
            inactive_tab_foreground #008800
            inactive_tab_background #001100

            # ── Performance tuning for Pi 4/5 ────────────────────────
            repaint_delay   8
            input_delay     2
            sync_to_monitor no
        """)

    # ─────────────────────────────────────────────────────────────────────────
    # 17.2  .zshrc — Oh-My-Zsh + P10k + plugins + aliases
    # ─────────────────────────────────────────────────────────────────────────
    zshrc = REAL_HOME / ".zshrc"
    omz_exists = (REAL_HOME / ".oh-my-zsh").exists() or DRY_RUN

    if omz_exists:
        if zshrc.exists() and not DRY_RUN:
            bak = REAL_HOME / ".zshrc.grimoire.bak"
            if not bak.exists():
                shutil.copy2(str(zshrc), str(bak))
                info("Backed up existing .zshrc → .zshrc.grimoire.bak")

        write_file(zshrc, f"""\
            # ============================================================
            # PI OVERLORD GRIMOIRE — .zshrc
            # ============================================================

            # Powerlevel10k instant prompt (must be near top)
            if [[ -r "${{XDG_CACHE_HOME:-$HOME/.cache}}/p10k-instant-prompt-${{(%):-%n}}.zsh" ]]; then
              source "${{XDG_CACHE_HOME:-$HOME/.cache}}/p10k-instant-prompt-${{(%):-%n}}.zsh"
            fi

            export ZSH="$HOME/.oh-my-zsh"
            ZSH_THEME="powerlevel10k/powerlevel10k"

            plugins=(
              git
              zsh-autosuggestions
              zsh-syntax-highlighting
              fzf
              direnv
              rust
            )

            source "$ZSH/oh-my-zsh.sh"

            # P10k config (generated by p10k configure)
            [[ -f ~/.p10k.zsh ]] && source ~/.p10k.zsh

            # ── PATH ─────────────────────────────────────────────────
            export PATH="$HOME/.cargo/bin:$PATH"
            [[ -f "$HOME/.cargo/env" ]] && source "$HOME/.cargo/env"
            export GOPATH="$HOME/go"
            export PATH="$GOPATH/bin:$PATH"

            # ── Editor ───────────────────────────────────────────────
            export EDITOR='nvim'
            export VISUAL='nvim'

            # ── KDE: preferred terminal (Konsole falls back to this) ──────────
            export TERMINAL='kitty'

            # ── Modern CLI replacements ───────────────────────────────
            command -v eza    &>/dev/null && alias ls='eza --icons --group-directories-first'
            command -v eza    &>/dev/null && alias ll='eza -la --icons --group-directories-first'
            command -v eza    &>/dev/null && alias lt='eza --tree --icons'
            command -v bat    &>/dev/null && alias cat='bat --paging=never'
            command -v fd     &>/dev/null && alias find='fd'
            command -v rg     &>/dev/null && alias grep='rg'

            # ── Useful aliases ────────────────────────────────────────
            alias ..='cd ..'
            alias ...='cd ../..'
            alias ....='cd ../../..'
            alias md='mkdir -p'
            alias reload='exec zsh'
            alias update='sudo dnf upgrade -y'
            alias please='sudo'
            alias df='df -h'
            alias free='free -h'

            # ── Git shortcuts ─────────────────────────────────────────
            alias g='git'
            alias gs='git status'
            alias ga='git add'
            alias gc='git commit'
            alias gp='git push'
            alias gl='git log --oneline --graph --decorate'
            alias gd='git diff'

            # ── Podman as docker ──────────────────────────────────────
            command -v podman &>/dev/null && alias docker='podman'

            # ── Tool inits ────────────────────────────────────────────
            command -v zoxide &>/dev/null && eval "$(zoxide init zsh)"
            command -v atuin  &>/dev/null && eval "$(atuin init zsh)"
            command -v direnv &>/dev/null && eval "$(direnv hook zsh)"

            # ── fzf key bindings ──────────────────────────────────────
            [ -f /usr/share/fzf/shell/key-bindings.zsh ] && \\
              source /usr/share/fzf/shell/key-bindings.zsh
            [ -f /usr/share/fzf/shell/completion.zsh ]   && \\
              source /usr/share/fzf/shell/completion.zsh

            # ── fastfetch on new terminal ─────────────────────────────
            command -v fastfetch &>/dev/null && fastfetch
        """)

    # ─────────────────────────────────────────────────────────────────────────
    # 17.3  Neovim minimal config
    # ─────────────────────────────────────────────────────────────────────────
    nvim_init = cfg / "nvim" / "init.vim"
    if not nvim_init.exists():
        write_file(nvim_init, """\
            " ============================================================
            " PI OVERLORD GRIMOIRE — Neovim init.vim
            " ============================================================

            set number relativenumber
            set tabstop=4 shiftwidth=4 expandtab smartindent
            set wrap ignorecase smartcase
            set incsearch hlsearch
            set scrolloff=8
            set signcolumn=yes
            set updatetime=50
            set termguicolors

            " Phosphor green theme
            highlight Normal       guibg=#000000 guifg=#33FF33
            highlight LineNr       guifg=#005500
            highlight CursorLineNr guifg=#00FF00
            highlight Comment      guifg=#008800 gui=italic
            highlight Visual       guibg=#003300

            let mapleader = " "
            nnoremap <leader>w :w<CR>
            nnoremap <leader>q :q<CR>
            nnoremap <leader>h :nohlsearch<CR>
        """)
    else:
        skip("nvim init.vim")

    # ─────────────────────────────────────────────────────────────────────────
    # 17.4  fastfetch config
    # ─────────────────────────────────────────────────────────────────────────
    ff_conf = cfg / "fastfetch" / "config.jsonc"
    if not ff_conf.exists():
        write_file(ff_conf, """\
            {
              "$schema": "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json",
              "logo": {
                "source": "fedora",
                "color": { "1": "green", "2": "bright_green" }
              },
              "display": { "separator": "  ", "color": "green" },
              "modules": [
                "title", "separator",
                "os", "host", "kernel", "uptime",
                "separator",
                "cpu", "memory", "swap", "disk",
                "separator",
                "shell", "terminal", "de", "wm",
                "separator",
                "localip",
                "colors"
              ]
            }
        """)
    else:
        skip("fastfetch config")

    ok("Phase 17 complete")

def phase_18_services() -> None:
    banner("Phase 18 — Enable Services + Final Ownership Fix")

    for unit in [
        "NetworkManager", "firewalld", "sshd", "fail2ban",
        "earlyoom", "irqbalance", "tuned",
        "podman.socket",
        "snapper-timeline.timer", "snapper-cleanup.timer",
        # SDDM display manager — KDE's native DM, X11 session
        "sddm",
        # Argon One case fan + power button (installed by Phase 18)
        "argononed",
    ]:
        systemctl_enable(unit)

    # Set graphical.target as default so SDDM starts on boot
    info("Setting default boot target to graphical...")
    run(["systemctl", "set-default", "graphical.target"], check=False)
    ok("Default target: graphical.target")

    # Firewall: allow SSH through
    run(["firewall-cmd", "--permanent", "--add-service=ssh"], check=False)
    run(["firewall-cmd", "--reload"], check=False)
    ok("Firewall: SSH allowed")

    # Fix ownership of everything we wrote into the user's home
    if REAL_USER and REAL_USER != "root" and not DRY_RUN:
        info(f"Fixing ownership of {REAL_HOME} → {REAL_USER}:{REAL_USER}")
        run(["chown", "-R", f"{REAL_USER}:{REAL_USER}", str(REAL_HOME)], check=False)
        ok("Ownership corrected")

    # Final font cache rebuild
    run(["fc-cache", "-fv"], check=False)
    ok("Font cache rebuilt")

    ok("Phase 18 complete")

# =============================================================================
#  PHASE 19 — ARGON ONE DAEMON (FAN + POWER BUTTON)
# =============================================================================

def phase_19_argononed() -> None:
    banner("Phase 19 — Argon One Daemon (argononed)")

    # ── What this is ──────────────────────────────────────────────────────────
    # argononed by DarkElvenAngel is a C daemon that controls the Argon One
    # case fan speed (via I2C) and the power button (via GPIO 4) on Pi 4B.
    # It builds from source with ./configure && make all && sudo make install.
    # The project ships its own device tree overlay which activates the I2C bus,
    # so dtparam=i2c_arm=on is not strictly required but we set it anyway for
    # maximum compatibility.
    #
    # Fedora note: /dev/gpiomem does NOT exist on Fedora's upstream kernel.
    # Fan control (I2C) works fine. The power button uses GPIO 4 and works
    # via /dev/gpiochip0 — argononed's configure script detects this automatically.

    # ── Build prerequisites (most already installed by Phase 8) ───────────────
    dnf_install([
        "gcc", "make", "dtc",      # core build tools
        "i2c-tools",               # i2cdetect, i2cset — needed by argononed at runtime
        "i2c-tools-devel",         # i2c dev headers (just in case)
    ], label="argononed build deps")

    # ── Enable I2C in /boot/efi/config.txt (Fedora ARM EFI path) ─────────────
    # Fedora ARM uses /boot/efi/config.txt NOT /boot/config.txt
    config_txt = Path("/boot/efi/config.txt")
    if not config_txt.exists():
        # Try the older path as fallback
        config_txt = Path("/boot/config.txt")

    if config_txt.exists():
        append_once(
            config_txt,
            "dtparam=i2c_arm=on",
            "dtparam=i2c_arm=on\n",
            fix_owner=False,
        )
    else:
        warn(f"config.txt not found at /boot/efi/config.txt or /boot/config.txt — skipping i2c dtparam")
        warn("You may need to manually add 'dtparam=i2c_arm=on' to your Pi config.txt")

    # ── Clone or update the repo ──────────────────────────────────────────────
    src_dir = Path("/opt/argononed")
    git_clone_or_pull(
        "https://gitlab.com/DarkElvenAngel/argononed.git",
        src_dir,
        depth=0,   # full clone — needed for make/configure
    )

    # ── Check if already installed and up to date ─────────────────────────────
    daemon_bin = Path("/usr/sbin/argononed")
    cli_bin    = Path("/usr/sbin/argonone-cli")

    # We track the installed git hash to know if a reinstall is needed
    hash_file  = Path("/opt/argononed/.installed_hash")
    repo_hash  = run(["git", "-C", str(src_dir), "rev-parse", "HEAD"],
                     capture=True, check=False).stdout.strip()

    if daemon_bin.exists() and hash_file.exists() and not DRY_RUN:
        if hash_file.read_text().strip() == repo_hash:
            skip("argononed already installed at current commit")
            ok("Phase 19 complete")
            return

    # ── Build and install ─────────────────────────────────────────────────────
    info("Running ./configure...")
    run(["bash", "-c", f"cd {src_dir} && ./configure"], check=True)

    info("Running make all...")
    run(["bash", "-c", f"cd {src_dir} && make all"], check=True)

    info("Running make install...")
    run(["bash", "-c", f"cd {src_dir} && make install"], check=True)
    ok("argononed installed")

    # Record installed hash for idempotency on future runs
    if not DRY_RUN:
        hash_file.write_text(repo_hash + "\n")

    # ── Write a sensible default config ───────────────────────────────────────
    # /etc/argononed.conf — fan curve tuned for Pi 4B in Argon One case
    # Temps in °C, fan speeds in %
    # Format: TEMP=SPEED per line, plus HYSTERESIS setting
    argon_conf = Path("/etc/argononed.conf")
    if argon_conf.exists():
        skip("/etc/argononed.conf (already exists — not overwriting your fan curve)")
    else:
        write_file(argon_conf, """\
            # =================================================================
            # PI OVERLORD GRIMOIRE — argononed fan config
            # =================================================================
            # Fan curve for Argon One case on Pi 4B
            # Temp in °C | Fan speed in %
            # Three stages: cool / warm / hot
            #
            # How it works:
            #   Below FAN0TEMP  → fan off
            #   At FAN0TEMP     → FAN0SPEED (whisper)
            #   At FAN1TEMP     → FAN1SPEED (normal)
            #   At FAN2TEMP     → FAN2SPEED (full blast)
            #   HYSTERESIS      → degrees to drop below threshold before stepping down
            #
            # Tweak with: argonone-cli --temp0 55 --fan0 25 --commit
            # Check live:  argonone-cli --decode

            FAN0TEMP=55
            FAN0SPEED=25

            FAN1TEMP=65
            FAN1SPEED=50

            FAN2TEMP=75
            FAN2SPEED=100

            HYSTERESIS=3
        """, fix_owner=False)

    # ── Enable and start the service ──────────────────────────────────────────
    # make install creates /etc/systemd/system/argononed.service
    run(["systemctl", "daemon-reload"], check=False)
    systemctl_enable("argononed")

    # ── Add REAL_USER to the i2c group (allows argonone-cli without sudo) ─────
    if REAL_USER and REAL_USER != "root":
        run(["usermod", "-aG", "i2c", REAL_USER], check=False)
        ok(f"Added {REAL_USER} to i2c group")

    ok("Phase 19 complete")
    info("Fan control active immediately. Power button active after reboot.")
    info("Tune fan curve: argonone-cli --decode  |  argonone-cli --help")


# =============================================================================
#  PHASE TABLE & MAIN
# =============================================================================

PHASE_MAP: dict[int, tuple[str, object]] = {
    0:  ("Preflight",            phase_0_preflight),
    1:  ("Core CLI Tools",       phase_1_core_tools),
    2:  ("KDE Plasma (X11)",      phase_2_kde),
    3:  ("Kitty Terminal",       phase_3_kitty),
    4:  ("Fonts (Nerd Fonts)",   phase_4_fonts),
    5:  ("Zsh + OMZ + P10k",     phase_5_zsh),
    6:  ("Rust Toolchain",       phase_6_rust),
    7:  ("Cargo Tools",          phase_7_cargo_tools),
    8:  ("Build Tools",          phase_8_build_tools),
    9:  ("Git Forge",            phase_9_git_forge),
    10: ("Containers (Podman)",  phase_10_containers),
    11: ("Filesystem Snapshots", phase_11_snapshots),
    12: ("Network + Security",   phase_12_network),
    13: ("Multi-Language Stack", phase_13_languages),
    14: ("Performance Tuning",   phase_14_performance),
    15: ("Wayland Extras",       phase_15_wayland),
    16: ("Workflow + Appearance",phase_16_workflow),
    17: ("Config Files",         phase_17_dotfiles),
    18: ("Argon One Daemon",     phase_19_argononed),
    19: ("Services + Ownership", phase_18_services),
}

PHASE_ORDER = list(PHASE_MAP.keys())


def main() -> None:
    global DRY_RUN

    parser = argparse.ArgumentParser(
        formatter_class=argparse.RawDescriptionHelpFormatter,
        description="PI OVERLORD GRIMOIRE — complete one-shot Fedora ARM installer",
        epilog=(
            "Examples:\n"
            "  sudo python3 pi_overlord_grimoire.py\n"
            "  sudo python3 pi_overlord_grimoire.py --dry-run\n"
            "  sudo python3 pi_overlord_grimoire.py --phases 1,2,3\n"
            "  sudo python3 pi_overlord_grimoire.py --skip-cargo\n"
            "  sudo python3 pi_overlord_grimoire.py --list-phases\n"
        ),
    )
    parser.add_argument("--dry-run",     action="store_true",
                        help="Show what would happen without doing anything")
    parser.add_argument("--phases",      default="",
                        help="Run specific phases only, e.g.  --phases 1,2,5")
    parser.add_argument("--skip-cargo",  action="store_true",
                        help="Skip slow cargo tool installs (Phase 7)")
    parser.add_argument("--list-phases", action="store_true",
                        help="Print phase list and exit")

    args = parser.parse_args()
    DRY_RUN = args.dry_run

    if args.list_phases:
        print(f"\n  {'#':<4}  Phase")
        print(f"  {'─'*4}  {'─'*40}")
        for k, (name, _) in PHASE_MAP.items():
            print(f"  {k:<4}  {name}")
        print()
        sys.exit(0)

    if args.phases:
        try:
            phases_to_run = [int(p.strip()) for p in args.phases.split(",")]
        except ValueError:
            fatal("--phases must be comma-separated integers, e.g. --phases 1,2,3")
    else:
        phases_to_run = PHASE_ORDER

    # ── Splash screen ─────────────────────────────────────────────────────────
    print(f"""
{BOLD}{GREEN}
╔══════════════════════════════════════════════════════════════════════════╗
║                                                                          ║
║   🧠  PI OVERLORD GRIMOIRE  —  Complete One-Shot Installer               ║
║       FrankenPi Overlord Mode  ×  The Green Grimoire                    ║
║                                                                          ║
║   Raspberry Pi 4/5  ·  Fedora ARM  ·  KDE Plasma X11  ·  Btrfs         ║
║   Phosphor Green  ·  Rust-first  ·  Full Dev Stack                      ║
║                                                                          ║
╚══════════════════════════════════════════════════════════════════════════╝
{RESET}
  Phases    : {phases_to_run}
  Dry-run   : {DRY_RUN}
  Log file  : {LOG_FILE}

  {YELLOW}First run takes 20–45 minutes. Safe to re-run any time.
  Already-done steps are detected and skipped automatically.{RESET}
""")

    start = time.time()

    for phase_num in PHASE_ORDER:
        if phase_num not in phases_to_run:
            continue
        _, fn = PHASE_MAP[phase_num]
        if phase_num == 7:
            fn(skip_cargo=args.skip_cargo)
        else:
            fn()

    elapsed = int(time.time() - start)
    mins, secs = divmod(elapsed, 60)

    print(f"""
{BOLD}{GREEN}
╔══════════════════════════════════════════════════════════════════════════╗
║   ✅  INSTALLATION COMPLETE                                              ║
╚══════════════════════════════════════════════════════════════════════════╝
{RESET}
  Time taken  : {mins}m {secs}s
  Log file    : {LOG_FILE}

  {BOLD}Installed & configured:{RESET}
    {GREEN}✔{RESET}  KDE Plasma Desktop (X11, SDDM, Pi 4B stable — no Wayland weirdness)
    {GREEN}✔{RESET}  Argon One daemon (fan curve + power button, argonone-cli available)
    {GREEN}✔{RESET}  Kitty terminal — TerminessNerdFontMono 14px, phosphor green
    {GREEN}✔{RESET}  Nerd Fonts full suite (via git clone + install.sh)
    {GREEN}✔{RESET}  Zsh + Oh-My-Zsh + Powerlevel10k + autosuggestions + highlighting
    {GREEN}✔{RESET}  Rust toolchain via rustup (stable, clippy, rustfmt, rust-analyzer)
    {GREEN}✔{RESET}  Cargo tools: watch, edit, audit, nextest, just, bacon
    {GREEN}✔{RESET}  Full build stack: gcc, clang, cmake, ninja, meson
    {GREEN}✔{RESET}  Git forge: gh CLI, lazygit, delta, tig
    {GREEN}✔{RESET}  Podman container stack (rootless, socket enabled)
    {GREEN}✔{RESET}  Snapper Btrfs snapshots + dnf hook + golden baseline
    {GREEN}✔{RESET}  Network: nmap, WireGuard, Tailscale, firewalld, fail2ban
    {GREEN}✔{RESET}  Python3, Node.js, Go, Lua + shellcheck/shfmt
    {GREEN}✔{RESET}  Performance: earlyoom, tuned balanced, irqbalance, zram 40%
    {GREEN}✔{RESET}  All config files: kitty, zshrc, nvim, fastfetch
    {GREEN}✔{RESET}  All services enabled

  {BOLD}Next steps:{RESET}

    1.  {BOLD}Reboot your Pi{RESET}  (activates SDDM, zram, tuned, sysctl)
            {CYAN}sudo reboot{RESET}

    2.  {BOLD}SDDM login screen appears automatically{RESET}
            Select "Plasma (X11)" session from the session menu.
            Log in — KDE Plasma desktop launches.

    3.  {BOLD}Set Kitty as default terminal in KDE{RESET}:
            System Settings → Default Applications → Terminal → Kitty

    4.  {BOLD}Set up your Powerlevel10k prompt{RESET}  (first time you open Kitty):
            {CYAN}p10k configure{RESET}

    5.  {BOLD}Apply Papirus icons{RESET}:
            System Settings → Icons → Papirus

    6.  {BOLD}Connect Tailscale{RESET}  (optional VPN):
            {CYAN}sudo tailscale up{RESET}

    7.  {BOLD}Tune Argon One fan curve{RESET}  (optional):
            Check live status : {CYAN}argonone-cli --decode{RESET}
            Adjust a threshold: {CYAN}argonone-cli --temp1 60 --fan1 40 --commit{RESET}
            Config file       : {CYAN}sudo nvim /etc/argononed.conf{RESET}
            {DIM}Note: power button needs a reboot to activate (GPIO init){RESET}

  {DIM}The machine glows green. Rust compiles in silence.{RESET}
""")


if __name__ == "__main__":
    main()
