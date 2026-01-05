// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Verification Status
//!
//! Verification status for Telegram users and bots.
//!
//! ## Overview
//!
//! This module provides types for representing verification status of Telegram
//! entities. It mirrors TDLib's `VerificationStatus` functionality, providing
//! information about whether a user/bot is verified, marked as scam, or fake.
//!
//! ## Types
//!
//! - [`VerificationStatus`] - Main type representing verification status
//! - [`CustomEmojiId`] - Identifier for custom emoji used in bot verification
//!
//! ## Example
//!
//! ```rust
//! use rustgram_verification_status::{VerificationStatus, CustomEmojiId};
//!
//! // Create a verified user status
//! let status = VerificationStatus::verified();
//! assert_eq!(status.is_verified(), true);
//!
//! // Create a bot verification with custom emoji
//! let emoji_id = CustomEmojiId::new(1234567890);
//! let bot_status = VerificationStatus::bot_verified(emoji_id);
//! assert_eq!(bot_status.is_verified(), true);
//! assert!(bot_status.bot_verification_custom_emoji_id().is_some());
//! ```

use serde::{Deserialize, Serialize};

/// Custom emoji identifier used for bot verification.
///
/// Represents a unique identifier for a custom emoji that is displayed
/// next to verified bot names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomEmojiId {
    /// The unique identifier
    id: i64,
}

impl CustomEmojiId {
    /// Creates a new custom emoji ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_verification_status::CustomEmojiId;
    ///
    /// let emoji_id = CustomEmojiId::new(1234567890);
    /// ```
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self { id }
    }

    /// Returns the inner identifier value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_verification_status::CustomEmojiId;
    ///
    /// let emoji_id = CustomEmojiId::new(1234567890);
    /// assert_eq!(emoji_id.get(), 1234567890);
    /// ```
    #[must_use]
    pub const fn get(self) -> i64 {
        self.id
    }

    /// Checks if this is a valid custom emoji ID.
    ///
    /// A valid ID must be positive.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_verification_status::CustomEmojiId;
    ///
    /// assert!(CustomEmojiId::new(1234567890).is_valid());
    /// assert!(!CustomEmojiId::new(0).is_valid());
    /// assert!(!CustomEmojiId::new(-1).is_valid());
    /// ```
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.id > 0
    }
}

impl From<i64> for CustomEmojiId {
    fn from(id: i64) -> Self {
        Self::new(id)
    }
}

/// Verification status for a Telegram user or bot.
///
/// Contains information about verification state, scam/fake markings,
/// and optional custom emoji for bot verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationStatus {
    /// Whether the entity is verified
    is_verified: bool,
    /// Whether the entity is marked as scam
    is_scam: bool,
    /// Whether the entity is marked as fake
    is_fake: bool,
    /// Custom emoji ID for bot verification (optional)
    bot_verification_custom_emoji_id: Option<i64>,
}

impl VerificationStatus {
    /// Creates a new verification status with all fields.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_verification_status::VerificationStatus;
    ///
    /// let status = VerificationStatus::new(
    ///     true,   // verified
    ///     false,  // not scam
    ///     false,  // not fake
    ///     Some(1234567890), // custom emoji
    /// );
    /// ```
    #[must_use]
    pub const fn new(
        is_verified: bool,
        is_scam: bool,
        is_fake: bool,
        bot_verification_custom_emoji_id: Option<i64>,
    ) -> Self {
        Self {
            is_verified,
            is_scam,
            is_fake,
            bot_verification_custom_emoji_id,
        }
    }

    /// Creates a verification status for a verified user/bot.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_verification_status::VerificationStatus;
    ///
    /// let status = VerificationStatus::verified();
    /// assert!(status.is_verified());
    /// ```
    #[must_use]
    pub const fn verified() -> Self {
        Self {
            is_verified: true,
            is_scam: false,
            is_fake: false,
            bot_verification_custom_emoji_id: None,
        }
    }

    /// Creates a verification status for a bot with custom emoji.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_verification_status::{VerificationStatus, CustomEmojiId};
    ///
    /// let emoji_id = CustomEmojiId::new(1234567890);
    /// let status = VerificationStatus::bot_verified(emoji_id);
    /// assert!(status.is_verified());
    /// ```
    #[must_use]
    pub const fn bot_verified(custom_emoji_id: CustomEmojiId) -> Self {
        Self {
            is_verified: true,
            is_scam: false,
            is_fake: false,
            bot_verification_custom_emoji_id: Some(custom_emoji_id.get()),
        }
    }

    /// Creates a verification status for a scam entity.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_verification_status::VerificationStatus;
    ///
    /// let status = VerificationStatus::scam();
    /// assert!(status.is_scam());
    /// ```
    #[must_use]
    pub const fn scam() -> Self {
        Self {
            is_verified: false,
            is_scam: true,
            is_fake: false,
            bot_verification_custom_emoji_id: None,
        }
    }

    /// Creates a verification status for a fake entity.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_verification_status::VerificationStatus;
    ///
    /// let status = VerificationStatus::fake();
    /// assert!(status.is_fake());
    /// ```
    #[must_use]
    pub const fn fake() -> Self {
        Self {
            is_verified: false,
            is_scam: false,
            is_fake: true,
            bot_verification_custom_emoji_id: None,
        }
    }

