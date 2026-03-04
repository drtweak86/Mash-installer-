use crate::ai_agents;
use crate::argon;
use crate::buildroot;
use crate::context::UserOptionsContext;
use crate::docker;
use crate::fonts;
use crate::github;
use crate::localization::Localization;
use crate::options::ProfileLevel;
use crate::phase_runner::{FunctionPhase, Phase, PhaseResult};
use crate::phases::wallpapers;
use crate::pi4b;
use crate::pkg;
use crate::rclone;
use crate::rust;
use crate::snapshots;
use crate::software_tiers;
use crate::zsh;
use crate::PhaseContext;
use anyhow::Result;

/// Metadata-driven registry of installer phases.
pub struct PhaseRegistry {
    entries: Vec<PhaseEntry>,
}

use crate::dependency_graph::DependencyGraph;
use std::collections::HashMap;

impl PhaseRegistry {
    fn new(entries: Vec<PhaseEntry>) -> Self {
        Self { entries }
    }

    pub fn build_phases(
        &self,
        options: &UserOptionsContext,
        strings: &Localization,
    ) -> Vec<Box<dyn Phase>> {
        // First filter by gate
        let filtered_entries: Vec<&PhaseEntry> = self
            .entries
            .iter()
            .filter(|entry| entry.should_run(options))
            .collect();

        // Build dependency graph
        let mut graph = DependencyGraph::new();
        let mut entry_map = HashMap::new();

        for entry in &filtered_entries {
            graph.add_node(
                entry.key.to_string(),
                entry.deps.iter().map(|s| s.to_string()).collect(),
            );
            entry_map.insert(entry.key, *entry);
        }

        // Sort names
        let sorted_names = match graph.topological_sort() {
            Ok(names) => names,
            Err(e) => {
                // If there's a circular dependency, we log and fallback to original order
                // which is better than panicking in production.
                tracing::error!(
                    "Circular dependency in phases: {}. Falling back to default order.",
                    e
                );
                return filtered_entries
                    .iter()
                    .map(|entry| entry.to_phase(strings))
                    .collect();
            }
        };

        // Reconstruct phases in sorted order
        sorted_names
            .into_iter()
            .filter_map(|name| entry_map.get(name.as_str()))
            .map(|entry| entry.to_phase(strings))
            .collect()
    }
}

impl Default for PhaseRegistry {
    fn default() -> Self {
        Self::new(vec![
            PhaseEntry::new(
                "snapshots",
                "Filesystem Snapshots",
                "Pre-install snapshot ready",
                snapshots::install_phase,
                PhaseGate::Always,
            ),
            PhaseEntry::new(
                "system_packages",
                "System packages",
                "System packages installed",
                pkg::install_phase,
                PhaseGate::Always,
            )
            .with_deps(&["snapshots"]),
            PhaseEntry::new(
                "rust_toolchain",
                "Rust toolchain + cargo tools",
                "Rust toolchain ready",
                rust::install_phase,
                PhaseGate::Always,
            )
            .with_deps(&["system_packages"]),
            PhaseEntry::new(
                "git_cli",
                "Git, GitHub CLI, SSH",
                "Git / GitHub CLI ready",
                github::install_phase,
                PhaseGate::Always,
            )
            .with_deps(&["system_packages"]),
            PhaseEntry::new(
                "fonts",
                "Fonts",
                "Fonts installed",
                fonts::install_phase,
                PhaseGate::Always,
            )
            .with_deps(&["system_packages"]),
            PhaseEntry::new(
                "pi4b_hdd_tuning",
                "Pi 4B HDD Tuning",
                "HDD tuning applied",
                pi4b::install_phase,
                PhaseGate::Always,
            )
            .with_deps(&["system_packages"]),
            PhaseEntry::new(
                "shell_ux",
                "Shell & UX (zsh, starship)",
                "Shell & UX ready",
                zsh::install_phase,
                PhaseGate::Profile(ProfileLevel::Dev),
            )
            .with_deps(&["system_packages", "git_cli"]),
            PhaseEntry::new(
                "docker_engine",
                "Docker Engine",
                "Docker Engine ready",
                docker::install_phase,
                PhaseGate::Profile(ProfileLevel::Dev),
            )
            .with_deps(&["system_packages"]),
            PhaseEntry::new(
                "buildroot_dependencies",
                "Buildroot dependencies",
                "Buildroot dependencies ready",
                buildroot::install_phase,
                PhaseGate::Profile(ProfileLevel::Dev),
            )
            .with_deps(&["system_packages"]),
            PhaseEntry::new(
                "software_tiers",
                "Curated software tiers",
                "Software tiers installed",
                software_tiers::install_phase,
                PhaseGate::SoftwareTiers,
            )
            .with_deps(&["system_packages", "rust_toolchain", "git_cli"]),
            PhaseEntry::new(
                "wallpapers",
                "Wallpapers",
                "Retro-futuristic wallpapers installed",
                wallpapers::install_phase,
                PhaseGate::SoftwareTiers,
            )
            .with_deps(&["system_packages"]),
            PhaseEntry::new(
                "ai_spirits",
                "AI Spirits",
                "AI assistants installed",
                ai_agents::install_phase,
                PhaseGate::Always,
            )
            .with_deps(&["system_packages"]),
            PhaseEntry::new(
                "rclone",
                "rclone",
                "rclone ready",
                rclone::install_phase,
                PhaseGate::Profile(ProfileLevel::Dev),
            )
            .with_deps(&["system_packages"]),
            PhaseEntry::new(
                "argon_one",
                "Argon One fan script",
                "Argon One installed",
                argon::install_phase,
                PhaseGate::ModuleArgon,
            )
            .with_deps(&["system_packages"]),
        ])
    }
}

