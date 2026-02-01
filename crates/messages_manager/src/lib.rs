// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Messages Manager - Telegram message send/receive operations.
//!
//! This crate provides message sending and receiving functionality for
//! the Rustgram Telegram client. It integrates with the MTProto network
//! layer to send messages and process server updates.
//!
//! # Architecture
//!
//! The MessagesManager is divided into three main modules:
//!
//! - **tl_types** - TL schema types for MTProto communication
//! - **network** - Network client integration
//! - **send** - Message send operations
//! - **receive** - Message receive operations
//!
//! # TDLib Alignment
//!
//! Based on TDLib's MessagesManager from `td/telegram/MessagesManager.h`
//! and `td/telegram/MessagesManager.cpp` (39,000+ lines).
//!
//! Phase 1 implements ~15% of TDLib's functionality:
//! - Text message sending
//! - Incoming message processing
//! - Basic reply-to support
//! - No media, no editing, no forwarding
//!
//! # Example
//!
//! ```no_run
//! use rustgram_messages_manager::{MessagesManager, MessageNetworkConfig};
//! use rustgram_types::UserId;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create manager with network client
//! # let network_client = Arc::new(rustgram_messages_manager::MessageNetworkClient::new(
//! #     std::sync::Arc::new(rustgram_net::NetQueryDispatcher::new()),
//! #     rustgram_messages_manager::MessageNetworkConfig::default()
//! # ));
//! # let config = rustgram_messages_manager::MessagesManagerConfig::default();
//! let manager = MessagesManager::new(network_client, config);
//!
//! // Send a message
//! let user_id = UserId::new(123456)?;
//! let message_id = manager.send_text(user_id.into(), "Hello, world!".to_string(), None).await?;
//!
//! println!("Message sent with ID: {:?}", message_id);
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod tl_types;
pub mod network;

use std::sync::Arc;
use std::time::Duration;

use rustgram_types::{DialogId, MessageId};
use rustgram_message_types::{Message, MessageValidationError};
use tracing::{debug, error, info, warn};

pub use tl_types::{
    constructors, InputPeer, MessageData, MessageEntity, MessageFwdHeader, MessageReplyHeader,
    Peer, SendMessageRequest, SendMessageResult, TlSerializationError, Update, UpdateDeleteMessages,
    UpdateEditMessage, UpdateNewMessage, UpdateReadHistory, UpdateShortChatMessage,
    UpdateShortMessage, Updates, User, Chat,
};

pub use network::{
    MessageNetworkClient, MessageNetworkConfig, MessageUpdateCallback, ProcessUpdateError,
    SendMessageNetworkError,
};

// ============================================================================
// Errors
// ============================================================================

