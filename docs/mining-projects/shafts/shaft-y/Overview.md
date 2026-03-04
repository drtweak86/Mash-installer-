# Shaft Y: Repository Restructuring & Code Quality

**Shaft Title**: Repository Restructuring & Code Quality
**Status**: ⏳ PLANNING PHASE
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-03
**Risk**: HIGH (major restructuring)
**Reward**: HIGH (improved maintainability, performance, modularity)

## 🎯 SCOPE

This shaft focuses on comprehensive repository restructuring and code quality improvements to address technical debt and enhance maintainability. The overhaul includes:

1. **Codebase Analysis**: Scan and earmark areas for improvement
2. **Workspace Splitting**: Restructure monolithic repo into logical workspaces
3. **Dependency Reduction**: Minimize heavy dependencies
4. **Macro Optimization**: Avoid overuse of macros and generics
5. **Macro Isolation**: Move rarely changed macros into separate crates

## 📁 FILES TO BE CREATED OR TOUCHED

### New Files
- `docs/mining-projects/shafts/shaft-y/EX_Y01_Codebase_Analysis.md` - Analysis methodology
- `docs/mining-projects/shafts/shaft-y/EX_Y02_Workspace_Splitting.md` - Workspace design
- `docs/mining-projects/shafts/shaft-y/EX_Y03_Dependency_Reduction.md` - Dependency analysis
- `docs/mining-projects/shafts/shaft-y/EX_Y04_Macro_Optimization.md` - Macro refactoring
- `docs/mining-projects/shafts/shaft-y/EX_Y05_Macro_Isolation.md` - Macro crate creation
- `workspace-hack/Cargo.toml` - Workspace configuration
- `macros/Cargo.toml` - New macros crate
- `macros/src/lib.rs` - Isolated macros

### Modified Files
- `Cargo.toml` - Workspace root configuration
- `installer-core/Cargo.toml` - Dependency cleanup
- `installer-cli/Cargo.toml` - Dependency cleanup
- Various source files with macro usage

## ⚒️ METHODOLOGY

### Technical Strategy
1. **Analysis First**: Comprehensive codebase scan before changes
2. **Incremental Refactoring**: Small, testable changes
3. **Workspace Design**: Logical separation of concerns
4. **Dependency Audit**: Identify and eliminate unnecessary dependencies
5. **Macro Review**: Evaluate each macro for necessity and complexity

### Workspace Structure
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

### Dependency Reduction Strategy
1. Audit all dependencies with `cargo tree`
2. Identify heavy dependencies (large compile times, many features)
3. Evaluate alternatives or elimination
4. Test reduced dependency set thoroughly

### Macro Optimization Strategy
1. Catalog all macros in codebase
2. Evaluate each for:
   - Frequency of change
   - Complexity
   - Necessity
3. Move stable macros to separate crate
4. Simplify or eliminate complex macros

## 📦 DELIVERABLES

### Phase 1: Codebase Analysis ✅ PLANNED
- [ ] Comprehensive code scan with analysis tools
- [ ] Dependency graph visualization
- [ ] Macro usage catalog
- [ ] Performance profiling
- [ ] Technical debt identification

### Phase 2: Workspace Splitting ✅ PLANNED
- [ ] Design logical workspace structure
- [ ] Create workspace-hack crate
- [ ] Configure root Cargo.toml
- [ ] Test workspace build
- [ ] Verify cross-crate dependencies

### Phase 3: Dependency Reduction ✅ PLANNED
- [ ] Audit all dependencies
- [ ] Identify heavy dependencies
- [ ] Evaluate alternatives
- [ ] Implement reductions
- [ ] Test reduced dependency set

### Phase 4: Macro Optimization ✅ PLANNED
- [ ] Catalog all macros
- [ ] Evaluate complexity
- [ ] Simplify where possible
- [ ] Document macro usage
- [ ] Create style guide

### Phase 5: Macro Isolation ✅ PLANNED
- [ ] Create macros crate
- [ ] Move stable macros
- [ ] Update imports
- [ ] Test macro functionality
- [ ] Document macro crate

### Phase 6: Testing & Verification ✅ PLANNED
- [ ] Unit tests for new structure
- [ ] Integration tests
- [ ] Performance benchmarks
- [ ] Build time comparison
- [ ] Documentation updates

