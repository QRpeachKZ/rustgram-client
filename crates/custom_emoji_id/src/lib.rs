// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Custom Emoji ID
//!
//! Custom emoji identifier for Telegram.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::fmt;

/// Custom emoji identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CustomEmojiId(i64);

impl CustomEmojiId {
    /// Creates a new CustomEmojiId.
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the inner ID value.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Checks if this is a valid custom emoji ID (non-zero).
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl fmt::Display for CustomEmojiId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "custom_emoji {}", self.0)
    }
}

impl From<i64> for CustomEmojiId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<CustomEmojiId> for i64 {
    fn from(id: CustomEmojiId) -> Self {
        id.0
    }
}

#[cfg(feature = "serde")]
mod serde_support {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for CustomEmojiId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for CustomEmojiId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            i64::deserialize(deserializer).map(CustomEmojiId)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let emoji_id = CustomEmojiId::new(1234567890);
        assert_eq!(emoji_id.get(), 1234567890);
    }

    #[test]
    fn test_is_valid() {
        let valid = CustomEmojiId::new(123);
        assert!(valid.is_valid());

        let invalid = CustomEmojiId::new(0);
        assert!(!invalid.is_valid());
    }
}
