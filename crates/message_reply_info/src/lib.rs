// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Message Reply Info
//!
//! Information about replies to a message.
//!
//! ## TDLib Alignment
//!
//! This type aligns with TDLib's `MessageReplyInfo` struct.
//! - TDLib header: `td/telegram/MessageReplyInfo.h`
//! - TDLib type: Struct with reply_count, pts, recent_replier_dialog_ids, etc.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_reply_info::MessageReplyInfo;
//! use rustgram_types::{DialogId, MessageId, ChannelId};
//!
//! let info = MessageReplyInfo::new(
//!     5,
//!     100,
//!     vec![],
//!     vec![],
//!     ChannelId::new(1000000000).unwrap(),
//!     MessageId::from_server_id(200),
//!     MessageId::from_server_id(201),
//!     MessageId::from_server_id(202),
//!     false,
//!     false
//! );
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

use rustgram_types::{ChannelId, DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stub for MinChannel.
///
/// TODO: Full implementation when channel module is available.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MinChannel {
    /// Channel ID
    pub channel_id: ChannelId,
    /// Channel participant count
    pub participant_count: i32,
}

impl MinChannel {
    /// Creates a new MinChannel.
    pub fn new(channel_id: ChannelId, participant_count: i32) -> Self {
        Self {
            channel_id,
            participant_count,
        }
    }
}

/// Information about replies to a message.
///
/// Contains comprehensive reply information including:
/// - Total reply count
/// - PTS (Permanent Timestamp) for server-side ordering
/// - Recent repliers' dialog IDs
/// - Channel-specific information
/// - Read message IDs
///
/// ## Field Descriptions
///
/// - `reply_count`: Total number of replies (-1 if empty/unknown)
/// - `pts`: Permanent timestamp for server-side ordering
/// - `recent_replier_dialog_ids`: IDs of dialogs that recently replied (max 3)
/// - `replier_min_channels`: Channels with minimal info for repliers
/// - `channel_id`: ID of the channel (for comments)
/// - `max_message_id`: Highest message ID among replies
/// - `last_read_inbox_message_id`: Last read incoming reply message ID
/// - `last_read_outbox_message_id`: Last read outgoing reply message ID
/// - `is_comment`: Whether this is a comment (as opposed to a reply)
/// - `is_dropped`: Whether reply info was dropped (not available)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageReplyInfo {
    /// Total number of replies (-1 if unknown)
    reply_count: i32,

    /// Permanent timestamp for server-side ordering
    pts: i32,

    /// IDs of dialogs that recently replied (max 3)
    recent_replier_dialog_ids: Vec<DialogId>,

    /// Channels with minimal info for repliers
    replier_min_channels: Vec<(ChannelId, MinChannel)>,

    /// ID of the channel (for comments)
    channel_id: ChannelId,

    /// Highest message ID among replies
    max_message_id: MessageId,

    /// Last read incoming reply message ID
    last_read_inbox_message_id: MessageId,

    /// Last read outgoing reply message ID
    last_read_outbox_message_id: MessageId,

    /// Whether this is a comment (as opposed to a reply)
    is_comment: bool,

    /// Whether reply info was dropped (not available)
    is_dropped: bool,
}

impl MessageReplyInfo {
    /// Maximum number of recent repliers to track.
    pub const MAX_RECENT_REPLIERS: usize = 3;

