//! Saved messages topic ID implementation.

use rustgram_types::DialogId;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Saved messages topic ID.
///
/// This is a wrapper around DialogId that uniquely identifies a saved messages topic.
/// It combines a dialog ID with an optional topic-specific identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedMessagesTopicId {
    /// The underlying dialog ID.
    dialog_id: DialogId,
    /// Unique identifier for topic ordering and hashing.
    unique_id: i64,
}

impl SavedMessagesTopicId {
    /// Creates a new saved messages topic ID from a dialog ID.
    ///
    /// # Arguments
    /// * `dialog_id` - The dialog ID to wrap.
    ///
    /// # Returns
    /// A new `SavedMessagesTopicId` instance.
    #[inline]
    pub const fn new(dialog_id: DialogId) -> Self {
        let unique_id = match dialog_id {
            DialogId::User(user_id) => user_id.get(),
            DialogId::Chat(chat_id) => chat_id.get(),
            DialogId::Channel(channel_id) => channel_id.get(),
            DialogId::SecretChat(secret_chat_id) => secret_chat_id.get() as i64,
        };
        Self {
            dialog_id,
            unique_id,
        }
    }

    /// Returns the underlying dialog ID.
    #[inline]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the unique ID for hashing and comparison.
    #[inline]
    pub const fn get_unique_id(&self) -> i64 {
        self.unique_id
    }

    /// Checks if this is a valid topic ID.
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.dialog_id.is_valid()
    }

    /// Creates a topic ID from a user dialog ID.
    #[inline]
    pub fn from_user_id(user_id: rustgram_types::UserId) -> Self {
        Self::new(DialogId::from_user(user_id))
    }

    /// Creates a topic ID from a chat dialog ID.
    #[inline]
    pub fn from_chat_id(chat_id: rustgram_types::ChatId) -> Self {
        Self::new(DialogId::from_chat(chat_id))
    }

    /// Creates a topic ID from a channel dialog ID.
    #[inline]
    pub fn from_channel_id(channel_id: rustgram_types::ChannelId) -> Self {
        Self::new(DialogId::from_channel(channel_id))
    }

    /// Creates a topic ID from a secret chat dialog ID.
    #[inline]
    pub fn from_secret_chat_id(secret_chat_id: rustgram_types::SecretChatId) -> Self {
        Self::new(DialogId::from_secret_chat(secret_chat_id))
    }

    /// Returns the inner unique ID value.
    #[inline]
    pub const fn get(&self) -> i64 {
        self.unique_id
    }
}

impl Default for SavedMessagesTopicId {
    fn default() -> Self {
        Self::new(DialogId::default())
    }
}

impl Hash for SavedMessagesTopicId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unique_id.hash(state);
    }
}

impl fmt::Display for SavedMessagesTopicId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "saved_topic_{}", self.unique_id)
    }
}

impl From<DialogId> for SavedMessagesTopicId {
    fn from(dialog_id: DialogId) -> Self {
        Self::new(dialog_id)
    }
}

impl From<SavedMessagesTopicId> for DialogId {
    fn from(topic_id: SavedMessagesTopicId) -> Self {
        topic_id.dialog_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::{ChannelId, ChatId, SecretChatId, UserId};

    #[test]
    fn test_topic_id_from_user() {
        let user_id = UserId::new(123456).unwrap();
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        assert!(topic_id.is_valid());
        assert_eq!(topic_id.get_unique_id(), 123456);
    }

    #[test]
    fn test_topic_id_from_chat() {
        let chat_id = ChatId::new(123456789).unwrap();
        let topic_id = SavedMessagesTopicId::from_chat_id(chat_id);
        assert!(topic_id.is_valid());
        assert_eq!(topic_id.get_unique_id(), 123456789);
    }

    #[test]
    fn test_topic_id_from_channel() {
        let channel_id = ChannelId::new(100000000000).unwrap();
        let topic_id = SavedMessagesTopicId::from_channel_id(channel_id);
        assert!(topic_id.is_valid());
        assert_eq!(topic_id.get_unique_id(), 100000000000);
    }

    #[test]
    fn test_topic_id_from_secret_chat() {
        let secret_chat_id = SecretChatId::new(123).unwrap();
        let topic_id = SavedMessagesTopicId::from_secret_chat_id(secret_chat_id);
        assert!(topic_id.is_valid());
        assert_eq!(topic_id.get_unique_id(), 123);
    }

    #[test]
    fn test_topic_id_default() {
        let topic_id = SavedMessagesTopicId::default();
        assert!(!topic_id.is_valid());
    }

    #[test]
    fn test_topic_id_equality() {
        let user_id = UserId::new(123456).unwrap();
        let topic_id1 = SavedMessagesTopicId::from_user_id(user_id);
        let topic_id2 = SavedMessagesTopicId::from_user_id(user_id);
        assert_eq!(topic_id1, topic_id2);

        let user_id2 = UserId::new(789012).unwrap();
        let topic_id3 = SavedMessagesTopicId::from_user_id(user_id2);
        assert_ne!(topic_id1, topic_id3);
    }

    #[test]
    fn test_topic_id_hash() {
        use std::collections::hash_map::DefaultHasher;

        let user_id = UserId::new(123456).unwrap();
        let topic_id1 = SavedMessagesTopicId::from_user_id(user_id);
        let topic_id2 = SavedMessagesTopicId::from_user_id(user_id);

        let mut hasher1 = DefaultHasher::new();
        topic_id1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        topic_id2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_topic_id_display() {
        let user_id = UserId::new(123456).unwrap();
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        assert_eq!(format!("{}", topic_id), "saved_topic_123456");
    }

    #[test]
    fn test_topic_id_from_dialog_id() {
        let user_id = UserId::new(123456).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let topic_id = SavedMessagesTopicId::new(dialog_id);
        assert_eq!(topic_id.dialog_id(), dialog_id);
        assert_eq!(topic_id.get_unique_id(), 123456);
    }

    #[test]
    fn test_topic_id_into_dialog_id() {
        let user_id = UserId::new(123456).unwrap();
        let topic_id = SavedMessagesTopicId::from_user_id(user_id);
        let dialog_id: DialogId = topic_id.into();
        assert_eq!(dialog_id, DialogId::from_user(user_id));
    }
}
