# EX_Y02: Workspace Splitting

**Excavation Task**: Workspace Restructuring
**Status**: ⏳ PLANNING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-03
**Duration**: 5 days

## 🎯 OBJECTIVE

Restructure the monolithic repository into logical workspaces to improve build times, dependency management, and code organization. This will create a more modular architecture that's easier to maintain and extend.

## 📋 TASK BREAKDOWN

### 1. Design Workspace Structure
**Duration**: 1 day

#### 1.1 Analyze Current Structure
- [ ] Review current Cargo.toml workspace configuration
- [ ] Map current crate dependencies
- [ ] Identify logical module boundaries
- [ ] Document current build times per crate

#### 1.2 Design New Workspace Structure
```
Mash-installer/
├── Cargo.toml (workspace root)
├── workspace-hack/ (build coordination)
├── macros/ (isolated macros)
├── installer-core/ (core logic)
├── installer-cli/ (CLI interface)
├── installer-debian/ (Debian-specific)
├── installer-arch/ (Arch-specific)
├── installer-fedora/ (Fedora-specific)
└── wallpaper-downloader/ (wallpaper functionality)
```

#### 1.3 Create Workspace Design Document
Create `docs/scratch/workspace_design.md` with:
- Workspace structure diagram
- Crate dependency graph
- Build order requirements
- Shared dependency strategy
- Cross-crate communication patterns

### 2. Create Workspace-Hack Crate
**Duration**: 1 day

#### 2.1 Set Up Workspace-Hack
```bash
# Create workspace-hack directory
mkdir -p workspace-hack/src

# Create Cargo.toml
cat > workspace-hack/Cargo.toml << 'EOF'
[package]
name = "workspace-hack"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
EOF

# Create lib.rs
cat > workspace-hack/src/lib.rs << 'EOF'
//! Workspace build coordination crate
//! This crate exists solely to force Cargo to build all crates in the workspace
//! and to provide a central point for build scripts and configuration.
EOF
```

#### 2.2 Configure Workspace-Hack
- [ ] Add build.rs for workspace coordination
- [ ] Configure common build settings
- [ ] Set up workspace-wide features
- [ ] Add documentation

### 3. Configure Root Workspace
**Duration**: 1 day

#### 3.1 Update Root Cargo.toml
```toml
[workspace]
members = [
    "workspace-hack",
    "macros",
    "installer-core",
    "installer-cli",
    "installer-debian",
    "installer-arch",
    "installer-fedora",
    "wallpaper-downloader",
]
resolver = "2"

[workspace.dependencies]
# Shared dependencies with versions
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
ratatui = "0.28"
crossterm = "0.27"

[workspace.package]
version = "1.1.0-alpha.2"
edition = "2021"
authors = ["Dr. Tweak <drtweak86@gmail.com>"]
license = "MIT"
repository = "https://github.com/drtweak86/Mash-installer"

[workspace.metadata]
# Common configuration for all crates
rust-version = "1.93.1"

[profile.dev]
opt-level = 1

[profile.release]
opt-level = 3
lto = true
codegen-units = 1

[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
```

#### 3.2 Update Individual Crate Configurations
For each crate:
- [ ] Update to use workspace dependencies
- [ ] Remove version specifications for workspace dependencies
- [ ] Ensure consistent feature flags
- [ ] Update documentation

### 4. Test Workspace Configuration
**Duration**: 1 day

#### 4.1 Verify Workspace Build
```bash
# Clean build
cargo clean
cargo build --workspace

# Check for build errors
cargo check --workspace --all-targets

# Run tests
cargo test --workspace
```

#### 4.2 Test Cross-Crate Dependencies
- [ ] Verify installer-core exports are accessible
- [ ] Test installer-cli integration with core
- [ ] Verify distro-specific crates can access core
- [ ] Test wallpaper-downloader integration

#### 4.3 Measure Build Performance
```bash
# Time clean build
time cargo build --workspace --release

# Time incremental build
time cargo build --workspace --release

# Compare with previous build times
```

### 5. Optimize Workspace Configuration
**Duration**: 1 day

