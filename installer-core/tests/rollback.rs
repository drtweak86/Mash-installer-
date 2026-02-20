use anyhow::{anyhow, Result};
use installer_core::RollbackManager;
use std::sync::{Arc, Mutex};

#[test]
fn rollback_manager_executes_actions_in_reverse_order() -> Result<()> {
    let manager = RollbackManager::new();
    let executed = Arc::new(Mutex::new(Vec::new()));

    manager.register_action("first", {
        let executed = executed.clone();
        move || {
            executed.lock().unwrap().push("first".to_string());
            Ok(())
        }
    });
    manager.register_action("second", {
        let executed = executed.clone();
        move || {
            executed.lock().unwrap().push("second".to_string());
            Ok(())
        }
    });
    manager.register_action("third", {
        let executed = executed.clone();
        move || {
            executed.lock().unwrap().push("third".to_string());
            Ok(())
        }
    });

    manager.rollback_all()?;

    let history = executed.lock().unwrap();
    assert_eq!(history.as_slice(), ["third", "second", "first"]);
    Ok(())
}

#[test]
fn rollback_manager_reports_errors_and_runs_all_actions() -> Result<()> {
    let manager = RollbackManager::new();
    let executed = Arc::new(Mutex::new(Vec::new()));

    manager.register_action("success", {
        let executed = executed.clone();
        move || {
            executed.lock().unwrap().push("success".to_string());
            Ok(())
        }
    });

    manager.register_action("failing", {
        let executed = executed.clone();
        move || {
            executed.lock().unwrap().push("failing".to_string());
            Err(anyhow!("boom"))
        }
    });

    let err = manager
        .rollback_all()
        .expect_err("expected rollback to fail");
    let history = executed.lock().unwrap();
    assert_eq!(history.as_slice(), ["failing", "success"]);
    let message = err.to_string();
    assert!(message.contains("rollback failures"));
    assert!(message.contains("failing: boom"));
    Ok(())
}
