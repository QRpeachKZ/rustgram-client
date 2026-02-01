// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Sticker Photo Size
//!
//! Sticker photo size for Telegram.
//!
//! Based on TDLib's `StickerPhotoSize` from `td/telegram/StickerPhotoSize.h`.
//!
//! # Overview
//!
//! A `StickerPhotoSize` represents a sticker that can be used as a chat photo.
//!
//! # Example
//!
//! ```rust
//! use rustgram_sticker_photo_size::StickerPhotoSize;
//!
//! let size = StickerPhotoSize::new();
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_custom_emoji_id::CustomEmojiId;
use rustgram_sticker_set_id::StickerSetId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Sticker type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[repr(i32)]
enum Type {
    /// Custom emoji
    #[default]
    CustomEmoji = 0,
    /// Sticker
    Sticker = 1,
}

/// Sticker photo size.
///
/// Represents a sticker that can be used as a chat photo.
///
/// # TDLib Mapping
///
/// TDLib: `StickerPhotoSize`
///
/// # Example
///
/// ```rust
/// use rustgram_sticker_photo_size::StickerPhotoSize;
///
/// let size = StickerPhotoSize::new();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StickerPhotoSize {
    type_: Type,
    custom_emoji_id: Option<CustomEmojiId>,
    sticker_set_id: Option<StickerSetId>,
    sticker_id: Option<i64>,
    background_colors: Option<Vec<i32>>,
}

impl StickerPhotoSize {
    /// Creates a new sticker photo size.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_photo_size::StickerPhotoSize;
    ///
    /// let size = StickerPhotoSize::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if this is a custom emoji.
    #[must_use]
    pub fn is_custom_emoji(&self) -> bool {
        matches!(self.type_, Type::CustomEmoji)
    }

    /// Checks if this is a sticker.
    #[must_use]
    pub fn is_sticker(&self) -> bool {
        matches!(self.type_, Type::Sticker)
    }
}

impl fmt::Display for StickerPhotoSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_ {
            Type::CustomEmoji => write!(f, "CustomEmojiPhotoSize"),
            Type::Sticker => write!(f, "StickerPhotoSize"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let size = StickerPhotoSize::new();
        assert!(size.is_custom_emoji());
    }

    #[test]
    fn test_default() {
        let size = StickerPhotoSize::default();
        assert!(size.is_custom_emoji());
    }

    #[test]
    fn test_is_custom_emoji() {
        let size = StickerPhotoSize::new();
        assert!(size.is_custom_emoji());
    }

    #[test]
    fn test_is_sticker() {
        let size = StickerPhotoSize::new();
        assert!(!size.is_sticker());
    }

    #[test]
    fn test_display() {
        let size = StickerPhotoSize::new();
        assert!(format!("{size}").contains("CustomEmojiPhotoSize"));
    }

    #[test]
    fn test_equality() {
        let size1 = StickerPhotoSize::new();
        let size2 = StickerPhotoSize::new();
        assert_eq!(size1, size2);
    }

    #[test]
    fn test_clone() {
        let size1 = StickerPhotoSize::new();
        let size2 = size1.clone();
        assert_eq!(size1, size2);
    }

    #[test]
    fn test_serialization() {
        let size = StickerPhotoSize::new();
        let json = serde_json::to_string(&size).expect("Failed to serialize");
        let deserialized: StickerPhotoSize =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, size);
    }
}
