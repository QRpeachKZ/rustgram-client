// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Stub types for business connection manager.

use std::fmt;

/// Stub for input message content.
///
/// This is a simplified stub representing different types of message content.
/// TODO: Replace with full TL implementation when message content layer is ready.
///
/// # Examples
///
/// ```rust
/// use rustgram_business_connection_manager::InputMessageContent;
///
/// let text = InputMessageContent::text("Hello".to_string());
/// assert!(!text.is_empty());
///
/// let photo = InputMessageContent::photo("file123".to_string());
/// assert!(!photo.is_empty());
///
/// let empty = InputMessageContent::text("".to_string());
/// assert!(empty.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum InputMessageContent {
    /// Text message
    Text(String),
    /// Photo message
    Photo(String),
    /// Video message
    Video(String),
    /// Document message
    Document(String),
    /// Audio message
    Audio(String),
}

impl InputMessageContent {
    /// Creates a text message content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::InputMessageContent;
    ///
    /// let content = InputMessageContent::text("Hello, world!".to_string());
    /// ```
    pub fn text(text: String) -> Self {
        Self::Text(text)
    }

    /// Creates a photo message content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::InputMessageContent;
    ///
    /// let content = InputMessageContent::photo("file123".to_string());
    /// ```
    pub fn photo(file_id: String) -> Self {
        Self::Photo(file_id)
    }

    /// Creates a video message content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::InputMessageContent;
    ///
    /// let content = InputMessageContent::video("file123".to_string());
    /// ```
    pub fn video(file_id: String) -> Self {
        Self::Video(file_id)
    }

    /// Creates a document message content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::InputMessageContent;
    ///
    /// let content = InputMessageContent::document("file123".to_string());
    /// ```
    pub fn document(file_id: String) -> Self {
        Self::Document(file_id)
    }

    /// Creates an audio message content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::InputMessageContent;
    ///
    /// let content = InputMessageContent::audio("file123".to_string());
    /// ```
    pub fn audio(file_id: String) -> Self {
        Self::Audio(file_id)
    }

    /// Returns true if the content is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::InputMessageContent;
    ///
    /// assert!(InputMessageContent::text("".to_string()).is_empty());
    /// assert!(!InputMessageContent::text("Hi".to_string()).is_empty());
    /// assert!(!InputMessageContent::photo("file".to_string()).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Text(s) => s.is_empty(),
            Self::Photo(s) | Self::Video(s) | Self::Document(s) | Self::Audio(s) => s.is_empty(),
        }
    }

    /// Returns the text content if present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::InputMessageContent;
    ///
    /// let text = InputMessageContent::text("Hello".to_string());
    /// assert_eq!(text.as_text(), Some("Hello"));
    ///
    /// let photo = InputMessageContent::photo("file".to_string());
    /// assert_eq!(photo.as_text(), None);
    /// ```
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s),
            _ => None,
        }
    }
}

impl fmt::Display for InputMessageContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(s) => write!(f, "Text({})", s),
            Self::Photo(s) => write!(f, "Photo({})", s),
            Self::Video(s) => write!(f, "Video({})", s),
            Self::Document(s) => write!(f, "Document({})", s),
            Self::Audio(s) => write!(f, "Audio({})", s),
        }
    }
}

/// Stub for message effect ID.
///
/// This is a newtype wrapper around i64 for message effect identifiers.
/// TODO: Expand with full effect ID validation when TDLib schema is integrated.
///
/// # Examples
///
/// ```rust
/// use rustgram_business_connection_manager::MessageEffectId;
///
/// let effect_id = MessageEffectId::new(12345);
/// assert_eq!(effect_id.get(), 12345);
///
/// let default = MessageEffectId::default();
/// assert_eq!(default.get(), 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageEffectId(i64);

impl Default for MessageEffectId {
    fn default() -> Self {
        Self(0)
    }
}

impl MessageEffectId {
    /// Creates a new message effect ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::MessageEffectId;
    ///
    /// let effect_id = MessageEffectId::new(12345);
    /// assert_eq!(effect_id.get(), 12345);
    /// ```
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the effect ID value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::MessageEffectId;
    ///
    /// let effect_id = MessageEffectId::new(12345);
    /// assert_eq!(effect_id.get(), 12345);
    /// ```
    pub const fn get(&self) -> i64 {
        self.0
    }

