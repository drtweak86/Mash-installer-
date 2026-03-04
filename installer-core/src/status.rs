use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::process::Command;

use crate::config;

#[derive(Clone, Copy, Debug, ValueEnum, Default)]
#[value(rename_all = "lower")]
pub enum StatusOutput {
    #[default]
    Pretty,
    Json,
}

#[derive(Clone, Debug, Serialize)]
pub struct PlatformStatus {
    pub distro: String,
    pub arch: String,
    pub pi_model: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ConfigStatus {
    pub path: String,
    /// "loaded", "missing", or "invalid"
    pub state: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct WallpaperKeyStatus {
    pub provider: &'static str,
    pub env_var: &'static str,
    pub present: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct PreflightSummary {
    pub pass: usize,
    pub warn: usize,
    pub fail: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct StatusReport {
    pub platform: PlatformStatus,
    pub config: ConfigStatus,
    pub wallpaper_keys: Vec<WallpaperKeyStatus>,
    pub preflight: PreflightSummary,
}

/// Collect and display a lightweight status overview.
/// Unlike `doctor`, this skips network checks so it returns immediately.
pub fn run_status(format: StatusOutput, out: &mut dyn Write) -> Result<()> {
    let report = collect_status()?;

    if matches!(format, StatusOutput::Json) {
        writeln!(out, "{}", serde_json::to_string_pretty(&report)?)?;
        return Ok(());
    }

    writeln!(out, "mash-setup status")?;
    writeln!(out, "=================")?;
    writeln!(out)?;

    writeln!(out, "── Platform ──")?;
    writeln!(out, "  Distro:   {}", report.platform.distro)?;
    writeln!(out, "  Arch:     {}", report.platform.arch)?;
    if let Some(model) = &report.platform.pi_model {
        writeln!(out, "  Pi:       {}", model)?;
    }
    writeln!(out)?;

    writeln!(out, "── Configuration ──")?;
    writeln!(
        out,
        "  Config:   {} [{}]",
        report.config.path, report.config.state
    )?;
    writeln!(out)?;

    writeln!(out, "── Wallpaper API keys ──")?;
    for key in &report.wallpaper_keys {
        let status = if key.present {
            "PASS"
        } else {
            "WARN (not set)"
        };
        writeln!(out, "  {:<12} {}", key.provider, status)?;
    }
    writeln!(out)?;

    writeln!(out, "── Pre-flight summary (fast checks) ──")?;
    writeln!(
        out,
        "  pass: {}  warn: {}  fail: {}",
        report.preflight.pass, report.preflight.warn, report.preflight.fail
    )?;
    if report.preflight.fail > 0 {
        writeln!(out, "  Run `mash-setup doctor` for full details.")?;
    }
    writeln!(out)?;

    Ok(())
}

fn collect_status() -> Result<StatusReport> {
    Ok(StatusReport {
        platform: detect_platform_status(),
        config: detect_config_status(),
        wallpaper_keys: detect_wallpaper_keys(),
        preflight: run_fast_preflight(),
    })
}

fn detect_platform_status() -> PlatformStatus {
    let distro = fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|l| l.starts_with("PRETTY_NAME="))
                .map(|l| {
                    l.trim_start_matches("PRETTY_NAME=")
                        .trim_matches('"')
                        .to_string()
                })
        })
        .unwrap_or_else(|| "Unknown".into());

    let arch = Command::new("uname")
        .arg("-m")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".into());

    let pi_model = fs::read_to_string("/proc/device-tree/model")
        .ok()
        .map(|s| s.trim_end_matches('\0').trim().to_string())
        .filter(|s| !s.is_empty());

    PlatformStatus {
        distro,
        arch,
        pi_model,
    }
}

fn detect_config_status() -> ConfigStatus {
    let path = config::config_path();
    let state = if !path.exists() {
        "missing".into()
    } else {
        match config::load_or_default() {
            Ok(_) => "loaded".into(),
            Err(_) => "invalid".into(),
        }
    };
    ConfigStatus {
        path: path.display().to_string(),
        state,
    }
}

fn detect_wallpaper_keys() -> Vec<WallpaperKeyStatus> {
    let providers: &[(&str, &str)] = &[
        ("Wallhaven", "MASH_WALLHAVEN_KEY"),
        ("Pexels", "MASH_PEXELS_KEY"),
        ("Pixabay", "MASH_PIXABAY_KEY"),
    ];
    providers
        .iter()
        .map(|(provider, env_var)| WallpaperKeyStatus {
            provider,
            env_var,
            present: std::env::var(env_var)
                .map(|v| !v.trim().is_empty())
                .unwrap_or(false),
        })
        .collect()
}

const FAST_CHECK_COUNT: usize = 7;

/// Fast preflight: required tools, memory, CPU, package manager, OS.
/// Skips network connectivity so status returns instantly.
fn run_fast_preflight() -> PreflightSummary {
    let mut pass = 0usize;
    let mut warn = 0usize;
    let mut fail = 0usize;

    // Required tools (error if missing)
    for tool in &["curl", "git", "tar"] {
        if which::which(tool).is_ok() {
            pass += 1;
        } else {
            fail += 1;
        }
    }

    // Memory: error <2 GiB, warn <3 GiB
    match read_mem_available_kb() {
        Ok(kb) => {
            let bytes = kb * 1024;
            if bytes < 2 * 1024 * 1024 * 1024 {
                fail += 1;
            } else if bytes < 3 * 1024 * 1024 * 1024 {
                warn += 1;
            } else {
                pass += 1;
            }
        }
        Err(_) => warn += 1,
    }

    // CPU cores (warn if <2)
    if std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        >= 2
    {
        pass += 1;
    } else {
        warn += 1;
    }

    // Package manager (error if none found)
    let has_pm = ["apt", "apt-get", "pacman", "dnf", "yum"]
        .iter()
        .any(|pm| which::which(pm).is_ok());
    if has_pm {
        pass += 1;
    } else {
        fail += 1;
    }

    // OS compatibility (warn if unrecognised)
    if os_is_supported() {
        pass += 1;
    } else {
        warn += 1;
    }

    debug_assert_eq!(pass + warn + fail, FAST_CHECK_COUNT);
    PreflightSummary { pass, warn, fail }
}

fn read_mem_available_kb() -> Result<u64> {
    let content = fs::read_to_string("/proc/meminfo")?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("MemAvailable:") {
            let kb: u64 = rest
                .split_whitespace()
                .next()
                .ok_or_else(|| anyhow::anyhow!("malformed MemAvailable"))?
                .parse()?;
            return Ok(kb);
        }
    }
    Err(anyhow::anyhow!("MemAvailable not found in /proc/meminfo"))
}

