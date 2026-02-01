// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Requests Manager
//!
//! Request routing and handling for TDLib client.
//!
//! This module provides the Requests manager which handles routing of TDLib
//! JSON requests to appropriate handlers. Based on TDLib's Requests.h.
//!
//! # TODO
//!
//! This is a simplified implementation. The full TDLib Requests.h has ~450
//! on_request method declarations. This stub provides:
//! - Request routing infrastructure
//! - Promise management for async requests
//! - Example request handlers
//!
//! A full implementation would need all TDLib API methods.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_actor::{Actor, ActorId};
use rustgram_promise::{FutureActor, Promise};
use rustgram_td::Td;
use rustgram_types::UserId;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::sync::{Arc, RwLock};

/// Request manager for TDLib client.
///
/// Handles routing of TDLib JSON requests to appropriate handlers.
/// This is a simplified version focusing on core infrastructure.
///
/// # TODO
///
/// The full TDLib Requests.h has 2000+ lines with ~450 on_request methods.
/// This implementation provides the infrastructure and a few examples.
pub struct Requests {
    /// Reference to the main Td instance
    td: Arc<Td>,

    /// Actor ID for sending messages to Td
    td_actor: ActorId<Td>,

    /// Pending requests by ID
    pending: Arc<RwLock<HashMap<u64, RequestPromiseBase>>>,

    /// Next request ID
    next_id: Arc<std::sync::atomic::AtomicU64>,
}

impl Requests {
    /// Creates a new Requests manager.
    ///
    /// # Arguments
    ///
    /// * `td` - Reference to the main Td instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// ```
    pub fn new(td: &Arc<Td>) -> Self {
        Self {
            td: Arc::clone(td),
            td_actor: ActorId::new(0),
            pending: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(std::sync::atomic::AtomicU64::new(1)),
        }
    }

    /// Returns the Td instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// assert!(!requests.td().is_initialized());
    /// ```
    pub fn td(&self) -> &Arc<Td> {
        &self.td
    }

    /// Returns the Td actor ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// assert_eq!(requests.td_actor().as_u64(), 0);
    /// ```
    pub fn td_actor(&self) -> ActorId<Td> {
        self.td_actor
    }

