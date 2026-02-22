use std::env;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", secs)
}

fn run_cargo_test(log_path: &str) -> bool {
    println!("Running cargo test -p installer-core...");
    let output = Command::new("cargo")
        .args(["test", "-p", "installer-core"])
        .env("CARGO_HOME", "/tmp/cargo")
        .env("RUST_TEST_THREADS", "1")
        .output();

    match output {
        Ok(out) => {
            let combined = format!(
                "{}\n{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
            let _ = fs::write(log_path, &combined);
            print!("{}", combined);
            out.status.success()
        }
        Err(e) => {
            eprintln!("Failed to run cargo test: {}", e);
            false
        }
    }
}

fn run_hardware_tests(log_path: &str) -> bool {
    println!("Running hardware/kernel-dependent tests...");
    println!("  * Ensure target hardware or QEMU aarch64 VM is provisioned.");
    let output = Command::new("cargo")
        .args(["test", "-p", "installer-core", "--", "--ignored"])
        .env("CARGO_HOME", "/tmp/cargo")
        .env("RUST_TEST_THREADS", "1")
        .output();

    match output {
        Ok(out) => {
            let combined = format!(
                "{}\n{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
            let _ = fs::write(log_path, &combined);
            print!("{}", combined);
            out.status.success()
        }
        Err(e) => {
            eprintln!("Failed to run cargo test --ignored: {}", e);
            false
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("maelstrom");

    fs::create_dir_all(".logs")?;
    let log_path = format!(".logs/test-{}-{}.log", mode, timestamp());

    let success = match mode {
        "maelstrom" => {
            let has_maelstrom = Command::new("which")
                .arg("maelstrom")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            if has_maelstrom {
                println!("Running maelstrom-compatible tests...");
                let output = Command::new("maelstrom")
                    .args(["test", "-p", "installer-core"])
                    .output()?;
                let combined = format!(
                    "{}\n{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                fs::write(&log_path, &combined)?;
                print!("{}", combined);
                output.status.success()
            } else {
                println!("maelstrom not found; falling back to cargo test");
                run_cargo_test(&log_path)
            }
        }
        "hardware" => run_hardware_tests(&log_path),
        other => {
            eprintln!(
                "Unknown mode '{}'. Supported modes: maelstrom, hardware.",
                other
            );
            std::process::exit(1);
        }
    };

    if success {
        println!("\nTests passed. Log: {}", log_path);
    } else {
        eprintln!("\nTests failed. Log: {}", log_path);
        std::process::exit(1);
    }
    Ok(())
}
