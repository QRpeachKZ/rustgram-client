// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Business Connection Manager
//!
//! Manages business connections for Telegram business accounts interacting with bots.
//!
//! This module provides functionality for:
//! - Managing active business connections
//! - Sending messages through business connections
//! - Editing and deleting business messages
//! - Managing business account settings
//! - Transferring stars between accounts
//!
//! ## TDLib Correspondence
//!
//! Based on TDLib's `BusinessConnectionManager` from `td/telegram/BusinessConnectionManager.h`
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_business_connection_manager::BusinessConnectionManager;
//! use rustgram_types::{UserId, DialogId};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let manager = BusinessConnectionManager::new();
//!
//! // Check if a connection is valid
//! let connection_id = "abc123".to_string();
//! // Note: This will fail if connection doesn't exist
//! // let result = manager.check_business_connection(connection_id).await;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::collections::HashMap;
use std::sync::Arc;

use rustgram_dialog_id::DialogId;
use rustgram_message_input_reply_to::MessageInputReplyTo;
use rustgram_message_types::{MessageSendOptions, ReplyMarkup};
use rustgram_net::DcId;
use rustgram_types::{MessageId, UserId};
use thiserror::Error;
use tokio::sync::RwLock;

mod error;
mod stubs;
mod types;

pub use error::{Error, Result};
pub use stubs::{InputMessageContent, MessageEffectId};
pub use types::{BusinessConnection, BusinessMessage, BusinessMessages};

/// Maximum length for business account names.
const MAX_NAME_LENGTH: usize = 64;

/// Maximum number of media items in an album.
const MAX_ALBUM_SIZE: usize = 10;

/// Business connection manager.
///
/// Manages all aspects of business connections including:
/// - Connection validation and state management
/// - Message sending (single and album)
/// - Message editing and deletion
/// - Story management
/// - Business account settings
/// - Star balance and transfers
///
/// # Thread Safety
///
/// This manager is thread-safe and can be safely shared across threads.
/// All internal state is protected by `Arc<RwLock<T>>`.
#[derive(Debug, Clone)]
pub struct BusinessConnectionManager {
    /// Active business connections indexed by connection ID
    connections: Arc<RwLock<HashMap<String, BusinessConnection>>>,
}

impl BusinessConnectionManager {
    /// Creates a new business connection manager.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnectionManager;
    ///
    /// let manager = BusinessConnectionManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Validates that a business connection exists and is active.
    ///
    /// Returns error if connection is invalid or deleted.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Connection to validate
    ///
    /// # Returns
    ///
    /// Ok(()) if valid, Err otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnectionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessConnectionManager::new();
    /// let connection_id = "test_connection".to_string();
    ///
    /// // Note: Will fail if connection doesn't exist
    /// let result = manager.check_business_connection(connection_id).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_business_connection(&self, connection_id: String) -> Result<()> {
        let connections = self.connections.read().await;

