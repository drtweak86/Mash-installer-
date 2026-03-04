# EX_AA03: Zero-HTTP

**Objective**: Reduce binary size by removing the heavy HTTP stack (`ureq`, `rustls`, `webpki`) and leveraging the system's `curl`.

## 🛠️ Technical Implementation
1.  **The Wrapper**:
    *   Create `mash-system/src/curl.rs`.
    *   Implement `fn download(url: &str, dest: &Path) -> Result<()>` wrapping `std::process::Command::new("curl")`.
    *   Use `-sS` (silent, show error) and `-L` (follow redirects).
2.  **Migration**:
    *   Update `mash-wallpaper` to use `mash_system::curl` instead of `ureq`.
    *   Remove `ureq` and `rustls` dependencies from `installer-core` and `mash-wallpaper`.
3.  **Fallback Safety**:
    *   Ensure `install.sh` verifies `curl` presence (it already does).
    *   Add a runtime check in `main.rs` to panic early if `curl` is missing (unlikely, but safe).

## 🧪 Verification
*   Binary size drops < 3.0 MiB.
*   Wallpaper download works on a system with standard `curl`.