## 🔧 VERIFICATION CHECKLIST

### Analysis Verification
- [ ] Codebase scan complete
- [ ] Dependency graph generated
- [ ] Macro catalog complete
- [ ] Performance profile created
- [ ] Technical debt documented

### Workspace Verification
- [ ] Workspace structure designed
- [ ] workspace-hack crate functional
- [ ] Root configuration correct
- [ ] Cross-crate dependencies working
- [ ] Build successful

### Dependency Verification
- [ ] Dependency audit complete
- [ ] Heavy dependencies identified
- [ ] Alternatives evaluated
- [ ] Reductions implemented
- [ ] Tests passing

### Macro Verification
- [ ] Macro catalog complete
- [ ] Complexity evaluation done
- [ ] Simplifications implemented
- [ ] Style guide created
- [ ] Documentation updated

### Isolation Verification
- [ ] Macros crate created
- [ ] Stable macros moved
- [ ] Imports updated
- [ ] Functionality tested
- [ ] Documentation complete

## 📝 PHASE BREAKDOWN

### EX_Y01_Codebase_Analysis.md
1. Set up analysis tools
2. Run comprehensive code scan
3. Generate dependency graph
4. Catalog macro usage
5. Profile performance
6. Identify technical debt

### EX_Y02_Workspace_Splitting.md
1. Design workspace structure
2. Create workspace-hack
3. Configure root Cargo.toml
4. Test workspace build
5. Verify dependencies

### EX_Y03_Dependency_Reduction.md
1. Audit dependencies
2. Identify heavy dependencies
3. Evaluate alternatives
4. Implement reductions
5. Test changes

### EX_Y04_Macro_Optimization.md
1. Catalog macros
2. Evaluate complexity
3. Simplify macros
4. Create style guide
5. Document usage

### EX_Y05_Macro_Isolation.md
1. Create macros crate
2. Move stable macros
3. Update imports
4. Test functionality
5. Document crate

### EX_Y06_Testing_Verification.md
1. Write unit tests
2. Create integration tests
3. Benchmark performance
4. Compare build times
5. Update documentation

## 🎯 SUCCESS CRITERIA (GREEN BUILD)

### Code Quality
- ✅ Cleaner codebase structure
- ✅ Reduced technical debt
- ✅ Improved maintainability
- ✅ Better documentation
- ✅ Consistent style

### Workspace Structure
- ✅ Logical separation of concerns
- ✅ Working cross-crate dependencies
- ✅ Efficient build system
- ✅ Clear module boundaries
- ✅ Easy navigation

### Dependency Management
- ✅ Reduced heavy dependencies
- ✅ Faster compile times
- ✅ Smaller binary sizes
- ✅ Better dependency hygiene
- ✅ Clear dependency rationale

### Macro Usage
- ✅ Simplified complex macros
- ✅ Isolated stable macros
- ✅ Better macro documentation
- ✅ Consistent macro style
- ✅ Reduced macro overuse

### Performance
- ✅ Faster build times
- ✅ Smaller memory footprint
- ✅ Better runtime performance
- ✅ Efficient resource usage
- ✅ Optimized code paths

## 📋 DEPENDENCIES

### Internal Dependencies
- Existing codebase structure
- Current build system
- Test infrastructure

### External Dependencies
- Analysis tools (cargo-tree, cargo-bloat, etc.)
- Workspace management tools
- Documentation tools

## ⚠️ RISKS & MITIGATIONS

| Risk | Mitigation |
|---|---|
| Build failures during restructuring | Incremental changes, thorough testing |
| Dependency conflicts | Careful dependency analysis, testing |
| Macro functionality issues | Comprehensive testing, gradual migration |
| Performance regressions | Benchmarking, profiling |
| Workspace complexity | Clear documentation, logical structure |

## 📚 REFERENCE DOCUMENTS

- `docs/forge-tavern/MINING_GOVERNANCE.md` - Governance rules
- `docs/forge-tavern/maps.md` - Current shaft status
- `Cargo.toml` - Current workspace configuration
- `installer-core/Cargo.toml` - Core dependencies
- `installer-cli/Cargo.toml` - CLI dependencies

"*A well-structured forge is the foundation of great craftsmanship.*" — Bard 🍺⚒️