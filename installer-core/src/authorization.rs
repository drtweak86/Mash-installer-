use anyhow::{anyhow, Context, Result};
use std::process::Command;
use tracing::{info, warn};

use crate::{context::UserOptionsContext, phase_runner::PhaseObserver};
use mash_system::cmd;

/// Types of interactive authorizations supported by the forge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthType {
    /// GitHub CLI login (`gh auth login`)
    GitHubCli,
    /// SSH key generation (`ssh-keygen`)
    SshKey,
    /// Global Git configuration (`git config --global`)
    GitConfig,
    /// Rclone configuration (`rclone config`)
    RcloneConfig,
    /// Borg repository initialization (`borg init`)
    BorgSetup,
    /// Tailscale node authorization (`tailscale up`)
    TailscaleAuth,
    /// Ngrok authtoken configuration
    NgrokAuth,
    /// Cloudflared tunnel login
    CloudflaredAuth,
    /// Docker registry login
    DockerAuth,
    /// Argon One fan curve configuration
    ArgonOneConfig,
}

/// Service for handling interactive authorizations.
pub struct AuthorizationService<'a> {
    observer: &'a mut dyn PhaseObserver,
    options: &'a UserOptionsContext,
}

impl<'a> AuthorizationService<'a> {
    pub fn new(observer: &'a mut dyn PhaseObserver, options: &'a UserOptionsContext) -> Self {
        Self { observer, options }
    }

    /// Check if a specific authorization is already configured.
    pub fn is_authorized(&self, auth_type: AuthType) -> bool {
        match auth_type {
            AuthType::GitHubCli => self.check_gh_auth(),
            AuthType::SshKey => self.check_ssh_key(),
            AuthType::GitConfig => self.check_git_config(),
            AuthType::RcloneConfig => self.check_rclone_config(),
            AuthType::BorgSetup => self.check_borg_config(),
            AuthType::TailscaleAuth => self.check_tailscale_auth(),
            AuthType::NgrokAuth => self.check_ngrok_auth(),
            AuthType::CloudflaredAuth => self.check_cloudflared_auth(),
            AuthType::DockerAuth => self.check_docker_auth(),
            AuthType::ArgonOneConfig => self.check_argon_config(),
        }
    }

    /// Perform the interactive authorization flow.
    pub fn authorize(&mut self, auth_type: AuthType) -> Result<()> {
        if self.options.dry_run {
            info!(
                "Dry run: Skipping interactive authorization for {:?}",
                auth_type
            );
            return Ok(());
        }

        if !self.options.interactive {
            warn!(
                "Non-interactive mode: Cannot perform interactive authorization for {:?}",
                auth_type
            );
            return Ok(());
        }

        match auth_type {
            AuthType::GitHubCli => self.authorize_gh(),
            AuthType::SshKey => self.authorize_ssh(),
            AuthType::GitConfig => self.authorize_git(),
            AuthType::RcloneConfig => self.authorize_rclone(),
            AuthType::BorgSetup => self.authorize_borg(),
            AuthType::TailscaleAuth => self.authorize_tailscale(),
            AuthType::NgrokAuth => self.authorize_ngrok(),
            AuthType::CloudflaredAuth => self.authorize_cloudflared(),
            AuthType::DockerAuth => self.authorize_docker(),
            AuthType::ArgonOneConfig => self.authorize_argon(),
        }
    }

