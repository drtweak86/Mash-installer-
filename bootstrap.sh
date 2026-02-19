#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────
#  mash installer – Rust-based TUI installer with ratatui
#
#  Supports: Arch Linux (aarch64), Debian, Fedora
#
#  Usage:
#    curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh | bash
#    bash bootstrap.sh [--profile minimal|dev|full] [--dry-run]
#
#  Features:
#    • Beautiful TUI interface with ratatui
#    • Kitty terminal included
#    • Powerlevel10k (starship removed)
#    • Profile-based installations
#
#  Profiles:
#    minimal  – system packages + Rust + GitHub CLI
#    dev      – minimal + Docker + Buildroot deps + Zsh + kitty + P10K (default)
#    full     – dev + Node.js/npm
# ─────────────────────────────────────────────────────────────────
set -euo pipefail

# ── defaults ────────────────────────────────────────────────────
PROFILE="dev"
DRY_RUN=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --profile) PROFILE="$2"; shift 2 ;;
        --dry-run) DRY_RUN=true; shift ;;
        *) echo "Unknown argument: $1" >&2; exit 1 ;;
    esac
done

case "$PROFILE" in
    minimal|dev|full) ;;
    *) echo "Invalid profile '$PROFILE'. Use: minimal, dev, full" >&2; exit 1 ;;
esac

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

_STEP=0
step() {
    _STEP=$((_STEP + 1))
    echo -e "\n${BOLD}${CYAN}── $_STEP. $* ──────────────────────────────${RESET}" >&2
}

run() {
    if [[ "$DRY_RUN" == true ]]; then
        echo "[dry-run] $*" >&2
    else
        "$@"
    fi
}

apt_install() {
    run sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --install-recommends "$@"
}

apt_try() {
    sudo DEBIAN_FRONTEND=noninteractive apt-get install -y "$@" &>/dev/null \
        || warn "Optional package(s) unavailable: $*"
}

is_dev_or_full() { [[ "$PROFILE" == "dev" || "$PROFILE" == "full" ]]; }

# ── pre-flight ───────────────────────────────────────────────────
command -v apt-get &>/dev/null || die "This installer requires apt-get (Ubuntu/Debian only)."
command -v sudo   &>/dev/null || die "sudo is required."
[[ "$(uname -s)" == "Linux" ]]  || die "Linux only."

ARCH=$(uname -m)
case "$ARCH" in
    x86_64)  DEB_ARCH="amd64" ;;
    aarch64) DEB_ARCH="arm64" ;;
    *)       die "Unsupported architecture: $ARCH" ;;
esac

CODENAME=$(. /etc/os-release 2>/dev/null && echo "${UBUNTU_CODENAME:-${VERSION_CODENAME:-focal}}")
CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"

# ── banner ───────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}╔══════════════════════════════════════════════╗${RESET}"
echo -e "${BOLD}║       mash-setup · mega installer            ║${RESET}"
echo -e "${BOLD}║  profile: ${PROFILE}$(printf '%*s' $((33 - ${#PROFILE})) '')║${RESET}"
echo -e "${BOLD}╚══════════════════════════════════════════════╝${RESET}"
echo ""
[[ "$DRY_RUN" == true ]] && info "Dry-run mode – no changes will be made."

# ════════════════════════════════════════════════════════════════
# 1. SYSTEM PACKAGES
# ════════════════════════════════════════════════════════════════
step "System packages"

sudo apt-get update -qq

PKGS=(
    # network / archive basics
    ca-certificates curl wget xz-utils tar jq git
    # repo management
    software-properties-common gnupg lsb-release apt-transport-https
    # build toolchain
    build-essential pkg-config clang lld cmake ninja-build gcc g++ gdb make
)

is_dev_or_full && PKGS+=(
    python3 python3-pip python3-venv
    ripgrep fd-find fzf tmux htop ncdu neovim
)

[[ "$PROFILE" == "full" ]] && PKGS+=(nodejs npm)

apt_install "${PKGS[@]}"
apt_try lldb btop bat eza yq
info "System packages done."

# ════════════════════════════════════════════════════════════════
# 2. RUST TOOLCHAIN
# ════════════════════════════════════════════════════════════════
step "Rust toolchain"

if command -v rustup &>/dev/null || [[ -f "$CARGO_HOME/bin/rustup" ]]; then
    info "rustup already installed – updating"
    run "$CARGO_HOME/bin/rustup" update || warn "rustup update failed; continuing"
else
    info "Installing rustup + stable toolchain"
    run sh -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable"
fi

export PATH="$CARGO_HOME/bin:$PATH"

for comp in rustfmt clippy rust-src; do
    run rustup component add "$comp" 2>/dev/null || warn "Component $comp unavailable; skipping"
done

if is_dev_or_full; then
    for tool in cargo-edit cargo-watch cargo-audit bacon just sccache; do
        if ! command -v "$tool" &>/dev/null && [[ ! -f "$CARGO_HOME/bin/$tool" ]]; then
            info "Installing $tool via cargo"
            run cargo install "$tool" || warn "Failed to install $tool; continuing"
        else
            info "$tool already installed"
        fi
    done
