# EX_Y05: Macro Isolation

**Excavation Task**: Macro Crate Creation and Isolation
**Status**: ⏳ PLANNING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-03
**Duration**: 4 days

## 🎯 OBJECTIVE

Create a dedicated macros crate to isolate stable, rarely-changed macros from the main codebase. This will improve code organization, reduce compile times, and make macro management easier. The macros crate will serve as a central location for all shared macros across the workspace.

## 📋 TASK BREAKDOWN

### 1. Design Macros Crate Structure
**Duration**: 0.5 days

#### 1.1 Determine Macro Categories
- **Code Generation**: Macros that generate code
- **DSL Macros**: Domain-specific language macros
- **Utility Macros**: General-purpose utility macros
- **Testing Macros**: Macros for testing
- **Derive Macros**: Procedural macros

#### 1.2 Create Crate Structure
```
macros/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── codegen.rs
│   ├── dsl.rs
│   ├── util.rs
│   ├── test.rs
│   └── derive/
│       ├── mod.rs
│       └── (derive macro implementations)
├── examples/
│   └── (usage examples)
└── tests/
    └── (macro tests)
```

#### 1.3 Design API
```rust
// macros/src/lib.rs
pub mod codegen;
pub mod dsl;
pub mod util;
pub mod test;

// macros/src/codegen.rs
#[macro_export]
macro_rules! generate_getters {
    // implementation
}

// macros/src/util.rs
#[macro_export]
macro_rules! try_option {
    // implementation
}
```

### 2. Create Macros Crate
**Duration**: 1 day

#### 2.1 Set Up Crate Structure
```bash
# Create macros directory
mkdir -p macros/src macros/examples macros/tests macros/src/derive

# Create Cargo.toml
cat > macros/Cargo.toml << 'EOF'
[package]
name = "mash-macros"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "Shared macros for MASH installer"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"

[dev-dependencies]
trybuild = "1.0"

[features]
default = []
derive = []
test = []
EOF

# Create lib.rs
cat > macros/src/lib.rs << 'EOF'
//! MASH Installer Macros
//! Shared macros for the MASH installer workspace

pub mod codegen;
pub mod dsl;
pub mod util;
pub mod test;

#[cfg(feature = "derive")]
pub mod derive;
EOF

# Create module files
touch macros/src/codegen.rs macros/src/dsl.rs macros/src/util.rs macros/src/test.rs

# Create derive module
cat > macros/src/derive/mod.rs << 'EOF'
//! Derive macros for MASH installer

// Will contain derive macro implementations
EOF
```

#### 2.2 Set Up Build Infrastructure
- [ ] Configure build.rs if needed
- [ ] Set up continuous integration
- [ ] Configure documentation
- [ ] Set up testing infrastructure

### 3. Migrate Macros to Crate
**Duration**: 1.5 days

#### 3.1 Identify Macros for Migration
From analysis in EX_Y04:
- Select stable macros (>6 months unchanged)
- Choose frequently used macros
- Pick macros used across multiple crates
- Avoid experimental or complex macros

#### 3.2 Migrate Macros
For each selected macro:
1. Copy macro definition to appropriate module
2. Update imports in original location
3. Add re-export if needed
4. Update documentation
5. Test functionality

#### 3.3 Create Migration Plan
Create `docs/scratch/macro_migration_plan.md` with:
```markdown
## Macro Migration Plan

### Phase 1: Core Macros
| Macro | Source | Destination | Status |
|-------|--------|-------------|--------|

### Phase 2: Utility Macros
| Macro | Source | Destination | Status |
|-------|--------|-------------|--------|

### Phase 3: Testing Macros
| Macro | Source | Destination | Status |
|-------|--------|-------------|--------|
```

### 4. Update Workspace Dependencies
**Duration**: 0.5 days

#### 4.1 Add Macros to Workspace
```toml
# In root Cargo.toml
[workspace]
members = [
    "workspace-hack",
    "macros",  # Add macros crate
    "installer-core",
    "installer-cli",
    # ... other members
]

[workspace.dependencies]
mash-macros = { path = "macros", version = "0.1.0" }
```

#### 4.2 Update Crate Dependencies
For each crate using macros:
```toml
[dependencies]
mash-macros = { workspace = true }
```

