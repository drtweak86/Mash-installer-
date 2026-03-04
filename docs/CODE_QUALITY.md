# 💎 Code Quality Standards

This document defines the minimum quality standards for the MASH Installer project. Adherence to these standards is mandatory for all contributions.

## 📏 CODE LIMITS

To maintain readability and modularity, the following limits are enforced:

- **Function Length**: Maximum 50 lines of executable code.
- **Nesting Depth**: Maximum 3 levels of nesting.
- **Cyclomatic Complexity**: Maximum 15 per function.
- **File Length**: Maximum 600 lines. Files exceeding this must be modularized into a directory structure.
- **Module Size**: Prefer small, focused modules over monolithic ones.

## 🧪 TESTING & COVERAGE

- **Mandatory Coverage**: All new features must include unit tests. Total workspace coverage must remain **above 80%**.
- **Distributed Testing**: Use `maelstrom-cargo` for parallel, containerized test execution.
- **Snapshot Testing**: Use `insta` for verifying complex outputs (CLI/TUI).

## 🛡️ CI/CD GREEN GATE REQUIREMENTS

For a PR to be considered "Green" and ready for merge, the following must pass:

1. **`cargo fmt --check`**: Zero formatting deviations.
2. **`cargo clippy -- -D warnings`**: Zero warnings allowed.
3. **`cargo test`**: All tests must pass (distributed or local).
4. **`cargo build`**: Workspace must build without errors.
5. **`cargo tarpaulin`**: Coverage must meet the 80% threshold.
6. **`cargo audit`**: No known security vulnerabilities in dependencies.
7. **`cargo deny check`**: All licenses and advisory checks must pass.

## ✍️ DOCUMENTATION STANDARDS

- **Public APIs**: Every public struct, enum, and function must have a triple-slash (`///`) doc comment.
- **Architectural Decisions**: Major changes must be documented in `docs/mining-projects/`.
- **In-code Comments**: Explain the *why*, not the *what*. If the *what* is unclear, refactor the code.

---
*"A clean blade is a sharp blade. A clean forge is a fast forge."* — Bard 🍺⚒️
