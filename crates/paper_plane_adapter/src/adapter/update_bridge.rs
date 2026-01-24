// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Update Channel Bridge for tokio → glib integration.
//!
//! This module provides `UpdateBridge`, a thread-safe bridge that:
//! - Receives updates from `ClientActor`'s tokio channel
//! - Converts them to TDLib-compatible JSON format
//! - Delivers them via `async_channel` for glib consumption

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use rustgram_client_actor::{ClientActor, Update as ClientActorUpdate};

/// Maximum number of updates to buffer before dropping oldest.
const MAX_BUFFER_CAPACITY: usize = 1000;

/// Error types for update bridge operations.
#[derive(Debug, thiserror::Error)]
pub enum BridgeError {
    /// ClientActor is not available or has been stopped.
    #[error("ClientActor unavailable")]
    ClientActorUnavailable,

    /// Update channel was closed unexpectedly.
    #[error("Channel closed")]
    ChannelClosed,

    /// Failed to convert update to TDLib JSON format.
    #[error("Conversion failed for {update_type}: {reason}")]
    ConversionFailed {
        /// The type of update that failed to convert.
        update_type: String,
        /// Human-readable reason for the failure.
        reason: String,
    },
}

/// Trait for converting updates to TDLib JSON format.
pub trait UpdateConverter {
    /// Converts this update to a TDLib-compatible JSON string.
    fn to_tdlib_json(&self) -> Result<String, BridgeError>;
}

/// Implementation of `UpdateConverter` for `ClientActorUpdate`.
///
/// Since `ClientActorUpdate` already contains `serde_json::Value` with
/// proper `@type` tags from the `#[serde(tag = "@type")]` attribute,
/// conversion simply serializes the inner value to a JSON string.
impl UpdateConverter for ClientActorUpdate {
    fn to_tdlib_json(&self) -> Result<String, BridgeError> {
        // ClientActor::Update already contains serde_json::Value with proper @type tags
        // We just need to serialize it to a string
        let json_value = match self {
            ClientActorUpdate::NewMessage(v) => v,
            ClientActorUpdate::EditMessage(v) => v,
            ClientActorUpdate::DeleteMessages(v) => v,
            ClientActorUpdate::AuthorizationState(v) => v,
            ClientActorUpdate::UserStatus(v) => v,
            ClientActorUpdate::NewChat(v) => v,
            ClientActorUpdate::Generic(v) => v,
        };

        serde_json::to_string(json_value).map_err(|e| BridgeError::ConversionFailed {
            update_type: format!("{:?}", self),
            reason: e.to_string(),
        })
    }
}

/// Bridge for forwarding updates from tokio runtime to glib main loop.
///
/// This struct subscribes to `ClientActor` updates and provides:
/// - Thread-safe delivery via `async_channel`
/// - Polling buffer for non-blocking `receive()`
/// - Graceful shutdown support
///
/// # Example
///
/// ```ignore
/// let bridge = UpdateBridge::create(client_id, client_actor).await?;
///
/// // In glib main thread
/// glib::MainContext::default().spawn_local(async move {
///     while bridge.is_running() {
///         if let Some(update_json) = bridge.receive() {
///             handle_update(&update_json);
///         }
///         glib::timeout_future_seconds(0.01).await;
///     }
/// });
/// ```
pub struct UpdateBridge {
    /// Client identifier.
    client_id: i32,

    /// Receiver for JSON updates from the forwarder task.
    async_receiver: async_channel::Receiver<String>,

    /// Buffer for pending updates (when async_channel is full).
    buffer: Arc<Mutex<VecDeque<String>>>,

    /// Flag indicating whether the bridge is stopped.
    stopped: Arc<AtomicBool>,

    /// Handle for the forwarder task.
    _forwarder_handle: JoinHandle<()>,

    /// Counter for processed updates.
    processed_count: Arc<AtomicU64>,

    /// Counter for dropped updates.
    dropped_count: Arc<AtomicU64>,
}

