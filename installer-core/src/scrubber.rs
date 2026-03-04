use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

lazy_static! {
    static ref SENSITIVE_STRINGS: Arc<RwLock<HashSet<String>>> =
        Arc::new(RwLock::new(HashSet::new()));
}

/// Register a string as sensitive so it can be scrubbed from logs and output.
pub fn register_secret(secret: impl Into<String>) {
    let secret = secret.into();
    if secret.len() < 4 {
        // Too short to reliably scrub without false positives
        return;
    }
    if let Ok(mut secrets) = SENSITIVE_STRINGS.write() {
        secrets.insert(secret);
    }
}

/// Scrub all registered secrets from the input string.
pub fn scrub(input: &str) -> String {
    let mut output = input.to_string();
    if let Ok(secrets) = SENSITIVE_STRINGS.read() {
        for secret in secrets.iter() {
            if !secret.is_empty() {
                let mask = "*".repeat(8);
                output = output.replace(secret, &mask);
            }
        }
    }

    // Also scrub common patterns
    scrub_patterns(&output)
}

fn scrub_patterns(input: &str) -> String {
    // Basic regex-less scrubbing for common patterns if needed
    // For now, we rely on registered secrets.
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrubbing() {
        register_secret("my-super-secret-password");
        let input = "Logging in with password: my-super-secret-password now.";
        let output = scrub(input);
        assert!(!output.contains("my-super-secret-password"));
        assert!(output.contains("********"));
    }
}
