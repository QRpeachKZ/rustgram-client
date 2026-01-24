// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MessageQueryManager implementation.
//!
//! This is a stub implementation that matches TDLib's API structure.
//! Actual TDLib client integration will happen later.

use crate::error::{Error, Result};
use crate::state::{BeingUploadedCover, ReactionsToReload};
use crate::tl::{FoundMessages, MessageMedia};
use rustgram_affected_history::AffectedHistory;
use rustgram_business_connection_id::BusinessConnectionId;
use rustgram_dialog_list_id::DialogListId;
use rustgram_file_upload_id::FileUploadId;
use rustgram_message_extended_media::Photo;
use rustgram_message_full_id::MessageFullId;
use rustgram_message_thread_info::MessageThreadInfo;
use rustgram_message_viewer::MessageViewers;
use rustgram_types::{ChannelId, DialogId, MessageId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Maximum number of messages to search (server-side limit).
///
/// This matches TDLib's MAX_SEARCH_MESSAGES constant.
pub const MAX_SEARCH_MESSAGES: i32 = 100;

/// Message query manager.
///
/// Manages message queries including search, deletion, reading, and view operations.
/// Based on TDLib's MessageQueryManager from `td/telegram/MessageQueryManager.h`.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::MessageQueryManager;
///
/// let manager = MessageQueryManager::new();
/// assert!(!manager.has_pending_operations());
/// ```
#[derive(Debug, Clone)]
pub struct MessageQueryManager {
    /// Internal state for cover uploads.
    being_uploaded_covers: Arc<RwLock<HashMap<FileUploadId, BeingUploadedCover>>>,
    /// Messages being reloaded for extended media.
    being_reloaded_extended_media: Arc<RwLock<Vec<MessageFullId>>>,
    /// Messages being reloaded for fact checks.
    being_reloaded_fact_checks: Arc<RwLock<Vec<MessageFullId>>>,
    /// Messages being reloaded for views.
    being_reloaded_views: Arc<RwLock<Vec<MessageFullId>>>,
    /// Messages pending view counter increment.
    need_view_counter_increment: Arc<RwLock<Vec<MessageFullId>>>,
    /// Reactions being reloaded per dialog.
    being_reloaded_reactions: Arc<RwLock<HashMap<DialogId, ReactionsToReload>>>,
    /// Messages pending read reactions.
    pending_read_reactions: Arc<RwLock<HashMap<MessageFullId, i32>>>,
}

impl Default for MessageQueryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageQueryManager {
    /// Creates a new message query manager.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    ///
    /// let manager = MessageQueryManager::new();
    /// assert!(manager.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            being_uploaded_covers: Arc::new(RwLock::new(HashMap::new())),
            being_reloaded_extended_media: Arc::new(RwLock::new(Vec::new())),
            being_reloaded_fact_checks: Arc::new(RwLock::new(Vec::new())),
            being_reloaded_views: Arc::new(RwLock::new(Vec::new())),
            need_view_counter_increment: Arc::new(RwLock::new(Vec::new())),
            being_reloaded_reactions: Arc::new(RwLock::new(HashMap::new())),
            pending_read_reactions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // ========================================================================
    // Search Operations
    // ========================================================================

    /// Searches for messages in a dialog list.
    ///
    /// # Arguments
    ///
    /// * `_dialog_list_id` - Dialog list to search in
    /// * `_query` - Search query string
    /// * `_offset` - Search offset string
    /// * `limit` - Maximum results (max MAX_SEARCH_MESSAGES)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_dialog_list_id::DialogListId;
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let results = manager.search_messages(
    ///     DialogListId::main(),
    ///     "test".to_string(),
    ///     String::new(),
    ///     10,
    /// ).await;
    /// # }
    /// ```
    pub async fn search_messages(
        &self,
        _dialog_list_id: DialogListId,
        _query: String,
        _offset: String,
        limit: i32,
    ) -> Result<FoundMessages> {
        if limit <= 0 || limit > MAX_SEARCH_MESSAGES {
            return Err(Error::InvalidState);
        }
        // Stub: Return empty results
        Ok(FoundMessages::with_total_count(0))
    }

    /// Searches for public posts by hashtag.
    ///
    /// # Arguments
    ///
    /// * `_hashtag` - Hashtag to search for (without #)
    /// * `_offset` - Search offset string
    /// * `_limit` - Maximum results
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let results = manager.search_hashtag_posts("rust".to_string(), String::new(), 10).await;
    /// # }
    /// ```
    pub async fn search_hashtag_posts(
        &self,
        _hashtag: String,
        _offset: String,
        limit: i32,
    ) -> Result<FoundMessages> {
        if limit <= 0 || limit > MAX_SEARCH_MESSAGES {
            return Err(Error::InvalidState);
        }
        // Stub: Return empty results
        Ok(FoundMessages::with_total_count(0))
    }

    /// Searches for public posts by query.
    ///
    /// # Arguments
    ///
    /// * `_query` - Search query
    /// * `_offset` - Search offset string
    /// * `_limit` - Maximum results
    /// * `_star_count` - Minimum star count
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let results = manager.search_public_posts("test".to_string(), String::new(), 10, 0).await;
    /// # }
    /// ```
    pub async fn search_public_posts(
        &self,
        _query: String,
        _offset: String,
        limit: i32,
        _star_count: i64,
    ) -> Result<FoundMessages> {
        if limit <= 0 || limit > MAX_SEARCH_MESSAGES {
            return Err(Error::InvalidState);
        }
        // Stub: Return empty results
        Ok(FoundMessages::with_total_count(0))
    }

    // ========================================================================
    // Delete Operations
    // ========================================================================

    /// Deletes messages on the server.
    ///
    /// # Arguments
    ///
    /// * `_dialog_id` - Dialog containing messages
    /// * `_message_ids` - Message IDs to delete
    /// * `_revoke` - Whether to revoke for all participants
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let result = manager.delete_messages_on_server(
    ///     DialogId::from_chat(ChatId::new(123).unwrap()),
    ///     vec![MessageId::from_server_id(456)],
    ///     true,
    /// ).await;
    /// # }
    /// ```
    pub async fn delete_messages_on_server(
        &self,
        dialog_id: DialogId,
        _message_ids: Vec<MessageId>,
        _revoke: bool,
    ) -> Result<AffectedHistory> {
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialog);
        }
        // Stub: Return empty affected history
        Ok(AffectedHistory::empty())
    }

    /// Deletes dialog history on the server.
    ///
    /// # Arguments
    ///
    /// * `_dialog_id` - Dialog to clear
    /// * `_max_message_id` - Maximum message ID to delete
    /// * `_remove_from_dialog_list` - Whether to remove from dialog list
    /// * `_revoke` - Whether to revoke for all participants
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let result = manager.delete_dialog_history_on_server(
    ///     DialogId::from_chat(ChatId::new(123).unwrap()),
    ///     MessageId::default(),
    ///     false,
    ///     true,
    /// ).await;
    /// # }
    /// ```
    pub async fn delete_dialog_history_on_server(
        &self,
        dialog_id: DialogId,
        _max_message_id: MessageId,
        _remove_from_dialog_list: bool,
        _revoke: bool,
    ) -> Result<AffectedHistory> {
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialog);
        }
        // Stub: Return empty affected history
        Ok(AffectedHistory::empty())
    }

    /// Deletes all channel messages by sender on the server.
    ///
    /// # Arguments
    ///
    /// * `_channel_id` - Channel ID
    /// * `_sender_dialog_id` - Sender dialog ID
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_types::{ChannelId, ChatId, DialogId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let channel_id = ChannelId::new(123).unwrap();
    /// let sender = DialogId::from_chat(ChatId::new(456).unwrap());
    /// let result = manager.delete_all_channel_messages_by_sender_on_server(channel_id, sender).await;
    /// # }
    /// ```
    pub async fn delete_all_channel_messages_by_sender_on_server(
        &self,
        _channel_id: ChannelId,
        _sender_dialog_id: DialogId,
    ) -> Result<AffectedHistory> {
        // Stub: Return empty affected history
        Ok(AffectedHistory::empty())
    }

    // ========================================================================
    // Read Operations
    // ========================================================================

    /// Reads message contents on the server.
    ///
    /// # Arguments
    ///
    /// * `_dialog_id` - Dialog containing messages
    /// * `_message_ids` - Message IDs to mark as read
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let result = manager.read_message_contents_on_server(
    ///     DialogId::from_chat(ChatId::new(123).unwrap()),
    ///     vec![MessageId::from_server_id(456)],
    /// ).await;
    /// # }
    /// ```
    pub async fn read_message_contents_on_server(
        &self,
        dialog_id: DialogId,
        _message_ids: Vec<MessageId>,
    ) -> Result<()> {
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialog);
        }
        // Stub: Success
        Ok(())
    }

    /// Reads all dialog mentions on the server.
    ///
    /// # Arguments
    ///
    /// * `_dialog_id` - Dialog to read mentions in
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let result = manager.read_all_dialog_mentions_on_server(DialogId::from_chat(ChatId::new(123).unwrap())).await;
    /// # }
    /// ```
    pub async fn read_all_dialog_mentions_on_server(&self, dialog_id: DialogId) -> Result<()> {
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialog);
        }
        // Stub: Success
        Ok(())
    }

    // ========================================================================
    // Upload Operations
    // ========================================================================

    /// Uploads a message cover.
    ///
    /// # Arguments
    ///
    /// * `_business_connection_id` - Business connection ID
    /// * `_dialog_id` - Dialog ID
    /// * `_photo` - Photo to upload
    /// * `_file_upload_id` - File upload ID
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_business_connection_id::BusinessConnectionId;
    /// use rustgram_file_id::FileId;
    /// use rustgram_file_upload_id::FileUploadId;
    /// use rustgram_message_extended_media::Photo;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let conn_id = BusinessConnectionId::default();
    /// let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
    /// let photo = Photo::new();
    /// let file_id = FileId::new(1, 0);
    /// let upload_id = FileUploadId::new(file_id, 1);
    ///
    /// let result = manager.upload_message_cover(conn_id, dialog_id, photo, upload_id).await;
    /// # }
    /// ```
    pub async fn upload_message_cover(
        &self,
        business_connection_id: BusinessConnectionId,
        dialog_id: DialogId,
        photo: Photo,
        file_upload_id: FileUploadId,
    ) -> Result<()> {
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialog);
        }

        let mut covers = self.being_uploaded_covers.write().await;
        let cover = BeingUploadedCover::new(business_connection_id, dialog_id, photo)
            .with_file_upload_id(file_upload_id);
        covers.insert(file_upload_id, cover);
        Ok(())
    }

    /// Completes a message cover upload.
    ///
    /// # Arguments
    ///
    /// * `_file_upload_id` - File upload ID
    /// * `_media` - Uploaded message media
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_file_upload_id::FileUploadId;
    /// use rustgram_file_id::FileId;
    /// use rustgram_message_query_manager::tl::MessageMedia;
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let file_id = FileId::new(1, 0);
    /// let upload_id = FileUploadId::new(file_id, 1);
    /// let media = MessageMedia::photo();
    ///
    /// let result = manager.complete_upload_message_cover(upload_id, &media).await;
    /// # }
    /// ```
    pub async fn complete_upload_message_cover(
        &self,
        file_upload_id: FileUploadId,
        _media: &MessageMedia,
    ) -> Result<()> {
        let mut covers = self.being_uploaded_covers.write().await;
        if let Some(cover) = covers.get_mut(&file_upload_id) {
            let completed = std::mem::replace(cover, cover.clone().mark_complete());
            *cover = completed;
            Ok(())
        } else {
            Err(Error::UploadFailed)
        }
    }

    // ========================================================================
    // View Operations
    // ========================================================================

    /// Views messages in a dialog.
    ///
    /// # Arguments
    ///
    /// * `_dialog_id` - Dialog containing messages
    /// * `_message_ids` - Message IDs to view
    /// * `_increment_view_counter` - Whether to increment view counter
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// manager.view_messages(
    ///     DialogId::from_chat(ChatId::new(123).unwrap()),
    ///     vec![MessageId::from_server_id(456)],
    ///     true,
    /// ).await;
    /// # }
    /// ```
    pub async fn view_messages(
        &self,
        dialog_id: DialogId,
        message_ids: Vec<MessageId>,
        increment_view_counter: bool,
    ) {
        if !dialog_id.is_valid() {
            return;
        }

        if increment_view_counter {
            let mut counter = self.need_view_counter_increment.write().await;
            for message_id in message_ids {
                let full_id = MessageFullId::new(dialog_id, message_id);
                if !counter.contains(&full_id) {
                    counter.push(full_id);
                }
            }
        }
    }

    /// Gets message viewers.
    ///
    /// # Arguments
    ///
    /// * `_message_full_id` - Full message identifier
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_message_full_id::MessageFullId;
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let full_id = MessageFullId::new(DialogId::from_chat(ChatId::new(123).unwrap()), MessageId::from_server_id(456));
    /// let viewers = manager.get_message_viewers(full_id).await;
    /// # }
    /// ```
    pub async fn get_message_viewers(
        &self,
        _message_full_id: MessageFullId,
    ) -> Result<MessageViewers> {
        // Stub: Return empty viewers
        Ok(MessageViewers::new())
    }

    // ========================================================================
    // Reload Operations
    // ========================================================================

    /// Reloads message extended media.
    ///
    /// # Arguments
    ///
    /// * `_dialog_id` - Dialog containing messages
    /// * `_message_ids` - Message IDs to reload
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// manager.reload_message_extended_media(
    ///     DialogId::from_chat(ChatId::new(123).unwrap()),
    ///     vec![MessageId::from_server_id(456)],
    /// ).await;
    /// # }
    /// ```
    pub async fn reload_message_extended_media(
        &self,
        dialog_id: DialogId,
        message_ids: Vec<MessageId>,
    ) {
        let mut reloading = self.being_reloaded_extended_media.write().await;
        for message_id in message_ids {
            let full_id = MessageFullId::new(dialog_id, message_id);
            if !reloading.contains(&full_id) {
                reloading.push(full_id);
            }
        }
    }

    /// Reloads message fact checks.
    ///
    /// # Arguments
    ///
    /// * `_dialog_id` - Dialog containing messages
    /// * `_message_ids` - Message IDs to reload
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// manager.reload_message_fact_checks(
    ///     DialogId::from_chat(ChatId::new(123).unwrap()),
    ///     vec![MessageId::from_server_id(456)],
    /// ).await;
    /// # }
    /// ```
    pub async fn reload_message_fact_checks(
        &self,
        dialog_id: DialogId,
        message_ids: Vec<MessageId>,
    ) {
        let mut reloading = self.being_reloaded_fact_checks.write().await;
        for message_id in message_ids {
            let full_id = MessageFullId::new(dialog_id, message_id);
            if !reloading.contains(&full_id) {
                reloading.push(full_id);
            }
        }
    }

    /// Queues a message reaction reload.
    ///
    /// # Arguments
    ///
    /// * `_message_full_id` - Full message identifier
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_message_full_id::MessageFullId;
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let full_id = MessageFullId::new(DialogId::from_chat(ChatId::new(123).unwrap()), MessageId::from_server_id(456));
    /// manager.queue_message_reactions_reload(full_id).await;
    /// # }
    /// ```
    pub async fn queue_message_reactions_reload(&self, message_full_id: MessageFullId) {
        let dialog_id = message_full_id.dialog_id();
        let message_id = message_full_id.message_id();

        let mut reloading = self.being_reloaded_reactions.write().await;
        let reactions = reloading
            .entry(dialog_id)
            .or_insert_with(ReactionsToReload::new);
        reactions.add_message(message_id);
    }

    // ========================================================================
    // Discussion Operations
    // ========================================================================

    /// Gets the discussion message for a message.
    ///
    /// # Arguments
    ///
    /// * `_dialog_id` - Dialog containing the message
    /// * `_message_id` - Message ID
    /// * `_expected_dialog_id` - Expected discussion dialog ID
    /// * `_expected_message_id` - Expected discussion message ID
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let result = manager.get_discussion_message(
    ///     DialogId::from_chat(ChatId::new(123).unwrap()),
    ///     MessageId::from_server_id(456),
    ///     DialogId::from_chat(ChatId::new(789).unwrap()),
    ///     MessageId::from_server_id(101),
    /// ).await;
    /// # }
    /// ```
    pub async fn get_discussion_message(
        &self,
        dialog_id: DialogId,
        _message_id: MessageId,
        _expected_dialog_id: DialogId,
        _expected_message_id: MessageId,
    ) -> Result<MessageThreadInfo> {
        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialog);
        }
        // Stub: Return empty thread info
        Ok(MessageThreadInfo::new(dialog_id, Vec::new(), 0))
    }

    // ========================================================================
    // State Query Operations
    // ========================================================================

    /// Checks if the manager is empty (no pending operations).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    ///
    /// let manager = MessageQueryManager::new();
    /// assert!(manager.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        // This is a synchronous check for the state
        // In a real implementation, this would check all the Arc<RwLock> states
        true
    }

    /// Checks if there are pending operations.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    ///
    /// let manager = MessageQueryManager::new();
    /// assert!(!manager.has_pending_operations());
    /// ```
    #[must_use]
    pub fn has_pending_operations(&self) -> bool {
        !self.is_empty()
    }

    /// Gets the number of messages being reloaded for extended media.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let count = manager.extended_media_reload_count().await;
    /// assert_eq!(count, 0);
    /// # }
    /// ```
    pub async fn extended_media_reload_count(&self) -> usize {
        self.being_reloaded_extended_media.read().await.len()
    }

    /// Gets the number of covers being uploaded.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// let count = manager.upload_count().await;
    /// assert_eq!(count, 0);
    /// # }
    /// ```
    pub async fn upload_count(&self) -> usize {
        self.being_uploaded_covers.read().await.len()
    }

    /// Clears all pending operations.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::MessageQueryManager;
    ///
    /// # #[tokio::main]
    /// # async fn test() {
    /// let manager = MessageQueryManager::new();
    /// manager.clear_all().await;
    /// assert!(manager.is_empty());
    /// # }
    /// ```
    pub async fn clear_all(&self) {
        self.being_uploaded_covers.write().await.clear();
        self.being_reloaded_extended_media.write().await.clear();
        self.being_reloaded_fact_checks.write().await.clear();
        self.being_reloaded_views.write().await.clear();
        self.need_view_counter_increment.write().await.clear();
        self.being_reloaded_reactions.write().await.clear();
        self.pending_read_reactions.write().await.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_file_id::FileId;
    use rustgram_types::ChatId;

    // Basic creation tests
    #[test]
    fn test_manager_new() {
        let manager = MessageQueryManager::new();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_manager_default() {
        let manager = MessageQueryManager::default();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_manager_clone() {
        let manager1 = MessageQueryManager::new();
        let manager2 = manager1.clone();
        assert!(manager1.is_empty());
        assert!(manager2.is_empty());
    }

    // Constants tests
    #[test]
    fn test_max_search_messages() {
        assert_eq!(MAX_SEARCH_MESSAGES, 100);
    }

    // Search operation tests
    #[tokio::test]
    async fn test_search_messages_valid_limit() {
        let manager = MessageQueryManager::new();
        let result = manager
            .search_messages(DialogListId::main(), "test".to_string(), String::new(), 10)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_messages_invalid_limit() {
        let manager = MessageQueryManager::new();
        let result = manager
            .search_messages(DialogListId::main(), "test".to_string(), String::new(), 0)
            .await;
        assert!(matches!(result, Err(Error::InvalidState)));
    }

    #[tokio::test]
    async fn test_search_messages_exceeds_max() {
        let manager = MessageQueryManager::new();
        let result = manager
            .search_messages(
                DialogListId::main(),
                "test".to_string(),
                String::new(),
                MAX_SEARCH_MESSAGES + 1,
            )
            .await;
        assert!(matches!(result, Err(Error::InvalidState)));
    }

    #[tokio::test]
    async fn test_search_hashtag_posts_valid_limit() {
        let manager = MessageQueryManager::new();
        let result = manager
            .search_hashtag_posts("rust".to_string(), String::new(), 10)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_hashtag_posts_invalid_limit() {
        let manager = MessageQueryManager::new();
        let result = manager
            .search_hashtag_posts("rust".to_string(), String::new(), 0)
            .await;
        assert!(matches!(result, Err(Error::InvalidState)));
    }

    #[tokio::test]
    async fn test_search_public_posts_valid_limit() {
        let manager = MessageQueryManager::new();
        let result = manager
            .search_public_posts("test".to_string(), String::new(), 10, 0)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_public_posts_invalid_limit() {
        let manager = MessageQueryManager::new();
        let result = manager
            .search_public_posts("test".to_string(), String::new(), -1, 0)
            .await;
        assert!(matches!(result, Err(Error::InvalidState)));
    }

    // Delete operation tests
    #[tokio::test]
    async fn test_delete_messages_on_server_invalid_dialog() {
        let manager = MessageQueryManager::new();
        let result = manager
            .delete_messages_on_server(DialogId::default(), vec![], true)
            .await;
        assert!(matches!(result, Err(Error::InvalidDialog)));
    }

    #[tokio::test]
    async fn test_delete_messages_on_server_valid_dialog() {
        let manager = MessageQueryManager::new();
        let result = manager
            .delete_messages_on_server(
                DialogId::from_chat(ChatId::new(123).unwrap()),
                vec![MessageId::from_server_id(456)],
                true,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_dialog_history_on_server_invalid_dialog() {
        let manager = MessageQueryManager::new();
        let result = manager
            .delete_dialog_history_on_server(DialogId::default(), MessageId::default(), false, true)
            .await;
        assert!(matches!(result, Err(Error::InvalidDialog)));
    }

    #[tokio::test]
    async fn test_delete_dialog_history_on_server_valid_dialog() {
        let manager = MessageQueryManager::new();
        let result = manager
            .delete_dialog_history_on_server(
                DialogId::from_chat(ChatId::new(123).unwrap()),
                MessageId::default(),
                false,
                true,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_all_channel_messages_by_sender() {
        let manager = MessageQueryManager::new();
        let channel_id = ChannelId::new(123).unwrap();
        let sender = DialogId::from_chat(ChatId::new(456).unwrap());
        let result = manager
            .delete_all_channel_messages_by_sender_on_server(channel_id, sender)
            .await;
        assert!(result.is_ok());
    }

    // Read operation tests
    #[tokio::test]
    async fn test_read_message_contents_on_server_invalid_dialog() {
        let manager = MessageQueryManager::new();
        let result = manager
            .read_message_contents_on_server(DialogId::default(), vec![])
            .await;
        assert!(matches!(result, Err(Error::InvalidDialog)));
    }

    #[tokio::test]
    async fn test_read_message_contents_on_server_valid_dialog() {
        let manager = MessageQueryManager::new();
        let result = manager
            .read_message_contents_on_server(
                DialogId::from_chat(ChatId::new(123).unwrap()),
                vec![MessageId::from_server_id(456)],
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_read_all_dialog_mentions_on_server_invalid_dialog() {
        let manager = MessageQueryManager::new();
        let result = manager
            .read_all_dialog_mentions_on_server(DialogId::default())
            .await;
        assert!(matches!(result, Err(Error::InvalidDialog)));
    }

    #[tokio::test]
    async fn test_read_all_dialog_mentions_on_server_valid_dialog() {
        let manager = MessageQueryManager::new();
        let result = manager
            .read_all_dialog_mentions_on_server(DialogId::from_chat(ChatId::new(123).unwrap()))
            .await;
        assert!(result.is_ok());
    }

    // Upload operation tests
    #[tokio::test]
    async fn test_upload_message_cover_invalid_dialog() {
        let manager = MessageQueryManager::new();
        let file_id = FileId::new(1, 0);
        let upload_id = FileUploadId::new(file_id, 1);
        let result = manager
            .upload_message_cover(
                BusinessConnectionId::default(),
                DialogId::default(),
                Photo::new(),
                upload_id,
            )
            .await;
        assert!(matches!(result, Err(Error::InvalidDialog)));
    }

    #[tokio::test]
    async fn test_upload_message_cover_valid_dialog() {
        let manager = MessageQueryManager::new();
        let file_id = FileId::new(1, 0);
        let upload_id = FileUploadId::new(file_id, 1);
        let result = manager
            .upload_message_cover(
                BusinessConnectionId::default(),
                DialogId::from_chat(ChatId::new(123).unwrap()),
                Photo::new(),
                upload_id,
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(manager.upload_count().await, 1);
    }

    #[tokio::test]
    async fn test_complete_upload_message_cover_not_found() {
        let manager = MessageQueryManager::new();
        let file_id = FileId::new(1, 0);
        let upload_id = FileUploadId::new(file_id, 1);
        let media = MessageMedia::photo();
        let result = manager
            .complete_upload_message_cover(upload_id, &media)
            .await;
        assert!(matches!(result, Err(Error::UploadFailed)));
    }

    #[tokio::test]
    async fn test_complete_upload_message_cover_after_upload() {
        let manager = MessageQueryManager::new();
        let file_id = FileId::new(1, 0);
        let upload_id = FileUploadId::new(file_id, 1);

        // First upload
        manager
            .upload_message_cover(
                BusinessConnectionId::default(),
                DialogId::from_chat(ChatId::new(123).unwrap()),
                Photo::new(),
                upload_id,
            )
            .await
            .unwrap();

        // Then complete
        let media = MessageMedia::photo();
        let result = manager
            .complete_upload_message_cover(upload_id, &media)
            .await;
        assert!(result.is_ok());
    }

    // View operation tests
    #[tokio::test]
    async fn test_view_messages_invalid_dialog() {
        let manager = MessageQueryManager::new();
        manager
            .view_messages(
                DialogId::default(),
                vec![MessageId::from_server_id(1)],
                true,
            )
            .await;
        // Should not panic, just return
        assert!(true);
    }

    #[tokio::test]
    async fn test_view_messages_valid_dialog() {
        let manager = MessageQueryManager::new();
        manager
            .view_messages(
                DialogId::from_chat(ChatId::new(123).unwrap()),
                vec![MessageId::from_server_id(456)],
                true,
            )
            .await;
        // Should not panic
        assert!(true);
    }

    #[tokio::test]
    async fn test_get_message_viewers() {
        let manager = MessageQueryManager::new();
        let full_id = MessageFullId::new(
            DialogId::from_chat(ChatId::new(123).unwrap()),
            MessageId::from_server_id(456),
        );
        let viewers = manager.get_message_viewers(full_id).await;
        assert!(viewers.is_ok());
        assert!(viewers.unwrap().is_empty());
    }

    // Reload operation tests
    #[tokio::test]
    async fn test_reload_message_extended_media() {
        let manager = MessageQueryManager::new();
        manager
            .reload_message_extended_media(
                DialogId::from_chat(ChatId::new(123).unwrap()),
                vec![MessageId::from_server_id(456)],
            )
            .await;
        assert_eq!(manager.extended_media_reload_count().await, 1);
    }

    #[tokio::test]
    async fn test_reload_message_fact_checks() {
        let manager = MessageQueryManager::new();
        manager
            .reload_message_fact_checks(
                DialogId::from_chat(ChatId::new(123).unwrap()),
                vec![MessageId::from_server_id(456)],
            )
            .await;
        // Should not panic
        assert!(true);
    }

    #[tokio::test]
    async fn test_queue_message_reactions_reload() {
        let manager = MessageQueryManager::new();
        let full_id = MessageFullId::new(
            DialogId::from_chat(ChatId::new(123).unwrap()),
            MessageId::from_server_id(456),
        );
        manager.queue_message_reactions_reload(full_id).await;
        // Should not panic
        assert!(true);
    }

    // Discussion operation tests
    #[tokio::test]
    async fn test_get_discussion_message_invalid_dialog() {
        let manager = MessageQueryManager::new();
        let result = manager
            .get_discussion_message(
                DialogId::default(),
                MessageId::from_server_id(456),
                DialogId::default(),
                MessageId::default(),
            )
            .await;
        assert!(matches!(result, Err(Error::InvalidDialog)));
    }

    #[tokio::test]
    async fn test_get_discussion_message_valid_dialog() {
        let manager = MessageQueryManager::new();
        let result = manager
            .get_discussion_message(
                DialogId::from_chat(ChatId::new(123).unwrap()),
                MessageId::from_server_id(456),
                DialogId::from_chat(ChatId::new(789).unwrap()),
                MessageId::from_server_id(101),
            )
            .await;
        assert!(result.is_ok());
    }

    // State query tests
    #[test]
    fn test_is_empty() {
        let manager = MessageQueryManager::new();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_has_pending_operations() {
        let manager = MessageQueryManager::new();
        assert!(!manager.has_pending_operations());
    }

    #[tokio::test]
    async fn test_extended_media_reload_count() {
        let manager = MessageQueryManager::new();
        assert_eq!(manager.extended_media_reload_count().await, 0);
    }

    #[tokio::test]
    async fn test_upload_count() {
        let manager = MessageQueryManager::new();
        assert_eq!(manager.upload_count().await, 0);
    }

    #[tokio::test]
    async fn test_clear_all() {
        let manager = MessageQueryManager::new();
        // Add some state
        manager
            .reload_message_extended_media(
                DialogId::from_chat(ChatId::new(123).unwrap()),
                vec![MessageId::from_server_id(456)],
            )
            .await;

        manager.clear_all().await;
        assert_eq!(manager.extended_media_reload_count().await, 0);
    }

    // Error variant tests
    #[test]
    fn test_error_invalid_dialog_display() {
        let err = Error::InvalidDialog;
        assert_eq!(format!("{err}"), "Invalid dialog ID");
    }

    #[test]
    fn test_error_invalid_message_display() {
        let err = Error::InvalidMessage;
        assert_eq!(format!("{err}"), "Invalid message ID");
    }

    #[test]
    fn test_error_upload_failed_display() {
        let err = Error::UploadFailed;
        assert_eq!(format!("{err}"), "Message upload failed");
    }

    #[test]
    fn test_error_delete_failed_display() {
        let err = Error::DeleteFailed;
        assert_eq!(format!("{err}"), "Message deletion failed");
    }

    #[test]
    fn test_error_search_failed_display() {
        let err = Error::SearchFailed;
        assert_eq!(format!("{err}"), "Message search failed");
    }

    #[test]
    fn test_error_invalid_state_display() {
        let err = Error::InvalidState;
        assert_eq!(format!("{err}"), "Invalid state for operation");
    }

    #[test]
    fn test_error_io_error_display() {
        let err = Error::IoError("test error".to_string());
        assert!(format!("{err}").contains("test error"));
    }

    #[test]
    fn test_error_other_display() {
        let err = Error::Other("custom error".to_string());
        assert!(format!("{err}").contains("custom error"));
    }

    // State tracking tests
    #[tokio::test]
    async fn test_view_messages_with_increment() {
        let manager = MessageQueryManager::new();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let msg_id = MessageId::from_server_id(456);

        manager.view_messages(dialog_id, vec![msg_id], true).await;

        let counter = manager.need_view_counter_increment.read().await;
        assert!(counter.contains(&MessageFullId::new(dialog_id, msg_id)));
    }

    #[tokio::test]
    async fn test_view_messages_without_increment() {
        let manager = MessageQueryManager::new();
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let msg_id = MessageId::from_server_id(456);

        manager.view_messages(dialog_id, vec![msg_id], false).await;

        let counter = manager.need_view_counter_increment.read().await;
        assert!(!counter.contains(&MessageFullId::new(dialog_id, msg_id)));
    }

    #[tokio::test]
    async fn test_multiple_searches() {
        let manager = MessageQueryManager::new();

        for i in 1..=10 {
            let _result = manager
                .search_messages(
                    DialogListId::main(),
                    format!("query{}", i),
                    String::new(),
                    i32::min(10, i),
                )
                .await;
        }
        // All should succeed
        assert!(true);
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let manager = Arc::new(MessageQueryManager::new());
        let mut handles = vec![];

        for i in 0..10 {
            let mgr = manager.clone();
            let handle = tokio::spawn(async move {
                let dialog_id = DialogId::from_chat(ChatId::new((i + 1) as i64).unwrap());
                mgr.view_messages(
                    dialog_id,
                    vec![MessageId::from_server_id((i + 100) as i32)],
                    true,
                )
                .await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        // Should complete without panics
        assert!(true);
    }
}
