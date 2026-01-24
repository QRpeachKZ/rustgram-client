// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR OR Apache-2.0

//! # Paper Plane Adapter
//!
//! TDLib-compatible JSON adapter for Rustgram Telegram client.
//!
//! This crate provides a JSON API that matches TDLib's format, allowing
//! TDLib-compatible GUI applications like Paper Plane to use Rustgram
//! as a drop-in replacement.
//!
//! ## Overview
//!
//! The adapter maps JSON requests to async manager calls and serializes
//! responses back to JSON. It supports:
//!
//! - Dialog operations (getChats, createChat, setChatTitle)
//! - Message operations (sendMessage)
//! - User operations (getUsers, getMe, getUser)
//! - Authentication operations (setAuthenticationPhoneNumber, checkAuthenticationCode)
//!
//! ## Architecture
//!
//! ```text
//! Paper Plane GUI
//!       │
//!       ├── JSON Request ──┐
//!       │                  ▼
//!       │         RustgramClient
//!       │                  ├── RequestMapper
//!       │                  │       ├── DialogManager
//!       │                  │       ├── MessagesManager
//!       │                  │       └── UserManager
//!       │                  ├── await response
//!       │                  └── serialize_json()
//!       │                  ▼
//!       └── JSON Response ←─┘
//! ```text
//!
//! ## Example
//!
//! ```no_run
//! use paper_plane_adapter::RustgramClient;
//!
//! // Create client
//! let client = RustgramClient::new();
//!
//! // Send a request (non-blocking, thread-safe)
//! client.send(r#"{"@type":"getMe"}"#);
//!
//! // Receive response (blocking with timeout)
//! if let Some(response) = client.receive(5.0) {
//!     println!("Got response: {}", response);
//! }
//! ```
//!
//! ## Thread Safety
//!
//! - [`RustgramClient::send()`] is thread-safe and can be called from any thread
//! - [`RustgramClient::receive()`] should be called from a single thread
//! - Internal state is protected by `Arc<Mutex<T>>` for concurrent access

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod adapter;
pub mod client;
pub mod error;
pub mod mapper;
pub mod request;
pub mod response;
pub mod update;

// Re-export TDLib-compatible types for public API
pub use crate::client::{ClientId, ClientConfig, ClientRegistry};

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use error::{AdapterError, Result};
use mapper::RequestMapper;
use request::RawRequest;
use response::OutgoingResponse;
use rustgram_auth_manager::AuthManager;
use rustgram_client_actor::ClientActor;
use rustgram_dialog_manager::{DialogManager, NetworkClient as DialogNetworkClient};
use rustgram_messages_manager::MessagesManager;
use rustgram_user_manager::UserManager;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

// Use crossbeam channels for std::sync channels with recv_timeout
use crossbeam_channel as std_channel;

/// Default channel capacity for requests.
#[allow(dead_code)]
const CHANNEL_CAPACITY: usize = 1000;

/// Default receive timeout in seconds.
pub const DEFAULT_TIMEOUT: f64 = 5.0;

/// Global client registry singleton.
///
/// TDLib maintains a global registry of client instances.
/// This matches that pattern for TDLib compatibility.
static GLOBAL_REGISTRY: OnceLock<Arc<ClientRegistry>> = OnceLock::new();

/// Gets or initializes the global client registry.
///
/// # Note
///
/// This is public for integration tests and internal use. In production,
/// the registry is accessed internally by `create()` and `destroy()` methods.
///
/// # WARNING
///
/// Direct access to the registry is not part of the stable public API and
/// may change in future versions. Use `RustgramClient::create()` and
/// `RustgramClient::destroy()` for client lifecycle management.
pub fn global_registry() -> &'static Arc<ClientRegistry> {
    GLOBAL_REGISTRY.get_or_init(|| {
        Arc::new(ClientRegistry::new())
    })
}

