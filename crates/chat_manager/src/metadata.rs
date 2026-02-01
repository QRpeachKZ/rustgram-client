//! Metadata structures for chats and channels.

use rustgram_types::access::AccessHash;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Metadata for a basic group chat.
///
/// Basic groups are small group chats (up to ~200 members) that don't have
/// a separate channel ID.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `ChatManager::Chat` struct.
/// This is a simplified version containing the most essential fields.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatMetadata {
    /// Chat title.
    pub title: String,

    /// Number of participants in the chat.
    pub participant_count: i32,

    /// Whether the chat is currently active.
    pub is_active: bool,

    /// Chat creation date.
    pub date: i32,

    /// Version of the chat (incremented on changes).
    pub version: i32,
}

impl ChatMetadata {
    /// Creates new chat metadata with default values.
    ///
    /// # Arguments
    ///
    /// * `title` - The chat title
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatMetadata;
    ///
    /// let metadata = ChatMetadata::new("My Group".to_string());
    /// assert_eq!(metadata.title, "My Group");
    /// assert_eq!(metadata.participant_count, 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(title: String) -> Self {
        Self {
            title,
            participant_count: 0,
            is_active: true,
            date: 0,
            version: -1,
        }
    }

    /// Creates chat metadata with the specified participant count.
    ///
    /// # Arguments
    ///
    /// * `title` - The chat title
    /// * `participant_count` - Number of participants
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatMetadata;
    ///
    /// let metadata = ChatMetadata::with_participants("My Group".to_string(), 25);
    /// assert_eq!(metadata.participant_count, 25);
    /// ```
    #[inline]
    #[must_use]
    pub const fn with_participants(title: String, participant_count: i32) -> Self {
        Self {
            title,
            participant_count,
            is_active: true,
            date: 0,
            version: -1,
        }
    }
}

impl Default for ChatMetadata {
    fn default() -> Self {
        Self::new(String::new())
    }
}

/// Metadata for a channel or megagroup.
///
/// Channels can be either broadcast channels or megagroups (large group chats).
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `ChatManager::Channel` struct.
/// This is a simplified version containing the most essential fields.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChannelMetadata {
    /// Channel title.
    pub title: String,

    /// Access hash for API calls.
    pub access_hash: AccessHash,

    /// Number of participants.
    pub participant_count: i32,

    /// Whether this is a broadcast channel.
    pub is_broadcast: bool,

    /// Whether this is a megagroup.
    pub is_megagroup: bool,

    /// Whether the channel is verified.
    pub is_verified: bool,

    /// Channel creation date.
    pub date: i32,

    /// Boost level of the channel.
    pub boost_level: i32,
}

impl ChannelMetadata {
    /// Creates new channel metadata.
    ///
    /// # Arguments
    ///
    /// * `title` - The channel title
    /// * `access_hash` - Access hash for API calls
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChannelMetadata;
    /// use rustgram_types::access::AccessHash;
    ///
    /// let access_hash = AccessHash::new(12345);
    /// let metadata = ChannelMetadata::new("My Channel".to_string(), access_hash);
    /// assert_eq!(metadata.title, "My Channel");
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(title: String, access_hash: AccessHash) -> Self {
        Self {
            title,
            access_hash,
            participant_count: 0,
            is_broadcast: false,
            is_megagroup: false,
            is_verified: false,
            date: 0,
            boost_level: 0,
        }
    }

    /// Creates channel metadata for a broadcast channel.
    ///
    /// # Arguments
    ///
    /// * `title` - The channel title
    /// * `access_hash` - Access hash for API calls
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChannelMetadata;
    /// use rustgram_types::access::AccessHash;
    ///
    /// let access_hash = AccessHash::new(12345);
    /// let metadata = ChannelMetadata::broadcast("My Channel".to_string(), access_hash);
    /// assert!(metadata.is_broadcast);
    /// ```
    #[inline]
    #[must_use]
    pub const fn broadcast(title: String, access_hash: AccessHash) -> Self {
        Self {
            title,
            access_hash,
            participant_count: 0,
            is_broadcast: true,
            is_megagroup: false,
            is_verified: false,
            date: 0,
            boost_level: 0,
        }
    }

