#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

banner() {
  echo
  echo "==== RELEASE CHECKLIST: $1 ===="
}

banner "Ensure repository is clean"
git -C "$ROOT" status -sb

banner "Verify documentation integrity"
"$ROOT/scripts/check-docs.py"

banner "Run maelstrom-compatible suite"
"$ROOT/scripts/test-infrastructure.sh" maelstrom

banner "Run hardware/kernel-dependent suite"
"$ROOT/scripts/test-infrastructure.sh" hardware

banner "Manual release reminders"
cat <<'REM'
- Update documentation (ARCH, REPORT, modules, etc.) and ensure links are fresh.
- Update the changelog and version in Cargo.toml (consider using Conventional Commits/`cargo-release`).
- Run the dry-run (`mash-setup --dry-run` or `mash-setup doctor`) to exercise the new logging/reporting.
- After verification, tag the release, push tags, and deploy documentation.
REM