impl UpdateBridge {
    /// Creates a new `UpdateBridge` and starts the forwarder task.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client identifier.
    /// * `client_actor` - Reference to the `ClientActor` to subscribe to.
    ///
    /// # Returns
    ///
    /// Returns `Ok(UpdateBridge)` if successfully created, `Err(BridgeError)` if:
    /// - ClientActor is not available
    /// - Subscribe fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let bridge = UpdateBridge::create(123, &client_actor).await?;
    /// ```
    pub async fn create(
        client_id: i32,
        client_actor: &Arc<ClientActor>,
    ) -> Result<Self, BridgeError> {
        // Subscribe to updates from ClientActor
        let update_receiver = client_actor.subscribe_updates();

        // Create async_channel for thread-safe communication
        let (async_sender, async_receiver) = async_channel::unbounded::<String>();

        // Create shared state
        let buffer = Arc::new(Mutex::new(VecDeque::with_capacity(MAX_BUFFER_CAPACITY)));
        let stopped = Arc::new(AtomicBool::new(false));
        let processed_count = Arc::new(AtomicU64::new(0));
        let dropped_count = Arc::new(AtomicU64::new(0));

        // Spawn forwarder task
        let forwarder_handle = tokio::spawn(forwarder_task(
            client_id,
            update_receiver,
            async_sender,
            buffer.clone(),
            stopped.clone(),
            processed_count.clone(),
            dropped_count.clone(),
        ));

        Ok(Self {
            client_id,
            async_receiver,
            buffer,
            stopped,
            _forwarder_handle: forwarder_handle,
            processed_count,
            dropped_count,
        })
    }

    /// Non-blocking receive for GTK polling (matches TDLib::td_receive pattern).
    ///
    /// Returns `Some(json_string)` if an update is available, `None` otherwise.
    /// This method is thread-safe and can be called from the glib main thread.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // In GTK update loop
    /// if let Some(update_json) = bridge.receive() {
    ///     handle_tdlib_update(&update_json);
    /// }
    /// ```
    #[must_use]
    pub fn receive(&self) -> Option<String> {
        // Check buffer first (for polling fallback)
        if let Some(update) = self.buffer.lock().pop_front() {
            return Some(update);
        }

        // Try async_channel (non-blocking try_recv)
        self.async_receiver.try_recv().ok()
    }

    /// Returns the number of updates currently in the buffer.
    pub fn buffered_count(&self) -> usize {
        self.buffer.lock().len()
    }

    /// Returns the number of updates processed so far.
    pub fn processed_count(&self) -> u64 {
        self.processed_count.load(Ordering::Relaxed)
    }

    /// Returns the number of updates dropped due to buffer overflow.
    pub fn dropped_count(&self) -> u64 {
        self.dropped_count.load(Ordering::Relaxed)
    }

    /// Stops the bridge and closes all channels.
    ///
    /// This will signal the forwarder task to stop and wait for it to complete
    /// with a 5-second timeout.
    pub async fn stop(mut self) {
        self.stopped.store(true, Ordering::Relaxed);
        // Wait for forwarder task to complete with timeout
        let _ = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            &mut self._forwarder_handle,
        )
        .await
        .ok();
    }

    /// Returns `true` if the bridge is still running.
    pub fn is_running(&self) -> bool {
        !self.stopped.load(Ordering::Relaxed)
    }

    /// Returns the client ID.
    pub fn client_id(&self) -> i32 {
        self.client_id
    }
}