    /// Generates a new request ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// let id1 = requests.generate_id();
    /// let id2 = requests.generate_id();
    /// assert!(id2 > id1);
    /// ```
    pub fn generate_id(&self) -> u64 {
        self.next_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    /// Creates a promise for a request.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The result type
    ///
    /// # Returns
    ///
    /// Returns a tuple of (Promise<T>, FutureActor<T>).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// let (promise, future) = requests.create_promise::<String>();
    /// ```
    pub fn create_promise<T>(&self) -> (Promise<T>, FutureActor<T>) {
        FutureActor::pair()
    }

    /// Runs a TDLib request.
    ///
    /// # Arguments
    ///
    /// * `id` - The request ID
    /// * `function` - The function to call
    ///
    /// # TODO
    ///
    /// Implement actual request routing and execution.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use serde_json::json;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// let function = json!({"@type": "getTextEntities", "text": "@telegram"});
    /// requests.run_request(1, &function);
    /// ```
    pub fn run_request(&self, id: u64, function: &JsonValue) {
        // Stub: Store the request
        let promise = RequestPromiseBase {
            state: RequestState::Pending,
            request_id: id,
        };

        let mut pending = self.pending.write().unwrap();
        pending.insert(id, promise);
    }

    /// Gets the text entities from a string.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to parse
    ///
    /// # Returns
    ///
    /// Returns a JSON value with the entities.
    ///
    /// # TODO
    ///
    /// Implement actual text entity parsing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// let entities = requests.on_get_text_entities("@telegram hello");
    /// ```
    pub fn on_get_text_entities(&self, text: &str) -> JsonValue {
        serde_json::json!({
            "@type": "textEntities",
            "entities": []
        })
    }

    /// Processes a JSON request.
    ///
    /// # Arguments
    ///
    /// * `request` - The JSON request
    ///
    /// # Returns
    ///
    /// Returns a JSON response.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use serde_json::json;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// let request = json!({"@type": "getMe"});
    /// let response = requests.process_request(&request);
    /// ```
    pub fn process_request(&self, request: &JsonValue) -> JsonValue {
        let request_type = request
            .get("@type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        match request_type {
            "getMe" => self.on_get_me(),
            "getTextEntities" => {
                let text = request.get("text").and_then(|v| v.as_str()).unwrap_or("");
                self.on_get_text_entities(text)
            }
            "getUserId" => {
                let user_id = request.get("user_id").and_then(|v| v.as_i64()).unwrap_or(0);
                self.on_get_user_id(user_id)
            }
            _ => serde_json::json!({
                "@type": "error",
                "code": 400,
                "message": format!("Unknown request type: {}", request_type)
            }),
        }
    }

    /// Gets the current user.
    ///
    /// # TODO
    ///
    /// Return actual user data.
    ///
    /// # Returns
    ///
    /// Returns a JSON value with user information.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// let me = requests.on_get_me();
    /// ```
    pub fn on_get_me(&self) -> JsonValue {
        serde_json::json!({
            "@type": "user",
            "id": 0,
            "first_name": "",
            "last_name": "",
            "username": "",
            "phone_number": "",
            "status": {"@type": "userStatusEmpty"}
        })
    }

    /// Gets user information by ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    ///
    /// # TODO
    ///
    /// Return actual user data from database.
    ///
    /// # Returns
    ///
    /// Returns a JSON value with user information.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// let user = requests.on_get_user_id(12345);
    /// ```
    pub fn on_get_user_id(&self, user_id: i64) -> JsonValue {
        let _ = UserId::new(user_id);
        serde_json::json!({
            "@type": "user",
            "id": user_id,
            "first_name": "",
            "last_name": "",
            "username": "",
            "phone_number": "",
            "status": {"@type": "userStatusEmpty"}
        })
    }

    /// Cancels a pending request.
    ///
    /// # Arguments
    ///
    /// * `id` - The request ID to cancel
    ///
    /// # Returns
    ///
    /// Returns true if the request was cancelled, false if not found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use serde_json::json;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// let function = json!({"@type": "getMe"});
    /// requests.run_request(1, &function);
    /// assert!(requests.cancel_request(1));
    /// ```
    pub fn cancel_request(&self, id: u64) -> bool {
        let mut pending = self.pending.write().unwrap();
        pending.remove(&id).is_some()
    }

    /// Returns the number of pending requests.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use serde_json::json;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// assert_eq!(requests.pending_count(), 0);
    /// let function = json!({"@type": "getMe"});
    /// requests.run_request(1, &function);
    /// assert_eq!(requests.pending_count(), 1);
    /// ```
    pub fn pending_count(&self) -> usize {
        let pending = self.pending.read().unwrap();
        pending.len()
    }

    /// Clears all pending requests.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requests::Requests;
    /// use rustgram_td::Td;
    /// use serde_json::json;
    /// use std::sync::Arc;
    ///
    /// let td = Arc::new(Td::new());
    /// let requests = Requests::new(&td);
    /// let function = json!({"@type": "getMe"});
    /// requests.run_request(1, &function);
    /// requests.clear_pending();
    /// assert_eq!(requests.pending_count(), 0);
    /// ```
    pub fn clear_pending(&self) {
        let mut pending = self.pending.write().unwrap();
        pending.clear();
    }
}

impl Debug for Requests {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Requests")
            .field("td_actor", &self.td_actor)
            .field("pending_count", &self.pending_count())
            .finish()
    }
}

/// Base promise type for request handling.
#[derive(Debug, Clone)]
struct RequestPromiseBase {
    /// Current state of the request
    state: RequestState,
    /// Request identifier
    request_id: u64,
}

/// State of a request promise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestState {
    /// Request is pending
    Pending,
    /// Request has a result ready
    Ready,
    /// Request is complete
    Complete,
}

