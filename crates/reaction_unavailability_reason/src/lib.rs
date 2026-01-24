// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Reaction Unavailability Reason
//!
//! Reason why a reaction is unavailable.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};

/// Reason why a reaction is unavailable.
///
/// Based on TDLib's `ReactionUnavailabilityReason` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReactionUnavailabilityReason {
    /// No restriction.
    None,
    /// Reaction from anonymous administrator not allowed.
    AnonymousAdministrator,
    /// Reaction from guest not allowed.
    Guest,
}

impl Default for ReactionUnavailabilityReason {
    fn default() -> Self {
        Self::None
    }
}

impl ReactionUnavailabilityReason {
    /// Checks if this is None (no restriction).
    #[must_use]
    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    /// Checks if this is AnonymousAdministrator.
    #[must_use]
    pub const fn is_anonymous_administrator(self) -> bool {
        matches!(self, Self::AnonymousAdministrator)
    }

    /// Checks if this is Guest.
    #[must_use]
    pub const fn is_guest(self) -> bool {
        matches!(self, Self::Guest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let reason = ReactionUnavailabilityReason::default();
        assert!(reason.is_none());
    }

    #[test]
    fn test_is_none() {
        assert!(ReactionUnavailabilityReason::None.is_none());
        assert!(!ReactionUnavailabilityReason::Guest.is_none());
    }

    #[test]
    fn test_is_anonymous_administrator() {
        assert!(ReactionUnavailabilityReason::AnonymousAdministrator.is_anonymous_administrator());
        assert!(!ReactionUnavailabilityReason::Guest.is_anonymous_administrator());
    }

    #[test]
    fn test_is_guest() {
        assert!(ReactionUnavailabilityReason::Guest.is_guest());
        assert!(!ReactionUnavailabilityReason::AnonymousAdministrator.is_guest());
    }
}
