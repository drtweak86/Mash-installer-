use anyhow::{anyhow, Context, Result};
use nix::sys::statvfs;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use crate::{config, staging};

/// Run diagnostics and print a summary of what is installed / missing.
#[allow(dead_code)]
pub fn run_doctor() -> Result<()> {
    println!("mash-setup doctor");
    println!("==================");
    println!();

    run_preflight_checks()?;

    // ── System info ──
    section("System");
    show_file("/etc/os-release", &["PRETTY_NAME", "VERSION_ID"]);
    show_cmd("Architecture", "uname", &["-m"]);
    show_cmd("Kernel", "uname", &["-r"]);

    // Pi model
    if let Ok(model) = std::fs::read_to_string("/proc/device-tree/model") {
        let model = model.trim_end_matches('\0').trim();
        println!("  Pi model:      {model}");
    }

    println!();

    // ── Package manager ──
    section("Package manager");
    if which::which("pacman").is_ok() {
        println!("  Backend:       pacman (Arch-based)");
    } else {
        println!("  Backend:       apt (Debian-based)");
    }

    println!();

    // ── Disk space ──
    section("Disk space");
    let _ = Command::new("df")
        .args([
            "-h",
            "--output=target,avail,pcent",
            "/",
            "/mnt/data",
            "/data",
        ])
        .status();
    println!();

    // ── Tools ──
    section("Tools");
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
        check_tool(name, cmd_str);
    }
    println!();

    // ── Cargo tools ──
    section("Cargo tools");
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
        println!("  {tool:<20} {status}");
    }
    println!();

    // ── Docker group ──
    section("Docker group");
    let user = std::env::var("USER").unwrap_or_else(|_| "unknown".into());
    let output = Command::new("id").arg("-nG").arg(&user).output();
    match output {
        Ok(o) => {
            let groups = String::from_utf8_lossy(&o.stdout);
            let in_docker = groups.split_whitespace().any(|g| g == "docker");
            println!(
                "  User '{user}' in docker group: {}",
                if in_docker { "yes" } else { "NO" }
            );
        }
        Err(_) => println!("  Could not determine groups for user '{user}'"),
    }
    println!();

    // ── Config ──
    section("Config");
    let config_path = dirs::home_dir()
        .unwrap_or_default()
        .join(".config/mash-installer/config.toml");
    println!(
        "  Config file: {} ({})",
        config_path.display(),
        if config_path.exists() {
            "exists"
        } else {
            "not found"
        }
    );

    // ── SSH ──
    section("SSH keys");
    let ssh_dir = dirs::home_dir().unwrap_or_default().join(".ssh");
    if ssh_dir.exists() {
        for e in std::fs::read_dir(&ssh_dir).into_iter().flatten().flatten() {
            let name = e.file_name();
            let name = name.to_string_lossy();
            if name.ends_with(".pub") {
                println!("  {}", e.path().display());
            }
        }
    } else {
        println!("  No ~/.ssh directory found");
    }
    println!();

    Ok(())
}

const REQUIRED_COMMANDS: &[&str] = &["curl", "git", "tar"];
const MIN_ROOT_SPACE_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const CONNECTIVITY_TARGETS: &[(&str, u16)] = &[("github.com", 443), ("crates.io", 443)];
const CONNECTIVITY_TIMEOUT_SECS: u64 = 5;
const WRITE_TEST_FILE: &str = ".mash-doctor-write-test";

fn run_preflight_checks() -> Result<()> {
    section("Pre-flight checks");
    let mut failures = Vec::new();

    for &cmd in REQUIRED_COMMANDS {
        match which::which(cmd) {
            Ok(path) => println!("  {cmd:<16} {}", path.display()),
            Err(_) => {
                println!("  {cmd:<16} MISSING");
                failures.push(format!("command '{cmd}' not found in PATH"));
            }
        }
    }

    match check_root_space() {
        Ok(bytes) => println!("  Root partition: {} free", format_bytes(bytes)),
        Err(err) => {
            println!("  Root partition check failed: {err}");
            failures.push(err.to_string());
        }
    }

    for &(host, port) in CONNECTIVITY_TARGETS {
        match check_connectivity(host, port, Duration::from_secs(CONNECTIVITY_TIMEOUT_SECS)) {
            Ok(_) => println!("  {host}:{port} reachable"),
            Err(err) => {
                println!("  {host}:{port} unreachable ({err})");
                failures.push(err.to_string());
            }
        }
    }

    let home_dir = dirs::home_dir().ok_or_else(|| anyhow!("could not determine home directory"))?;
    match check_directory_writeable(&home_dir) {
        Ok(_) => println!("  Home directory writeable: {}", home_dir.display()),
        Err(err) => {
            println!(
                "  Home directory ({}) write check failed: {err}",
                home_dir.display()
            );
            failures.push(err.to_string());
        }
    }

    let cfg = config::load_or_default()?;
    let staging_dir = staging::resolve(None, &cfg).context("resolving staging directory")?;
    match check_directory_writeable(&staging_dir) {
        Ok(_) => println!("  Staging directory writeable: {}", staging_dir.display()),
        Err(err) => {
            println!(
                "  Staging directory ({}) write check failed: {err}",
                staging_dir.display()
            );
            failures.push(err.to_string());
        }
    }

    println!();
    if failures.is_empty() {
        println!("  Pre-flight checks passed");
        println!();
        Ok(())
    } else {
        Err(anyhow!(failures.join("; ")))
    }
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

fn check_connectivity(host: &str, port: u16, timeout: Duration) -> Result<()> {
    let addrs = (host, port).to_socket_addrs()?;
    for addr in addrs {
        if TcpStream::connect_timeout(&addr, timeout).is_ok() {
            return Ok(());
        }
    }
    Err(anyhow!("failed to reach {host}:{port}"))
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
fn section(name: &str) {
    println!("── {name} ──");
}

#[allow(dead_code)]
fn show_cmd(label: &str, cmd: &str, args: &[&str]) {
    match Command::new(cmd).args(args).output() {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout);
            println!("  {label:<16} {}", out.trim());
        }
        Err(_) => println!("  {label:<16} (not available)"),
    }
}

#[allow(dead_code)]
fn show_file(path: &str, keys: &[&str]) {
    if let Ok(content) = std::fs::read_to_string(path) {
        for key in keys {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix(&format!("{key}=")) {
                    println!("  {key:<16} {}", rest.trim_matches('"'));
                }
            }
        }
    }
}

#[allow(dead_code)]
fn check_tool(name: &str, cmd_str: &str) {
    let parts: Vec<&str> = cmd_str.split_whitespace().collect();
    let result = Command::new(parts[0]).args(&parts[1..]).output();

    match result {
        Ok(o) if o.status.success() => {
            let ver = String::from_utf8_lossy(&o.stdout);
            let first_line = ver.lines().next().unwrap_or("").trim();
            println!("  {name:<20} {first_line}");
        }
        _ => {
            println!("  {name:<20} MISSING");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;

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
        let result = check_connectivity("127.0.0.1", addr.port(), Duration::from_secs(1));
        let _ = handle.join();
        result
    }
}
