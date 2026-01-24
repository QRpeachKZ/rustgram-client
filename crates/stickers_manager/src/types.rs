// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Sticker-related types for StickersManager.

use crate::error::CheckStickerSetNameResult;
use rustgram_dimensions::Dimensions;
use rustgram_file_id::FileId;
use rustgram_sticker_format::StickerFormat;
use rustgram_sticker_set_id::StickerSetId;
use rustgram_sticker_type::StickerType;
use std::fmt;

/// Maximum length for sticker set short name.
pub const MAX_STICKER_SET_SHORT_NAME_LENGTH: usize = 64;

/// Maximum number of stickers that can be returned in a search.
pub const MAX_FOUND_STICKERS: usize = 100;

/// Maximum number of custom emoji stickers to fetch at once.
pub const MAX_GET_CUSTOM_EMOJI_STICKERS: usize = 200;

/// Individual sticker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sticker {
    /// Sticker set identifier.
    pub set_id: StickerSetId,
    /// Alternative emoji representation.
    pub alt: String,
    /// Sticker dimensions.
    pub dimensions: Dimensions,
    /// Sticker file ID.
    pub file_id: FileId,
    /// Sticker format.
    pub format: StickerFormat,
    /// Sticker type.
    pub sticker_type: StickerType,
    /// Whether this is a premium sticker.
    pub is_premium: bool,
}

impl Sticker {
    /// Creates a new sticker.
    #[must_use]
    pub const fn new(
        set_id: StickerSetId,
        file_id: FileId,
        dimensions: Dimensions,
        format: StickerFormat,
        sticker_type: StickerType,
    ) -> Self {
        Self {
            set_id,
            alt: String::new(),
            dimensions,
            file_id,
            format,
            sticker_type,
            is_premium: false,
        }
    }

    /// Returns the sticker file ID.
    #[must_use]
    pub const fn file_id(&self) -> FileId {
        self.file_id
    }

    /// Returns the sticker set ID.
    #[must_use]
    pub const fn set_id(&self) -> StickerSetId {
        self.set_id
    }

    /// Returns the sticker format.
    #[must_use]
    pub const fn format(&self) -> StickerFormat {
        self.format
    }

    /// Returns the sticker type.
    #[must_use]
    pub const fn sticker_type(&self) -> StickerType {
        self.sticker_type
    }

    /// Returns whether this is a premium sticker.
    #[must_use]
    pub const fn is_premium(&self) -> bool {
        self.is_premium
    }
}

impl fmt::Display for Sticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sticker(set_id={}, file_id={}, format={}, type={:?})",
            self.set_id.get(),
            self.file_id.get(),
            self.format,
            self.sticker_type
        )
    }
}

/// Full sticker set information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StickerSet {
    /// Sticker set identifier.
    pub id: StickerSetId,
    /// Sticker set title.
    pub title: String,
    /// Sticker set short name.
    pub short_name: String,
    /// Stickers in the set.
    pub stickers: Vec<Sticker>,
    /// Sticker type.
    pub sticker_type: StickerType,
    /// Whether the set is installed.
    pub is_installed: bool,
    /// Whether the set is archived.
    pub is_archived: bool,
    /// Total number of stickers in the set.
    pub sticker_count: i32,
}

impl StickerSet {
    /// Creates a new sticker set.
    #[must_use]
    pub fn new(
        id: StickerSetId,
        title: String,
        short_name: String,
        sticker_type: StickerType,
    ) -> Self {
        Self {
            id,
            title,
            short_name,
            stickers: Vec::new(),
            sticker_type,
            is_installed: false,
            is_archived: false,
            sticker_count: 0,
        }
    }

    /// Returns the sticker set ID.
    #[must_use]
    pub const fn id(&self) -> StickerSetId {
        self.id
    }

    /// Returns the sticker set title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the sticker set short name.
    #[must_use]
    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    /// Returns the stickers in the set.
    #[must_use]
    pub fn stickers(&self) -> &[Sticker] {
        &self.stickers
    }

    /// Returns the sticker type.
    #[must_use]
    pub const fn sticker_type(&self) -> StickerType {
        self.sticker_type
    }

    /// Returns whether the set is installed.
    #[must_use]
    pub const fn is_installed(&self) -> bool {
        self.is_installed
    }

    /// Returns whether the set is archived.
    #[must_use]
    pub const fn is_archived(&self) -> bool {
        self.is_archived
    }

    /// Returns the total number of stickers.
    #[must_use]
    pub const fn sticker_count(&self) -> i32 {
        self.sticker_count
    }

    /// Validates the sticker set name.
    #[must_use]
    pub fn check_short_name(short_name: &str) -> CheckStickerSetNameResult {
        if short_name.len() > MAX_STICKER_SET_SHORT_NAME_LENGTH {
            return CheckStickerSetNameResult::TooLong;
        }
        if short_name.is_empty() {
            return CheckStickerSetNameResult::Invalid;
        }
        if !short_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return CheckStickerSetNameResult::InvalidCharacters;
        }
        CheckStickerSetNameResult::Ok
    }
}