fn os_is_supported() -> bool {
    let supported = ["debian", "ubuntu", "raspbian", "arch", "manjaro", "fedora"];
    fs::read_to_string("/etc/os-release")
        .map(|content| {
            content.lines().any(|l| {
                let val = l
                    .strip_prefix("ID=")
                    .or_else(|| l.strip_prefix("ID_LIKE="))
                    .map(|v| v.trim_matches('"').to_lowercase());
                val.map(|v| supported.iter().any(|s| v.contains(s)))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_pretty_output_contains_sections() {
        let mut buf = Vec::new();
        run_status(StatusOutput::Pretty, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("── Platform ──"));
        assert!(output.contains("── Configuration ──"));
        assert!(output.contains("── Wallpaper API keys ──"));
        assert!(output.contains("── Pre-flight summary"));
    }

    #[test]
    fn status_json_output_is_valid_json() {
        let mut buf = Vec::new();
        run_status(StatusOutput::Json, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(parsed.get("platform").is_some());
        assert!(parsed.get("config").is_some());
        assert!(parsed.get("wallpaper_keys").is_some());
        assert!(parsed.get("preflight").is_some());
    }

    #[test]
    fn wallpaper_keys_reports_three_providers() {
        let keys = detect_wallpaper_keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.iter().any(|k| k.provider == "Wallhaven"));
        assert!(keys.iter().any(|k| k.provider == "Pexels"));
        assert!(keys.iter().any(|k| k.provider == "Pixabay"));
    }

    #[test]
    fn fast_preflight_counts_sum_to_check_count() {
        let summary = run_fast_preflight();
        assert_eq!(summary.pass + summary.warn + summary.fail, FAST_CHECK_COUNT);
    }
}
