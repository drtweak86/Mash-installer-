use crate::options::ProfileLevel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PackageIntent {
    Required,
    Optional,
}

#[derive(Clone, Copy, Debug)]
pub struct PackageSpec<'a> {
    canonical: &'a str,
    intent: PackageIntent,
    min_profile: ProfileLevel,
}

impl<'a> PackageSpec<'a> {
    pub const fn new(canonical: &'a str, intent: PackageIntent, min_profile: ProfileLevel) -> Self {
        Self {
            canonical,
            intent,
            min_profile,
        }
    }

    pub const fn required(canonical: &'a str) -> Self {
        Self::new(canonical, PackageIntent::Required, ProfileLevel::Minimal)
    }

    pub const fn required_for(canonical: &'a str, min_profile: ProfileLevel) -> Self {
        Self::new(canonical, PackageIntent::Required, min_profile)
    }

    pub const fn optional(canonical: &'a str) -> Self {
        Self::new(canonical, PackageIntent::Optional, ProfileLevel::Minimal)
    }

    pub const fn optional_for(canonical: &'a str, min_profile: ProfileLevel) -> Self {
        Self::new(canonical, PackageIntent::Optional, min_profile)
    }

    pub fn canonical(&self) -> &'a str {
        self.canonical
    }

    pub fn intent(&self) -> PackageIntent {
        self.intent
    }

    pub fn is_applicable(&self, profile: ProfileLevel) -> bool {
        profile >= self.min_profile
    }
}
