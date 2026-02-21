use anyhow::{anyhow, Result};
use clap::ValueEnum;
use nix::sys::statvfs;
use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use crate::{
    cmd, config,
    context::{ConfigOverrides, ConfigService},
    system::{RealSystem, SystemOps},
};

#[derive(Clone, Copy, Debug, ValueEnum, Default)]
#[value(rename_all = "lower")]
pub enum DoctorOutput {
    #[default]
    Pretty,
    Json,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum CheckStatus {
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug, Serialize)]
pub struct PreflightCheck {
    pub label: String,
    pub status: CheckStatus,
    pub detail: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PreflightReport {
    pub checks: Vec<PreflightCheck>,
}

const MIN_MEMORY_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const WARN_MEMORY_BYTES: u64 = 3 * 1024 * 1024 * 1024;
const MIN_CPU_CORES: usize = 2;

/// Run diagnostics and write a summary of what is installed / missing.
#[allow(dead_code)]
pub fn run_doctor(format: DoctorOutput, out: &mut dyn Write) -> Result<()> {
    writeln!(out, "mash-setup doctor")?;
    writeln!(out, "==================")?;
    writeln!(out)?;

    let system = RealSystem;
    let report = collect_preflight_checks(&system, None)?;
    if matches!(format, DoctorOutput::Json) {
        writeln!(out, "{}", serde_json::to_string_pretty(&report)?)?;
        return Ok(());
    }
    write_section(out, "Pre-flight checks")?;
    display_preflight_checks(&report.checks, out)?;
    writeln!(out)?;

    // ── System info ──
    write_section(out, "System")?;
    show_file(
        &system,
        out,
        Path::new("/etc/os-release"),
        &["PRETTY_NAME", "VERSION_ID"],
    )?;
    show_cmd(&system, out, "Architecture", "uname", &["-m"])?;
    show_cmd(&system, out, "Kernel", "uname", &["-r"])?;

    // Pi model
    if let Ok(model) = system.read_to_string(Path::new("/proc/device-tree/model")) {
        let model = model.trim_end_matches('\0').trim();
        writeln!(out, "  Pi model:      {model}")?;
    }

    writeln!(out)?;

    // ── Package manager ──
    write_section(out, "Package manager")?;
    if which::which("pacman").is_ok() {
        writeln!(out, "  Backend:       pacman (Arch-based)")?;
    } else {
        writeln!(out, "  Backend:       apt (Debian-based)")?;
    }

    writeln!(out)?;

    // ── Disk space ──
    write_section(out, "Disk space")?;
    let df_output = Command::new("df")
        .args([
            "-h",
            "--output=target,avail,pcent",
            "/",
            "/mnt/data",
            "/data",
        ])
        .output();
    if let Ok(o) = df_output {
        out.write_all(&o.stdout)?;
    }
    writeln!(out)?;

    // ── Tools ──
    write_section(out, "Tools")?;
    let tools = [
        ("git", "git --version"),
        ("gh", "gh --version"),
        ("rustup", "rustup --version"),
        ("cargo", "cargo --version"),
        ("rustc", "rustc --version"),
        ("docker", "docker --version"),
        ("docker compose", "docker compose version"),
        ("python3", "python3 --version"),
        ("node", "node --version"),
        ("npm", "npm --version"),
        ("zsh", "zsh --version"),
        ("starship", "starship --version"),
        ("rclone", "rclone --version"),
        ("tmux", "tmux -V"),
        ("neovim", "nvim --version"),
        ("ripgrep", "rg --version"),
        ("fd", "fd --version"),
        ("fzf", "fzf --version"),
        ("jq", "jq --version"),
        ("bat", "bat --version"),
        ("just", "just --version"),
        ("sccache", "sccache --version"),
        ("bacon", "bacon --version"),
        ("argononed", "argononed --version"),
    ];

    for (name, cmd_str) in &tools {
        check_tool(&system, out, name, cmd_str)?;
    }
    writeln!(out)?;

    // ── Cargo tools ──
    write_section(out, "Cargo tools")?;
    let cargo_tools = [
        "cargo-watch",
        "cargo-audit",
        "cargo-maelstrom",
        "cargo-add",
        "bacon",
        "just",
        "sccache",
    ];
    for tool in &cargo_tools {
        let home = dirs::home_dir().unwrap_or_default();
        let path = home.join(".cargo/bin").join(tool);
        let status = if path.exists() || which::which(tool).is_ok() {
            "installed"
        } else {
            "MISSING"
        };
        writeln!(out, "  {tool:<20} {status}")?;
    }
    writeln!(out)?;

    // ── Docker group ──
    write_section(out, "Docker group")?;
    let user = std::env::var("USER").unwrap_or_else(|_| "unknown".into());
    let output = Command::new("id").arg("-nG").arg(&user).output();
    match output {
        Ok(o) => {
            let groups = String::from_utf8_lossy(&o.stdout);
            let in_docker = groups.split_whitespace().any(|g| g == "docker");
            writeln!(
                out,
                "  User '{user}' in docker group: {}",
                if in_docker { "yes" } else { "NO" }
            )?;
        }
        Err(_) => writeln!(out, "  Could not determine groups for user '{user}'")?,
    }
    writeln!(out)?;

    // ── Config ──
    write_section(out, "Config")?;
    let config_path = config::config_path();
    writeln!(
        out,
        "  Config file: {} ({})",
        config_path.display(),
        if config_path.exists() {
            "exists"
        } else {
            "not found"
        }
    )?;

    // ── SSH ──
    write_section(out, "SSH keys")?;
    let ssh_dir = dirs::home_dir().unwrap_or_default().join(".ssh");
    if ssh_dir.exists() {
        for e in std::fs::read_dir(&ssh_dir).into_iter().flatten().flatten() {
            let name = e.file_name();
            let name = name.to_string_lossy();
            if name.ends_with(".pub") {
                writeln!(out, "  {}", e.path().display())?;
            }
        }
    } else {
        writeln!(out, "  No ~/.ssh directory found")?;
    }
    writeln!(out)?;

    Ok(())
}

const REQUIRED_COMMANDS: &[&str] = &["curl", "git", "tar", "docker"];
const MIN_ROOT_SPACE_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const CONNECTIVITY_TARGETS: &[(&str, u16)] = &[
    ("github.com", 443),
    ("crates.io", 443),
    ("registry-1.docker.io", 443),
];
const SUPPORTED_PACKAGE_MANAGERS: &[(&str, &str)] = &[
    ("apt", "APT"),
    ("apt-get", "APT"),
    ("pacman", "pacman"),
    ("dnf", "DNF"),
    ("yum", "YUM"),
];
const CONNECTIVITY_TIMEOUT_SECS: u64 = 5;
const WRITE_TEST_FILE: &str = ".mash-doctor-write-test";

#[allow(dead_code)]
pub fn run_preflight_checks(
    system: &dyn SystemOps,
    staging_override: Option<&Path>,
    out: &mut dyn Write,
) -> Result<()> {
    write_section(out, "Pre-flight checks")?;
    let report = collect_preflight_checks(system, staging_override)?;
    let had_errno = display_preflight_checks(&report.checks, out)?;
    writeln!(out)?;
    if had_errno {
        Err(anyhow!("pre-flight checks reported critical issues"))
    } else {
        writeln!(out, "  Pre-flight checks passed")?;
        writeln!(out)?;
        Ok(())
    }
}

pub fn collect_preflight_checks(
    system: &dyn SystemOps,
    staging_override: Option<&Path>,
) -> Result<PreflightReport> {
    let mut checks = Vec::new();

    for &cmd in REQUIRED_COMMANDS {
        checks.push(check_required_command(cmd));
    }

    checks.push(check_root_partition());
    checks.push(check_memory());
    checks.push(check_cpu());
    checks.push(check_package_manager());

    for &(host, port) in CONNECTIVITY_TARGETS {
        checks.push(connectivity_check_entry(system, host, port));
    }

    if let Some(home_dir) = dirs::home_dir() {
        checks.push(directory_writeable_check_entry(&home_dir, "Home directory"));
    } else {
        checks.push(PreflightCheck {
            label: "Home directory".into(),
            status: CheckStatus::Warning,
            detail: Some("Unable to determine user home directory".into()),
        });
    }

    let overrides = ConfigOverrides {
        staging_dir: staging_override.map(|p| p.to_path_buf()),
    };
    let config_service = ConfigService::load_with_overrides(overrides)?;
    match config_service.resolve_staging_dir() {
        Ok(path) => checks.push(directory_writeable_check_entry(&path, "Staging directory")),
        Err(err) => checks.push(PreflightCheck {
            label: "Staging directory".into(),
            status: CheckStatus::Error,
            detail: Some(format!("{err}")),
        }),
    }

    checks.push(check_sudo());
    checks.push(check_os_compatibility());
    checks.push(check_existing_config());

    // Add Pi 4B HDD specific checks
    if let Ok(pi4b_checks) = crate::pi4b_hdd::pi4b_hdd_preflight_checks(system) {
        checks.extend(pi4b_checks);
    }

    Ok(PreflightReport { checks })
}

fn display_preflight_checks(checks: &[PreflightCheck], out: &mut dyn Write) -> Result<bool> {
    let mut had_error = false;
    for check in checks {
        let mut label = check.label.clone();
        if let Some(detail) = &check.detail {
            label = format!("{label}: {detail}");
        }
        write_report(out, label, check.status)?;
        if check.status == CheckStatus::Error {
            had_error = true;
        }
    }
    Ok(had_error)
}

fn check_required_command(cmd: &str) -> PreflightCheck {
    match which::which(cmd) {
        Ok(path) => PreflightCheck {
            label: format!("{cmd} command"),
            status: CheckStatus::Success,
            detail: Some(format!("available at {}", path.display())),
        },
        Err(_) => PreflightCheck {
            label: format!("{cmd} command"),
            status: CheckStatus::Error,
            detail: Some("not found in PATH".into()),
        },
    }
}

fn check_root_partition() -> PreflightCheck {
    match check_root_space() {
        Ok(bytes) => PreflightCheck {
            label: format!("Root partition has {}", format_bytes(bytes)),
            status: CheckStatus::Success,
            detail: None,
        },
        Err(err) => PreflightCheck {
            label: "Root partition".into(),
            status: CheckStatus::Error,
            detail: Some(err.to_string()),
        },
    }
}

fn check_memory() -> PreflightCheck {
    match read_mem_available() {
        Ok(bytes) => {
            let status = if bytes < MIN_MEMORY_BYTES {
                CheckStatus::Error
            } else if bytes < WARN_MEMORY_BYTES {
                CheckStatus::Warning
            } else {
                CheckStatus::Success
            };
            let detail = if status == CheckStatus::Success {
                None
            } else {
                Some(format!("{} available", format_bytes(bytes)))
            };
            PreflightCheck {
                label: "Available memory".into(),
                status,
                detail,
            }
        }
        Err(err) => PreflightCheck {
            label: "Available memory".into(),
            status: CheckStatus::Warning,
            detail: Some(format!("unable to read memory info: {err}")),
        },
    }
}

fn check_cpu() -> PreflightCheck {
    let cores = num_cpus::get();
    let status = if cores < MIN_CPU_CORES {
        CheckStatus::Warning
    } else {
        CheckStatus::Success
    };
    PreflightCheck {
        label: format!("CPU cores: {cores}"),
        status,
        detail: if status == CheckStatus::Success {
            None
        } else {
            Some(format!("minimum {MIN_CPU_CORES} cores recommended"))
        },
    }
}

fn check_package_manager() -> PreflightCheck {
    for (cmd, label) in SUPPORTED_PACKAGE_MANAGERS {
        if which::which(cmd).is_ok() {
            return PreflightCheck {
                label: format!("{label} package manager"),
                status: CheckStatus::Success,
                detail: Some(format!("available via {cmd}")),
            };
        }
    }

    PreflightCheck {
        label: "Package manager".into(),
        status: CheckStatus::Error,
        detail: Some("No supported package manager available".into()),
    }
}

fn connectivity_check_entry(system: &dyn SystemOps, host: &str, port: u16) -> PreflightCheck {
    match check_connectivity(
        system,
        host,
        port,
        Duration::from_secs(CONNECTIVITY_TIMEOUT_SECS),
    ) {
        Ok(_) => PreflightCheck {
            label: format!("{host}:{port} reachable"),
            status: CheckStatus::Success,
            detail: None,
        },
        Err(err) => PreflightCheck {
            label: format!("{host}:{port} connectivity"),
            status: CheckStatus::Error,
            detail: Some(err.to_string()),
        },
    }
}

fn directory_writeable_check_entry(path: &Path, label: &str) -> PreflightCheck {
    match check_directory_writeable(path) {
        Ok(_) => PreflightCheck {
            label: format!("{label} writeable: {}", path.display()),
            status: CheckStatus::Success,
            detail: None,
        },
        Err(err) => PreflightCheck {
            label: format!("{label} write check"),
            status: CheckStatus::Error,
            detail: Some(format!("{} ({err})", path.display())),
        },
    }
}

fn check_sudo() -> PreflightCheck {
    if which::which("sudo").is_err() {
        return PreflightCheck {
            label: "sudo availability".into(),
            status: CheckStatus::Warning,
            detail: Some("sudo binary not found".into()),
        };
    }
    let mut command = Command::new("sudo");
    command.args(["-n", "true"]);
    match cmd::run(&mut command) {
        Ok(_) => PreflightCheck {
            label: "sudo access".into(),
            status: CheckStatus::Success,
            detail: None,
        },
        Err(err) => PreflightCheck {
            label: "sudo access".into(),
            status: CheckStatus::Warning,
            detail: Some(format!("{err}")),
        },
    }
}

fn check_os_compatibility() -> PreflightCheck {
    match fs::read_to_string("/etc/os-release") {
        Ok(contents) => {
            let ids = parse_os_release_ids(&contents);
            let supported = ["debian", "ubuntu", "raspbian", "arch", "manjaro", "fedora"];
            if ids.iter().any(|id| supported.contains(&id.as_str())) {
                PreflightCheck {
                    label: format!("Operating system: {}", ids.join(",")),
                    status: CheckStatus::Success,
                    detail: None,
                }
            } else {
                PreflightCheck {
                    label: "Operating system compatibility".into(),
                    status: CheckStatus::Warning,
                    detail: Some(format!(
                        "Detected {} - not explicitly supported yet",
                        ids.join(", ")
                    )),
                }
            }
        }
        Err(err) => PreflightCheck {
            label: "Operating system".into(),
            status: CheckStatus::Warning,
            detail: Some(format!("unable to read /etc/os-release ({err})")),
        },
    }
}

fn check_existing_config() -> PreflightCheck {
    let path = config::config_path();
    if path.exists() {
        PreflightCheck {
            label: "Mash configuration".into(),
            status: CheckStatus::Warning,
            detail: Some(format!(
                "{} already exists; re-running may override settings",
                path.display()
            )),
        }
    } else {
        PreflightCheck {
            label: "Mash configuration".into(),
            status: CheckStatus::Success,
            detail: None,
        }
    }
}

fn read_mem_available() -> Result<u64> {
    let content = fs::read_to_string("/proc/meminfo")?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("MemAvailable:") {
            let value = rest
                .split_whitespace()
                .next()
                .ok_or_else(|| anyhow!("MemAvailable entry in /proc/meminfo is malformed"))?;
            let kb: u64 = value.parse()?;
            return Ok(kb * 1024);
        }
    }
    Err(anyhow!("MemAvailable entry missing from /proc/meminfo"))
}