impl Default for RequestState {
    fn default() -> Self {
        Self::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_requests() -> Requests {
        let td = Arc::new(Td::new());
        Requests::new(&td)
    }

    #[test]
    fn test_requests_new() {
        let td = Arc::new(Td::new());
        let requests = Requests::new(&td);
        assert_eq!(requests.td_actor().as_u64(), 0);
        assert_eq!(requests.pending_count(), 0);
    }

    #[test]
    fn test_requests_td() {
        let td = Arc::new(Td::new());
        let requests = Requests::new(&td);
        assert!(!requests.td().is_initialized());
    }

    #[test]
    fn test_requests_td_actor() {
        let requests = create_test_requests();
        assert_eq!(requests.td_actor().as_u64(), 0);
    }

    #[test]
    fn test_requests_generate_id() {
        let requests = create_test_requests();
        let id1 = requests.generate_id();
        let id2 = requests.generate_id();
        let id3 = requests.generate_id();
        assert!(id2 > id1);
        assert!(id3 > id2);
        assert_eq!(id2, id1 + 1);
        assert_eq!(id3, id2 + 1);
    }

    #[test]
    fn test_requests_create_promise() {
        let requests = create_test_requests();
        let (promise, future): (Promise<String>, FutureActor<String>) =
            requests.create_promise();
        // Should not panic
        let _ = (promise, future);
    }

    #[test]
    fn test_requests_run_request() {
        let requests = create_test_requests();
        let function = json!({"@type": "getMe"});
        requests.run_request(1, &function);
        assert_eq!(requests.pending_count(), 1);
    }

    #[test]
    fn test_requests_cancel_request() {
        let requests = create_test_requests();
        let function = json!({"@type": "getMe"});
        requests.run_request(1, &function);
        assert!(requests.cancel_request(1));
        assert_eq!(requests.pending_count(), 0);
        assert!(!requests.cancel_request(1)); // Already cancelled
    }

    #[test]
    fn test_requests_cancel_nonexistent() {
        let requests = create_test_requests();
        assert!(!requests.cancel_request(999));
    }

    #[test]
    fn test_requests_pending_count() {
        let requests = create_test_requests();
        assert_eq!(requests.pending_count(), 0);

        requests.run_request(1, &json!({"@type": "getMe"}));
        assert_eq!(requests.pending_count(), 1);

        requests.run_request(2, &json!({"@type": "getMe"}));
        assert_eq!(requests.pending_count(), 2);

        requests.cancel_request(1);
        assert_eq!(requests.pending_count(), 1);
    }

    #[test]
    fn test_requests_clear_pending() {
        let requests = create_test_requests();
        requests.run_request(1, &json!({"@type": "getMe"}));
        requests.run_request(2, &json!({"@type": "getMe"}));
        requests.run_request(3, &json!({"@type": "getMe"}));
        assert_eq!(requests.pending_count(), 3);

        requests.clear_pending();
        assert_eq!(requests.pending_count(), 0);
    }

    #[test]
    fn test_requests_process_get_me() {
        let requests = create_test_requests();
        let request = json!({"@type": "getMe"});
        let response = requests.process_request(&request);

        assert_eq!(response["@type"], "user");
        assert_eq!(response["id"], 0);
    }

    #[test]
    fn test_requests_process_get_text_entities() {
        let requests = create_test_requests();
        let request = json!({"@type": "getTextEntities", "text": "@telegram"});
        let response = requests.process_request(&request);

        assert_eq!(response["@type"], "textEntities");
        assert!(response["entities"].is_array());
    }

    #[test]
    fn test_requests_process_get_user_id() {
        let requests = create_test_requests();
        let request = json!({"@type": "getUserId", "user_id": 12345});
        let response = requests.process_request(&request);

        assert_eq!(response["@type"], "user");
        assert_eq!(response["id"], 12345);
    }

    #[test]
    fn test_requests_process_unknown() {
        let requests = create_test_requests();
        let request = json!({"@type": "unknownMethod"});
        let response = requests.process_request(&request);

        assert_eq!(response["@type"], "error");
        assert_eq!(response["code"], 400);
    }

    #[test]
    fn test_requests_on_get_me() {
        let requests = create_test_requests();
        let me = requests.on_get_me();

        assert_eq!(me["@type"], "user");
        assert_eq!(me["id"], 0);
        assert_eq!(me["first_name"], "");
        assert_eq!(me["last_name"], "");
    }

    #[test]
    fn test_requests_on_get_user_id() {
        let requests = create_test_requests();
        let user = requests.on_get_user_id(999);

        assert_eq!(user["@type"], "user");
        assert_eq!(user["id"], 999);
    }

    #[test]
    fn test_requests_on_get_text_entities() {
        let requests = create_test_requests();
        let entities = requests.on_get_text_entities("@telegram hello world");

        assert_eq!(entities["@type"], "textEntities");
        assert!(entities["entities"].is_array());
    }

    #[test]
    fn test_requests_on_get_text_entities_empty() {
        let requests = create_test_requests();
        let entities = requests.on_get_text_entities("");

        assert_eq!(entities["@type"], "textEntities");
    }

    #[test]
    fn test_requests_debug() {
        let requests = create_test_requests();
        let debug_str = format!("{:?}", requests);
        assert!(debug_str.contains("Requests"));
        assert!(debug_str.contains("pending_count"));
    }

    #[test]
    fn test_requests_multiple_ids() {
        let requests = create_test_requests();

        let id1 = requests.generate_id();
        let id2 = requests.generate_id();
        let id3 = requests.generate_id();

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_requests_concurrent() {
        let requests1 = Arc::new(create_test_requests());
        let requests2 = Arc::clone(&requests1);

        requests1.run_request(1, &json!({"@type": "getMe"}));
        assert_eq!(requests2.pending_count(), 1);

        requests2.run_request(2, &json!({"@type": "getMe"}));
        assert_eq!(requests1.pending_count(), 2);
    }

    #[test]
    fn test_requests_cancel_and_re_add() {
        let requests = create_test_requests();

        requests.run_request(1, &json!({"@type": "getMe"}));
        assert!(requests.cancel_request(1));

        requests.run_request(1, &json!({"@type": "getMe"}));
        assert_eq!(requests.pending_count(), 1);
    }

    #[test]
    fn test_requests_various_promises() {
        let requests = create_test_requests();

        let _: (Promise<String>, FutureActor<String>) = requests.create_promise();
        let _: (Promise<i32>, FutureActor<i32>) = requests.create_promise();
        let _: (Promise<()>, FutureActor<()>) = requests.create_promise();

        // Should not panic
    }

    #[test]
    fn test_requests_process_multiple() {
        let requests = create_test_requests();

        let r1 = requests.process_request(&json!({"@type": "getMe"}));
        let r2 = requests.process_request(&json!({"@type": "getTextEntities", "text": "test"}));
        let r3 = requests.process_request(&json!({"@type": "getUserId", "user_id": 123}));

        assert_eq!(r1["@type"], "user");
        assert_eq!(r2["@type"], "textEntities");
        assert_eq!(r3["@type"], "user");
        assert_eq!(r3["id"], 123);
    }

    #[test]
    fn test_requests_get_text_entities_with_mentions() {
        let requests = create_test_requests();
        let entities = requests.on_get_text_entities("@user1 @user2");

        assert_eq!(entities["@type"], "textEntities");
    }

    #[test]
    fn test_requests_empty_request_handling() {
        let requests = create_test_requests();
        let request = json!({});
        let response = requests.process_request(&request);

        assert_eq!(response["@type"], "error");
    }

    #[test]
    fn test_requests_large_id_generation() {
        let requests = create_test_requests();

        for _ in 0..100 {
            requests.generate_id();
        }

        let id = requests.generate_id();
        assert!(id > 100);
    }
}