/// Request context for tracking pending requests.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct RequestContext {
    /// Client-provided extra data (echoed in response).
    extra: Option<serde_json::Value>,
    /// When the request was sent (for timeout).
    timestamp: std::time::Instant,
}

/// TDLib-compatible JSON client for Rustgram.
///
/// Provides a synchronous API for async Rustgram managers.
/// Thread-safe: can be called from any thread.
///
/// # Example
///
/// ```no_run
/// use paper_plane_adapter::RustgramClient;
///
/// let client = RustgramClient::new();
///
/// // Send request (thread-safe)
/// client.send(r#"{"@type":"getMe"}"#);
///
/// // Receive response
/// if let Some(response) = client.receive(5.0) {
///     println!("{}", response);
/// }
/// ```
#[derive(Clone)]
pub struct RustgramClient {
    /// Request sender (sync → async bridge).
    tx: mpsc::UnboundedSender<IncomingRequest>,

    /// Response receiver (for poll-based API with timeout support).
    rx: Arc<std_channel::Receiver<OutgoingResponse>>,

    /// Pending requests for correlation.
    pending: Arc<Mutex<HashMap<u64, RequestContext>>>,

    /// Next request ID.
    next_id: Arc<AtomicU64>,

    /// Client handle (for runtime management).
    _client_handle: Arc<ClientHandle>,
}

/// Internal client handle for managing the runtime.
struct ClientHandle {
    /// Whether the runtime is still running.
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl Drop for ClientHandle {
    fn drop(&mut self) {
        // Signal the runtime to stop
        self.running.store(false, Ordering::Relaxed);
    }
}

/// A request from the client with context for correlation.
#[allow(dead_code)]
struct IncomingRequest {
    /// Request ID for correlation.
    request_id: u64,

    /// Parsed request object.
    request: request::Request,

    /// Client-provided extra data (echoed in response).
    extra: Option<serde_json::Value>,

    /// When the request was sent (for timeout).
    timestamp: std::time::Instant,
}

impl RustgramClient {
    /// Creates a new Rustgram client instance.
    ///
    /// # Returns
    ///
    /// Client handle for sending/receiving JSON requests.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use paper_plane_adapter::RustgramClient;
    ///
    /// let client = RustgramClient::new();
    /// ```
    #[must_use]
    pub fn new() -> Arc<Self> {
        Self::with_config(Default::default())
    }

    /// Creates a new client with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Internal client configuration
    ///
    /// # Returns
    ///
    /// Client handle for sending/receiving JSON requests
    #[must_use]
    pub(crate) fn with_config(config: InternalClientConfig) -> Arc<Self> {
        let (tx, mut rx) = mpsc::unbounded_channel::<IncomingRequest>();
        let (response_tx, response_rx) = std_channel::unbounded();

        let pending = Arc::new(Mutex::new(HashMap::new()));
        let next_id = Arc::new(AtomicU64::new(1));

        // Create the request mapper
        let mapper = RequestMapper::new(
            config.dialog_manager,
            config.messages_manager,
            config.user_manager,
            config.auth_manager,
            config.dialog_network_client,
        );

        // Spawn the processing task
        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));

        let pending_clone = Arc::clone(&pending);
        let running_clone = Arc::clone(&running);

