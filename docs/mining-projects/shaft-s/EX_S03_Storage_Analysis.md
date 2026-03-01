# EX_S03: Storage & Filesystem Audit
> *"Mapping the machine's hoard and the foundations of the land."* — Bard 🍺

## 🎯 OBJECTIVE
Implement detection for block devices, partitions, mountpoints, and Btrfs-specific subvolume data.

## 📋 DETAILED STEPS

### 1. Identify Block Devices & Mountpoints
- [ ] Use `lsblk --json -o NAME,SIZE,TYPE,MOUNTPOINT,FSTYPE,UUID,RO` to list all devices.
- [ ] Identify boot, root, and home partitions.
- [ ] Record free space for key mounts: `/`, `/home`, `/data`, `/boot`.

### 2. Audit Filesystems
- [ ] Distinguish between Btrfs, ext4, vfat, and other common filesystems.
- [ ] Identify if a filesystem is multi-device (e.g., Btrfs RAID).

### 3. Dig Deep into Btrfs
- [ ] If Btrfs is detected:
  - Scry for subvolumes (`@`, `@home`, `@data`, `@borg`, `.snapshots`).
  - Check for active compression (`zstd`, `lzo`) in `/proc/mounts` or `/proc/self/mountinfo`.
  - Extract the UUID of the root filesystem.

### 4. Wire the Profiler
- [ ] Ensure `StorageInfo` correctly aggregates this data from `lsblk`, `findmnt`, and Btrfs tools.
- [ ] Use `SystemOps` trait if filesystem calls are needed (to maintain dry-run safety).

## ✅ VERIFICATION
- [ ] `cargo test -p installer-core profile` correctly parses `lsblk` JSON.
- [ ] Btrfs subvolumes are identified correctly on a Btrfs system.
- [ ] Mountpoints and free space are accurate.