        if let Some(conn) = connections.get(&connection_id) {
            if conn.is_deleted {
                return Err(Error::ConnectionClosed(connection_id));
            }
            Ok(())
        } else {
            Err(Error::ConnectionNotFound(connection_id))
        }
    }

    /// Gets the user ID for a business connection.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Connection to query
    ///
    /// # Returns
    ///
    /// UserId of the business account owner
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnectionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessConnectionManager::new();
    /// let connection_id = "test_connection".to_string();
    ///
    /// // Note: Will fail if connection doesn't exist
    /// let result = manager.get_business_connection_user_id(connection_id).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_business_connection_user_id(&self, connection_id: String) -> Result<UserId> {
        let connections = self.connections.read().await;

        connections
            .get(&connection_id)
            .map(|conn| conn.user_id)
            .ok_or_else(|| Error::ConnectionNotFound(connection_id))
    }

    /// Adds or updates a business connection.
    ///
    /// This is called internally when a new connection is established.
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection to add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::{BusinessConnectionManager, BusinessConnection};
    /// use rustgram_types::UserId;
    /// use rustgram_net::DcId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessConnectionManager::new();
    /// let connection = BusinessConnection::new(
    ///     "test_conn".to_string(),
    ///     UserId::new(123).expect("valid"),
    ///     DcId::internal(2),
    /// );
    ///
    /// manager.add_connection(connection).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_connection(&self, connection: BusinessConnection) {
        let mut connections = self.connections.write().await;
        connections.insert(connection.connection_id.clone(), connection);
    }

    /// Removes a business connection.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Connection to remove
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnectionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessConnectionManager::new();
    /// let connection_id = "test_connection".to_string();
    ///
    /// manager.remove_connection(connection_id).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_connection(&self, connection_id: String) {
        let mut connections = self.connections.write().await;
        connections.remove(&connection_id);
    }

    /// Sends a message through a business connection.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Connection to send through
    /// * `dialog_id` - Target dialog
    /// * `content` - Message content
    /// * `options` - Send options
    ///
    /// # Returns
    ///
    /// Future yielding sent message info
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::{BusinessConnectionManager, InputMessageContent};
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_message_types::MessageSendOptions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessConnectionManager::new();
    /// let connection_id = "test_connection".to_string();
    /// let dialog_id = DialogId::new(123456);
    /// let content = InputMessageContent::text("Hello".to_string());
    /// let options = MessageSendOptions::new();
    ///
    /// // Note: Will fail if connection doesn't exist
    /// let result = manager.send_message(connection_id, dialog_id, content, options).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message(
        &self,
        connection_id: String,
        dialog_id: DialogId,
        content: InputMessageContent,
        options: MessageSendOptions,
    ) -> Result<BusinessMessage> {
        // Validate connection
        self.check_business_connection(connection_id.clone())
            .await?;

        // Validate dialog ID
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialogId);
        }

        // Validate content
        if content.is_empty() {
            return Err(Error::EmptyContent);
        }

        // In a real implementation, this would send through the connection
        // For now, return a stub message
        Ok(BusinessMessage {
            message_id: MessageId::new(1, 0),
            date: 0,
        })
    }

    /// Sends an album of messages through a business connection.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Connection to send through
    /// * `dialog_id` - Target dialog
    /// * `contents` - Album contents (2-10 items)
    /// * `options` - Send options
    ///
    /// # Returns
    ///
    /// Future yielding sent album info
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::{BusinessConnectionManager, InputMessageContent};
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_message_types::MessageSendOptions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessConnectionManager::new();
    /// let connection_id = "test_connection".to_string();
    /// let dialog_id = DialogId::new(123456);
    /// let contents = vec![
    ///     InputMessageContent::photo("file1".to_string()),
    ///     InputMessageContent::photo("file2".to_string()),
    /// ];
    /// let options = MessageSendOptions::new();
    ///
    /// // Note: Will fail if connection doesn't exist
    /// let result = manager.send_message_album(connection_id, dialog_id, contents, options).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message_album(
        &self,
        connection_id: String,
        dialog_id: DialogId,
        contents: Vec<InputMessageContent>,
        options: MessageSendOptions,
    ) -> Result<BusinessMessages> {
        // Validate connection
        self.check_business_connection(connection_id).await?;

        // Validate dialog ID
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialogId);
        }

        // Validate album size
        if contents.len() < 2 || contents.len() > MAX_ALBUM_SIZE {
            return Err(Error::InvalidAlbumSize(contents.len()));
        }

        // Validate all contents
        if contents.iter().any(|c| c.is_empty()) {
            return Err(Error::EmptyContent);
        }

        // In a real implementation, this would send the album
        // For now, return stub messages
        Ok(BusinessMessages {
            messages: vec![],
            total_count: contents.len() as i32,
        })
    }

    /// Edits a text message in a business chat.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Connection to edit through
    /// * `dialog_id` - Dialog containing the message
    /// * `message_id` - Message to edit
    /// * `content` - New message content
    /// * `reply_markup` - Optional new inline keyboard
    ///
    /// # Returns
    ///
    /// Future yielding edited message info
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::{BusinessConnectionManager, InputMessageContent};
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_types::MessageId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessConnectionManager::new();
    /// let connection_id = "test_connection".to_string();
    /// let dialog_id = DialogId::new(123456);
    /// let message_id = MessageId::new(1, 0);
    /// let content = InputMessageContent::text("Updated text".to_string());
    ///
    /// // Note: Will fail if connection doesn't exist
    /// let result = manager.edit_message_text(connection_id, dialog_id, message_id, content, None).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn edit_message_text(
        &self,
        connection_id: String,
        dialog_id: DialogId,
        message_id: MessageId,
        content: InputMessageContent,
        reply_markup: Option<ReplyMarkup>,
    ) -> Result<BusinessMessage> {
        // Validate connection
        self.check_business_connection(connection_id).await?;

        // Validate dialog ID
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialogId);
        }

        // Validate content
        if content.is_empty() {
            return Err(Error::EmptyContent);
        }

        // In a real implementation, this would edit the message
        Ok(BusinessMessage {
            message_id,
            date: 0,
        })
    }

    /// Deletes messages from a business chat.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Connection to delete through
    /// * `message_ids` - Messages to delete
    ///
    /// # Returns
    ///
    /// Future completing when messages are deleted
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnectionManager;
    /// use rustgram_types::MessageId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessConnectionManager::new();
    /// let connection_id = "test_connection".to_string();
    /// let message_ids = vec![MessageId::new(1, 0), MessageId::new(2, 0)];
    ///
    /// // Note: Will fail if connection doesn't exist
    /// let result = manager.delete_messages(connection_id, message_ids).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_messages(
        &self,
        connection_id: String,
        message_ids: Vec<MessageId>,
    ) -> Result<()> {
        // Validate connection
        self.check_business_connection(connection_id).await?;

        // Validate message IDs
        if message_ids.is_empty() {
            return Err(Error::EmptyMessageList);
        }

        // In a real implementation, this would delete the messages
        Ok(())
    }

    /// Sets the business account display name.
    ///
    /// Names are limited to MAX_NAME_LENGTH characters.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Connection to update
    /// * `first_name` - First name (max 64 chars)
    /// * `last_name` - Last name (max 64 chars, optional)
    ///
    /// # Returns
    ///
    /// Future completing when name is set
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnectionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessConnectionManager::new();
    /// let connection_id = "test_connection".to_string();
    ///
    /// // Note: Will fail if connection doesn't exist
    /// let result = manager.set_business_name(connection_id, "John".to_string(), Some("Doe".to_string())).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_business_name(
        &self,
        connection_id: String,
        first_name: String,
        last_name: Option<String>,
    ) -> Result<()> {
        // Validate connection
        self.check_business_connection(connection_id).await?;

        // Validate name length
        if first_name.len() > MAX_NAME_LENGTH {
            return Err(Error::NameTooLong {
                field: "first_name",
                max: MAX_NAME_LENGTH,
                actual: first_name.len(),
            });
        }

        if let Some(last) = &last_name {
            if last.len() > MAX_NAME_LENGTH {
                return Err(Error::NameTooLong {
                    field: "last_name",
                    max: MAX_NAME_LENGTH,
                    actual: last.len(),
                });
            }
        }

        // In a real implementation, this would update the name
        Ok(())
    }

    /// Gets the current star balance for the business account.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Connection to query
    ///
    /// # Returns
    ///
    /// Future yielding star balance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::BusinessConnectionManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BusinessConnectionManager::new();
    /// let connection_id = "test_connection".to_string();
    ///
    /// // Note: Will fail if connection doesn't exist
    /// let result = manager.get_star_balance(connection_id).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_star_balance(&self, connection_id: String) -> Result<i64> {
        // Validate connection
        self.check_business_connection(connection_id).await?;

        // In a real implementation, this would fetch the balance
        Ok(0)
    }
}

