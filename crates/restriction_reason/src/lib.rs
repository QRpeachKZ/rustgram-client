// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Restriction Reason
//!
//! Reason for content or account restriction in Telegram.
//!
//! Based on TDLib's `RestrictionReason` from `td/telegram/RestrictionReason.h`.
//!
//! # Overview
//!
//! A `RestrictionReason` contains information about why a user, chat, or message
//! is restricted in certain regions or platforms. It includes the platform identifier,
//! a reason code, and a human-readable description.
//!
//! # Example
//!
//! ```rust
//! use rustgram_restriction_reason::RestrictionReason;
//!
//! let reason = RestrictionReason::new(
//!     "ios".to_string(),
//!     "sensitive".to_string(),
//!     "Content restricted due to sensitive nature".to_string(),
//! );
//! assert!(reason.is_sensitive());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Reason for content or account restriction.
///
/// Contains information about why a user, chat, or message is restricted
/// in certain regions or platforms.
///
/// # Fields
///
/// * `platform` - Platform where the restriction applies (e.g., "ios", "android")
/// * `reason` - Reason code (e.g., "sensitive")
/// * `description` - Human-readable description of the restriction
///
/// # Example
///
/// ```
/// use rustgram_restriction_reason::RestrictionReason;
///
/// let reason = RestrictionReason::new(
///     "ios".to_string(),
///     "sensitive".to_string(),
///     "Restricted content".to_string(),
/// );
/// assert_eq!(reason.platform(), "ios");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RestrictionReason {
    /// Platform where the restriction applies.
    platform: String,
    /// Reason code (e.g., "sensitive").
    reason: String,
    /// Human-readable description of the restriction.
    description: String,
}

impl RestrictionReason {
    /// Creates a new `RestrictionReason`.
    ///
    /// If `description` is empty, it will be set to the value of `reason`.
    ///
    /// # Arguments
    ///
    /// * `platform` - Platform where the restriction applies
    /// * `reason` - Reason code (e.g., "sensitive")
    /// * `description` - Human-readable description of the restriction
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_restriction_reason::RestrictionReason;
    ///
    /// let reason = RestrictionReason::new(
    ///     "ios".to_string(),
    ///     "sensitive".to_string(),
    ///     "Restricted content".to_string(),
    /// );
    /// ```
    #[must_use]
    pub fn new(platform: String, reason: String, description: String) -> Self {
        let description = if description.is_empty() {
            reason.clone()
        } else {
            description
        };

        Self {
            platform,
            reason,
            description,
        }
    }

    /// Checks if this is a sensitive content restriction.
    ///
    /// Returns `true` if the reason code equals "sensitive".
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_restriction_reason::RestrictionReason;
    ///
    /// let reason = RestrictionReason::new(
    ///     "ios".to_string(),
    ///     "sensitive".to_string(),
    ///     "Restricted content".to_string(),
    /// );
    /// assert!(reason.is_sensitive());
    ///
    /// let other = RestrictionReason::new(
    ///     "android".to_string(),
    ///     "copyright".to_string(),
    ///     "Copyright violation".to_string(),
    /// );
    /// assert!(!other.is_sensitive());
    /// ```
    #[must_use]
    pub fn is_sensitive(&self) -> bool {
        self.reason == "sensitive"
    }

    /// Returns the platform where the restriction applies.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_restriction_reason::RestrictionReason;
    ///
    /// let reason = RestrictionReason::new(
    ///     "ios".to_string(),
    ///     "sensitive".to_string(),
    ///     "Restricted".to_string(),
    /// );
    /// assert_eq!(reason.platform(), "ios");
    /// ```
    #[must_use]
    pub fn platform(&self) -> &str {
        &self.platform
    }

