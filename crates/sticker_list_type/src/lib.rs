// rustgram_sticker_list_type
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

//! # Sticker List Type
//!
//! Defines the type of sticker list for custom emoji identifiers.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_sticker_list_type::StickerListType;
//!
//! let list_type = StickerListType::DialogPhoto;
//! assert_eq!(list_type.database_key(), "default_dialog_photo_custom_emoji_ids");
//! assert_eq!(list_type.display_name(), "default chat photo custom emoji identifiers");
//! ```

use std::fmt;

/// Type of sticker list for custom emoji identifiers.
///
/// This enum defines different contexts where custom emoji identifiers can be used,
/// such as dialog photos, user profile photos, backgrounds, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum StickerListType {
    /// Custom emoji for default dialog/chat photos
    #[default]
    DialogPhoto = 0,
    /// Custom emoji for default user profile photos
    UserProfilePhoto = 1,
    /// Custom emoji for default backgrounds
    Background = 2,
    /// Custom emoji that are disallowed for channel emoji status
    DisallowedChannelEmojiStatus = 3,
}

impl StickerListType {
    /// Maximum value for validation purposes
    pub const MAX: i32 = 4;

    /// Returns the database key for storing custom emoji IDs of this type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_sticker_list_type::StickerListType;
    ///
    /// assert_eq!(
    ///     StickerListType::DialogPhoto.database_key(),
    ///     "default_dialog_photo_custom_emoji_ids"
    /// );
    /// ```
    #[must_use]
    pub const fn database_key(self) -> &'static str {
        match self {
            Self::DialogPhoto => "default_dialog_photo_custom_emoji_ids",
            Self::UserProfilePhoto => "default_profile_photo_custom_emoji_ids",
            Self::Background => "default_background_custom_emoji_ids",
            Self::DisallowedChannelEmojiStatus => {
                "disallowed_channel_emoji_status_custom_emoji_ids"
            }
        }
    }

    /// Returns a human-readable display name for this sticker list type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_sticker_list_type::StickerListType;
    ///
    /// assert_eq!(
    ///     StickerListType::Background.display_name(),
    ///     "default background custom emoji identifiers"
    /// );
    /// ```
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::DialogPhoto => "default chat photo custom emoji identifiers",
            Self::UserProfilePhoto => "default user profile photo custom emoji identifiers",
            Self::Background => "default background custom emoji identifiers",
            Self::DisallowedChannelEmojiStatus => {
                "disallowed chat emoji status custom emoji identifiers"
            }
        }
    }

    /// Creates a `StickerListType` from an i32 value.
    ///
    /// Returns `None` if the value is out of range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_sticker_list_type::StickerListType;
    ///
    /// assert_eq!(StickerListType::from_i32(0), Some(StickerListType::DialogPhoto));
    /// assert_eq!(StickerListType::from_i32(3), Some(StickerListType::DisallowedChannelEmojiStatus));
    /// assert_eq!(StickerListType::from_i32(4), None);
    /// assert_eq!(StickerListType::from_i32(-1), None);
    /// ```
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::DialogPhoto),
            1 => Some(Self::UserProfilePhoto),
            2 => Some(Self::Background),
            3 => Some(Self::DisallowedChannelEmojiStatus),
            _ => None,
        }
    }

    /// Returns the underlying i32 value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_sticker_list_type::StickerListType;
    ///
    /// assert_eq!(StickerListType::DialogPhoto.to_i32(), 0);
    /// assert_eq!(StickerListType::UserProfilePhoto.to_i32(), 1);
    /// ```
    #[must_use]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for StickerListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(StickerListType::default(), StickerListType::DialogPhoto);
    }

    #[test]
    fn test_database_keys() {
        assert_eq!(
            StickerListType::DialogPhoto.database_key(),
            "default_dialog_photo_custom_emoji_ids"
        );
        assert_eq!(
            StickerListType::UserProfilePhoto.database_key(),
            "default_profile_photo_custom_emoji_ids"
        );
        assert_eq!(
            StickerListType::Background.database_key(),
            "default_background_custom_emoji_ids"
        );
        assert_eq!(
            StickerListType::DisallowedChannelEmojiStatus.database_key(),
            "disallowed_channel_emoji_status_custom_emoji_ids"
        );
    }

    #[test]
    fn test_display_names() {
        assert_eq!(
            StickerListType::DialogPhoto.display_name(),
            "default chat photo custom emoji identifiers"
        );
        assert_eq!(
            StickerListType::UserProfilePhoto.display_name(),
            "default user profile photo custom emoji identifiers"
        );
        assert_eq!(
            StickerListType::Background.display_name(),
            "default background custom emoji identifiers"
        );
        assert_eq!(
            StickerListType::DisallowedChannelEmojiStatus.display_name(),
            "disallowed chat emoji status custom emoji identifiers"
        );
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            StickerListType::from_i32(0),
            Some(StickerListType::DialogPhoto)
        );
        assert_eq!(
            StickerListType::from_i32(1),
            Some(StickerListType::UserProfilePhoto)
        );
        assert_eq!(
            StickerListType::from_i32(2),
            Some(StickerListType::Background)
        );
        assert_eq!(
            StickerListType::from_i32(3),
            Some(StickerListType::DisallowedChannelEmojiStatus)
        );
        assert_eq!(StickerListType::from_i32(4), None);
        assert_eq!(StickerListType::from_i32(-1), None);
        assert_eq!(StickerListType::from_i32(999), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(StickerListType::DialogPhoto.to_i32(), 0);
        assert_eq!(StickerListType::UserProfilePhoto.to_i32(), 1);
        assert_eq!(StickerListType::Background.to_i32(), 2);
        assert_eq!(StickerListType::DisallowedChannelEmojiStatus.to_i32(), 3);
    }

    #[test]
    fn test_roundtrip_i32() {
        for value in 0i32..4 {
            let list_type = StickerListType::from_i32(value);
            assert_eq!(list_type.map(|lt| lt.to_i32()), Some(value));
        }
    }

    #[test]
    fn test_equality() {
        assert_eq!(StickerListType::DialogPhoto, StickerListType::DialogPhoto);
        assert_eq!(
            StickerListType::UserProfilePhoto,
            StickerListType::UserProfilePhoto
        );
        assert_ne!(StickerListType::DialogPhoto, StickerListType::Background);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", StickerListType::DialogPhoto),
            "default chat photo custom emoji identifiers"
        );
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(StickerListType::DialogPhoto);
        set.insert(StickerListType::UserProfilePhoto);
        set.insert(StickerListType::Background);
        set.insert(StickerListType::DisallowedChannelEmojiStatus);
        assert_eq!(set.len(), 4);
    }
}
