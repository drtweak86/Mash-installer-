#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────
#  mash-installer bootstrap
#
#  Tiny bootstrap script that:
#    1. Installs minimal prerequisites via the system package manager.
#    2. Downloads the prebuilt mash-setup binary from GitHub Releases.
#    3. Verifies sha256 if a checksum file is available.
#    4. Runs the installer.
#
#  Supports: Ubuntu/Debian (apt) and Manjaro/Arch (pacman).
#
#  Usage:
#    curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh | bash
#    # or
#    ./bootstrap.sh [--profile dev|minimal|full] [--staging-dir /path] [--dry-run]
# ─────────────────────────────────────────────────────────────────
set -euo pipefail

REPO="drtweak86/Mash-installer-"
BINARY_PREFIX="mash-setup"
INSTALL_DIR="/usr/local/bin"

# ── colours (if tty) ─────────────────────────────────────────────
if [ -t 2 ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    BOLD='\033[1m'
    RESET='\033[0m'
else
    RED='' GREEN='' YELLOW='' BOLD='' RESET=''
fi

info()  { echo -e "${GREEN}[info]${RESET}  $*" >&2; }
warn()  { echo -e "${YELLOW}[warn]${RESET}  $*" >&2; }
error() { echo -e "${RED}[error]${RESET} $*" >&2; }
die()   { error "$@"; exit 1; }

# ── detect package manager ──────────────────────────────────────
detect_pkg_manager() {
    if command -v pacman &>/dev/null; then
        echo "pacman"
    elif command -v apt-get &>/dev/null; then
        echo "apt"
    else
        die "No supported package manager found (need apt or pacman)."
    fi
}

# ── detect architecture ─────────────────────────────────────────
detect_arch() {
    local arch
    arch="$(uname -m)"
    case "$arch" in
        aarch64|arm64) echo "aarch64-unknown-linux-gnu" ;;
        x86_64|amd64)  echo "x86_64-unknown-linux-gnu" ;;
        armv7l|armhf)
            die "Unsupported architecture: $arch (32-bit ARM). Switch to a 64-bit OS (aarch64) to run this installer."
            ;;
        *)
            die "Unsupported architecture: $arch"
            ;;
    esac
}

# ── install minimal deps (apt) ──────────────────────────────────
install_deps_apt() {
    info "Installing minimal prerequisites via apt …"
    local pkgs=(ca-certificates curl xz-utils tar coreutils git)

    if ! command -v jq &>/dev/null; then
        pkgs+=(jq)
    fi

    if command -v sudo &>/dev/null; then
        sudo apt-get update -qq
        sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --install-recommends "${pkgs[@]}"
    else
        apt-get update -qq
        DEBIAN_FRONTEND=noninteractive apt-get install -y --install-recommends "${pkgs[@]}"
    fi
}

# ── install minimal deps (pacman) ───────────────────────────────
install_deps_pacman() {
    info "Installing minimal prerequisites via pacman …"
    local pkgs=(ca-certificates curl xz tar coreutils git)

    if ! command -v jq &>/dev/null; then
        pkgs+=(jq)
    fi

    if command -v sudo &>/dev/null; then
        sudo pacman -Syu --noconfirm --needed "${pkgs[@]}"
    else
        pacman -Syu --noconfirm --needed "${pkgs[@]}"
    fi
}

# ── install minimal deps (dispatch) ─────────────────────────────
install_deps() {
    local mgr
    mgr="$(detect_pkg_manager)"
    case "$mgr" in
        apt)    install_deps_apt ;;
        pacman) install_deps_pacman ;;
    esac
}

# ── find latest release tag ─────────────────────────────────────
latest_release_tag() {
    local api_url="https://api.github.com/repos/${REPO}/releases/latest"
    if command -v jq &>/dev/null; then
        curl -fsSL "$api_url" | jq -r '.tag_name'
    else
        # fallback: parse JSON with grep/sed
        curl -fsSL "$api_url" | grep '"tag_name"' | head -1 | sed -E 's/.*"tag_name":\s*"([^"]+)".*/\1/'
    fi
}

# ── download binary ──────────────────────────────────────────────
download_binary() {
    local tag="$1"
    local target="$2"
    local asset_name="${BINARY_PREFIX}-${target}"
    local base_url="https://github.com/${REPO}/releases/download/${tag}"
    local tmp_dir
    tmp_dir="$(mktemp -d)"

    info "Downloading ${asset_name} (${tag}) …"
    curl -fsSL -o "${tmp_dir}/${asset_name}" "${base_url}/${asset_name}" \
        || die "Failed to download ${base_url}/${asset_name}"

    # Require checksum verification
    local sha_file="${asset_name}.sha256"
    if ! curl -fsSL -o "${tmp_dir}/${sha_file}" "${base_url}/${sha_file}"; then
        die "Checksum file ${sha_file} missing for ${asset_name}"
    fi
    info "Verifying SHA-256 checksum …"
    (cd "$tmp_dir" && sha256sum -c "$sha_file" >&2) \
        || die "Checksum verification failed!"
    info "Checksum OK"

    chmod +x "${tmp_dir}/${asset_name}"
    echo "${tmp_dir}/${asset_name}"
}

# ── main ─────────────────────────────────────────────────────────
main() {
    echo ""
    echo -e "${BOLD}╔══════════════════════════════════════════════╗${RESET}"
    echo -e "${BOLD}║       mash-installer bootstrap                ║${RESET}"
    echo -e "${BOLD}╚══════════════════════════════════════════════╝${RESET}"
    echo ""

    # Check we're on Linux
    [[ "$(uname -s)" == "Linux" ]] || die "This script is for Linux only."

    # Install deps
    install_deps

    # Detect arch
    local target
    target="$(detect_arch)"
    info "Detected target: ${target}"

    # Get latest release
    local tag
    tag="$(latest_release_tag)"
    if [[ -z "$tag" || "$tag" == "null" ]]; then
        die "Could not determine latest release. Check https://github.com/${REPO}/releases"
    fi
    info "Latest release: ${tag}"

    # Download
    local binary_path
    binary_path="$(download_binary "$tag" "$target")"

    # Install to PATH
    if command -v sudo &>/dev/null; then
        sudo install -m 0755 "$binary_path" "${INSTALL_DIR}/${BINARY_PREFIX}"
    else
        install -m 0755 "$binary_path" "${INSTALL_DIR}/${BINARY_PREFIX}"
    fi
    info "Installed ${BINARY_PREFIX} to ${INSTALL_DIR}/${BINARY_PREFIX}"

    # Clean up temp
    rm -rf "$(dirname "$binary_path")"

    # Run the installer non-interactively, passing through any arguments
    info "Running: ${BINARY_PREFIX} --non-interactive $*"
    echo ""
    "${INSTALL_DIR}/${BINARY_PREFIX}" --non-interactive "$@"
}

# Default to dev profile; pass --profile <level> to override
if [[ $# -eq 0 ]]; then
    main --profile dev
else
    main "$@"
fi
