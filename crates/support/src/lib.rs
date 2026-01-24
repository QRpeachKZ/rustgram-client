// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Support Information
//!
//! Support information for Telegram users.
//!
//! ## Overview
//!
//! This module provides types for user support information, including
//! the support name and user support details.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_support::UserSupportInfo;
//! use rustgram_formatted_text::FormattedText;
//!
//! // Create user support info
//! let info = UserSupportInfo::new(
//!     FormattedText::new("Support message"),
//!     "Telegram Support".to_string(),
//!     1704067200
//! );
//! ```

use std::fmt;

use rustgram_formatted_text::FormattedText;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// User support information.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib's `userSupportInfo` type.
///
/// # Example
///
/// ```rust
/// use rustgram_support::UserSupportInfo;
/// use rustgram_formatted_text::FormattedText;
///
/// let info = UserSupportInfo::new(
///     FormattedText::new("Support message"),
///     "Telegram Support".to_string(),
///     1704067200
/// );
/// assert_eq!(info.author(), "Telegram Support");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserSupportInfo {
    message: FormattedText,
    author: String,
    date: i32,
}

impl UserSupportInfo {
    /// Creates a new user support info.
    ///
    /// # Arguments
    ///
    /// * `message` - The support message
    /// * `author` - The author of the message
    /// * `date` - The message date (Unix timestamp)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_support::UserSupportInfo;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let info = UserSupportInfo::new(
    ///     FormattedText::new("Support message"),
    ///     "Telegram Support".to_string(),
    ///     1704067200
    /// );
    /// ```
    #[must_use]
    pub fn new(message: FormattedText, author: String, date: i32) -> Self {
        Self {
            message,
            author,
            date,
        }
    }

    /// Returns the support message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_support::UserSupportInfo;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let info = UserSupportInfo::new(
    ///     FormattedText::new("Support message"),
    ///     "Telegram Support".to_string(),
    ///     1704067200
    /// );
    /// assert_eq!(info.message().text(), "Support message");
    /// ```
    #[must_use]
    pub const fn message(&self) -> &FormattedText {
        &self.message
    }

    /// Returns the author of the support message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_support::UserSupportInfo;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let info = UserSupportInfo::new(
    ///     FormattedText::new("Support message"),
    ///     "Telegram Support".to_string(),
    ///     1704067200
    /// );
    /// assert_eq!(info.author(), "Telegram Support");
    /// ```
    #[must_use]
    pub fn author(&self) -> &str {
        &self.author
    }

    /// Returns the date of the support message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_support::UserSupportInfo;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let info = UserSupportInfo::new(
    ///     FormattedText::new("Support message"),
    ///     "Telegram Support".to_string(),
    ///     1704067200
    /// );
    /// assert_eq!(info.date(), 1704067200);
    /// ```
    #[must_use]
    pub const fn date(&self) -> i32 {
        self.date
    }
}

impl fmt::Display for UserSupportInfo {
    /// Formats the user support info for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_support::UserSupportInfo;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let info = UserSupportInfo::new(
    ///     FormattedText::new("Support message"),
    ///     "Telegram Support".to_string(),
    ///     1704067200
    /// );
    /// let s = format!("{}", info);
    /// assert!(s.contains("UserSupportInfo"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UserSupportInfo(author: {}, date: {})",
            self.author, self.date
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info = UserSupportInfo::new(
            FormattedText::new("Support message"),
            "Telegram Support".to_string(),
            1704067200,
        );
        assert_eq!(info.message().text(), "Support message");
        assert_eq!(info.author(), "Telegram Support");
        assert_eq!(info.date(), 1704067200);
    }

    #[test]
    fn test_message() {
        let info =
            UserSupportInfo::new(FormattedText::new("Test message"), "Author".to_string(), 0);
        assert_eq!(info.message().text(), "Test message");
    }

    #[test]
    fn test_author() {
        let info = UserSupportInfo::new(FormattedText::new(""), "Test Author".to_string(), 0);
        assert_eq!(info.author(), "Test Author");
    }

    #[test]
    fn test_date() {
        let info = UserSupportInfo::new(FormattedText::new(""), "".to_string(), 12345);
        assert_eq!(info.date(), 12345);
    }

    #[test]
    fn test_equality() {
        let info1 = UserSupportInfo::new(FormattedText::new("message"), "author".to_string(), 100);
        let info2 = UserSupportInfo::new(FormattedText::new("message"), "author".to_string(), 100);
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_inequality() {
        let info1 = UserSupportInfo::new(FormattedText::new("message1"), "author".to_string(), 100);
        let info2 = UserSupportInfo::new(FormattedText::new("message2"), "author".to_string(), 100);
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_clone_semantics() {
        let info1 = UserSupportInfo::new(FormattedText::new("message"), "author".to_string(), 100);
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_display_format() {
        let info = UserSupportInfo::new(FormattedText::new("message"), "author".to_string(), 100);
        let s = format!("{}", info);
        assert!(s.contains("UserSupportInfo"));
    }

    #[test]
    fn test_debug_format() {
        let info = UserSupportInfo::new(FormattedText::new("message"), "author".to_string(), 100);
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("UserSupportInfo"));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original =
            UserSupportInfo::new(FormattedText::new("message"), "author".to_string(), 100);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: UserSupportInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
