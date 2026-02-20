use crate::{
    driver::DistroDriver, package_manager, PackageIntent, PackageSpec, PhaseContext, ProfileLevel,
};
use anyhow::Result;

// ── Phase 1: core packages ─────────────────────────────────────

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    if ctx.options.dry_run {
        ctx.record_dry_run(
            "system_packages",
            "Would refresh package database",
            Some(format!("Driver: {}", ctx.platform.driver.name())),
        );
    }
    package_manager::update(ctx.platform.driver, ctx.options.dry_run)?;

    let specs = system_package_specs();
    let applicable_specs: Vec<_> = specs
        .iter()
        .filter(|spec| spec.is_applicable(ctx.options.profile))
        .collect();

    let required_specs: Vec<_> = applicable_specs
        .iter()
        .cloned()
        .filter(|spec| spec.intent() == PackageIntent::Required)
        .collect();

    let missing_required = missing_packages(
        ctx.platform.driver,
        required_specs.iter().map(|spec| spec.canonical()),
    );
    if missing_required.is_empty() {
        tracing::info!("System packages already installed");
    } else {
        if ctx.options.dry_run {
            ctx.record_dry_run(
                "system_packages",
                "Would install required packages",
                Some(format!("Missing: {}", missing_required.join(", "))),
            );
        }
        package_manager::ensure_packages(
            ctx.platform.driver,
            &missing_required,
            ctx.options.dry_run,
        )?;
    }

    for spec in applicable_specs
        .iter()
        .filter(|spec| spec.intent() == PackageIntent::Optional)
    {
        if ctx.options.dry_run {
            ctx.record_dry_run(
                "system_packages",
                "Would attempt optional package",
                Some(spec.canonical().to_string()),
            );
        }
        package_manager::try_optional(ctx.platform.driver, spec.canonical(), ctx.options.dry_run);
    }

    Ok(())
}

fn system_package_specs() -> Vec<PackageSpec<'static>> {
    let mut specs = vec![
        PackageSpec::required("ca-certificates"),
        PackageSpec::required("curl"),
        PackageSpec::required("wget"),
        PackageSpec::required("xz-utils"),
        PackageSpec::required("tar"),
        PackageSpec::required("coreutils"),
        PackageSpec::required("jq"),
        PackageSpec::required("git"),
        PackageSpec::required("software-properties-common"),
        PackageSpec::required("gnupg"),
        PackageSpec::required("lsb-release"),
        PackageSpec::required("apt-transport-https"),
        PackageSpec::required("build-essential"),
        PackageSpec::required("pkg-config"),
        PackageSpec::required("clang"),
        PackageSpec::required("lld"),
        PackageSpec::required("cmake"),
        PackageSpec::required("ninja-build"),
        PackageSpec::required("gcc"),
        PackageSpec::required("g++"),
        PackageSpec::required("gdb"),
        PackageSpec::required("make"),
    ];

    specs.extend_from_slice(&[
        PackageSpec::required_for("python3", ProfileLevel::Dev),
        PackageSpec::required_for("python3-pip", ProfileLevel::Dev),
        PackageSpec::required_for("python3-venv", ProfileLevel::Dev),
        PackageSpec::required_for("ripgrep", ProfileLevel::Dev),
        PackageSpec::required_for("fd-find", ProfileLevel::Dev),
        PackageSpec::required_for("fzf", ProfileLevel::Dev),
        PackageSpec::required_for("tmux", ProfileLevel::Dev),
        PackageSpec::required_for("htop", ProfileLevel::Dev),
        PackageSpec::required_for("ncdu", ProfileLevel::Dev),
        PackageSpec::required_for("neovim", ProfileLevel::Dev),
        PackageSpec::required_for("kitty", ProfileLevel::Dev),
    ]);

    specs.extend_from_slice(&[
        PackageSpec::required_for("nodejs", ProfileLevel::Full),
        PackageSpec::required_for("npm", ProfileLevel::Full),
    ]);

    specs.extend_from_slice(&[
        PackageSpec::optional("lldb"),
        PackageSpec::optional("btop"),
        PackageSpec::optional("bat"),
        PackageSpec::optional("eza"),
        PackageSpec::optional("yq"),
    ]);

    specs
}

fn missing_packages<'a, I>(driver: &dyn DistroDriver, packages: I) -> Vec<&'a str>
where
    I: IntoIterator<Item = &'a str>,
{
    packages
        .into_iter()
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
        assert_eq!(
            super::missing_packages(&driver, pkgs.iter().copied()),
            vec!["tar"]
        );
    }
}
