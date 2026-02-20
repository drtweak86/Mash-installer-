#!/usr/bin/env bash
set -euo pipefail

download_target() {
  local arch="$1"
  case "$arch" in
    x86_64) echo "x86_64" ;;
    aarch64|arm64) echo "aarch64" ;;
    *)
      printf 'Unsupported architecture: %s\n' "$arch" >&2
      return 1
      ;;
  esac
}

target_arch=$(download_target "$(uname -m)")
file="mash-setup-${target_arch}-unknown-linux-gnu"
url="https://github.com/drtweak86/Mash-installer/releases/latest/download/${file}"

staging_dir="$(mktemp -d "${TMPDIR:-/tmp}/mash-installer.XXXXXXXX")"
trap 'rm -rf "${staging_dir}"' EXIT INT TERM

curl -fsSL -o "${staging_dir}/${file}" "${url}"
chmod +x "${staging_dir}/${file}"
exec "${staging_dir}/${file}" "$@"
