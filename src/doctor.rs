use anyhow::Result;
use std::process::Command;

/// Run diagnostics and print a summary of what is installed / missing.
pub fn run_doctor() -> Result<()> {
    println!("mash-setup doctor");
    println!("==================");
    println!();

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

fn section(name: &str) {
    println!("── {name} ──");
}

fn show_cmd(label: &str, cmd: &str, args: &[&str]) {
    match Command::new(cmd).args(args).output() {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout);
            println!("  {label:<16} {}", out.trim());
        }
        Err(_) => println!("  {label:<16} (not available)"),
    }
}

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
