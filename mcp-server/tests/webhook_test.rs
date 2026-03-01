//! Integration tests for GitHub MCP webhook server

use std::process::Command;
use std::thread;
use std::time::Duration;

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

    // Give the server a moment to start
    thread::sleep(Duration::from_secs(2));

    // Test health endpoint
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("http://127.0.0.1:34567/health")
        .send()
        .expect("Failed to send request");

    assert!(response.status().is_success());
    let body: serde_json::Value = response.json().expect("Failed to parse JSON");
    assert_eq!(body["status"], "healthy");

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

    thread::sleep(Duration::from_secs(2));

    let client = reqwest::blocking::Client::new();

    // Test invalid webhook (missing headers)
    let response = client
        .post("http://127.0.0.1:34568/webhook")
        .body("{\"test\": \"data\"}")
        .send()
        .expect("Failed to send request");

    // Should get a 400 Bad Request due to missing headers
    assert_eq!(response.status().as_u16(), 400);

    server.kill().expect("Failed to kill server");
    server.wait().expect("Failed to wait for server");
}
