use crate::{driver::DistroDriver, package_manager, PhaseExecutionContext};
use anyhow::Result;

// ── Phase 1: core packages ─────────────────────────────────────

pub fn install_phase(ctx: &PhaseExecutionContext) -> Result<()> {
    package_manager::update(ctx.platform.driver, ctx.options.dry_run)?;

    // Always-needed core packages (Debian canonical names)
    let mut pkgs: Vec<&str> = vec![
        "ca-certificates",
        "curl",
        "wget",
        "xz-utils",
        "tar",
        "coreutils",
        "jq",
        "git",
        "software-properties-common",
        "gnupg",
        "lsb-release",
        "apt-transport-https",
    ];

    // Build essentials (all profiles)
    pkgs.extend_from_slice(&[
        "build-essential",
        "pkg-config",
        "clang",
        "lld",
        "cmake",
        "ninja-build",
        "gcc",
        "g++",
        "gdb",
        "make",
    ]);

    // Dev+ packages
    if ctx.options.profile >= crate::ProfileLevel::Dev {
        pkgs.extend_from_slice(&[
            "python3",
            "python3-pip",
            "python3-venv",
            "ripgrep",
            "fd-find",
            "fzf",
            "tmux",
            "htop",
            "ncdu",
            "neovim",
        ]);
    }

    // Full profile extras
    if ctx.options.profile >= crate::ProfileLevel::Full {
        pkgs.extend_from_slice(&["nodejs", "npm"]);
    }

    // Optional packages – may not exist in every distro version
    let optional = ["btop", "eza", "yq", "lldb", "bat"];

    // Split required vs optional
    let required: Vec<&str> = pkgs
        .iter()
        .copied()
        .filter(|p| !optional.contains(p))
        .collect();

    let missing_required = missing_packages(ctx.platform.driver, &required);
    if missing_required.is_empty() {
        tracing::info!("System packages already installed");
    } else {
        package_manager::ensure_packages(
            ctx.platform.driver,
            &missing_required,
            ctx.options.dry_run,
        )?;
    }

    // Always attempt lldb
    package_manager::try_optional(ctx.platform.driver, "lldb", ctx.options.dry_run);

    // Dev+ optional packages
    if ctx.options.profile >= crate::ProfileLevel::Dev {
        for pkg in &["btop", "bat", "eza", "yq"] {
            package_manager::try_optional(ctx.platform.driver, pkg, ctx.options.dry_run);
        }
    }

    Ok(())
}

fn missing_packages<'a>(driver: &dyn DistroDriver, packages: &'a [&'a str]) -> Vec<&'a str> {
    packages
        .iter()
        .copied()
        .filter(|pkg| !driver.is_package_installed(pkg))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        distro,
        driver::{RepoKind, ServiceName},
        DistroDriver, PkgBackend,
    };

    struct TestDriver;

    impl DistroDriver for TestDriver {
        fn name(&self) -> &'static str {
            "test"
        }

        fn description(&self) -> &'static str {
            "test driver"
        }

        fn matches(&self, _: &crate::platform::PlatformInfo) -> bool {
            true
        }

        fn pkg_backend(&self) -> PkgBackend {
            PkgBackend::Apt
        }

        fn translate_package(&self, canonical: &str) -> Option<String> {
            match canonical {
                "foo" => Some("foo-native".to_string()),
                "drop" => None,
                _ => Some(canonical.to_string()),
            }
        }

        fn apt_repo_config(&self, _repo: RepoKind) -> Option<crate::driver::AptRepoConfig> {
            None
        }

        fn service_unit(&self, _service: ServiceName) -> &'static str {
            "test.service"
        }
    }

    #[test]
    fn translate_names_respects_driver() {
        let driver = TestDriver;
        let pkgs = ["foo", "bar", "drop"];
        let names = distro::translate_names(&driver, &pkgs);
        assert_eq!(names, vec!["foo-native".to_string(), "bar".to_string()]);
    }

    struct InstalledDriver;

    impl DistroDriver for InstalledDriver {
        fn name(&self) -> &'static str {
            "installed"
        }

        fn description(&self) -> &'static str {
            "reports installed packages"
        }

        fn matches(&self, _: &crate::platform::PlatformInfo) -> bool {
            true
        }

        fn pkg_backend(&self) -> PkgBackend {
            PkgBackend::Apt
        }

        fn translate_package(&self, canonical: &str) -> Option<String> {
            Some(canonical.to_string())
        }

        fn apt_repo_config(&self, _repo: RepoKind) -> Option<crate::driver::AptRepoConfig> {
            None
        }

        fn service_unit(&self, _service: ServiceName) -> &'static str {
            "installed.service"
        }

        fn is_package_installed(&self, package_name: &str) -> bool {
            matches!(package_name, "curl" | "git")
        }
    }

    #[test]
    fn missing_packages_filters_installed_items() {
        let driver = InstalledDriver;
        let pkgs = ["curl", "git", "tar"];
        assert_eq!(super::missing_packages(&driver, &pkgs), vec!["tar"]);
    }
}
