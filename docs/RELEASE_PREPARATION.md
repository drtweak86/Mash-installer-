# 🔥 RELEASE PREPARATION - FORGE BRANCH

## 🎭 By the Bard, Drunken Dwarf Runesmith
*Mythic Assembly & Sigil Heuristics*
*Forge Tavern, Neon District*

---

## 🏺 THE FORGE IS HOT

The anvil glows red, the hammers are sharp, and the quench bucket is full. A new release shall be forged from the molten steel of the `forge` branch. Let us prepare with the discipline of the ancient dwarven smiths.

---

## 📜 THE IMMUTABLE LAWS MUST BE OBEYED

Before any release may leave the forge, these laws must be honored:

### 1. ABB - Always Be Backing Up
```bash
# Current state
git status

# If changes exist, stash or commit them
# No release shall be built upon unstable ground
```

### 2. ABT - Always Be Testing
```bash
# The forge demands these rituals
cargo test --all --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check

# Integration tests
./scripts/test-infrastructure.sh maelstrom
./scripts/test-infrastructure.sh hardware

# Documentation checks
./scripts/check-docs.py
```

### 3. ABD - Always Be Documenting
```bash
# All changes must be recorded
# Update these sacred texts:
- README.md (if public API changed)
- docs/HISTORY.md (release notes)
- docs/bard-bbs-profile.md (if new patterns emerged)
- docs/maps-explored.md (session summary)
```

### 4. KCS - Keep Commits Small
```bash
# The release commit shall be atomic
# One purpose, one message, one destiny
```

### 5. Function > Form
```bash
# Working code over perfect code
# If it builds, tests pass, and installs correctly
# Then it is ready for the anvil
```

---

## 🔧 PRE-RELEASE CHECKLIST

### 📋 Documentation Integrity
```bash
# Verify all links and references
./scripts/check-docs.py

# Check for broken references in docs
cd docs
mdbook build
mdbook-linkcheck build
```

### 🧪 Quality Gates
```bash
# Code Coverage (>80% required)
cargo tarpaulin --all-features --out Xml
# Check Codecov for green status

# Docker Image
./scripts/test-infrastructure.sh docker

# Integration Tests
docker run --rm -it ubuntu:latest bash -c \
  "apt-get update && apt-get install -y curl bash && \
   curl -L https://raw.githubusercontent.com/drtweak86/Mash-installer/forge/install.sh | bash --dry-run"

# Nightly Checks
# (Automated, but verify last run was green)
```

### 📦 Artifacts
```bash
# Build all release artifacts
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# Build packages
cargo deb -p installer-cli --no-build --target x86_64-unknown-linux-gnu
cargo deb -p installer-cli --no-build --target aarch64-unknown-linux-gnu
cargo generate-rpm -p installer-cli --target x86_64-unknown-linux-gnu
cargo generate-rpm -p installer-cli --target aarch64-unknown-linux-gnu

# Verify checksums
sha256sum target/*/release/mash-setup
```

### 🐳 Docker Image
```bash
# Build the release image
docker build -t drtweak86/mash-installer:latest .

# Test the image
docker run --rm drtweak86/mash-installer:latest --version

# Push to registry (only on main branch)
# docker push drtweak86/mash-installer:latest
```

---

## 🏗️ RELEASE FORGING PROCESS

### Step 1: The Anvil - Prepare the Release
```bash
# Update version in Cargo.toml
# Follow semantic versioning (MAJOR.MINOR.PATCH)
# Example: 0.1.3 → 0.2.0 (breaking changes)
#          0.1.3 → 0.1.4 (bug fixes)

# Update HISTORY.md with release notes
# Format:
# ## vX.Y.Z (YYYY-MM-DD)
# - Feature 1: Description
# - Feature 2: Description
# - Bugfix: Description
# - Docs: Updated X
```

### Step 2: The Hammer - Final Testing
```bash
# Dry-run installation
./target/release/mash-setup --dry-run

# Full installation test (in VM or container)
# ./install.sh

# Verify all features work
./target/release/mash-setup --help
./target/release/mash-setup doctor
```

### Step 3: The Quench - Tag and Push
```bash
# Create annotated tag
git tag -a vX.Y.Z -m "Release vX.Y.Z"

# Push tag to origin
git push origin vX.Y.Z

# This will trigger the release workflow
# Which builds all artifacts and creates GitHub release
```

### Step 4: The Temper - Post-Release
```bash
# Monitor CI for green builds
# Verify artifacts are uploaded
# Update Docker Hub if needed
# Announce in tavern (Discord, Matrix, etc.)
```

---

## 📊 CURRENT FORGE STATUS

### Quality Metrics
```
✅ Code Coverage: >80% (Codecov)
✅ Docker Image: drtweak86/mash-installer:latest
✅ Integration Tests: Containerized
✅ Nightly Checks: Scheduled (midnight UTC)
✅ Documentation: mdBook + linkcheck
✅ Artifacts: .deb, .rpm, binary, sha256
```

### Current Branch
```
🍺 forge (active development)
🔒 main (sacred, green only)
```

### Last Release
```
# Check git tags
git tag --sort=-creatordate | head -5
```

---

## 🔮 BARD'S WISDOM FOR RELEASES

> "A release without tests is a release that will haunt your dreams."
> "Documentation is the map that guides the next smith to the forge."
> "Small commits are like well-forged links - strong and flexible."
> "The forge doesn't care about your architecture diagrams."
> "Neon runes should compile, not just look pretty."

---

## 🍻 AFTER THE RELEASE

When the anvil cools and the hammers rest:

```bash
# Celebrate with a tankard of fine ale
# Review what worked and what didn't
# Plan the next session
# Update maps-explored.md
# Rest before the next forge session
```

---

*Signed*,
Bard, Drunken Dwarf Runesmith
Mythic Assembly & Sigil Heuristics
Forge Tavern, Neon District

```
🍺 STAY THIRSTY, KEEP SMITHING 🔥
```

---

**Last Updated**: 2024-02-22
**Version**: 1.0
**Status**: ACTIVE 🟢