impl Default for BusinessConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_net::DcId;

    #[tokio::test]
    async fn test_manager_new() {
        let manager = BusinessConnectionManager::new();
        assert!(manager.connections.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_manager_default() {
        let manager = BusinessConnectionManager::default();
        assert!(manager.connections.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_add_connection() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let connections = manager.connections.read().await;
        assert!(connections.contains_key("test_conn"));
    }

    #[tokio::test]
    async fn test_remove_connection() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;
        manager.remove_connection("test_conn".to_string()).await;

        let connections = manager.connections.read().await;
        assert!(!connections.contains_key("test_conn"));
    }

    #[tokio::test]
    async fn test_check_connection_valid() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let result = manager
            .check_business_connection("test_conn".to_string())
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_connection_not_found() {
        let manager = BusinessConnectionManager::new();

        let result = manager
            .check_business_connection("nonexistent".to_string())
            .await;

        assert!(matches!(result, Err(Error::ConnectionNotFound(_))));
    }

    #[tokio::test]
    async fn test_check_connection_deleted() {
        let manager = BusinessConnectionManager::new();
        let mut connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );
        connection.is_deleted = true;

        manager.add_connection(connection).await;

        let result = manager
            .check_business_connection("test_conn".to_string())
            .await;
        assert!(matches!(result, Err(Error::ConnectionClosed(_))));
    }

    #[tokio::test]
    async fn test_get_connection_user_id() {
        let manager = BusinessConnectionManager::new();
        let user_id = UserId::new(123).expect("valid");
        let connection =
            BusinessConnection::new("test_conn".to_string(), user_id, DcId::internal(2));

        manager.add_connection(connection).await;

        let result = manager
            .get_business_connection_user_id("test_conn".to_string())
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), UserId::new(123).expect("valid"));
    }

    #[tokio::test]
    async fn test_get_connection_user_id_not_found() {
        let manager = BusinessConnectionManager::new();

        let result = manager
            .get_business_connection_user_id("nonexistent".to_string())
            .await;

        assert!(matches!(result, Err(Error::ConnectionNotFound(_))));
    }

    #[tokio::test]
    async fn test_send_message_success() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let content = InputMessageContent::text("Hello".to_string());
        let options = MessageSendOptions::new();

        let result = manager
            .send_message(
                "test_conn".to_string(),
                DialogId::new(123456),
                content,
                options,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_message_connection_not_found() {
        let manager = BusinessConnectionManager::new();
        let content = InputMessageContent::text("Hello".to_string());
        let options = MessageSendOptions::new();

        let result = manager
            .send_message(
                "nonexistent".to_string(),
                DialogId::new(123456),
                content,
                options,
            )
            .await;

        assert!(matches!(result, Err(Error::ConnectionNotFound(_))));
    }

    #[tokio::test]
    async fn test_send_message_invalid_dialog_id() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let content = InputMessageContent::text("Hello".to_string());
        let options = MessageSendOptions::new();

        let result = manager
            .send_message("test_conn".to_string(), DialogId::new(0), content, options)
            .await;

        assert!(matches!(result, Err(Error::InvalidDialogId)));
    }

    #[tokio::test]
    async fn test_send_message_empty_content() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let content = InputMessageContent::text("".to_string());
        let options = MessageSendOptions::new();

        let result = manager
            .send_message(
                "test_conn".to_string(),
                DialogId::new(123456),
                content,
                options,
            )
            .await;

        assert!(matches!(result, Err(Error::EmptyContent)));
    }

    #[tokio::test]
    async fn test_send_message_album_success() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let contents = vec![
            InputMessageContent::photo("file1".to_string()),
            InputMessageContent::photo("file2".to_string()),
        ];
        let options = MessageSendOptions::new();

        let result = manager
            .send_message_album(
                "test_conn".to_string(),
                DialogId::new(123456),
                contents,
                options,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_message_album_too_small() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let contents = vec![InputMessageContent::photo("file1".to_string())];
        let options = MessageSendOptions::new();

        let result = manager
            .send_message_album(
                "test_conn".to_string(),
                DialogId::new(123456),
                contents,
                options,
            )
            .await;

        assert!(matches!(result, Err(Error::InvalidAlbumSize(1))));
    }

    #[tokio::test]
    async fn test_send_message_album_too_large() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let contents = vec![
            InputMessageContent::photo("file1".to_string()),
            InputMessageContent::photo("file2".to_string()),
            InputMessageContent::photo("file3".to_string()),
            InputMessageContent::photo("file4".to_string()),
            InputMessageContent::photo("file5".to_string()),
            InputMessageContent::photo("file6".to_string()),
            InputMessageContent::photo("file7".to_string()),
            InputMessageContent::photo("file8".to_string()),
            InputMessageContent::photo("file9".to_string()),
            InputMessageContent::photo("file10".to_string()),
            InputMessageContent::photo("file11".to_string()),
        ];
        let options = MessageSendOptions::new();

        let result = manager
            .send_message_album(
                "test_conn".to_string(),
                DialogId::new(123456),
                contents,
                options,
            )
            .await;

        assert!(matches!(result, Err(Error::InvalidAlbumSize(11))));
    }

    #[tokio::test]
    async fn test_send_message_album_empty_content() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let contents = vec![
            InputMessageContent::photo("file1".to_string()),
            InputMessageContent::text("".to_string()),
        ];
        let options = MessageSendOptions::new();

        let result = manager
            .send_message_album(
                "test_conn".to_string(),
                DialogId::new(123456),
                contents,
                options,
            )
            .await;

        assert!(matches!(result, Err(Error::EmptyContent)));
    }

    #[tokio::test]
    async fn test_edit_message_text_success() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let content = InputMessageContent::text("Updated".to_string());
        let message_id = MessageId::new(1, 0);

        let result = manager
            .edit_message_text(
                "test_conn".to_string(),
                DialogId::new(123456),
                message_id,
                content,
                None,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_edit_message_text_connection_not_found() {
        let manager = BusinessConnectionManager::new();

        let content = InputMessageContent::text("Updated".to_string());
        let message_id = MessageId::new(1, 0);

        let result = manager
            .edit_message_text(
                "nonexistent".to_string(),
                DialogId::new(123456),
                message_id,
                content,
                None,
            )
            .await;

        assert!(matches!(result, Err(Error::ConnectionNotFound(_))));
    }

    #[tokio::test]
    async fn test_delete_messages_success() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let message_ids = vec![MessageId::new(1, 0), MessageId::new(2, 0)];

        let result = manager
            .delete_messages("test_conn".to_string(), message_ids)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_messages_empty_list() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let message_ids = vec![];

        let result = manager
            .delete_messages("test_conn".to_string(), message_ids)
            .await;

        assert!(matches!(result, Err(Error::EmptyMessageList)));
    }

    #[tokio::test]
    async fn test_set_business_name_success() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let result = manager
            .set_business_name(
                "test_conn".to_string(),
                "John".to_string(),
                Some("Doe".to_string()),
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_business_name_first_name_too_long() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let long_name = "a".repeat(65);
        let result = manager
            .set_business_name("test_conn".to_string(), long_name, None)
            .await;

        assert!(matches!(result, Err(Error::NameTooLong { .. })));
    }

    #[tokio::test]
    async fn test_set_business_name_last_name_too_long() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let long_name = "a".repeat(65);
        let result = manager
            .set_business_name("test_conn".to_string(), "John".to_string(), Some(long_name))
            .await;

        assert!(matches!(result, Err(Error::NameTooLong { .. })));
    }

    #[tokio::test]
    async fn test_set_business_name_connection_not_found() {
        let manager = BusinessConnectionManager::new();

        let result = manager
            .set_business_name("nonexistent".to_string(), "John".to_string(), None)
            .await;

        assert!(matches!(result, Err(Error::ConnectionNotFound(_))));
    }

    #[tokio::test]
    async fn test_get_star_balance_success() {
        let manager = BusinessConnectionManager::new();
        let connection = BusinessConnection::new(
            "test_conn".to_string(),
            UserId::new(123).expect("valid"),
            DcId::internal(2),
        );

        manager.add_connection(connection).await;

        let result = manager.get_star_balance("test_conn".to_string()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_get_star_balance_connection_not_found() {
        let manager = BusinessConnectionManager::new();

        let result = manager.get_star_balance("nonexistent".to_string()).await;

        assert!(matches!(result, Err(Error::ConnectionNotFound(_))));
    }

    #[tokio::test]
    async fn test_concurrent_connection_access() {
        let manager = Arc::new(BusinessConnectionManager::new());
        let mut handles = vec![];

        // Spawn multiple concurrent tasks
        for i in 0..10 {
            let manager = Arc::clone(&manager);
            let handle = tokio::spawn(async move {
                let connection = BusinessConnection::new(
                    format!("conn_{}", i),
                    UserId::new(100 + i as i64).expect("valid"),
                    DcId::internal(2),
                );
                manager.add_connection(connection).await;
                let _ = manager
                    .check_business_connection(format!("conn_{}", i))
                    .await;
            });
            handles.push(handle);
        }

        // Wait for all tasks
        for handle in handles {
            handle.await.expect("task completed");
        }

        // Verify state is consistent
        let connections = manager.connections.read().await;
        assert_eq!(connections.len(), 10);
    }
}
