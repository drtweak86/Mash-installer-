use anyhow::{bail, Result};
use std::fs;

/// Information about the host we are running on.
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub arch: String,
    pub distro: String,
    pub distro_version: String,
    pub distro_codename: String,
    /// "debian" for Ubuntu/Debian, "arch" for Manjaro/Arch/EndeavourOS, etc.
    pub distro_family: String,
    pub pi_model: Option<String>,
}

impl PlatformInfo {
    pub fn is_arch_family(&self) -> bool {
        self.distro_family == "arch"
    }

    pub fn is_debian_family(&self) -> bool {
        self.distro_family == "debian"
    }
}

/// Detect the current platform.
pub fn detect() -> Result<PlatformInfo> {
    let arch = std::env::consts::ARCH.to_string();

    if arch.starts_with("arm") && arch != "aarch64" {
        bail!(
            "32-bit ARM ({arch}) is not supported; install a 64-bit (aarch64) \
             image before running mash-setup. See docs/QAREPORT.md (Medium 6)."
        );
    }

    // Read /etc/os-release
    let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let distro = parse_os_field(&os_release, "ID").unwrap_or_else(|| "unknown".into());
    let distro_version = parse_os_field(&os_release, "VERSION_ID").unwrap_or_else(|| "0".into());
    let distro_codename = parse_os_field(&os_release, "VERSION_CODENAME").unwrap_or_default();
    let id_like = parse_os_field(&os_release, "ID_LIKE").unwrap_or_default();

    // Determine family
    let distro_family = determine_family(&distro, &id_like);

    // Sanity checks per family
    match distro_family.as_str() {
        "debian" => {
            let ver_major: u32 = distro_version
                .split('.')
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            if distro == "ubuntu" && ver_major < 22 {
                bail!("Ubuntu {} is too old; 22.04+ is required.", distro_version);
            }
        }
        "arch" => {
            tracing::info!("Detected Arch-based distro: {} {}", distro, distro_version);
        }
        _ => {
            tracing::warn!(
                "Detected distro '{}' (family '{}') – this installer targets \
                 Ubuntu/Debian and Manjaro/Arch but may partially work.",
                distro,
                distro_family
            );
        }
    }

    let pi_model = detect_pi_model();

    Ok(PlatformInfo {
        arch,
        distro,
        distro_version,
        distro_codename,
        distro_family,
        pi_model,
    })
}

fn determine_family(distro: &str, id_like: &str) -> String {
    // Exact matches first
    match distro {
        "ubuntu" | "debian" | "linuxmint" | "pop" | "raspbian" => {
            return "debian".into();
        }
        "manjaro" | "arch" | "endeavouros" | "garuda" | "artix" | "cachyos" => {
            return "arch".into();
        }
        _ => {}
    }
    // Fall back to ID_LIKE
    let like_lower = id_like.to_lowercase();
    if like_lower.contains("arch") || like_lower.contains("manjaro") {
        "arch".into()
    } else if like_lower.contains("debian") || like_lower.contains("ubuntu") {
        "debian".into()
    } else {
        "unknown".into()
    }
}

fn parse_os_field(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix(&format!("{key}=")) {
            return Some(rest.trim_matches('"').to_string());
        }
    }
    None
}

fn detect_pi_model() -> Option<String> {
    if let Ok(model) = fs::read_to_string("/proc/device-tree/model") {
        let model = model.trim_end_matches('\0').trim().to_string();
        if !model.is_empty() {
            return Some(model);
        }
    }
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("Model") || line.starts_with("Hardware") {
                if let Some((_k, v)) = line.split_once(':') {
                    return Some(v.trim().to_string());
                }
            }
        }
    }
    None
}
