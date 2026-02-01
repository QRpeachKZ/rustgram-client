// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! TL (Type Language) stub types for MessageQueryManager.
//!
//! These are simplified stub implementations for TL types that will be
//! replaced with full TL layer implementations when available.

use rustgram_formatted_text::FormattedText;
use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Input file for upload operations.
///
/// Stub implementation for TDLib's InputFile.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::tl::InputFile;
///
/// let file = InputFile::local("/path/to/file.jpg".to_string());
/// assert!(file.is_local());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InputFile {
    /// File path or ID.
    id: String,
    /// Whether this is a local file.
    is_local: bool,
}

impl InputFile {
    /// Creates a local input file.
    ///
    /// # Arguments
    ///
    /// * `path` - Local file path
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::tl::InputFile;
    ///
    /// let file = InputFile::local("/path/to/file.jpg".to_string());
    /// assert!(file.is_local());
    /// ```
    #[must_use]
    pub fn local(path: String) -> Self {
        Self {
            id: path,
            is_local: true,
        }
    }

    /// Creates a remote input file.
    ///
    /// # Arguments
    ///
    /// * `id` - Remote file ID
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::tl::InputFile;
    ///
    /// let file = InputFile::remote("file_id_123".to_string());
    /// assert!(!file.is_local());
    /// ```
    #[must_use]
    pub fn remote(id: String) -> Self {
        Self {
            id,
            is_local: false,
        }
    }

    /// Returns the file ID/path.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Checks if this is a local file.
    #[must_use]
    pub const fn is_local(&self) -> bool {
        self.is_local
    }
}

impl fmt::Display for InputFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_local {
            write!(f, "local file: {}", self.id)
        } else {
            write!(f, "remote file: {}", self.id)
        }
    }
}

/// Message media attachment.
///
/// Stub implementation for TDLib's MessageMedia.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::tl::MessageMedia;
///
/// let media = MessageMedia::photo();
/// assert!(matches!(media, MessageMedia::Photo { .. }));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageMedia {
    /// Photo media.
    Photo {
        /// Photo ID
        id: String,
    },
    /// Video media.
    Video {
        /// Video ID
        id: String,
    },
    /// Document media.
    Document {
        /// Document ID
        id: String,
    },
    /// Unsupported media type.
    Unsupported,
}

impl MessageMedia {
    /// Creates a photo media.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::tl::MessageMedia;
    ///
    /// let media = MessageMedia::photo();
    /// ```
    #[must_use]
    pub fn photo() -> Self {
        Self::Photo { id: String::new() }
    }

    /// Creates a video media.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::tl::MessageMedia;
    ///
    /// let media = MessageMedia::video();
    /// ```
    #[must_use]
    pub fn video() -> Self {
        Self::Video { id: String::new() }
    }

    /// Checks if this media is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Unsupported)
    }
}

impl fmt::Display for MessageMedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Photo { .. } => write!(f, "photo"),
            Self::Video { .. } => write!(f, "video"),
            Self::Document { .. } => write!(f, "document"),
            Self::Unsupported => write!(f, "unsupported media"),
        }
    }
}

/// Search posts flood information.
///
/// Stub implementation for TDLib's searchPostsFlood.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::tl::SearchPostsFlood;
///
/// let flood = SearchPostsFlood::new(10, 100);
/// assert_eq!(flood.wait_seconds(), 10);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchPostsFlood {
    /// Seconds to wait before next search.
    wait_seconds: i32,
    /// Total flood limit.
    total_limit: i32,
}

impl SearchPostsFlood {
    /// Creates a new search posts flood info.
    ///
    /// # Arguments
    ///
    /// * `wait_seconds` - Seconds to wait
    /// * `total_limit` - Total limit
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::tl::SearchPostsFlood;
    ///
    /// let flood = SearchPostsFlood::new(10, 100);
    /// ```
    #[must_use]
    pub const fn new(wait_seconds: i32, total_limit: i32) -> Self {
        Self {
            wait_seconds,
            total_limit,
        }
    }

    /// Returns the wait time in seconds.
    #[must_use]
    pub const fn wait_seconds(&self) -> i32 {
        self.wait_seconds
    }

    /// Returns the total limit.
    #[must_use]
    pub const fn total_limit(&self) -> i32 {
        self.total_limit
    }
}

impl fmt::Display for SearchPostsFlood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "wait {} seconds (limit: {})",
            self.wait_seconds, self.total_limit
        )
    }
}

/// Discussion message information.
///
/// Stub implementation for TDLib's messages.discussionMessage.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::tl::DiscussionMessage;
/// use rustgram_types::{ChatId, DialogId, MessageId};
///
/// let msg = DiscussionMessage::new(DialogId::from_chat(ChatId::new(123).unwrap()), MessageId::from_server_id(456));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscussionMessage {
    /// Discussion dialog ID.
    discussion_dialog_id: DialogId,
    /// Discussion message ID.
    discussion_message_id: MessageId,
}

