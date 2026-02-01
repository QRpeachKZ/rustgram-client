// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Input business chat link types for Telegram MTProto client.
//!
//! This module implements TDLib's InputBusinessChatLink from `td/telegram/InputBusinessChatLink.h`.
//!
//! # Overview
//!
//! Business chat links allow Telegram Business accounts to create pre-filled chat links
//! with custom text and titles. When users click these links, they start a conversation
//! with the business account with the specified message template.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt;

/// A simple formatted text type for business chat links.
///
/// This is a simplified representation of TDLib's FormattedText type.
/// In a full implementation, this would include text entities for formatting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormattedText {
    /// The plain text content.
    text: String,
    /// Optional caption for the text.
    caption: Option<String>,
}

impl FormattedText {
    /// Creates a new FormattedText with the given content.
    ///
    /// # Arguments
    ///
    /// * `text` - The plain text content
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_input_business_chat_link::FormattedText;
    ///
    /// let text = FormattedText::new("Hello! How can I help you?");
    /// assert_eq!(text.as_str(), "Hello! How can I help you?");
    /// ```
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            caption: None,
        }
    }

    /// Creates a new FormattedText with text and caption.
    ///
    /// # Arguments
    ///
    /// * `text` - The plain text content
    /// * `caption` - Optional caption for the text
    pub fn with_caption<S: Into<String>>(text: S, caption: S) -> Self {
        Self {
            text: text.into(),
            caption: Some(caption.into()),
        }
    }

    /// Returns the text content as a string slice.
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Returns the optional caption.
    pub fn caption(&self) -> Option<&str> {
        self.caption.as_deref()
    }

    /// Checks if the text is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the length of the text.
    pub fn len(&self) -> usize {
        self.text.len()
    }
}

impl fmt::Display for FormattedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl<S: Into<String>> From<S> for FormattedText {
    fn from(text: S) -> Self {
        Self::new(text)
    }
}

/// Input business chat link for Telegram Business accounts.
///
/// This type represents a business chat link that can be shared with users.
/// When a user clicks the link, they start a conversation with the business
/// account with the pre-filled message text.
///
/// # Example
///
/// ```
/// use rustgram_input_business_chat_link::{InputBusinessChatLink, FormattedText};
///
/// let text = FormattedText::new("Hello! I'd like to inquire about your services.");
/// let link = InputBusinessChatLink::new(text, "Business Inquiry".to_string());
///
/// assert_eq!(link.title(), "Business Inquiry");
/// assert_eq!(link.text().as_str(), "Hello! I'd like to inquire about your services.");
///
/// // Using builder pattern
/// let link2 = InputBusinessChatLink::new(
///     FormattedText::new("Question about pricing"),
///     "Pricing".to_string()
/// )
/// .with_title("Pricing Inquiry".to_string());
/// assert_eq!(link2.title(), "Pricing Inquiry");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputBusinessChatLink {
    /// The formatted text message that will be pre-filled in the chat.
    text: FormattedText,
    /// The title/label for this business chat link.
    title: String,
}

impl InputBusinessChatLink {
    /// Creates a new InputBusinessChatLink.
    ///
    /// # Arguments
    ///
    /// * `text` - The formatted text message to pre-fill
    /// * `title` - The title for the business chat link
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_input_business_chat_link::{InputBusinessChatLink, FormattedText};
    ///
    /// let text = FormattedText::new("I need help with my order");
    /// let link = InputBusinessChatLink::new(text, "Customer Support".to_string());
    /// ```
    pub fn new(text: FormattedText, title: String) -> Self {
        Self { text, title }
    }

    /// Returns a reference to the formatted text.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_input_business_chat_link::{InputBusinessChatLink, FormattedText};
    ///
    /// let text = FormattedText::new("Hello");
    /// let link = InputBusinessChatLink::new(text, "Title".to_string());
    /// assert_eq!(link.text().as_str(), "Hello");
    /// ```
    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    /// Returns the title of the business chat link.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_input_business_chat_link::{InputBusinessChatLink, FormattedText};
    ///
    /// let link = InputBusinessChatLink::new(
    ///     FormattedText::new("text"),
    ///     "Support".to_string()
    /// );
    /// assert_eq!(link.title(), "Support");
    /// ```
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Sets a new title for the business chat link.
    ///
    /// This method uses the builder pattern to allow chaining.
    ///
    /// # Arguments
    ///
    /// * `title` - The new title
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_input_business_chat_link::{InputBusinessChatLink, FormattedText};
    ///
    /// let link = InputBusinessChatLink::new(
    ///     FormattedText::new("text"),
    ///     "Old Title".to_string()
    /// )
    /// .with_title("New Title".to_string());
    /// assert_eq!(link.title(), "New Title");
    /// ```
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Checks if the business chat link is valid.
    ///
    /// A valid link must have non-empty text and a non-empty title.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_input_business_chat_link::{InputBusinessChatLink, FormattedText};
    ///
    /// let link = InputBusinessChatLink::new(
    ///     FormattedText::new("Hello"),
    ///     "Title".to_string()
    /// );
    /// assert!(link.is_valid());
    ///
    /// let invalid = InputBusinessChatLink::new(
    ///     FormattedText::new(""),
    ///     "".to_string()
    /// );
    /// assert!(!invalid.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.text.is_empty() && !self.title.is_empty()
    }
}

