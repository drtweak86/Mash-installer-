//! Integration tests for GitHub MCP webhook server

use std::process::Command;
use std::thread;
use std::time::Duration;

/// Helper function to check if server is responsive
fn wait_for_server(url: &str, max_attempts: usize) -> Result<(), String> {
    let client = reqwest::blocking::Client::new();

    for attempt in 1..=max_attempts {
        match client.get(url).send() {
            Ok(response) if response.status().is_success() => return Ok(()),
            Ok(_) => thread::sleep(Duration::from_millis(100)), // Server responded but not ready
            Err(_) => {
                if attempt < max_attempts {
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }
    }
    Err(format!(
        "Server not responsive after {} attempts",
        max_attempts
    ))
}

#[test]
fn test_server_starts_and_responds() {
    // Start the server in the background
    let mut server = Command::new("cargo")
        .args([
            "run",
            "--",
            "--bind-address",
            "127.0.0.1:34567",
            "--github-webhook-secret",
            "test_secret",
            "--max-event-history",
            "10",
        ])
        .spawn()
        .expect("Failed to start server");

    // Wait for server to be responsive with retry logic
    let health_url = "http://127.0.0.1:34567/health";
    match wait_for_server(health_url, 10) {
        Ok(_) => {
            // Test health endpoint
            let client = reqwest::blocking::Client::new();
            let response = client
                .get(health_url)
                .send()
                .expect("Failed to send request");

            assert!(response.status().is_success());
            let body: serde_json::Value = response.json().expect("Failed to parse JSON");
            assert_eq!(body["status"], "healthy");
        }
        Err(e) => {
            // Clean up even if test fails
            let _ = server.kill();
            let _ = server.wait();
            panic!("{}", e);
        }
    }

    // Clean up
    server.kill().expect("Failed to kill server");
    server.wait().expect("Failed to wait for server");
}

#[test]
fn test_webhook_validation() {
    // This would test actual webhook signature validation
    // For now, just verify the server can handle a POST request

    let mut server = Command::new("cargo")
        .args([
            "run",
            "--",
            "--bind-address",
            "127.0.0.1:34568",
            "--github-webhook-secret",
            "test_secret",
        ])
        .spawn()
        .expect("Failed to start server");

    // Wait for server to be responsive
    let webhook_url = "http://127.0.0.1:34568/webhook";
    match wait_for_server("http://127.0.0.1:34568/health", 10) {
        Ok(_) => {
            let client = reqwest::blocking::Client::new();

            // Test invalid webhook (missing headers)
            let response = client
                .post(webhook_url)
                .body("{\"test\": \"data\"}")
                .send()
                .expect("Failed to send request");

            // Should get a 400 Bad Request due to missing headers
            assert_eq!(response.status().as_u16(), 400);
        }
        Err(e) => {
            // Clean up even if test fails
            let _ = server.kill();
            let _ = server.wait();
            panic!("{}", e);
        }
    }

    server.kill().expect("Failed to kill server");
    server.wait().expect("Failed to wait for server");
}

#[test]
#[ignore = "Requires manual setup and GitHub webhook configuration"]
fn test_actual_webhook_signature_validation() {
    // This test would require:
    // 1. A running server instance
    // 2. Proper GitHub webhook secret configuration
    // 3. Actual GitHub webhook payloads
    // 4. Signature validation setup

    // For now, this test is ignored but serves as documentation
    // for future implementation
}