impl fmt::Display for StickerSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StickerSet(id={}, name='{}', title='{}', count={})",
            self.id.get(),
            self.short_name,
            self.title,
            self.sticker_count
        )
    }
}

/// Input sticker for creating/updating stickers.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InputSticker {
    /// Placeholder for stub implementation.
    pub placeholder: String,
}

impl InputSticker {
    /// Creates a new input sticker (stub).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Input file reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputFile {
    /// Placeholder for stub implementation.
    pub id: i32,
}

impl InputFile {
    /// Creates a new input file (stub).
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self { id }
    }
}

/// Result containing stickers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stickers {
    /// List of stickers.
    pub stickers: Vec<Sticker>,
}

impl Stickers {
    /// Creates a new stickers result.
    #[must_use]
    pub fn new(stickers: Vec<Sticker>) -> Self {
        Self { stickers }
    }

    /// Returns the stickers.
    #[must_use]
    pub fn get_stickers(&self) -> &[Sticker] {
        &self.stickers
    }

    /// Returns the number of stickers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.stickers.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stickers.is_empty()
    }
}

/// Animated emoji data.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnimatedEmoji {
    /// The sticker.
    pub sticker: Option<Sticker>,
}

impl AnimatedEmoji {
    /// Creates a new animated emoji (stub).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Trending sticker sets result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrendingStickerSets {
    /// Total count of trending sets.
    pub total_count: i32,
    /// Offset for pagination.
    pub offset: String,
}

impl TrendingStickerSets {
    /// Creates a new trending sticker sets result.
    #[must_use]
    pub fn new(total_count: i32, offset: String) -> Self {
        Self {
            total_count,
            offset,
        }
    }

    /// Returns the total count.
    #[must_use]
    pub const fn total_count(&self) -> i32 {
        self.total_count
    }

    /// Returns the pagination offset.
    #[must_use]
    pub fn offset(&self) -> &str {
        &self.offset
    }
}

/// Mask position for mask stickers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaskPosition {
    /// X coordinate shift.
    pub x_shift: f32,
    /// Y coordinate shift.
    pub y_shift: f32,
    /// Scale.
    pub scale: f32,
}

impl MaskPosition {
    /// Creates a new mask position (stub).
    #[must_use]
    pub const fn new(x_shift: f32, y_shift: f32, scale: f32) -> Self {
        Self {
            x_shift,
            y_shift,
            scale,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sticker_new() {
        let set_id = StickerSetId::new(123);
        let file_id = FileId::new(456, 0);
        let dimensions = Dimensions::from_wh(512, 512);
        let sticker = Sticker::new(
            set_id,
            file_id,
            dimensions,
            StickerFormat::Webp,
            StickerType::Regular,
        );

        assert_eq!(sticker.set_id().get(), 123);
        assert_eq!(sticker.file_id().get(), 456);
        assert_eq!(sticker.format(), StickerFormat::Webp);
        assert_eq!(sticker.sticker_type(), StickerType::Regular);
        assert!(!sticker.is_premium());
    }

    #[test]
    fn test_sticker_set_new() {
        let id = StickerSetId::new(123);
        let set = StickerSet::new(
            id,
            "Test Set".to_string(),
            "testset".to_string(),
            StickerType::Regular,
        );

        assert_eq!(set.id().get(), 123);
        assert_eq!(set.title(), "Test Set");
        assert_eq!(set.short_name(), "testset");
        assert_eq!(set.sticker_type(), StickerType::Regular);
        assert!(set.stickers().is_empty());
        assert_eq!(set.sticker_count(), 0);
    }

    #[test]
    fn test_sticker_set_check_short_name_valid() {
        assert_eq!(
            StickerSet::check_short_name("valid_name"),
            CheckStickerSetNameResult::Ok
        );
    }

    #[test]
    fn test_sticker_set_check_short_name_too_long() {
        let long_name = "a".repeat(MAX_STICKER_SET_SHORT_NAME_LENGTH + 1);
        assert_eq!(
            StickerSet::check_short_name(&long_name),
            CheckStickerSetNameResult::TooLong
        );
    }

    #[test]
    fn test_stickers_new() {
        let stickers = Stickers::new(Vec::new());
        assert!(stickers.is_empty());
        assert_eq!(stickers.len(), 0);
    }

    #[test]
    fn test_constants() {
        assert_eq!(MAX_STICKER_SET_SHORT_NAME_LENGTH, 64);
        assert_eq!(MAX_FOUND_STICKERS, 100);
        assert_eq!(MAX_GET_CUSTOM_EMOJI_STICKERS, 200);
    }
}
