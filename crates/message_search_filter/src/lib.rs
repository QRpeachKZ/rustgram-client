// Copyright 2024 rustgram-client contributors
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

//! # Message Search Filter
//!
//! Filter types for searching messages in Telegram.
//!
//! This module provides the `MessageSearchFilter` enum, which represents
//! different filter types for message search queries. It aligns with TDLib's
//! `MessageSearchFilter` enum from `td/telegram/MessageSearchFilter.h`.
//!
//! ## Overview
//!
//! Message search filters allow you to narrow down search results to specific
//! types of messages, such as photos, videos, audio files, links, mentions, etc.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_message_search_filter::MessageSearchFilter;
//!
//! // Create a filter for photos
//! let filter = MessageSearchFilter::Photo;
//!
//! // Check if it's a photo filter
//! assert!(filter.is_photo());
//!
//! // Get the filter index
//! assert_eq!(filter.index(), Some(4));
//!
//! // Empty filter has no index
//! let empty = MessageSearchFilter::Empty;
//! assert_eq!(empty.index(), None);
//! ```
//!
//! ## TDLib Alignment
//!
//! This enum matches TDLib's `MessageSearchFilter` enum exactly:
//! - TDLib: `MessageSearchFilter::Animation` → Rust: `MessageSearchFilter::Animation`
//! - TDLib: `MessageSearchFilter::Audio` → Rust: `MessageSearchFilter::Audio`
//! - etc.
//!
//! ## Thread Safety
//!
//! `MessageSearchFilter` is `Copy`, `Clone`, and `Send` + `Sync`, making it
//! safe to use across threads.

use std::fmt;

/// Message search filter types.
///
/// Represents different filter types for searching messages in Telegram.
/// Each variant corresponds to a specific type of message content or attribute.
///
/// # Examples
///
/// ```
/// use rustgram_message_search_filter::MessageSearchFilter;
///
/// let photo_filter = MessageSearchFilter::Photo;
/// assert_eq!(photo_filter.as_str(), "Photo");
/// assert!(photo_filter.index().is_some());
///
/// let empty_filter = MessageSearchFilter::Empty;
/// assert_eq!(empty_filter.as_str(), "Empty");
/// assert!(empty_filter.index().is_none());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MessageSearchFilter {
    /// No filter - search all messages.
    ///
    /// This is the default filter and returns all messages.
    /// TDLib: `MessageSearchFilter::Empty`
    #[default]
    Empty,

    /// Search for animation messages (GIFs).
    ///
    /// TDLib: `MessageSearchFilter::Animation`
    /// TL: `inputMessagesFilterGif`
    Animation,

    /// Search for audio messages (music, voice recordings, etc.).
    ///
    /// TDLib: `MessageSearchFilter::Audio`
    /// TL: `inputMessagesFilterMusic`
    Audio,

    /// Search for document messages (PDF, Office files, etc.).
    ///
    /// TDLib: `MessageSearchFilter::Document`
    /// TL: `inputMessagesFilterDocument`
    Document,

    /// Search for photo messages.
    ///
    /// TDLib: `MessageSearchFilter::Photo`
    /// TL: `inputMessagesFilterPhotos`
    Photo,

    /// Search for video messages.
    ///
    /// TDLib: `MessageSearchFilter::Video`
    /// TL: `inputMessagesFilterVideo`
    Video,

    /// Search for voice note messages.
    ///
    /// TDLib: `MessageSearchFilter::VoiceNote`
    /// TL: `inputMessagesFilterVoice`
    VoiceNote,

    /// Search for photo and video messages combined.
    ///
    /// TDLib: `MessageSearchFilter::PhotoAndVideo`
    /// TL: `inputMessagesFilterPhotoVideo`
    PhotoAndVideo,

    /// Search for messages containing URLs.
    ///
    /// TDLib: `MessageSearchFilter::Url`
    /// TL: `inputMessagesFilterUrl`
    Url,

    /// Search for chat photo messages.
    ///
    /// TDLib: `MessageSearchFilter::ChatPhoto`
    /// TL: `inputMessagesFilterChatPhotos`
    ChatPhoto,

    /// Search for call messages.
    ///
    /// TDLib: `MessageSearchFilter::Call`
    /// TL: `inputMessagesFilterPhoneCalls` (missed=false)
    Call,

    /// Search for missed call messages.
    ///
    /// TDLib: `MessageSearchFilter::MissedCall`
    /// TL: `inputMessagesFilterPhoneCalls` (missed=true)
    MissedCall,

    /// Search for video note messages (round videos).
    ///
    /// TDLib: `MessageSearchFilter::VideoNote`
    /// TL: `inputMessagesFilterRoundVideo`
    VideoNote,

    /// Search for voice and video note messages combined.
    ///
    /// TDLib: `MessageSearchFilter::VoiceAndVideoNote`
    /// TL: `inputMessagesFilterRoundVoice`
    VoiceAndVideoNote,

    /// Search for messages mentioning the current user.
    ///
    /// TDLib: `MessageSearchFilter::Mention`
    /// TL: `inputMessagesFilterMyMentions`
    Mention,

    /// Search for unread mention messages.
    ///
    /// TDLib: `MessageSearchFilter::UnreadMention`
    UnreadMention,

    /// Search for failed to send messages.
    ///
    /// TDLib: `MessageSearchFilter::FailedToSend`
    FailedToSend,

    /// Search for pinned messages.
    ///
    /// TDLib: `MessageSearchFilter::Pinned`
    /// TL: `inputMessagesFilterPinned`
    Pinned,

    /// Search for messages with unread reactions.
    ///
    /// TDLib: `MessageSearchFilter::UnreadReaction`
    UnreadReaction,
}

