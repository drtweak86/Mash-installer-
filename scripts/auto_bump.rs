use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_workspace_members(content: &str) -> Vec<String> {
    let workspace_start = match content.find("[workspace]") {
        Some(p) => p,
        None => return Vec::new(),
    };
    let after = &content[workspace_start..];
    let members_start = match after.find("members") {
        Some(p) => p,
        None => return Vec::new(),
    };
    let after_members = &after[members_start..];
    let open = match after_members.find('[') {
        Some(p) => p,
        None => return Vec::new(),
    };
    let close = match after_members.find(']') {
        Some(p) => p,
        None => return Vec::new(),
    };
    after_members[open + 1..close]
        .split(',')
        .map(|s| s.trim().trim_matches('"').trim().to_string())
        .filter(|s| !s.is_empty() && Path::new(s).is_dir())
        .collect()
}

fn get_current_version(content: &str) -> Option<String> {
    // Find the first `version = "X.Y.Z"` that looks like a semver
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("version") && line.contains('"') {
            if let Some(start) = line.find('"') {
                let rest = &line[start + 1..];
                if let Some(end) = rest.find('"') {
                    let ver = &rest[..end];
                    if ver.split('.').count() == 3
                        && ver.chars().all(|c| c.is_ascii_digit() || c == '.')
                    {
                        return Some(ver.to_string());
                    }
                }
            }
        }
    }
    None
}

fn bump_version(current: &str, bump_type: &str) -> Result<String, String> {
    let parts: Result<Vec<u32>, _> = current
        .split('.')
        .map(|p| p.parse::<u32>())
        .collect();
    let mut parts = parts.map_err(|e| format!("Invalid version '{}': {}", current, e))?;
    if parts.len() != 3 {
        return Err(format!("Expected X.Y.Z, got: {}", current));
    }
    match bump_type {
        "major" => {
            parts[0] += 1;
            parts[1] = 0;
            parts[2] = 0;
        }
        "minor" => {
            parts[1] += 1;
            parts[2] = 0;
        }
        "patch" => {
            parts[2] += 1;
        }
        other => return Err(format!("Unknown bump type: {}. Use patch, minor, or major.", other)),
    }
    Ok(format!("{}.{}.{}", parts[0], parts[1], parts[2]))
}

fn update_file(path: &Path, old: &str, new: &str) -> Result<(), std::io::Error> {
    let content = fs::read_to_string(path)?;
    // Replace `version = "OLD"` entries (Cargo.toml style)
    let updated = content.replace(
        &format!("version = \"{}\"", old),
        &format!("version = \"{}\"", new),
    );
    // Also replace bare version strings in doc/UI files
    let updated = updated.replace(old, new);
    if updated != content {
        println!("  Updating {}: {} -> {}", path.display(), old, new);
        fs::write(path, updated)?;
    } else {
        println!("  No change: {}", path.display());
    }
    Ok(())
}

fn run_command(cmd: &str, args: &[&str]) -> Result<(), String> {
    println!("Running: {} {}", cmd, args.join(" "));
    let status = Command::new(cmd)
        .args(args)
        .status()
        .map_err(|e| format!("Failed to spawn '{}': {}", cmd, e))?;
    if !status.success() {
        return Err(format!(
            "Command `{} {}` failed with exit code {:?}",
            cmd,
            args.join(" "),
            status.code()
        ));
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || !["patch", "minor", "major"].contains(&args[1].as_str()) {
        eprintln!("Usage: auto_bump <patch|minor|major>");
        std::process::exit(1);
    }
    let bump_type = &args[1];

    let root = env::current_dir()?;
    let root_cargo = root.join("Cargo.toml");
    let root_content = fs::read_to_string(&root_cargo)?;

    let current = get_current_version(&root_content)
        .ok_or("Version not found in root Cargo.toml")?;
    let new_version = bump_version(&current, bump_type)?;

    println!("Bumping version: {} -> {} ({})", current, new_version, bump_type);

    // Collect all Cargo.toml files from workspace members
    let members = get_workspace_members(&root_content);
    let mut files: Vec<PathBuf> = vec![root_cargo];
    for member in &members {
        let member_cargo = root.join(member).join("Cargo.toml");
        if member_cargo.exists() {
            files.push(member_cargo);
        }
    }

    // Add doc and UI files that embed the version string
    for extra in &[
        "docs/MANUAL.md",
        "installer-cli/src/tui/render.rs",
        "installer-cli/src/tui/menus.rs",
        "docs/HISTORY.md",
    ] {
        let p = root.join(extra);
        if p.exists() {
            files.push(p);
        }
    }

    for file in &files {
        update_file(file, &current, &new_version)?;
    }

    println!("\nRunning post-bump validation...");
    run_command("cargo", &["update"])?;
    run_command("cargo", &["build", "--workspace"])?;
    run_command("cargo", &["test", "--workspace"])?;

    println!("\nVersion bumped to {}.", new_version);
    println!("Next: update HISTORY.md with a bardic release entry, then commit and tag.");
    Ok(())
}