    /// Returns the reason code.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_restriction_reason::RestrictionReason;
    ///
    /// let reason = RestrictionReason::new(
    ///     "ios".to_string(),
    ///     "sensitive".to_string(),
    ///     "Restricted".to_string(),
    /// );
    /// assert_eq!(reason.reason(), "sensitive");
    /// ```
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Returns the human-readable description.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_restriction_reason::RestrictionReason;
    ///
    /// let reason = RestrictionReason::new(
    ///     "ios".to_string(),
    ///     "sensitive".to_string(),
    ///     "Restricted content".to_string(),
    /// );
    /// assert_eq!(reason.description(), "Restricted content");
    /// ```
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for RestrictionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RestrictionReason[{}, {}, {}]",
            self.platform, self.reason, self.description
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted content".to_string(),
        );
        assert_eq!(reason.platform(), "ios");
        assert_eq!(reason.reason(), "sensitive");
        assert_eq!(reason.description(), "Restricted content");
    }

    #[test]
    fn test_default() {
        let reason = RestrictionReason::default();
        assert_eq!(reason.platform(), "");
        assert_eq!(reason.reason(), "");
        assert_eq!(reason.description(), "");
    }

    #[test]
    fn test_empty_description_defaults_to_reason() {
        let reason = RestrictionReason::new(
            "android".to_string(),
            "copyright".to_string(),
            "".to_string(),
        );
        assert_eq!(reason.description(), "copyright");
    }

    #[test]
    fn test_is_sensitive_true() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        assert!(reason.is_sensitive());
    }

    #[test]
    fn test_is_sensitive_false() {
        let reason = RestrictionReason::new(
            "android".to_string(),
            "copyright".to_string(),
            "Copyright violation".to_string(),
        );
        assert!(!reason.is_sensitive());
    }

    #[test]
    fn test_is_sensitive_case_sensitive() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "Sensitive".to_string(),
            "Restricted".to_string(),
        );
        assert!(!reason.is_sensitive());
    }

    #[test]
    fn test_platform() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        assert_eq!(reason.platform(), "ios");
    }

    #[test]
    fn test_reason() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        assert_eq!(reason.reason(), "sensitive");
    }

    #[test]
    fn test_description() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted content".to_string(),
        );
        assert_eq!(reason.description(), "Restricted content");
    }

    #[test]
    fn test_equality() {
        let reason1 = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        let reason2 = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        let reason3 = RestrictionReason::new(
            "android".to_string(),
            "copyright".to_string(),
            "Violation".to_string(),
        );

        assert_eq!(reason1, reason2);
        assert_ne!(reason1, reason3);
    }

    #[test]
    fn test_clone() {
        let reason1 = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        let reason2 = reason1.clone();
        assert_eq!(reason1, reason2);
    }

    #[test]
    fn test_display() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted content".to_string(),
        );
        let display = format!("{reason}");
        assert!(display.contains("RestrictionReason"));
        assert!(display.contains("ios"));
        assert!(display.contains("sensitive"));
        assert!(display.contains("Restricted content"));
    }

    #[test]
    fn test_debug_formatting() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        let debug = format!("{reason:?}");
        assert!(debug.contains("ios"));
        assert!(debug.contains("sensitive"));
    }

    #[test]
    fn test_serialization() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted content".to_string(),
        );
        let json = serde_json::to_string(&reason).expect("Failed to serialize");
        assert!(json.contains("ios"));
        assert!(json.contains("sensitive"));

        let deserialized: RestrictionReason =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, reason);
    }

    #[test]
    fn test_serialization_with_empty_description() {
        let reason = RestrictionReason::new(
            "android".to_string(),
            "copyright".to_string(),
            "".to_string(),
        );
        let json = serde_json::to_string(&reason).expect("Failed to serialize");

        let deserialized: RestrictionReason =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.description(), "copyright");
    }

    #[test]
    fn test_empty_platform() {
        let reason = RestrictionReason::new(
            "".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        assert_eq!(reason.platform(), "");
        assert!(reason.is_sensitive());
    }

    #[test]
    fn test_empty_reason() {
        let reason =
            RestrictionReason::new("ios".to_string(), "".to_string(), "Restricted".to_string());
        assert_eq!(reason.reason(), "");
        assert!(!reason.is_sensitive());
    }

    #[test]
    fn test_all_empty() {
        let reason = RestrictionReason::new("".to_string(), "".to_string(), "".to_string());
        assert_eq!(reason.platform(), "");
        assert_eq!(reason.reason(), "");
        assert_eq!(reason.description(), "");
        assert!(!reason.is_sensitive());
    }

    #[test]
    fn test_multi_word_description() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Content restricted due to sensitive nature in this region".to_string(),
        );
        assert_eq!(
            reason.description(),
            "Content restricted due to sensitive nature in this region"
        );
    }

    #[test]
    fn test_special_characters_in_reason() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "content-warning".to_string(),
            "Restricted".to_string(),
        );
        assert_eq!(reason.reason(), "content-warning");
        assert!(!reason.is_sensitive());
    }

    #[test]
    fn test_unicode_in_description() {
        let reason = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted content: Contains sensitive information".to_string(),
        );
        assert!(reason.description().contains("sensitive"));
    }

    #[test]
    fn test_multiple_platforms() {
        let platforms = vec!["ios", "android", "web", "desktop"];

        for platform in platforms {
            let reason = RestrictionReason::new(
                platform.to_string(),
                "sensitive".to_string(),
                "Restricted".to_string(),
            );
            assert_eq!(reason.platform(), platform);
            assert!(reason.is_sensitive());
        }
    }

    #[test]
    fn test_partial_equality_platform_differs() {
        let reason1 = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        let reason2 = RestrictionReason::new(
            "android".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        assert_ne!(reason1, reason2);
    }

    #[test]
    fn test_partial_equality_reason_differs() {
        let reason1 = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        let reason2 = RestrictionReason::new(
            "ios".to_string(),
            "copyright".to_string(),
            "Restricted".to_string(),
        );
        assert_ne!(reason1, reason2);
    }

    #[test]
    fn test_partial_equality_description_differs() {
        let reason1 = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Restricted".to_string(),
        );
        let reason2 = RestrictionReason::new(
            "ios".to_string(),
            "sensitive".to_string(),
            "Different description".to_string(),
        );
        assert_ne!(reason1, reason2);
    }
}
