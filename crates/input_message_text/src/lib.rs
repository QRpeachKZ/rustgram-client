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

//! # Input Message Text
//!
//! Text content for sending messages in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`InputMessageText`] type which represents
//! the text content of a message being sent. It includes:
//!
//! - Formatted text with entities (bold, italic, links, etc.)
//! - Web page preview settings
//! - Media display options
//!
//! ## TDLib Reference
//!
//! Corresponds to `td/telegram/InputMessageText.h` in TDLib.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_input_message_text::InputMessageText;
//! use rustgram_formatted_text::FormattedText;
//!
//! let text = FormattedText::new("Hello, world!");
//! let input = InputMessageText::with_text(text);
//! assert!(!input.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_formatted_text::FormattedText;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Input message text for sending messages.
///
/// Contains the formatted text content along with options for
/// web page preview and media display.
///
/// # Fields
///
/// - `text` - The formatted text content with entities
/// - `web_page_url` - Optional URL for web page preview
/// - `disable_web_page_preview` - Whether to disable link preview
/// - `force_small_media` - Force media preview to be small
/// - `force_large_media` - Force media preview to be large
/// - `show_above_text` - Show media preview above text
/// - `clear_draft` - Clear the draft after sending
///
/// # Example
///
/// ```rust
/// use rustgram_input_message_text::InputMessageText;
/// use rustgram_formatted_text::FormattedText;
///
/// let text = FormattedText::new("Check out https://example.com!");
/// let mut input = InputMessageText::with_text(text);
/// input.set_web_page_url("https://example.com".to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputMessageText {
    /// The formatted text content
    text: FormattedText,
    /// URL for web page preview
    web_page_url: String,
    /// Whether to disable link preview
    disable_web_page_preview: bool,
    /// Force media preview to be small
    force_small_media: bool,
    /// Force media preview to be large
    force_large_media: bool,
    /// Show media preview above text
    show_above_text: bool,
    /// Clear the draft after sending
    clear_draft: bool,
}

impl InputMessageText {
    /// Creates a new empty input message text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_message_text::InputMessageText;
    ///
    /// let input = InputMessageText::new();
    /// assert!(input.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            text: FormattedText::new(""),
            web_page_url: String::new(),
            disable_web_page_preview: false,
            force_small_media: false,
            force_large_media: false,
            show_above_text: false,
            clear_draft: false,
        }
    }

    /// Creates input message text with the given formatted text.
    ///
    /// # Arguments
    ///
    /// * `text` - The formatted text content
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_message_text::InputMessageText;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Hello, world!");
    /// let input = InputMessageText::with_text(text);
    /// assert_eq!(input.text().text(), "Hello, world!");
    /// ```
    #[must_use]
    pub fn with_text(text: FormattedText) -> Self {
        Self {
            text,
            web_page_url: String::new(),
            disable_web_page_preview: false,
            force_small_media: false,
            force_large_media: false,
            show_above_text: false,
            clear_draft: false,
        }
    }

    /// Returns `true` if both text and web_page_url are empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_message_text::InputMessageText;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let input = InputMessageText::new();
    /// assert!(input.is_empty());
    ///
    /// let input = InputMessageText::with_text(FormattedText::new("Hello"));
    /// assert!(!input.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty() && self.web_page_url.is_empty()
    }

    /// Returns the formatted text content.
    #[must_use]
    pub const fn text(&self) -> &FormattedText {
        &self.text
    }

    /// Sets the formatted text content.
    pub fn set_text(&mut self, text: FormattedText) {
        self.text = text;
    }

    /// Returns the web page URL.
    #[must_use]
    pub fn web_page_url(&self) -> &str {
        &self.web_page_url
    }

    /// Sets the web page URL.
    pub fn set_web_page_url(&mut self, url: String) {
        self.web_page_url = url;
    }

    /// Returns `true` if web page preview is disabled.
    #[must_use]
    pub const fn disable_web_page_preview(&self) -> bool {
        self.disable_web_page_preview
    }

    /// Sets whether to disable web page preview.
    pub fn set_disable_web_page_preview(&mut self, disable: bool) {
        self.disable_web_page_preview = disable;
    }

    /// Returns `true` if media preview should be forced small.
    #[must_use]
    pub const fn force_small_media(&self) -> bool {
        self.force_small_media
    }

    /// Sets whether to force small media preview.
    pub fn set_force_small_media(&mut self, force: bool) {
        self.force_small_media = force;
    }

    /// Returns `true` if media preview should be forced large.
    #[must_use]
    pub const fn force_large_media(&self) -> bool {
        self.force_large_media
    }

    /// Sets whether to force large media preview.
    pub fn set_force_large_media(&mut self, force: bool) {
        self.force_large_media = force;
    }

    /// Returns `true` if media preview should be shown above text.
    #[must_use]
    pub const fn show_above_text(&self) -> bool {
        self.show_above_text
    }

    /// Sets whether to show media preview above text.
    pub fn set_show_above_text(&mut self, show: bool) {
        self.show_above_text = show;
    }

    /// Returns `true` if draft should be cleared after sending.
    #[must_use]
    pub const fn clear_draft(&self) -> bool {
        self.clear_draft
    }

    /// Sets whether to clear draft after sending.
    pub fn set_clear_draft(&mut self, clear: bool) {
        self.clear_draft = clear;
    }

    /// Returns `true` if this input has a web page URL set.
    #[must_use]
    pub fn has_web_page_url(&self) -> bool {
        !self.web_page_url.is_empty()
    }

    /// Returns `true` if media size is forced (small or large).
    #[must_use]
    pub const fn has_forced_media_size(&self) -> bool {
        self.force_small_media || self.force_large_media
    }
}