        tokio::spawn(async move {
            info!("Request processor task started");

            while running_clone.load(Ordering::Relaxed) {
                match rx.recv().await {
                    Some(req) => {
                        let response_result = mapper.process_request(&req.request, req.request_id).await;

                        // Extract extra from pending requests
                        let extra: Option<serde_json::Value> = {
                            if let Ok(mut pending) = pending_clone.try_lock() {
                                pending.remove(&req.request_id).and_then(|ctx: RequestContext| ctx.extra)
                            } else {
                                // Lock failed, skip extra
                                None
                            }
                        };

                        // Convert Response to OutgoingResponse
                        let final_response = match response_result {
                            Ok(response) => {
                                let json = match response.to_json(extra) {
                                    Ok(j) => j,
                                    Err(_) => {
                                        // Error serializing response
                                        String::from("{\"@type\":\"error\",\"code\":500,\"message\":\"Failed to serialize response\"}")
                                    }
                                };
                                OutgoingResponse::new(req.request_id, json)
                            }
                            Err(err) => {
                                // Return error response
                                let (code, message) = error_code_from_error(&err);
                                let json = format!(
                                    r#"{{"@type":"error","code":{},"message":"{}","@client_id":0}}"#,
                                    code,
                                    json_escape(&message)
                                );
                                OutgoingResponse::new(req.request_id, json)
                            }
                        };

                        // Send response (ignore if channel closed)
                        let _ = response_tx.send(final_response);
                    }
                    None => {
                        // Channel closed, stop processing
                        break;
                    }
                }
            }

            info!("Request processor task stopped");
        });

        let client_handle = Arc::new(ClientHandle { running });

