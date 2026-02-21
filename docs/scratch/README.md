# 🗂️ Scratch Folder - Document Archive

This folder contains archived and historical documents that are no longer actively maintained but may contain useful reference material.

## 📚 Document Categories

### 📜 Historical Documents
- **HISTORY.md** - The Bard's personal journal with chronological entries
- **bard-bbs-profile.md** - Bard's original BBS profile and guidelines
- **bard-quick-ref.md** - Quick reference guide for Bard's workflow

### 🗺️ Mining Project Archives
- **maps-explored.md** - Historical ledger of completed shafts and sessions
- **maps-2024-02-21-backup.md** - Backup snapshot from February 2024
- **maps-2024-02-21-snapshot.md** - Snapshot from February 2024
- **shaftb-phase5-testing.md** - Phase 5 testing documentation

### 🏗️ Legacy QA and Planning
- **QA/** - Quality assurance findings and reports
  - Block-1-findings.md through Block-4-findings.md
  - PlanA.md, PlanB.md, PlanC.md
  - QAREPORT.md - Comprehensive QA report

- **wo/** - Work orders and enhancement proposals
  - WO-001 through WO-019 - Detailed work order specifications

### 🐍 Legacy Code
- **wallpaper_downloader-1.py** - Earlier version of wallpaper downloader

## 🔍 Searching for Specific Information

Use `grep` to search across all archived documents:

```bash
# Search for specific content
grep -r "search term" docs/scratch/

# Search for work orders
grep -r "WO-" docs/scratch/

# Search for QA findings
grep -r "Block-" docs/scratch/
```

## 📅 Version History

This archive was created during document hygiene on 2026-02-21 to organize the documentation structure and remove duplicates.

## 📋 Active Documentation

For current, active documentation, see:
- `docs/maps.md` - Current mining projects and active shafts
- `docs/mining-projects/shafta.md` - Shaft A reconnaissance report
- `docs/mining-projects/shaftb.md` - Shaft B integration plan
- `docs/legacy/` - Legacy documentation still in use
- `docs/incoming-files/` - New and proposed features