    /// Creates a default (unverified) status.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_verification_status::VerificationStatus;
    ///
    /// let status = VerificationStatus::default();
    /// assert!(!status.is_verified());
    /// assert!(!status.is_scam());
    /// assert!(!status.is_fake());
    /// ```
    #[must_use]
    pub const fn default() -> Self {
        Self {
            is_verified: false,
            is_scam: false,
            is_fake: false,
            bot_verification_custom_emoji_id: None,
        }
    }

    /// Checks if this status represents any meaningful verification state.
    ///
    /// Returns `false` for completely default (unverified) status.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_verification_status::VerificationStatus;
    ///
    /// assert!(!VerificationStatus::default().has_verification());
    /// assert!(VerificationStatus::verified().has_verification());
    /// assert!(VerificationStatus::scam().has_verification());
    /// ```
    #[must_use]
    pub const fn has_verification(&self) -> bool {
        self.is_verified
            || self.is_scam
            || self.is_fake
            || self.bot_verification_custom_emoji_id.is_some()
    }

    /// Returns whether the entity is verified.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.is_verified
    }

    /// Returns whether the entity is marked as scam.
    #[must_use]
    pub const fn is_scam(&self) -> bool {
        self.is_scam
    }

    /// Returns whether the entity is marked as fake.
    #[must_use]
    pub const fn is_fake(&self) -> bool {
        self.is_fake
    }

    /// Returns the custom emoji ID for bot verification, if present.
    #[must_use]
    pub const fn bot_verification_custom_emoji_id(&self) -> Option<i64> {
        self.bot_verification_custom_emoji_id
    }
}

impl Default for VerificationStatus {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // CustomEmojiId tests
    #[test]
    fn test_custom_emoji_id_new() {
        let id = CustomEmojiId::new(1234567890);
        assert_eq!(id.get(), 1234567890);
    }

    #[test]
    fn test_custom_emoji_id_valid() {
        assert!(CustomEmojiId::new(1).is_valid());
        assert!(CustomEmojiId::new(1234567890).is_valid());
        assert!(!CustomEmojiId::new(0).is_valid());
        assert!(!CustomEmojiId::new(-1).is_valid());
    }

    #[test]
    fn test_custom_emoji_id_from_i64() {
        let id: CustomEmojiId = 12345.into();
        assert_eq!(id.get(), 12345);
    }

    #[test]
    fn test_custom_emoji_id_copy() {
        let id1 = CustomEmojiId::new(123);
        let id2 = id1;
        assert_eq!(id1, id2);
    }

    // VerificationStatus tests
    #[test]
    fn test_verification_status_new() {
        let status = VerificationStatus::new(true, false, false, Some(123));
        assert!(status.is_verified());
        assert!(!status.is_scam());
        assert!(!status.is_fake());
        assert_eq!(status.bot_verification_custom_emoji_id(), Some(123));
    }

    #[test]
    fn test_verification_status_verified() {
        let status = VerificationStatus::verified();
        assert!(status.is_verified());
        assert!(!status.is_scam());
        assert!(!status.is_fake());
    }

    #[test]
    fn test_verification_status_bot_verified() {
        let emoji_id = CustomEmojiId::new(12345);
        let status = VerificationStatus::bot_verified(emoji_id);
        assert!(status.is_verified());
        assert_eq!(status.bot_verification_custom_emoji_id(), Some(12345));
    }

    #[test]
    fn test_verification_status_scam() {
        let status = VerificationStatus::scam();
        assert!(!status.is_verified());
        assert!(status.is_scam());
        assert!(!status.is_fake());
    }

    #[test]
    fn test_verification_status_fake() {
        let status = VerificationStatus::fake();
        assert!(!status.is_verified());
        assert!(!status.is_scam());
        assert!(status.is_fake());
    }

    #[test]
    fn test_verification_status_default() {
        let status = VerificationStatus::default();
        assert!(!status.is_verified());
        assert!(!status.is_scam());
        assert!(!status.is_fake());
        assert_eq!(status.bot_verification_custom_emoji_id(), None);
    }

    #[test]
    fn test_has_verification() {
        assert!(!VerificationStatus::default().has_verification());
        assert!(VerificationStatus::verified().has_verification());
        assert!(VerificationStatus::scam().has_verification());
        assert!(VerificationStatus::fake().has_verification());
        assert!(VerificationStatus::bot_verified(CustomEmojiId::new(123)).has_verification());
    }

    #[test]
    fn test_equality() {
        let status1 = VerificationStatus::verified();
        let status2 = VerificationStatus::verified();
        assert_eq!(status1, status2);

        let status3 = VerificationStatus::scam();
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_clone() {
        let status1 = VerificationStatus::new(true, false, true, Some(123));
        let status2 = status1.clone();
        assert_eq!(status1, status2);
    }

    // Serialization tests
    #[test]
    fn test_serialize_custom_emoji_id() {
        let id = CustomEmojiId::new(12345);
        let serialized = bincode::serialize(&id).unwrap();
        let deserialized: CustomEmojiId = bincode::deserialize(&serialized).unwrap();
        assert_eq!(id, deserialized);
    }

    #[test]
    fn test_serialize_verification_status() {
        let status = VerificationStatus::new(true, false, true, Some(123));
        let serialized = bincode::serialize(&status).unwrap();
        let deserialized: VerificationStatus = bincode::deserialize(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }
}
