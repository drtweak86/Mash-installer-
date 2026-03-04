# EX_Y01: Codebase Analysis

**Excavation Task**: Comprehensive Codebase Analysis
**Status**: ⏳ PLANNING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-03
**Duration**: 3 days

## 🎯 OBJECTIVE

Perform a comprehensive analysis of the MASH codebase to identify technical debt, performance bottlenecks, dependency issues, and macro usage patterns. This analysis will serve as the foundation for all subsequent restructuring work.

## 📋 TASK BREAKDOWN

### 1. Set Up Analysis Tools
**Duration**: 0.5 days

#### 1.1 Install Required Tools
```bash
# Install analysis tools
cargo install cargo-tree
cargo install cargo-bloat
cargo install cargo-audit
cargo install cargo-udeps
cargo install cargo-tarpaulin

# Install profiling tools
cargo install flamegraph
cargo install hyperfine
```

#### 1.2 Verify Tool Installation
- [ ] `cargo tree --version`
- [ ] `cargo bloat --version`
- [ ] `cargo audit --version`
- [ ] `cargo udeps --version`
- [ ] `cargo tarpaulin --version`

### 2. Dependency Analysis
**Duration**: 0.5 days

#### 2.1 Generate Dependency Graph
```bash
# Full dependency tree
cargo tree -i inverted > docs/scratch/dependency_tree_full.txt

# Per-crate dependency trees
for crate in installer-core installer-cli installer-debian installer-arch installer-fedora wallpaper-downloader; do
    echo "=== $crate ===" >> docs/scratch/dependency_tree_per_crate.txt
    cargo tree -p $crate >> docs/scratch/dependency_tree_per_crate.txt
    echo "" >> docs/scratch/dependency_tree_per_crate.txt
done
```

#### 2.2 Identify Heavy Dependencies
```bash
# Find largest dependencies
cargo bloat --crates --release > docs/scratch/dependency_sizes.txt

# Find unused dependencies
cargo udeps > docs/scratch/unused_dependencies.txt
```

#### 2.3 Document Findings
Create `docs/scratch/dependency_analysis.md` with:
- Dependency graph visualization
- Top 10 largest dependencies
- Unused dependencies list
- Dependency duplication analysis
- Recommendations for reduction

### 3. Macro Usage Analysis
**Duration**: 0.5 days

#### 3.1 Catalog All Macros
```bash
# Find all macro definitions
grep -r "macro_rules!" --include="*.rs" . > docs/scratch/macro_definitions.txt

# Find all macro usages
grep -r "!" --include="*.rs" . | grep -E "(println|format|vec|hash)" > docs/scratch/macro_usage.txt
```

#### 3.2 Analyze Macro Complexity
For each macro found:
- Count lines of code
- Count parameters
- Count pattern matches
- Document purpose and usage frequency

#### 3.3 Create Macro Catalog
Create `docs/scratch/macro_catalog.md` with:
```markdown
## Macro Catalog

### Frequently Used Macros
| Macro | LOC | Parameters | Usage Count | Purpose |
|-------|-----|-----------|------------|---------|

### Rarely Used Macros
| Macro | LOC | Parameters | Usage Count | Purpose |
|-------|-----|-----------|------------|---------|

### Complex Macros (>20 LOC)
| Macro | LOC | Parameters | Complexity | Purpose |
|-------|-----|-----------|-----------|---------|
```

### 4. Performance Profiling
**Duration**: 0.5 days

#### 4.1 Build Time Analysis
```bash
# Clean build timing
cargo clean
time cargo build --release > docs/scratch/build_time_clean.txt

# Incremental build timing
cargo build --release > docs/scratch/build_time_incremental.txt

# Per-crate build timing
for crate in installer-core installer-cli installer-debian installer-arch installer-fedora wallpaper-downloader; do
    echo "=== $crate ===" >> docs/scratch/build_time_per_crate.txt
    time cargo build -p $crate --release >> docs/scratch/build_time_per_crate.txt 2>&1
    echo "" >> docs/scratch/build_time_per_crate.txt
done
```

#### 4.2 Binary Size Analysis
```bash
# Analyze binary sizes
cargo bloat --release --crates > docs/scratch/binary_size_analysis.txt

# Detailed function analysis
cargo bloat --release -n 50 > docs/scratch/top_functions.txt
```

#### 4.3 Create Performance Report
Create `docs/scratch/performance_analysis.md` with:
- Clean vs incremental build times
- Per-crate build times
- Binary size breakdown
- Top 50 largest functions
- Recommendations for optimization

### 5. Code Quality Analysis
**Duration**: 0.5 days

#### 5.1 Run Clippy Analysis
```bash
# Run clippy with all warnings
cargo clippy --all-targets --all-features -- -D warnings > docs/scratch/clippy_warnings.txt

# Run clippy with pedantic warnings
cargo clippy --all-targets --all-features -- -W clippy::pedantic > docs/scratch/clippy_pedantic.txt
```

#### 5.2 Run Rustfmt Check
```bash
# Check formatting
cargo fmt --check > docs/scratch/rustfmt_check.txt
```

#### 5.3 Code Complexity Analysis
```bash
# Install tokei for code metrics
cargo install tokei

# Generate code statistics
tokei --output json > docs/scratch/code_metrics.json
```

#### 5.4 Create Code Quality Report
Create `docs/scratch/code_quality_analysis.md` with:
- Clippy warning count and categories
- Formatting issues
- Code complexity metrics
- Function length analysis
- Cyclomatic complexity analysis
- Recommendations for improvement

