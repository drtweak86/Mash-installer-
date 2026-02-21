use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration-driven defaults for interaction points.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(default)]
pub struct InteractionConfig {
    #[serde(default)]
    pub confirm_defaults: HashMap<String, bool>,
    #[serde(default)]
    pub text_defaults: HashMap<String, String>,
    #[serde(default)]
    pub selection_defaults: HashMap<String, usize>,
}

impl InteractionConfig {
    fn confirm_default(&self, key: &str) -> Option<bool> {
        self.confirm_defaults.get(key).copied()
    }

    fn text_default(&self, key: &str) -> Option<String> {
        self.text_defaults.get(key).cloned()
    }

    fn selection_default(&self, key: &str) -> Option<usize> {
        self.selection_defaults.get(key).copied()
    }
}

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
        if let Some(value) = self.config.confirm_default(key) {
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
        if let Some(value) = self.config.text_default(key) {
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
        if let Some(value) = self.config.selection_default(key) {
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
<<<<<<< HEAD
=======

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
>>>>>>> ddc3885dfd9b45043e46295832538432e7c593a8
}
