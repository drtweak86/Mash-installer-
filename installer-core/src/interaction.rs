pub use crate::model::config::InteractionConfig;
use anyhow::{anyhow, Result};

/// Central gatekeeper for every prompt or interactive decision.
#[derive(Debug, Clone)]
pub struct InteractionService {
    interactive: bool,
    config: InteractionConfig,
}

impl InteractionService {
    pub fn new(interactive: bool, config: InteractionConfig) -> Self {
        Self {
            interactive,
            config,
        }
    }

    pub fn is_interactive(&self) -> bool {
        self.interactive
    }

    pub fn confirm<F>(
        &self,
        key: &str,
        _prompt: &str,
        default: bool,
        mut interactive_fn: F,
    ) -> Result<bool>
    where
        F: FnMut() -> Result<bool>,
    {
        if let Some(value) = self.config.confirm_defaults.get(key).copied() {
            return Ok(value);
        }
        if !self.interactive {
            return Ok(default);
        }
        interactive_fn()
    }

    pub fn get_text_input<F>(
        &self,
        key: &str,
        prompt: &str,
        sensitive: bool,
        default: Option<&str>,
        mut interactive_fn: F,
    ) -> Result<String>
    where
        F: FnMut(&str, bool) -> Result<String>,
    {
        if let Some(value) = self.config.text_defaults.get(key).cloned() {
            return Ok(value);
        }
        if !self.interactive {
            if let Some(value) = default {
                return Ok(value.to_string());
            }
            return Err(anyhow!(
                "non-interactive mode requires {} but no default answer was provided",
                prompt
            ));
        }
        interactive_fn(prompt, sensitive)
    }

    pub fn select_option<F>(
        &self,
        key: &str,
        prompt: &str,
        options: &[&str],
        default: usize,
        mut interactive_fn: F,
    ) -> Result<usize>
    where
        F: FnMut(&str, &[&str]) -> Result<usize>,
    {
        if let Some(value) = self.config.selection_defaults.get(key).copied() {
            return Ok(value);
        }
        if !self.interactive {
            return Ok(default);
        }
        interactive_fn(prompt, options)
    }

    /// Prompt for sudo password
    pub fn sudo_password<F>(&self, mut interactive_fn: F) -> Result<String>
    where
        F: FnMut(&str) -> Result<String>,
    {
        if !self.interactive {
            return Ok(String::new());
        }
        interactive_fn("Enter sudo password:")
    }

    /// Prompt for sudo password with custom message
    #[allow(dead_code)]
    pub fn sudo_password_with_message<F>(
        &self,
        prompt: &str,
        mut interactive_fn: F,
    ) -> Result<String>
    where
        F: FnMut(&str) -> Result<String>,
    {
        if !self.interactive {
            return Ok(String::new());
        }
        interactive_fn(prompt)
    }
}