impl Default for InputMessageText {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InputMessageText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

/// Errors that can occur when working with input message text.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InputMessageTextError {
    /// Both force_small_media and force_large_media are set
    #[error("Cannot force both small and large media")]
    ConflictingMediaSize,

    /// Web page URL is set but preview is disabled
    #[error("Web page URL is set but preview is disabled")]
    InvalidPreviewSettings,
}

impl InputMessageText {
    /// Validates the input message text configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Both `force_small_media` and `force_large_media` are true
    /// - `web_page_url` is set but `disable_web_page_preview` is true
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_message_text::InputMessageText;
    ///
    /// let input = InputMessageText::new();
    /// assert!(input.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), InputMessageTextError> {
        if self.force_small_media && self.force_large_media {
            return Err(InputMessageTextError::ConflictingMediaSize);
        }

        if self.has_web_page_url() && self.disable_web_page_preview {
            return Err(InputMessageTextError::InvalidPreviewSettings);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = InputMessageText::new();
        assert!(input.is_empty());
        assert!(input.text().is_empty());
        assert!(input.web_page_url().is_empty());
        assert!(!input.disable_web_page_preview());
        assert!(!input.force_small_media());
        assert!(!input.force_large_media());
        assert!(!input.show_above_text());
        assert!(!input.clear_draft());
    }

    #[test]
    fn test_default() {
        let input = InputMessageText::default();
        assert!(input.is_empty());
    }

    #[test]
    fn test_with_text() {
        let text = FormattedText::new("Hello, world!");
        let input = InputMessageText::with_text(text);
        assert_eq!(input.text().text(), "Hello, world!");
        assert!(!input.is_empty());
    }

    #[test]
    fn test_is_empty() {
        let input = InputMessageText::new();
        assert!(input.is_empty());

        let text = FormattedText::new("Hello");
        let input = InputMessageText::with_text(text);
        assert!(!input.is_empty());
    }

    #[test]
    fn test_is_empty_with_web_page_url() {
        let mut input = InputMessageText::new();
        assert!(input.is_empty());

        input.set_web_page_url("https://example.com".to_string());
        assert!(!input.is_empty());
    }

    #[test]
    fn test_set_text() {
        let mut input = InputMessageText::new();
        let text = FormattedText::new("New text");
        input.set_text(text);
        assert_eq!(input.text().text(), "New text");
    }

    #[test]
    fn test_set_web_page_url() {
        let mut input = InputMessageText::new();
        input.set_web_page_url("https://example.com".to_string());
        assert_eq!(input.web_page_url(), "https://example.com");
        assert!(input.has_web_page_url());
    }

    #[test]
    fn test_set_disable_web_page_preview() {
        let mut input = InputMessageText::new();
        input.set_disable_web_page_preview(true);
        assert!(input.disable_web_page_preview());
    }

    #[test]
    fn test_set_force_small_media() {
        let mut input = InputMessageText::new();
        input.set_force_small_media(true);
        assert!(input.force_small_media());
        assert!(input.has_forced_media_size());
    }

    #[test]
    fn test_set_force_large_media() {
        let mut input = InputMessageText::new();
        input.set_force_large_media(true);
        assert!(input.force_large_media());
        assert!(input.has_forced_media_size());
    }

