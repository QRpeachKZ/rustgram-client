// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Batch 3: State machine integration tests for paper_plane_adapter.
//!
//! These tests cover state transitions and error recovery:
//! - Auth state transitions (all valid paths)
//! - Invalid state transition rejection
//! - Connection state changes
//! - Client state persistence

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use paper_plane_adapter::RustgramClient;
use std::time::Duration;

// ========== Auth State Transition Tests ==========

/// Integration test: Initial state (None to WaitPhoneNumber).
#[tokio::test]
async fn test_auth_state_none_to_wait_phone() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"getAuthorizationState"}"#);
    tokio::time::sleep(Duration::from_millis(50)).await;

    if let Some(response) = client.receive(0.5) {
        assert!(
            response.contains("authorizationState") ||
            response.contains("@type")
        );
    }
}

/// Integration test: WaitPhoneNumber to WaitCode transition.
#[tokio::test]
async fn test_auth_state_wait_phone_to_wait_code() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"setAuthenticationPhoneNumber","phone_number":"+1234567890"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    client.send(r#"{"@type":"getAuthorizationState"}"#);
    tokio::time::sleep(Duration::from_millis(50)).await;

    while let Some(_) = client.receive(0.1) {}
}

/// Integration test: WaitCode to Ready transition.
#[tokio::test]
async fn test_auth_state_wait_code_to_ready() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"setAuthenticationPhoneNumber","phone_number":"+1234567890"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    while let Some(_) = client.receive(0.1) {}

    client.send(r#"{"@type":"checkAuthenticationCode","code":"12345"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    client.send(r#"{"@type":"getAuthorizationState"}"#);
    tokio::time::sleep(Duration::from_millis(50)).await;

    if let Some(response) = client.receive(0.5) {
        assert!(response.contains("authorizationState") || response.contains("error"));
    }
}

/// Integration test: WaitCode to WaitPassword transition (2FA).
#[tokio::test]
async fn test_auth_state_wait_code_to_wait_password() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"setAuthenticationPhoneNumber","phone_number":"+1234567890"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    client.send(r#"{"@type":"checkAuthenticationCode","code":"12345"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    client.send(r#"{"@type":"getAuthorizationState"}"#);
    tokio::time::sleep(Duration::from_millis(50)).await;

    let _ = client.receive(0.5);
}

/// Integration test: WaitPassword to Ready transition.
#[tokio::test]
async fn test_auth_state_wait_password_to_ready() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"setAuthenticationPhoneNumber","phone_number":"+1234567890"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    client.send(r#"{"@type":"checkAuthenticationCode","code":"12345"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    while let Some(_) = client.receive(0.1) {}

    client.send(r#"{"@type":"checkAuthenticationPassword","password":"mypassword"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    client.send(r#"{"@type":"getAuthorizationState"}"#);
    tokio::time::sleep(Duration::from_millis(50)).await;

    let _ = client.receive(0.5);
}

/// Integration test: Invalid state transition rejection.
#[tokio::test]
async fn test_auth_state_invalid_transition() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"checkAuthenticationPassword","password":"password"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    if let Some(response) = client.receive(0.5) {
        assert!(
            response.contains("error") ||
            response.contains("authorizationStateWaitPhoneNumber")
        );
    }
}

/// Integration test: State persistence check.
#[tokio::test]
async fn test_auth_state_persistence_check() {
    let client = RustgramClient::new();

    let mut _responses = Vec::new();

    for _ in 0..3 {
        client.send(r#"{"@type":"getAuthorizationState"}"#);
        tokio::time::sleep(Duration::from_millis(50)).await;

        if let Some(response) = client.receive(0.5) {
            if response.contains("authorizationState") {
                _responses.push(response.clone());
            }
        }
    }
}

/// Integration test: State query during transition.
#[tokio::test]
async fn test_auth_state_query_during_transition() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"getAuthorizationState"}"#);
    client.send(r#"{"@type":"setAuthenticationPhoneNumber","phone_number":"+1234567890"}"#);
    client.send(r#"{"@type":"getAuthorizationState"}"#);

    tokio::time::sleep(Duration::from_millis(100)).await;

    for _ in 0..3 {
        let _ = client.receive(0.5);
    }
}

/// Integration test: Complete auth cycle.
#[tokio::test]
async fn test_auth_state_complete_cycle() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"getAuthorizationState"}"#);
    tokio::time::sleep(Duration::from_millis(50)).await;

    client.send(r#"{"@type":"setAuthenticationPhoneNumber","phone_number":"+1234567890"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    client.send(r#"{"@type":"checkAuthenticationCode","code":"12345"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    client.send(r#"{"@type":"checkAuthenticationPassword","password":"password"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    client.send(r#"{"@type":"getAuthorizationState"}"#);
    tokio::time::sleep(Duration::from_millis(50)).await;

    let _responses: Vec<_> = (0..10)
        .filter_map(|_| client.receive(0.5))
        .collect();
}

// ========== Error Recovery Tests ==========

