use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct SoftwareTierPlan {
    pub full_install: bool,
    pub selections: BTreeMap<&'static str, &'static str>,
}

impl SoftwareTierPlan {
    pub fn new(full_install: bool, selections: BTreeMap<&'static str, &'static str>) -> Self {
        Self {
            full_install,
            selections,
        }
    }
}

impl Default for SoftwareTierPlan {
    fn default() -> Self {
        Self {
            full_install: true,
            selections: BTreeMap::new(),
        }
    }
}
