mod argon;
mod buildroot;
mod config;
mod docker;
mod doctor;
mod fonts;
mod github;
mod pkg;
mod platform;
mod rclone;
mod rust;
mod staging;
mod zsh;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tracing::info;

/// Mash Installer – idempotent mega-installer for Raspberry Pi / Ubuntu dev machines.
#[derive(Parser)]
#[command(name = "mash-setup", version, about)]
struct Cli {
    /// Enable verbose logging (RUST_LOG override)
    #[arg(long, short, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the full installation
    Install {
        /// Installation profile
        #[arg(long, default_value = "dev")]
        profile: Profile,

        /// Override staging directory
        #[arg(long)]
        staging_dir: Option<PathBuf>,

        /// Dry run – print what would happen without executing
        #[arg(long)]
        dry_run: bool,

        /// Enable interactive prompts (default is unattended)
        #[arg(long)]
        interactive: bool,

        /// Enable Ollama installation (off by default on ARM)
        #[arg(long)]
        enable_ollama: bool,

        /// Enable Argon One fan script installation
        #[arg(long)]
        enable_argon: bool,

        /// Enable Powerlevel10k zsh theme installation
        #[arg(long)]
        enable_p10k: bool,

        /// Set Docker data-root to staging_dir/docker
        #[arg(long)]
        docker_data_root: bool,
    },
    /// Diagnose the current system state
    Doctor,
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Write default config to ~/.config/mash-installer/config.toml
    Init,
    /// Print the current configuration
    Show,
}

#[derive(Clone, ValueEnum)]
enum Profile {
    /// Minimal: essential build tools + git only
    Minimal,
    /// Dev: full developer workstation (default)
    Dev,
    /// Full: everything including optional components
    Full,
}

/// Central context threaded through every install phase.
pub struct InstallContext {
    pub profile: ProfileLevel,
    pub staging_dir: PathBuf,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_ollama: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
    pub mp: MultiProgress,
    /// Overall progress bar (% done + ETA).
    pub overall: ProgressBar,
    pub config: config::MashConfig,
    pub platform: platform::PlatformInfo,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialise tracing
    let filter = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(filter)),
        )
        .without_time()
        .init();

    match cli.command {
        Commands::Install {
            profile,
            staging_dir,
            dry_run,
            interactive,
            enable_ollama,
            enable_argon,
            enable_p10k,
            docker_data_root,
        } => {
            let profile_level = match profile {
                Profile::Minimal => ProfileLevel::Minimal,
                Profile::Dev => ProfileLevel::Dev,
                Profile::Full => ProfileLevel::Full,
            };

            run_install(
                profile_level,
                staging_dir,
                dry_run,
                interactive,
                enable_ollama,
                enable_argon,
                enable_p10k,
                docker_data_root,
            )
        }
        Commands::Doctor => doctor::run_doctor(),
        Commands::Config { action } => match action {
            ConfigAction::Init => config::init_config(),
            ConfigAction::Show => config::show_config(),
        },
    }
}

/// Descriptor for a single install phase.
struct Phase {
    label: &'static str,
    done_msg: &'static str,
    run: fn(&InstallContext) -> Result<()>,
}

#[allow(clippy::too_many_arguments)]
fn run_install(
    profile: ProfileLevel,
    staging_dir_override: Option<PathBuf>,
    dry_run: bool,
    interactive: bool,
    enable_ollama: bool,
    enable_argon: bool,
    enable_p10k: bool,
    docker_data_root: bool,
) -> Result<()> {
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
    if let Some(ref model) = plat.pi_model {
        info!("Raspberry Pi model: {}", model);
    }

    let cfg = config::load_or_default()?;
    let staging = staging::resolve(staging_dir_override.as_deref(), &cfg)?;
    info!("Staging directory: {}", staging.display());

    // ── Build phase list based on profile / flags ───────────────
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

    // ── Set up progress bars ────────────────────────────────────
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
        dry_run,
        interactive,
        enable_ollama,
        enable_argon,
        enable_p10k,
        docker_data_root,
        mp,
        overall,
        config: cfg,
        platform: plat,
    };

    // ── Execute phases ──────────────────────────────────────────
    for (i, phase) in phases.iter().enumerate() {
        let label = format!("Phase {}/{} · {}", i + 1, total, phase.label,);
        let pb = ctx.phase_spinner(&label);
        match (phase.run)(&ctx) {
            Ok(()) => ctx.finish_phase(&pb, phase.done_msg),
            Err(e) => {
                pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
                pb.set_prefix("✗");
                pb.finish_with_message(format!("{} FAILED: {e:#}", phase.label));
                ctx.overall.inc(1);
                return Err(e);
            }
        }
    }

    ctx.overall.finish_and_clear();

    // ── Summary ─────────────────────────────────────────────────
    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       Installation complete!                  ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    if ctx.dry_run {
        println!("(dry-run mode – no changes were made)");
        println!();
    }

    // Post-install notes
    println!("Post-install notes:");
    println!("  - Log out and back in for docker group membership to take effect.");
    println!("  - Run `mash-setup doctor` to verify everything.");
    println!("  - Config lives at ~/.config/mash-installer/config.toml");
    if ctx.platform.arch == "aarch64" && !ctx.enable_ollama {
        println!("  - Ollama was skipped (ARM). Re-run with --enable-ollama to install.");
    }
    println!();

    Ok(())
}
