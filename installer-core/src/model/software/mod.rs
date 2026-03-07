use crate::model::Validator;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum::{Display, EnumIter, EnumString, VariantNames};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThemePlan {
    #[default]
    None,
    RetroOnly,
    RetroWithWallpapers,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Display, EnumString,
)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    F,
}

impl Tier {
    pub fn resolve(&self) -> Vec<Tier> {
        match self {
            Tier::S => vec![Tier::S],
            Tier::A => vec![Tier::S, Tier::A],
            Tier::B => vec![Tier::S, Tier::A, Tier::B],
            Tier::C => vec![Tier::S, Tier::A, Tier::B, Tier::C],
            Tier::D => vec![Tier::S, Tier::A, Tier::B, Tier::C, Tier::D],
            Tier::F => vec![Tier::S, Tier::A, Tier::B, Tier::C, Tier::D, Tier::F],
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Display,
    EnumIter,
    EnumString,
    VariantNames,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SoftwareCategory {
    Internet,
    Development,
    Multimedia,
    Games,
    Office,
    System,
    Themes,
    Virtualization,
    Science,
    Accessibility,
    // Subcategories elevated for UI selection
    Editors,
    Terminals,
    Shells,
    // Misc
    Social,
    Workflow,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareTierPlan {
    pub full_install: bool,
    /// Selections mapping Category -> Program IDs
    pub selections: BTreeMap<SoftwareCategory, Vec<String>>,
    pub theme_plan: ThemePlan,
    pub preset_id: Option<String>,
    pub target_tier: Option<Tier>,
}

impl Validator for SoftwareTierPlan {
    fn validate(&self) -> Vec<String> {
        Vec::new()
    }
}

impl SoftwareTierPlan {
    pub fn new(
        full_install: bool,
        selections: BTreeMap<SoftwareCategory, Vec<String>>,
        theme_plan: ThemePlan,
        preset_id: Option<String>,
        target_tier: Option<Tier>,
    ) -> Self {
        Self {
            full_install,
            selections,
            theme_plan,
            preset_id,
            target_tier,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.selections.is_empty() && self.target_tier.is_none()
    }
}

impl Default for SoftwareTierPlan {
    fn default() -> Self {
        Self {
            full_install: true,
            selections: BTreeMap::new(),
            theme_plan: ThemePlan::None,
            preset_id: None,
            target_tier: Some(Tier::S),
        }
    }
}
