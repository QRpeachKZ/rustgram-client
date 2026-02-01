// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Sticker Set Info
//!
//! Basic information about a sticker set.
//!
//! Based on TDLib's `StickerSetInfo` from `td/telegram/td_api.tl`.
//!
//! # Overview
//!
//! A `StickerSetInfo` contains basic information about a sticker set,
//! including its ID, title, name, and various status flags.
//!
//! # Example
//!
//! ```rust
//! use rustgram_sticker_set_info::{StickerSetInfo, StickerType};
//! use rustgram_sticker_set_id::StickerSetId;
//!
//! let info = StickerSetInfo::new(
//!     StickerSetId::new(1234567890),
//!     "My Stickers".to_string(),
//!     "mystickers".to_string(),
//!     StickerType::Regular,
//! );
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_sticker_set_id::StickerSetId;
use rustgram_sticker_set_type::StickerType;
use serde::{Deserialize, Serialize};

/// Basic information about a sticker set.
///
/// Contains metadata about a sticker set including identification,
/// display information, and status flags.
///
/// # TDLib Mapping
///
/// This is a simplified version of TDLib's `stickerSetInfo`.
/// Fields that depend on unimplemented types are omitted.
///
/// # Example
///
/// ```
/// use rustgram_sticker_set_info::{StickerSetInfo, StickerType};
/// use rustgram_sticker_set_id::StickerSetId;
///
/// let info = StickerSetInfo::new(
///     StickerSetId::new(1234567890),
///     "My Stickers".to_string(),
///     "mystickers".to_string(),
///     StickerType::Regular,
/// );
///
/// assert_eq!(info.id().get(), 1234567890);
/// assert_eq!(info.title(), "My Stickers");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StickerSetInfo {
    /// Sticker set identifier
    id: StickerSetId,
    /// Title of the sticker set
    title: String,
    /// Name of the sticker set
    name: String,
    /// True, if the sticker set is owned by the current user
    is_owned: bool,
    /// True, if the sticker set is installed
    is_installed: bool,
    /// True, if the sticker set is archived
    is_archived: bool,
    /// True, if the sticker set is official
    is_official: bool,
    /// Type of stickers in the set
    sticker_type: StickerType,
    /// Total number of stickers in the set
    size: i32,
}

impl StickerSetInfo {
    /// Creates a new `StickerSetInfo`.
    ///
    /// # Arguments
    ///
    /// * `id` - Sticker set identifier
    /// * `title` - Title of the sticker set
    /// * `name` - Name of the sticker set
    /// * `sticker_type` - Type of stickers in the set
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_set_info::{StickerSetInfo, StickerType};
    /// use rustgram_sticker_set_id::StickerSetId;
    ///
    /// let info = StickerSetInfo::new(
    ///     StickerSetId::new(1234567890),
    ///     "My Stickers".to_string(),
    ///     "mystickers".to_string(),
    ///     StickerType::Regular,
    /// );
    /// ```
    #[must_use]
    pub fn new(id: StickerSetId, title: String, name: String, sticker_type: StickerType) -> Self {
        Self {
            id,
            title,
            name,
            is_owned: false,
            is_installed: false,
            is_archived: false,
            is_official: false,
            sticker_type,
            size: 0,
        }
    }

    /// Returns the sticker set ID.
    #[must_use]
    pub const fn id(&self) -> StickerSetId {
        self.id
    }

    /// Returns the title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true if the sticker set is owned by the current user.
    #[must_use]
    pub const fn is_owned(&self) -> bool {
        self.is_owned
    }

    /// Returns true if the sticker set is installed.
    #[must_use]
    pub const fn is_installed(&self) -> bool {
        self.is_installed
    }

    /// Returns true if the sticker set is archived.
    #[must_use]
    pub const fn is_archived(&self) -> bool {
        self.is_archived
    }

    /// Returns true if the sticker set is official.
    #[must_use]
    pub const fn is_official(&self) -> bool {
        self.is_official
    }

    /// Returns the sticker type.
    #[must_use]
    pub const fn sticker_type(&self) -> StickerType {
        self.sticker_type
    }

    /// Returns the total number of stickers in the set.
    #[must_use]
    pub const fn size(&self) -> i32 {
        self.size
    }

    /// Sets the owned flag.
    pub fn set_owned(&mut self, is_owned: bool) {
        self.is_owned = is_owned;
    }

    /// Sets the installed flag.
    pub fn set_installed(&mut self, is_installed: bool) {
        self.is_installed = is_installed;
    }