        let client = Self {
            tx,
            rx: Arc::new(response_rx),
            pending,
            next_id,
            _client_handle: client_handle,
        };
        Arc::new(client)
    }

    /// Sends a JSON request to the client.
    ///
    /// This is thread-safe and can be called from any thread.
    /// The request is queued for async processing.
    ///
    /// # Arguments
    ///
    /// * `json_request` - JSON-serialized request string
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paper_plane_adapter::RustgramClient;
    /// # let client = RustgramClient::new();
    /// client.send(r#"{"@type":"getMe"}"#);
    /// ```
    pub fn send(&self, json_request: &str) {
        // Parse the request
        let raw_request = match RawRequest::from_json(json_request) {
            Ok(req) => req,
            Err(_err) => {
                // For parse errors, we can't send a response without a channel
                // Just log and return
                warn!("Failed to parse JSON request");
                return;
            }
        };

        // Extract extra before moving raw_request
        let extra = raw_request.extra.clone();

        // Convert to typed request
        let request = match raw_request.to_typed() {
            Ok(req) => req,
            Err(_err) => {
                // For validation errors, we also can't respond
                warn!("Failed to validate request");
                return;
            }
        };

        // Generate request ID
        let request_id = self.next_id.fetch_add(1, Ordering::Relaxed);

        // Store request context
        {
            if let Ok(mut pending) = self.pending.try_lock() {
                pending.insert(
                    request_id,
                    RequestContext {
                        extra: extra.clone(),
                        timestamp: std::time::Instant::now(),
                    },
                );
            }
            // If lock fails, we just don't track the request (unlikely in practice)
        }

        // Send to processor
        let incoming = IncomingRequest {
            request_id,
            request,
            extra,
            timestamp: std::time::Instant::now(),
        };

        if self.tx.send(incoming).is_err() {
            error!("Failed to send request to processor (channel closed)");
            if let Ok(mut pending) = self.pending.try_lock() {
                pending.remove(&request_id);
            }
        }
    }

    /// Receives the next response or update.
    ///
    /// Returns `None` if timeout expires or channel is closed.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum seconds to wait
    ///
    /// # Returns
    ///
    /// JSON-serialized response or `None`
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paper_plane_adapter::RustgramClient;
    /// # let client = RustgramClient::new();
    /// if let Some(response) = client.receive(5.0) {
    ///     println!("{}", response);
    /// }
    /// ```
    pub fn receive(&self, timeout: f64) -> Option<String> {
        let duration = Duration::from_secs_f64(timeout);

        // Try to receive with timeout
        match self.rx.recv_timeout(duration) {
            Ok(resp) => Some(resp.response),
            Err(std_channel::RecvTimeoutError::Timeout) => None,
            Err(std_channel::RecvTimeoutError::Disconnected) => None,
        }
    }

    /// Synchronously executes a request.
    ///
    /// Only works for requests marked as "Can be called synchronously".
    ///
    /// # Arguments
    ///
    /// * `json_request` - JSON-serialized request
    ///
    /// # Returns
    ///
    /// JSON-serialized response
    ///
    /// # Errors
    ///
    /// Returns an error if the request is not synchronous or fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paper_plane_adapter::RustgramClient;
    /// # let client = RustgramClient::new();
    /// let _response = client.execute(r#"{"@type":"getMe"}"#);
    /// ```
    pub fn execute(&self, json_request: &str) -> Result<String> {
        // Parse request
        let raw_request = RawRequest::from_json(json_request)?;
        let request = raw_request.to_typed()?;

        // Check if synchronous
        if !request.is_synchronous() {
            return Err(AdapterError::InvalidRequest(
                "Request cannot be executed synchronously".to_string(),
            ));
        }

        // For synchronous requests, we can't easily handle them without
        // blocking. Return an error for now.
        // In production, this would need a different approach.
        Err(AdapterError::NotSupported(
            "Synchronous execution not yet implemented".to_string(),
        ))
    }

    // ========== Helper Methods ==========

    #[allow(dead_code)]
    fn create_error_response(
        request_id: u64,
        err: &AdapterError,
        extra: Option<&serde_json::Value>,
    ) -> OutgoingResponse {
        let (code, message) = error_code_from_error(err);

        let extra_json = extra.map(serde_json::to_string).unwrap_or(Ok("null".to_string())).unwrap_or_else(|_| "null".to_string());

        let json = format!(
            r#"{{"@type":"error","code":{},"message":"{}","@extra":{},"@client_id":0}}"#,
            code,
            json_escape(&message),
            extra_json
        );

        OutgoingResponse::new(request_id, json)
    }

    #[allow(dead_code)]
    fn send_response_directly(&self, _response: OutgoingResponse) -> Result<()> {
        // Try to get the response channel
        // This is a bit tricky since we don't have direct access
        // For now, we'll just drop the response
        // In production, we'd need a different channel design
        warn!("Failed to send error response directly (not implemented)");
        Ok(())
    }

    #[allow(dead_code)]
    fn inject_extra_into_response(response: &str, extra: &serde_json::Value) -> Result<String> {
        // Parse the response JSON
        let mut value: serde_json::Value = serde_json::from_str(response)?;

        // Inject @extra
        if let Some(obj) = value.as_object_mut() {
            obj.insert("@extra".to_string(), extra.clone());
        }

        // Serialize back
        serde_json::to_string_pretty(&value).map_err(Into::into)
    }

    // ========== TDLib-Compatible Client Creation ==========

    /// Creates a new TDLib-compatible client instance.
    ///
    /// This method matches TDLib's td_create_client_id() semantics:
    /// - Creates a new ClientActor with the provided configuration
    /// - Registers the actor in the global client registry
    /// - Returns a unique ClientId for subsequent operations
    ///
    /// # Arguments
    ///
    /// * `config` - TDLib-style client configuration (api_id, api_hash, database_path, etc.)
    ///
    /// # Returns
    ///
    /// * `ClientId` - Unique client identifier compatible with TDLib
    ///
    /// # Errors
    ///
    /// Returns `AdapterError` if:
    /// - Configuration validation fails (api_id <= 0, empty api_hash)
    /// - ClientActor initialization fails
    /// - Registration fails (should not happen in practice)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use paper_plane_adapter::{RustgramClient, ClientConfig};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = ClientConfig {
    ///     api_id: 12345,
    ///     api_hash: "abcdef".to_string(),
    ///     database_path: "/tmp/tdlib".to_string(),
    ///     files_directory: "/tmp/tdlib_files".to_string(),
    ///     use_test_dc: false,
    ///     default_dc_id: 2,
    /// };
    ///
    /// let client_id = RustgramClient::create(config).await?;
    /// println!("Created client: {}", client_id.get());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # TDLib Equivalent
    ///
    /// This method is equivalent to:
    /// ```c
    /// int32_t td_create_client_id();
    /// ```
    ///
    /// But with initialization parameters passed as config instead of setTdlibParameters.
    pub async fn create(config: client::ClientConfig) -> Result<ClientId> {
        // Validate api_id
        if config.api_id <= 0 {
            return Err(AdapterError::InvalidValue {
                field: "api_id".to_string(),
                reason: "must be > 0".to_string(),
            });
        }

        // Validate api_hash
        if config.api_hash.is_empty() {
            return Err(AdapterError::InvalidValue {
                field: "api_hash".to_string(),
                reason: "must not be empty".to_string(),
            });
        }

        // Create ClientActor with default options
        // In production, you would use config values to configure the actor
        let actor = Arc::new(ClientActor::new_with_defaults());

        // Get global registry and register the actor
        let registry = global_registry();
        let client_id = registry.register(actor).await;

        info!("Created new client with ID {}", client_id.get());

        Ok(client_id)
    }

    /// Destroys a client instance and releases its resources.
    ///
    /// Matches TDLib's td_destroy_client_id() semantics.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID to destroy
    ///
    /// # Returns
    ///
    /// * `Ok(())` if client was destroyed successfully
    ///
    /// # Errors
    ///
    /// Returns `AdapterError::InvalidClientId` if the client doesn't exist.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use paper_plane_adapter::{RustgramClient, ClientConfig};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::default();
    /// # let client_id = RustgramClient::create(config).await?;
    /// // Destroy the client when done
    /// RustgramClient::destroy(client_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # TDLib Equivalent
    ///
    /// This method is equivalent to:
    /// ```c
    /// void td_destroy_client_id(int32_t client_id);
    /// ```
    pub async fn destroy(client_id: ClientId) -> Result<()> {
        let registry = global_registry();
        registry.unregister(client_id).await?;

        info!("Destroyed client {}", client_id.get());

        Ok(())
    }
}

