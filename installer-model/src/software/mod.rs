use crate::Validator;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum ThemePlan {
    #[default]
    None,
    RetroOnly,
    RetroWithWallpapers,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareTierPlan {
    pub full_install: bool,
    /// Selections mapping Category ID -> Program ID
    pub selections: BTreeMap<String, String>,
    pub theme_plan: ThemePlan,
    pub preset_id: Option<String>,
}

impl Validator for SoftwareTierPlan {
    fn validate(&self) -> Vec<String> {
        // Validation logic can be expanded here if needed.
        // For now, we assume IDs are valid if they come from the UI.
        Vec::new()
    }
}

impl SoftwareTierPlan {
    pub fn new(
        full_install: bool,
        selections: BTreeMap<String, String>,
        theme_plan: ThemePlan,
        preset_id: Option<String>,
    ) -> Self {
        Self {
            full_install,
            selections,
            theme_plan,
            preset_id,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.selections.is_empty()
    }
}

impl Default for SoftwareTierPlan {
    fn default() -> Self {
        Self {
            full_install: true,
            selections: BTreeMap::new(),
            theme_plan: ThemePlan::None,
            preset_id: None,
        }
    }
}
