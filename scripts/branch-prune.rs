use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

fn get_branch_age_days(branch: &str) -> u64 {
    let output = Command::new("git")
        .args(["log", branch, "-1", "--format=%ct"])
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            let timestamp_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if let Ok(last_commit_timestamp) = timestamp_str.parse::<u64>() {
                let current_timestamp = get_current_timestamp();
                if current_timestamp > last_commit_timestamp {
                    return (current_timestamp - last_commit_timestamp) / 86400;
                }
            }
        }
    }
    0
}

fn prune_old_branches() {
    println!("🗑️ Pruning branches older than 7 days...");

    let output = Command::new("git")
        .args(["branch", "--format=%(refname:short)"])
        .output()
        .expect("Failed to execute git branch");

    let branches_str = String::from_utf8_lossy(&output.stdout);
    let branches: Vec<&str> = branches_str
        .lines()
        .map(|l| l.trim())
        .filter(|&l| !l.is_empty() && l != "main" && l != "forge" && !l.starts_with('*'))
        .collect();

    if branches.is_empty() {
        println!("  ✅ No branches to prune (only main and forge exist)");
        return;
    }

    println!("  Found branches to check:");
    for branch in &branches {
        let age = get_branch_age_days(branch);
        if age > 7 {
            println!("    🗑️  Pruning {} (age: {} days)", branch, age);
            let _ = Command::new("git").args(["branch", "-D", branch]).status();
        } else {
            println!("    🔄  Keeping {} (age: {} days)", branch, age);
        }
    }
    println!("✅ Branch pruning complete");
}

fn check_remote_branches() {
    println!("🌐 Checking remote branches...");

    let output = Command::new("git")
        .args(["branch", "-r", "--format=%(refname:short)"])
        .output()
        .expect("Failed to execute git branch -r");

    let branches_str = String::from_utf8_lossy(&output.stdout);
    let branches: Vec<&str> = branches_str
        .lines()
        .map(|l| l.trim())
        .filter(|&l| !l.is_empty() && l != "origin/main" && l != "origin/forge" && !l.contains("HEAD"))
        .collect();

    if branches.is_empty() {
        println!("  ✅ No remote branches to check (only main and forge exist)");
        return;
    }

    println!("  Found remote branches:");
    for branch in branches {
        let age = get_branch_age_days(branch);
        println!("    - {} (age: {} days)", branch, age);
    }
    println!("✅ Remote branch check complete");
}

fn show_branch_status() {
    println!("🌿 Current Branch Status:");
    println!("");

    println!("  Local branches:");
    let _ = Command::new("git")
        .args(["branch", "--format=    %(refname:short) %(committerdate:short)"])
        .status();

    println!("
  Remote branches:");
    let _ = Command::new("git")
        .args(["branch", "-r", "--format=    %(refname:short) %(committerdate:short)"])
        .status();
    println!("");
}

fn show_policy() {
    println!("📜 Branch Policy:");
    println!("");
    println!("  ✅ Protected branches: main, forge");
    println!("  🗑️  Prune branches: older than 7 days");
    println!("  💡 Recommendation: Use feature branches for work");
    println!("  📝 Documentation: docs/forge-tavern/bard-quick-ref.md");
    println!("");
}

fn main() {
    println!("🌱 Starting Branch Pruning...");
    prune_old_branches();
    check_remote_branches();
    show_branch_status();
    show_policy();
    println!("🎉 Branch Pruning Complete!");
}