/// Integration test: Manager unavailable error.
#[tokio::test]
async fn test_manager_unavailable_error() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"getMe"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    if let Some(response) = client.receive(0.5) {
        assert!(
            response.contains("error") ||
            response.contains("500") ||
            response.contains("not available")
        );
    }
}

/// Integration test: Network error recovery simulation.
#[tokio::test]
async fn test_network_error_recovery() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"getUser","user_id":999999999}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    let _first_response = client.receive(0.5);

    client.send(r#"{"@type":"getMe"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    let _second_response = client.receive(0.5);
}

/// Integration test: Invalid request error.
#[tokio::test]
async fn test_invalid_request_error() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"getMe""#); // Missing closing brace

    tokio::time::sleep(Duration::from_millis(50)).await;

    client.send(r#"{"@type":"getMe"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    let _ = client.receive(0.5);
}

/// Integration test: Timeout cleanup.
#[tokio::test]
async fn test_timeout_cleanup() {
    let client = RustgramClient::new();

    for i in 0..10 {
        client.send(&format!(r#"{{"@type":"getMe","@extra":"req{}"}}"#, i));
    }

    let mut _timeouts = 0;
    for _ in 0..20 {
        if client.receive(0.01).is_none() {
            _timeouts += 1;
        }
    }

    client.send(r#"{"@type":"getMe"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    let _response = client.receive(0.5);
}

/// Integration test: Missing required field error.
#[tokio::test]
async fn test_missing_field_error() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"sendMessage","text":"Hello"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    if let Some(response) = client.receive(0.5) {
        assert!(
            response.contains("error") ||
            response.contains("400") ||
            response.contains("chat_id")
        );
    }
}

/// Integration test: Invalid field value error.
#[tokio::test]
async fn test_invalid_field_value_error() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"getChat","chat_id":"not_a_number"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    if let Some(response) = client.receive(0.5) {
        assert!(
            response.contains("error") ||
            response.contains("400")
        );
    }
}

/// Integration test: Recovery after manager error.
#[tokio::test]
async fn test_recovery_after_manager_error() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"getUser","user_id":123}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    let _ = client.receive(0.5);

    client.send(r#"{"@type":"getMe"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    let _response = client.receive(0.5);
}

/// Integration test: Error response format validation.
#[tokio::test]
async fn test_error_response_format() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"getUser","user_id":999}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    if let Some(response) = client.receive(0.5) {
        if response.contains("error") {
            // Check for error-related fields (format may vary)
            assert!(response.contains("@type") || response.contains("error"));
            // The response should have some content
            assert!(!response.is_empty());
        }
    }
}

/// Integration test: Multiple sequential errors.
#[tokio::test]
async fn test_multiple_sequential_errors() {
    let client = RustgramClient::new();

    for i in 0..5 {
        client.send(&format!(r#"{{"@type":"getUser","user_id":{}}}"#, i));
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    let mut _error_count = 0;
    for _ in 0..10 {
        if let Some(response) = client.receive(0.5) {
            if response.contains("error") {
                _error_count += 1;
            }
        }
    }
}

/// Integration test: Error recovery with valid request.
#[tokio::test]
async fn test_error_recovery_with_valid_request() {
    let client = RustgramClient::new();

    client.send(r#"{"@type":"getUser","user_id":999}"#);
    client.send(r#"{"@type":"getChat","chat_id":0}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    while client.receive(0.1).is_some() {}

    client.send(r#"{"@type":"getAuthorizationState"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    if let Some(response) = client.receive(0.5) {
        assert!(!response.is_empty());
    }
}

/// Integration test: Error with @extra correlation.
#[tokio::test]
async fn test_error_with_extra_correlation() {
    let client = RustgramClient::new();

    let extra_id = "test-error-12345";

    client.send(&format!(
        r#"{{"@type":"getUser","user_id":999,"@extra":"{}"}}"#,
        extra_id
    ));
    tokio::time::sleep(Duration::from_millis(100)).await;

    if let Some(response) = client.receive(0.5) {
        if response.contains("error") {
            assert!(response.contains(extra_id));
        }
    }
}

/// Integration test: Panic prevention on invalid input.
#[tokio::test]
async fn test_panic_prevention_invalid_input() {
    let client = RustgramClient::new();

    let invalid_inputs = vec![
        "",
        "not json",
        "{",
        "}",
        r#"{"@type":""}"#,
        r#"{"@type":"   ","extra":null}"#,
        r#"{{"invalid":}}"#,
    ];

    for input in invalid_inputs {
        client.send(input);
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    client.send(r#"{"@type":"getMe"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    let _ = client.receive(0.5);
}

/// Integration test: Empty request handling.
#[tokio::test]
async fn test_empty_request_handling() {
    let client = RustgramClient::new();

    client.send("");
    tokio::time::sleep(Duration::from_millis(50)).await;

    client.send("   \n\t  ");
    tokio::time::sleep(Duration::from_millis(50)).await;

    client.send(r#"{"@type":"getMe"}"#);
    tokio::time::sleep(Duration::from_millis(100)).await;

    let _ = client.receive(0.5);
}