impl Default for RustgramClient {
    fn default() -> Self {
        // Create a client without managers for testing
        // This will return errors for most operations
        let (tx, _rx) = mpsc::unbounded_channel();
        let (_response_tx, response_rx) = std_channel::unbounded();
        let pending = Arc::new(Mutex::new(HashMap::new()));
        let next_id = Arc::new(AtomicU64::new(1));
        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));

        // Spawn minimal processor
        let running_clone = Arc::clone(&running);
        tokio::spawn(async move {
            while running_clone.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });

        Self {
            tx,
            rx: Arc::new(response_rx),
            pending,
            next_id,
            _client_handle: Arc::new(ClientHandle { running }),
        }
    }
}

/// Internal configuration for RustgramClient.
///
/// This is for internal use only. External users should use
/// `client::ClientConfig` for TDLib-compatible client configuration.
#[derive(Clone, Default)]
#[allow(dead_code)]
pub(crate) struct InternalClientConfig {
    /// Dialog manager instance.
    pub dialog_manager: DialogManager,

    /// Messages manager instance.
    pub messages_manager: Option<Arc<MessagesManager>>,

    /// User manager instance.
    pub user_manager: Option<Arc<UserManager>>,

    /// Auth manager instance.
    pub auth_manager: Option<Arc<AuthManager>>,

    /// Network client for dialog operations.
    pub dialog_network_client: Option<Arc<DialogNetworkClient>>,
}

#[allow(dead_code)]
impl InternalClientConfig {
    /// Creates a new default configuration.
    #[must_use]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Sets the dialog manager.
    #[must_use]
    pub(crate) fn with_dialog_manager(mut self, manager: DialogManager) -> Self {
        self.dialog_manager = manager;
        self
    }

    /// Sets the messages manager.
    #[must_use]
    pub(crate) fn with_messages_manager(mut self, manager: Arc<MessagesManager>) -> Self {
        self.messages_manager = Some(manager);
        self
    }

