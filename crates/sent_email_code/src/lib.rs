// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Sent Email Code
//!
//! Information about a sent email authentication code.
//!
//! Based on TDLib's `SentEmailCode` from `td/telegram/SentEmailCode.h`.
//!
//! # Overview
//!
//! A `SentEmailCode` contains information about an email authentication code
//! that has been sent to a user, including the email address pattern and code length.
//!
//! # Example
//!
//! ```rust
//! use rustgram_sent_email_code::SentEmailCode;
//!
//! let code = SentEmailCode::new("u***@example.com", 6);
//! assert!(!code.is_empty());
//! assert_eq!(code.code_length(), 6);
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Information about a sent email authentication code.
///
/// Contains the email address pattern (with some characters hidden) and
/// the length of the authentication code.
///
/// # TDLib Mapping
///
/// - `SentEmailCode::new(pattern, length)` → TDLib: `SentEmailCode(string, int32)`
/// - `is_empty()` → TDLib: Checks if `email_address_pattern_.empty()`
///
/// # Example
///
/// ```rust
/// use rustgram_sent_email_code::SentEmailCode;
///
/// let code = SentEmailCode::new("u***@example.com", 6);
/// assert_eq!(code.email_address_pattern(), "u***@example.com");
/// assert_eq!(code.code_length(), 6);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SentEmailCode {
    /// Email address pattern with some characters hidden for privacy
    email_address_pattern: String,
    /// Length of the authentication code
    code_length: i32,
}

impl SentEmailCode {
    /// Creates a new `SentEmailCode`.
    ///
    /// # Arguments
    ///
    /// * `email_address_pattern` - Email address pattern with hidden characters
    /// * `code_length` - Length of the authentication code
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sent_email_code::SentEmailCode;
    ///
    /// let code = SentEmailCode::new("u***@example.com", 6);
    /// assert_eq!(code.email_address_pattern(), "u***@example.com");
    /// ```
    #[must_use]
    pub fn new(email_address_pattern: impl Into<String>, code_length: i32) -> Self {
        Self {
            email_address_pattern: email_address_pattern.into(),
            code_length,
        }
    }

    /// Returns the email address pattern.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sent_email_code::SentEmailCode;
    ///
    /// let code = SentEmailCode::new("u***@example.com", 6);
    /// assert_eq!(code.email_address_pattern(), "u***@example.com");
    /// ```
    #[must_use]
    pub fn email_address_pattern(&self) -> &str {
        &self.email_address_pattern
    }

    /// Returns the length of the authentication code.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sent_email_code::SentEmailCode;
    ///
    /// let code = SentEmailCode::new("u***@example.com", 6);
    /// assert_eq!(code.code_length(), 6);
    /// ```
    #[must_use]
    pub fn code_length(&self) -> i32 {
        self.code_length
    }

    /// Checks if this email code is empty (no pattern set).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sent_email_code::SentEmailCode;
    ///
    /// assert!(SentEmailCode::new("", 0).is_empty());
    /// assert!(!SentEmailCode::new("u***@example.com", 6).is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.email_address_pattern.is_empty()
    }
}


impl fmt::Display for SentEmailCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EmailCode({} length={})",
            self.email_address_pattern, self.code_length
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let code = SentEmailCode::new("u***@example.com", 6);
        assert_eq!(code.email_address_pattern(), "u***@example.com");
        assert_eq!(code.code_length(), 6);
    }

    #[test]
    fn test_default() {
        let code = SentEmailCode::default();
        assert_eq!(code.email_address_pattern(), "");
        assert_eq!(code.code_length(), 0);
        assert!(code.is_empty());
    }

    #[test]
    fn test_from_string() {
        let pattern = String::from("t***@test.org");
        let code = SentEmailCode::new(pattern.clone(), 8);
        assert_eq!(code.email_address_pattern(), pattern);
    }

    #[test]
    fn test_from_str() {
        let code = SentEmailCode::new("u***@example.com", 6);
        assert_eq!(code.email_address_pattern(), "u***@example.com");
    }

    #[test]
    fn test_is_empty_true() {
        let code = SentEmailCode::new("", 0);
        assert!(code.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let code = SentEmailCode::new("u***@example.com", 6);
        assert!(!code.is_empty());
    }

    #[test]
    fn test_is_empty_zero_code_length() {
        let code = SentEmailCode::new("u***@example.com", 0);
        assert!(!code.is_empty());
    }

    #[test]
    fn test_equality() {
        let code1 = SentEmailCode::new("u***@example.com", 6);
        let code2 = SentEmailCode::new("u***@example.com", 6);
        let code3 = SentEmailCode::new("t***@test.org", 8);

        assert_eq!(code1, code2);
        assert_ne!(code1, code3);
    }

    #[test]
    fn test_equality_different_length() {
        let code1 = SentEmailCode::new("u***@example.com", 6);
        let code2 = SentEmailCode::new("u***@example.com", 8);

        assert_ne!(code1, code2);
    }

    #[test]
    fn test_clone() {
        let code1 = SentEmailCode::new("u***@example.com", 6);
        let code2 = code1.clone();
        assert_eq!(code1, code2);
    }

    #[test]
    fn test_display() {
        let code = SentEmailCode::new("u***@example.com", 6);
        let display = format!("{code}");
        assert!(display.contains("u***@example.com"));
        assert!(display.contains("6"));
    }

    #[test]
    fn test_display_empty() {
        let code = SentEmailCode::default();
        let display = format!("{code}");
        assert!(display.contains("EmailCode"));
    }

    #[test]
    fn test_serialization() {
        let code = SentEmailCode::new("u***@example.com", 6);
        let json = serde_json::to_string(&code).expect("Failed to serialize");
        assert!(json.contains("u***@example.com"));
        assert!(json.contains("6"));

        let deserialized: SentEmailCode =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, code);
    }

    #[test]
    fn test_serialization_empty() {
        let code = SentEmailCode::default();
        let json = serde_json::to_string(&code).expect("Failed to serialize");

        let deserialized: SentEmailCode =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, code);
        assert!(deserialized.is_empty());
    }

    #[test]
    fn test_various_code_lengths() {
        for length in [4, 5, 6, 8, 10] {
            let code = SentEmailCode::new("u***@example.com", length);
            assert_eq!(code.code_length(), length);
        }
    }

    #[test]
    fn test_negative_code_length() {
        let code = SentEmailCode::new("u***@example.com", -1);
        assert_eq!(code.code_length(), -1);
    }

    #[test]
    fn test_various_patterns() {
        let patterns = vec![
            "u***@example.com",
            "t***@test.org",
            "a***@gmail.com",
            "j***@outlook.com",
        ];

        for pattern in patterns {
            let code = SentEmailCode::new(pattern, 6);
            assert_eq!(code.email_address_pattern(), pattern);
            assert!(!code.is_empty());
        }
    }

    #[test]
    fn test_special_characters_in_pattern() {
        let code = SentEmailCode::new("u+s***@ex-ample.com", 6);
        assert_eq!(code.email_address_pattern(), "u+s***@ex-ample.com");
    }
}