fn parse_os_release_ids(contents: &str) -> Vec<String> {
    let mut ids = Vec::new();
    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("ID=") {
            ids.push(rest.trim_matches('"').to_lowercase());
        }
        if let Some(rest) = line.strip_prefix("ID_LIKE=") {
            for id in rest.trim_matches('"').split_whitespace() {
                ids.push(id.to_lowercase());
            }
        }
    }
    if ids.is_empty() {
        ids.push("unknown".into());
    }
    ids
}

fn check_root_space() -> Result<u64> {
    let stat = statvfs::statvfs("/")?;
    let avail = stat.blocks_available() as u64 * stat.fragment_size() as u64;
    if avail < MIN_ROOT_SPACE_BYTES {
        Err(anyhow!(
            "only {} free on '/', need at least {}",
            format_bytes(avail),
            format_bytes(MIN_ROOT_SPACE_BYTES)
        ))
    } else {
        Ok(avail)
    }
}

fn check_connectivity(
    system: &dyn SystemOps,
    host: &str,
    port: u16,
    timeout: Duration,
) -> Result<()> {
    system.connect(host, port, timeout)?;
    Ok(())
}

fn check_directory_writeable(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)?;
    let test_file = path.join(WRITE_TEST_FILE);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&test_file)?;
    file.write_all(b"mash-doctor-check")?;
    drop(file);
    let _ = std::fs::remove_file(&test_file);
    Ok(())
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    let amount = bytes as f64;
    if amount >= KB.powi(3) {
        format!("{:.1} GiB", amount / KB.powi(3))
    } else if amount >= KB.powi(2) {
        format!("{:.1} MiB", amount / KB.powi(2))
    } else if amount >= KB {
        format!("{:.1} KiB", amount / KB)
    } else {
        format!("{bytes} bytes")
    }
}