    /// Creates a new MessageReplyInfo.
    ///
    /// # Arguments
    ///
    /// * `reply_count` - Total number of replies (-1 if unknown)
    /// * `pts` - Permanent timestamp for server-side ordering
    /// * `recent_replier_dialog_ids` - IDs of dialogs that recently replied
    /// * `replier_min_channels` - Channels with minimal info for repliers
    /// * `channel_id` - ID of the channel (for comments)
    /// * `max_message_id` - Highest message ID among replies
    /// * `last_read_inbox_message_id` - Last read incoming reply message ID
    /// * `last_read_outbox_message_id` - Last read outgoing reply message ID
    /// * `is_comment` - Whether this is a comment
    /// * `is_dropped` - Whether reply info was dropped
    #[must_use]
    pub fn new(
        reply_count: i32,
        pts: i32,
        recent_replier_dialog_ids: Vec<DialogId>,
        replier_min_channels: Vec<(ChannelId, MinChannel)>,
        channel_id: ChannelId,
        max_message_id: MessageId,
        last_read_inbox_message_id: MessageId,
        last_read_outbox_message_id: MessageId,
        is_comment: bool,
        is_dropped: bool,
    ) -> Self {
        Self {
            reply_count,
            pts,
            recent_replier_dialog_ids,
            replier_min_channels,
            channel_id,
            max_message_id,
            last_read_inbox_message_id,
            last_read_outbox_message_id,
            is_comment,
            is_dropped,
        }
    }

    /// Returns the reply count.
    #[must_use]
    pub const fn reply_count(&self) -> i32 {
        self.reply_count
    }

    /// Returns the PTS value.
    #[must_use]
    pub const fn pts(&self) -> i32 {
        self.pts
    }

    /// Returns the recent replier dialog IDs.
    #[must_use]
    pub fn recent_replier_dialog_ids(&self) -> &[DialogId] {
        &self.recent_replier_dialog_ids
    }

    /// Returns the replier min channels.
    #[must_use]
    pub fn replier_min_channels(&self) -> &[(ChannelId, MinChannel)] {
        &self.replier_min_channels
    }

    /// Returns the channel ID.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the max message ID.
    #[must_use]
    pub const fn max_message_id(&self) -> MessageId {
        self.max_message_id
    }

    /// Returns the last read inbox message ID.
    #[must_use]
    pub const fn last_read_inbox_message_id(&self) -> MessageId {
        self.last_read_inbox_message_id
    }

    /// Returns the last read outbox message ID.
    #[must_use]
    pub const fn last_read_outbox_message_id(&self) -> MessageId {
        self.last_read_outbox_message_id
    }

    /// Returns `true` if this is a comment.
    #[must_use]
    pub const fn is_comment(&self) -> bool {
        self.is_comment
    }

    /// Returns `true` if reply info was dropped.
    #[must_use]
    pub const fn is_dropped(&self) -> bool {
        self.is_dropped
    }

    /// Returns `true` if this reply info is empty (no replies).
    ///
    /// Empty means reply_count is negative (-1), indicating
    /// no reply information is available.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.reply_count < 0
    }

    /// Returns `true` if this reply info was dropped.
    ///
    /// Dropped means the reply information is not available,
    /// possibly due to message age or server limitations.
    #[must_use]
    pub const fn was_dropped(&self) -> bool {
        self.is_dropped
    }

    /// Returns `true` if this reply info has valid data.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.reply_count >= 0 && !self.is_dropped
    }

    /// Returns the number of recent repliers.
    #[must_use]
    pub fn recent_replier_count(&self) -> usize {
        self.recent_replier_dialog_ids.len()
    }

    /// Sets the reply count.
    pub fn set_reply_count(&mut self, count: i32) {
        self.reply_count = count;
    }

    /// Sets the dropped flag.
    pub fn set_dropped(&mut self, dropped: bool) {
        self.is_dropped = dropped;
    }

    /// Adds a replier to the recent repliers list.
    ///
    /// Maintains maximum of MAX_RECENT_REPLIERS entries.
    pub fn add_replier(&mut self, dialog_id: DialogId) {
        self.recent_replier_dialog_ids.retain(|&id| id != dialog_id);

        self.recent_replier_dialog_ids.insert(0, dialog_id);

        if self.recent_replier_dialog_ids.len() > Self::MAX_RECENT_REPLIERS {
            self.recent_replier_dialog_ids
                .truncate(Self::MAX_RECENT_REPLIERS);
        }
    }

    /// Increments the reply count.
    pub fn increment_reply_count(&mut self) {
        if self.reply_count < 0 {
            self.reply_count = 1;
        } else {
            self.reply_count += 1;
        }
    }
}