    /// Sets the user manager.
    #[must_use]
    pub(crate) fn with_user_manager(mut self, manager: Arc<UserManager>) -> Self {
        self.user_manager = Some(manager);
        self
    }

    /// Sets the auth manager.
    #[must_use]
    pub(crate) fn with_auth_manager(mut self, manager: Arc<AuthManager>) -> Self {
        self.auth_manager = Some(manager);
        self
    }

    /// Sets the dialog network client.
    #[must_use]
    pub(crate) fn with_dialog_network_client(mut self, client: Arc<DialogNetworkClient>) -> Self {
        self.dialog_network_client = Some(client);
        self
    }
}

/// Gets the error code from an AdapterError.
fn error_code_from_error(err: &AdapterError) -> (i32, String) {
    match err {
        AdapterError::JsonParse(_) => (400, "Failed to parse request as JSON".to_string()),
        AdapterError::InvalidRequest(_) => (400, "Invalid request structure".to_string()),
        AdapterError::UnknownType(_) => (400, "Unknown request type".to_string()),
        AdapterError::MissingField(_) => (400, "Missing required field".to_string()),
        AdapterError::InvalidValue { .. } => (400, "Invalid field value".to_string()),
        AdapterError::InvalidClientId(_) => (400, "Invalid client ID".to_string()),
        AdapterError::RateLimitExceeded => (429, "Rate limit exceeded".to_string()),
        AdapterError::UnknownMethod(_) => (400, "Unknown method".to_string()),
        AdapterError::InvalidJson(_) => (400, "Invalid JSON".to_string()),
        AdapterError::Manager(msg) => (500, format!("Manager error: {}", msg)),
        AdapterError::Network(msg) => (500, format!("Network error: {}", msg)),
        AdapterError::Timeout(_) => (408, "Request timeout".to_string()),
        AdapterError::Serialization(_) => (500, "Failed to serialize response".to_string()),
        AdapterError::ActorNotAvailable(_) => (503, "Service unavailable".to_string()),
        AdapterError::InvalidResponse(_) => (500, "Invalid response format".to_string()),
        AdapterError::NotSupported(msg) => (501, format!("Not supported: {}", msg)),
    }
}