struct PhaseEntry {
    key: &'static str,
    label: &'static str,
    description: &'static str,
    run: fn(&mut PhaseContext) -> Result<PhaseResult>,
    gate: PhaseGate,
    deps: &'static [&'static str],
}

impl PhaseEntry {
    fn new(
        key: &'static str,
        label: &'static str,
        description: &'static str,
        run: fn(&mut PhaseContext) -> Result<PhaseResult>,
        gate: PhaseGate,
    ) -> Self {
        Self {
            key,
            label,
            description,
            run,
            gate,
            deps: &[],
        }
    }

    fn with_deps(mut self, deps: &'static [&'static str]) -> Self {
        self.deps = deps;
        self
    }

    fn should_run(&self, options: &UserOptionsContext) -> bool {
        self.gate.should_run(options)
    }

    fn to_phase(&self, strings: &Localization) -> Box<dyn Phase> {
        let entry = strings.phase_or_default(self.key, self.label, self.description);
        Box::new(
            FunctionPhase::new(entry.label, entry.description, self.run)
                .with_deps(self.deps.to_vec()),
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn dummy_run(_ctx: &mut PhaseContext) -> Result<PhaseResult> {
        Ok(PhaseResult::Success)
    }

    #[test]
    fn test_phase_registry_sorting() -> Result<()> {
        let registry = PhaseRegistry::new(vec![
            PhaseEntry::new("C", "C", "C", dummy_run, PhaseGate::Always).with_deps(&["B"]),
            PhaseEntry::new("A", "A", "A", dummy_run, PhaseGate::Always),
            PhaseEntry::new("B", "B", "B", dummy_run, PhaseGate::Always).with_deps(&["A"]),
        ]);

        let options = UserOptionsContext {
            profile: ProfileLevel::Minimal,
            staging_dir: PathBuf::from("/tmp"),
            dry_run: true,
            interactive: false,
            enable_argon: false,
            enable_p10k: false,
            docker_data_root: false,
            software_plan: Default::default(),
        };
        let strings = Localization::load_default()?;

        let phases = registry.build_phases(&options, &strings);

        assert_eq!(phases.len(), 3);
        assert_eq!(phases[0].name(), "A");
        assert_eq!(phases[1].name(), "B");
        assert_eq!(phases[2].name(), "C");

        Ok(())
    }
}
