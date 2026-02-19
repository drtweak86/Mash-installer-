#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────
#  mash installer – Rust-based TUI installer with ratatui
#
#  Supports: Arch Linux (aarch64), Debian, Fedora
#
#  Usage:
#    curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh | bash
#    bash bootstrap.sh [OPTIONS]
#
#  Features:
#    • Beautiful TUI interface with ratatui
#    • Kitty terminal included
#    • Powerlevel10k (starship removed)
#    • Profile-based installations
#
#  Options are passed through to the mash-setup binary:
#    --profile minimal|dev|full
#    --dry-run
#    --non-interactive
#    etc.
# ─────────────────────────────────────────────────────────────────
set -euo pipefail

# ── colours ─────────────────────────────────────────────────────
if [ -t 2 ]; then
    RED='\033[0;31m' GREEN='\033[0;32m' YELLOW='\033[0;33m'
    CYAN='\033[0;36m' BOLD='\033[1m' RESET='\033[0m'
else
    RED='' GREEN='' YELLOW='' CYAN='' BOLD='' RESET=''
fi

# ── helpers ─────────────────────────────────────────────────────
info()  { echo -e "${GREEN}[info]${RESET}  $*" >&2; }
warn()  { echo -e "${YELLOW}[warn]${RESET}  $*" >&2; }
die()   { echo -e "${RED}[error]${RESET} $*" >&2; exit 1; }

# ── banner ───────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}╔══════════════════════════════════════════════╗${RESET}"
echo -e "${BOLD}║       mash-setup · installer bootstrap       ║${RESET}"
echo -e "${BOLD}╚══════════════════════════════════════════════╝${RESET}"
echo ""

# ── pre-flight ───────────────────────────────────────────────────
[[ "$(uname -s)" == "Linux" ]] || die "Linux only."
command -v sudo &>/dev/null || die "sudo is required."

ARCH=$(uname -m)
case "$ARCH" in
    x86_64|aarch64|armv7l) ;;
    *) die "Unsupported architecture: $ARCH" ;;
esac

# ── detect package manager ──────────────────────────────────────
if command -v pacman &>/dev/null; then
    PKG_MGR="pacman"
    INSTALL_CMD="sudo pacman -S --noconfirm"
elif command -v apt-get &>/dev/null; then
    PKG_MGR="apt"
    INSTALL_CMD="sudo DEBIAN_FRONTEND=noninteractive apt-get install -y"
elif command -v dnf &>/dev/null; then
    PKG_MGR="dnf"
    INSTALL_CMD="sudo dnf install -y"
else
    die "No supported package manager found (tried: pacman, apt-get, dnf)"
fi

info "Detected package manager: $PKG_MGR"

# ── install Rust if needed ───────────────────────────────────────
if ! command -v cargo &>/dev/null; then
    info "Rust/Cargo not found. Installing..."

    # Install curl and git if needed
    if ! command -v curl &>/dev/null; then
        info "Installing curl..."
        $INSTALL_CMD curl
    fi

    if ! command -v git &>/dev/null; then
        info "Installing git..."
        $INSTALL_CMD git
    fi

    # Install rustup
    info "Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

    # Source cargo env
    export PATH="$HOME/.cargo/bin:$PATH"
    source "$HOME/.cargo/env" 2>/dev/null || true

    if ! command -v cargo &>/dev/null; then
        die "Failed to install Rust. Please install it manually from https://rustup.rs"
    fi

    info "Rust installed successfully!"
else
    info "Rust/Cargo already installed"
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# ── clone or use existing repo ───────────────────────────────────
REPO_URL="https://github.com/drtweak86/Mash-installer-.git"
BRANCH="${MASH_BRANCH:-claude/fix-ubuntu-installer-error-w1E1o}"
WORK_DIR="${MASH_WORK_DIR:-}"

if [[ -z "$WORK_DIR" ]]; then
    # Check if we're already in the repo
    if [[ -f "Cargo.toml" ]] && grep -q "mash-installer" Cargo.toml 2>/dev/null; then
        info "Already in mash-installer directory"
        WORK_DIR="$(pwd)"
    else
        # Clone the repo
        WORK_DIR="$HOME/.cache/mash-installer-build"
        info "Cloning repository to $WORK_DIR..."

        if [[ -d "$WORK_DIR" ]]; then
            info "Cleaning existing directory..."
            rm -rf "$WORK_DIR"
        fi

        git clone --branch "$BRANCH" --depth 1 "$REPO_URL" "$WORK_DIR"
    fi
fi

cd "$WORK_DIR"

# ── build the installer ──────────────────────────────────────────
info "Building mash-setup binary (this may take a few minutes)..."
cargo build --release --bin mash-setup

if [[ ! -f "target/release/mash-setup" ]]; then
    die "Build failed - binary not found at target/release/mash-setup"
fi

info "Build complete!"

# ── run the installer ────────────────────────────────────────────
echo ""
echo -e "${BOLD}${CYAN}Running mash-setup installer...${RESET}"
echo ""

exec target/release/mash-setup "$@"