    #[test]
    fn test_set_show_above_text() {
        let mut input = InputMessageText::new();
        input.set_show_above_text(true);
        assert!(input.show_above_text());
    }

    #[test]
    fn test_set_clear_draft() {
        let mut input = InputMessageText::new();
        input.set_clear_draft(true);
        assert!(input.clear_draft());
    }

    #[test]
    fn test_has_web_page_url() {
        let mut input = InputMessageText::new();
        assert!(!input.has_web_page_url());

        input.set_web_page_url("https://example.com".to_string());
        assert!(input.has_web_page_url());

        input.set_web_page_url(String::new());
        assert!(!input.has_web_page_url());
    }

    #[test]
    fn test_has_forced_media_size() {
        let input = InputMessageText::new();
        assert!(!input.has_forced_media_size());

        let mut input = InputMessageText::new();
        input.set_force_small_media(true);
        assert!(input.has_forced_media_size());

        let mut input = InputMessageText::new();
        input.set_force_large_media(true);
        assert!(input.has_forced_media_size());
    }

    #[test]
    fn test_validate_success() {
        let input = InputMessageText::new();
        assert!(input.validate().is_ok());

        let mut input = InputMessageText::new();
        input.set_force_small_media(true);
        assert!(input.validate().is_ok());

        let mut input = InputMessageText::new();
        input.set_disable_web_page_preview(true);
        assert!(input.validate().is_ok());
    }

    #[test]
    fn test_validate_conflicting_media_size() {
        let mut input = InputMessageText::new();
        input.set_force_small_media(true);
        input.set_force_large_media(true);
        assert!(matches!(
            input.validate(),
            Err(InputMessageTextError::ConflictingMediaSize)
        ));
    }

    #[test]
    fn test_validate_invalid_preview_settings() {
        let mut input = InputMessageText::new();
        input.set_web_page_url("https://example.com".to_string());
        input.set_disable_web_page_preview(true);
        assert!(matches!(
            input.validate(),
            Err(InputMessageTextError::InvalidPreviewSettings)
        ));
    }

    #[test]
    fn test_equality() {
        let input1 = InputMessageText::new();
        let input2 = InputMessageText::new();
        assert_eq!(input1, input2);

        let text = FormattedText::new("Hello");
        let input3 = InputMessageText::with_text(text);
        assert_ne!(input1, input3);
    }

    #[test]
    fn test_clone() {
        let text = FormattedText::new("Hello");
        let input1 = InputMessageText::with_text(text);
        let input2 = input1.clone();
        assert_eq!(input1, input2);
    }

    #[test]
    fn test_display() {
        let text = FormattedText::new("Hello, world!");
        let input = InputMessageText::with_text(text);
        assert_eq!(format!("{}", input), "Hello, world!");
    }

    #[test]
    fn test_serialization() {
        let text = FormattedText::new("Hello");
        let input = InputMessageText::with_text(text);
        let json = serde_json::to_string(&input).unwrap();
        let parsed: InputMessageText = serde_json::from_str(&json).unwrap();
        assert_eq!(input, parsed);
    }

    #[test]
    fn test_bincode_serialization() {
        let text = FormattedText::new("Hello");
        let input = InputMessageText::with_text(text);
        let encoded = bincode::serialize(&input).unwrap();
        let decoded: InputMessageText = bincode::deserialize(&encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_with_web_page_url() {
        let mut input = InputMessageText::new();
        input.set_web_page_url("https://example.com".to_string());
        input.set_show_above_text(true);
        assert_eq!(input.web_page_url(), "https://example.com");
        assert!(input.show_above_text());
    }

    #[test]
    fn test_complete_configuration() {
        let text = FormattedText::new("Check this out!");
        let mut input = InputMessageText::with_text(text);
        input.set_web_page_url("https://example.com".to_string());
        input.set_force_small_media(true);
        input.set_show_above_text(true);
        input.set_clear_draft(true);

        assert_eq!(input.text().text(), "Check this out!");
        assert!(input.has_web_page_url());
        assert!(input.force_small_media());
        assert!(input.show_above_text());
        assert!(input.clear_draft());
        assert!(input.validate().is_ok());
    }

    #[test]
    fn test_clear_after_send() {
        let mut input = InputMessageText::new();
        assert!(!input.clear_draft());

        input.set_clear_draft(true);
        assert!(input.clear_draft());
    }
}