#[allow(dead_code)]
fn write_section(out: &mut dyn Write, name: &str) -> std::io::Result<()> {
    writeln!(out, "── {name} ──")
}

#[allow(dead_code)]
fn show_cmd(
    system: &dyn SystemOps,
    out: &mut dyn Write,
    label: &str,
    cmd: &str,
    args: &[&str],
) -> std::io::Result<()> {
    let mut command = Command::new(cmd);
    command.args(args);
    match system.command_output(&mut command) {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            writeln!(out, "  {label:<16} {}", stdout.trim())
        }
        Err(_) => writeln!(out, "  {label:<16} (not available)"),
    }
}

#[allow(dead_code)]
fn show_file(
    system: &dyn SystemOps,
    out: &mut dyn Write,
    path: &Path,
    keys: &[&str],
) -> std::io::Result<()> {
    if let Ok(content) = system.read_to_string(path) {
        for key in keys {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix(&format!("{key}=")) {
                    writeln!(out, "  {key:<16} {}", rest.trim_matches('"'))?;
                }
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn check_tool(
    system: &dyn SystemOps,
    out: &mut dyn Write,
    name: &str,
    cmd_str: &str,
) -> std::io::Result<()> {
    let parts: Vec<&str> = cmd_str.split_whitespace().collect();
    let mut command = Command::new(parts[0]);
    command.args(&parts[1..]);

    match system.command_output(&mut command) {
        Ok(o) if o.status.success() => {
            let ver = String::from_utf8_lossy(&o.stdout);
            let first_line = ver.lines().next().unwrap_or("").trim();
            writeln!(out, "  {name:<20} {first_line}")
        }
        _ => writeln!(out, "  {name:<20} MISSING"),
    }
}

fn write_report(
    out: &mut dyn Write,
    message: impl AsRef<str>,
    status: CheckStatus,
) -> std::io::Result<()> {
    match status {
        CheckStatus::Success => writeln!(out, "{}", message.as_ref()),
        CheckStatus::Warning => writeln!(out, "Warning: {}", message.as_ref()),
        CheckStatus::Error => writeln!(out, "ERROR: {}", message.as_ref()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::{RealSystem, SystemOps};
    use std::net::{TcpListener, TcpStream};
    use std::path::Path;
    use std::process::Command;
    use std::process::Output;
    use std::thread;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn format_bytes_human_readable() {
        assert_eq!(format_bytes(0), "0 bytes");
        assert_eq!(format_bytes(1024), "1.0 KiB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MiB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.0 GiB");
    }

    #[test]
    fn check_directory_writeable_temp_dir() -> Result<()> {
        let base = std::env::temp_dir().join(format!("mash-doctor-{}", std::process::id()));
        std::fs::create_dir_all(&base)?;
        let result = check_directory_writeable(&base);
        let _ = std::fs::remove_dir_all(&base);
        result
    }

    #[test]
    fn check_connectivity_localhost() -> Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;
        let handle = thread::spawn(move || {
            let _ = listener.accept();
        });
        let system = RealSystem;
        let result = check_connectivity(&system, "127.0.0.1", addr.port(), Duration::from_secs(1));
        let _ = handle.join();
        result
    }

    type ConnectFn = Box<dyn Fn(&str, u16, Duration) -> Result<TcpStream> + Send + Sync>;

    struct StubSystem {
        connect_fn: ConnectFn,
    }

    impl StubSystem {
        fn with_connect<F>(connect: F) -> Self
        where
            F: Fn(&str, u16, Duration) -> Result<TcpStream> + Send + Sync + 'static,
        {
            Self {
                connect_fn: Box::new(connect),
            }
        }
    }

    impl SystemOps for StubSystem {
        fn read_to_string(&self, _path: &Path) -> Result<String> {
            Err(anyhow!("not implemented"))
        }

        fn command_output(&self, _cmd: &mut Command) -> Result<Output> {
            Err(anyhow!("not implemented"))
        }

        fn connect(&self, host: &str, port: u16, timeout: Duration) -> Result<TcpStream> {
            (self.connect_fn)(host, port, timeout)
        }
    }

    #[test]
    fn connectivity_check_entry_reports_success() -> Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;
        let handle = thread::spawn(move || {
            let _ = listener.accept();
        });

        let system = StubSystem::with_connect(move |_, _, _| Ok(TcpStream::connect(addr)?));
        let check = connectivity_check_entry(&system, "example.com", 1234);
        assert_eq!(check.status, CheckStatus::Success);
        let _ = handle.join();
        Ok(())
    }

    #[test]
    fn connectivity_check_entry_reports_failure() {
        let system = StubSystem::with_connect(move |_, _, _| Err(anyhow!("network gone")));
        let check = connectivity_check_entry(&system, "example.com", 1234);
        assert_eq!(check.status, CheckStatus::Error);
        assert!(check.detail.unwrap().contains("network gone"));
    }

    #[test]
    fn directory_writeable_check_entry_reports_success() -> Result<()> {
        let base = tempdir()?;
        let check = directory_writeable_check_entry(base.path(), "Temp");
        assert_eq!(check.status, CheckStatus::Success);
        assert!(check.detail.is_none());
        Ok(())
    }

    #[test]
    fn display_preflight_checks_reports_errors() {
        let success = PreflightCheck {
            label: "safe".into(),
            status: CheckStatus::Success,
            detail: None,
        };
        let error_check = PreflightCheck {
            label: "fail".into(),
            status: CheckStatus::Error,
            detail: None,
        };
        let mut buf = Vec::new();
        assert!(!display_preflight_checks(&[success], &mut buf).unwrap());
        let mut buf = Vec::new();
        assert!(display_preflight_checks(&[error_check], &mut buf).unwrap());
    }
}
