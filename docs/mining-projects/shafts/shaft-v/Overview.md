# Shaft V: THE INTERACTIVE FORGE (Installation Flow & Authorizations)

**Shaft Title**: THE INTERACTIVE FORGE (Installation Flow & Authorizations)
**Status**: ⏳ PLANNING COMPLETE | 🌑 IMPLEMENTATION PENDING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-01

## 🎯 SCOPE

This shaft focuses on perfecting the installation sequence and enabling "Living Installations" through interactive setup. It moves beyond simple binary installation to fully configured environments.

1. **Dependency-First Orchestration**: Ensure dependencies (like `sccache`, `build-essential`, or `git`) are installed and verified before dependent tools are touched.
2. **Interactive Authorizations**: Prompt for tool-specific authorizations (e.g., `gh auth login`, `ssh-keygen`) immediately after installation.
3. **Post-Install Verification**: Verify not just the file presence, but the functionality of installed tools via the `doctor` subsystem.
4. **Git Personalization**: Interactive setup of global `user.name` and `user.email`.

## 📁 FILES TO BE CREATED OR TOUCHED

### New Files
- `installer-core/src/authorization.rs` - Interactive authorization logic
- `installer-core/src/dependency_graph.rs` - Topological sorting of phases
- `installer-cli/src/tui/auth_modal.rs` - Interactive authorization modals

### Modified Files
- `installer-core/src/orchestrator.rs` - Reorder execution based on dependency graph
- `installer-core/src/doctor.rs` - Extend with post-install verification
- `installer-core/src/phase_runner.rs` - Integrate prerequisite gates
- `installer-cli/src/main.rs` - Add flags for interactive vs. automated auth

## ⚒️ METHODOLOGY

### Technical Strategy
1. **Topological Sort**: Dynamically order installation phases based on a dependency graph.
2. **Interactive Hooks**: Introduce "Post-Phase Hooks" that trigger user interaction for tools requiring login or setup.
3. **Painless Authorizations**: Use `gh` and other CLI tools' native interactive flows, but integrated into the TUI context where possible.
4. **Prerequisite Gates**: Stop dependent installations early if their prerequisites fail.

## 📦 DELIVERABLES

### Phase 1: Dependency-First Orchestration ✅ PLANNED
- [ ] Implement a topological sort for the `PhaseRunner`.
- [ ] Add explicit dependency declarations to each `Phase` trait implementation.
- [ ] Implement "Prerequisite Gates" that skip dependent tasks on failure.

### Phase 2: Interactive Authorizations ✅ PLANNED
- [ ] Implement interactive `gh auth login` flow.
- [ ] Implement interactive `ssh-keygen` and GitHub SSH registration.
- [ ] Implement interactive `git config --global` setup.
- [ ] Optional: Add current user to `docker` group with warning.

### Phase 3: Post-Install Verification ✅ PLANNED
- [ ] Wire `doctor` to run automatically after each major tool installation.
- [ ] Report functionality status in the final "Task Completion Box".

## 🔧 VERIFICATION CHECKLIST
- [ ] Installation order follows logical dependencies in dry-run mode.
- [ ] Authorization prompts trigger correctly after tool installation.
- [ ] `doctor` reports reflect actual functional state of tools.
- [ ] User can skip interactive auth for fully automated runs.

"*An installed tool is useless if it's locked. Time to forge the keys.*" — Bard 🍺⚒️
