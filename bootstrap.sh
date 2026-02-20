#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────
#  mash-installer – The Over-Engineered Pi 4 Setup Wizard 🧙‍♂️✨
# ─────────────────────────────────────────────────────────────────
set -euo pipefail

# ── colours ─────────────────────────────────────────────────────
if [ -t 2 ]; then
    RED='\033[0;31m' GREEN='\033[0;32m' YELLOW='\033[0;33m'
    CYAN='\033[0;36m' PURPLE='\033[1;35m' BOLD='\033[1m' RESET='\033[0m'
else
    RED='' GREEN='' YELLOW='' CYAN='' PURPLE='' BOLD='' RESET=''
fi

info()  { echo -e "${GREEN}[info]${RESET}  $*" >&2; }
warn()  { echo -e "${YELLOW}[warn]${RESET}  $*" >&2; }
die()   { echo -e "${RED}[error]${RESET} $*" >&2; exit 1; }

echo ""
echo -e "${PURPLE}===================================================${RESET}"
echo -e "${CYAN} 🧙‍♂️ Welcome to the Mash Setup Wizard 🧙‍♂️ ${RESET}"
echo -e "${PURPLE}===================================================${RESET}"
echo ""

[[ "$(uname -s)" == "Linux" ]] || die "Linux only."
command -v sudo &>/dev/null || die "sudo is required."

# ── The Sudo Keep-Alive Spell 🪄 ────────────────────────────────
info "Unlocking the sudo gates..."
sudo -v
# Keep-alive: update existing \`sudo\` timestamp until script finishes
while true; do sudo -n true; sleep 60; kill -0 "$$" || exit; done 2>/dev/null &

ARCH=$(uname -m)
case "$ARCH" in
    x86_64|aarch64|armv7l) ;;
    *) die "Unsupported architecture: $ARCH" ;;
esac

# ── detect package manager ──────────────────────────────────────
if command -v pacman &>/dev/null; then
    PKG_MGR="pacman"
    INSTALL_CMD="sudo pacman -S --noconfirm --needed"
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

# ── Phase 1: The Font Forging (Nerd Fonts) 🖋️ ───────────────────
info "Forging the Glyphs (Installing Nerd Fonts)..."
if [[ "$PKG_MGR" == "pacman" ]]; then
    $INSTALL_CMD ttf-nerd-fonts-symbols ttf-dejavu ttf-font-awesome
    info "Nerd Fonts installed! Your terminal is now pretty. ✨"
elif [[ "$PKG_MGR" == "apt" ]]; then
    $INSTALL_CMD fonts-font-awesome
    info "Installing basic fonts (manual Nerd Font install required on apt for full icons)."
fi

# ── Phase 2: OS Detection & The Hyprland Question 🌌 ────────────
if [[ "$PKG_MGR" == "pacman" ]]; then
    echo -e "\n${CYAN}Ah, I see it. You use Arch BTW. 🏹${RESET}"
    echo -e "${YELLOW}Optimizing Pi 4 for 4TB HDD build speeds...${RESET}"
    
    # Simple Pi 4 optimization (4 Cores + 1)
    sudo sed -i 's/#MAKEFLAGS="-j2"/MAKEFLAGS="-j5"/g' /etc/makepkg.conf 2>/dev/null || true
    sudo sed -i 's/COMPRESSXZ=(xz -c -z -)/COMPRESSXZ=(xz -c -z - --threads=0)/g' /etc/makepkg.conf 2>/dev/null || true

    echo -e "\n${PURPLE}Would you like to summon Hyprland? (y/n)${RESET}"
    read -r -p "> " summon_hyprland </dev/tty || summon_hyprland="n"
    
    if [[ "$summon_hyprland" =~ ^[Yy]$ ]]; then
        info "Summoning the Wayland compositor! Hold onto your 4TB drive..."
        $INSTALL_CMD hyprland waybar wofi kitty
    else
        info "Hyprland remains dormant. Just the terminal for you today!"
    fi
fi

# ── install Rust if needed ───────────────────────────────────────
if ! command -v cargo &>/dev/null; then
    info "Rust/Cargo not found. Installing..."
    if ! command -v curl &>/dev/null; then $INSTALL_CMD curl; fi
    if ! command -v git &>/dev/null; then $INSTALL_CMD git; fi

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    export PATH="$HOME/.cargo/bin:$PATH"
    source "$HOME/.cargo/env" 2>/dev/null || true

    if ! command -v cargo &>/dev/null; then die "Failed to install Rust."; fi
    info "Rust installed successfully!"
else
    info "Rust/Cargo already installed"
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# ── clone or use existing repo ───────────────────────────────────
REPO_URL="https://github.com/drtweak86/Mash-installer-.git"
# Default to main branch instead of the fix branch unless specified
BRANCH="${MASH_BRANCH:-main}"
WORK_DIR="${MASH_WORK_DIR:-}"

if [[ -z "$WORK_DIR" ]]; then
    if [[ -f "Cargo.toml" ]] && grep -q "mash-installer" Cargo.toml 2>/dev/null; then
        info "Already in mash-installer directory"
        WORK_DIR="$(pwd)"
    else
        WORK_DIR="$HOME/.cache/mash-installer-build"
        info "Cloning repository to $WORK_DIR..."
        if [[ -d "$WORK_DIR" ]]; then rm -rf "$WORK_DIR"; fi
        git clone --branch "$BRANCH" --depth 1 "$REPO_URL" "$WORK_DIR"
    fi
fi

cd "$WORK_DIR"

# ── build the installer ──────────────────────────────────────────
info "Building mash-setup binary (this may take a few minutes on a Pi)..."
cargo build --release --bin mash-setup

if [[ ! -f "target/release/mash-setup" ]]; then
    die "Build failed - binary not found at target/release/mash-setup"
fi

info "Build complete! Launching the TUI... 🚀"
sleep 1
exec target/release/mash-setup "$@"
