# EX_Y03: Dependency Reduction

**Excavation Task**: Dependency Analysis and Reduction
**Status**: ⏳ PLANNING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-03
**Duration**: 4 days

## 🎯 OBJECTIVE

Analyze and reduce dependencies across the MASH codebase to improve compile times, reduce binary sizes, and minimize maintenance burden. Focus on eliminating unnecessary dependencies, consolidating duplicate dependencies, and optimizing dependency usage.

## 📋 TASK BREAKDOWN

### 1. Audit Current Dependencies
**Duration**: 0.5 days

#### 1.1 Generate Dependency Reports
```bash
# Full dependency tree for each crate
for crate in installer-core installer-cli installer-debian installer-arch installer-fedora wallpaper-downloader; do
    echo "=== $crate ===" > "docs/scratch/dependencies_$crate.txt"
    cargo tree -p $crate >> "docs/scratch/dependencies_$crate.txt"
    echo "" >> "docs/scratch/dependencies_$crate.txt"
    cargo bloat -p $crate --release >> "docs/scratch/dependencies_$crate.txt"
done

# Find unused dependencies
cargo udeps > docs/scratch/unused_dependencies.txt

# Find duplicate dependencies
cargo tree -d > docs/scratch/duplicate_dependencies.txt
```

#### 1.2 Analyze Dependency Usage
For each dependency:
- Identify which crates use it
- Determine usage patterns
- Check if it's a direct or transitive dependency
- Document purpose and necessity

#### 1.3 Create Dependency Inventory
Create `docs/scratch/dependency_inventory.md` with:
```markdown
## Dependency Inventory

### Core Dependencies
| Dependency | Version | Used By | Purpose | Necessity |
|------------|---------|---------|--------|-----------|

### Heavy Dependencies (>1MB)
| Dependency | Size | Used By | Purpose | Alternatives |
|------------|------|---------|--------|-------------|

### Unused Dependencies
| Dependency | Found In | Reason |
|------------|----------|--------|

### Duplicate Dependencies
| Dependency | Versions | Used By |
|------------|----------|---------|
```

### 2. Identify Reduction Opportunities
**Duration**: 1 day

#### 2.1 Analyze Heavy Dependencies
Focus on largest dependencies:
- Identify top 10 largest dependencies
- Research lighter alternatives
- Evaluate feature usage
- Document reduction potential

#### 2.2 Review Unused Dependencies
For each unused dependency:
- Verify it's truly unused
- Check for conditional usage
- Determine removal safety
- Document removal plan

#### 2.3 Consolidate Duplicate Dependencies
For duplicate dependencies:
- Identify version conflicts
- Determine compatible version
- Plan consolidation strategy
- Document migration path

#### 2.4 Create Reduction Plan
Create `docs/scratch/dependency_reduction_plan.md` with:
- Prioritized list of dependencies to reduce/remove
- Estimated impact (build time, binary size)
- Migration steps for each
- Risk assessment
- Fallback plans

### 3. Implement Dependency Reductions
**Duration**: 1.5 days

#### 3.1 Remove Unused Dependencies
```bash
# For each unused dependency
cargo remove <dependency>
```

#### 3.2 Replace Heavy Dependencies
Example: Replace reqwest with lighter alternative
```toml
# Before
reqwest = { version = "0.11", features = ["json", "blocking"] }

# After  
ureq = "2.9"
```

#### 3.3 Consolidate Duplicate Dependencies
```toml
# Example: Consolidate serde versions
[workspace.dependencies]
serde = "1.0"

# In each crate
serde = { workspace = true, features = ["derive"] }
```

#### 3.4 Optimize Feature Flags
```toml
# Example: Reduce tokio features
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
```

### 4. Test Reduced Dependencies
**Duration**: 1 day

#### 4.1 Verify Build
```bash
# Clean build with reduced dependencies
cargo clean
cargo build --workspace

# Check for missing dependencies
cargo check --workspace --all-targets
```

