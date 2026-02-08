use crate::{package_manager, InstallContext};
use anyhow::Result;

// ── Phase 1: core packages ─────────────────────────────────────

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    package_manager::update(ctx.driver, ctx.dry_run)?;

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
    if ctx.profile >= crate::ProfileLevel::Dev {
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
    if ctx.profile >= crate::ProfileLevel::Full {
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

    package_manager::ensure_packages(ctx.driver, &required, ctx.dry_run)?;

    // Always attempt lldb
    package_manager::try_optional(ctx.driver, "lldb", ctx.dry_run);

    // Dev+ optional packages
    if ctx.profile >= crate::ProfileLevel::Dev {
        for pkg in &["btop", "bat", "eza", "yq"] {
            package_manager::try_optional(ctx.driver, pkg, ctx.dry_run);
        }
    }

    Ok(())
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
}