    fn check_gh_auth(&self) -> bool {
        if which::which("gh").is_err() {
            return false;
        }

        let mut cmd = Command::new("gh");
        cmd.args(["auth", "status", "-h", "github.com"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());

        cmd::run(&mut cmd).is_ok()
    }

    fn check_ssh_key(&self) -> bool {
        let home = match dirs::home_dir() {
            Some(path) => path,
            None => return false,
        };

        let ssh_dir = home.join(".ssh");
        if !ssh_dir.exists() {
            return false;
        }

        // Check for common private key files
        let keys = ["id_rsa", "id_ed25519", "id_ecdsa"];
        keys.iter().any(|key| ssh_dir.join(key).exists())
    }

    fn check_git_config(&self) -> bool {
        let has_user = Command::new("git")
            .args(["config", "--global", "user.name"])
            .output()
            .map(|o| !o.stdout.is_empty())
            .unwrap_or(false);

        let has_email = Command::new("git")
            .args(["config", "--global", "user.email"])
            .output()
            .map(|o| !o.stdout.is_empty())
            .unwrap_or(false);

        has_user && has_email
    }

    fn check_rclone_config(&self) -> bool {
        let home = match dirs::home_dir() {
            Some(path) => path,
            None => return false,
        };

        let config_path = home.join(".config/rclone/rclone.conf");
        config_path.exists()
    }

    fn check_borg_config(&self) -> bool {
        let home = match dirs::home_dir() {
            Some(path) => path,
            None => return false,
        };

        let borgmatic_path = home.join(".config/borgmatic/config.yaml");
        borgmatic_path.exists()
    }

    fn check_tailscale_auth(&self) -> bool {
        if which::which("tailscale").is_err() {
            return false;
        }

        let output = Command::new("tailscale").arg("status").output();

        match output {
            Ok(o) => {
                let status = String::from_utf8_lossy(&o.stdout);
                !status.contains("Logged out") && !status.contains("Tailscale is stopped")
            }
            Err(_) => false,
        }
    }

    fn check_ngrok_auth(&self) -> bool {
        let home = match dirs::home_dir() {
            Some(path) => path,
            None => return false,
        };

        // ngrok v3 config path
        let config_path = home.join(".config/ngrok/ngrok.yml");
        if !config_path.exists() {
            return false;
        }

        let content = std::fs::read_to_string(config_path).unwrap_or_default();
        content.contains("authtoken:")
    }

    fn check_cloudflared_auth(&self) -> bool {
        let home = match dirs::home_dir() {
            Some(path) => path,
            None => return false,
        };

        let cert_path = home.join(".cloudflared/cert.pem");
        cert_path.exists()
    }

    fn check_docker_auth(&self) -> bool {
        let home = match dirs::home_dir() {
            Some(path) => path,
            None => return false,
        };

        let config_path = home.join(".docker/config.json");
        if !config_path.exists() {
            return false;
        }

        let content = std::fs::read_to_string(config_path).unwrap_or_default();
        content.contains("\"auths\": {") && !content.contains("\"auths\": {}")
    }

    fn check_argon_config(&self) -> bool {
        std::path::Path::new("/etc/argononed.conf").exists()
    }

    fn authorize_gh(&mut self) -> Result<()> {
        info!("Launching interactive GitHub CLI authorization...");

        self.observer.confirm(
            "GitHub CLI Authorization required. \
             The forge will now launch 'gh auth login'. \
             Please follow the prompts in your terminal.",
        );

        let mut cmd = Command::new("gh");
        cmd.args(["auth", "login", "-h", "github.com", "-p", "ssh", "-w"]);

        let status = cmd
            .status()
            .map_err(|e| anyhow!("Failed to launch gh auth login: {}", e))?;

        if status.success() {
            info!("GitHub CLI authorization successful.");
            Ok(())
        } else {
            Err(anyhow!("GitHub CLI authorization failed or was cancelled."))
        }
    }

    fn authorize_ssh(&mut self) -> Result<()> {
        info!("Launching interactive SSH key generation...");

        let should_gen = self.observer.confirm(
            "No SSH keys found. Would you like the forge to generate one for you (Ed25519)?",
        );

        if !should_gen {
            info!("SSH key generation skipped by user.");
            return Ok(());
        }

        let mut cmd = Command::new("ssh-keygen");
        cmd.args(["-t", "ed25519"]);

        let status = cmd
            .status()
            .map_err(|e| anyhow!("Failed to launch ssh-keygen: {}", e))?;

        if status.success() {
            info!("SSH key generated successfully.");

            if self.check_gh_auth() {
                let add_to_gh = self
                    .observer
                    .confirm("Would you like to add this new SSH key to your GitHub account?");

                if add_to_gh {
                    let mut gh_cmd = Command::new("gh");
                    gh_cmd.args(["ssh-key", "add", "--title", "MASH Forge Key"]);
                    let gh_status = gh_cmd
                        .status()
                        .map_err(|e| anyhow!("Failed to add SSH key to GitHub: {}", e))?;
                    if gh_status.success() {
                        info!("SSH key added to GitHub successfully.");
                    }
                }
            }
            Ok(())
        } else {
            Err(anyhow!("SSH key generation failed or was cancelled."))
        }
    }

    fn authorize_git(&mut self) -> Result<()> {
        info!("Launching interactive Git configuration...");

        self.observer.confirm(
            "Git global configuration is missing. \
             The forge will now launch interactive git config prompts.",
        );

        warn!("Interactive Git config not fully implemented in TUI yet. Please run: git config --global user.name \"Your Name\" && git config --global user.email \"your@email.com\"");

        Ok(())
    }

    fn authorize_rclone(&mut self) -> Result<()> {
        info!("Launching interactive rclone configuration...");

        self.observer.confirm(
            "rclone is not configured. The forge will now launch 'rclone config'. \
             Please follow the prompts in your terminal to set up your cloud remotes.",
        );

        let mut cmd = Command::new("rclone");
        cmd.arg("config");

        let status = cmd
            .status()
            .map_err(|e| anyhow!("Failed to launch rclone config: {}", e))?;

        if status.success() {
            info!("rclone configuration completed.");
            Ok(())
        } else {
            Err(anyhow!("rclone configuration failed or was cancelled."))
        }
    }

    fn authorize_borg(&mut self) -> Result<()> {
        info!("Launching interactive Borg setup...");

        self.observer.confirm(
            "BorgBackup is installed but no configuration was found. \
             The forge can help you initialize a local backup repository.",
        );

        let home = dirs::home_dir().ok_or_else(|| anyhow!("Unable to find home directory"))?;
        let default_repo = home.join("backups/borg-repo");

        let should_init = self.observer.confirm(&format!(
            "Would you like to initialize a new Borg repository at {}?",
            default_repo.display()
        ));

        if !should_init {
            info!("Borg initialization skipped by user.");
            return Ok(());
        }

        if !default_repo.exists() {
            std::fs::create_dir_all(&default_repo).context("creating borg repository directory")?;
        }

        let mut cmd = Command::new("borg");
        cmd.args([
            "init",
            "--encryption",
            "repokey",
            &default_repo.display().to_string(),
        ]);

        let status = cmd
            .status()
            .map_err(|e| anyhow!("Failed to launch borg init: {}", e))?;

        if status.success() {
            info!(
                "Borg repository initialized successfully at {}.",
                default_repo.display()
            );
            Ok(())
        } else {
            Err(anyhow!("Borg initialization failed or was cancelled."))
        }
    }

    fn authorize_tailscale(&mut self) -> Result<()> {
        info!("Launching interactive Tailscale authorization...");

        self.observer.confirm(
            "Tailscale is installed but not authorized. \
             The forge will now launch 'tailscale up'. \
             Follow the login link provided in the terminal.",
        );

        let mut cmd = Command::new("tailscale");
        cmd.arg("up");

        let status = cmd
            .status()
            .map_err(|e| anyhow!("Failed to launch tailscale up: {}", e))?;

        if status.success() {
            info!("Tailscale authorized successfully.");
            Ok(())
        } else {
            Err(anyhow!("Tailscale authorization failed or was cancelled."))
        }
    }

    fn authorize_ngrok(&mut self) -> Result<()> {
        info!("Launching interactive Ngrok configuration...");

        self.observer.confirm(
            "Ngrok requires an authtoken for many features. \
             The forge will now launch 'ngrok config add-authtoken'. \
             You will need to paste your token from the Ngrok dashboard.",
        );

        let mut cmd = Command::new("ngrok");
        cmd.args(["config", "add-authtoken"]);

        let status = cmd
            .status()
            .map_err(|e| anyhow!("Failed to launch ngrok config: {}", e))?;

        if status.success() {
            info!("Ngrok token configured successfully.");
            Ok(())
        } else {
            Err(anyhow!("Ngrok configuration failed or was cancelled."))
        }
    }

    fn authorize_cloudflared(&mut self) -> Result<()> {
        info!("Launching interactive Cloudflared authorization...");

        self.observer.confirm(
            "Cloudflared (Argo Tunnel) requires a login. \
             The forge will now launch 'cloudflared tunnel login'. \
             Follow the link in your terminal to authorize.",
        );

        let mut cmd = Command::new("cloudflared");
        cmd.args(["tunnel", "login"]);

        let status = cmd
            .status()
            .map_err(|e| anyhow!("Failed to launch cloudflared login: {}", e))?;

        if status.success() {
            info!("Cloudflared authorized successfully.");
            Ok(())
        } else {
            Err(anyhow!(
                "Cloudflared authorization failed or was cancelled."
            ))
        }
    }

    fn authorize_docker(&mut self) -> Result<()> {
        info!("Launching interactive Docker registry login...");

        self.observer
            .confirm("Would you like to log in to a Docker registry (e.g., Docker Hub) now?");

        let mut cmd = Command::new("docker");
        cmd.arg("login");

        let status = cmd
            .status()
            .map_err(|e| anyhow!("Failed to launch docker login: {}", e))?;

        if status.success() {
            info!("Docker login successful.");
            Ok(())
        } else {
            Err(anyhow!("Docker login failed or was cancelled."))
        }
    }

    fn authorize_argon(&mut self) -> Result<()> {
        info!("Launching interactive Argon One fan configuration...");

        self.observer.confirm(
            "Argon One fan script is installed. \
             The forge will now launch the configuration tool for fan thresholds.",
        );

        let mut cmd = Command::new("argonone-config");

        let status = cmd
            .status()
            .map_err(|e| anyhow!("Failed to launch argonone-config: {}", e))?;

        if status.success() {
            info!("Argon One fan configuration completed.");
            Ok(())
        } else {
            Err(anyhow!("Argon One configuration failed or was cancelled."))
        }
    }
}