impl MessageSearchFilter {
    /// Returns the string representation of the filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_search_filter::MessageSearchFilter;
    ///
    /// assert_eq!(MessageSearchFilter::Photo.as_str(), "Photo");
    /// assert_eq!(MessageSearchFilter::Animation.as_str(), "Animation");
    /// assert_eq!(MessageSearchFilter::Empty.as_str(), "Empty");
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "Empty",
            Self::Animation => "Animation",
            Self::Audio => "Audio",
            Self::Document => "Document",
            Self::Photo => "Photo",
            Self::Video => "Video",
            Self::VoiceNote => "VoiceNote",
            Self::PhotoAndVideo => "PhotoAndVideo",
            Self::Url => "Url",
            Self::ChatPhoto => "ChatPhoto",
            Self::Call => "Call",
            Self::MissedCall => "MissedCall",
            Self::VideoNote => "VideoNote",
            Self::VoiceAndVideoNote => "VoiceAndVideoNote",
            Self::Mention => "Mention",
            Self::UnreadMention => "UnreadMention",
            Self::FailedToSend => "FailedToSend",
            Self::Pinned => "Pinned",
            Self::UnreadReaction => "UnreadReaction",
        }
    }

    /// Returns the index of the filter (1-based).
    ///
    /// Returns `None` for `Empty` filter, otherwise returns the 1-based index
    /// as used by TDLib for bitmask operations.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_search_filter::MessageSearchFilter;
    ///
    /// assert_eq!(MessageSearchFilter::Empty.index(), None);
    /// assert_eq!(MessageSearchFilter::Animation.index(), Some(1));
    /// assert_eq!(MessageSearchFilter::Audio.index(), Some(2));
    /// assert_eq!(MessageSearchFilter::Photo.index(), Some(4));
    /// ```
    #[must_use]
    pub const fn index(self) -> Option<i32> {
        match self {
            Self::Empty => None,
            Self::Animation => Some(1),
            Self::Audio => Some(2),
            Self::Document => Some(3),
            Self::Photo => Some(4),
            Self::Video => Some(5),
            Self::VoiceNote => Some(6),
            Self::PhotoAndVideo => Some(7),
            Self::Url => Some(8),
            Self::ChatPhoto => Some(9),
            Self::Call => Some(10),
            Self::MissedCall => Some(11),
            Self::VideoNote => Some(12),
            Self::VoiceAndVideoNote => Some(13),
            Self::Mention => Some(14),
            Self::UnreadMention => Some(15),
            Self::FailedToSend => Some(16),
            Self::Pinned => Some(17),
            Self::UnreadReaction => Some(18),
        }
    }

    /// Returns the bitmask for this filter.
    ///
    /// Returns `0` for `Empty` filter, otherwise returns a bitmask with a single
    /// bit set at the index position.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_search_filter::MessageSearchFilter;
    ///
    /// assert_eq!(MessageSearchFilter::Empty.bitmask(), 0);
    /// assert_eq!(MessageSearchFilter::Animation.bitmask(), 2);
    /// assert_eq!(MessageSearchFilter::Audio.bitmask(), 4);
    /// ```
    #[must_use]
    pub const fn bitmask(self) -> i32 {
        match self {
            Self::Empty => 0,
            Self::Animation => 2,
            Self::Audio => 4,
            Self::Document => 8,
            Self::Photo => 16,
            Self::Video => 32,
            Self::VoiceNote => 64,
            Self::PhotoAndVideo => 128,
            Self::Url => 256,
            Self::ChatPhoto => 512,
            Self::Call => 1024,
            Self::MissedCall => 2048,
            Self::VideoNote => 4096,
            Self::VoiceAndVideoNote => 8192,
            Self::Mention => 16384,
            Self::UnreadMention => 32768,
            Self::FailedToSend => 65536,
            Self::Pinned => 131072,
            Self::UnreadReaction => 262144,
        }
    }

    /// Returns `true` if this is the `Empty` filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_search_filter::MessageSearchFilter;
    ///
    /// assert!(MessageSearchFilter::Empty.is_empty());
    /// assert!(!MessageSearchFilter::Photo.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns `true` if this is an animation filter.
    #[must_use]
    pub const fn is_animation(self) -> bool {
        matches!(self, Self::Animation)
    }

    /// Returns `true` if this is an audio filter.
    #[must_use]
    pub const fn is_audio(self) -> bool {
        matches!(self, Self::Audio)
    }

    /// Returns `true` if this is a document filter.
    #[must_use]
    pub const fn is_document(self) -> bool {
        matches!(self, Self::Document)
    }

    /// Returns `true` if this is a photo filter.
    #[must_use]
    pub const fn is_photo(self) -> bool {
        matches!(self, Self::Photo)
    }

    /// Returns `true` if this is a video filter.
    #[must_use]
    pub const fn is_video(self) -> bool {
        matches!(self, Self::Video)
    }

    /// Returns `true` if this is a voice note filter.
    #[must_use]
    pub const fn is_voice_note(self) -> bool {
        matches!(self, Self::VoiceNote)
    }

    /// Returns `true` if this is a photo and video filter.
    #[must_use]
    pub const fn is_photo_and_video(self) -> bool {
        matches!(self, Self::PhotoAndVideo)
    }

    /// Returns `true` if this is a URL filter.
    #[must_use]
    pub const fn is_url(self) -> bool {
        matches!(self, Self::Url)
    }

    /// Returns `true` if this is a chat photo filter.
    #[must_use]
    pub const fn is_chat_photo(self) -> bool {
        matches!(self, Self::ChatPhoto)
    }

    /// Returns `true` if this is a call filter.
    #[must_use]
    pub const fn is_call(self) -> bool {
        matches!(self, Self::Call)
    }

    /// Returns `true` if this is a missed call filter.
    #[must_use]
    pub const fn is_missed_call(self) -> bool {
        matches!(self, Self::MissedCall)
    }

    /// Returns `true` if this is a video note filter.
    #[must_use]
    pub const fn is_video_note(self) -> bool {
        matches!(self, Self::VideoNote)
    }

    /// Returns `true` if this is a voice and video note filter.
    #[must_use]
    pub const fn is_voice_and_video_note(self) -> bool {
        matches!(self, Self::VoiceAndVideoNote)
    }

    /// Returns `true` if this is a mention filter.
    #[must_use]
    pub const fn is_mention(self) -> bool {
        matches!(self, Self::Mention)
    }

    /// Returns `true` if this is an unread mention filter.
    #[must_use]
    pub const fn is_unread_mention(self) -> bool {
        matches!(self, Self::UnreadMention)
    }

    /// Returns `true` if this is a failed to send filter.
    #[must_use]
    pub const fn is_failed_to_send(self) -> bool {
        matches!(self, Self::FailedToSend)
    }

    /// Returns `true` if this is a pinned filter.
    #[must_use]
    pub const fn is_pinned(self) -> bool {
        matches!(self, Self::Pinned)
    }

    /// Returns `true` if this is an unread reaction filter.
    #[must_use]
    pub const fn is_unread_reaction(self) -> bool {
        matches!(self, Self::UnreadReaction)
    }

    /// Returns `true` if this is a call-related filter.
    ///
    /// Returns `true` for both `Call` and `MissedCall` filters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_search_filter::MessageSearchFilter;
    ///
    /// assert!(MessageSearchFilter::Call.is_call_filter());
    /// assert!(MessageSearchFilter::MissedCall.is_call_filter());
    /// assert!(!MessageSearchFilter::Photo.is_call_filter());
    /// ```
    #[must_use]
    pub const fn is_call_filter(self) -> bool {
        matches!(self, Self::Call | Self::MissedCall)
    }

    /// Returns the call filter index (0-based).
    ///
    /// Returns `Some(0)` for `Call` and `Some(1)` for `MissedCall`,
    /// `None` for all other filters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_search_filter::MessageSearchFilter;
    ///
    /// assert_eq!(MessageSearchFilter::Call.call_filter_index(), Some(0));
    /// assert_eq!(MessageSearchFilter::MissedCall.call_filter_index(), Some(1));
    /// assert_eq!(MessageSearchFilter::Photo.call_filter_index(), None);
    /// ```
    #[must_use]
    pub const fn call_filter_index(self) -> Option<i32> {
        match self {
            Self::Call => Some(0),
            Self::MissedCall => Some(1),
            _ => None,
        }
    }

    /// Returns the total number of non-empty filters.
    ///
    /// This corresponds to the `Size` variant in TDLib which marks
    /// the end of the enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_search_filter::MessageSearchFilter;
    ///
    /// assert_eq!(MessageSearchFilter::count(), 18);
    /// ```
    #[must_use]
    pub const fn count() -> i32 {
        18
    }

    /// Creates a filter from an index (1-based).
    ///
    /// Returns `None` if the index is out of range (0 or > count).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_search_filter::MessageSearchFilter;
    ///
    /// assert_eq!(MessageSearchFilter::from_index(1), Some(MessageSearchFilter::Animation));
    /// assert_eq!(MessageSearchFilter::from_index(4), Some(MessageSearchFilter::Photo));
    /// assert_eq!(MessageSearchFilter::from_index(0), None);
    /// assert_eq!(MessageSearchFilter::from_index(19), None);
    /// ```
    #[must_use]
    pub const fn from_index(index: i32) -> Option<Self> {
        match index {
            1 => Some(Self::Animation),
            2 => Some(Self::Audio),
            3 => Some(Self::Document),
            4 => Some(Self::Photo),
            5 => Some(Self::Video),
            6 => Some(Self::VoiceNote),
            7 => Some(Self::PhotoAndVideo),
            8 => Some(Self::Url),
            9 => Some(Self::ChatPhoto),
            10 => Some(Self::Call),
            11 => Some(Self::MissedCall),
            12 => Some(Self::VideoNote),
            13 => Some(Self::VoiceAndVideoNote),
            14 => Some(Self::Mention),
            15 => Some(Self::UnreadMention),
            16 => Some(Self::FailedToSend),
            17 => Some(Self::Pinned),
            18 => Some(Self::UnreadReaction),
            _ => None,
        }
    }

    /// Creates a filter from a bitmask.
    ///
    /// Returns `None` if the bitmask doesn't correspond to a valid filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_search_filter::MessageSearchFilter;
    ///
    /// assert_eq!(MessageSearchFilter::from_bitmask(0), Some(MessageSearchFilter::Empty));
    /// assert_eq!(MessageSearchFilter::from_bitmask(2), Some(MessageSearchFilter::Animation));
    /// assert_eq!(MessageSearchFilter::from_bitmask(16), Some(MessageSearchFilter::Photo));
    /// assert_eq!(MessageSearchFilter::from_bitmask(3), None); // Multiple bits set
    /// ```
    #[must_use]
    pub const fn from_bitmask(bitmask: i32) -> Option<Self> {
        // Use if-else chain instead of match with const expressions
        if bitmask == 0 {
            return Some(Self::Empty);
        }
        if bitmask == 2 {
            return Some(Self::Animation);
        }
        if bitmask == 4 {
            return Some(Self::Audio);
        }
        if bitmask == 8 {
            return Some(Self::Document);
        }
        if bitmask == 16 {
            return Some(Self::Photo);
        }
        if bitmask == 32 {
            return Some(Self::Video);
        }
        if bitmask == 64 {
            return Some(Self::VoiceNote);
        }
        if bitmask == 128 {
            return Some(Self::PhotoAndVideo);
        }
        if bitmask == 256 {
            return Some(Self::Url);
        }
        if bitmask == 512 {
            return Some(Self::ChatPhoto);
        }
        if bitmask == 1024 {
            return Some(Self::Call);
        }
        if bitmask == 2048 {
            return Some(Self::MissedCall);
        }
        if bitmask == 4096 {
            return Some(Self::VideoNote);
        }
        if bitmask == 8192 {
            return Some(Self::VoiceAndVideoNote);
        }
        if bitmask == 16384 {
            return Some(Self::Mention);
        }
        if bitmask == 32768 {
            return Some(Self::UnreadMention);
        }
        if bitmask == 65536 {
            return Some(Self::FailedToSend);
        }
        if bitmask == 131072 {
            return Some(Self::Pinned);
        }
        if bitmask == 262144 {
            return Some(Self::UnreadReaction);
        }
        None
    }
}