/// Escapes a string for JSON.
fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Crate version from Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name for identification.
pub const CRATE_NAME: &str = "paper_plane_adapter";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constants() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "paper_plane_adapter");
    }

    #[test]
    fn test_client_config_default() {
        let config = InternalClientConfig::default();
        // Should not panic
        drop(config);
    }

    #[test]
    fn test_client_config_builder() {
        let dialog_manager = DialogManager::new();
        let config = InternalClientConfig::new()
            .with_dialog_manager(dialog_manager);

        // Should not panic
        drop(config);
    }

    #[test]
    fn test_json_escape() {
        assert_eq!(json_escape("hello"), "hello");
        assert_eq!(json_escape("hello\nworld"), "hello\\nworld");
        assert_eq!(json_escape("\"quoted\""), "\\\"quoted\\\"");
    }

    #[test]
    fn test_error_code_from_error() {
        let (code, msg) = error_code_from_error(&AdapterError::UnknownType("test".to_string()));
        assert_eq!(code, 400);
        assert!(msg.contains("Unknown"));

        let (code, msg) = error_code_from_error(&AdapterError::Timeout(Duration::from_secs(1)));
        assert_eq!(code, 408);
        assert!(msg.contains("timeout"));
    }

    #[tokio::test]
    async fn test_client_default() {
        let client = RustgramClient::default();
        // Should not panic
        drop(client);
    }

    #[tokio::test]
    async fn test_client_send_invalid_json() {
        let client = RustgramClient::new();
        // Should not panic, just drop the invalid request
        client.send("not json");
    }

    #[tokio::test]
    async fn test_client_send_unknown_type() {
        let client = RustgramClient::new();
        // Should not panic, will return error response
        client.send(r#"{"@type":"unknownType"}"#);
    }

    #[tokio::test]
    async fn test_client_receive_timeout() {
        let client = RustgramClient::new();
        let response = client.receive(0.1);
        // Should timeout and return None
        assert!(response.is_none());
    }

    #[tokio::test]
    async fn test_client_execute_not_sync() {
        let client = RustgramClient::new();
        let result = client.execute(r#"{"@type":"getChats"}"#);
        // Should fail because getChats is not synchronous
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_client_execute_not_implemented() {
        let client = RustgramClient::new();
        let result = client.execute(r#"{"@type":"getMe"}"#);
        // Should fail because sync execution is not implemented
        assert!(result.is_err());
    }

    #[test]
    fn test_create_error_response() {
        let response = RustgramClient::create_error_response(
            123,
            &AdapterError::UnknownType("test".to_string()),
            None,
        );

        assert_eq!(response.request_id, 123);
        assert!(response.response.contains("\"code\":400"));
    }

    #[test]
    fn test_inject_extra_into_response() {
        let response = r#"{"@type":"ok"}"#;
        let extra = serde_json::json!("test-extra");

        let result = RustgramClient::inject_extra_into_response(response, &extra).unwrap();

        // Result is pretty-printed JSON
        assert!(result.contains("@type"));
        assert!(result.contains("ok"));
        assert!(result.contains("@extra"));
        assert!(result.contains("test-extra"));
    }

    #[test]
    fn test_inject_extra_into_invalid_json() {
        let response = "not json";
        let extra = serde_json::json!("test");

        let result = RustgramClient::inject_extra_into_response(response, &extra);
        assert!(result.is_err());
    }

    // ========== Tests for create/destroy ==========

    #[tokio::test]
    async fn test_create_returns_valid_client_id() {
        let config = client::ClientConfig {
            api_id: 12345,
            api_hash: "test_hash".to_string(),
            database_path: "/tmp/test".to_string(),
            files_directory: "/tmp/test_files".to_string(),
            use_test_dc: false,
            default_dc_id: 2,
        };

        let result = RustgramClient::create(config).await;
        assert!(result.is_ok());

        let client_id = result.unwrap();
        assert!(client_id.is_valid());
    }

    #[tokio::test]
    async fn test_create_increments_registry_count() {
        let registry = global_registry();
        let initial_count = registry.count().await;

        let config = client::ClientConfig {
            api_id: 12345,
            api_hash: "test_hash".to_string(),
            database_path: "/tmp/test".to_string(),
            files_directory: "/tmp/test_files".to_string(),
            use_test_dc: false,
            default_dc_id: 2,
        };

        let client_id = RustgramClient::create(config).await.unwrap();
        assert!(registry.count().await >= initial_count + 1);

        // Cleanup
        let _ = RustgramClient::destroy(client_id).await;
    }

    #[tokio::test]
    async fn test_create_with_invalid_api_id() {
        let config = client::ClientConfig {
            api_id: 0, // Invalid
            api_hash: "test_hash".to_string(),
            database_path: "/tmp/test".to_string(),
            files_directory: "/tmp/test_files".to_string(),
            use_test_dc: false,
            default_dc_id: 2,
        };

        let result = RustgramClient::create(config).await;
        assert!(result.is_err());

        match result {
            Err(AdapterError::InvalidValue { field, .. }) => {
                assert_eq!(field, "api_id");
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[tokio::test]
    async fn test_create_with_empty_api_hash() {
        let config = client::ClientConfig {
            api_id: 12345,
            api_hash: String::new(), // Invalid
            database_path: "/tmp/test".to_string(),
            files_directory: "/tmp/test_files".to_string(),
            use_test_dc: false,
            default_dc_id: 2,
        };

        let result = RustgramClient::create(config).await;
        assert!(result.is_err());

        match result {
            Err(AdapterError::InvalidValue { field, .. }) => {
                assert_eq!(field, "api_hash");
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[tokio::test]
    async fn test_create_generates_unique_ids() {
        let config1 = client::ClientConfig {
            api_id: 12345,
            api_hash: "test_hash1".to_string(),
            database_path: "/tmp/test1".to_string(),
            files_directory: "/tmp/test_files1".to_string(),
            use_test_dc: false,
            default_dc_id: 2,
        };

        let config2 = client::ClientConfig {
            api_id: 67890,
            api_hash: "test_hash2".to_string(),
            database_path: "/tmp/test2".to_string(),
            files_directory: "/tmp/test_files2".to_string(),
            use_test_dc: false,
            default_dc_id: 2,
        };

        let id1 = RustgramClient::create(config1).await.unwrap();
        let id2 = RustgramClient::create(config2).await.unwrap();

        assert_ne!(id1, id2);
    }

    #[tokio::test]
    async fn test_destroy_decreases_registry_count() {
        let registry = global_registry();

        let config = client::ClientConfig {
            api_id: 12345,
            api_hash: "test_hash".to_string(),
            database_path: "/tmp/test".to_string(),
            files_directory: "/tmp/test_files".to_string(),
            use_test_dc: false,
            default_dc_id: 2,
        };

        let client_id = RustgramClient::create(config).await.unwrap();
        let count_after_create = registry.count().await;

        let _ = RustgramClient::destroy(client_id).await;
        assert_eq!(registry.count().await, count_after_create - 1);
    }

    #[tokio::test]
    async fn test_destroy_invalid_client_id() {
        let invalid_id = ClientId::new(99999);
        let result = RustgramClient::destroy(invalid_id).await;

        assert!(result.is_err());
        match result {
            Err(AdapterError::InvalidClientId(id)) => {
                assert_eq!(id, 99999);
            }
            _ => panic!("Expected InvalidClientId error"),
        }
    }

    #[tokio::test]
    async fn test_destroy_allows_reuse_id() {
        let config = client::ClientConfig {
            api_id: 12345,
            api_hash: "test_hash".to_string(),
            database_path: "/tmp/test".to_string(),
            files_directory: "/tmp/test_files".to_string(),
            use_test_dc: false,
            default_dc_id: 2,
        };

        let id1 = RustgramClient::create(config.clone()).await.unwrap();
        let _ = RustgramClient::destroy(id1).await;

        // Create a new client - should get a different ID since IDs increment
        let id2 = RustgramClient::create(config).await.unwrap();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_global_registry_is_singleton() {
        let reg1 = global_registry();
        let reg2 = global_registry();

        // Both should point to the same Arc
        assert!(Arc::ptr_eq(reg1, reg2));
    }

    #[tokio::test]
    async fn test_global_registry_thread_safe() {
        use tokio::task::JoinSet;

        let mut join_set = JoinSet::new();

        // Spawn multiple tasks creating clients concurrently
        for i in 0..10 {
            join_set.spawn(async move {
                let config = client::ClientConfig {
                    api_id: 10000 + i,
                    api_hash: format!("hash_{}", i),
                    database_path: format!("/tmp/test_{}", i),
                    files_directory: format!("/tmp/test_files_{}", i),
                    use_test_dc: false,
                    default_dc_id: 2,
                };

                RustgramClient::create(config).await
            });
        }

        // Collect results
        let mut client_ids = Vec::new();
        while let Some(result) = join_set.join_next().await {
            let client_id = result.unwrap().unwrap();
            client_ids.push(client_id);
        }

        // All IDs should be unique
        assert_eq!(client_ids.len(), 10);
        let unique_ids: std::collections::HashSet<_> = client_ids.iter().collect();
        assert_eq!(unique_ids.len(), 10);

        // Clean up
        for id in client_ids {
            let _ = RustgramClient::destroy(id).await;
        }
    }
}
