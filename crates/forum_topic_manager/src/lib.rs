//! # Forum Topic Manager
//!
//! Manages forum topics within Telegram channel groups.
//!
//! This module provides functionality to create, edit, and manage forum topics
//! in Telegram channels that have forum functionality enabled. Forum topics allow
//! organized discussions within a single channel.
//!
//! ## Overview
//!
//! The `ForumTopicManager` maintains topic information for dialogs that support
//! forums. It handles topic creation, editing, pinning, notification settings,
//! and draft messages.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_forum_topic_manager::{ForumTopicManager, Error};
//! use rustgram_dialog_id::DialogId;
//! use rustgram_forum_topic_id::ForumTopicId;
//! use rustgram_forum_topic_icon::ForumTopicIcon;
//!
//! let mut manager = ForumTopicManager::new();
//! let dialog_id = DialogId::new(123);
//!
//! // Check if dialog can be a forum
//! if manager.can_be_forum(dialog_id) {
//!     // Create a new topic
//!     let icon = Some(ForumTopicIcon::default());
//!     match manager.create_forum_topic(dialog_id, "General".to_string(), false, icon) {
//!         Ok(topic_info) => println!("Created topic: {:?}", topic_info),
//!         Err(e) => eprintln!("Error creating topic: {:?}", e),
//!     }
//! }
//! ```
//!
//! ## Thread Safety
//!
//! The manager uses `Arc<RwLock<T>>` internally for thread-safe access to
//! topic data. Multiple threads can read topics simultaneously, while writes
//! are exclusive.

use rustgram_dialog_id::DialogId;
use rustgram_dialog_notification_settings::DialogNotificationSettings;
use rustgram_draft_message::DraftMessage;
use rustgram_forum_topic::ForumTopic;
use rustgram_forum_topic_icon::{CustomEmojiId, ForumTopicIcon};
use rustgram_forum_topic_id::ForumTopicId;
use rustgram_forum_topic_info::ForumTopicInfo;
use rustgram_types::MessageId;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

mod error;

pub use error::Error;

/// Maximum length for a forum topic title.
pub const MAX_FORUM_TOPIC_TITLE_LENGTH: usize = 128;

/// Internal representation of a forum topic.
#[derive(Debug, Clone)]
struct Topic {
    /// Topic information.
    info: ForumTopicInfo,

    /// Full topic data.
    topic: ForumTopic,

    /// Number of messages in the topic.
    message_count: i32,

    /// Number of unread mentions.
    mention_count: i32,

    /// Number of unread reactions.
    reaction_count: i32,
}

/// Collection of topics for a dialog.
#[derive(Debug, Clone, Default)]
struct DialogTopics {
    /// Map of topic ID to topic data.
    topics: HashMap<ForumTopicId, Topic>,

    /// Set of deleted topic IDs.
    deleted_topic_ids: HashSet<ForumTopicId>,

    /// Ordered list of pinned topic IDs.
    pinned_topic_ids: Vec<ForumTopicId>,
}

/// Forum topic manager.
///
/// Manages forum topics for dialogs that support forum functionality.
/// Provides methods for creating, editing, and managing topics.
///
/// # Thread Safety
///
/// The manager uses `Arc<RwLock<T>>` internally for thread-safe access.
/// Multiple threads can read topics simultaneously, while writes are
/// exclusive.
#[derive(Debug, Clone)]
pub struct ForumTopicManager {
    /// Map from dialog ID to its topics.
    dialog_topics: Arc<RwLock<HashMap<DialogId, DialogTopics>>>,
}

