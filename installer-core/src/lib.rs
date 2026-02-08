mod apt_repo;
mod argon;
mod buildroot;
mod config;
mod docker;
mod doctor;
mod driver;
mod fonts;
mod github;
mod pkg;
mod platform;
mod rclone;
mod rust;
mod staging;
mod systemd;
mod zsh;

use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tracing::{error, info};

pub use driver::{AptRepoConfig, DistroDriver, RepoKind, ServiceName};
pub use pkg::PkgBackend;
pub use platform::{detect as detect_platform, PlatformInfo};

/// Options provided by the CLI that drive `run_with_driver`.
pub struct InstallOptions {
    pub profile: ProfileLevel,
    pub staging_dir: Option<PathBuf>,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
}

/// Central context threaded through every install phase.
pub struct InstallContext {
    pub profile: ProfileLevel,
    pub staging_dir: PathBuf,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
    pub mp: MultiProgress,
    /// Overall progress bar (% done + ETA).
    pub overall: ProgressBar,
    pub config: config::MashConfig,
    pub platform: platform::PlatformInfo,
    pub driver_name: &'static str,
    pub driver: &'static dyn DistroDriver,
    pub pkg_backend: PkgBackend,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProfileLevel {
    Minimal = 0,
    Dev = 1,
    Full = 2,
}

impl InstallContext {
    /// Create a spinner-style progress bar attached to the MultiProgress.
    pub fn phase_spinner(&self, msg: &str) -> ProgressBar {
        let pb = self
            .mp
            .insert_before(&self.overall, ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
        );
        pb.set_message(msg.to_string());
        pb.enable_steady_tick(std::time::Duration::from_millis(120));
        pb
    }

    /// Finish a spinner with a checkmark and advance the overall bar.
    pub fn finish_phase(&self, pb: &ProgressBar, msg: &str) {
        pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
        pb.set_prefix("✓");
        pb.finish_with_message(msg.to_string());
        self.overall.inc(1);
    }

    /// Finish a spinner indicating it was skipped and advance the overall bar.
    pub fn skip_phase(&self, pb: &ProgressBar, msg: &str) {
        pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
        pb.set_prefix("–");
        pb.finish_with_message(msg.to_string());
        self.overall.inc(1);
    }
}

/// Run the installer using the supplied distro driver and CLI options.
pub fn run_with_driver(driver: &'static dyn DistroDriver, opts: InstallOptions) -> Result<()> {
    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       mash-setup · mega installer            ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    let plat = platform::detect()?;
    info!(
        "Platform: {} {} on {}",
        plat.distro, plat.distro_version, plat.arch
    );
    info!(
        "Using distro driver: {} ({})",
        driver.name(),
        driver.description()
    );
    if let Some(ref model) = plat.pi_model {
        info!("Raspberry Pi model: {}", model);
    }

    let cfg = config::load_or_default()?;
    let staging = staging::resolve(opts.staging_dir.as_deref(), &cfg)?;
    info!("Staging directory: {}", staging.display());

    let profile = opts.profile;
    let enable_argon = opts.enable_argon;
    let enable_p10k = opts.enable_p10k;
    let docker_data_root = opts.docker_data_root;

    let mut phases: Vec<Phase> = vec![
        Phase {
            label: "System packages",
            done_msg: "System packages installed",
            run: pkg::install_phase,
        },
        Phase {
            label: "Rust toolchain + cargo tools",
            done_msg: "Rust toolchain ready",
            run: rust::install_phase,
        },
        Phase {
            label: "Git, GitHub CLI, SSH",
            done_msg: "Git / GitHub CLI ready",
            run: github::install_phase,
        },
    ];

    if profile >= ProfileLevel::Dev {
        phases.push(Phase {
            label: "Buildroot dependencies",
            done_msg: "Buildroot dependencies ready",
            run: buildroot::install_phase,
        });
        phases.push(Phase {
            label: "Docker Engine",
            done_msg: "Docker Engine ready",
            run: docker::install_phase,
        });
        phases.push(Phase {
            label: "Shell & UX (zsh, starship)",
            done_msg: "Shell & UX ready",
            run: zsh::install_phase,
        });
        phases.push(Phase {
            label: "Fonts",
            done_msg: "Fonts installed",
            run: fonts::install_phase,
        });
        phases.push(Phase {
            label: "rclone",
            done_msg: "rclone ready",
            run: rclone::install_phase,
        });
    }

    if enable_argon {
        phases.push(Phase {
            label: "Argon One fan script",
            done_msg: "Argon One installed",
            run: argon::install_phase,
        });
    }

    let total = phases.len() as u64;

    let mp = MultiProgress::new();
    let overall = mp.add(ProgressBar::new(total));
    overall.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} [{bar:30.green/dim}] {pos}/{len} phases  {percent}%  ETA {eta}  [{elapsed}]",
        )
        .unwrap()
        .progress_chars("━╸─")
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
    );
    overall.enable_steady_tick(std::time::Duration::from_millis(200));

    let ctx = InstallContext {
        profile,
        staging_dir: staging,
        dry_run: opts.dry_run,
        interactive: opts.interactive,
        enable_argon,
        enable_p10k,
        docker_data_root,
        mp,
        overall,
        config: cfg,
        platform: plat,
        driver_name: driver.name(),
        driver,
        pkg_backend: driver.pkg_backend(),
    };

    let mut completed_phases = Vec::new();
    for (i, phase) in phases.iter().enumerate() {
        let label = format!("Phase {}/{} · {}", i + 1, total, phase.label);
        let pb = ctx.phase_spinner(&label);
        match (phase.run)(&ctx) {
            Ok(()) => {
                ctx.finish_phase(&pb, phase.done_msg);
                completed_phases.push(phase.label);
            }
            Err(e) => {
                pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
                pb.set_prefix("✗");
                pb.finish_with_message(format!("{} FAILED: {e:#}", phase.label));
                ctx.overall.inc(1);
                let completed = if completed_phases.is_empty() {
                    "none".to_string()
                } else {
                    completed_phases.join(", ")
                };
                error!(
                    "Installation aborted during {} (staging dir: {}). Completed phases: {}. \
                     Rerun `mash-setup doctor` or remove the staging directory before retrying.",
                    phase.label,
                    ctx.staging_dir.display(),
                    completed
                );
                return Err(e);
            }
        }
    }

    ctx.overall.finish_and_clear();

    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       Installation complete!                  ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    if ctx.dry_run {
        println!("(dry-run mode – no changes were made)");
        println!();
    }

    println!("Post-install notes:");
    println!("  - Log out and back in for docker group membership to take effect.");
    println!("  - Run `mash-setup doctor` to verify everything.");
    println!("  - Config lives at ~/.config/mash-installer/config.toml");
    println!();

    Ok(())
}

struct Phase {
    label: &'static str,
    done_msg: &'static str,
    run: fn(&InstallContext) -> Result<()>,
}
