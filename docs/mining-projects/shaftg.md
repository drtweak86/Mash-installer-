# 🏗️ SHAFT G: The Foundation
> **Strategic Mining Plan**
> *“A forge built on sand will crumble. We build on stone, semantics, and equal rights for all distros.”* — Bard 🍺

## 📜 Project Summary
Solidify the codebase. Refactor documentation, ensure distro equality, enforce semantic versioning, and automate distribution.

## 🛠️ Technical Plan (Tasks 7, 8, 9, 14, 17, 18)

### 1. Distro Equality
- **Objective**: Arch, Fedora, and Debian are first-class citizens.
- **Files**: `installer-arch`, `installer-debian`, `installer-fedora`.
- **Action**: Audit phase parity. If Arch gets a feature, Debian gets the equivalent (where possible).

### 2. Semantic Release Rituals
- **Objective**: Automated, predictable versioning.
- **Action**: Tag `vMAJOR.MINOR.PATCH`. Update `Cargo.toml` and lockfiles before tagging.

### 3. Packaging Automata
- **Objective**: Native packages for all.
- **Action**:
    - **Debian**: `cargo-deb` configuration.
    - **Fedora**: `cargo-generate-rpm`.
    - **Arch**: `PKGBUILD` generation.
    - **Integration**: Add to `release.yml`.

### 4. Refactor & Dedup
- **Objective**: Clean code, clean mind.
- **Action**: Consolidate duplicate docs in `docs/legacy`. Remove unused code (as found by clippy).

## 🏗️ Technical Dependencies
- `cargo-deb`, `cargo-generate-rpm`.

## ⚠️ Risks
- **Maintenance Burden**: Keeping three distro paths in sync requires discipline.

---
**Status**: Planned ⏳
**Owner**: Bard
