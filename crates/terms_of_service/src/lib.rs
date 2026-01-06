// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Terms of Service
//!
//! Telegram terms of service information.
//!
//! ## Overview
//!
//! This module provides the [`TermsOfService`] struct, which represents
//! the terms of service information in Telegram. It includes the unique ID,
//! text content, minimum user age, and whether to show a popup.
//! It mirrors TDLib's `TermsOfService` class.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_terms_of_service::TermsOfService;
//!
//! // Create terms of service
//! let tos = TermsOfService::new(
//!     "tos_v1".to_string(),
//!     "Terms content here...".to_string(),
//!     18,
//!     true
//! );
//! assert_eq!(tos.id(), "tos_v1");
//! assert_eq!(tos.min_user_age(), 18);
//! assert!(tos.show_popup());
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Telegram terms of service information.
///
/// This type represents the terms of service that users must agree to
/// when using Telegram.
///
/// # Fields
///
/// - `id` - Unique identifier for this version of terms
/// - `text` - The text content of the terms
/// - `min_user_age` - Minimum user age to accept these terms
/// - `show_popup` - Whether to show the terms in a popup
///
/// # Example
///
/// ```rust
/// use rustgram_terms_of_service::TermsOfService;
///
/// let tos = TermsOfService::new(
///     "tos_v1".to_string(),
///     "Terms content here...".to_string(),
///     18,
///     true
/// );
/// assert_eq!(tos.id(), "tos_v1");
/// assert_eq!(tos.text(), "Terms content here...");
/// assert_eq!(tos.min_user_age(), 18);
/// assert!(tos.show_popup());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TermsOfService {
    /// Unique identifier for this version of terms.
    id: String,

    /// The text content of the terms.
    text: String,

    /// Minimum user age to accept these terms.
    min_user_age: i32,

    /// Whether to show the terms in a popup.
    show_popup: bool,
}

impl TermsOfService {
    /// Creates new terms of service.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this version of terms
    /// * `text` - The text content of the terms
    /// * `min_user_age` - Minimum user age (0 if no restriction)
    /// * `show_popup` - Whether to show the terms in a popup
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// let tos = TermsOfService::new(
    ///     "tos_v1".to_string(),
    ///     "Terms content...".to_string(),
    ///     18,
    ///     true
    /// );
    /// assert_eq!(tos.id(), "tos_v1");
    /// ```
    #[must_use]
    pub fn new(id: String, text: String, min_user_age: i32, show_popup: bool) -> Self {
        Self {
            id,
            text,
            min_user_age,
            show_popup,
        }
    }

    /// Returns the unique identifier for this version of terms.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// let tos = TermsOfService::new(
    ///     "tos_v1".to_string(),
    ///     "Terms content...".to_string(),
    ///     18,
    ///     true
    /// );
    /// assert_eq!(tos.id(), "tos_v1");
    /// ```
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the text content of the terms.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// let tos = TermsOfService::new(
    ///     "tos_v1".to_string(),
    ///     "Terms content here...".to_string(),
    ///     18,
    ///     true
    /// );
    /// assert_eq!(tos.text(), "Terms content here...");
    /// ```
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the minimum user age.
    ///
    /// A value of 0 indicates no age restriction.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// let tos = TermsOfService::new(
    ///     "tos_v1".to_string(),
    ///     "Terms content...".to_string(),
    ///     18,
    ///     true
    /// );
    /// assert_eq!(tos.min_user_age(), 18);
    ///
    /// let no_age_restriction = TermsOfService::new(
    ///     "tos_v2".to_string(),
    ///     "Terms content...".to_string(),
    ///     0,
    ///     false
    /// );
    /// assert_eq!(no_age_restriction.min_user_age(), 0);
    /// ```
    #[must_use]
    pub const fn min_user_age(&self) -> i32 {
        self.min_user_age
    }

    /// Returns whether to show the terms in a popup.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// let tos = TermsOfService::new(
    ///     "tos_v1".to_string(),
    ///     "Terms content...".to_string(),
    ///     18,
    ///     true
    /// );
    /// assert!(tos.show_popup());
    ///
    /// let no_popup = TermsOfService::new(
    ///     "tos_v2".to_string(),
    ///     "Terms content...".to_string(),
    ///     18,
    ///     false
    /// );
    /// assert!(!no_popup.show_popup());
    /// ```
    #[must_use]
    pub const fn show_popup(&self) -> bool {
        self.show_popup
    }

    /// Returns all values as a tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// let tos = TermsOfService::new(
    ///     "tos_v1".to_string(),
    ///     "Terms content...".to_string(),
    ///     18,
    ///     true
    /// );
    /// let (id, text, age, popup) = tos.get();
    /// assert_eq!(id, "tos_v1");
    /// assert_eq!(text, "Terms content...");
    /// assert_eq!(age, 18);
    /// assert!(popup);
    /// ```
    #[must_use]
    pub fn get(&self) -> (&str, &str, i32, bool) {
        (&self.id, &self.text, self.min_user_age, self.show_popup)
    }

    /// Checks if there's an age restriction.
    ///
    /// Returns true if min_user_age > 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// let tos = TermsOfService::new(
    ///     "tos_v1".to_string(),
    ///     "Terms content...".to_string(),
    ///     18,
    ///     true
    /// );
    /// assert!(tos.has_age_restriction());
    ///
    /// let no_restriction = TermsOfService::new(
    ///     "tos_v2".to_string(),
    ///     "Terms content...".to_string(),
    ///     0,
    ///     false
    /// );
    /// assert!(!no_restriction.has_age_restriction());
    /// ```
    #[must_use]
    pub const fn has_age_restriction(&self) -> bool {
        self.min_user_age > 0
    }
}

