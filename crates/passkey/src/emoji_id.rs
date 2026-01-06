// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

use crate::PasskeyError;
use std::fmt;

/// Custom emoji identifier.
///
/// Used to identify custom emoji stickers and icons in Telegram.
/// A value of `0` indicates no emoji is set.
///
/// # Examples
///
/// ```
/// use rustgram_passkey::CustomEmojiId;
///
/// // Create a custom emoji ID
/// let emoji = CustomEmojiId::new(54321);
/// assert!(emoji.is_valid());
/// assert_eq!(emoji.get(), 54321);
///
/// // Zero is invalid
/// let invalid = CustomEmojiId::new(0);
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CustomEmojiId(i64);

impl CustomEmojiId {
    /// Creates a new custom emoji ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The emoji identifier (0 means no emoji)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::CustomEmojiId;
    ///
    /// let emoji = CustomEmojiId::new(12345);
    /// ```
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns `true` if this is a valid custom emoji ID.
    ///
    /// An ID is valid if it is non-zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::CustomEmojiId;
    ///
    /// assert!(CustomEmojiId::new(12345).is_valid());
    /// assert!(!CustomEmojiId::new(0).is_valid());
    /// ```
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }

    /// Returns the raw emoji ID value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::CustomEmojiId;
    ///
    /// let emoji = CustomEmojiId::new(54321);
    /// assert_eq!(emoji.get(), 54321);
    /// ```
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl fmt::Display for CustomEmojiId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "custom_emoji_{}", self.0)
    }
}

impl TryFrom<i64> for CustomEmojiId {
    type Error = PasskeyError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let id = Self(value);
        if id.is_valid() {
            Ok(id)
        } else {
            Err(PasskeyError::InvalidCustomEmojiId(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let emoji = CustomEmojiId::new(12345);
        assert_eq!(emoji.get(), 12345);
    }

    #[test]
    fn test_is_valid() {
        assert!(CustomEmojiId::new(12345).is_valid());
        assert!(CustomEmojiId::new(-1).is_valid());
        assert!(!CustomEmojiId::new(0).is_valid());
    }

    #[test]
    fn test_get() {
        assert_eq!(CustomEmojiId::new(54321).get(), 54321);
        assert_eq!(CustomEmojiId::new(0).get(), 0);
    }

    #[test]
    fn test_default() {
        assert_eq!(CustomEmojiId::default().get(), 0);
        assert!(!CustomEmojiId::default().is_valid());
    }

    #[test]
    fn test_equality() {
        let a = CustomEmojiId::new(12345);
        let b = CustomEmojiId::new(12345);
        let c = CustomEmojiId::new(54321);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_display() {
        let emoji = CustomEmojiId::new(12345);
        assert_eq!(format!("{}", emoji), "custom_emoji_12345");
    }

    #[test]
    fn test_try_from_valid() {
        let id: i64 = 54321;
        let emoji = match CustomEmojiId::try_from(id) {
            Ok(e) => e,
            Err(e) => panic!("Valid ID should convert: {:?}", e),
        };
        assert_eq!(emoji.get(), 54321);
    }

    #[test]
    fn test_try_from_invalid() {
        let id: i64 = 0;
        let result = CustomEmojiId::try_from(id);
        assert!(matches!(result, Err(PasskeyError::InvalidCustomEmojiId(0))));
    }
}