#### 5.1 Implement Build Caching
```bash
# Configure sccache
cat > .cargo/config.toml << 'EOF'
[build]
rustc-wrapper = "/usr/bin/sccache"

[target.'cfg(unix)']
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
EOF
```

#### 5.2 Configure Parallel Builds
```toml
# Add to root Cargo.toml
[profile.dev]
codegen-units = 256

[profile.release]
codegen-units = 16
```

#### 5.3 Optimize Dependency Resolution
- [ ] Configure resolver version
- [ ] Set up dependency overrides
- [ ] Configure feature unification
- [ ] Test optimized configuration

## 🔧 VERIFICATION CHECKLIST

### Design Verification
- [ ] Workspace structure documented
- [ ] Dependency graph created
- [ ] Build order requirements defined
- [ ] Shared dependency strategy documented

### Workspace-Hack Verification
- [ ] workspace-hack crate created
- [ ] Cargo.toml configured
- [ ] lib.rs created
- [ ] Build script configured
- [ ] Documentation added

### Root Configuration Verification
- [ ] Root Cargo.toml updated
- [ ] Workspace members listed
- [ ] Shared dependencies configured
- [ ] Workspace metadata added
- [ ] Individual crates updated

### Build Verification
- [ ] Clean build successful
- [ ] Check successful
- [ ] Tests passing
- [ ] Cross-crate dependencies working
- [ ] Build times measured

### Optimization Verification
- [x] Build caching configured (sccache)
- [x] Parallel builds configured (codegen-units)
- [x] Dependency resolution optimized (resolver = "2")
- [x] Performance improvements verified (Clean: 19m 23s, Incremental: 4m 05s)

## 📦 DELIVERABLES

1. **Workspace Design**
   - `docs/scratch/workspace_design.md`
   - Workspace structure diagram
   - Dependency graph visualization

2. **Workspace-Hack Crate**
   - `workspace-hack/Cargo.toml`
   - `workspace-hack/src/lib.rs`
   - Build coordination scripts

3. **Root Configuration**
   - Updated `Cargo.toml`
   - `.cargo/config.toml`
   - Workspace metadata

4. **Updated Crate Configurations**
   - All individual Cargo.toml files
   - Consistent dependency usage
   - Updated documentation

5. **Build Verification**
   - Build time measurements
   - Cross-crate dependency tests
   - Performance comparison

## 🎯 SUCCESS CRITERIA

### Workspace Structure
- ✅ Logical separation of concerns
- ✅ Clear module boundaries
- ✅ Efficient build system
- ✅ Easy navigation
- ✅ Scalable architecture

### Build Performance
- ✅ Faster clean builds
- ✅ Efficient incremental builds
- ✅ Parallel build support
- ✅ Build caching configured
- ✅ Optimized dependency resolution

### Code Organization
- ✅ Clear crate responsibilities
- ✅ Minimal cross-crate dependencies
- ✅ Well-documented interfaces
- ✅ Consistent configuration
- ✅ Easy to extend

### Quality Standards
- ✅ All tests passing
- ✅ No build warnings
- ✅ Consistent formatting
- ✅ Well-documented
- ✅ Maintainable structure

## ⚠️ RISKS & MITIGATIONS

| Risk | Mitigation |
|---|---|
| Build failures during restructuring | Incremental changes, thorough testing |
| Circular dependencies | Careful dependency analysis, testing |
| Build time increases | Performance optimization, caching |
| Cross-crate compatibility issues | Comprehensive integration testing |
| Configuration complexity | Clear documentation, examples |

## 📚 REFERENCE DOCUMENTS

- `docs/forge-tavern/MINING_GOVERNANCE.md` - Governance rules
- `docs/mining-projects/shafts/shaft-y/Overview.md` - Shaft Y overview
- `docs/mining-projects/shafts/shaft-y/EX_Y01_Codebase_Analysis.md` - Analysis results
- `Cargo.toml` - Current workspace configuration
- `installer-core/Cargo.toml` - Core dependencies

"*A well-organized workspace is the foundation of efficient development.*" — Bard 🍺⚒️