impl DiscussionMessage {
    /// Creates a new discussion message.
    ///
    /// # Arguments
    ///
    /// * `discussion_dialog_id` - Discussion dialog ID
    /// * `discussion_message_id` - Discussion message ID
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::tl::DiscussionMessage;
    /// use rustgram_types::{ChatId, DialogId, MessageId};
    ///
    /// let msg = DiscussionMessage::new(DialogId::from_chat(ChatId::new(123).unwrap()), MessageId::from_server_id(456));
    /// ```
    #[must_use]
    pub fn new(discussion_dialog_id: DialogId, discussion_message_id: MessageId) -> Self {
        Self {
            discussion_dialog_id,
            discussion_message_id,
        }
    }

    /// Returns the discussion dialog ID.
    #[must_use]
    pub const fn discussion_dialog_id(&self) -> DialogId {
        self.discussion_dialog_id
    }

    /// Returns the discussion message ID.
    #[must_use]
    pub const fn discussion_message_id(&self) -> MessageId {
        self.discussion_message_id
    }
}

impl fmt::Display for DiscussionMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DiscussionMessage(dialog={}, msg={})",
            self.discussion_dialog_id, self.discussion_message_id
        )
    }
}

/// Fact check information.
///
/// Stub implementation for TDLib's factCheck.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::tl::FactCheck;
/// use rustgram_formatted_text::FormattedText;
///
/// let text = FormattedText::new("Fact check text");
/// let fact_check = FactCheck::new(text, 123);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactCheck {
    /// Fact check text.
    text: FormattedText,
    /// Country code.
    country_code: String,
    /// Verification time.
    verify_time: i32,
}

impl FactCheck {
    /// Creates a new fact check.
    ///
    /// # Arguments
    ///
    /// * `text` - Fact check text
    /// * `verify_time` - Verification time
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::tl::FactCheck;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Fact check");
    /// let fact_check = FactCheck::new(text, 1234567890);
    /// ```
    #[must_use]
    pub fn new(text: FormattedText, verify_time: i32) -> Self {
        Self {
            text,
            country_code: String::new(),
            verify_time,
        }
    }

    /// Returns the fact check text.
    #[must_use]
    pub const fn text(&self) -> &FormattedText {
        &self.text
    }

    /// Returns the country code.
    #[must_use]
    pub fn country_code(&self) -> &str {
        &self.country_code
    }

    /// Returns the verification time.
    #[must_use]
    pub const fn verify_time(&self) -> i32 {
        self.verify_time
    }

    /// Sets the country code.
    pub fn with_country_code(mut self, country_code: String) -> Self {
        self.country_code = country_code;
        self
    }
}

impl fmt::Display for FactCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FactCheck(verify_time={}, country={})",
            self.verify_time, self.country_code
        )
    }
}

/// Found messages result.
///
/// Stub implementation for search results.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::tl::FoundMessages;
///
/// let found = FoundMessages::with_total_count(10);
/// assert_eq!(found.total_count(), 10);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FoundMessages {
    /// Total number of matching messages.
    total_count: i32,
    /// Found message IDs.
    messages: Vec<MessageId>,
}

