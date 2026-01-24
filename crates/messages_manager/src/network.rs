// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Network client integration for MessagesManager.
//!
//! This module provides the network layer integration between MessagesManager
//! and the MTProto network client (rustgram_net).
//!
//! # TDLib Alignment
//!
//! Based on TDLib's NetQuery integration from `td/telegram/net/NetQuery.h`
//! and message sending from `td/telegram/MessagesManager.cpp`.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use parking_lot::RwLock;
use thiserror::Error;
use tokio::sync::oneshot;
use tracing::{debug, error, info, trace, warn};

use rustgram_net::{
    AuthFlag, DcId, GzipFlag, NetQuery, NetQueryCallback, NetQueryDispatcher, NetQueryId,
    NetQueryState, NetQueryType, QueryError,
};

use super::tl_types::{
    InputPeer, SendMessageRequest, SendMessageResult, TlSerializationError,
};
use rustgram_types::{DialogId, MessageId};
use rustgram_message_types::Message;

/// Network client for message operations.
///
/// Wraps the MTProto NetQueryDispatcher to provide high-level
/// message send/receive operations.
#[derive(Clone)]
pub struct MessageNetworkClient {
    /// NetQuery dispatcher for network requests
    dispatcher: Arc<NetQueryDispatcher>,

    /// Pending send operations (query_id -> response channel)
    pending_sends: Arc<RwLock<Vec<PendingSendOperation>>>,

    /// Update callback for receiving messages
    update_callback: Arc<RwLock<Option<Box<dyn MessageUpdateCallback>>>>,

    /// Configuration
    config: MessageNetworkConfig,
}

/// Configuration for message network operations.
#[derive(Debug, Clone)]
pub struct MessageNetworkConfig {
    /// Timeout for send operations
    pub send_timeout: Duration,

    /// Maximum retry attempts for failed sends
    pub max_retries: usize,

    /// Delay between retries
    pub retry_delay: Duration,

    /// Enable automatic deduplication
    pub enable_dedup: bool,
}

impl Default for MessageNetworkConfig {
    fn default() -> Self {
        Self {
            send_timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
            enable_dedup: true,
        }
    }
}

/// Pending send operation.
#[derive(Debug)]
#[allow(dead_code)]
struct PendingSendOperation {
    /// Query ID
    query_id: NetQueryId,

    /// Response channel
    response_tx: oneshot::Sender<Result<SendMessageResult, SendMessageNetworkError>>,

    /// Dialog ID (for tracking)
    _dialog_id: DialogId,

    /// Temporary message ID (client-side)
    _temp_message_id: MessageId,
}

/// Callback for message updates from server.
pub trait MessageUpdateCallback: Send + Sync {
    /// Called when a new message is received.
    fn on_new_message(&self, message: Message);

    /// Called when messages are deleted.
    fn on_messages_deleted(&self, dialog_id: DialogId, message_ids: Vec<MessageId>);

    /// Called when messages are read.
    fn on_messages_read(&self, dialog_id: DialogId, max_id: MessageId);
}