impl fmt::Display for TermsOfService {
    /// Formats the terms of service for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_terms_of_service::TermsOfService;
    ///
    /// let tos = TermsOfService::new(
    ///     "tos_v1".to_string(),
    ///     "Terms content...".to_string(),
    ///     18,
    ///     true
    /// );
    /// assert_eq!(
    ///     format!("{}", tos),
    ///     "TermsOfService(id: tos_v1, min_age: 18, show_popup: true)"
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TermsOfService(id: {}, min_age: {}, show_popup: {})",
            self.id, self.min_user_age, self.show_popup
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tos = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        assert_eq!(tos.id(), "tos_v1");
        assert_eq!(tos.text(), "Terms content...");
        assert_eq!(tos.min_user_age(), 18);
        assert!(tos.show_popup());
    }

    #[test]
    fn test_id() {
        let tos = TermsOfService::new(
            "tos_v2".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        assert_eq!(tos.id(), "tos_v2");
    }

    #[test]
    fn test_text() {
        let tos = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content here...".to_string(),
            18,
            true,
        );
        assert_eq!(tos.text(), "Terms content here...");
    }

    #[test]
    fn test_min_user_age() {
        let tos = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            21,
            true,
        );
        assert_eq!(tos.min_user_age(), 21);

        let no_age = TermsOfService::new(
            "tos_v2".to_string(),
            "Terms content...".to_string(),
            0,
            false,
        );
        assert_eq!(no_age.min_user_age(), 0);
    }

    #[test]
    fn test_show_popup() {
        let tos = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        assert!(tos.show_popup());

        let no_popup = TermsOfService::new(
            "tos_v2".to_string(),
            "Terms content...".to_string(),
            18,
            false,
        );
        assert!(!no_popup.show_popup());
    }

    #[test]
    fn test_get() {
        let tos = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        let (id, text, age, popup) = tos.get();
        assert_eq!(id, "tos_v1");
        assert_eq!(text, "Terms content...");
        assert_eq!(age, 18);
        assert!(popup);
    }

    #[test]
    fn test_has_age_restriction_true() {
        let tos = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        assert!(tos.has_age_restriction());

        let tos2 = TermsOfService::new(
            "tos_v2".to_string(),
            "Terms content...".to_string(),
            1,
            false,
        );
        assert!(tos2.has_age_restriction());
    }

    #[test]
    fn test_has_age_restriction_false() {
        let tos = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            0,
            false,
        );
        assert!(!tos.has_age_restriction());
    }

    #[test]
    fn test_equality() {
        let tos1 = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        let tos2 = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        assert_eq!(tos1, tos2);
    }

    #[test]
    fn test_inequality_different_id() {
        let tos1 = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        let tos2 = TermsOfService::new(
            "tos_v2".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        assert_ne!(tos1, tos2);
    }

    #[test]
    fn test_inequality_different_text() {
        let tos1 = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        let tos2 = TermsOfService::new(
            "tos_v1".to_string(),
            "Different content...".to_string(),
            18,
            true,
        );
        assert_ne!(tos1, tos2);
    }

    #[test]
    fn test_inequality_different_age() {
        let tos1 = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        let tos2 = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            21,
            true,
        );
        assert_ne!(tos1, tos2);
    }

    #[test]
    fn test_inequality_different_popup() {
        let tos1 = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        let tos2 = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            false,
        );
        assert_ne!(tos1, tos2);
    }

    #[test]
    fn test_clone_semantics() {
        let tos1 = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        let tos2 = tos1.clone();
        assert_eq!(tos1, tos2);
    }

    #[test]
    fn test_display_format() {
        let tos = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        assert_eq!(
            format!("{}", tos),
            "TermsOfService(id: tos_v1, min_age: 18, show_popup: true)"
        );

        let tos2 = TermsOfService::new(
            "tos_v2".to_string(),
            "Terms content...".to_string(),
            0,
            false,
        );
        assert_eq!(
            format!("{}", tos2),
            "TermsOfService(id: tos_v2, min_age: 0, show_popup: false)"
        );
    }

    #[test]
    fn test_debug_format() {
        let tos = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );
        let debug_str = format!("{:?}", tos);
        assert!(debug_str.contains("TermsOfService"));
        assert!(debug_str.contains("tos_v1"));
    }

    #[test]
    fn test_empty_strings() {
        let tos = TermsOfService::new(String::new(), String::new(), 18, true);
        assert_eq!(tos.id(), "");
        assert_eq!(tos.text(), "");
    }

    #[test]
    fn test_negative_age() {
        let tos = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            -1,
            true,
        );
        assert_eq!(tos.min_user_age(), -1);
        assert!(!tos.has_age_restriction());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = TermsOfService::new(
            "tos_v1".to_string(),
            "Terms content...".to_string(),
            18,
            true,
        );

        // Test JSON serialization
        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(
            json,
            r#"{"id":"tos_v1","text":"Terms content...","min_user_age":18,"show_popup":true}"#
        );

        let deserialized: TermsOfService = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);

        // Test binary serialization
        let encoded = bincode::serialize(&original).unwrap();
        let decoded: TermsOfService = bincode::deserialize(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_empty_fields() {
        let original = TermsOfService::new(String::new(), String::new(), 0, false);

        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(
            json,
            r#"{"id":"","text":"","min_user_age":0,"show_popup":false}"#
        );

        let deserialized: TermsOfService = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_with_special_chars() {
        let original = TermsOfService::new(
            "tos_v1\nwith\nnewlines".to_string(),
            "Text with \"quotes\" and 'apostrophes'".to_string(),
            18,
            true,
        );

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TermsOfService = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