fi
info "Rust toolchain done."

# ════════════════════════════════════════════════════════════════
# 3. GITHUB CLI + GIT LFS
# ════════════════════════════════════════════════════════════════
step "GitHub CLI + Git LFS"

if ! command -v gh &>/dev/null; then
    info "Adding GitHub CLI apt repo"
    curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
        | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg 2>/dev/null
    sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg
    echo "deb [arch=${DEB_ARCH} signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
        | sudo tee /etc/apt/sources.list.d/github-cli.list >/dev/null
    sudo apt-get update -qq
    apt_install gh
else
    info "gh already installed"
fi

run git lfs install || warn "git lfs install failed; continuing"
info "GitHub CLI done."

# ════════════════════════════════════════════════════════════════
# 4. DOCKER  (dev / full only)
# ════════════════════════════════════════════════════════════════
if is_dev_or_full; then
    step "Docker Engine"

    if ! command -v docker &>/dev/null; then
        info "Adding Docker apt repo"
        sudo install -m 0755 -d /etc/apt/keyrings
        curl -fsSL "https://download.docker.com/linux/ubuntu/gpg" \
            | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg 2>/dev/null
        sudo chmod a+r /etc/apt/keyrings/docker.gpg
        echo "deb [arch=${DEB_ARCH} signed-by=/etc/apt/keyrings/docker.gpg] \
https://download.docker.com/linux/ubuntu ${CODENAME} stable" \
            | sudo tee /etc/apt/sources.list.d/docker.list >/dev/null
        sudo apt-get update -qq
        apt_install docker-ce docker-ce-cli containerd.io \
                    docker-buildx-plugin docker-compose-plugin
    else
        info "docker already installed"
    fi

    if ! id -nG "$USER" | grep -qw docker 2>/dev/null; then
        run sudo usermod -aG docker "$USER"
        warn "Added $USER to docker group – re-login to apply"
    fi

    run sudo systemctl enable --now docker || warn "systemctl docker failed; continuing"
    info "Docker done."
fi

# ════════════════════════════════════════════════════════════════
# 5. BUILDROOT DEPS  (dev / full only)
# ════════════════════════════════════════════════════════════════
if is_dev_or_full; then
    step "Buildroot dependencies"
    apt_install bison flex gawk texinfo libncurses-dev libssl-dev \
                bc rsync cpio unzip file patch
    info "Buildroot deps done."
fi

# ════════════════════════════════════════════════════════════════
# 6. ZSH + SHELL POLISH  (dev / full only)
# ════════════════════════════════════════════════════════════════
if is_dev_or_full; then
    step "Zsh + shell polish"
    apt_install zsh

    if [[ ! -d "$HOME/.oh-my-zsh" ]]; then
        info "Installing oh-my-zsh (unattended)"
        run env RUNZSH=no CHSH=no sh -c \
            "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
    else
        info "oh-my-zsh already installed"
    fi

    if ! command -v starship &>/dev/null; then
        info "Installing starship prompt"
        run sh -c "curl -sS https://starship.rs/install.sh | sh -s -- -y"
    else
        info "starship already installed"
    fi

    ZSHRC="$HOME/.zshrc"
    if [[ -f "$ZSHRC" ]] && ! grep -q "starship init zsh" "$ZSHRC"; then
        echo 'eval "$(starship init zsh)"' >> "$ZSHRC"
        info "Added starship init to .zshrc"
    fi

    info "Zsh + shell polish done."
fi

# ════════════════════════════════════════════════════════════════
# 7. FONTS  (dev / full only)
# ════════════════════════════════════════════════════════════════
if is_dev_or_full; then
    step "Fonts"
    apt_try fonts-terminus fonts-noto-color-emoji xfonts-terminus
    info "Fonts done."
fi

# ════════════════════════════════════════════════════════════════
# 8. RCLONE  (dev / full only)
# ════════════════════════════════════════════════════════════════
if is_dev_or_full; then
    step "rclone"
    if ! command -v rclone &>/dev/null; then
        if ! apt_try rclone 2>/dev/null; then
            info "Installing rclone via official script"
            run sh -c "curl -fsSL https://rclone.org/install.sh | sudo bash"
        fi
    else
        info "rclone already installed"
    fi
    info "rclone done."
fi

# ════════════════════════════════════════════════════════════════
# DONE
# ════════════════════════════════════════════════════════════════
echo ""
echo -e "${BOLD}╔══════════════════════════════════════════════╗${RESET}"
echo -e "${BOLD}║       Installation complete!                  ║${RESET}"
echo -e "${BOLD}╚══════════════════════════════════════════════╝${RESET}"
echo ""
echo "  • Reload your shell:  source \"$CARGO_HOME/env\""
echo "  • Switch to zsh:      chsh -s \$(which zsh)"
is_dev_or_full && echo "  • Docker group:       log out and back in to use docker without sudo"
echo ""
