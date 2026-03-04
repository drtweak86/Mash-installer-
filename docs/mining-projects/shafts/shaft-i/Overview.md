# Shaft I: Software Catalog & Installation Flow Overhaul

**Shaft Title**: Software Catalog & Installation Flow Overhaul
**Status**: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-02-26
**Risk**: MEDIUM (catalog restructuring, UI changes)
**Reward**: HIGH (significantly improved user experience, logical organization)

## 🎯 SCOPE

This shaft focuses on completely reorganizing the MASH software catalog and installation flow to create a more logical, user-friendly, and efficient system. The overhaul includes:

1. **Curated Software Catalog**: S-tier applications in every category with reasoning
2. **Enhanced Installation Modes**: Manual, Auto, and Bard's Recommendations
3. **Logical Menu Structure**: Category/Subcategory organization
4. **Optimized Installation Flow**: Prerequisites before dependencies
5. **Special Handling**: Ensure Brave Browser inclusion and proper ordering

## 📁 FILES TO BE CREATED OR TOUCHED

### New Files
- `installer-core/src/software_catalog.rs` - Comprehensive software catalog
- `installer-core/src/installation_modes.rs` - Manual/Auto/Bard's modes
- `installer-core/src/category_organization.rs` - Menu structure logic
- `installer-core/src/optimization_flow.rs` - Prerequisite ordering
- `resources/catalog/s-tier_catalog.toml` - Curated software list
- `resources/catalog/full_catalog.toml` - Complete software database
- `installer-cli/src/tui/software_menus.rs` - Enhanced software selection UI
- `installer-cli/src/tui/installation_flow.rs` - Reworked installation screens

### Modified Files
- `installer-core/src/lib.rs` - Export new modules
- `installer-cli/src/tui/menus.rs` - Updated menu structure
- `installer-cli/src/tui/app.rs` - New installation flow
- `installer-core/Cargo.toml` - New dependencies if needed
- `installer-cli/Cargo.toml` - UI dependencies

### Reference Files
- `docs/incoming-files/software_tiers.md` - Existing tier definitions
- `installer-cli/src/software_catalog.rs` - Current catalog structure

## ⚒️ METHODOLOGY

### Technical Strategy
1. **Catalog-First Approach**: Define comprehensive software catalog before UI changes
2. **Modular Design**: Separate catalog, installation logic, and UI components
3. **Configuration-Driven**: Use TOML for easy catalog maintenance
4. **Dependency-Aware**: Build dependency graph for optimal installation order
5. **User-Centric**: Design for both power users and beginners

### Data Structures
```rust
// Software Catalog Structure
struct SoftwareCategory {
    name: String,
    display_name: String,
    description: String,
    subcategories: Vec<SoftwareSubcategory>,
    icon: Option<String>,
}

struct SoftwareSubcategory {
    name: String,
    description: String,
    programs: Vec<SoftwareProgram>,
}

struct SoftwareProgram {
    id: String,
    name: String,
    description: String,
    category: String,
    subcategory: String,
    tier: SoftwareTier,  // S, A, B, C
    packages: HashMap<Distro, Vec<String>>,
    dependencies: Vec<String>,
    post_install: Option<String>,
    recommended: bool,
    reasoning: Option<String>,
}

enum SoftwareTier {
    S,  // Top tier
    A,  // Excellent
    B,  // Good
    C,  // Basic
}

enum InstallationMode {
    Manual,      // User selects individual programs
    Auto,        // Install by category
    BardsChoice, // Only S-tier #1 from each category
}
```

### Installation Flow Optimization
```rust
struct InstallationPlan {
    mode: InstallationMode,
    selected_software: Vec<SoftwareProgram>,
    optimized_order: Vec<SoftwareProgram>,
    dependency_graph: HashMap<String, Vec<String>>,
}

impl InstallationPlan {
    fn optimize_order(&mut self) {
        // 1. System optimizations (ccache, sccache)
        // 2. Dependencies (libraries, tools)
        // 3. Languages and runtimes
        // 4. Applications
        // 5. Themes and customizations
    }
}
```

## 📦 DELIVERABLES

### Phase 1: Software Catalog Curation ✅ PLANNED
- [ ] Define category structure (Games, Graphics, Programming, etc.)
- [ ] Curate S-tier applications (5 per category) with reasoning
- [ ] Include all programming languages
- [ ] Ensure Brave Browser in Internet category
- [ ] Create TOML catalog files
- [ ] Add tier definitions and reasoning

### Phase 2: Installation Modes ✅ PLANNED
- [ ] Manual mode: Individual program selection
- [ ] Auto mode: Category-based installation
- [ ] Bard's Recommendations: S-tier #1 from each category
- [ ] Implementation in installation_modes.rs
- [ ] UI integration for mode selection

### Phase 3: Menu Restructuring ✅ PLANNED
- [ ] Category/Subcategory organization
- [ ] Themes: All thematic installation options
- [ ] Development: Languages, tools, IDEs grouped
- [ ] System: Optimizations, utilities, drivers
- [ ] Internet: Browsers, email, messaging
- [ ] Multimedia: Graphics, audio, video

### Phase 4: Installation Flow Optimization ✅ PLANNED
- [ ] Prerequisite detection and installation
- [ ] Dependency ordering (ccache before Rust)
- [ ] Parallel installation where possible
- [ ] Progress tracking by category
- [ ] Error handling and recovery

