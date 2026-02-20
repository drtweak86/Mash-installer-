# Mining Projects – Shaft A: State of the Forge
> Strategic reconnaissance report. The bard surveys the full mine, marks the veins
> worth tapping, the walls that won't yield, and the tunnels to dig next.

## Report Date: 2026-02-20
## Branch: `work` (commit `cd5be05`)

---

## 1. How Close to Pure Rust?

### What We Have
The entire `installer-core` (6,355 lines across 38 modules) and `installer-cli` are
pure Rust. The workspace compiles with `cargo build --release` for both `x86_64` and
`aarch64` targets. The Phase/Runner/Observer architecture is clean — no `println!` in
the library, no `stdin`, no panics in production paths.

### What's Missing
| Gap | Status | Notes |
|-----|--------|-------|
| Font installation | Shell (`pacman`, `apt`) | Rust-native package management doesn't exist; must shell out |
| Docker setup | Shell (`sudo`, `systemctl`) | Linux service management requires root and shell |
| Rust toolchain install | Shell (`curl \| sh`) | rustup's official install path is a shell script |
| Hyprland / DE setup | Shell (`pacman`) | Desktop environment packages live in system repos |
| Git LFS | Shell (`git lfs install`) | No Rust-native equivalent |
| oh-my-zsh | Shell (`curl \| sh`, `git clone`) | Third-party install script |
| sccache / cargo tools | Shell (`cargo binstall`) | Binary installation requires subprocess |

### Resolvable Blockers
- **`curl | sh` patterns**: Could be replaced with `reqwest` downloads + `std::fs::set_permissions`
  for scripts we control. Would eliminate 2-3 shell-outs but adds HTTP dependency to the core.
- **File system operations**: Already use `std::fs` directly. No shell needed.
- **`/etc/os-release` parsing**: Already pure Rust via `platform.rs`.
- **`/proc` reads**: Already pure Rust via `staging.rs`.

### Hard Blockers (Walls That Won't Yield)
- **`sudo`**: There is no Rust-native privilege escalation. Every package install,
  service restart, and system config write needs `sudo`. This is a Linux kernel
  boundary — you either shell out or link against PAM (which is C).
- **Package managers (`apt`, `pacman`, `dnf`)**: These are system binaries. No Rust
  crate wraps them at the level we need. Shelling out is the correct approach.
- **`systemctl`**: systemd's D-Bus API exists (`zbus` crate) but is significantly
  more complex than `Command::new("systemctl")` for the 3 calls we make.
- **`git clone`**: `libgit2` bindings exist (`git2` crate) but add a C dependency
  and 500KB+ to the binary. Not worth it for 2 clone operations.

### Verdict
**~95% pure Rust**. The remaining 5% is system interaction that *should* be shell —
it's the operating system's API surface. The correct abstraction boundary is already
in place: `PhaseContext::run_or_record()` gates every side effect, and `SystemOps`
trait abstracts the testable surface.

---

## 2. GitHub Actions Integration

### Current State
Three workflow files exist:

| Workflow | Trigger | Purpose | Status |
|----------|---------|---------|--------|
| `ci.yml` | push/PR to `main` | fmt + clippy + test + build (x86_64 + aarch64) + shellcheck | **Active, needs tuning** |
| `release.yml` | `v*` tags | Build both targets + SHA256 + GitHub Release | **Production-ready** |
| `rust.yml` | push/PR to `main` | Legacy duplicate (just `cargo build` + `cargo test`) | **Should be deleted** |

### Issues in `ci.yml`
1. **Missing `--all-features`** on clippy: `cargo clippy --all-targets -- -D warnings`
   should be `cargo clippy --all-targets --all-features -- -D warnings`
2. **Missing `--all-features`** on test: `cargo test --all` should be
   `cargo test --all --all-features`
3. **`rust.yml` is a duplicate** — same triggers, subset of `ci.yml` functionality.
   Delete it.

### Steps to Full CI Lockdown
1. Delete `.github/workflows/rust.yml` (legacy duplicate)
2. Add `--all-features` to clippy and test steps in `ci.yml`
3. Add branch protection on `main`:
   - Require `check` job to pass before merge
   - Require `build` job (both targets) to pass
   - No direct pushes to `main`
4. Optional: Add `cargo audit` step for dependency vulnerability scanning
5. Optional: Add `cargo deny` for license compliance