impl FoundMessages {
    /// Creates a new found messages result.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::tl::FoundMessages;
    ///
    /// let found = FoundMessages::new();
    /// assert_eq!(found.total_count(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            total_count: 0,
            messages: Vec::new(),
        }
    }

    /// Creates a found messages result with a total count.
    ///
    /// # Arguments
    ///
    /// * `total_count` - Total number of matching messages
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_query_manager::tl::FoundMessages;
    ///
    /// let found = FoundMessages::with_total_count(42);
    /// assert_eq!(found.total_count(), 42);
    /// ```
    #[must_use]
    pub fn with_total_count(total_count: i32) -> Self {
        Self {
            total_count,
            messages: Vec::new(),
        }
    }

    /// Returns the total count.
    #[must_use]
    pub const fn total_count(&self) -> i32 {
        self.total_count
    }

    /// Returns the found messages.
    #[must_use]
    pub fn messages(&self) -> &[MessageId] {
        &self.messages
    }

    /// Adds a message to the results.
    pub fn add_message(&mut self, message_id: MessageId) {
        self.messages.push(message_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::ChatId;

    // InputFile tests
    #[test]
    fn test_input_file_local() {
        let file = InputFile::local("/path/to/file.jpg".to_string());
        assert!(file.is_local());
        assert_eq!(file.id(), "/path/to/file.jpg");
    }

    #[test]
    fn test_input_file_remote() {
        let file = InputFile::remote("file_id_123".to_string());
        assert!(!file.is_local());
        assert_eq!(file.id(), "file_id_123");
    }

    #[test]
    fn test_input_file_display_local() {
        let file = InputFile::local("test.jpg".to_string());
        let display = format!("{file}");
        assert!(display.contains("local file"));
    }

    #[test]
    fn test_input_file_clone() {
        let file = InputFile::local("test.jpg".to_string());
        let file2 = file.clone();
        assert_eq!(file, file2);
    }

    // MessageMedia tests
    #[test]
    fn test_message_media_photo() {
        let media = MessageMedia::photo();
        assert!(matches!(media, MessageMedia::Photo { .. }));
        assert!(!media.is_empty());
    }

    #[test]
    fn test_message_media_video() {
        let media = MessageMedia::video();
        assert!(matches!(media, MessageMedia::Video { .. }));
    }

    #[test]
    fn test_message_media_unsupported() {
        let media = MessageMedia::Unsupported;
        assert!(media.is_empty());
    }

    #[test]
    fn test_message_media_display() {
        let media = MessageMedia::photo();
        let display = format!("{media}");
        assert!(display.contains("photo"));
    }

    // SearchPostsFlood tests
    #[test]
    fn test_search_posts_flood_new() {
        let flood = SearchPostsFlood::new(10, 100);
        assert_eq!(flood.wait_seconds(), 10);
        assert_eq!(flood.total_limit(), 100);
    }

    #[test]
    fn test_search_posts_flood_display() {
        let flood = SearchPostsFlood::new(30, 500);
        let display = format!("{flood}");
        assert!(display.contains("30"));
        assert!(display.contains("500"));
    }

    #[test]
    fn test_search_posts_flood_clone() {
        let flood1 = SearchPostsFlood::new(10, 100);
        let flood2 = flood1;
        assert_eq!(flood1, flood2);
    }

    // DiscussionMessage tests
    #[test]
    fn test_discussion_message_new() {
        let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
        let message_id = MessageId::from_server_id(456);
        let msg = DiscussionMessage::new(dialog_id, message_id);

        assert_eq!(msg.discussion_dialog_id(), dialog_id);
        assert_eq!(msg.discussion_message_id(), message_id);
    }

    #[test]
    fn test_discussion_message_display() {
        let msg = DiscussionMessage::new(
            DialogId::from_chat(ChatId::new(123).unwrap()),
            MessageId::from_server_id(456),
        );
        let display = format!("{msg}");
        assert!(display.contains("DiscussionMessage"));
    }

    #[test]
    fn test_discussion_message_clone() {
        let msg1 = DiscussionMessage::new(
            DialogId::from_chat(ChatId::new(123).unwrap()),
            MessageId::from_server_id(456),
        );
        let msg2 = msg1.clone();
        assert_eq!(msg1, msg2);
    }

    // FactCheck tests
    #[test]
    fn test_fact_check_new() {
        let text = FormattedText::new("Fact check");
        let fact_check = FactCheck::new(text, 1234567890);

        assert_eq!(fact_check.verify_time(), 1234567890);
        assert_eq!(fact_check.country_code(), "");
    }

    #[test]
    fn test_fact_check_with_country() {
        let text = FormattedText::new("Fact check");
        let fact_check = FactCheck::new(text, 123).with_country_code("US".to_string());

        assert_eq!(fact_check.country_code(), "US");
    }

    #[test]
    fn test_fact_check_display() {
        let text = FormattedText::new("Check");
        let fact_check = FactCheck::new(text, 1234567890);
        let display = format!("{fact_check}");
        assert!(display.contains("1234567890"));
    }

    #[test]
    fn test_fact_check_clone() {
        let text = FormattedText::new("Check");
        let fact_check1 = FactCheck::new(text, 123);
        let fact_check2 = fact_check1.clone();
        assert_eq!(fact_check1, fact_check2);
    }

    // FoundMessages tests
    #[test]
    fn test_found_messages_new() {
        let found = FoundMessages::new();
        assert_eq!(found.total_count(), 0);
        assert!(found.messages().is_empty());
    }

    #[test]
    fn test_found_messages_default() {
        let found = FoundMessages::default();
        assert_eq!(found.total_count(), 0);
    }

    #[test]
    fn test_found_messages_with_total_count() {
        let found = FoundMessages::with_total_count(42);
        assert_eq!(found.total_count(), 42);
    }

    #[test]
    fn test_found_messages_add_message() {
        let mut found = FoundMessages::new();
        found.add_message(MessageId::from_server_id(123));

        assert_eq!(found.messages().len(), 1);
    }

    #[test]
    fn test_found_messages_clone() {
        let mut found1 = FoundMessages::new();
        found1.add_message(MessageId::from_server_id(123));
        let found2 = found1.clone();
        assert_eq!(found1, found2);
    }
}