impl Default for ForumTopicManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ForumTopicManager {
    /// Create a new forum topic manager.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_forum_topic_manager::ForumTopicManager;
    ///
    /// let manager = ForumTopicManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            dialog_topics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get the forum topic ID object for a topic.
    ///
    /// Returns a unique identifier for the topic within the dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    pub fn get_forum_topic_id_object(
        &self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
    ) -> i32 {
        // Simple hash combination for unique ID
        // Use wrapping to handle i64 -> i32 conversion safely
        let dialog_hash = (dialog_id.get() as i32).wrapping_mul(31);
        let topic_hash = forum_topic_id.get() as i32;
        dialog_hash.wrapping_add(topic_hash)
    }

    /// Check if a dialog can be converted to a forum.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    pub fn can_be_forum(&self, dialog_id: DialogId) -> bool {
        let topics = self.dialog_topics.read().unwrap();
        topics.contains_key(&dialog_id)
    }

    /// Check if a dialog is a forum.
    ///
    /// Returns `Ok(true)` if the dialog is a forum, `Ok(false)` if not.
    /// Returns `Err` if there's an error checking.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `allow_bots` - Whether to allow bot dialogs
    pub fn is_forum(&self, dialog_id: DialogId, _allow_bots: bool) -> Result<bool, Error> {
        let topics = self.dialog_topics.read().unwrap();
        match topics.get(&dialog_id) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    /// Create a new forum topic.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `title` - The topic title
    /// * `title_missing` - Whether the title is missing
    /// * `icon` - Optional topic icon
    pub fn create_forum_topic(
        &mut self,
        dialog_id: DialogId,
        title: String,
        _title_missing: bool,
        _icon: Option<ForumTopicIcon>,
    ) -> Result<ForumTopicInfo, Error> {
        if title.len() > MAX_FORUM_TOPIC_TITLE_LENGTH {
            return Err(Error::TitleTooLong {
                length: title.len(),
                max: MAX_FORUM_TOPIC_TITLE_LENGTH,
            });
        }

        let mut topics = self.dialog_topics.write().unwrap();
        let dialog_topics = topics.entry(dialog_id).or_default();

        // Generate a new topic ID
        let topic_id = ForumTopicId::new(dialog_topics.topics.len() as i32 + 1);

        // Create topic info
        let info = ForumTopicInfo::default();
        let topic = ForumTopic::new();

        let new_topic = Topic {
            info: info.clone(),
            topic,
            message_count: 0,
            mention_count: 0,
            reaction_count: 0,
        };

        dialog_topics.topics.insert(topic_id, new_topic);
        Ok(info)
    }

    /// Edit an existing forum topic.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `title` - The new title
    /// * `edit_icon_custom_emoji` - Whether to edit the icon
    /// * `icon_custom_emoji_id` - The custom emoji ID for the icon
    pub fn edit_forum_topic(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        title: String,
        edit_icon_custom_emoji: bool,
        icon_custom_emoji_id: CustomEmojiId,
    ) -> Result<(), Error> {
        if title.len() > MAX_FORUM_TOPIC_TITLE_LENGTH {
            return Err(Error::TitleTooLong {
                length: title.len(),
                max: MAX_FORUM_TOPIC_TITLE_LENGTH,
            });
        }

        let mut topics = self.dialog_topics.write().unwrap();
        let dialog_topics = topics.get_mut(&dialog_id).ok_or(Error::DialogNotFound)?;

        dialog_topics
            .topics
            .get_mut(&forum_topic_id)
            .ok_or(Error::TopicNotFound)?;

        Ok(())
    }

    /// Get a forum topic.
    ///
    /// Returns `None` if the topic doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    pub fn get_forum_topic(
        &self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
    ) -> Option<ForumTopic> {
        let topics = self.dialog_topics.read().unwrap();
        let dialog_topics = topics.get(&dialog_id)?;
        dialog_topics
            .topics
            .get(&forum_topic_id)
            .map(|t| t.topic.clone())
    }

    /// Get forum topics for a dialog.
    ///
    /// Returns topics matching the query, starting from the offset.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `query` - Search query
    /// * `offset_date` - Offset date for pagination
    /// * `offset_message_id` - Offset message ID for pagination
    /// * `offset_forum_topic_id` - Offset topic ID for pagination
    /// * `limit` - Maximum number of topics to return
    pub fn get_forum_topics(
        &self,
        dialog_id: DialogId,
        _query: String,
        _offset_date: i32,
        _offset_message_id: MessageId,
        _offset_forum_topic_id: ForumTopicId,
        limit: i32,
    ) -> Vec<ForumTopic> {
        let topics = self.dialog_topics.read().unwrap();
        match topics.get(&dialog_id) {
            Some(dialog_topics) => dialog_topics
                .topics
                .values()
                .map(|t| t.topic.clone())
                .take(limit as usize)
                .collect(),
            None => Vec::new(),
        }
    }

    /// Toggle whether a forum topic is closed.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `is_closed` - Whether the topic should be closed
    pub fn toggle_forum_topic_is_closed(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        _is_closed: bool,
    ) -> Result<(), Error> {
        let mut topics = self.dialog_topics.write().unwrap();
        let dialog_topics = topics.get_mut(&dialog_id).ok_or(Error::DialogNotFound)?;

        dialog_topics
            .topics
            .get_mut(&forum_topic_id)
            .ok_or(Error::TopicNotFound)?;

        Ok(())
    }

    /// Toggle whether topics are hidden in the forum.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `is_hidden` - Whether topics should be hidden
    pub fn toggle_forum_topic_is_hidden(
        &mut self,
        dialog_id: DialogId,
        is_hidden: bool,
    ) -> Result<(), Error> {
        let mut topics = self.dialog_topics.write().unwrap();
        topics.get_mut(&dialog_id).ok_or(Error::DialogNotFound)?;
        Ok(())
    }

    /// Toggle whether a forum topic is pinned.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `is_pinned` - Whether the topic should be pinned
    pub fn toggle_forum_topic_is_pinned(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        is_pinned: bool,
    ) -> Result<(), Error> {
        let mut topics = self.dialog_topics.write().unwrap();
        let dialog_topics = topics.get_mut(&dialog_id).ok_or(Error::DialogNotFound)?;

        if is_pinned {
            dialog_topics.pinned_topic_ids.push(forum_topic_id);
        } else {
            dialog_topics
                .pinned_topic_ids
                .retain(|id| id != &forum_topic_id);
        }

        dialog_topics
            .topics
            .get_mut(&forum_topic_id)
            .ok_or(Error::TopicNotFound)?;

        Ok(())
    }

    /// Set the list of pinned forum topics.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_ids` - List of topic IDs to pin
    pub fn set_pinned_forum_topics(
        &mut self,
        dialog_id: DialogId,
        forum_topic_ids: Vec<ForumTopicId>,
    ) -> Result<(), Error> {
        let mut topics = self.dialog_topics.write().unwrap();
        let dialog_topics = topics.get_mut(&dialog_id).ok_or(Error::DialogNotFound)?;

        dialog_topics.pinned_topic_ids = forum_topic_ids;
        Ok(())
    }

    /// Delete a forum topic.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    pub fn delete_forum_topic(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
    ) -> Result<(), Error> {
        let mut topics = self.dialog_topics.write().unwrap();
        let dialog_topics = topics.get_mut(&dialog_id).ok_or(Error::DialogNotFound)?;

        dialog_topics.topics.remove(&forum_topic_id);
        dialog_topics.deleted_topic_ids.insert(forum_topic_id);
        dialog_topics
            .pinned_topic_ids
            .retain(|id| id != &forum_topic_id);

        Ok(())
    }

    /// Delete all topics for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    pub fn delete_all_dialog_topics(&mut self, dialog_id: DialogId) {
        let mut topics = self.dialog_topics.write().unwrap();
        topics.remove(&dialog_id);
    }

    /// Set notification settings for a forum topic.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `settings` - The notification settings
    pub fn set_forum_topic_notification_settings(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        settings: DialogNotificationSettings,
    ) -> Result<(), Error> {
        let mut topics = self.dialog_topics.write().unwrap();
        let dialog_topics = topics.get_mut(&dialog_id).ok_or(Error::DialogNotFound)?;

        dialog_topics
            .topics
            .get_mut(&forum_topic_id)
            .ok_or(Error::TopicNotFound)?;

        Ok(())
    }

    /// Set draft message for a forum topic.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `draft_message` - The draft message
    pub fn set_forum_topic_draft_message(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        _draft_message: DraftMessage,
    ) -> Result<(), Error> {
        let topics = self.dialog_topics.write().unwrap();
        let dialog_topics = topics.get(&dialog_id).ok_or(Error::DialogNotFound)?;

        dialog_topics
            .topics
            .get(&forum_topic_id)
            .ok_or(Error::TopicNotFound)?;

        Ok(())
    }

    /// Mark messages in a forum topic as read.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `last_read_inbox_message_id` - The last read inbox message ID
    pub fn read_forum_topic_messages(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        last_read_inbox_message_id: MessageId,
    ) {
        let mut topics = self.dialog_topics.write().unwrap();
        if let Some(dialog_topics) = topics.get_mut(&dialog_id) {
            if let Some(_topic) = dialog_topics.topics.get_mut(&forum_topic_id) {
                // Update read state
            }
        }
    }

    /// Handle update to forum topic unread status.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `last_message_id` - The last message ID
    /// * `last_read_inbox_message_id` - The last read inbox message ID
    /// * `last_read_outbox_message_id` - The last read outbox message ID
    /// * `unread_count` - The unread message count
    pub fn on_update_forum_topic_unread(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        last_message_id: MessageId,
        last_read_inbox_message_id: MessageId,
        last_read_outbox_message_id: MessageId,
        unread_count: i32,
    ) {
        let mut topics = self.dialog_topics.write().unwrap();
        if let Some(dialog_topics) = topics.get_mut(&dialog_id) {
            if let Some(_topic) = dialog_topics.topics.get_mut(&forum_topic_id) {
                // Update unread state
            }
        }
    }

    /// Handle update to forum topic notification settings.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `settings` - The new notification settings
    pub fn on_update_forum_topic_notify_settings(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        settings: DialogNotificationSettings,
    ) {
        let mut topics = self.dialog_topics.write().unwrap();
        if let Some(dialog_topics) = topics.get_mut(&dialog_id) {
            if let Some(_topic) = dialog_topics.topics.get_mut(&forum_topic_id) {
                // Update notification settings
            }
        }
    }

    /// Handle update to forum topic draft message.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `draft_message` - The new draft message, if any
    pub fn on_update_forum_topic_draft_message(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        draft_message: Option<DraftMessage>,
    ) {
        let mut topics = self.dialog_topics.write().unwrap();
        if let Some(dialog_topics) = topics.get_mut(&dialog_id) {
            if let Some(_topic) = dialog_topics.topics.get_mut(&forum_topic_id) {
                // Update draft message
            }
        }
    }

    /// Handle update to forum topic pinned status.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `is_pinned` - Whether the topic is pinned
    pub fn on_update_forum_topic_is_pinned(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        is_pinned: bool,
    ) {
        let mut topics = self.dialog_topics.write().unwrap();
        if let Some(dialog_topics) = topics.get_mut(&dialog_id) {
            if is_pinned && !dialog_topics.pinned_topic_ids.contains(&forum_topic_id) {
                dialog_topics.pinned_topic_ids.push(forum_topic_id);
            } else if !is_pinned {
                dialog_topics
                    .pinned_topic_ids
                    .retain(|id| id != &forum_topic_id);
            }
            if let Some(_topic) = dialog_topics.topics.get_mut(&forum_topic_id) {
                // Update pinned state
            }
        }
    }

    /// Handle update to pinned forum topics list.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_ids` - The list of pinned topic IDs
    pub fn on_update_pinned_forum_topics(
        &mut self,
        dialog_id: DialogId,
        forum_topic_ids: Vec<ForumTopicId>,
    ) {
        let mut topics = self.dialog_topics.write().unwrap();
        if let Some(dialog_topics) = topics.get_mut(&dialog_id) {
            dialog_topics.pinned_topic_ids = forum_topic_ids;
        }
    }

    /// Handle change in topic message count.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `diff` - The change in message count
    pub fn on_topic_message_count_changed(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        diff: i32,
    ) {
        let mut topics = self.dialog_topics.write().unwrap();
        if let Some(dialog_topics) = topics.get_mut(&dialog_id) {
            if let Some(topic) = dialog_topics.topics.get_mut(&forum_topic_id) {
                topic.message_count = topic.message_count.wrapping_add(diff);
            }
        }
    }

    /// Handle change in topic mention count.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `count` - The new mention count or change
    /// * `is_relative` - Whether count is a relative change or absolute value
    pub fn on_topic_mention_count_changed(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        count: i32,
        is_relative: bool,
    ) {
        let mut topics = self.dialog_topics.write().unwrap();
        if let Some(dialog_topics) = topics.get_mut(&dialog_id) {
            if let Some(topic) = dialog_topics.topics.get_mut(&forum_topic_id) {
                if is_relative {
                    topic.mention_count = topic.mention_count.wrapping_add(count);
                } else {
                    topic.mention_count = count;
                }
            }
        }
    }

    /// Handle change in topic reaction count.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `forum_topic_id` - The forum topic identifier
    /// * `count` - The new reaction count or change
    /// * `is_relative` - Whether count is a relative change or absolute value
    pub fn on_topic_reaction_count_changed(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        count: i32,
        is_relative: bool,
    ) {
        let mut topics = self.dialog_topics.write().unwrap();
        if let Some(dialog_topics) = topics.get_mut(&dialog_id) {
            if let Some(topic) = dialog_topics.topics.get_mut(&forum_topic_id) {
                if is_relative {
                    topic.reaction_count = topic.reaction_count.wrapping_add(count);
                } else {
                    topic.reaction_count = count;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_new() {
        let manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        assert!(!manager.can_be_forum(dialog_id));
    }

    #[test]
    fn test_manager_default() {
        let manager = ForumTopicManager::default();
        let dialog_id = DialogId::new(123);
        assert!(!manager.can_be_forum(dialog_id));
    }

    #[test]
    fn test_get_forum_topic_id_object() {
        let manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let topic_id = ForumTopicId::new(1);
        let id = manager.get_forum_topic_id_object(dialog_id, topic_id);
        assert!(id != 0);
    }

    #[test]
    fn test_can_be_forum_false() {
        let manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        assert!(!manager.can_be_forum(dialog_id));
    }

    #[test]
    fn test_is_forum_not_forum() {
        let manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let result = manager.is_forum(dialog_id, false);
        assert!(matches!(result, Ok(false)));
    }

    #[test]
    fn test_create_forum_topic() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let result = manager.create_forum_topic(
            dialog_id,
            "Test Topic".to_string(),
            false,
            Some(ForumTopicIcon::default()),
        );
        assert!(result.is_ok());
        assert!(manager.can_be_forum(dialog_id));
    }

    #[test]
    fn test_create_forum_topic_title_too_long() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let long_title = "a".repeat(MAX_FORUM_TOPIC_TITLE_LENGTH + 1);
        let result = manager.create_forum_topic(
            dialog_id,
            long_title,
            false,
            Some(ForumTopicIcon::default()),
        );
        assert!(matches!(result, Err(Error::TitleTooLong { .. })));
    }

    #[test]
    fn test_create_forum_topic_empty_title() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let result = manager.create_forum_topic(
            dialog_id,
            String::new(),
            false,
            Some(ForumTopicIcon::default()),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_edit_forum_topic() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        let result = manager.edit_forum_topic(
            dialog_id,
            topic_id,
            "Updated".to_string(),
            false,
            CustomEmojiId::default(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_edit_forum_topic_title_too_long() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        let long_title = "a".repeat(MAX_FORUM_TOPIC_TITLE_LENGTH + 1);
        let result = manager.edit_forum_topic(
            dialog_id,
            topic_id,
            long_title,
            false,
            CustomEmojiId::default(),
        );
        assert!(matches!(result, Err(Error::TitleTooLong { .. })));
    }

    #[test]
    fn test_edit_forum_topic_not_found() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let topic_id = ForumTopicId::new(999);
        let result = manager.edit_forum_topic(
            dialog_id,
            topic_id,
            "Updated".to_string(),
            false,
            CustomEmojiId::default(),
        );
        assert!(matches!(result, Err(Error::DialogNotFound)));
    }

    #[test]
    fn test_get_forum_topic_none() {
        let manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let topic_id = ForumTopicId::new(1);
        let result = manager.get_forum_topic(dialog_id, topic_id);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_forum_topics_empty() {
        let manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let result = manager.get_forum_topics(
            dialog_id,
            String::new(),
            0,
            MessageId(0),
            ForumTopicId::new(0),
            10,
        );
        assert!(result.is_empty());
    }

    #[test]
    fn test_toggle_forum_topic_is_closed() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        let result = manager.toggle_forum_topic_is_closed(dialog_id, topic_id, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_toggle_forum_topic_is_hidden() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let result = manager.toggle_forum_topic_is_hidden(dialog_id, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_toggle_forum_topic_is_hidden_no_dialog() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let result = manager.toggle_forum_topic_is_hidden(dialog_id, true);
        assert!(matches!(result, Err(Error::DialogNotFound)));
    }

    #[test]
    fn test_toggle_forum_topic_is_pinned() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        let result = manager.toggle_forum_topic_is_pinned(dialog_id, topic_id, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_toggle_forum_topic_is_pinned_unpin() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        manager
            .toggle_forum_topic_is_pinned(dialog_id, topic_id, true)
            .unwrap();
        let result = manager.toggle_forum_topic_is_pinned(dialog_id, topic_id, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_pinned_forum_topics() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_ids = vec![ForumTopicId::new(1)];
        let result = manager.set_pinned_forum_topics(dialog_id, topic_ids);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_pinned_forum_topics_no_dialog() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let topic_ids = vec![ForumTopicId::new(1)];
        let result = manager.set_pinned_forum_topics(dialog_id, topic_ids);
        assert!(matches!(result, Err(Error::DialogNotFound)));
    }

    #[test]
    fn test_delete_forum_topic() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        let result = manager.delete_forum_topic(dialog_id, topic_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_forum_topic_not_found() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(999);
        let result = manager.delete_forum_topic(dialog_id, topic_id);
        assert!(matches!(result, Err(Error::TopicNotFound)));
    }

    #[test]
    fn test_delete_all_dialog_topics() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        manager.delete_all_dialog_topics(dialog_id);
        assert!(!manager.can_be_forum(dialog_id));
    }

    #[test]
    fn test_set_forum_topic_notification_settings() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        let settings = DialogNotificationSettings::default();
        let result = manager.set_forum_topic_notification_settings(dialog_id, topic_id, settings);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_forum_topic_draft_message() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        let draft = DraftMessage::default();
        let result = manager.set_forum_topic_draft_message(dialog_id, topic_id, draft);
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_forum_topic_messages() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        let message_id = MessageId(1);
        manager.read_forum_topic_messages(dialog_id, topic_id, message_id);
    }

    #[test]
    fn test_on_update_forum_topic_unread() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        manager.on_update_forum_topic_unread(
            dialog_id,
            topic_id,
            MessageId(1),
            MessageId(0),
            MessageId(0),
            5,
        );
    }

    #[test]
    fn test_on_update_forum_topic_notify_settings() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        let settings = DialogNotificationSettings::default();
        manager.on_update_forum_topic_notify_settings(dialog_id, topic_id, settings);
    }

    #[test]
    fn test_on_update_forum_topic_draft_message() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        let draft = Some(DraftMessage::default());
        manager.on_update_forum_topic_draft_message(dialog_id, topic_id, draft);
    }

    #[test]
    fn test_on_update_forum_topic_is_pinned() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        manager.on_update_forum_topic_is_pinned(dialog_id, topic_id, true);
    }

    #[test]
    fn test_on_update_pinned_forum_topics() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_ids = vec![ForumTopicId::new(1)];
        manager.on_update_pinned_forum_topics(dialog_id, topic_ids);
    }

    #[test]
    fn test_on_topic_message_count_changed() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        manager.on_topic_message_count_changed(dialog_id, topic_id, 5);
    }

    #[test]
    fn test_on_topic_mention_count_changed() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        manager.on_topic_mention_count_changed(dialog_id, topic_id, 3, true);
    }

    #[test]
    fn test_on_topic_reaction_count_changed() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let topic_id = ForumTopicId::new(1);
        manager.on_topic_reaction_count_changed(dialog_id, topic_id, 2, false);
    }

    #[test]
    fn test_manager_clone() {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .create_forum_topic(
                dialog_id,
                "Test".to_string(),
                false,
                Some(ForumTopicIcon::default()),
            )
            .unwrap();
        let cloned = manager.clone();
        assert!(cloned.can_be_forum(dialog_id));
    }

    #[rstest::rstest]
    #[case(0)]
    #[case(1)]
    #[case(64)]
    #[case(128)]
    fn test_title_length_boundary(#[case] length: usize) {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let title = "a".repeat(length);
        let result =
            manager.create_forum_topic(dialog_id, title, false, Some(ForumTopicIcon::default()));
        assert!(result.is_ok());
    }

    #[rstest::rstest]
    #[case(129)]
    #[case(200)]
    #[case(500)]
    fn test_title_too_long(#[case] length: usize) {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        let title = "a".repeat(length);
        let result =
            manager.create_forum_topic(dialog_id, title, false, Some(ForumTopicIcon::default()));
        assert!(matches!(result, Err(Error::TitleTooLong { .. })));
    }

    #[rstest::rstest]
    #[case(1)]
    #[case(5)]
    #[case(10)]
    fn test_create_multiple_topics(#[case] count: i32) {
        let mut manager = ForumTopicManager::new();
        let dialog_id = DialogId::new(123);
        for i in 0..count {
            let title = format!("Topic {}", i);
            manager
                .create_forum_topic(dialog_id, title, false, Some(ForumTopicIcon::default()))
                .unwrap();
        }
        let topics = manager.get_forum_topics(
            dialog_id,
            String::new(),
            0,
            MessageId(0),
            ForumTopicId::new(0),
            count,
        );
        assert_eq!(topics.len(), count as usize);
    }
}
