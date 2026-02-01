//! Saved messages topic management for Telegram client.
//!
//! This module provides functionality for managing saved messages topics,
//! including topic creation, message tracking, and state management.
//!
//! # Overview
//!
//! Saved messages topics allow users to organize saved messages from different
//! dialogs. Each topic is associated with a dialog and contains messages from
//! that dialog that have been saved by the user.
//!
//! # Main Components
//!
//! - [`SavedMessagesTopicId`]: Unique identifier for saved messages topics
//! - [`SavedMessagesTopic`]: Main topic struct with message state
//! - [`SavedMessagesTopicInfo`]: Lightweight info for monoforum topics
//! - [`TopicList`]: Container for managing topic collections
//! - [`SavedMessagesManager`]: Main manager for operations
//!
//! # Example
//!
//! ```rust
//! use rustgram_saved_messages_manager::{SavedMessagesManager, SavedMessagesTopicId};
//! use rustgram_types::{DialogId, UserId};
//!
//! // Create a manager
//! let mut manager = SavedMessagesManager::new();
//!
//! // Create a topic ID from a user ID
//! let user_id = UserId::new(12345).unwrap();
//! let dialog_id = DialogId::from_user(user_id);
//! let topic_id = SavedMessagesTopicId::from_user_id(user_id);
//!
//! // Check if a topic exists
//! let has_topic = manager.has_topic(dialog_id, topic_id);
//! ```

pub mod error;
pub mod manager;
pub mod topic;
pub mod topic_id;
pub mod topic_list;

// Re-export commonly used types
pub use error::{Result, SavedMessagesError};
pub use manager::{constants, SavedMessagesManager};
pub use topic::{DraftMessage, OrderedMessages, SavedMessagesTopic, SavedMessagesTopicInfo};
pub use topic_id::SavedMessagesTopicId;
pub use topic_list::{TopicDate, TopicList};

/// Library version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::{ChatId, UserId};

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_reexports() {
        // Test that all re-exports are accessible
        let _ = SavedMessagesError::topic_not_found(123);
        let _ = Result::<()>::Ok(());
        let _ = SavedMessagesManager::new();
        let _ = DraftMessage::new(0);
        let _ = OrderedMessages::new();

        let user_id = UserId::new(123).unwrap();
        let _ = SavedMessagesTopicId::from_user_id(user_id);
        let dialog_id = rustgram_types::DialogId::from_user(user_id);
        let _ = TopicList::new(dialog_id);
    }

    #[test]
    fn test_constants_module() {
        // Test that constants are accessible
        assert_eq!(constants::MAX_GET_HISTORY, 100);
        assert_eq!(constants::MIN_PINNED_TOPIC_ORDER, 9_223_372_036_854_716_160);
        assert_eq!(constants::HIDDEN_AUTHOR_DIALOG_ID, 2_666_000);
    }

    #[test]
    fn test_end_to_end_workflow() {
        // Create a manager
        let mut manager = SavedMessagesManager::new();

        // Create topic IDs
        let user_id1 = UserId::new(123).unwrap();
        let user_id2 = UserId::new(456).unwrap();
        let dialog_id1 = rustgram_types::DialogId::from_user(user_id1);
        let dialog_id2 = rustgram_types::DialogId::from_user(user_id2);
        let topic_id1 = SavedMessagesTopicId::from_user_id(user_id1);
        let topic_id2 = SavedMessagesTopicId::from_user_id(user_id2);

        // Check that topics don't exist yet
        assert!(!manager.has_topic(dialog_id1, topic_id1));
        assert!(!manager.has_topic(dialog_id2, topic_id2));

        // Test get_topic_id
        let result = manager.get_topic_id(dialog_id1, 789);
        assert!(result.is_ok());

        // Test get_topic_ids
        let result = manager.get_topic_ids(dialog_id1, &[1, 2, 3]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);

        // Test topic operations
        let message_id = rustgram_types::MessageId::from_server_id(100);
        let result = manager.on_topic_message_added(
            dialog_id1, topic_id1, message_id, 123456, false, true, false,
        );
        assert!(result.is_ok());

        // Test pinned operations
        let result = manager.toggle_saved_messages_topic_is_pinned(topic_id1, true);
        assert!(result.is_ok());

        let result = manager.toggle_saved_messages_topic_is_pinned(topic_id1, false);
        assert!(result.is_ok());

        let result = manager.set_pinned_saved_messages_topics(vec![topic_id1, topic_id2]);
        assert!(result.is_ok());
        assert_eq!(manager.topic_list().pinned_topic_ids().len(), 2);

        // Test update operations
        let result = manager.on_topic_draft_message_updated(dialog_id1, topic_id1, 123456);
        assert!(result.is_ok());

        let result = manager.on_update_topic_is_marked_as_unread(dialog_id1, topic_id1, true);
        assert!(result.is_ok());

        let result = manager.on_topic_reaction_count_changed(dialog_id1, topic_id1, 5, true);
        assert!(result.is_ok());

        let result = manager.on_update_read_monoforum_inbox(dialog_id1, topic_id1, message_id);
        assert!(result.is_ok());

        let result = manager.on_update_read_all_monoforum_inbox(dialog_id1, message_id);
        assert!(result.is_ok());

        // Test deletion
        let result = manager.on_topic_message_deleted(dialog_id1, topic_id1, message_id);
        assert!(result.is_ok());

        let result = manager.on_all_dialog_messages_deleted(dialog_id1);
        assert!(result.is_ok());

        // Note: delete_saved_messages_topic_history will fail if topic doesn't exist
        // This is expected behavior - we're testing that the error is handled
        let _ = manager.delete_saved_messages_topic_history(topic_id1);
    }

    #[test]
    fn test_topic_id_from_different_types() {
        // Test creating topic IDs from different dialog types
        let user_id = UserId::new(123).unwrap();
        let chat_id = ChatId::new(456).unwrap();

        let topic_id_from_user = SavedMessagesTopicId::from_user_id(user_id);
        let topic_id_from_chat = SavedMessagesTopicId::from_chat_id(chat_id);

        assert!(topic_id_from_user.is_valid());
        assert!(topic_id_from_chat.is_valid());
    }

    #[test]
    fn test_error_handling() {
        // Test error creation and display
        let err = SavedMessagesError::topic_not_found(123);
        assert_eq!(err.to_string(), "topic not found: 123");

        let err = SavedMessagesError::invalid_topic_id(0);
        assert_eq!(err.to_string(), "invalid topic ID: 0");

        let err = SavedMessagesError::max_pinned_exceeded(6, 5);
        assert_eq!(err.to_string(), "maximum pinned topics exceeded: 6 > 5");
    }

    #[test]
    fn test_topic_date_ordering() {
        use crate::TopicDate;
        use rustgram_types::UserId;

        let user_id1 = UserId::new(123).unwrap();
        let user_id2 = UserId::new(456).unwrap();
        let topic_id1 = SavedMessagesTopicId::from_user_id(user_id1);
        let topic_id2 = SavedMessagesTopicId::from_user_id(user_id2);

        let date1 = TopicDate::new(100, topic_id1);
        let date2 = TopicDate::new(200, topic_id2);

        // Higher order should come first (Greater in reverse order)
        assert_eq!(date1.cmp(&date2), std::cmp::Ordering::Greater);
        assert_eq!(date2.cmp(&date1), std::cmp::Ordering::Less);
    }
}
