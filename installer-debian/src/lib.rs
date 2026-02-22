use anyhow::{bail, Result};
use installer_core::{cmd, AptRepoConfig, DistroDriver, PkgBackend, PlatformInfo, RepoKind};
use std::process::Command;

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

    fn translate_package(&self, canonical: &str) -> Option<String> {
        match canonical {
            // Debian 13 (Trixie) and newer may not need/have software-properties-common
            // It's often for add-apt-repository which is less common on pure Debian
            "software-properties-common" => {
                // Return None to skip it on Debian if it's failing,
                // or just keep it as is if we want to try.
                // The log showed it failing to locate.
                None
            }
            "fd-find" => Some("fd-find".to_string()), // Keep as is, but some distros use 'fd'
            _ => Some(canonical.to_string()),
        }
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

    fn is_package_installed(&self, package_name: &str) -> bool {
        let native = match self.translate_package(package_name) {
            Some(name) => name,
            None => return false,
        };
        let mut cmd = Command::new("dpkg-query");
        cmd.args(["-W", "-f=${Status}", native.as_str()]);
        match cmd::run(&mut cmd) {
            Ok(output) => String::from_utf8_lossy(&output.stdout).contains("install ok installed"),
            Err(_) => false,
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

#[cfg(test)]
mod tests {
    use super::*;
    use installer_core::{PlatformInfo, RepoKind};

    fn sample_platform() -> PlatformInfo {
        PlatformInfo {
            arch: "aarch64".to_string(),
            distro: "ubuntu".to_string(),
            distro_version: "24.04".to_string(),
            distro_codename: "lunar".to_string(),
            distro_family: "debian".to_string(),
            pi_model: None,
        }
    }

    #[test]
    fn docker_repo_config_is_available() {
        let info = sample_platform();
        let config = driver()
            .apt_repo_config(RepoKind::Docker)
            .expect("docker config missing");
        assert_eq!(config.key_path, "/etc/apt/keyrings/docker.asc");
        let line = (config.repo_line)(&info).expect("repo line failed");
        assert!(line.contains("download.docker.com"));
    }

    #[test]
    fn github_repo_config_headers_match() {
        let info = sample_platform();
        let config = driver()
            .apt_repo_config(RepoKind::GitHubCli)
            .expect("github config missing");
        assert_eq!(
            config.key_path,
            "/etc/apt/keyrings/githubcli-archive-keyring.gpg"
        );
        let line = (config.repo_line)(&info).expect("repo line failed");
        assert!(line.contains("cli.github.com"));
    }
}