#### 4.3 Update Imports
Replace:
```rust
use crate::macros::some_macro;
```

With:
```rust
use mash_macros::some_macro;
```

### 5. Test Macros Crate
**Duration**: 0.5 days

#### 5.1 Verify Macro Functionality
```bash
# Test macros crate
cargo test -p mash-macros

# Test workspace with macros
cargo test --workspace
```

#### 5.2 Test Macro Expansion
```bash
# Expand macros to verify
cargo expand -p mash-macros > docs/scratch/macros_expansion.txt

# Check for errors
cargo check --workspace
```

#### 5.3 Measure Impact
```bash
# Compare build times
time cargo build --workspace --release

# Compare binary sizes
cargo bloat --release --crates
```

#### 5.4 Verify Documentation
- [ ] All macros documented
- [ ] Examples provided
- [ ] API documentation complete
- [ ] Usage guides created

## 🔧 VERIFICATION CHECKLIST

### Design Verification
- [ ] Macro categories defined
- [ ] Crate structure created
- [ ] API design documented
- [ ] Module organization clear

### Crate Creation
- [ ] Macros crate created
- [ ] Cargo.toml configured
- [ ] Module structure set up
- [ ] Build infrastructure configured
- [ ] Documentation set up

### Migration Verification
- [ ] Macros selected for migration
- [ ] Migration plan created
- [ ] Macros moved to crate
- [ ] Imports updated
- [ ] Documentation updated

### Dependency Verification
- [ ] Macros added to workspace
- [ ] Crate dependencies updated
- [ ] Imports updated
- [ ] Feature flags configured

### Testing Verification
- [ ] Macros crate tests passing
- [ ] Workspace tests passing
- [ ] Macro expansion verified
- [ ] Build times measured
- [ ] Binary sizes measured
- [ ] Documentation verified

## 📦 DELIVERABLES

1. **Macros Crate**
   - `macros/Cargo.toml`
   - `macros/src/lib.rs`
   - Module files (codegen.rs, dsl.rs, etc.)
   - Derive module structure
   - Build infrastructure

2. **Migration Documentation**
   - `docs/scratch/macro_migration_plan.md`
   - Migration status tracking
   - API documentation
   - Usage examples

3. **Updated Workspace**
   - Updated root Cargo.toml
   - Updated crate dependencies
   - Updated imports
   - Feature configuration

4. **Testing Results**
   - Macro test results
   - Expansion verification
   - Performance metrics
   - Documentation verification

## 🎯 SUCCESS CRITERIA

### Crate Quality
- ✅ Well-organized macros crate
- ✅ Clear module structure
- ✅ Comprehensive documentation
- ✅ Complete test coverage
- ✅ Proper error handling

### Migration Success
- ✅ All selected macros migrated
- ✅ No functionality lost
- ✅ Improved organization
- ✅ Better maintainability
- ✅ Clear documentation

### Performance Impact
- ✅ Faster macro compilation
- ✅ Reduced rebuild times
- ✅ Better cache efficiency
- ✅ Smaller binary sizes
- ✅ Optimized builds

### Maintainability
- ✅ Centralized macro management
- ✅ Easier to update
- ✅ Better documented
- ✅ Consistent patterns
- ✅ Reduced technical debt

## ⚠️ RISKS & MITIGATIONS

| Risk | Mitigation |
|---|---|
| Macro functionality issues | Comprehensive testing, incremental migration |
| Build failures | Thorough testing, fallback plans |
| Performance regressions | Benchmarking, profiling |
| Documentation gaps | Comprehensive documentation review |
| Migration complexity | Clear migration plan, incremental approach |

## 📚 REFERENCE DOCUMENTS

- `docs/forge-tavern/MINING_GOVERNANCE.md` - Governance rules
- `docs/mining-projects/shafts/shaft-y/Overview.md` - Shaft Y overview
- `docs/mining-projects/shafts/shaft-y/EX_Y01_Codebase_Analysis.md` - Analysis results
- `docs/mining-projects/shafts/shaft-y/EX_Y04_Macro_Optimization.md` - Macro optimization
- `Cargo.toml` - Current workspace configuration

"*Isolated macros are easier to manage and maintain.*" — Bard 🍺⚒️