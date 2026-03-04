# EX_T03: Storage & Filesystem Wisdom
> *"Mapping the land for the perfect foundation."* — Bard 🍺

## 🎯 OBJECTIVE
Implement detection for storage-related optimizations and filesystem-specific features.

## 📋 DETAILED STEPS

### 1. Btrfs Bootstrapping
- [ ] Implement `BtrfsSnapshotRule`:
  - Trigger: Root filesystem is `btrfs`.
  - Action: Recommend `btrfs-assistant` or `snapper` for snapshot management.
- [ ] Implement `BtrfsCompressionRule`:
  - Trigger: `btrfs` without active compression.
  - Action: Suggest enabling `zstd:3` for better life on SD cards/NVMe.

### 2. Workspace Optimization
- [ ] Implement `SmallRootLargeDataRule`:
  - Trigger: `/` < 30GB AND `/data` (or other large partition) > 100GB.
  - Action: Suggest relocating GitHub workspace and Docker data root.
- [ ] Implement `SdCardWriteWarning`:
  - Trigger: Working directory on an SD card.
  - Action: Recommend using a USB SSD or NVMe for project builds.

### 3. Filesystem Health
- [ ] Implement `NoJournalRule`:
  - Trigger: `ext4` on flash storage without `noatime` or `commit` optimizations.
  - Action: Recommend mount options for longevity.

## ✅ VERIFICATION
- [ ] Unit tests for storage-based advice using `StorageInfo` mocks.
- [ ] `AdviceEngine` identifies workspace relocation opportunities.
