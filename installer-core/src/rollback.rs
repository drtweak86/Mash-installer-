use anyhow::{anyhow, Result};
use std::cell::RefCell;

pub struct RollbackEntry {
    pub label: String,
    pub action: Box<dyn Fn() -> Result<()> + 'static>,
}

impl RollbackEntry {
    pub fn new(label: impl Into<String>, action: impl Fn() -> Result<()> + 'static) -> Self {
        Self {
            label: label.into(),
            action: Box::new(action),
        }
    }
}

pub struct RollbackManager {
    entries: RefCell<Vec<RollbackEntry>>,
}

impl RollbackManager {
    pub fn new() -> Self {
        Self {
            entries: RefCell::new(Vec::new()),
        }
    }

    pub fn register_action(
        &self,
        label: impl Into<String>,
        action: impl Fn() -> Result<()> + 'static,
    ) {
        self.entries
            .borrow_mut()
            .push(RollbackEntry::new(label, action));
    }

    pub fn rollback_all(&self) -> Result<()> {
        let mut entries = self.entries.borrow_mut();
        let mut failures = Vec::new();
        while let Some(entry) = entries.pop() {
            if let Err(err) = (entry.action)() {
                failures.push(format!("{}: {err}", entry.label));
            }
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(anyhow!("rollback failures: {}", failures.join("; ")))
        }
    }
}