    /// Returns true if this is the default (no effect) ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_business_connection_manager::MessageEffectId;
    ///
    /// assert!(MessageEffectId::default().is_none());
    /// assert!(!MessageEffectId::new(12345).is_none());
    /// ```
    pub const fn is_none(&self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for MessageEffectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EffectId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_message_content_text() {
        let content = InputMessageContent::text("Hello".to_string());
        assert!(!content.is_empty());
        assert_eq!(content.as_text(), Some("Hello"));
        assert_eq!(format!("{}", content), "Text(Hello)");
    }

    #[test]
    fn test_input_message_content_text_empty() {
        let content = InputMessageContent::text("".to_string());
        assert!(content.is_empty());
        assert_eq!(content.as_text(), Some(""));
    }

    #[test]
    fn test_input_message_content_photo() {
        let content = InputMessageContent::photo("file123".to_string());
        assert!(!content.is_empty());
        assert_eq!(content.as_text(), None);
        assert_eq!(format!("{}", content), "Photo(file123)");
    }

    #[test]
    fn test_input_message_content_video() {
        let content = InputMessageContent::video("video123".to_string());
        assert!(!content.is_empty());
        assert_eq!(format!("{}", content), "Video(video123)");
    }

    #[test]
    fn test_input_message_content_document() {
        let content = InputMessageContent::document("doc123".to_string());
        assert!(!content.is_empty());
        assert_eq!(format!("{}", content), "Document(doc123)");
    }

    #[test]
    fn test_input_message_content_audio() {
        let content = InputMessageContent::audio("audio123".to_string());
        assert!(!content.is_empty());
        assert_eq!(format!("{}", content), "Audio(audio123)");
    }

    #[test]
    fn test_input_message_content_clone() {
        let content1 = InputMessageContent::text("Hello".to_string());
        let content2 = content1.clone();
        assert_eq!(content1, content2);
    }

    #[test]
    fn test_input_message_content_equality() {
        let content1 = InputMessageContent::text("Hello".to_string());
        let content2 = InputMessageContent::text("Hello".to_string());
        assert_eq!(content1, content2);
    }

    #[test]
    fn test_input_message_content_inequality() {
        let content1 = InputMessageContent::text("Hello".to_string());
        let content2 = InputMessageContent::text("World".to_string());
        assert_ne!(content1, content2);
    }

    #[test]
    fn test_message_effect_id_new() {
        let effect_id = MessageEffectId::new(12345);
        assert_eq!(effect_id.get(), 12345);
        assert!(!effect_id.is_none());
    }

    #[test]
    fn test_message_effect_id_default() {
        let effect_id = MessageEffectId::default();
        assert_eq!(effect_id.get(), 0);
        assert!(effect_id.is_none());
    }

    #[test]
    fn test_message_effect_id_is_none() {
        assert!(MessageEffectId::default().is_none());
        assert!(MessageEffectId::new(0).is_none());
        assert!(!MessageEffectId::new(1).is_none());
        assert!(!MessageEffectId::new(-1).is_none());
    }

    #[test]
    fn test_message_effect_id_display() {
        let effect_id = MessageEffectId::new(12345);
        assert_eq!(format!("{}", effect_id), "EffectId(12345)");
    }

    #[test]
    fn test_message_effect_id_copy() {
        let effect_id1 = MessageEffectId::new(12345);
        let effect_id2 = effect_id1;
        assert_eq!(effect_id1.get(), effect_id2.get());
    }

    #[test]
    fn test_message_effect_id_equality() {
        let effect_id1 = MessageEffectId::new(12345);
        let effect_id2 = MessageEffectId::new(12345);
        assert_eq!(effect_id1, effect_id2);
    }

    #[test]
    fn test_message_effect_id_inequality() {
        let effect_id1 = MessageEffectId::new(12345);
        let effect_id2 = MessageEffectId::new(54321);
        assert_ne!(effect_id1, effect_id2);
    }
}
