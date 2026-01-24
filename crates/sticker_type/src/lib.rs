// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Sticker type enumeration for Telegram MTProto client.
//!
//! This module implements TDLib's StickerType.
//!
//! # Example
//!
//! ```rust
//! use rustgram_sticker_type::StickerType;
//!
//! let sticker_type = StickerType::Regular;
//! assert!(sticker_type.is_regular());
//! assert_eq!(sticker_type.name(), "regular");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt::{self, Display, Formatter};

/// Sticker type.
///
/// Based on TDLib's `StickerType` enum.
///
/// Represents the type of sticker in Telegram.
///
/// # Example
///
/// ```rust
/// use rustgram_sticker_type::StickerType;
///
/// let regular = StickerType::Regular;
/// assert!(regular.is_regular());
/// assert!(!regular.is_mask());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum StickerType {
    /// Regular sticker
    Regular = 0,

    /// Mask sticker
    Mask = 1,

    /// Custom emoji sticker
    CustomEmoji = 2,

    /// Unknown sticker type
    #[default]
    Unknown = 3,
}

impl StickerType {
    /// Creates a new StickerType from an i32 value.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value to convert
    ///
    /// # Returns
    ///
    /// Returns `Some(StickerType)` if the value is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert_eq!(StickerType::from_i32(0), Some(StickerType::Regular));
    /// assert_eq!(StickerType::from_i32(1), Some(StickerType::Mask));
    /// assert_eq!(StickerType::from_i32(99), None);
    /// ```
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Regular),
            1 => Some(Self::Mask),
            2 => Some(Self::CustomEmoji),
            3 => Some(Self::Unknown),
            _ => None,
        }
    }

    /// Returns the i32 representation of this sticker type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert_eq!(StickerType::Regular.to_i32(), 0);
    /// assert_eq!(StickerType::Mask.to_i32(), 1);
    /// ```
    pub fn to_i32(self) -> i32 {
        self as i32
    }

    /// Returns the name of this sticker type.
    ///
    /// # Returns
    ///
    /// * `"regular"` for regular stickers
    /// * `"mask"` for mask stickers
    /// * `"custom_emoji"` for custom emoji stickers
    /// * `"unknown"` for unknown types
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert_eq!(StickerType::Regular.name(), "regular");
    /// assert_eq!(StickerType::Mask.name(), "mask");
    /// assert_eq!(StickerType::CustomEmoji.name(), "custom_emoji");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::Mask => "mask",
            Self::CustomEmoji => "custom_emoji",
            Self::Unknown => "unknown",
        }
    }

    /// Checks if this is a regular sticker.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a regular sticker, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert!(StickerType::Regular.is_regular());
    /// assert!(!StickerType::Mask.is_regular());
    /// ```
    pub fn is_regular(self) -> bool {
        matches!(self, Self::Regular)
    }

    /// Checks if this is a mask sticker.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a mask sticker, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert!(StickerType::Mask.is_mask());
    /// assert!(!StickerType::Regular.is_mask());
    /// ```
    pub fn is_mask(self) -> bool {
        matches!(self, Self::Mask)
    }

    /// Checks if this is a custom emoji sticker.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a custom emoji sticker, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert!(StickerType::CustomEmoji.is_custom_emoji());
    /// assert!(!StickerType::Regular.is_custom_emoji());
    /// ```
    pub fn is_custom_emoji(self) -> bool {
        matches!(self, Self::CustomEmoji)
    }

    /// Checks if this sticker type is unknown.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is an unknown type, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert!(StickerType::Unknown.is_unknown());
    /// assert!(!StickerType::Regular.is_unknown());
    /// ```
    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Checks if this sticker type is supported.
    ///
    /// # Returns
    ///
    /// Returns `true` if the type is supported, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert!(StickerType::Regular.is_supported());
    /// assert!(!StickerType::Unknown.is_supported());
    /// ```
    pub fn is_supported(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Checks if this sticker type can be animated.
    ///
    /// # Returns
    ///
    /// Returns `true` if the type can be animated, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert!(StickerType::Regular.can_be_animated());
    /// assert!(!StickerType::Mask.can_be_animated());
    /// ```
    pub fn can_be_animated(self) -> bool {
        matches!(self, Self::Regular | Self::CustomEmoji)
    }

    /// Checks if this sticker type can be used in emoji status.
    ///
    /// # Returns
    ///
    /// Returns `true` if the type can be used in emoji status, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert!(StickerType::CustomEmoji.can_be_emoji_status());
    /// assert!(!StickerType::Mask.can_be_emoji_status());
    /// ```
    pub fn can_be_emoji_status(self) -> bool {
        matches!(self, Self::CustomEmoji)
    }

    /// Checks if this sticker type can have an associated emoji.
    ///
    /// # Returns
    ///
    /// Returns `true` if the type can have an emoji, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_type::StickerType;
    ///
    /// assert!(StickerType::Regular.has_emoji());
    /// assert!(StickerType::CustomEmoji.has_emoji());
    /// ```
    pub fn has_emoji(self) -> bool {
        matches!(self, Self::Regular | Self::CustomEmoji)
    }
}