    /// Creates channel metadata for a megagroup.
    ///
    /// # Arguments
    ///
    /// * `title` - The channel title
    /// * `access_hash` - Access hash for API calls
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChannelMetadata;
    /// use rustgram_types::access::AccessHash;
    ///
    /// let access_hash = AccessHash::new(12345);
    /// let metadata = ChannelMetadata::megagroup("My Group".to_string(), access_hash);
    /// assert!(metadata.is_megagroup);
    /// ```
    #[inline]
    #[must_use]
    pub const fn megagroup(title: String, access_hash: AccessHash) -> Self {
        Self {
            title,
            access_hash,
            participant_count: 0,
            is_broadcast: false,
            is_megagroup: true,
            is_verified: false,
            date: 0,
            boost_level: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== ChatMetadata Tests ==========

    #[test]
    fn test_chat_metadata_new() {
        let metadata = ChatMetadata::new("Test Group".to_string());
        assert_eq!(metadata.title, "Test Group");
        assert_eq!(metadata.participant_count, 0);
        assert!(metadata.is_active);
        assert_eq!(metadata.date, 0);
        assert_eq!(metadata.version, -1);
    }

    #[test]
    fn test_chat_metadata_with_participants() {
        let metadata = ChatMetadata::with_participants("Test Group".to_string(), 25);
        assert_eq!(metadata.title, "Test Group");
        assert_eq!(metadata.participant_count, 25);
        assert!(metadata.is_active);
    }

    #[test]
    fn test_chat_metadata_default() {
        let metadata = ChatMetadata::default();
        assert_eq!(metadata.title, "");
        assert_eq!(metadata.participant_count, 0);
    }

    #[test]
    fn test_chat_metadata_clone() {
        let metadata1 = ChatMetadata::with_participants("Test".to_string(), 10);
        let metadata2 = metadata1.clone();
        assert_eq!(metadata1, metadata2);
    }

    #[test]
    fn test_chat_metadata_equality() {
        let metadata1 = ChatMetadata::new("Test".to_string());
        let metadata2 = ChatMetadata::new("Test".to_string());
        let metadata3 = ChatMetadata::new("Different".to_string());

        assert_eq!(metadata1, metadata2);
        assert_ne!(metadata1, metadata3);
    }

    // ========== ChannelMetadata Tests ==========

    #[test]
    fn test_channel_metadata_new() {
        let access_hash = AccessHash::new(12345);
        let metadata = ChannelMetadata::new("Test Channel".to_string(), access_hash);

        assert_eq!(metadata.title, "Test Channel");
        assert_eq!(metadata.access_hash, access_hash);
        assert_eq!(metadata.participant_count, 0);
        assert!(!metadata.is_broadcast);
        assert!(!metadata.is_megagroup);
        assert!(!metadata.is_verified);
    }

    #[test]
    fn test_channel_metadata_broadcast() {
        let access_hash = AccessHash::new(12345);
        let metadata = ChannelMetadata::broadcast("Broadcast".to_string(), access_hash);

        assert!(metadata.is_broadcast);
        assert!(!metadata.is_megagroup);
        assert_eq!(metadata.title, "Broadcast");
    }

    #[test]
    fn test_channel_metadata_megagroup() {
        let access_hash = AccessHash::new(12345);
        let metadata = ChannelMetadata::megagroup("Megagroup".to_string(), access_hash);

        assert!(!metadata.is_broadcast);
        assert!(metadata.is_megagroup);
        assert_eq!(metadata.title, "Megagroup");
    }

    #[test]
    fn test_channel_metadata_clone() {
        let access_hash = AccessHash::new(12345);
        let metadata1 = ChannelMetadata::new("Test".to_string(), access_hash);
        let metadata2 = metadata1.clone();
        assert_eq!(metadata1, metadata2);
    }

    #[test]
    fn test_channel_metadata_equality() {
        let access_hash1 = AccessHash::new(12345);
        let access_hash2 = AccessHash::new(67890);
        let metadata1 = ChannelMetadata::new("Test".to_string(), access_hash1);
        let metadata2 = ChannelMetadata::new("Test".to_string(), access_hash1);
        let metadata3 = ChannelMetadata::new("Test".to_string(), access_hash2);

        assert_eq!(metadata1, metadata2);
        assert_ne!(metadata1, metadata3);
    }

    #[test]
    fn test_channel_metadata_with_participants() {
        let access_hash = AccessHash::new(12345);
        let mut metadata = ChannelMetadata::new("Test".to_string(), access_hash);
        metadata.participant_count = 100;

        assert_eq!(metadata.participant_count, 100);
    }

    #[test]
    fn test_channel_metadata_verified() {
        let access_hash = AccessHash::new(12345);
        let mut metadata = ChannelMetadata::new("Test".to_string(), access_hash);
        metadata.is_verified = true;

        assert!(metadata.is_verified);
    }

    #[test]
    fn test_channel_metadata_boost_level() {
        let access_hash = AccessHash::new(12345);
        let mut metadata = ChannelMetadata::new("Test".to_string(), access_hash);
        metadata.boost_level = 5;

        assert_eq!(metadata.boost_level, 5);
    }

    // ========== Mixed Tests ==========

    #[test]
    fn test_chat_vs_channel_metadata() {
        let chat = ChatMetadata::new("Group".to_string());
        let access_hash = AccessHash::new(12345);
        let channel = ChannelMetadata::new("Channel".to_string(), access_hash);

        assert_eq!(chat.title, "Group");
        assert_eq!(channel.title, "Channel");
        assert_ne!(chat.title, channel.title);
    }

    #[test]
    fn test_metadata_mutability() {
        let mut metadata = ChatMetadata::new("Test".to_string());
        metadata.participant_count = 50;
        metadata.is_active = false;

        assert_eq!(metadata.participant_count, 50);
        assert!(!metadata.is_active);
    }
}