impl Default for MessageReplyInfo {
    fn default() -> Self {
        Self {
            reply_count: -1,
            pts: -1,
            recent_replier_dialog_ids: Vec::new(),
            replier_min_channels: Vec::new(),
            channel_id: ChannelId::default(),
            max_message_id: MessageId::default(),
            last_read_inbox_message_id: MessageId::default(),
            last_read_outbox_message_id: MessageId::default(),
            is_comment: false,
            is_dropped: false,
        }
    }
}

impl fmt::Display for MessageReplyInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "no reply info");
        }

        let kind = if self.is_comment { "comment" } else { "reply" };

        if self.is_dropped {
            write!(f, "{} info dropped", kind)
        } else {
            write!(f, "{}{} ({})", self.reply_count, kind, kind)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    // Constructor tests (2)
    #[test]
    fn test_new() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let channel_id = ChannelId::new(1000000000).unwrap();

        let info = MessageReplyInfo::new(
            5,
            100,
            vec![dialog_id],
            vec![],
            channel_id,
            MessageId::from_server_id(200),
            MessageId::from_server_id(201),
            MessageId::from_server_id(202),
            false,
            false,
        );

        assert_eq!(info.reply_count(), 5);
        assert!(!info.is_empty());
        assert!(!info.is_dropped());
    }

    #[test]
    fn test_default() {
        let info = MessageReplyInfo::default();
        assert!(info.is_empty());
        assert_eq!(info.reply_count(), -1);
    }

    // Property tests (10)
    #[test]
    fn test_reply_count() {
        let info = MessageReplyInfo::default();
        assert_eq!(info.reply_count(), -1);
    }

    #[test]
    fn test_pts() {
        let info = MessageReplyInfo::default();
        assert_eq!(info.pts(), -1);
    }

    #[test]
    fn test_recent_replier_dialog_ids() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![dialog_id],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        assert_eq!(info.recent_replier_dialog_ids().len(), 1);
        assert_eq!(info.recent_replier_dialog_ids()[0], dialog_id);
    }

    #[test]
    fn test_replier_min_channels() {
        let channel_id = ChannelId::new(1000000000).unwrap();
        let min_channel = MinChannel::new(channel_id, 100);
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![(channel_id, min_channel.clone())],
            channel_id,
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        assert_eq!(info.replier_min_channels().len(), 1);
    }

    #[test]
    fn test_channel_id() {
        let channel_id = ChannelId::new(1000000000).unwrap();
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            channel_id,
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        assert_eq!(info.channel_id(), channel_id);
    }

    #[test]
    fn test_max_message_id() {
        let max_id = MessageId::from_server_id(200);
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            max_id,
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        assert_eq!(info.max_message_id(), max_id);
    }

    #[test]
    fn test_last_read_inbox_message_id() {
        let inbox_id = MessageId::from_server_id(201);
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            inbox_id,
            MessageId::default(),
            false,
            false,
        );

        assert_eq!(info.last_read_inbox_message_id(), inbox_id);
    }

    #[test]
    fn test_last_read_outbox_message_id() {
        let outbox_id = MessageId::from_server_id(202);
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            outbox_id,
            false,
            false,
        );

        assert_eq!(info.last_read_outbox_message_id(), outbox_id);
    }

    #[test]
    fn test_is_comment() {
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            true,
            false,
        );

        assert!(info.is_comment());
    }

    #[test]
    fn test_is_dropped() {
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            true,
        );

        assert!(info.is_dropped());
    }

    // State tests (5)
    #[test]
    fn test_is_empty() {
        let info = MessageReplyInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_is_not_empty() {
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        assert!(!info.is_empty());
    }

    #[test]
    fn test_was_dropped() {
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            true,
        );

        assert!(info.was_dropped());
    }

    #[test]
    fn test_was_not_dropped() {
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        assert!(!info.was_dropped());
    }

    #[test]
    fn test_is_valid() {
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        assert!(info.is_valid());
    }

    // Method tests (6)
    #[test]
    fn test_recent_replier_count() {
        let user_id1 = UserId::new(123).unwrap();
        let user_id2 = UserId::new(456).unwrap();
        let dialog_id1 = DialogId::from_user(user_id1);
        let dialog_id2 = DialogId::from_user(user_id2);

        let info = MessageReplyInfo::new(
            5,
            100,
            vec![dialog_id1, dialog_id2],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        assert_eq!(info.recent_replier_count(), 2);
    }

    #[test]
    fn test_set_reply_count() {
        let mut info = MessageReplyInfo::default();
        info.set_reply_count(10);
        assert_eq!(info.reply_count(), 10);
    }

    #[test]
    fn test_set_dropped() {
        let mut info = MessageReplyInfo::default();
        info.set_dropped(true);
        assert!(info.is_dropped());
    }

    #[test]
    fn test_add_replier() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let mut info = MessageReplyInfo::default();
        info.add_replier(dialog_id);

        assert_eq!(info.recent_replier_count(), 1);
        assert_eq!(info.recent_replier_dialog_ids()[0], dialog_id);
    }

    #[test]
    fn test_add_replier_existing() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let mut info = MessageReplyInfo::default();
        info.add_replier(dialog_id);
        info.add_replier(dialog_id);

        assert_eq!(info.recent_replier_count(), 1);
    }

    #[test]
    fn test_add_replier_max_limit() {
        let mut info = MessageReplyInfo::default();

        for i in 0..10 {
            let user_id = UserId::new(i + 1).unwrap();
            let dialog_id = DialogId::from_user(user_id);
            info.add_replier(dialog_id);
        }

        assert_eq!(info.recent_replier_count(), 3);
    }

    #[test]
    fn test_increment_reply_count_from_empty() {
        let mut info = MessageReplyInfo::default();
        info.increment_reply_count();
        assert_eq!(info.reply_count(), 1);
    }

    #[test]
    fn test_increment_reply_count_from_existing() {
        let mut info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        info.increment_reply_count();
        assert_eq!(info.reply_count(), 6);
    }

    // Clone tests (2)
    #[test]
    fn test_clone() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let info1 = MessageReplyInfo::new(
            5,
            100,
            vec![dialog_id],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_clone_independence() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let mut info1 = MessageReplyInfo::new(
            5,
            100,
            vec![dialog_id],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        let info2 = info1.clone();
        info1.set_reply_count(10);

        assert_eq!(info2.reply_count(), 5);
    }

    // Display tests (3)
    #[test]
    fn test_display_empty() {
        let info = MessageReplyInfo::default();
        let display = format!("{}", info);
        assert!(display.contains("no reply info"));
    }

    #[test]
    fn test_display_reply() {
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            false,
        );

        let display = format!("{}", info);
        assert!(display.contains("5") && display.contains("reply"));
    }

    #[test]
    fn test_display_comment() {
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            true,
            false,
        );

        let display = format!("{}", info);
        assert!(display.contains("5") && display.contains("comment"));
    }

    #[test]
    fn test_display_dropped() {
        let info = MessageReplyInfo::new(
            5,
            100,
            vec![],
            vec![],
            ChannelId::default(),
            MessageId::default(),
            MessageId::default(),
            MessageId::default(),
            false,
            true,
        );

        let display = format!("{}", info);
        assert!(display.contains("dropped"));
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let channel_id = ChannelId::new(1000000000).unwrap();

        let info = MessageReplyInfo::new(
            5,
            100,
            vec![dialog_id],
            vec![],
            channel_id,
            MessageId::from_server_id(200),
            MessageId::from_server_id(201),
            MessageId::from_server_id(202),
            true,
            false,
        );

        let json = serde_json::to_string(&info).unwrap();
        let parsed: MessageReplyInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, parsed);
    }

    #[test]
    fn test_serialize_empty() {
        let info = MessageReplyInfo::default();
        let json = serde_json::to_string(&info).unwrap();
        let parsed: MessageReplyInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, parsed);
    }
}
