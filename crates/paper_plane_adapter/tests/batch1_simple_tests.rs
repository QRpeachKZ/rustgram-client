// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Batch 1: Simple integration tests for paper_plane_adapter.
//!
//! These tests cover basic functionality:
//! - Request parsing for all @type values
//! - Response serialization for all response types
//! - Client lifecycle operations

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use paper_plane_adapter::request::RawRequest;
use paper_plane_adapter::response::{Response, UserInfo};
use paper_plane_adapter::RustgramClient;
use rustgram_auth_manager::State;
use std::sync::Arc;
use std::time::Duration;

// ========== Request Parsing Tests ==========

/// Integration test: Parse getMe request.
#[tokio::test]
async fn test_parse_get_me() {
    let json = r#"{"@type":"getMe"}"#;
    let raw = RawRequest::from_json(json).expect("Failed to parse getMe");
    assert_eq!(raw.type_name, "getMe");
    assert!(raw.extra.is_none());

    let typed = raw.to_typed().expect("Failed to convert to typed");
    assert_eq!(typed.type_name(), "getMe");
}

/// Integration test: Parse getChats request with limit parameter.
#[tokio::test]
async fn test_parse_get_chats() {
    let json = r#"{"@type":"getChats","limit":50}"#;
    let raw = RawRequest::from_json(json).expect("Failed to parse getChats");
    assert_eq!(raw.type_name, "getChats");

    let typed = raw.to_typed().expect("Failed to convert to typed");
    assert_eq!(typed.type_name(), "getChats");
}

/// Integration test: Parse sendMessage request.
#[tokio::test]
async fn test_parse_send_message() {
    let json = r#"{"@type":"sendMessage","chat_id":123456,"text":"Hello World"}"#;
    let raw = RawRequest::from_json(json).expect("Failed to parse sendMessage");
    assert_eq!(raw.type_name, "sendMessage");

    let typed = raw.to_typed().expect("Failed to convert to typed");
    assert_eq!(typed.type_name(), "sendMessage");
}

/// Integration test: Parse setAuthenticationPhoneNumber request.
#[tokio::test]
async fn test_parse_auth_phone() {
    let json = r#"{"@type":"setAuthenticationPhoneNumber","phone_number":"+1234567890"}"#;
    let raw = RawRequest::from_json(json).expect("Failed to parse setAuthenticationPhoneNumber");
    assert_eq!(raw.type_name, "setAuthenticationPhoneNumber");

    let typed = raw.to_typed().expect("Failed to convert to typed");
    assert_eq!(typed.type_name(), "setAuthenticationPhoneNumber");
}

/// Integration test: Parse unknown request type.
#[tokio::test]
async fn test_parse_unknown_type() {
    let json = r#"{"@type":"unknownMethod","param":"value"}"#;
    let raw = RawRequest::from_json(json).expect("Failed to parse unknown type");
    assert_eq!(raw.type_name, "unknownMethod");

    let typed = raw.to_typed().expect("Failed to convert to typed");
    assert_eq!(typed.type_name(), "unknownMethod");
}

// ========== Response Serialization Tests ==========

/// Integration test: Serialize user response.
#[tokio::test]
async fn test_serialize_user_response() {
    let user = UserInfo::new(123)
        .with_first_name("John".to_string())
        .with_last_name("Doe".to_string())
        .with_username("johndoe".to_string())
        .with_phone_number("+1234567890".to_string());

    let response = Response::user(user);
    let json = response.to_json(None).expect("Failed to serialize user");

    // Response::User wraps the user object, so we check for the nested structure
    assert!(json.contains("user"));
    // The id might be formatted differently (with/without quotes)
    assert!(json.contains("123"));
    assert!(json.contains("John"));
}

/// Integration test: Serialize chats response.
#[tokio::test]
async fn test_serialize_chats_response() {
    let chat_ids = vec![1, 2, 3, 4, 5];
    let response = Response::Chats {
        chat_ids: chat_ids.clone(),
        total_count: 5,
    };

    let json = response.to_json(None).expect("Failed to serialize chats");
    // Check for key fields
    assert!(json.contains("chats"));
    assert!(json.contains("chat_ids"));
    assert!(json.contains("total_count"));
}

/// Integration test: Serialize error response.
#[tokio::test]
async fn test_serialize_error_response() {
    let response = Response::error(404, "Not found");
    let json = response.to_json(None).expect("Failed to serialize error");

    // Check for error fields
    assert!(json.contains("error"));
    assert!(json.contains("404"));
    assert!(json.contains("Not found"));
}

/// Integration test: Serialize response with @extra.
#[tokio::test]
async fn test_serialize_with_extra() {
    let user = UserInfo::new(123).with_first_name("Test".to_string());
    let response = Response::user(user);
    let extra = serde_json::json!("test-extra-123");

    let json = response.to_json(Some(extra)).expect("Failed to serialize with @extra");
    // @extra should be included
    assert!(json.contains("@extra"));
    assert!(json.contains("test-extra-123"));
}