impl fmt::Display for InputBusinessChatLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessChatLink(title={}, text={})",
            self.title, self.text
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-input-business-chat-link";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-input-business-chat-link");
    }

    #[test]
    fn test_formatted_text_new() {
        let text = FormattedText::new("Hello, world!");
        assert_eq!(text.as_str(), "Hello, world!");
        assert_eq!(text.len(), 13);
        assert!(!text.is_empty());
        assert!(text.caption().is_none());
    }

    #[test]
    fn test_formatted_text_with_caption() {
        let text = FormattedText::with_caption("Hello", "Greeting");
        assert_eq!(text.as_str(), "Hello");
        assert_eq!(text.caption(), Some("Greeting"));
    }

    #[test]
    fn test_formatted_text_empty() {
        let text = FormattedText::new("");
        assert!(text.is_empty());
        assert_eq!(text.len(), 0);
    }

    #[test]
    fn test_formatted_text_from_string() {
        let text = FormattedText::from("Test message");
        assert_eq!(text.as_str(), "Test message");
    }

    #[test]
    fn test_formatted_text_display() {
        let text = FormattedText::new("Display test");
        assert_eq!(format!("{text}"), "Display test");
    }

    #[test]
    fn test_input_business_chat_link_new() {
        let text = FormattedText::new("I need help");
        let link = InputBusinessChatLink::new(text.clone(), "Support".to_string());

        assert_eq!(link.text().as_str(), "I need help");
        assert_eq!(link.title(), "Support");
    }

    #[test]
    fn test_input_business_chat_link_with_title() {
        let text = FormattedText::new("Question");
        let link =
            InputBusinessChatLink::new(text, "Old".to_string()).with_title("New".to_string());

        assert_eq!(link.title(), "New");
    }

    #[test]
    fn test_input_business_chat_link_is_valid() {
        let valid = InputBusinessChatLink::new(FormattedText::new("Hello"), "Title".to_string());
        assert!(valid.is_valid());

        let empty_text = InputBusinessChatLink::new(FormattedText::new(""), "Title".to_string());
        assert!(!empty_text.is_valid());

        let empty_title = InputBusinessChatLink::new(FormattedText::new("Hello"), "".to_string());
        assert!(!empty_title.is_valid());

        let both_empty = InputBusinessChatLink::new(FormattedText::new(""), "".to_string());
        assert!(!both_empty.is_valid());
    }

    #[test]
    fn test_input_business_chat_link_clone() {
        let text = FormattedText::new("Test");
        let link1 = InputBusinessChatLink::new(text.clone(), "Test".to_string());
        let link2 = link1.clone();

        assert_eq!(link1, link2);
    }

    #[test]
    fn test_input_business_chat_link_eq() {
        let text = FormattedText::new("Same");
        let link1 = InputBusinessChatLink::new(text.clone(), "Same".to_string());
        let link2 = InputBusinessChatLink::new(text, "Same".to_string());

        assert_eq!(link1, link2);
    }

    #[test]
    fn test_input_business_chat_link_display() {
        let link = InputBusinessChatLink::new(FormattedText::new("Test"), "Title".to_string());
        let display = format!("{link}");
        assert!(display.contains("Title"));
        assert!(display.contains("Test"));
    }

    #[test]
    fn test_formatted_text_clone() {
        let text1 = FormattedText::new("Clone test");
        let text2 = text1.clone();
        assert_eq!(text1, text2);
    }

    #[test]
    fn test_business_link_builder_chain() {
        let text = FormattedText::new("Initial text");
        let link = InputBusinessChatLink::new(text, "Initial".to_string())
            .with_title("Updated".to_string());

        assert_eq!(link.title(), "Updated");
        assert_eq!(link.text().as_str(), "Initial text");
    }

    #[test]
    fn test_formatted_text_with_caption_methods() {
        let text = FormattedText::with_caption("Main text", "Caption text");
        assert_eq!(text.as_str(), "Main text");
        assert_eq!(text.caption(), Some("Caption text"));
        assert_eq!(text.len(), 9);
    }

    #[test]
    fn test_multiple_links_with_same_text() {
        let text = FormattedText::new("Shared text");
        let link1 = InputBusinessChatLink::new(text.clone(), "Link 1".to_string());
        let link2 = InputBusinessChatLink::new(text, "Link 2".to_string());

        assert_eq!(link1.text().as_str(), link2.text().as_str());
        assert_ne!(link1.title(), link2.title());
    }

    #[test]
    fn test_input_business_chat_link_debug() {
        let link =
            InputBusinessChatLink::new(FormattedText::new("Debug test"), "Debug Title".to_string());
        let debug = format!("{link:?}");
        assert!(debug.contains("Debug Title"));
        assert!(debug.contains("Debug test"));
    }

    #[test]
    fn test_formatted_text_debug() {
        let text = FormattedText::with_caption("Text", "Caption");
        let debug = format!("{text:?}");
        assert!(debug.contains("Text"));
        assert!(debug.contains("Caption"));
    }

    #[test]
    fn test_empty_caption() {
        let text = FormattedText::with_caption("Text", "");
        assert_eq!(text.caption(), Some(""));
    }
}