### 6. Technical Debt Identification
**Duration**: 0.5 days

#### 6.1 Review TODO Comments
```bash
# Find all TODO comments
grep -r "TODO" --include="*.rs" . > docs/scratch/todo_comments.txt

# Find all FIXME comments
grep -r "FIXME" --include="*.rs" . > docs/scratch/fixme_comments.txt

# Find all HACK comments
grep -r "HACK" --include="*.rs" . > docs/scratch/hack_comments.txt
```

#### 6.2 Identify Code Smells
- Large functions (>50 lines)
- Complex conditionals (nested >3 levels)
- Duplicate code patterns
- Overly complex type signatures
- Excessive trait bounds

#### 6.3 Create Technical Debt Report
Create `docs/scratch/technical_debt.md` with:
```markdown
## Technical Debt Report

### TODO Items
- [ ] List of TODO items with locations

### FIXME Items
- [ ] List of FIXME items with locations

### HACK Items
- [ ] List of HACK items with locations

### Code Smells
- [ ] Large functions (>50 lines)
- [ ] Complex conditionals
- [ ] Duplicate code
- [ ] Complex type signatures
- [ ] Excessive trait bounds

### Recommendations
- [ ] Prioritized list of improvements
- [ ] Estimated effort for each item
- [ ] Impact assessment
```

## 🔧 VERIFICATION CHECKLIST

### Tool Setup
- [ ] All analysis tools installed
- [ ] Tool versions verified
- [ ] Analysis scripts created

### Dependency Analysis
- [ ] Full dependency tree generated
- [ ] Per-crate dependency trees created
- [ ] Heavy dependencies identified
- [ ] Unused dependencies found
- [ ] Dependency analysis report created

### Macro Analysis
- [ ] All macros cataloged
- [ ] Macro complexity analyzed
- [ ] Usage frequency documented
- [ ] Macro catalog created

### Performance Analysis
- [ ] Clean build time measured
- [ ] Incremental build time measured
- [ ] Per-crate build times recorded
- [ ] Binary size analyzed
- [ ] Performance report created

### Code Quality Analysis
- [ ] Clippy analysis completed
- [ ] Rustfmt check completed
- [ ] Code metrics generated
- [ ] Code quality report created

### Technical Debt Identification
- [ ] TODO comments cataloged
- [ ] FIXME comments cataloged
- [ ] HACK comments cataloged
- [ ] Code smells identified
- [ ] Technical debt report created

## 📦 DELIVERABLES

1. **Analysis Tools Setup**
   - All required tools installed and verified
   - Analysis scripts created

2. **Dependency Analysis Report**
   - `docs/scratch/dependency_analysis.md`
   - `docs/scratch/dependency_tree_full.txt`
   - `docs/scratch/dependency_tree_per_crate.txt`
   - `docs/scratch/dependency_sizes.txt`
   - `docs/scratch/unused_dependencies.txt`

3. **Macro Catalog**
   - `docs/scratch/macro_catalog.md`
   - `docs/scratch/macro_definitions.txt`
   - `docs/scratch/macro_usage.txt`

4. **Performance Analysis Report**
   - `docs/scratch/performance_analysis.md`
   - `docs/scratch/build_time_clean.txt`
   - `docs/scratch/build_time_incremental.txt`
   - `docs/scratch/build_time_per_crate.txt`
   - `docs/scratch/binary_size_analysis.txt`
   - `docs/scratch/top_functions.txt`

5. **Code Quality Report**
   - `docs/scratch/code_quality_analysis.md`
   - `docs/scratch/clippy_warnings.txt`
   - `docs/scratch/clippy_pedantic.txt`
   - `docs/scratch/rustfmt_check.txt`
   - `docs/scratch/code_metrics.json`

6. **Technical Debt Report**
   - `docs/scratch/technical_debt.md`
   - `docs/scratch/todo_comments.txt`
   - `docs/scratch/fixme_comments.txt`
   - `docs/scratch/hack_comments.txt`

## 🎯 SUCCESS CRITERIA

### Comprehensive Analysis
- ✅ All major codebase aspects analyzed
- ✅ Dependency graph complete and documented
- ✅ Macro usage fully cataloged
- ✅ Performance metrics collected
- ✅ Code quality issues identified
- ✅ Technical debt documented

### Actionable Reports
- ✅ Clear recommendations for improvement
- ✅ Prioritized list of issues
- ✅ Estimated effort for fixes
- ✅ Impact assessment provided

### Quality Standards
- ✅ Analysis tools properly configured
- ✅ Reports well-structured and readable
- ✅ Findings backed by data
- ✅ Recommendations practical and actionable

## ⚠️ RISKS & MITIGATIONS

| Risk | Mitigation |
|---|---|
| Analysis tools fail | Use alternative tools, manual analysis |
| Incomplete data collection | Cross-verify with multiple tools |
| Performance impact during analysis | Run during low-usage periods |
| Data overload | Focus on key metrics first |
| Tool compatibility issues | Use stable tool versions |

## 📚 REFERENCE DOCUMENTS

- `docs/forge-tavern/MINING_GOVERNANCE.md` - Governance rules
- `docs/mining-projects/shafts/shaft-y/Overview.md` - Shaft Y overview
- `Cargo.toml` - Current workspace configuration
- `installer-core/Cargo.toml` - Core dependencies
- `installer-cli/Cargo.toml` - CLI dependencies

"*Know thy code before thou refactor it.*" — Bard 🍺⚒️