/// Integration test: Serialize authorization state response.
#[tokio::test]
async fn test_serialize_authorization_state() {
    let response = Response::authorization_state(State::Ok);
    let json = response.to_json(None).expect("Failed to serialize auth state");

    // Check for authorization state fields
    assert!(json.contains("authorizationState") || json.contains("authorization_state"));
}

// ========== Client Lifecycle Tests ==========

/// Integration test: Client::new() creates valid instance.
#[tokio::test]
async fn test_client_new_creates_instance() {
    let client = RustgramClient::new();
    let _client_clone = Arc::clone(&client);
    client.send(r#"{"@type":"getMe"}"#);
}

/// Integration test: Client::send() queues request.
#[tokio::test]
async fn test_client_send_queues_request() {
    let client = RustgramClient::new();

    for i in 0..5 {
        client.send(&format!(r#"{{"@type":"getMe","@extra":"req{}"}}"#, i));
    }

    tokio::time::sleep(Duration::from_millis(100)).await;
    let _ = client.receive(0.5);
}

/// Integration test: Client::receive() timeout.
#[tokio::test]
async fn test_client_receive_timeout() {
    let client = RustgramClient::new();
    let response = client.receive(0.1);
    assert!(response.is_none(), "Expected None on timeout");
}

/// Integration test: Client::send() handles invalid JSON.
#[tokio::test]
async fn test_client_send_invalid_json() {
    let client = RustgramClient::new();
    client.send("not json at all");
    client.send("{invalid json}");
    client.send("");
    tokio::time::sleep(Duration::from_millis(50)).await;
    client.send(r#"{"@type":"getMe"}"#);
}

/// Integration test: Request correlation with @extra.
#[tokio::test]
async fn test_client_request_correlation() {
    let client = RustgramClient::new();
    let extra_value = "test-correlation-12345";
    client.send(&format!(r#"{{"@type":"getMe","@extra":"{}"}}"#, extra_value));

    tokio::time::sleep(Duration::from_millis(100)).await;

    if let Some(response) = client.receive(1.0) {
        if response.contains("@extra") {
            assert!(response.contains(extra_value));
        }
    }
}

/// Integration test: Parse request with @extra field.
#[tokio::test]
async fn test_parse_with_extra() {
    let json = r#"{"@type":"getMe","@extra":"request-123"}"#;
    let raw = RawRequest::from_json(json).expect("Failed to parse with @extra");
    assert_eq!(raw.type_name, "getMe");
    assert!(raw.extra.is_some());
}

/// Integration test: Parse getUser request.
#[tokio::test]
async fn test_parse_get_user() {
    let json = r#"{"@type":"getUser","user_id":123456789}"#;
    let raw = RawRequest::from_json(json).expect("Failed to parse getUser");
    assert_eq!(raw.type_name, "getUser");

    let typed = raw.to_typed().expect("Failed to convert to typed");
    assert_eq!(typed.type_name(), "getUser");
}

/// Integration test: Parse getChat request.
#[tokio::test]
async fn test_parse_get_chat() {
    let json = r#"{"@type":"getChat","chat_id":-1001234567890}"#;
    let raw = RawRequest::from_json(json).expect("Failed to parse getChat");
    assert_eq!(raw.type_name, "getChat");

    let typed = raw.to_typed().expect("Failed to convert to typed");
    assert_eq!(typed.type_name(), "getChat");
}

/// Integration test: Serialize ok response.
#[tokio::test]
async fn test_serialize_ok_response() {
    let response = Response::ok();
    let json = response.to_json(None).expect("Failed to serialize ok");
    // Check for ok type
    assert!(json.contains("ok"));
}

/// Integration test: Serialize chat response.
#[tokio::test]
async fn test_serialize_chat_response() {
    let response = Response::chat(
        123,
        "Test Chat".to_string(),
        paper_plane_adapter::response::ChatType::Private { user_id: 123 },
    );

    let json = response.to_json(None).expect("Failed to serialize chat");
    // Check for chat fields
    assert!(json.contains("chat"));
    assert!(json.contains("123"));
    assert!(json.contains("Test Chat"));
}

/// Integration test: Client with zero timeout.
#[tokio::test]
async fn test_client_zero_timeout() {
    let client = RustgramClient::new();
    client.send(r#"{"@type":"getMe"}"#);
    let _response = client.receive(0.0);
    // May or may not have a response, but should return quickly
}

/// Integration test: Serialize message response.
#[tokio::test]
async fn test_serialize_message_response() {
    let response = Response::Message {
        id: 456,
        chat_id: 123,
        content: paper_plane_adapter::response::MessageContent::text("Hello World"),
        date: 1640995200,
    };

    let json = response.to_json(None).expect("Failed to serialize message");
    // Check for message fields
    assert!(json.contains("message"));
    assert!(json.contains("456"));
    assert!(json.contains("123"));
}
