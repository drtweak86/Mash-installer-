use std::fs;
use std::path::Path;

struct Results {
    pass: usize,
    fail: usize,
}

impl Results {
    fn new() -> Self {
        Self { pass: 0, fail: 0 }
    }

    fn check_file(&mut self, path: &str) -> bool {
        let exists = Path::new(path).exists();
        if exists {
            println!("  PASS  {}", path);
            self.pass += 1;
        } else {
            println!("  MISS  {}", path);
            self.fail += 1;
        }
        exists
    }

    fn check_contains(&mut self, file: &str, needle: &str) -> bool {
        match fs::read_to_string(file) {
            Ok(content) => {
                let found = content.contains(needle);
                if found {
                    println!("    PASS  '{}' in {}", needle, file);
                    self.pass += 1;
                } else {
                    println!("    MISS  '{}' in {}", needle, file);
                    self.fail += 1;
                }
                found
            }
            Err(_) => {
                println!("    MISS  '{}' (file not readable: {})", needle, file);
                self.fail += 1;
                false
            }
        }
    }
}

fn main() {
    println!("Bard's Theme Testing Tavern");
    println!("===========================\n");

    let mut r = Results::new();

    // Test 1: Theme resource files
    println!("Test 1: Theme File Verification");
    println!("--------------------------------");
    for file in &[
        "resources/themes/retro-bbc/i3-config",
        "resources/themes/retro-bbc/i3status-retro.conf",
        "resources/themes/retro-bbc/kitty.conf",
        "resources/themes/retro-bbc/conkyrc",
    ] {
        r.check_file(file);
    }

    // Test 2: Theme module structure
    println!("\nTest 2: Theme Module Structure");
    println!("--------------------------------");
    r.check_file("installer-core/src/theme.rs");
    r.check_contains("installer-core/src/theme.rs", "install_retro_theme");
    r.check_contains(
        "installer-core/src/theme.rs",
        "ensure_retro_theme_dependencies",
    );

    // Test 3: Menu integration
    println!("\nTest 3: Menu Integration");
    println!("------------------------");
    r.check_file("installer-cli/src/menu.rs");
    r.check_contains("installer-cli/src/menu.rs", "run_theme_menu");
    r.check_contains("installer-cli/src/menu.rs", "ThemePlan::RetroOnly");

    // Test 4: Software tiers
    println!("\nTest 4: Software Tiers Integration");
    println!("------------------------------------");
    r.check_file("installer-core/src/software_tiers.rs");
    r.check_contains("installer-core/src/software_tiers.rs", "pub enum ThemePlan");
    r.check_contains(
        "installer-core/src/software_tiers.rs",
        "theme_plan: ThemePlan",
    );

    // Test 5: Library exports
    println!("\nTest 5: Library Exports");
    println!("-----------------------");
    r.check_file("installer-core/src/lib.rs");
    r.check_contains("installer-core/src/lib.rs", "pub use theme::");

    // Test 6: Integration test file
    println!("\nTest 6: Integration Tests");
    println!("--------------------------");
    r.check_file("installer-core/tests/theme_integration.rs");

    // Summary
    println!("\nTest Summary");
    println!("============");
    println!("  PASS: {}  MISS: {}", r.pass, r.fail);
    if r.fail == 0 {
        println!("All checks passed. Ready for Rust compilation and testing.");
    } else {
        println!(
            "{} check(s) failed. See MISS entries above — these features may not be implemented yet.",
            r.fail
        );
        std::process::exit(1);
    }
}