/// Error types for message operations.
#[derive(Debug, thiserror::Error)]
pub enum MessagesManagerError {
    /// Network error
    #[error("Network error: {0}")]
    Network(#[from] SendMessageNetworkError),

    /// Validation error
    #[error("Message validation error: {0}")]
    Validation(#[from] MessageValidationError),

    /// Database error
    #[error("Database error: {0}")]
    Database(String),

    /// Dialog not found
    #[error("Dialog not found: {0:?}")]
    DialogNotFound(DialogId),

    /// Dialog not accessible (no write permission)
    #[error("Dialog not accessible: {0:?}")]
    DialogNotAccessible(DialogId),

    /// Message not found
    #[error("Message not found: {0:?}, {1:?}")]
    MessageNotFound(DialogId, MessageId),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

// ============================================================================
// MessagesManager
// ============================================================================

/// Main manager for message operations.
///
/// Provides high-level API for sending and receiving messages.
/// Integrates with the network layer, database, and update handlers.
pub struct MessagesManager {
    /// Network client for MTProto operations
    network_client: Arc<MessageNetworkClient>,

    /// Configuration
    config: MessagesManagerConfig,
}

/// Configuration for MessagesManager.
#[derive(Debug, Clone)]
pub struct MessagesManagerConfig {
    /// Maximum message length
    pub max_message_length: usize,

    /// Enable automatic deduplication
    pub enable_dedup: bool,

    /// Pending sends cleanup interval
    pub cleanup_interval: Duration,
}

impl Default for MessagesManagerConfig {
    fn default() -> Self {
        Self {
            max_message_length: 4096,
            enable_dedup: true,
            cleanup_interval: Duration::from_secs(60),
        }
    }
}

impl MessagesManager {
    /// Creates a new MessagesManager.
    ///
    /// # Arguments
    ///
    /// * `network_client` - Network client for MTProto operations
    /// * `config` - Manager configuration
    pub fn new(
        network_client: Arc<MessageNetworkClient>,
        config: MessagesManagerConfig,
    ) -> Self {
        Self {
            network_client,
            config,
        }
    }

    /// Sends a text message to a dialog.
    ///
    /// This is the main entry point for sending messages.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Target dialog
    /// * `text` - Message text (1-4096 characters)
    /// * `reply_to` - Optional message ID to reply to
    ///
    /// # Returns
    ///
    /// The assigned message ID on success.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The dialog doesn't exist or isn't accessible
    /// - The message text is too long or empty
    /// - Network operation fails
    /// - Rate limit is exceeded
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rustgram_messages_manager::MessagesManager;
    /// # use rustgram_types::{DialogId, UserId};
    /// # async fn example(manager: &MessagesManager) -> Result<(), Box<dyn std::error::Error>> {
    /// let user_id = UserId::new(123456)?;
    /// let dialog_id = DialogId::from(user_id);
    /// let message_id = manager.send_text(dialog_id, "Hello!".to_string(), None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_text(
        &self,
        dialog_id: DialogId,
        text: String,
        reply_to: Option<MessageId>,
    ) -> Result<MessageId, MessagesManagerError> {
        info!("Sending text message to dialog {:?}", dialog_id);

        // Validate text length
        if text.is_empty() {
            return Err(MessagesManagerError::Validation(
                MessageValidationError::InvalidContent("Message text cannot be empty".to_string()),
            ));
        }

        if text.len() > self.config.max_message_length {
            return Err(MessagesManagerError::Validation(
                MessageValidationError::InvalidContent(format!(
                    "Message text too long: {} > {}",
                    text.len(),
                    self.config.max_message_length
                )),
            ));
        }

        // Send via network client
        let result = self
            .network_client
            .send_message(dialog_id, text, reply_to)
            .await?;

        debug!("Message sent successfully with ID {:?}", result);
        Ok(result)
    }

    /// Processes an incoming update from the server.
    ///
    /// This method handles:
    /// - New messages
    /// - Message edits
    /// - Message deletions
    /// - Read receipts
    ///
    /// # Arguments
    ///
    /// * `update` - The update to process
    ///
    /// # Returns
    ///
    /// Ok(()) if the update was processed successfully.
    pub fn process_update(&self, update: Update) -> Result<(), MessagesManagerError> {
        match update {
            Update::NewMessage(new_msg) => {
                self.process_new_message(new_msg)?;
            }
            Update::EditMessage(edit_msg) => {
                self.process_edit_message(edit_msg)?;
            }
            Update::DeleteMessages(del_msg) => {
                self.process_delete_messages(del_msg)?;
            }
            Update::ReadHistory(read_msg) => {
                self.process_read_history(read_msg)?;
            }
            _ => {
                debug!("Ignoring unhandled update type");
            }
        }
        Ok(())
    }

    /// Processes a new message update.
    fn process_new_message(&self, update: UpdateNewMessage) -> Result<(), MessagesManagerError> {
        debug!("Processing new message {}", update.message_id());

        // Convert TL MessageData to internal Message type
        let message = Self::convert_message_data(update.message)?;

        // In production, store in database and trigger callbacks
        info!("New message received: id={}", message.id);

        Ok(())
    }

    /// Processes a message edit update.
    fn process_edit_message(&self, update: UpdateEditMessage) -> Result<(), MessagesManagerError> {
        debug!("Processing message edit {}", update.message.id);

        // In production, update message in database
        info!("Message {} was edited", update.message.id);

        Ok(())
    }

    /// Processes a delete messages update.
    fn process_delete_messages(
        &self,
        update: UpdateDeleteMessages,
    ) -> Result<(), MessagesManagerError> {
        debug!("Processing delete of {} messages", update.messages.len());

        // In production, delete from database
        for msg_id in &update.messages {
            info!("Message {} was deleted", msg_id);
        }

        Ok(())
    }

    /// Processes a read history update.
    fn process_read_history(&self, update: UpdateReadHistory) -> Result<(), MessagesManagerError> {
        debug!("Processing read history up to {}", update.max_id);

        // In production, update read state in database
        info!("Messages up to {} marked as read", update.max_id);

        Ok(())
    }

    /// Converts TL MessageData to internal Message type.
    pub fn convert_message_data(data: MessageData) -> Result<Message, MessagesManagerError> {
        use rustgram_formatted_text::FormattedText;

        // Create FormattedText
        let formatted = FormattedText::new(data.message.as_str());

        // Create DialogIds from i64 values
        let dialog_id = DialogId::from_encoded(data.dialog_id)
            .map_err(|e| MessagesManagerError::Generic(format!("Invalid dialog ID: {:?}", e)))?;

        let sender_id = DialogId::from_encoded(data.sender_id)
            .map_err(|e| MessagesManagerError::Generic(format!("Invalid sender ID: {:?}", e)))?;

        // Create MessageId from i32
        let message_id = MessageId::from_server_id(data.id);

        // Create Message
        let message = Message::new_with_date(
            message_id,
            dialog_id,
            sender_id,
            data.date,
            formatted,
        ).map_err(MessagesManagerError::Validation)?;

        Ok(message)
    }

    /// Registers a callback for message updates.
    pub fn set_update_callback(&self, callback: Box<dyn MessageUpdateCallback>) {
        self.network_client.set_update_callback(callback);
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_net::NetQueryDispatcher;
    use rustgram_types::UserId;

    fn create_test_manager() -> MessagesManager {
        let dispatcher = Arc::new(NetQueryDispatcher::new());
        let network_config = MessageNetworkConfig::default();
        let network_client = Arc::new(MessageNetworkClient::new(dispatcher, network_config));
        let manager_config = MessagesManagerConfig::default();
        MessagesManager::new(network_client, manager_config)
    }

    #[test]
    fn test_manager_config_default() {
        let config = MessagesManagerConfig::default();
        assert_eq!(config.max_message_length, 4096);
        assert!(config.enable_dedup);
        assert_eq!(config.cleanup_interval, Duration::from_secs(60));
    }

    #[test]
    fn test_send_text_empty() {
        let manager = create_test_manager();
        let user_id = UserId::new(123456).unwrap();
        let dialog_id = DialogId::from(user_id);

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(manager.send_text(dialog_id, "".to_string(), None));

        assert!(result.is_err());
        match result.unwrap_err() {
            MessagesManagerError::Validation(MessageValidationError::InvalidContent(_)) => {
                // Expected
            }
            _ => panic!("Expected InvalidContent validation error"),
        }
    }

    #[test]
    fn test_send_text_too_long() {
        let manager = create_test_manager();
        let user_id = UserId::new(123456).unwrap();
        let dialog_id = DialogId::from(user_id);
        let long_text = "a".repeat(10000);

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(manager.send_text(dialog_id, long_text, None));

        assert!(result.is_err());
        match result.unwrap_err() {
            MessagesManagerError::Validation(MessageValidationError::InvalidContent(msg)) => {
                assert!(msg.contains("too long"));
            }
            _ => panic!("Expected InvalidContent validation error"),
        }
    }

    #[test]
    fn test_send_text_valid() {
        let manager = create_test_manager();
        let user_id = UserId::new(123456).unwrap();
        let dialog_id = DialogId::from(user_id);

        // This will fail with network error, but validates the text is accepted
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(manager.send_text(dialog_id, "Hello, world!".to_string(), None));

        // Should get a network error (no real connection), not validation error
        match result {
            Err(MessagesManagerError::Network(_)) => {
                // Expected - validation passed, network failed
            }
            other => panic!("Expected Network error, got {:?}", other),
        }
    }

    #[test]
    fn test_convert_message_data() {
        let data = MessageData::new(1, 123456, 789, 1700000000, "Test message".to_string());
        let _manager = create_test_manager();

        let result = MessagesManager::convert_message_data(data);
        assert!(result.is_ok());

        let message = result.unwrap();
        assert_eq!(message.id, MessageId::from_server_id(1));
    }

    #[test]
    fn test_process_update_new_message() {
        let manager = create_test_manager();
        let data = MessageData::new(1, 123456, 789, 1700000000, "Test".to_string());
        let update = UpdateNewMessage::new(data, 100, 1);

        let result = manager.process_update(Update::NewMessage(update));
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_update_delete() {
        let manager = create_test_manager();
        let update = UpdateDeleteMessages {
            flags: 0,
            messages: vec![1, 2, 3],
            pts: 100,
            pts_count: 3,
        };

        let result = manager.process_update(Update::DeleteMessages(update));
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_update_read_history() {
        let manager = create_test_manager();
        let update = UpdateReadHistory {
            peer: Peer::User { user_id: 123456 },
            max_id: 100,
            pts: 100,
            pts_count: 1,
        };

        let result = manager.process_update(Update::ReadHistory(update));
        assert!(result.is_ok());
    }
}
