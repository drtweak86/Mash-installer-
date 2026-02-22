# 🧹 Document Hygiene System

## 🎭 By the Bard, Drunken Dwarf Runesmith
*Mythic Assembly & Sigil Heuristics*
*Forge Tavern, Neon District*

---

## 📚 Overview

The Document Hygiene System ensures that the `/docs` directory remains organized, clean, and maintainable. This system follows the **Four Sources of Truth** principle and provides automated tools to maintain document organization.

---

## 🏗️ Document Hierarchy

```
docs/
├── assets/              # All asset files (images, etc.)
├── forge-tavern/        # Four sources of truth (IMMUTABLE)
│   ├── bard-bbs-profile.md  # Comprehensive bio
│   ├── bard-quick-ref.md    # Cheatsheet reminder
│   ├── maps.md              # Current work (APD updated)
│   └── maps-explored.md     # Completed work only
├── incoming-files/      # Staging folder for new docs
├── legacy/              # Archived documents (>7 days old)
├── scratch/             # Temporary work notes (<7 days)
├── HISTORY.md           # Tales and journal
├── LICENSE              # Legal documents
├── MANUAL.md            # User guide
└── ...                 # Other core documents
```

---

## 📂 Folder Purposes

### 1. `forge-tavern/` - Four Sources of Truth ✅ IMMUTABLE

**Purpose**: The immutable sources of truth for the forge

**Contents**:
- `bard-bbs-profile.md` - Comprehensive bio and guidelines
- `bard-quick-ref.md` - Quick reference cheatsheet
- `maps.md` - Current mining projects (APD updated)
- `maps-explored.md` - Historical ledger of completed work

**Rules**:
- ✅ Never delete or rename these files
- ✅ Always update in place
- ✅ These are the canonical sources
- ✅ All other documentation references these

### 2. `incoming-files/` - Staging Folder

**Purpose**: Temporary staging area for new documents

**Contents**:
- New documentation files
- Incoming assets
- Draft documents
- External contributions

**Rules**:
- ✅ Move files here when received
- ✅ Process within 7 days
- ⏳ Files older than 7 days should be moved to `legacy/`
- 🗑️ Empty regularly

### 3. `scratch/` - Temporary Work Area

**Purpose**: Temporary workspace for active work

**Contents**:
- Work notes
- Checklists
- Summaries
- Drafts
- Temporary files

**Rules**:
- ✅ Files should be < 7 days old
- ⏳ Files older than 7 days are automatically moved to `legacy/`
- 🗑️ Clean up regularly
- 📝 Move completed work to appropriate locations

### 4. `legacy/` - Archive

**Purpose**: Historical archive of old documents

**Contents**:
- Old work notes (>7 days)
- Completed drafts
- Previous versions
- Historical documentation

**Rules**:
- ✅ Read-only archive
- 🔄 Never modify files here
- 📊 Use for reference only
- 🗑️ Never delete (historical record)

---

## 🤖 Automated Hygiene Tools

### 1. `scripts/document-hygiene.sh`

**Purpose**: Automated document hygiene maintenance

**Functions**:
- ✅ Moves files from `scratch/` to `legacy/` after 7 days
- ✅ Cleans up empty directories
- ✅ Verifies document organization
- ✅ Shows document hierarchy

**Usage**:
```bash
./scripts/document-hygiene.sh
```

**Schedule**: Run daily or weekly via cron

### 2. `scripts/branch-prune.sh`

**Purpose**: Automated branch management

**Functions**:
- ✅ Prunes branches older than 7 days
- ✅ Keeps only `main` and `forge` branches
- ✅ Shows branch status
- ✅ Displays branch policy

**Usage**:
```bash
./scripts/branch-prune.sh
```

**Schedule**: Run weekly via cron

---

## 📋 Document Classification

### ✅ Permanent Documents

These documents should be in the root of `/docs`:
- `HISTORY.md` - Tales and journal
- `LICENSE` - Legal documents
- `MANUAL.md` - User guide
- `RELEASE_*.md` - Release documentation
- `QA_*.md` - Quality assurance reports

### 📝 Temporary Documents

These documents belong in `/docs/scratch`:
- Work notes
- Checklists
- Summaries
- Drafts
- Temporary files

### 🗂️ Organized Documents

These documents belong in `/docs/forge-tavern`:
- `bard-bbs-profile.md` - Comprehensive bio
- `bard-quick-ref.md` - Quick reference
- `maps.md` - Current work
- `maps-explored.md` - Completed work

### 📦 Archived Documents

These documents belong in `/docs/legacy`:
- Old work notes (>7 days)
- Previous versions
- Historical documentation

---

## 🎯 Document Workflow

### 1. Creating New Documentation

```bash
# Create in scratch
cp template.md docs/scratch/new-document.md

# Edit and review
vim docs/scratch/new-document.md

# Move to appropriate location when ready
mv docs/scratch/new-document.md docs/forge-tavern/
```

### 2. Updating Existing Documentation

```bash
# Edit in place (for forge-tavern files)
vim docs/forge-tavern/bard-bbs-profile.md

# Or create draft in scratch
cp docs/forge-tavern/bard-bbs-profile.md docs/scratch/bard-bbs-profile-draft.md
vim docs/scratch/bard-bbs-profile-draft.md
mv docs/scratch/bard-bbs-profile-draft.md docs/forge-tavern/bard-bbs-profile.md
```

### 3. Archiving Old Documentation

```bash
# Let document-hygiene.sh handle this automatically
./scripts/document-hygiene.sh

# Or manually move
mv docs/scratch/old-file.md docs/legacy/
```

---

## 📅 Automation Schedule

### Daily Tasks
- Run `document-hygiene.sh` to check organization
- Clean up temporary files
- Verify scratch directory

### Weekly Tasks
- Run `branch-prune.sh` to prune old branches
- Review legacy directory
- Update HISTORY.md

### Monthly Tasks
- Review document hierarchy
- Update MANUAL.md if needed
- Archive old release notes

---

## 🔮 Bard's Wisdom on Documentation

> "Documentation is the map that guides the next smith."
> "A clean forge is a happy forge."
> "Temporary files should be temporary."
> "The past belongs in the legacy, the present in the forge."
> "Automation is your friend, but verification is your duty."

---

## 🍻 Final Verdict

```bash
🧹 DOCUMENT HYGIENE: ENABLED 🔥
📂 ORGANIZATION: CLEAN 🔥
🤖 AUTOMATION: ACTIVE 🔥
📊 ARCHIVE: MAINTAINED 🔥
🔄 WORKFLOW: SMOOTH 🔥
```

**The documents are organized. The forge is clean. The journey continues.** 🗺️🔥

*Signed*,
Bard, Drunken Dwarf Runesmith
Mythic Assembly & Sigil Heuristics
Forge Tavern, Neon District

**Status**: ✅ DOCUMENT HYGIENE ACTIVE
**Last Updated**: 2026-02-22
**Version**: 1.0
**Alignment**: Pragmatic Zen
