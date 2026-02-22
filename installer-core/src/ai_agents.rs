//! AI Spirits install phase.
//!
//! Installs optional AI coding assistants (Claude Code, Gemini CLI, Mistral Vibe)
//! when the user selects them under the "AI Spirits" software category. Also
//! injects a GitHub MCP server entry into any detected AI desktop config files
//! (Claude Desktop, Zed, Cursor, VS Code).
//!
//! Entry point: [`install_phase`], called from [`crate::phase_registry`].

use crate::{cmd, package_manager, PhaseContext};
use anyhow::Result;
use std::process::Command;

pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    let selections = &ctx.options.software_plan.selections;

    // Check if any AI agents are selected in the "AI Spirits" category
    let mut selected_agents = Vec::new();
    if let Some(pick) = selections.get("AI Spirits") {
        selected_agents.push(*pick);
    } else {
        // Check all selections just in case
        for (cat, pick) in selections {
            if cat == &"AI Spirits" || *pick == "Claude" || *pick == "Gemini" || *pick == "Vibe" {
                selected_agents.push(*pick);
            }
        }
    }

    if selected_agents.is_empty() {
        return Ok(());
    }

    // Ensure nodejs/npm are present for these tools
    ctx.record_action("Ensuring Node.js environment for AI spirits");
    package_manager::ensure_packages(ctx.platform.driver, &["nodejs", "npm"], ctx.options.dry_run)?;

    for agent in selected_agents {
        match agent {
            "Claude" => install_claude(ctx)?,
            "Gemini" => install_gemini(ctx)?,
            "Vibe" => install_vibe(ctx)?,
            _ => {}
        }
    }

    // Configure MCP servers if applicable
    configure_mcp_servers(ctx)?;

    Ok(())
}

fn configure_mcp_servers(ctx: &mut PhaseContext) -> Result<()> {
    let home = dirs::home_dir().unwrap_or_default();

    // Common locations for MCP-compatible configurations
    let config_paths = vec![
        (
            home.join(".config/Claude/claude_desktop_config.json"),
            "mcpServers",
        ),
        (
            home.join(".config/Claude Desktop/claude_desktop_config.json"),
            "mcpServers",
        ),
        (home.join(".config/zed/settings.json"), "context_servers"),
        (home.join(".config/Cursor/User/settings.json"), "mcpServers"),
        (home.join(".config/Code/User/settings.json"), "mcpServers"),
    ];

    // Try to find an existing GITHUB_PERSONAL_ACCESS_TOKEN in any of these files
    let mut existing_token = String::new();
    if !ctx.options.dry_run {
        for (path, key) in &config_paths {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(token) = find_github_token(&config, key) {
                            existing_token = token;
                            ctx.record_action(format!(
                                "Detected existing GitHub token in {}",
                                path.display()
                            ));
                            break;
                        }
                    }
                }
            }
        }
    }

    for (path, key) in config_paths {
        if path.exists() || ctx.options.dry_run {
            ctx.run_or_record(
                "AI Spirits",
                "Configure MCP GitHub Server",
                Some(format!("Injecting GitHub server into {}", path.display())),
                |_| {
                    if ctx.options.dry_run {
                        return Ok(());
                    }

                    let content =
                        std::fs::read_to_string(&path).unwrap_or_else(|_| "{}".to_string());
                    let mut config: serde_json::Value =
                        serde_json::from_str(&content).unwrap_or(serde_json::json!({}));

                    // Create server container if it doesn't exist
                    if config.get(key).is_none() {
                        config[key] = serde_json::json!({});
                    }

                    // Add github server
                    config[key]["github"] = serde_json::json!({
                        "command": "npx",
                        "args": ["-y", "@modelcontextprotocol/server-github"],
                        "env": {
                            "GITHUB_PERSONAL_ACCESS_TOKEN": existing_token
                        }
                    });

                    let new_content = serde_json::to_string_pretty(&config)?;
                    std::fs::write(&path, new_content)?;
                    Ok(())
                },
            )?;
        }
    }
    Ok(())
}

fn find_github_token(config: &serde_json::Value, key: &str) -> Option<String> {
    let token = config
        .get(key)?
        .get("github")?
        .get("env")?
        .get("GITHUB_PERSONAL_ACCESS_TOKEN")?
        .as_str()?;

    if token.trim().is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

fn install_claude(ctx: &mut PhaseContext) -> Result<()> {
    ctx.run_or_record(
        "AI Spirits",
        "Install Claude Code",
        Some("@anthropic-ai/claude-code".into()),
        |_| {
            let mut cmd = Command::new("sudo");
            cmd.args(["npm", "install", "-g", "@anthropic-ai/claude-code"]);
            cmd::run(&mut cmd)?;
            Ok(())
        },
    )
}

fn install_gemini(ctx: &mut PhaseContext) -> Result<()> {
    ctx.run_or_record(
        "AI Spirits",
        "Install Gemini CLI",
        Some("@google/gemini-cli".into()),
        |_| {
            let mut cmd = Command::new("sudo");
            cmd.args(["npm", "install", "-g", "@google/gemini-cli"]);
            cmd::run(&mut cmd)?;
            Ok(())
        },
    )
}

fn install_vibe(ctx: &mut PhaseContext) -> Result<()> {
    ctx.run_or_record(
        "AI Spirits",
        "Install Mistral Vibe",
        Some("@mistral-ai/vibe".into()),
        |_| {
            let mut cmd = Command::new("sudo");
            cmd.args(["npm", "install", "-g", "@mistral-ai/vibe"]);
            cmd::run(&mut cmd)?;
            Ok(())
        },
    )
}
