// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Message Link Info - Information about a link to a message.
//!
//! This module provides the [`MessageLinkInfo`] type which contains information
//! about a link to a message in a chat.
//!
//! ## Overview
//!
//! Message links can reference messages via username or channel ID, and may include
//! additional information such as timestamps, thread information, and comments.
//!
//! ## Examples
//!
//! ```
//! use rustgram_message_link_info::MessageLinkInfo;
//! use rustgram_message_id::MessageId;
//!
//! let info = MessageLinkInfo::builder()
//!     .with_message_id(MessageId::new(123))
//!     .with_username("example".to_string())
//!     .build();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_dialog_id::DialogId;
use rustgram_message_id::MessageId;
use std::hash::{Hash, Hasher};

/// Stub for ChannelId.
///
/// TODO: Replace with full rustgram-channel-id implementation when available.
///
/// Based on TDLib's `ChannelId` from `td/telegram/ChannelId.h`.
/// Valid channel IDs are in ranges:
/// - 0 < id < 1000000000000 - (1 << 31)  (normal channels)
/// - 1000000000000 + (1 << 31) + 1 <= id < 3000000000000  (monoforums)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChannelId(i64);

impl ChannelId {
    /// Maximum valid channel ID for normal channels.
    pub const MAX_CHANNEL_ID: i64 = 1_000_000_000_000 - (1 << 31);

    /// Minimum valid channel ID for monoforums.
    pub const MIN_MONOFORUM_CHANNEL_ID: i64 = 1_000_000_000_000 + (1 << 31) + 1;

    /// Maximum valid channel ID for monoforums.
    pub const MAX_MONOFORUM_CHANNEL_ID: i64 = 3_000_000_000_000;

    /// Creates a new [`ChannelId`] from an i64 value.
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the underlying i64 value.
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Returns `true` if this is a valid channel ID.
    pub fn is_valid(self) -> bool {
        (0 < self.0 && self.0 < Self::MAX_CHANNEL_ID)
            || (Self::MIN_MONOFORUM_CHANNEL_ID <= self.0 && self.0 < Self::MAX_MONOFORUM_CHANNEL_ID)
    }
}

impl std::fmt::Display for ChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "supergroup {}", self.0)
    }
}

/// Information about a link to a message.
///
/// Contains all the information needed to identify and access a message via a link.
///
/// # Examples
///
/// ```
/// use rustgram_message_link_info::MessageLinkInfo;
/// use rustgram_message_id::MessageId;
///
/// let info = MessageLinkInfo::builder()
///     .with_message_id(MessageId::new(123))
///     .with_username("example".to_string())
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MessageLinkInfo {
    /// The username of the chat (if linked by username).
    pub username: Option<String>,

    /// The channel ID (if linked by channel ID).
    pub channel_id: Option<ChannelId>,

    /// The message ID.
    pub message_id: MessageId,

    /// Whether this is a single message (not a thread).
    pub is_single: bool,

    /// Media timestamp in seconds (for video/audio messages).
    pub media_timestamp: i32,

    /// Top thread message ID (for thread links).
    pub top_thread_message_id: MessageId,

    /// Comment dialog ID.
    pub comment_dialog_id: DialogId,

    /// Comment message ID.
    pub comment_message_id: MessageId,

    /// Whether this link is for a comment.
    pub for_comment: bool,
}

impl MessageLinkInfo {
    /// Creates a builder for [`MessageLinkInfo`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_link_info::MessageLinkInfo;
    /// use rustgram_message_id::MessageId;
    ///
    /// let info = MessageLinkInfo::builder()
    ///     .with_message_id(MessageId::new(123))
    ///     .build();
    /// ```
    pub fn builder() -> MessageLinkInfoBuilder {
        MessageLinkInfoBuilder::new()
    }

    /// Returns `true` if this link uses a username.
    pub fn is_username_link(&self) -> bool {
        self.username.is_some()
    }

    /// Returns `true` if this link uses a channel ID.
    pub fn is_channel_link(&self) -> bool {
        self.channel_id.is_some()
    }

    /// Returns `true` if this link has thread information.
    pub fn has_thread(&self) -> bool {
        self.top_thread_message_id.get() != 0
    }

    /// Returns `true` if this link has comment information.
    pub fn has_comment(&self) -> bool {
        self.for_comment || self.comment_message_id.get() != 0
    }

    /// Returns `true` if this link has a media timestamp.
    pub fn has_media_timestamp(&self) -> bool {
        self.media_timestamp > 0
    }
}

