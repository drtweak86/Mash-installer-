# EX_AA01: The Grimoire Architecture

**Objective**: Transform the software catalog from a loose collection of strings into a **structurally typed graph** with tier inheritance and strict driver enforcement.

## 🛠️ Technical Implementation
1.  **Type-Safe Categories**:
    *   Replace string-based categories with an `enum SoftwareCategory { Editor, Terminal, Shell, ... }`.
    *   Enforce exhaustive matching in the UI.
2.  **Tier Inheritance**:
    *   Implement `Tier::resolve(&self) -> Vec<Tier>` logic.
    *   Selecting `Tier::A` should automatically include `Tier::S`.
3.  **Driver Enforcement**:
    *   Create a `PackageMap` struct that requires entries for `arch`, `debian`, and `fedora`.
    *   Use a compile-time check (or strict unit test) to ensure every `Tier::S` package has a valid mapping for all 3 supported drivers.
4.  **Data Structure**:
    *   Refactor `installer-core/src/catalog` to use `phf` (Perfect Hash Function) or `std::sync::LazyLock` for static catalog definitions, removing runtime parsing overhead.

## 🧪 Verification
*   Compiler fails if a new category is added without UI handling.
*   Unit tests verify that selecting "Tier A" yields the superset of S+A.