impl Display for StickerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(StickerType::from_i32(0), Some(StickerType::Regular));
        assert_eq!(StickerType::from_i32(1), Some(StickerType::Mask));
        assert_eq!(StickerType::from_i32(2), Some(StickerType::CustomEmoji));
        assert_eq!(StickerType::from_i32(3), Some(StickerType::Unknown));
        assert_eq!(StickerType::from_i32(-1), None);
        assert_eq!(StickerType::from_i32(4), None);
        assert_eq!(StickerType::from_i32(99), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(StickerType::Regular.to_i32(), 0);
        assert_eq!(StickerType::Mask.to_i32(), 1);
        assert_eq!(StickerType::CustomEmoji.to_i32(), 2);
        assert_eq!(StickerType::Unknown.to_i32(), 3);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for value in 0i32..=3 {
            let sticker_type = StickerType::from_i32(value);
            assert_eq!(sticker_type.map(|st| st.to_i32()), Some(value));
        }
    }

    #[test]
    fn test_name() {
        assert_eq!(StickerType::Regular.name(), "regular");
        assert_eq!(StickerType::Mask.name(), "mask");
        assert_eq!(StickerType::CustomEmoji.name(), "custom_emoji");
        assert_eq!(StickerType::Unknown.name(), "unknown");
    }

    #[test]
    fn test_is_regular() {
        assert!(StickerType::Regular.is_regular());
        assert!(!StickerType::Mask.is_regular());
        assert!(!StickerType::CustomEmoji.is_regular());
        assert!(!StickerType::Unknown.is_regular());
    }

    #[test]
    fn test_is_mask() {
        assert!(!StickerType::Regular.is_mask());
        assert!(StickerType::Mask.is_mask());
        assert!(!StickerType::CustomEmoji.is_mask());
        assert!(!StickerType::Unknown.is_mask());
    }

    #[test]
    fn test_is_custom_emoji() {
        assert!(!StickerType::Regular.is_custom_emoji());
        assert!(!StickerType::Mask.is_custom_emoji());
        assert!(StickerType::CustomEmoji.is_custom_emoji());
        assert!(!StickerType::Unknown.is_custom_emoji());
    }

    #[test]
    fn test_is_unknown() {
        assert!(!StickerType::Regular.is_unknown());
        assert!(!StickerType::Mask.is_unknown());
        assert!(!StickerType::CustomEmoji.is_unknown());
        assert!(StickerType::Unknown.is_unknown());
    }

    #[test]
    fn test_is_supported() {
        assert!(StickerType::Regular.is_supported());
        assert!(StickerType::Mask.is_supported());
        assert!(StickerType::CustomEmoji.is_supported());
        assert!(!StickerType::Unknown.is_supported());
    }

    #[test]
    fn test_can_be_animated() {
        assert!(StickerType::Regular.can_be_animated());
        assert!(!StickerType::Mask.can_be_animated());
        assert!(StickerType::CustomEmoji.can_be_animated());
        assert!(!StickerType::Unknown.can_be_animated());
    }

    #[test]
    fn test_can_be_emoji_status() {
        assert!(!StickerType::Regular.can_be_emoji_status());
        assert!(!StickerType::Mask.can_be_emoji_status());
        assert!(StickerType::CustomEmoji.can_be_emoji_status());
        assert!(!StickerType::Unknown.can_be_emoji_status());
    }

    #[test]
    fn test_has_emoji() {
        assert!(StickerType::Regular.has_emoji());
        assert!(!StickerType::Mask.has_emoji());
        assert!(StickerType::CustomEmoji.has_emoji());
        assert!(!StickerType::Unknown.has_emoji());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", StickerType::Regular), "regular");
        assert_eq!(format!("{}", StickerType::Mask), "mask");
        assert_eq!(format!("{}", StickerType::CustomEmoji), "custom_emoji");
        assert_eq!(format!("{}", StickerType::Unknown), "unknown");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", StickerType::Regular), "Regular");
        assert_eq!(format!("{:?}", StickerType::Mask), "Mask");
        assert_eq!(format!("{:?}", StickerType::CustomEmoji), "CustomEmoji");
        assert_eq!(format!("{:?}", StickerType::Unknown), "Unknown");
    }

    #[test]
    fn test_default() {
        assert_eq!(StickerType::default(), StickerType::Unknown);
    }

    #[test]
    fn test_equality() {
        assert_eq!(StickerType::Regular, StickerType::Regular);
        assert_eq!(StickerType::Mask, StickerType::Mask);
        assert_eq!(StickerType::CustomEmoji, StickerType::CustomEmoji);
        assert_eq!(StickerType::Unknown, StickerType::Unknown);
    }

    #[test]
    fn test_inequality() {
        assert_ne!(StickerType::Regular, StickerType::Mask);
        assert_ne!(StickerType::Mask, StickerType::CustomEmoji);
        assert_ne!(StickerType::CustomEmoji, StickerType::Unknown);
    }

    #[test]
    fn test_copy() {
        let a = StickerType::Regular;
        let b = a;
        assert_eq!(a, StickerType::Regular);
        assert_eq!(b, StickerType::Regular);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(StickerType::Regular);
        set.insert(StickerType::Mask);
        set.insert(StickerType::CustomEmoji);
        set.insert(StickerType::Unknown);
        assert_eq!(set.len(), 4);
    }

    #[test]
    fn test_all_types_distinct() {
        let types = [
            StickerType::Regular,
            StickerType::Mask,
            StickerType::CustomEmoji,
            StickerType::Unknown,
        ];

        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j]);
            }
        }
    }

    #[test]
    fn test_regular_properties() {
        let regular = StickerType::Regular;
        assert!(regular.is_regular());
        assert!(regular.is_supported());
        assert!(regular.can_be_animated());
        assert!(!regular.can_be_emoji_status());
        assert!(regular.has_emoji());
    }

    #[test]
    fn test_mask_properties() {
        let mask = StickerType::Mask;
        assert!(mask.is_mask());
        assert!(mask.is_supported());
        assert!(!mask.can_be_animated());
        assert!(!mask.can_be_emoji_status());
        assert!(!mask.has_emoji());
    }

    #[test]
    fn test_custom_emoji_properties() {
        let custom_emoji = StickerType::CustomEmoji;
        assert!(custom_emoji.is_custom_emoji());
        assert!(custom_emoji.is_supported());
        assert!(custom_emoji.can_be_animated());
        assert!(custom_emoji.can_be_emoji_status());
        assert!(custom_emoji.has_emoji());
    }

    #[test]
    fn test_unknown_properties() {
        let unknown = StickerType::Unknown;
        assert!(unknown.is_unknown());
        assert!(!unknown.is_supported());
        assert!(!unknown.can_be_animated());
        assert!(!unknown.can_be_emoji_status());
        assert!(!unknown.has_emoji());
    }

    #[test]
    fn test_i32_values() {
        assert_eq!(StickerType::Regular.to_i32(), 0);
        assert_eq!(StickerType::Mask.to_i32(), 1);
        assert_eq!(StickerType::CustomEmoji.to_i32(), 2);
        assert_eq!(StickerType::Unknown.to_i32(), 3);
    }

    #[test]
    fn test_from_invalid_i32() {
        assert_eq!(StickerType::from_i32(-100), None);
        assert_eq!(StickerType::from_i32(100), None);
        assert_eq!(StickerType::from_i32(i32::MAX), None);
    }

    #[test]
    fn test_type_count() {
        let types = [
            StickerType::Regular,
            StickerType::Mask,
            StickerType::CustomEmoji,
            StickerType::Unknown,
        ];
        assert_eq!(types.len(), 4);
    }

    #[test]
    fn test_sticker_type_use_cases() {
        // Regular stickers can be sent in chats
        let regular = StickerType::Regular;
        assert!(regular.is_supported());

        // Mask stickers are for face filters
        let mask = StickerType::Mask;
        assert!(mask.is_mask());

        // Custom emoji can be used anywhere emoji are used
        let emoji = StickerType::CustomEmoji;
        assert!(emoji.can_be_emoji_status());
    }

    #[test]
    fn test_animation_capability() {
        let animatable = [StickerType::Regular, StickerType::CustomEmoji];
        let not_animatable = [StickerType::Mask, StickerType::Unknown];

        for sticker_type in animatable {
            assert!(sticker_type.can_be_animated());
        }

        for sticker_type in not_animatable {
            assert!(!sticker_type.can_be_animated());
        }
    }

    #[test]
    fn test_name_underscore_format() {
        assert_eq!(StickerType::CustomEmoji.name(), "custom_emoji");
        assert!(StickerType::CustomEmoji.name().contains('_'));
    }

    #[test]
    fn test_all_names_unique() {
        let names = [
            StickerType::Regular.name(),
            StickerType::Mask.name(),
            StickerType::CustomEmoji.name(),
            StickerType::Unknown.name(),
        ];

        let mut unique_names = std::collections::HashSet::new();
        for name in names {
            unique_names.insert(name);
        }

        assert_eq!(unique_names.len(), 4);
    }
}
