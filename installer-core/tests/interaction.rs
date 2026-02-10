use anyhow::Result;
use installer_core::interaction::{InteractionConfig, InteractionService};

#[test]
fn confirm_uses_config_default_before_prompt() -> Result<()> {
    let mut cfg = InteractionConfig::default();
    cfg.confirm_defaults.insert("danger".into(), true);
    let svc = InteractionService::new(false, cfg);
    let confirmed = svc.confirm("danger", "Destroy data?", false, || {
        panic!("should not reach interactive path");
    })?;
    assert!(confirmed);
    Ok(())
}

#[test]
fn confirm_noninteractive_falls_back_to_default_when_missing() -> Result<()> {
    let svc = InteractionService::new(false, InteractionConfig::default());
    let confirmed = svc.confirm("safe", "Proceed?", true, || {
        panic!("interactive stub should not run");
    })?;
    assert!(confirmed);
    Ok(())
}

#[test]
fn select_option_uses_interactive_fn_when_needed() -> Result<()> {
    let svc = InteractionService::new(true, InteractionConfig::default());
    let mut invoked = false;
    let choice = svc.select_option(
        "select",
        "Make a choice",
        &["one", "two"],
        1,
        |_prompt, _options| {
            invoked = true;
            Ok(2)
        },
    )?;
    assert!(invoked);
    assert_eq!(choice, 2);
    Ok(())
}

#[test]
fn get_text_input_returns_error_when_noninteractive_without_default() {
    let svc = InteractionService::new(false, InteractionConfig::default());
    let result = svc.get_text_input("input", "Enter value", false, None, |_p, _s| {
        panic!("should not prompt");
    });
    assert!(result.is_err());
}

#[test]
fn get_text_input_uses_default_when_noninteractive() -> Result<()> {
    let svc = InteractionService::new(false, InteractionConfig::default());
    let answer = svc.get_text_input(
        "value",
        "Enter value",
        false,
        Some("override"),
        |_p, _s| {
            panic!("should not prompt");
        },
    )?;
    assert_eq!(answer, "override");
    Ok(())
}
