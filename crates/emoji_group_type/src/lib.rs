// rustgram_emoji_group_type
// Copyright (C) 2025 rustgram-client contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # Emoji Group Type
//!
//! Defines the type/category of emoji groups for organizing stickers and emoji.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_emoji_group_type::EmojiGroupType;
//!
//! let group_type = EmojiGroupType::EmojiStatus;
//! assert_eq!(group_type.display_name(), "EmojiStatus");
//! ```

use std::fmt;

/// Type of emoji group/category for organizing stickers and emoji.
///
/// This enum defines different categories for emoji groups in Telegram,
/// such as emoji status, profile photos, regular stickers, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum EmojiGroupType {
    /// Default/uncategorized emoji group
    #[default]
    Default = 0,
    /// Emoji status group
    EmojiStatus = 1,
    /// Profile photo emoji group (displayed as "ChatPhoto")
    ProfilePhoto = 2,
    /// Regular stickers group
    RegularStickers = 3,
}

impl EmojiGroupType {
    /// Maximum value for validation purposes
    pub const MAX: i32 = 4;

    /// Returns a human-readable display name for this emoji group type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_emoji_group_type::EmojiGroupType;
    ///
    /// assert_eq!(EmojiGroupType::Default.display_name(), "Default");
    /// assert_eq!(EmojiGroupType::ProfilePhoto.display_name(), "ChatPhoto");
    /// ```
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::EmojiStatus => "EmojiStatus",
            Self::ProfilePhoto => "ChatPhoto",
            Self::RegularStickers => "RegularStickers",
        }
    }

    /// Creates an `EmojiGroupType` from a TDLib API category type string.
    ///
    /// # Arguments
    ///
    /// * `category_type` - Optional string representing the TDLib API category type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_emoji_group_type::EmojiGroupType;
    ///
    /// assert_eq!(
    ///     EmojiGroupType::from_td_api_category_type(Some("emojiCategoryTypeDefault")),
    ///     EmojiGroupType::Default
    /// );
    /// assert_eq!(
    ///     EmojiGroupType::from_td_api_category_type(Some("emojiCategoryTypeEmojiStatus")),
    ///     EmojiGroupType::EmojiStatus
    /// );
    /// assert_eq!(
    ///     EmojiGroupType::from_td_api_category_type(Some("emojiCategoryTypeChatPhoto")),
    ///     EmojiGroupType::ProfilePhoto
    /// );
    /// assert_eq!(
    ///     EmojiGroupType::from_td_api_category_type(None),
    ///     EmojiGroupType::Default
    /// );
    /// ```
    #[must_use]
    pub fn from_td_api_category_type(category_type: Option<&str>) -> Self {
        match category_type {
            Some("emojiCategoryTypeDefault") => Self::Default,
            Some("emojiCategoryTypeEmojiStatus") => Self::EmojiStatus,
            Some("emojiCategoryTypeChatPhoto") => Self::ProfilePhoto,
            Some("emojiCategoryTypeRegularStickers") => Self::RegularStickers,
            _ => Self::Default,
        }
    }

    /// Creates an `EmojiGroupType` from an i32 value.
    ///
    /// Returns `None` if the value is out of range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_emoji_group_type::EmojiGroupType;
    ///
    /// assert_eq!(EmojiGroupType::from_i32(0), Some(EmojiGroupType::Default));
    /// assert_eq!(EmojiGroupType::from_i32(3), Some(EmojiGroupType::RegularStickers));
    /// assert_eq!(EmojiGroupType::from_i32(4), None);
    /// ```
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Default),
            1 => Some(Self::EmojiStatus),
            2 => Some(Self::ProfilePhoto),
            3 => Some(Self::RegularStickers),
            _ => None,
        }
    }

    /// Returns the underlying i32 value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_emoji_group_type::EmojiGroupType;
    ///
    /// assert_eq!(EmojiGroupType::Default.to_i32(), 0);
    /// assert_eq!(EmojiGroupType::EmojiStatus.to_i32(), 1);
    /// ```
    #[must_use]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for EmojiGroupType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(EmojiGroupType::default(), EmojiGroupType::Default);
    }

    #[test]
    fn test_display_names() {
        assert_eq!(EmojiGroupType::Default.display_name(), "Default");
        assert_eq!(EmojiGroupType::EmojiStatus.display_name(), "EmojiStatus");
        assert_eq!(EmojiGroupType::ProfilePhoto.display_name(), "ChatPhoto");
        assert_eq!(
            EmojiGroupType::RegularStickers.display_name(),
            "RegularStickers"
        );
    }

    #[test]
    fn test_from_td_api_category_type() {
        assert_eq!(
            EmojiGroupType::from_td_api_category_type(Some("emojiCategoryTypeDefault")),
            EmojiGroupType::Default
        );
        assert_eq!(
            EmojiGroupType::from_td_api_category_type(Some("emojiCategoryTypeEmojiStatus")),
            EmojiGroupType::EmojiStatus
        );
        assert_eq!(
            EmojiGroupType::from_td_api_category_type(Some("emojiCategoryTypeChatPhoto")),
            EmojiGroupType::ProfilePhoto
        );
        assert_eq!(
            EmojiGroupType::from_td_api_category_type(Some("emojiCategoryTypeRegularStickers")),
            EmojiGroupType::RegularStickers
        );
        assert_eq!(
            EmojiGroupType::from_td_api_category_type(None),
            EmojiGroupType::Default
        );
        assert_eq!(
            EmojiGroupType::from_td_api_category_type(Some("unknown")),
            EmojiGroupType::Default
        );
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(EmojiGroupType::from_i32(0), Some(EmojiGroupType::Default));
        assert_eq!(
            EmojiGroupType::from_i32(1),
            Some(EmojiGroupType::EmojiStatus)
        );
        assert_eq!(
            EmojiGroupType::from_i32(2),
            Some(EmojiGroupType::ProfilePhoto)
        );
        assert_eq!(
            EmojiGroupType::from_i32(3),
            Some(EmojiGroupType::RegularStickers)
        );
        assert_eq!(EmojiGroupType::from_i32(4), None);
        assert_eq!(EmojiGroupType::from_i32(-1), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(EmojiGroupType::Default.to_i32(), 0);
        assert_eq!(EmojiGroupType::EmojiStatus.to_i32(), 1);
        assert_eq!(EmojiGroupType::ProfilePhoto.to_i32(), 2);
        assert_eq!(EmojiGroupType::RegularStickers.to_i32(), 3);
    }

    #[test]
    fn test_roundtrip_i32() {
        for value in 0i32..4 {
            let group_type = EmojiGroupType::from_i32(value);
            assert_eq!(group_type.map(|gt| gt.to_i32()), Some(value));
        }
    }

    #[test]
    fn test_equality() {
        assert_eq!(EmojiGroupType::Default, EmojiGroupType::Default);
        assert_eq!(EmojiGroupType::EmojiStatus, EmojiGroupType::EmojiStatus);
        assert_ne!(EmojiGroupType::Default, EmojiGroupType::EmojiStatus);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", EmojiGroupType::Default), "Default");
        assert_eq!(format!("{}", EmojiGroupType::ProfilePhoto), "ChatPhoto");
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(EmojiGroupType::Default);
        set.insert(EmojiGroupType::EmojiStatus);
        set.insert(EmojiGroupType::ProfilePhoto);
        set.insert(EmojiGroupType::RegularStickers);
        assert_eq!(set.len(), 4);
    }
}
