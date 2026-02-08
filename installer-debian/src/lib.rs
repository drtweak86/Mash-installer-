use anyhow::{bail, Result};
use installer_core::{AptRepoConfig, DistroDriver, PkgBackend, PlatformInfo, RepoKind};

pub struct DebianDriver;

impl DistroDriver for DebianDriver {
    fn name(&self) -> &'static str {
        "Debian/Ubuntu"
    }

    fn description(&self) -> &'static str {
        "Debian-family with apt backend"
    }

    fn matches(&self, info: &PlatformInfo) -> bool {
        info.distro_family == "debian"
    }

    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Apt
    }

    fn apt_repo_config(&self, repo: RepoKind) -> Option<AptRepoConfig> {
        match repo {
            RepoKind::Docker => Some(AptRepoConfig {
                label: "Docker",
                key_path: "/etc/apt/keyrings/docker.asc",
                key_url: docker_key_url,
                sources_path: "/etc/apt/sources.list.d/docker.list",
                repo_line: docker_repo_line,
            }),
            RepoKind::GitHubCli => Some(AptRepoConfig {
                label: "GitHub CLI",
                key_path: "/etc/apt/keyrings/githubcli-archive-keyring.gpg",
                key_url: github_key_url,
                sources_path: "/etc/apt/sources.list.d/github-cli-stable.list",
                repo_line: github_repo_line,
            }),
        }
    }
}

static DEBIAN_DRIVER: DebianDriver = DebianDriver;

pub fn driver() -> &'static dyn DistroDriver {
    &DEBIAN_DRIVER
}

fn docker_repo_os(info: &PlatformInfo) -> Result<&'static str> {
    let distro = info.distro.to_lowercase();
    match distro.as_str() {
        "ubuntu" | "linuxmint" | "pop" | "elementary" | "zorin" => Ok("ubuntu"),
        _ if info.distro_family == "debian" => Ok("debian"),
        other => bail!("Unsupported distro '{}' for Docker repo setup", other),
    }
}

fn require_codename(info: &PlatformInfo) -> Result<&str> {
    let codename = info.distro_codename.as_str();
    if codename.is_empty() {
        bail!("Unable to determine distro codename for Debian/Ubuntu");
    }
    Ok(codename)
}

fn dpkg_arch(info: &PlatformInfo) -> Result<String> {
    let arch = match info.arch.as_str() {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        other => other,
    };
    Ok(arch.to_string())
}

fn docker_key_url(info: &PlatformInfo) -> Result<String> {
    let os = docker_repo_os(info)?;
    Ok(format!("https://download.docker.com/linux/{os}/gpg"))
}

fn docker_repo_line(info: &PlatformInfo) -> Result<String> {
    let arch = dpkg_arch(info)?;
    let codename = require_codename(info)?;
    let os = docker_repo_os(info)?;
    Ok(format!(
        "deb [arch={arch} signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/{os} {codename} stable"
    ))
}

fn github_key_url(_: &PlatformInfo) -> Result<String> {
    Ok("https://cli.github.com/packages/githubcli-archive-keyring.gpg".to_string())
}

fn github_repo_line(info: &PlatformInfo) -> Result<String> {
    let arch = dpkg_arch(info)?;
    Ok(format!(
        "deb [arch={arch} signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main"
    ))
}