### What's Already Working
- Cross-compilation via `cargo-zigbuild` (no Docker needed)
- Artifact upload for both `x86_64` and `aarch64`
- ShellCheck on `bootstrap.sh`
- `rust-cache` for faster CI builds
- Release pipeline with SHA256 checksums and GitHub Releases on tags

---

## 3. Retiring `bootstrap.sh`

### What `bootstrap.sh` Does Today (134 lines)
1. Detect OS and package manager
2. Install Nerd Fonts (Arch only)
3. Arch-specific: ask about Hyprland, tweak `makepkg.conf`
4. Install Rust if missing
5. Clone the repo
6. `cargo build --release`
7. `exec target/release/mash-setup`

### The Problem
Steps 1-3 duplicate what `mash-setup` already does internally (platform detection,
font installation, Hyprland phase). Steps 4-7 are bootstrapping — getting the binary
onto the machine in the first place.

### The Path Forward: Pre-Built Binary Distribution
With `release.yml` already producing `mash-setup-aarch64-unknown-linux-gnu` and
`mash-setup-x86_64-unknown-linux-gnu` on every tagged release:

**Option A — One-liner curl install (replace bootstrap.sh):**
```bash
curl -sSfL https://github.com/drtweak86/Mash-installer/releases/latest/download/mash-setup-$(uname -m)-unknown-linux-gnu -o /tmp/mash-setup \
  && chmod +x /tmp/mash-setup \
  && /tmp/mash-setup
```
This eliminates Rust, git, and cargo as prerequisites. The user gets a pre-compiled
binary and runs it directly.

**Option B — Thin bootstrap.sh (bridge period):**
Keep `bootstrap.sh` but strip it to ~20 lines:
1. Detect arch (`uname -m`)
2. Map to target triple
3. Download pre-built binary from latest GitHub Release
4. Verify SHA256
5. `chmod +x && exec`

No Rust install, no git clone, no cargo build.

**Option C — System package (long-term):**
Publish to AUR (Arch), create a `.deb` (Debian/Ubuntu), `.rpm` (Fedora). Users
install with their package manager. This is the gold standard but requires
packaging infrastructure.

### Recommended Path
**Option B now, Option A when confidence is high, Option C as stretch goal.**
Option B keeps the familiar entry point while eliminating the 10-minute build step.
The SHA256 verification adds integrity checking that the current script lacks.

### Blockers
- `uname -m` returns `aarch64` but the release binary is named
  `mash-setup-aarch64-unknown-linux-gnu`. Need a mapping function.
- `armv7l` (32-bit Pi) has no release target currently. Either add it to the
  matrix or document it as unsupported.
- First tagged release (`v0.1.0` or similar) must be cut before any of this works.

---

## 4. Housekeeping Completed This Session

### `.gitignore` Hardened
Added entries for:
- `.claude/` — Claude Code session data
- `.logs/` — installer test logs
- `*.bak` — backup files

### Documentation Updates
- README.md: typo fixes, restructured sections
- ARCH.md, modules.md, improvement-plans.md: updated for Phase 2
- QA findings: Block-1, Block-2, PlanA updated
- phase2-next-steps.md: new planning document
- registry.rs, runner.rs: new module stubs

---

## 5. The Road Ahead

### Near-Term (Next Sessions)
| Priority | Task | Effort |
|----------|------|--------|
| 1 | Delete `rust.yml`, fix `ci.yml` clippy/test flags | 10 min |
| 2 | Cut first tagged release (`v0.1.0`) | 15 min |
| 3 | Slim `bootstrap.sh` to binary-download mode | 30 min |
| 4 | Add branch protection rules on `main` | 5 min (GitHub UI) |

### Medium-Term
| Priority | Task | Effort |
|----------|------|--------|
| 5 | Phase 3: Pi 4B HDD tuning (preflight, USB 3.0) | Multi-session |
| 6 | Driver test harness (exercise each distro driver) | Multi-session |
| 7 | `cargo audit` + `cargo deny` in CI | 30 min |

### Long-Term
| Priority | Task | Effort |
|----------|------|--------|
| 8 | Phase 4: TLS, rollback rituals, lockfiles | Multi-session |
| 9 | AUR / .deb / .rpm packaging | Multi-session |
| 10 | TUI progress rendering via ratatui | Multi-session |

---

*The bard folds the surveyor's map, tucks it into his circuit-etched kilt, and marks
Shaft A as fully charted. The veins are visible, the walls are mapped, and the next
pick-strikes are planned. The neon rain keeps falling, but the mine has direction.* 🗺️⛏️