### Phase 5: UI Integration ✅ PLANNED
- [ ] Enhanced software selection screens
- [ ] Category navigation
- [ ] Installation mode selection
- [ ] Progress visualization
- [ ] Confirmation dialogs

### Phase 6: Testing & Documentation ✅ PLANNED
- [ ] Unit tests for catalog logic
- [ ] Integration tests for installation flow
- [ ] Cross-distro compatibility testing
- [ ] User documentation
- [ ] Update governance documents

## 🔧 VERIFICATION CHECKLIST

### Catalog Verification
- [ ] All categories defined with descriptions
- [ ] 5 S-tier programs per category with reasoning
- [ ] All programming languages included
- [ ] Brave Browser in Internet category
- [ ] TOML files valid and well-formatted
- [ ] Tier definitions consistent

### Installation Mode Verification
- [ ] Manual mode allows individual selection
- [ ] Auto mode installs by category
- [ ] Bard's mode selects only top S-tier
- [ ] Mode selection UI intuitive
- [ ] Mode persistence across sessions

### Menu Structure Verification
- [ ] Logical category/subcategory hierarchy
- [ ] Themes grouped together
- [ ] Development tools organized
- [ ] System optimizations first
- [ ] Navigation intuitive

### Flow Optimization Verification
- [ ] ccache/sccache before Rust
- [ ] Dependencies resolved correctly
- [ ] Parallel installation working
- [ ] Progress tracking accurate
- [ ] Error recovery functional

### UI Verification
- [ ] Software selection screens display correctly
- [ ] Category navigation smooth
- [ ] Installation progress visible
- [ ] Confirmation dialogs appropriate
- [ ] Responsive on different terminal sizes

## 📝 PHASE BREAKDOWN

### EX_I01_Software_Catalog_Curation.md
1. Research and define categories
2. Curate S-tier applications (5 per category)
3. Include all programming languages
4. Create TOML catalog structure
5. Add tier definitions and reasoning
6. Ensure Brave Browser inclusion

### EX_I02_Installation_Modes.md
1. Implement Manual mode logic
2. Implement Auto mode (category-based)
3. Implement Bard's Recommendations mode
4. Create mode selection UI
5. Integrate with installation flow

### EX_I03_Menu_Restructuring.md
1. Design category/subcategory hierarchy
2. Group Themes together
3. Organize Development tools
4. Structure System optimizations
5. Implement navigation logic

### EX_I04_Installation_Flow_Optimization.md
1. Create dependency resolution system
2. Implement prerequisite ordering
3. Add parallel installation logic
4. Create progress tracking
5. Implement error handling

### EX_I05_UI_Integration.md
1. Design software selection screens
2. Implement category navigation
3. Add installation mode selection
4. Create progress visualization
5. Add confirmation dialogs

### EX_I06_Testing_Documentation.md
1. Write unit tests for catalog
2. Create integration tests
3. Test cross-distro compatibility
4. Write user documentation
5. Update governance documents

## 🎯 SUCCESS CRITERIA (GREEN BUILD)

### Code Quality
- ✅ All code passes `cargo fmt`, `cargo clippy`, and tests
- ✅ No performance regressions in catalog operations
- ✅ Clean separation of concerns
- ✅ Comprehensive error handling
- ✅ Well-documented public APIs

### Functionality
- ✅ Software catalog with 50+ S-tier applications
- ✅ Three installation modes working correctly
- ✅ Logical category/subcategory organization
- ✅ Optimized installation flow
- ✅ Brave Browser included and installable

### User Experience
- ✅ Intuitive software selection
- ✅ Clear category navigation
- ✅ Informative installation progress
- ✅ Appropriate confirmations
- ✅ Helpful error messages

### Cross-Distro Compatibility
- ✅ Works on Fedora, Debian, Arch
- ✅ Package mapping for all distros
- ✅ Fallback mechanisms in place
- ✅ Distro-specific recommendations

### Documentation
- ✅ Complete catalog documentation
- ✅ Installation mode guides
- ✅ Menu structure explanation
- ✅ Troubleshooting section
- ✅ Governance compliance

## 📋 DEPENDENCIES

### Internal Dependencies
- Existing software catalog structure
- Current installation system
- Package management infrastructure
- UI framework and components

### External Dependencies
- TOML parsing library
- Graph library for dependency resolution
- Additional UI components if needed

## ⚠️ RISKS & MITIGATIONS

| Risk | Mitigation |
|---|---|
| Catalog becomes unwieldy | Modular design, category organization |
| Installation order issues | Dependency graph, thorough testing |
| Performance with large catalog | Efficient data structures, lazy loading |
| Cross-distro package differences | Comprehensive mapping, fallbacks |
| UI complexity | Clear navigation, user testing |
| Brave Browser compatibility | Multiple installation methods, fallbacks |

## 📚 REFERENCE DOCUMENTS

- `docs/incoming-files/software_tiers.md` - Existing tier definitions
- `installer-cli/src/software_catalog.rs` - Current catalog
- `docs/forge-tavern/MINING_GOVERNANCE.md` - Governance rules
- `docs/forge-tavern/maps.md` - Current shaft status

"*A well-organized catalog is like a well-stocked forge - everything has its place and purpose.*" — Bard 🍺⚒️