use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct DryRunEntry {
    pub phase: String,
    pub action: String,
    pub detail: Option<String>,
}

/// A structured summary of planned actions for a dry-run.
#[derive(Clone, Debug, Default)]
pub struct PreflightAuditReport {
    /// Actions grouped by phase
    pub phases: BTreeMap<String, Vec<DryRunEntry>>,
    /// Total number of actions planned
    pub total_actions: usize,
}

pub struct DryRunLog {
    entries: RefCell<Vec<DryRunEntry>>,
}

impl DryRunLog {
    pub fn new() -> Self {
        Self {
            entries: RefCell::new(Vec::new()),
        }
    }

    pub fn record(
        &self,
        phase: impl Into<String>,
        action: impl Into<String>,
        detail: Option<String>,
    ) {
        let entry = DryRunEntry {
            phase: phase.into(),
            action: action.into(),
            detail,
        };
        self.entries.borrow_mut().push(entry);
    }

    pub fn entries(&self) -> Vec<DryRunEntry> {
        self.entries.borrow().clone()
    }

    /// Generate a structured pre-flight audit report.
    pub fn audit_report(&self) -> PreflightAuditReport {
        let entries = self.entries.borrow();
        let mut phases: BTreeMap<String, Vec<DryRunEntry>> = BTreeMap::new();

        for entry in entries.iter() {
            phases
                .entry(entry.phase.clone())
                .or_default()
                .push(entry.clone());
        }

        PreflightAuditReport {
            total_actions: entries.len(),
            phases,
        }
    }
}

impl Default for DryRunLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_run_log_records_multiple_entries() {
        let log = DryRunLog::new();
        log.record("phase-a", "action-a", Some("detail-a".into()));
        log.record("phase-b", "action-b", None);

        let entries = log.entries();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].phase, "phase-a");
        assert_eq!(entries[0].action, "action-a");
        assert_eq!(entries[0].detail.as_deref(), Some("detail-a"));
        assert_eq!(entries[1].phase, "phase-b");
        assert_eq!(entries[1].action, "action-b");
        assert!(entries[1].detail.is_none());
    }
}
