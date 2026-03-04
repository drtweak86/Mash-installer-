use crate::software::SoftwareTierPlan;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A Preset is a curated combination of software, themes, and tweaks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub theme_id: String,
    pub software_plan: SoftwareTierPlan,
    #[serde(default)]
    pub tweaks: Vec<String>,
}

impl Preset {
    /// Applies the preset to the given software plan and options.
    pub fn apply(&self, software_plan: &mut SoftwareTierPlan) {
        // Overlay preset selections onto the existing plan
        for (category, program_id) in &self.software_plan.selections {
            software_plan
                .selections
                .insert(category.to_string(), program_id.to_string());
        }

        // Merge theme plan if specified
        if self.software_plan.theme_plan != crate::software::ThemePlan::None {
            software_plan.theme_plan = self.software_plan.theme_plan.clone();
        }
    }
}

/// Repository of available presets.
pub struct PresetRegistry {
    pub presets: BTreeMap<String, Preset>,
}

impl PresetRegistry {
    pub fn load_all() -> anyhow::Result<Self> {
        // In a real implementation, this would load from resources/presets/*.toml
        // For now, we'll return a few hardcoded defaults to get the engine running.
        let mut presets = BTreeMap::new();

        let cyberpunk = Preset {
            id: "cyberpunk".into(),
            name: "Cyberpunk Neon".into(),
            description: "High-contrast neon aesthetics with a tech-heavy toolset.".into(),
            theme_id: "neon-night".into(),
            software_plan: SoftwareTierPlan {
                full_install: true,
                selections: [
                    ("Terminal".into(), "kitty".into()),
                    ("Editor".into(), "neovim".into()),
                    ("Shell".into(), "zsh".into()),
                ]
                .into_iter()
                .collect(),
                theme_plan: crate::software::ThemePlan::None, // Will be managed by theme_id
                preset_id: Some("cyberpunk".into()),
            },
            tweaks: vec!["enable_p10k".into(), "terminal_transparency".into()],
        };

        presets.insert(cyberpunk.id.clone(), cyberpunk);

        Ok(Self { presets })
    }
}
