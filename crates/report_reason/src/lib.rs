// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Report Reason
//!
//! Reason for reporting content.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};

/// Report reason.
///
/// Based on TDLib's `ReportReason` class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportReason {
    /// Spam.
    Spam,
    /// Violence.
    Violence,
    /// Pornography.
    Pornography,
    /// Child abuse.
    ChildAbuse,
    /// Copyright infringement.
    Copyright,
    /// Unrelated location.
    UnrelatedLocation,
    /// Fake account.
    Fake,
    /// Illegal drugs.
    IllegalDrugs,
    /// Personal details.
    PersonalDetails,
    /// Custom reason with message.
    Custom {
        /// The custom message.
        message: String,
    },
}

impl Default for ReportReason {
    fn default() -> Self {
        Self::Spam
    }
}

impl ReportReason {
    /// Returns the message for custom reports.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Custom { message } => Some(message),
            _ => None,
        }
    }

    /// Checks if this is Spam.
    #[must_use]
    pub const fn is_spam(&self) -> bool {
        matches!(self, Self::Spam)
    }

    /// Checks if this is UnrelatedLocation.
    #[must_use]
    pub const fn is_unrelated_location(&self) -> bool {
        matches!(self, Self::UnrelatedLocation)
    }

    /// Creates a custom report reason.
    #[must_use]
    pub fn custom(message: String) -> Self {
        Self::Custom { message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let reason = ReportReason::default();
        assert!(reason.is_spam());
    }

    #[test]
    fn test_is_spam() {
        assert!(ReportReason::Spam.is_spam());
    }

    #[test]
    fn test_custom() {
        let reason = ReportReason::custom("test message".to_string());
        assert_eq!(reason.message(), Some("test message"));
    }

    #[test]
    fn test_is_unrelated_location() {
        assert!(ReportReason::UnrelatedLocation.is_unrelated_location());
    }

    #[test]
    fn test_equality() {
        let r1 = ReportReason::Spam;
        let r2 = ReportReason::Spam;
        let r3 = ReportReason::Violence;
        assert_eq!(r1, r2);
        assert_ne!(r1, r3);
    }
}
