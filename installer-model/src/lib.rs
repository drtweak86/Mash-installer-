pub mod config;
pub mod options;
pub mod phase;
pub mod preset;
pub mod profile;
pub mod software;

/// Trait for validating configuration and options.
pub trait Validator {
    /// Perform validation and return a list of error messages.
    /// An empty vector indicates a valid configuration.
    fn validate(&self) -> Vec<String>;

    /// Return true if the configuration is valid.
    fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}