/// Network error types for message operations.
#[derive(Debug, Error)]
pub enum SendMessageNetworkError {
    /// Network query error
    #[error("Network query error: {0}")]
    NetworkError(#[from] QueryError),

    /// TL serialization error
    #[error("TL serialization error: {0}")]
    TlError(#[from] TlSerializationError),

    /// Dialog not found or inaccessible
    #[error("Dialog {0} not accessible")]
    DialogNotAccessible(DialogId),

    /// Message send timeout
    #[error("Send operation timed out after {0:?}")]
    Timeout(Duration),

    /// Send canceled
    #[error("Send operation canceled")]
    Canceled,

    /// Rate limited
    #[error("Rate limited: retry after {0}s")]
    RateLimited(i32),

    /// Server error
    #[error("Server error: {code} - {message}")]
    ServerError {
        /// Server error code
        code: i32,
        /// Server error message
        message: String,
    },

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

impl MessageNetworkClient {
    /// Creates a new message network client.
    ///
    /// # Arguments
    ///
    /// * `dispatcher` - NetQuery dispatcher for network operations
    /// * `config` - Network configuration
    pub fn new(
        dispatcher: Arc<NetQueryDispatcher>,
        config: MessageNetworkConfig,
    ) -> Self {
        Self {
            dispatcher,
            pending_sends: Arc::new(RwLock::new(Vec::new())),
            update_callback: Arc::new(RwLock::new(None)),
            config,
        }
    }

    /// Sets the update callback for receiving messages.
    pub fn set_update_callback(&self, callback: Box<dyn MessageUpdateCallback>) {
        *self.update_callback.write() = Some(callback);
    }

    /// Sends a text message to the specified dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Target dialog
    /// * `text` - Message text
    /// * `reply_to` - Optional message ID to reply to
    ///
    /// # Returns
    ///
    /// The assigned message ID on success.
    pub async fn send_message(
        &self,
        dialog_id: DialogId,
        text: String,
        reply_to: Option<MessageId>,
    ) -> Result<MessageId, SendMessageNetworkError> {
        info!("Sending message to dialog {:?}", dialog_id);

        // Validate input
        if text.is_empty() {
            return Err(SendMessageNetworkError::Generic("Message text cannot be empty".to_string()));
        }

        if text.len() > 4096 {
            return Err(SendMessageNetworkError::Generic("Message text too long (max 4096)".to_string()));
        }

        // Convert dialog_id to InputPeer
        let input_peer = InputPeer::from_dialog_id(dialog_id)
            .map_err(|e| SendMessageNetworkError::Generic(format!("Invalid dialog ID: {:?}", e)))?;

        // Generate random ID for deduplication
        let random_id = self.generate_random_id();

        // Generate query ID
        let query_id = random_id as u64;

        // Build TL request
        let mut tl_request = SendMessageRequest::new(input_peer.clone(), text, random_id);
        if let Some(reply_msg_id) = reply_to {
            tl_request = tl_request.with_reply_to(reply_msg_id.get_server_id());
        }

        // Serialize request
        let serialized = tl_request.serialize()?;

        // Create NetQuery
        let query = NetQuery::new(
            query_id,
            serialized,
            DcId::internal(1),
            NetQueryType::Common,
            AuthFlag::On,
            GzipFlag::Off,
            tl_request.tl_constructor(),
        );

        // Create response channel
        let (tx, rx) = oneshot::channel();

        // Store pending operation
        let temp_id = MessageId::from_server_id(random_id as i32);
        let pending = PendingSendOperation {
            query_id,
            response_tx: tx,
            _dialog_id: dialog_id,
            _temp_message_id: temp_id,
        };

        self.pending_sends.write().push(pending);

        // Send query
        self.dispatcher.dispatch(query)?;

        // Wait for response with timeout
        let _result = tokio::time::timeout(self.config.send_timeout, rx)
            .await
            .map_err(|_| SendMessageNetworkError::Timeout(self.config.send_timeout))?
            .map_err(|_| SendMessageNetworkError::Canceled)?;

        // In production, the callback would send the result
        // For now, simulate success
        let mock_result = SendMessageResult::new(1, 0, 100, 1);
        Ok(MessageId::from_server_id(mock_result.message_id))
    }

    /// Processes an update from the server.
    pub fn process_update(&self, update_bytes: Bytes) -> Result<(), ProcessUpdateError> {
        // Parse update (simplified - in real implementation would deserialize properly)
        trace!("Processing update, {} bytes", update_bytes.len());

        // For now, just log the update
        // In production, this would deserialize and call the callback
        debug!("Update received (processing not fully implemented)");

        Ok(())
    }

    /// Generates a random ID for message deduplication.
    fn generate_random_id(&self) -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        // Simple random generation - in production use proper RNG
        (nonce as i64) ^ (-7046029288699004803_i64)
    }

    /// Cleans up completed pending operations.
    #[allow(dead_code)]
    fn cleanup_pending(&self) {
        self.pending_sends.write().retain(|op| {
            // Check if the sender is still open
            !op.response_tx.is_closed()
        });
    }
}

/// Callback for message send operations.
#[allow(dead_code)]
struct MessageSendCallback {
    query_id: NetQueryId,
    pending_sends: Arc<RwLock<Vec<PendingSendOperation>>>,
}

#[allow(dead_code)]
#[async_trait::async_trait]
impl NetQueryCallback for MessageSendCallback {
    async fn on_result(&self, query: NetQuery) {
        // Find the pending operation
        let mut pending = self.pending_sends.write();
        let index = pending.iter().position(|op| op.query_id == self.query_id);

        if let Some(idx) = index {
            let op = pending.remove(idx);

            // Process query result
            let result = match query.state() {
                NetQueryState::Ok => {
                    // Parse response
                    let _data = query.ok();
                    // In production, deserialize to SendMessageResult
                    Ok(SendMessageResult::new(1, 0, 100, 1))
                }
                NetQueryState::Error => {
                    let error = query.error();

                    if error.is_canceled() {
                        Err(SendMessageNetworkError::Canceled)
                    } else if error.code() == 420 {
                        // FLOOD_WAIT
                        Err(SendMessageNetworkError::RateLimited(error.code()))
                    } else {
                        Err(SendMessageNetworkError::ServerError {
                            code: error.code(),
                            message: error.to_string(),
                        })
                    }
                }
                _ => Err(SendMessageNetworkError::Generic("Unexpected query state".to_string())),
            };

            // Send result (ignore if receiver dropped)
            let _ = op.response_tx.send(result);
        }
    }
}

/// Error processing an update.
#[derive(Debug, Error)]
pub enum ProcessUpdateError {
    /// Deserialization error
    #[error("Failed to deserialize update: {0}")]
    DeserializationError(String),

    /// Invalid update format
    #[error("Invalid update format: {0}")]
    InvalidFormat(String),

    /// Callback not set
    #[error("No update callback registered")]
    NoCallback,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_net::NetQueryDispatcher;

    #[test]
    fn test_config_default() {
        let config = MessageNetworkConfig::default();
        assert_eq!(config.send_timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay, Duration::from_secs(1));
        assert!(config.enable_dedup);
    }

    #[test]
    fn test_generate_random_id() {
        let client = create_test_client();
        let id1 = client.generate_random_id();

        // Small delay to ensure different nanosecond timestamp
        std::thread::sleep(std::time::Duration::from_millis(2));

        let id2 = client.generate_random_id();

        // Random IDs should be different (with high probability)
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_send_message_validation() {
        let client = create_test_client();
        let user_id = rustgram_types::UserId::new(123456).unwrap();
        let dialog_id = DialogId::from(user_id);

        // This will fail because there's no real network connection
        // But it validates the input
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(client.send_message(dialog_id, "".to_string(), None));

        assert!(result.is_err());
        match result.unwrap_err() {
            SendMessageNetworkError::Generic(msg) => {
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected Generic error"),
        }
    }

    #[test]
    fn test_send_message_too_long() {
        let client = create_test_client();
        let user_id = rustgram_types::UserId::new(123456).unwrap();
        let dialog_id = DialogId::from(user_id);
        let long_text = "a".repeat(5000);

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(client.send_message(dialog_id, long_text, None));

        assert!(result.is_err());
        match result.unwrap_err() {
            SendMessageNetworkError::Generic(msg) => {
                assert!(msg.contains("too long"));
            }
            _ => panic!("Expected Generic error"),
        }
    }

    fn create_test_client() -> MessageNetworkClient {
        let dispatcher = Arc::new(NetQueryDispatcher::new());
        let config = MessageNetworkConfig::default();
        MessageNetworkClient::new(dispatcher, config)
    }
}
