use std::fs;
use std::path::{Path, PathBuf};

fn is_external(link: &str) -> bool {
    link.starts_with("http://")
        || link.starts_with("https://")
        || link.starts_with("mailto:")
        || link.starts_with("ftp://")
        || link.starts_with("javascript:")
}

fn extract_links(content: &str) -> Vec<String> {
    let mut links = Vec::new();
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
                links.push(content[start..i].trim().to_string());
            }
        }
        i += 1;
    }
    links
}

fn collect_md_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_md_files(&path, files);
            } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                files.push(path);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::current_dir()?;
    let docs_dir = root.join("docs");

    if !docs_dir.exists() {
        eprintln!("docs/ directory not found at {}", docs_dir.display());
        std::process::exit(1);
    }

    let mut md_files = Vec::new();
    collect_md_files(&docs_dir, &mut md_files);

    let mut missing: Vec<(PathBuf, String)> = Vec::new();

    for md_path in &md_files {
        let content = match fs::read_to_string(md_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for raw_link in extract_links(&content) {
            let link = raw_link.trim();
            if link.is_empty() || link.starts_with('#') || is_external(link) {
                continue;
            }
            let link_path = link.split('#').next().unwrap_or("").trim();
            if link_path.is_empty() {
                continue;
            }
            let target = md_path.parent().unwrap().join(link_path);
            if !target.exists() {
                let rel = md_path.strip_prefix(&root).unwrap_or(md_path);
                missing.push((rel.to_path_buf(), link.to_string()));
            }
        }
    }

    if missing.is_empty() {
        println!("Documentation link check passed.");
        Ok(())
    } else {
        println!("Broken documentation references detected:");
        for (source, target) in &missing {
            println!("  {} -> {}", source.display(), target);
        }
        std::process::exit(1);
    }
}
