# ⚒️ Shaft AA: The Ascended Architecture
> *"The foundation is solid. Now we build the spire."* — Bard 🍺

## 🎯 OBJECTIVE
Implement the four "Percolated Ideas" to evolve MASH from a robust installer into an intelligent, lightweight, and adaptive provisioning agent.

## 📋 SCOPE
1.  **The Grimoire (Catalog)**: Transform the software catalog into a compile-time, structurally typed graph with tier inheritance and strict driver enforcement.
2.  **Bard's Heuristics**: Evolve "Bard Recommends" from a static list into a dynamic decision engine based on hardware (`CpuInfo`, `Ram`) and environment.
3.  **Zero-HTTP (Lightweighting)**: Replace the heavy `ureq` + `rustls` stack with a `mash-curl` wrapper, leveraging the system's bootstrap tools to slash binary size.
4.  **Roaming Profiles**: Implement `mash-setup --url <gist>` to allow users to apply remote configurations instantly.

## 🛠️ METHODOLOGY
*   **Compile-Time Safety**: Use Rust's type system to enforce that every S-Tier package has a mapping for every supported distro.
*   **System Reuse**: Don't ship what the OS provides. Use `curl`, `proc`, and standard paths.
*   **Dynamic Logic**: Move from declarative lists to functional predicates for software selection.

## 📦 DELIVERABLES
*   **Binary Size**: Target < 3.0 MiB (via Zero-HTTP).
*   **New Capabilities**:
    *   `--url` argument support.
    *   Context-aware defaults (Pi 4B vs. Threadripper).
*   **Refactored Core**: `installer-core/src/catalog` rewritten as a graph.

---

## 🔧 VERIFICATION
*   **Size Check**: Binary size must decrease significantly.
*   **Integration Tests**: Mock `curl` calls to verify "Zero-HTTP" logic.
*   **Catalog Audit**: Compiler ensures completeness of the software map.

---
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Status**: ⚒️ EXCAVATION INITIATED
**Last Updated**: 2026-03-04
