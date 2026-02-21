use crate::argon;
use crate::buildroot;
use crate::context::UserOptionsContext;
use crate::docker;
use crate::fonts;
use crate::github;
use crate::localization::Localization;
use crate::options::ProfileLevel;
use crate::phase_runner::{FunctionPhase, Phase};
use crate::pi4b_hdd;
use crate::pkg;
use crate::rclone;
use crate::rust;
use crate::software_tiers;
use crate::zsh;
use crate::PhaseContext;
use anyhow::Result;

/// Metadata-driven registry of installer phases.
pub struct PhaseRegistry {
    entries: Vec<PhaseEntry>,
}

impl PhaseRegistry {
    fn new(entries: Vec<PhaseEntry>) -> Self {
        Self { entries }
    }

    pub fn build_phases(
        &self,
        options: &UserOptionsContext,
        strings: &Localization,
    ) -> Vec<Box<dyn Phase>> {
        self.entries
            .iter()
            .filter(|entry| entry.should_run(options))
            .map(|entry| entry.to_phase(strings))
            .collect()
    }
}

impl Default for PhaseRegistry {
    fn default() -> Self {
        Self::new(vec![
            PhaseEntry::new(
                "system_packages",
                "System packages",
                "System packages installed",
                pkg::install_phase,
                PhaseGate::Always,
            ),
            PhaseEntry::new(
                "software_tiers",
                "Curated software tiers",
                "Software tiers installed",
                software_tiers::install_phase,
                PhaseGate::SoftwareTiers,
            ),
            PhaseEntry::new(
                "rust_toolchain",
                "Rust toolchain + cargo tools",
                "Rust toolchain ready",
                rust::install_phase,
                PhaseGate::Always,
            ),
            PhaseEntry::new(
                "git_cli",
                "Git, GitHub CLI, SSH",
                "Git / GitHub CLI ready",
                github::install_phase,
                PhaseGate::Always,
            ),
            PhaseEntry::new(
                "buildroot_dependencies",
                "Buildroot dependencies",
                "Buildroot dependencies ready",
                buildroot::install_phase,
                PhaseGate::Profile(ProfileLevel::Dev),
            ),
            PhaseEntry::new(
                "docker_engine",
                "Docker Engine",
                "Docker Engine ready",
                docker::install_phase,
                PhaseGate::Profile(ProfileLevel::Dev),
            ),
            PhaseEntry::new(
                "shell_ux",
                "Shell & UX (zsh, starship)",
                "Shell & UX ready",
                zsh::install_phase,
                PhaseGate::Profile(ProfileLevel::Dev),
            ),
            PhaseEntry::new(
                "fonts",
                "Fonts",
                "Fonts installed",
                fonts::install_phase,
                PhaseGate::Profile(ProfileLevel::Dev),
            ),
            PhaseEntry::new(
                "rclone",
                "rclone",
                "rclone ready",
                rclone::install_phase,
                PhaseGate::Profile(ProfileLevel::Dev),
            ),
            PhaseEntry::new(
                "pi4b_hdd_tuning",
                "Pi 4B HDD Tuning",
                "HDD tuning applied",
                pi4b_hdd::install_phase,
                PhaseGate::Always,
            ),
            PhaseEntry::new(
                "argon_one",
                "Argon One fan script",
                "Argon One installed",
                argon::install_phase,
                PhaseGate::ModuleArgon,
            ),
        ])
    }
}

struct PhaseEntry {
    key: &'static str,
    label: &'static str,
    description: &'static str,
    run: fn(&mut PhaseContext) -> Result<()>,
    gate: PhaseGate,
}

impl PhaseEntry {
    fn new(
        key: &'static str,
        label: &'static str,
        description: &'static str,
        run: fn(&mut PhaseContext) -> Result<()>,
        gate: PhaseGate,
    ) -> Self {
        Self {
            key,
            label,
            description,
            run,
            gate,
        }
    }

    fn should_run(&self, options: &UserOptionsContext) -> bool {
        self.gate.should_run(options)
    }

    fn to_phase(&self, strings: &Localization) -> Box<dyn Phase> {
        let entry = strings.phase_or_default(self.key, self.label, self.description);
        Box::new(FunctionPhase::new(entry.label, entry.description, self.run))
    }
}

#[derive(Clone, Copy, Debug)]
enum PhaseGate {
    Always,
    Profile(ProfileLevel),
    ModuleArgon,
    SoftwareTiers,
}

impl PhaseGate {
    fn should_run(&self, options: &UserOptionsContext) -> bool {
        match self {
            PhaseGate::Always => true,
            PhaseGate::Profile(level) => options.profile >= *level,
            PhaseGate::ModuleArgon => options.enable_argon,
            PhaseGate::SoftwareTiers => !options.software_plan.is_empty(),
        }
    }
}
