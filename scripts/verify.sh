#!/bin/bash
set -e

# THE HOLY TRINITY + EXTENDED INSPECTION
echo "⚒️  STARTING FORGE VERIFICATION..."

echo "1/7 [FMT] Checking runes alignment..."
cargo fmt --all -- --check

echo "2/7 [CLIPPY] Inspecting for logical soot..."
cargo clippy --all-targets --all-features -- -D warnings

echo "3/7 [TEST] Proving changes in the fire..."
if command -v maelstrom-cargo >/dev/null 2>&1; then
    maelstrom-cargo test
else
    cargo test --workspace
fi

echo "4/7 [BUILD] Hardening the workspace..."
cargo build --workspace

echo "5/7 [TARPAULIN] Measuring plasma coverage..."
if command -v cargo-tarpaulin >/dev/null 2>&1; then
    cargo tarpaulin --out Xml
else
    echo "⚠️  cargo-tarpaulin not found, skipping coverage."
fi

echo "6/7 [AUDIT] Checking for dragon-breath vulnerabilities..."
if command -v cargo-audit >/dev/null 2>&1; then
    cargo audit
else
    echo "⚠️  cargo-audit not found, skipping audit."
fi

echo "7/7 [DENY] Verifying dependency pedigree..."
if command -v cargo-deny >/dev/null 2>&1; then
    cargo deny check
else
    echo "⚠️  cargo-deny not found, skipping deny check."
fi

echo "✅ FORGE FIRE APPROVES THE BLADE."
