#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-maelstrom}"
LOG_DIR=".logs"
mkdir -p "$LOG_DIR"
TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
LOG_FILE="$LOG_DIR/test-${MODE}-${TIMESTAMP}.log"

run_cargo_test() {
  echo "Running cargo test (maelstrom unavailable)..." | tee "$LOG_FILE"
  CARGO_HOME=/tmp/cargo RUST_TEST_THREADS=1 cargo test -p installer-core 2>&1 | tee -a "$LOG_FILE"
}

case "$MODE" in
  maelstrom)
    if command -v maelstrom >/dev/null 2>&1; then
      echo "Running maelstrom-compatible tests..." | tee "$LOG_FILE"
      maelstrom test -p installer-core 2>&1 | tee -a "$LOG_FILE"
    else
      echo "maelstrom not found; falling back to cargo test" | tee "$LOG_FILE"
      run_cargo_test
    fi
    ;;
  hardware)
    echo "Running hardware/kernel-dependent test suite..." | tee "$LOG_FILE"
    echo "  * Ensure the target hardware or VM is provisioned (e.g., aarch64 via QEMU)." | tee -a "$LOG_FILE"
    CARGO_HOME=/tmp/cargo RUST_TEST_THREADS=1 cargo test -p installer-core -- --ignored 2>&1 | tee -a "$LOG_FILE"
    ;;
  *)
    echo "Unknown test mode '$MODE'. Supported modes: maelstrom, hardware." | tee "$LOG_FILE"
    exit 1
    ;;
esac
