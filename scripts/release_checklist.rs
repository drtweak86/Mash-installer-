use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn banner(title: &str) {
    println!("\n==== RELEASE CHECKLIST: {} ====", title);
}

// Inlined doc-link checker (mirrors check_docs.rs logic)
fn check_docs(root: &Path) -> bool {
    fn is_external(link: &str) -> bool {
        link.starts_with("http://")
            || link.starts_with("https://")
            || link.starts_with("mailto:")
            || link.starts_with("ftp://")
    }

    fn collect_md(dir: &Path, out: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    collect_md(&p, out);
                } else if p.extension().map(|x| x == "md").unwrap_or(false) {
                    out.push(p);
                }
            }
        }
    }

    let docs_dir = root.join("docs");
    let mut md_files = Vec::new();
    collect_md(&docs_dir, &mut md_files);

    let mut missing: Vec<(PathBuf, String)> = Vec::new();
    for md_path in &md_files {
        let content = match fs::read_to_string(md_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let bytes = content.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b']' && i + 1 < bytes.len() && bytes[i + 1] == b'(' {
                i += 2;
                let start = i;
                while i < bytes.len() && bytes[i] != b')' && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() && bytes[i] == b')' {
                    let link = content[start..i].trim();
                    if !link.is_empty() && !link.starts_with('#') && !is_external(link) {
                        let lp = link.split('#').next().unwrap_or("").trim();
                        if !lp.is_empty() {
                            let target = md_path.parent().unwrap().join(lp);
                            if !target.exists() {
                                let rel = md_path.strip_prefix(root).unwrap_or(md_path);
                                missing.push((rel.to_path_buf(), link.to_string()));
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }

    if missing.is_empty() {
        println!("  Documentation link check passed.");
        true
    } else {
        println!("  Broken documentation references:");
        for (src, tgt) in &missing {
            println!("    {} -> {}", src.display(), tgt);
        }
        false
    }
}

fn run(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::current_dir()?;

    banner("Ensure repository is clean");
    if !run("git", &["status", "-sb"]) {
        eprintln!("git status failed");
        std::process::exit(1);
    }

    banner("Verify documentation integrity");
    if !check_docs(&root) {
        eprintln!("Documentation check failed — fix broken links before release.");
        std::process::exit(1);
    }

    banner("cargo fmt --check");
    if !run("cargo", &["fmt", "--all", "--", "--check"]) {
        eprintln!("cargo fmt check failed — run `cargo fmt --all` first.");
        std::process::exit(1);
    }

    banner("cargo clippy");
    if !run(
        "cargo",
        &[
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    ) {
        eprintln!("clippy found warnings — fix them before release.");
        std::process::exit(1);
    }

    banner("cargo test --workspace");
    if !run("cargo", &["test", "--workspace"]) {
        eprintln!("Tests failed — all tests must pass before release.");
        std::process::exit(1);
    }

    banner("Manual release reminders");
    println!("  - Update docs (ARCH.md, modules.md) and ensure all links are fresh.");
    println!("  - Update HISTORY.md with a bardic release entry.");
    println!("  - Bump version in all Cargo.toml files (use scripts/auto_bump.rs).");
    println!("  - Run dry-run: `mash-setup --dry-run` or `mash-setup doctor`.");
    println!("  - Tag the release: git tag v<VERSION> && git push --tags");
    println!("  - Deploy documentation.");

    println!("\nRelease checklist complete. The forge is ready to ship.");
    Ok(())
}