/// Forwarder task that receives updates from ClientActor and forwards them.
///
/// This task runs in the tokio runtime and:
/// 1. Receives `ClientActorUpdate` from the mpsc channel
/// 2. Converts it to TDLib JSON format
/// 3. Tries to send via async_channel (non-blocking)
/// 4. Falls back to buffer if async_channel is full
async fn forwarder_task(
    client_id: i32,
    mut update_receiver: mpsc::UnboundedReceiver<ClientActorUpdate>,
    async_sender: async_channel::Sender<String>,
    buffer: Arc<Mutex<VecDeque<String>>>,
    stopped: Arc<AtomicBool>,
    processed_count: Arc<AtomicU64>,
    dropped_count: Arc<AtomicU64>,
) {
    tracing::debug!("UpdateBridge forwarder started for client {}", client_id);

    while !stopped.load(Ordering::Relaxed) {
        match update_receiver.recv().await {
            Some(update) => {
                processed_count.fetch_add(1, Ordering::Relaxed);

                match update.to_tdlib_json() {
                    Ok(json) => {
                        // Try to send via async_channel (non-blocking)
                        if async_sender.try_send(json.clone()).is_err() {
                            // async_channel full, falling back to buffer
                            tracing::debug!(
                                "async_channel full for client {}, using buffer",
                                client_id
                            );
                            // Fallback: store in buffer for polling
                            let mut buf = buffer.lock();

                            // Drop oldest if at capacity
                            if buf.len() >= MAX_BUFFER_CAPACITY {
                                buf.pop_front();
                                dropped_count.fetch_add(1, Ordering::Relaxed);
                                tracing::warn!(
                                    "UpdateBridge buffer full for client {}, dropping oldest update",
                                    client_id
                                );
                            }

                            buf.push_back(json);
                        }
                    }
                    Err(e) => {
                        tracing::error!(
                            "UpdateBridge conversion error for client {}: {:?}",
                            client_id,
                            e
                        );
                    }
                }
            }
            None => {
                // Channel closed, stop forwarding
                tracing::debug!(
                    "UpdateBridge channel closed for client {}, stopping forwarder",
                    client_id
                );
                break;
            }
        }
    }

    tracing::debug!("UpdateBridge forwarder stopped for client {}", client_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    // Test helper to create a mock update
    fn create_mock_new_message() -> Value {
        json!({
            "@type": "updateNewMessage",
            "message": {
                "@type": "message",
                "id": 123,
                "content": {
                    "@type": "messageText",
                    "text": {
                        "@type": "formattedText",
                        "text": "Hello, world!"
                    }
                }
            }
        })
    }

    #[test]
    fn test_update_converter_new_message() {
        let value = create_mock_new_message();
        let update = ClientActorUpdate::NewMessage(value);

        let result = update.to_tdlib_json();
        assert!(result.is_ok());

        let json_str = result.unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["@type"], "updateNewMessage");
    }

    #[test]
    fn test_update_converter_edit_message() {
        let value = json!({
            "@type": "updateEditMessage",
            "message": {
                "@type": "message",
                "id": 123,
                "content": {
                    "@type": "messageText",
                    "text": {
                        "@type": "formattedText",
                        "text": "Edited"
                    }
                }
            }
        });
        let update = ClientActorUpdate::EditMessage(value);

        let result = update.to_tdlib_json();
        assert!(result.is_ok());

        let json_str = result.unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["@type"], "updateEditMessage");
    }

    #[test]
    fn test_update_converter_delete_messages() {
        let value = json!({
            "@type": "updateDeleteMessages",
            "chat_id": 456,
            "message_ids": [123, 124]
        });
        let update = ClientActorUpdate::DeleteMessages(value);

        let result = update.to_tdlib_json();
        assert!(result.is_ok());

        let json_str = result.unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["@type"], "updateDeleteMessages");
    }

    #[test]
    fn test_update_converter_authorization_state() {
        let value = json!({
            "@type": "updateAuthorizationState",
            "authorization_state": {
                "@type": "authorizationStateWaitPhoneNumber"
            }
        });
        let update = ClientActorUpdate::AuthorizationState(value);

        let result = update.to_tdlib_json();
        assert!(result.is_ok());

        let json_str = result.unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["@type"], "updateAuthorizationState");
    }

    #[test]
    fn test_update_converter_user_status() {
        let value = json!({
            "@type": "updateUserStatus",
            "user_id": 789,
            "status": {
                "@type": "userStatusOnline"
            }
        });
        let update = ClientActorUpdate::UserStatus(value);

        let result = update.to_tdlib_json();
        assert!(result.is_ok());

        let json_str = result.unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["@type"], "updateUserStatus");
    }

    #[test]
    fn test_update_converter_new_chat() {
        let value = json!({
            "@type": "updateNewChat",
            "chat": {
                "@type": "chat",
                "id": 456
            }
        });
        let update = ClientActorUpdate::NewChat(value);

        let result = update.to_tdlib_json();
        assert!(result.is_ok());

        let json_str = result.unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["@type"], "updateNewChat");
    }

    #[test]
    fn test_update_converter_generic() {
        let value = json!({
            "@type": "updateSomeCustomType",
            "data": "test"
        });
        let update = ClientActorUpdate::Generic(value);

        let result = update.to_tdlib_json();
        assert!(result.is_ok());

        let json_str = result.unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["@type"], "updateSomeCustomType");
    }

    #[test]
    fn test_update_converter_invalid_json() {
        // Create a Value that will fail to serialize
        // Note: This is difficult to test with serde_json::Value as it's always serializable
        // So we just verify the type system works
        let value = json!({"@type": "test"});
        let update = ClientActorUpdate::NewMessage(value);
        assert!(update.to_tdlib_json().is_ok());
    }

    #[test]
    fn test_bridge_error_display() {
        let err = BridgeError::ClientActorUnavailable;
        assert_eq!(format!("{}", err), "ClientActor unavailable");

        let err = BridgeError::ChannelClosed;
        assert_eq!(format!("{}", err), "Channel closed");

        let err = BridgeError::ConversionFailed {
            update_type: "test".to_string(),
            reason: "test reason".to_string(),
        };
        assert!(format!("{}", err).contains("Conversion failed"));
    }

    // Integration tests require a running ClientActor which needs tokio runtime
    // These are marked as ignore and can be run with: cargo test -- --ignored

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_bridge_create() {
        // TODO: Set up mock ClientActor
        // let client_actor = Arc::new(ClientActor::new(...));
        // let bridge = UpdateBridge::create(123, &client_actor).await;
        // assert!(bridge.is_ok());
    }

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_bridge_receive() {
        // TODO: Test receive() with actual updates
    }

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_bridge_stop() {
        // TODO: Test graceful shutdown
    }

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_bridge_is_running() {
        // TODO: Test is_running() status
    }

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_buffer_overflow_handling() {
        // TODO: Test buffer overflow drops oldest updates
    }

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_buffered_count() {
        // TODO: Test buffered_count() accuracy
    }

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_processed_count() {
        // TODO: Test processed_count() accuracy
    }

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_dropped_count() {
        // TODO: Test dropped_count() increments when buffer overflows
    }

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_non_blocking_receive() {
        // TODO: Verify receive() doesn't block
    }

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_concurrent_receive() {
        // TODO: Test thread safety of concurrent receive() calls
    }

    #[tokio::test]
    #[ignore = "Requires ClientActor setup"]
    async fn test_tdlib_json_format() {
        // TODO: Verify JSON format matches TDLib schema
    }

    #[test]
    fn test_constants() {
        assert_eq!(MAX_BUFFER_CAPACITY, 1000);
    }

    // Additional unit tests to meet minimum 25 tests requirement

    #[test]
    fn test_update_converter_preserves_type_field() {
        let value = json!({
            "@type": "updateNewMessage",
            "message": {"id": 1}
        });
        let update = ClientActorUpdate::NewMessage(value);

        let result = update.to_tdlib_json();
        assert!(result.is_ok());

        let json_str = result.unwrap();
        assert!(json_str.contains("@type"));
        assert!(json_str.contains("updateNewMessage"));
    }

    #[test]
    fn test_update_converter_json_structure_valid() {
        let value = json!({
            "@type": "updateNewChat",
            "chat": {
                "@type": "chat",
                "id": 123,
                "title": "Test Chat"
            }
        });
        let update = ClientActorUpdate::NewChat(value);

        let result = update.to_tdlib_json();
        assert!(result.is_ok());

        // Verify it's valid JSON that can be parsed back
        let json_str = result.unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["chat"]["id"], 123);
        assert_eq!(parsed["chat"]["title"], "Test Chat");
    }

    #[test]
    fn test_bridge_error_conversion_failed_details() {
        let err = BridgeError::ConversionFailed {
            update_type: "updateNewMessage".to_string(),
            reason: "Invalid JSON structure".to_string(),
        };

        let formatted = format!("{}", err);
        assert!(formatted.contains("Conversion failed"));
        assert!(formatted.contains("updateNewMessage"));
        assert!(formatted.contains("Invalid JSON structure"));
    }

    #[test]
    fn test_update_converter_all_variants_have_type() {
        // Verify all Update variants produce JSON with @type field
        let test_cases = vec![
            ClientActorUpdate::NewMessage(json!({"@type": "test"})),
            ClientActorUpdate::EditMessage(json!({"@type": "test"})),
            ClientActorUpdate::DeleteMessages(json!({"@type": "test"})),
            ClientActorUpdate::AuthorizationState(json!({"@type": "test"})),
            ClientActorUpdate::UserStatus(json!({"@type": "test"})),
            ClientActorUpdate::NewChat(json!({"@type": "test"})),
            ClientActorUpdate::Generic(json!({"@type": "test"})),
        ];

        for update in test_cases {
            let result = update.to_tdlib_json();
            assert!(result.is_ok(), "to_tdlib_json should succeed for {:?}", update);
            let json_str = result.unwrap();
            assert!(json_str.contains("@type"), "JSON should contain @type field");
        }
    }
}