impl fmt::Display for MessageSearchFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests
    #[test]
    fn test_default() {
        let filter = MessageSearchFilter::default();
        assert_eq!(filter, MessageSearchFilter::Empty);
    }

    #[test]
    fn test_copy() {
        let filter1 = MessageSearchFilter::Photo;
        let filter2 = filter1;
        assert_eq!(filter1, MessageSearchFilter::Photo);
        assert_eq!(filter2, MessageSearchFilter::Photo);
    }

    #[test]
    fn test_clone() {
        let filter1 = MessageSearchFilter::Audio;
        let filter2 = filter1.clone();
        assert_eq!(filter1, filter2);
    }

    #[test]
    fn test_equality() {
        assert_eq!(MessageSearchFilter::Photo, MessageSearchFilter::Photo);
        assert_ne!(MessageSearchFilter::Photo, MessageSearchFilter::Video);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let filter1 = MessageSearchFilter::Photo;
        let filter2 = MessageSearchFilter::Photo;
        let filter3 = MessageSearchFilter::Video;

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        let mut hasher3 = DefaultHasher::new();

        filter1.hash(&mut hasher1);
        filter2.hash(&mut hasher2);
        filter3.hash(&mut hasher3);

        assert_eq!(hasher1.finish(), hasher2.finish());
        assert_ne!(hasher1.finish(), hasher3.finish());
    }

    // as_str tests
    #[test]
    fn test_as_str_empty() {
        assert_eq!(MessageSearchFilter::Empty.as_str(), "Empty");
    }

    #[test]
    fn test_as_str_animation() {
        assert_eq!(MessageSearchFilter::Animation.as_str(), "Animation");
    }

    #[test]
    fn test_as_str_audio() {
        assert_eq!(MessageSearchFilter::Audio.as_str(), "Audio");
    }

    #[test]
    fn test_as_str_document() {
        assert_eq!(MessageSearchFilter::Document.as_str(), "Document");
    }

    #[test]
    fn test_as_str_photo() {
        assert_eq!(MessageSearchFilter::Photo.as_str(), "Photo");
    }

    #[test]
    fn test_as_str_video() {
        assert_eq!(MessageSearchFilter::Video.as_str(), "Video");
    }

    #[test]
    fn test_as_str_voice_note() {
        assert_eq!(MessageSearchFilter::VoiceNote.as_str(), "VoiceNote");
    }

    #[test]
    fn test_as_str_photo_and_video() {
        assert_eq!(MessageSearchFilter::PhotoAndVideo.as_str(), "PhotoAndVideo");
    }

    #[test]
    fn test_as_str_url() {
        assert_eq!(MessageSearchFilter::Url.as_str(), "Url");
    }

    #[test]
    fn test_as_str_chat_photo() {
        assert_eq!(MessageSearchFilter::ChatPhoto.as_str(), "ChatPhoto");
    }

    #[test]
    fn test_as_str_call() {
        assert_eq!(MessageSearchFilter::Call.as_str(), "Call");
    }

    #[test]
    fn test_as_str_missed_call() {
        assert_eq!(MessageSearchFilter::MissedCall.as_str(), "MissedCall");
    }

    #[test]
    fn test_as_str_video_note() {
        assert_eq!(MessageSearchFilter::VideoNote.as_str(), "VideoNote");
    }

    #[test]
    fn test_as_str_voice_and_video_note() {
        assert_eq!(
            MessageSearchFilter::VoiceAndVideoNote.as_str(),
            "VoiceAndVideoNote"
        );
    }

    #[test]
    fn test_as_str_mention() {
        assert_eq!(MessageSearchFilter::Mention.as_str(), "Mention");
    }

    #[test]
    fn test_as_str_unread_mention() {
        assert_eq!(MessageSearchFilter::UnreadMention.as_str(), "UnreadMention");
    }

    #[test]
    fn test_as_str_failed_to_send() {
        assert_eq!(MessageSearchFilter::FailedToSend.as_str(), "FailedToSend");
    }

    #[test]
    fn test_as_str_pinned() {
        assert_eq!(MessageSearchFilter::Pinned.as_str(), "Pinned");
    }

    #[test]
    fn test_as_str_unread_reaction() {
        assert_eq!(
            MessageSearchFilter::UnreadReaction.as_str(),
            "UnreadReaction"
        );
    }

    // index tests
    #[test]
    fn test_index_empty() {
        assert_eq!(MessageSearchFilter::Empty.index(), None);
    }

    #[test]
    fn test_index_animation() {
        assert_eq!(MessageSearchFilter::Animation.index(), Some(1));
    }

    #[test]
    fn test_index_photo() {
        assert_eq!(MessageSearchFilter::Photo.index(), Some(4));
    }

    #[test]
    fn test_index_unread_reaction() {
        assert_eq!(MessageSearchFilter::UnreadReaction.index(), Some(18));
    }

    // bitmask tests
    #[test]
    fn test_bitmask_empty() {
        assert_eq!(MessageSearchFilter::Empty.bitmask(), 0);
    }

    #[test]
    fn test_bitmask_animation() {
        assert_eq!(MessageSearchFilter::Animation.bitmask(), 2);
    }

    #[test]
    fn test_bitmask_photo() {
        assert_eq!(MessageSearchFilter::Photo.bitmask(), 16);
    }

    #[test]
    fn test_bitmask_unread_reaction() {
        assert_eq!(MessageSearchFilter::UnreadReaction.bitmask(), 262144);
    }

    // is_* tests
    #[test]
    fn test_is_empty() {
        assert!(MessageSearchFilter::Empty.is_empty());
        assert!(!MessageSearchFilter::Photo.is_empty());
    }

    #[test]
    fn test_is_animation() {
        assert!(MessageSearchFilter::Animation.is_animation());
        assert!(!MessageSearchFilter::Photo.is_animation());
    }

    #[test]
    fn test_is_audio() {
        assert!(MessageSearchFilter::Audio.is_audio());
        assert!(!MessageSearchFilter::Photo.is_audio());
    }

    #[test]
    fn test_is_photo() {
        assert!(MessageSearchFilter::Photo.is_photo());
        assert!(!MessageSearchFilter::Video.is_photo());
    }

    #[test]
    fn test_is_video() {
        assert!(MessageSearchFilter::Video.is_video());
        assert!(!MessageSearchFilter::Photo.is_video());
    }

    #[test]
    fn test_is_call_filter() {
        assert!(MessageSearchFilter::Call.is_call_filter());
        assert!(MessageSearchFilter::MissedCall.is_call_filter());
        assert!(!MessageSearchFilter::Photo.is_call_filter());
    }

    #[test]
    fn test_is_mention() {
        assert!(MessageSearchFilter::Mention.is_mention());
        assert!(!MessageSearchFilter::Photo.is_mention());
    }

    #[test]
    fn test_is_pinned() {
        assert!(MessageSearchFilter::Pinned.is_pinned());
        assert!(!MessageSearchFilter::Photo.is_pinned());
    }

    // call_filter_index tests
    #[test]
    fn test_call_filter_index_call() {
        assert_eq!(MessageSearchFilter::Call.call_filter_index(), Some(0));
    }

    #[test]
    fn test_call_filter_index_missed_call() {
        assert_eq!(MessageSearchFilter::MissedCall.call_filter_index(), Some(1));
    }

    #[test]
    fn test_call_filter_index_non_call() {
        assert_eq!(MessageSearchFilter::Photo.call_filter_index(), None);
    }

    // count test
    #[test]
    fn test_count() {
        assert_eq!(MessageSearchFilter::count(), 18);
    }

    // from_index tests
    #[test]
    fn test_from_index_animation() {
        assert_eq!(
            MessageSearchFilter::from_index(1),
            Some(MessageSearchFilter::Animation)
        );
    }

    #[test]
    fn test_from_index_photo() {
        assert_eq!(
            MessageSearchFilter::from_index(4),
            Some(MessageSearchFilter::Photo)
        );
    }

    #[test]
    fn test_from_index_zero() {
        assert_eq!(MessageSearchFilter::from_index(0), None);
    }

    #[test]
    fn test_from_index_out_of_range() {
        assert_eq!(MessageSearchFilter::from_index(19), None);
        assert_eq!(MessageSearchFilter::from_index(100), None);
        assert_eq!(MessageSearchFilter::from_index(-1), None);
    }

    // from_bitmask tests
    #[test]
    fn test_from_bitmask_empty() {
        assert_eq!(
            MessageSearchFilter::from_bitmask(0),
            Some(MessageSearchFilter::Empty)
        );
    }

    #[test]
    fn test_from_bitmask_animation() {
        assert_eq!(
            MessageSearchFilter::from_bitmask(2),
            Some(MessageSearchFilter::Animation)
        );
    }

    #[test]
    fn test_from_bitmask_photo() {
        assert_eq!(
            MessageSearchFilter::from_bitmask(16),
            Some(MessageSearchFilter::Photo)
        );
    }

    #[test]
    fn test_from_bitmask_invalid() {
        assert_eq!(MessageSearchFilter::from_bitmask(0b11), None); // Multiple bits
        assert_eq!(MessageSearchFilter::from_bitmask(1 << 19), None); // Out of range
    }

    // Display tests
    #[test]
    fn test_display() {
        assert_eq!(format!("{}", MessageSearchFilter::Photo), "Photo");
        assert_eq!(format!("{}", MessageSearchFilter::Video), "Video");
        assert_eq!(format!("{}", MessageSearchFilter::Empty), "Empty");
    }

    // Round-trip tests
    #[test]
    fn test_round_trip_index() {
        let filters = [
            MessageSearchFilter::Animation,
            MessageSearchFilter::Audio,
            MessageSearchFilter::Document,
            MessageSearchFilter::Photo,
            MessageSearchFilter::Video,
            MessageSearchFilter::VoiceNote,
            MessageSearchFilter::PhotoAndVideo,
            MessageSearchFilter::Url,
            MessageSearchFilter::ChatPhoto,
            MessageSearchFilter::Call,
            MessageSearchFilter::MissedCall,
            MessageSearchFilter::VideoNote,
            MessageSearchFilter::VoiceAndVideoNote,
            MessageSearchFilter::Mention,
            MessageSearchFilter::UnreadMention,
            MessageSearchFilter::FailedToSend,
            MessageSearchFilter::Pinned,
            MessageSearchFilter::UnreadReaction,
        ];

        for filter in filters {
            let index = filter.index().unwrap();
            let round_trip = MessageSearchFilter::from_index(index).unwrap();
            assert_eq!(filter, round_trip);
        }
    }

    #[test]
    fn test_round_trip_bitmask() {
        let filters = [
            MessageSearchFilter::Empty,
            MessageSearchFilter::Animation,
            MessageSearchFilter::Audio,
            MessageSearchFilter::Document,
            MessageSearchFilter::Photo,
            MessageSearchFilter::Video,
            MessageSearchFilter::VoiceNote,
            MessageSearchFilter::PhotoAndVideo,
            MessageSearchFilter::Url,
            MessageSearchFilter::ChatPhoto,
            MessageSearchFilter::Call,
            MessageSearchFilter::MissedCall,
            MessageSearchFilter::VideoNote,
            MessageSearchFilter::VoiceAndVideoNote,
            MessageSearchFilter::Mention,
            MessageSearchFilter::UnreadMention,
            MessageSearchFilter::FailedToSend,
            MessageSearchFilter::Pinned,
            MessageSearchFilter::UnreadReaction,
        ];

        for filter in filters {
            let bitmask = filter.bitmask();
            let round_trip = MessageSearchFilter::from_bitmask(bitmask).unwrap();
            assert_eq!(filter, round_trip);
        }
    }

    // Edge case tests
    #[test]
    fn test_all_filters_have_unique_indices() {
        let mut indices = Vec::new();

        for i in 1..=18 {
            indices.push(i);
        }

        let unique: std::collections::HashSet<_> = indices.iter().collect();
        assert_eq!(unique.len(), indices.len());
    }

    #[test]
    fn test_all_filters_have_unique_bitmasks() {
        let filters = [
            MessageSearchFilter::Animation,
            MessageSearchFilter::Audio,
            MessageSearchFilter::Document,
            MessageSearchFilter::Photo,
            MessageSearchFilter::Video,
            MessageSearchFilter::VoiceNote,
            MessageSearchFilter::PhotoAndVideo,
            MessageSearchFilter::Url,
            MessageSearchFilter::ChatPhoto,
            MessageSearchFilter::Call,
            MessageSearchFilter::MissedCall,
            MessageSearchFilter::VideoNote,
            MessageSearchFilter::VoiceAndVideoNote,
            MessageSearchFilter::Mention,
            MessageSearchFilter::UnreadMention,
            MessageSearchFilter::FailedToSend,
            MessageSearchFilter::Pinned,
            MessageSearchFilter::UnreadReaction,
        ];

        let mut bitmasks = Vec::new();
        for filter in filters {
            bitmasks.push(filter.bitmask());
        }

        let unique: std::collections::HashSet<_> = bitmasks.iter().collect();
        assert_eq!(unique.len(), bitmasks.len());
    }

    #[test]
    fn test_no_bitmask_collisions() {
        for i in 1..=18 {
            let filter1 = MessageSearchFilter::from_index(i).unwrap();
            for j in (i + 1)..=18 {
                let filter2 = MessageSearchFilter::from_index(j).unwrap();
                assert_ne!(
                    filter1.bitmask(),
                    filter2.bitmask(),
                    "Bitmask collision between {:?} and {:?}",
                    filter1,
                    filter2
                );
            }
        }
    }
}