impl Hash for MessageLinkInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.username.hash(state);
        self.channel_id.hash(state);
        self.message_id.hash(state);
        self.is_single.hash(state);
        self.media_timestamp.hash(state);
        self.top_thread_message_id.hash(state);
        self.comment_dialog_id.hash(state);
        self.comment_message_id.hash(state);
        self.for_comment.hash(state);
    }
}

/// Builder for [`MessageLinkInfo`].
///
/// # Examples
///
/// ```
/// use rustgram_message_link_info::MessageLinkInfo;
/// use rustgram_message_id::MessageId;
///
/// let info = MessageLinkInfo::builder()
///     .with_message_id(MessageId::new(123))
///     .with_username("example".to_string())
///     .with_is_single(true)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct MessageLinkInfoBuilder {
    username: Option<String>,
    channel_id: Option<ChannelId>,
    message_id: Option<MessageId>,
    is_single: bool,
    media_timestamp: i32,
    top_thread_message_id: Option<MessageId>,
    comment_dialog_id: Option<DialogId>,
    comment_message_id: Option<MessageId>,
    for_comment: bool,
}

impl MessageLinkInfoBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the username.
    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    /// Sets the channel ID.
    pub fn with_channel_id(mut self, channel_id: ChannelId) -> Self {
        self.channel_id = Some(channel_id);
        self
    }

    /// Sets the message ID.
    pub fn with_message_id(mut self, message_id: MessageId) -> Self {
        self.message_id = Some(message_id);
        self
    }

    /// Sets whether this is a single message.
    pub fn with_is_single(mut self, is_single: bool) -> Self {
        self.is_single = is_single;
        self
    }

    /// Sets the media timestamp.
    pub fn with_media_timestamp(mut self, media_timestamp: i32) -> Self {
        self.media_timestamp = media_timestamp;
        self
    }

    /// Sets the top thread message ID.
    pub fn with_top_thread_message_id(mut self, top_thread_message_id: MessageId) -> Self {
        self.top_thread_message_id = Some(top_thread_message_id);
        self
    }

    /// Sets the comment dialog ID.
    pub fn with_comment_dialog_id(mut self, comment_dialog_id: DialogId) -> Self {
        self.comment_dialog_id = Some(comment_dialog_id);
        self
    }

    /// Sets the comment message ID.
    pub fn with_comment_message_id(mut self, comment_message_id: MessageId) -> Self {
        self.comment_message_id = Some(comment_message_id);
        self
    }

    /// Sets whether this is for a comment.
    pub fn with_for_comment(mut self, for_comment: bool) -> Self {
        self.for_comment = for_comment;
        self
    }

    /// Builds the [`MessageLinkInfo`].
    pub fn build(self) -> MessageLinkInfo {
        MessageLinkInfo {
            username: self.username,
            channel_id: self.channel_id,
            message_id: self.message_id.unwrap_or_default(),
            is_single: self.is_single,
            media_timestamp: self.media_timestamp,
            top_thread_message_id: self.top_thread_message_id.unwrap_or_default(),
            comment_dialog_id: self.comment_dialog_id.unwrap_or_default(),
            comment_message_id: self.comment_message_id.unwrap_or_default(),
            for_comment: self.for_comment,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_id_default() {
        let id = ChannelId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_channel_id_new() {
        let id = ChannelId::new(1234567890);
        assert_eq!(id.get(), 1234567890);
        assert!(id.is_valid());
    }

    #[test]
    fn test_channel_id_zero() {
        let id = ChannelId::new(0);
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_channel_id_negative() {
        let id = ChannelId::new(-1);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_channel_id_normal_range() {
        let id = ChannelId::new(500_000_000_000);
        assert!(id.is_valid());
    }

    #[test]
    fn test_channel_id_monoforum_range() {
        let id = ChannelId::new(2_000_000_000_000);
        assert!(id.is_valid());
    }

    #[test]
    fn test_channel_id_invalid_high() {
        let id = ChannelId::new(4_000_000_000_000);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_channel_id_display() {
        let id = ChannelId::new(123);
        assert_eq!(format!("{}", id), "supergroup 123");
    }

    #[test]
    fn test_message_link_info_default() {
        let info = MessageLinkInfo::default();
        assert!(info.username.is_none());
        assert!(info.channel_id.is_none());
        assert_eq!(info.message_id.get(), 0);
        assert!(!info.is_single);
        assert_eq!(info.media_timestamp, 0);
        assert_eq!(info.top_thread_message_id.get(), 0);
        assert!(!info.for_comment);
    }

    #[test]
    fn test_message_link_info_builder_basic() {
        use rustgram_message_id::MessageId;

        let info = MessageLinkInfo::builder()
            .with_message_id(MessageId::new(123))
            .build();

        assert_eq!(info.message_id.get(), 123);
    }

    #[test]
    fn test_message_link_info_builder_full() {
        use rustgram_dialog_id::DialogId;
        use rustgram_message_id::MessageId;

        let info = MessageLinkInfo::builder()
            .with_username("example".to_string())
            .with_message_id(MessageId::new(123))
            .with_is_single(true)
            .with_media_timestamp(45)
            .with_top_thread_message_id(MessageId::new(10))
            .with_comment_dialog_id(DialogId::default())
            .with_comment_message_id(MessageId::new(20))
            .with_for_comment(true)
            .build();

        assert_eq!(info.username, Some("example".to_string()));
        assert_eq!(info.message_id.get(), 123);
        assert!(info.is_single);
        assert_eq!(info.media_timestamp, 45);
        assert_eq!(info.top_thread_message_id.get(), 10);
        assert!(info.for_comment);
        assert_eq!(info.comment_message_id.get(), 20);
    }

    #[test]
    fn test_message_link_info_builder_channel() {
        use rustgram_message_id::MessageId;

        let info = MessageLinkInfo::builder()
            .with_channel_id(ChannelId::new(1234567890))
            .with_message_id(MessageId::new(123))
            .build();

        assert!(info.channel_id.is_some());
        assert_eq!(info.channel_id.unwrap().get(), 1234567890);
    }

    #[test]
    fn test_is_username_link() {
        let info = MessageLinkInfo::builder()
            .with_username("example".to_string())
            .build();

        assert!(info.is_username_link());
        assert!(!info.is_channel_link());
    }

    #[test]
    fn test_is_channel_link() {
        let info = MessageLinkInfo::builder()
            .with_channel_id(ChannelId::new(1234567890))
            .build();

        assert!(!info.is_username_link());
        assert!(info.is_channel_link());
    }

    #[test]
    fn test_has_thread() {
        use rustgram_message_id::MessageId;

        let mut info = MessageLinkInfo::default();
        assert!(!info.has_thread());

        info.top_thread_message_id = MessageId::new(10);
        assert!(info.has_thread());
    }

    #[test]
    fn test_has_comment() {
        use rustgram_message_id::MessageId;

        let mut info = MessageLinkInfo::default();
        assert!(!info.has_comment());

        info.for_comment = true;
        assert!(info.has_comment());

        info.for_comment = false;
        info.comment_message_id = MessageId::new(20);
        assert!(info.has_comment());
    }

    #[test]
    fn test_has_media_timestamp() {
        let mut info = MessageLinkInfo::default();
        assert!(!info.has_media_timestamp());

        info.media_timestamp = 30;
        assert!(info.has_media_timestamp());
    }

    #[test]
    fn test_equality() {
        use rustgram_message_id::MessageId;

        let info1 = MessageLinkInfo::builder()
            .with_message_id(MessageId::new(123))
            .with_username("test".to_string())
            .build();

        let info2 = MessageLinkInfo::builder()
            .with_message_id(MessageId::new(123))
            .with_username("test".to_string())
            .build();

        assert_eq!(info1, info2);

        let info3 = MessageLinkInfo::builder()
            .with_message_id(MessageId::new(456))
            .build();

        assert_ne!(info1, info3);
    }

    #[test]
    fn test_clone() {
        use rustgram_message_id::MessageId;

        let info1 = MessageLinkInfo::builder()
            .with_message_id(MessageId::new(123))
            .build();

        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_hash() {
        use rustgram_message_id::MessageId;
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let info1 = MessageLinkInfo::builder()
            .with_message_id(MessageId::new(123))
            .build();

        map.insert(info1, "first");
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_channel_id_constants() {
        assert!(ChannelId::MAX_CHANNEL_ID > 0);
        assert!(ChannelId::MIN_MONOFORUM_CHANNEL_ID > ChannelId::MAX_CHANNEL_ID);
        assert!(ChannelId::MAX_MONOFORUM_CHANNEL_ID > ChannelId::MIN_MONOFORUM_CHANNEL_ID);
    }

    #[test]
    fn test_exclusive_username_channel() {
        // A link should have either username or channel_id, not both
        let info = MessageLinkInfo::builder()
            .with_username("example".to_string())
            .with_channel_id(ChannelId::new(1234567890))
            .build();

        // Both are set (builder allows this, but in practice only one should be used)
        assert!(info.is_username_link());
        assert!(info.is_channel_link());
    }
}
