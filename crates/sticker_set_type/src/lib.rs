// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Sticker Type
//!
//! Type of sticker in a sticker set.
//!
//! Based on TDLib's `StickerType` from `td/telegram/td_api.tl`.
//!
//! # Overview
//!
//! A `StickerType` represents the type of stickers contained in a sticker set.
//! Telegram supports regular stickers, mask stickers, and custom emoji stickers.
//!
//! # Example
//!
//! ```rust
//! use rustgram_sticker_set_type::StickerType;
//!
//! let regular = StickerType::Regular;
//! assert!(regular.is_regular());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use serde::{Deserialize, Serialize};

/// Type of sticker in a sticker set.
///
/// Represents the type of stickers contained in a Telegram sticker set.
///
/// # TDLib Mapping
///
/// - `Regular` → TDLib: `stickerTypeRegular`
/// - `Mask` → TDLib: `stickerTypeMask`
/// - `CustomEmoji` → TDLib: `stickerTypeCustomEmoji`
///
/// # Example
///
/// ```
/// use rustgram_sticker_set_type::StickerType;
///
/// let regular = StickerType::Regular;
/// assert!(regular.is_regular());
/// assert!(!regular.is_mask());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum StickerType {
    /// Regular sticker that can be freely used in messages.
    ///
    /// TDLib: `stickerTypeRegular`
    #[default]
    Regular,

    /// Mask sticker that can be applied to photos and videos.
    ///
    /// TDLib: `stickerTypeMask`
    Mask,

    /// Custom emoji sticker that represents an emoji.
    ///
    /// TDLib: `stickerTypeCustomEmoji`
    CustomEmoji,
}

impl StickerType {
    /// Checks if this is a regular sticker.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_set_type::StickerType;
    ///
    /// assert!(StickerType::Regular.is_regular());
    /// assert!(!StickerType::Mask.is_regular());
    /// ```
    #[must_use]
    pub const fn is_regular(self) -> bool {
        matches!(self, Self::Regular)
    }

    /// Checks if this is a mask sticker.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_set_type::StickerType;
    ///
    /// assert!(StickerType::Mask.is_mask());
    /// assert!(!StickerType::Regular.is_mask());
    /// ```
    #[must_use]
    pub const fn is_mask(self) -> bool {
        matches!(self, Self::Mask)
    }

    /// Checks if this is a custom emoji sticker.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_set_type::StickerType;
    ///
    /// assert!(StickerType::CustomEmoji.is_custom_emoji());
    /// assert!(!StickerType::Regular.is_custom_emoji());
    /// ```
    #[must_use]
    pub const fn is_custom_emoji(self) -> bool {
        matches!(self, Self::CustomEmoji)
    }

    /// Returns the TDLib API name for this sticker type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_set_type::StickerType;
    ///
    /// assert_eq!(StickerType::Regular.as_td_name(), "stickerTypeRegular");
    /// assert_eq!(StickerType::Mask.as_td_name(), "stickerTypeMask");
    /// assert_eq!(StickerType::CustomEmoji.as_td_name(), "stickerTypeCustomEmoji");
    /// ```
    #[must_use]
    pub const fn as_td_name(self) -> &'static str {
        match self {
            Self::Regular => "stickerTypeRegular",
            Self::Mask => "stickerTypeMask",
            Self::CustomEmoji => "stickerTypeCustomEmoji",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular_variant() {
        let st = StickerType::Regular;
        assert!(st.is_regular());
        assert!(!st.is_mask());
        assert!(!st.is_custom_emoji());
    }

    #[test]
    fn test_mask_variant() {
        let st = StickerType::Mask;
        assert!(!st.is_regular());
        assert!(st.is_mask());
        assert!(!st.is_custom_emoji());
    }

    #[test]
    fn test_custom_emoji_variant() {
        let st = StickerType::CustomEmoji;
        assert!(!st.is_regular());
        assert!(!st.is_mask());
        assert!(st.is_custom_emoji());
    }

    #[test]
    fn test_as_td_name() {
        assert_eq!(StickerType::Regular.as_td_name(), "stickerTypeRegular");
        assert_eq!(StickerType::Mask.as_td_name(), "stickerTypeMask");
        assert_eq!(
            StickerType::CustomEmoji.as_td_name(),
            "stickerTypeCustomEmoji"
        );
    }

    #[test]
    fn test_default() {
        let st = StickerType::default();
        assert_eq!(st, StickerType::Regular);
    }

    #[test]
    fn test_equality() {
        assert_eq!(StickerType::Regular, StickerType::Regular);
        assert_ne!(StickerType::Regular, StickerType::Mask);
        assert_ne!(StickerType::Mask, StickerType::CustomEmoji);
        assert_ne!(StickerType::CustomEmoji, StickerType::Regular);
    }

    #[test]
    fn test_clone() {
        let st1 = StickerType::Mask;
        let st2 = st1.clone();
        assert_eq!(st1, st2);
    }

    #[test]
    fn test_copy() {
        let st1 = StickerType::CustomEmoji;
        let st2 = st1;
        assert_eq!(st1, StickerType::CustomEmoji);
        assert_eq!(st2, StickerType::CustomEmoji);
    }

    #[test]
    fn test_debug_format() {
        let st = StickerType::Regular;
        let debug = format!("{st:?}");
        assert!(debug.contains("Regular"));
    }

    #[test]
    fn test_serialization() {
        let st = StickerType::Mask;
        let json = serde_json::to_string(&st).expect("Failed to serialize");
        assert!(json.contains("Mask"));

        let deserialized: StickerType = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, st);
    }

    #[test]
    fn test_all_types_exhaustive() {
        // Ensure we test all variants
        let types = [
            StickerType::Regular,
            StickerType::Mask,
            StickerType::CustomEmoji,
        ];
        assert_eq!(types.len(), 3);

        for st in types {
            // Each type should be exactly one of the three
            let count = [st.is_regular(), st.is_mask(), st.is_custom_emoji()]
                .iter()
                .filter(|&&x| x)
                .count();
            assert_eq!(count, 1, "Each StickerType should match exactly one check");
        }
    }

    #[test]
    fn test_match_exhaustive() {
        // Ensure all variants are covered in match
        fn check(st: StickerType) -> &'static str {
            match st {
                StickerType::Regular => "regular",
                StickerType::Mask => "mask",
                StickerType::CustomEmoji => "custom_emoji",
            }
        }

        assert_eq!(check(StickerType::Regular), "regular");
        assert_eq!(check(StickerType::Mask), "mask");
        assert_eq!(check(StickerType::CustomEmoji), "custom_emoji");
    }
}
