# Release Process
> **Neon Chronicle (Technical polish)**: release-process keeps the gears and runes aligned, so the neon hum is restrained to precise, actionable beats. ⚙️


## Purpose
Formalize the steps required to ship a mash-installer release. This process pairs automated helpers with a concise checklist so each release is consistent, well-tested, and well-documented.

## Automated Checklist
1. `scripts/check-docs.py` validates that every relative Markdown link under `docs/` points to an existing file. Run this before pushing documentation updates.
2. `scripts/test-infrastructure.sh maelstrom` executes the maelstrom-compatible suite. In CI it should be the default path that runs before any hardware or integration tests.
3. `scripts/test-infrastructure.sh hardware` captures the subset of tests that require real kernel features or special hardware (e.g., Docker, systemd, kernel modules).
4. `scripts/release-checklist.sh` orchestrates these steps and reminds the release owner about dry-run verification, changelog updates, and documentation deployment. Save the generated `.logs/test-*.log` artifacts for traceability.

## Mandatory Pre-release Steps
- **Documentation Review:** Every updated doc must be linted via `scripts/check-docs.py` and approved in the release candidate. Mention the reviewing parties in the release notes.
- **Dry-run Verification:** Before tagging a release, run `mash-setup doctor --dry-run` or `mash-setup --dry-run` to exercise WO-017's dry-run logging and to ensure no operations would fail.
- **Full Test Suite:** Run the category-aware suites described above. The scripts already encapsulate the `CARGO_HOME=/tmp/cargo RUST_TEST_THREADS=1` conventions from previous work orders.

## Release Roles
- **Release Manager:** Owns the automation scripts, ensures the release checklist passes, and publishes release notes or changelog entries generated from the conventional commits.
- **Documentation Steward:** Confirms new or edited docs are accurate, link-checked, and consistent with code changes. Updates `docs/modules.md`, `REPORT.md`, or `ARCH.md` as needed.
- **QA Lead:** Validates the dry-run report and ensures logs from WO-016/WO-017 are attached to the release artifacts.

## Versioning Guidance
1. When bumping versions, update `installer-core/Cargo.toml` and any downstream `Cargo.lock` entries. Consider using `cargo-release` or a scripted helper to enforce semantic versioning.
2. Generate changelog entries (manually or via conventional commits) that summarize the work captured in this release and reference the relevant work orders (WO-016 through WO-019).
3. After the release passes, tag the commit (`git tag -a vX.Y.Z -m "Release X.Y.Z"`) and push tags alongside the branch.

## Reporting and Deployment
- Collect logs from all automated steps (`.logs/test-*.log`) and attach them to the release ticket or merge request.
- Deploy updated documentation to the configured hosting target (GitHub Pages, docs site, etc.) after the release succeeds.