    /// Sets the archived flag.
    pub fn set_archived(&mut self, is_archived: bool) {
        self.is_archived = is_archived;
    }

    /// Sets the official flag.
    pub fn set_official(&mut self, is_official: bool) {
        self.is_official = is_official;
    }

    /// Sets the size.
    pub fn set_size(&mut self, size: i32) {
        self.size = size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info = StickerSetInfo::new(
            StickerSetId::new(1234567890),
            "My Stickers".to_string(),
            "mystickers".to_string(),
            StickerType::Regular,
        );

        assert_eq!(info.id().get(), 1234567890);
        assert_eq!(info.title(), "My Stickers");
        assert_eq!(info.name(), "mystickers");
        assert_eq!(info.sticker_type(), StickerType::Regular);
    }

    #[test]
    fn test_default_flags() {
        let info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        assert!(!info.is_owned());
        assert!(!info.is_installed());
        assert!(!info.is_archived());
        assert!(!info.is_official());
        assert_eq!(info.size(), 0);
    }

    #[test]
    fn test_set_owned() {
        let mut info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        assert!(!info.is_owned());
        info.set_owned(true);
        assert!(info.is_owned());
    }

    #[test]
    fn test_set_installed() {
        let mut info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        assert!(!info.is_installed());
        info.set_installed(true);
        assert!(info.is_installed());
    }

    #[test]
    fn test_set_archived() {
        let mut info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        assert!(!info.is_archived());
        info.set_archived(true);
        assert!(info.is_archived());
    }

    #[test]
    fn test_set_official() {
        let mut info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        assert!(!info.is_official());
        info.set_official(true);
        assert!(info.is_official());
    }

    #[test]
    fn test_set_size() {
        let mut info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        assert_eq!(info.size(), 0);
        info.set_size(42);
        assert_eq!(info.size(), 42);
    }

    #[test]
    fn test_with_mask_type() {
        let info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Masks".to_string(),
            "masks".to_string(),
            StickerType::Mask,
        );

        assert_eq!(info.sticker_type(), StickerType::Mask);
        assert!(info.sticker_type().is_mask());
    }

    #[test]
    fn test_with_custom_emoji_type() {
        let info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Emoji".to_string(),
            "emoji".to_string(),
            StickerType::CustomEmoji,
        );

        assert_eq!(info.sticker_type(), StickerType::CustomEmoji);
        assert!(info.sticker_type().is_custom_emoji());
    }

    #[test]
    fn test_equality() {
        let info1 = StickerSetInfo::new(
            StickerSetId::new(123),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        let info2 = StickerSetInfo::new(
            StickerSetId::new(123),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        let info3 = StickerSetInfo::new(
            StickerSetId::new(456),
            "Other".to_string(),
            "other".to_string(),
            StickerType::Mask,
        );

        assert_eq!(info1, info2);
        assert_ne!(info1, info3);
    }

    #[test]
    fn test_clone() {
        let info1 = StickerSetInfo::new(
            StickerSetId::new(123),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_serialization() {
        let info = StickerSetInfo::new(
            StickerSetId::new(1234567890),
            "My Stickers".to_string(),
            "mystickers".to_string(),
            StickerType::Regular,
        );

        let json = serde_json::to_string(&info).expect("Failed to serialize");
        assert!(json.contains("1234567890"));
        assert!(json.contains("My Stickers"));

        let deserialized: StickerSetInfo =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.id().get(), 1234567890);
        assert_eq!(deserialized.title(), "My Stickers");
    }

    #[test]
    fn test_multiple_flags() {
        let mut info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        info.set_owned(true);
        info.set_installed(true);
        info.set_official(true);

        assert!(info.is_owned());
        assert!(info.is_installed());
        assert!(!info.is_archived());
        assert!(info.is_official());
    }

    #[test]
    fn test_negative_size() {
        let mut info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        info.set_size(-1);
        assert_eq!(info.size(), -1);
    }

    #[test]
    fn test_large_size() {
        let mut info = StickerSetInfo::new(
            StickerSetId::new(1),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        info.set_size(i32::MAX);
        assert_eq!(info.size(), i32::MAX);
    }

    #[test]
    fn test_empty_strings() {
        let info = StickerSetInfo::new(
            StickerSetId::new(1),
            String::new(),
            String::new(),
            StickerType::Regular,
        );

        assert_eq!(info.title(), "");
        assert_eq!(info.name(), "");
    }

    #[test]
    fn test_zero_id() {
        let info = StickerSetInfo::new(
            StickerSetId::new(0),
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        assert_eq!(info.id().get(), 0);
        assert!(!info.id().is_valid());
    }
}