#### 4.2 Run Tests
```bash
# Run all tests
cargo test --workspace

# Run specific test suites
cargo test -p installer-core
cargo test -p installer-cli
```

#### 4.3 Measure Impact
```bash
# Measure build time improvement
time cargo build --workspace --release

# Measure binary size reduction
cargo bloat --release --crates

# Compare with baseline
```

#### 4.4 Verify Functionality
- [ ] Test core installer functionality
- [ ] Test CLI interface
- [ ] Test distro-specific installations
- [ ] Test wallpaper downloader
- [ ] Test all integration points

## 🔧 VERIFICATION CHECKLIST

### Audit Verification
- [ ] Dependency reports generated
- [ ] Usage patterns analyzed
- [ ] Inventory created
- [ ] Heavy dependencies identified
- [ ] Unused dependencies documented
- [ ] Duplicates cataloged

### Reduction Planning
- [ ] Reduction opportunities identified
- [ ] Alternatives researched
- [ ] Migration plan created
- [ ] Risk assessment completed
- [ ] Fallback plans documented

### Implementation Verification
- [ ] Unused dependencies removed
- [ ] Heavy dependencies replaced
- [ ] Duplicate dependencies consolidated
- [ ] Feature flags optimized
- [ ] Documentation updated

### Testing Verification
- [ ] Clean build successful
- [ ] All checks passing
- [ ] All tests passing
- [ ] Build time measured
- [ ] Binary size measured
- [ ] Functionality verified

## 📦 DELIVERABLES

1. **Dependency Analysis**
   - `docs/scratch/dependency_inventory.md`
   - Per-crate dependency reports
   - Unused dependencies list
   - Duplicate dependencies list

2. **Reduction Plan**
   - `docs/scratch/dependency_reduction_plan.md`
   - Prioritized reduction list
   - Migration steps
   - Risk assessment

3. **Updated Configurations**
   - Updated Cargo.toml files
   - Optimized feature flags
   - Consolidated dependencies

4. **Impact Measurement**
   - Build time comparison
   - Binary size comparison
   - Dependency count reduction
   - Performance metrics

## 🎯 SUCCESS CRITERIA

### Dependency Reduction
- ✅ 20%+ reduction in total dependencies
- ✅ 15%+ reduction in binary size
- ✅ 10%+ improvement in build times
- ✅ All unused dependencies removed
- ✅ Duplicate dependencies consolidated

### Code Quality
- ✅ Cleaner dependency graph
- ✅ Better dependency hygiene
- ✅ Clear dependency rationale
- ✅ Well-documented dependencies
- ✅ Minimal feature usage

### Performance
- ✅ Faster compile times
- ✅ Smaller binaries
- ✅ Reduced memory usage
- ✅ Better cache efficiency
- ✅ Optimized builds

### Maintainability
- ✅ Easier dependency management
- ✅ Clearer dependency tree
- ✅ Reduced upgrade burden
- ✅ Better version control
- ✅ Improved documentation

## ⚠️ RISKS & MITIGATIONS

| Risk | Mitigation |
|---|---|
| Build failures after removal | Incremental removal, thorough testing |
| Missing functionality | Comprehensive test coverage |
| Performance regressions | Benchmarking, profiling |
| Dependency conflicts | Careful version management |
| Upgrade issues | Semantic versioning compliance |

## 📚 REFERENCE DOCUMENTS

- `docs/forge-tavern/MINING_GOVERNANCE.md` - Governance rules
- `docs/mining-projects/shafts/shaft-y/Overview.md` - Shaft Y overview
- `docs/mining-projects/shafts/shaft-y/EX_Y01_Codebase_Analysis.md` - Analysis results
- `docs/mining-projects/shafts/shaft-y/EX_Y02_Workspace_Splitting.md` - Workspace design
- `Cargo.toml` - Current workspace configuration

"*Fewer dependencies mean lighter builds and happier maintainers.*" — Bard 🍺⚒️