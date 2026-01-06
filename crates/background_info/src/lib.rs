// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Background Info
//!
//! Information about chat backgrounds in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`BackgroundInfo`] struct and [`BackgroundType`] enum,
//! which represent information about chat backgrounds in Telegram. It includes
//! the background ID and type. It mirrors TDLib's `BackgroundInfo` class.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_background_info::{BackgroundInfo, BackgroundType};
//!
//! // Create background info
//! let info = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
//! assert!(info.is_valid());
//! assert_eq!(info.background_id(), 12345);
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type of background.
///
/// Represents the different types of backgrounds available in Telegram.
///
/// # Example
///
/// ```rust
/// use rustgram_background_info::BackgroundType;
///
/// let pattern = BackgroundType::Pattern;
/// let fill = BackgroundType::Fill;
/// let wallpaper = BackgroundType::Wallpaper;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BackgroundType {
    /// Pattern background with repeated elements.
    Pattern,

    /// Solid fill background.
    Fill,

    /// Full wallpaper image.
    #[default]
    Wallpaper,
}

impl fmt::Display for BackgroundType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pattern => write!(f, "Pattern"),
            Self::Fill => write!(f, "Fill"),
            Self::Wallpaper => write!(f, "Wallpaper"),
        }
    }
}

/// Information about a chat background.
///
/// This type provides background information including the unique
/// background ID and the type of background.
///
/// # Fields
///
/// - `background_id` - The unique background identifier
/// - `background_type` - The type of background
///
/// # Example
///
/// ```rust
/// use rustgram_background_info::{BackgroundInfo, BackgroundType};
///
/// // Create valid background info
/// let info = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
/// assert!(info.is_valid());
/// assert_eq!(info.background_id(), 12345);
///
/// // Create invalid background info (zero ID)
/// let invalid = BackgroundInfo::new(0, BackgroundType::Wallpaper);
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BackgroundInfo {
    /// The unique background identifier.
    background_id: i64,

    /// The type of background.
    background_type: BackgroundType,
}

impl BackgroundInfo {
    /// Creates new background info.
    ///
    /// # Arguments
    ///
    /// * `background_id` - The unique background identifier
    /// * `background_type` - The type of background
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let info = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
    /// assert_eq!(info.background_id(), 12345);
    /// ```
    #[must_use]
    pub const fn new(background_id: i64, background_type: BackgroundType) -> Self {
        Self {
            background_id,
            background_type,
        }
    }

    /// Returns the background ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let info = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
    /// assert_eq!(info.background_id(), 12345);
    /// ```
    #[must_use]
    pub const fn background_id(&self) -> i64 {
        self.background_id
    }

    /// Returns the background type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let info = BackgroundInfo::new(12345, BackgroundType::Pattern);
    /// assert_eq!(info.background_type(), BackgroundType::Pattern);
    /// ```
    #[must_use]
    pub const fn background_type(&self) -> BackgroundType {
        self.background_type
    }

    /// Returns both values as a tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let info = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
    /// assert_eq!(info.get(), (12345, BackgroundType::Wallpaper));
    /// ```
    #[must_use]
    pub const fn get(&self) -> (i64, BackgroundType) {
        (self.background_id, self.background_type)
    }

    /// Checks if this is valid background info.
    ///
    /// Valid background info must have a non-zero background_id.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// assert!(BackgroundInfo::new(12345, BackgroundType::Wallpaper).is_valid());
    /// assert!(!BackgroundInfo::new(0, BackgroundType::Wallpaper).is_valid());
    /// ```
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.background_id != 0
    }
}

impl Default for BackgroundInfo {
    /// Creates default background info with zero ID and Wallpaper type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let default = BackgroundInfo::default();
    /// assert_eq!(default.background_id(), 0);
    /// assert_eq!(default.background_type(), BackgroundType::Wallpaper);
    /// assert!(!default.is_valid());
    /// ```
    fn default() -> Self {
        Self {
            background_id: 0,
            background_type: BackgroundType::Wallpaper,
        }
    }
}

impl fmt::Display for BackgroundInfo {
    /// Formats the background info for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let info = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
    /// assert_eq!(format!("{}", info), "BackgroundInfo(id: 12345, type: Wallpaper)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BackgroundInfo(id: {}, type: {})",
            self.background_id, self.background_type
        )
    }
}

impl From<(i64, BackgroundType)> for BackgroundInfo {
    /// Creates background info from a tuple of (background_id, background_type).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let info: BackgroundInfo = (12345, BackgroundType::Pattern).into();
    /// assert_eq!(info.background_id(), 12345);
    /// assert_eq!(info.background_type(), BackgroundType::Pattern);
    /// ```
    fn from((background_id, background_type): (i64, BackgroundType)) -> Self {
        Self::new(background_id, background_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_type_display() {
        assert_eq!(format!("{}", BackgroundType::Pattern), "Pattern");
        assert_eq!(format!("{}", BackgroundType::Fill), "Fill");
        assert_eq!(format!("{}", BackgroundType::Wallpaper), "Wallpaper");
    }

    #[test]
    fn test_background_type_default() {
        assert_eq!(BackgroundType::default(), BackgroundType::Wallpaper);
    }

    #[test]
    fn test_new() {
        let info = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
        assert_eq!(info.background_id(), 12345);
        assert_eq!(info.background_type(), BackgroundType::Wallpaper);
    }

    #[test]
    fn test_background_id() {
        let info = BackgroundInfo::new(99999, BackgroundType::Pattern);
        assert_eq!(info.background_id(), 99999);
    }

    #[test]
    fn test_background_type_getter() {
        let info = BackgroundInfo::new(12345, BackgroundType::Fill);
        assert_eq!(info.background_type(), BackgroundType::Fill);
    }

    #[test]
    fn test_get() {
        let info = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
        assert_eq!(info.get(), (12345, BackgroundType::Wallpaper));
    }

    #[test]
    fn test_is_valid_true() {
        assert!(BackgroundInfo::new(1, BackgroundType::Wallpaper).is_valid());
        assert!(BackgroundInfo::new(12345, BackgroundType::Pattern).is_valid());
        assert!(BackgroundInfo::new(-1, BackgroundType::Fill).is_valid());
    }

    #[test]
    fn test_is_valid_false_zero() {
        assert!(!BackgroundInfo::new(0, BackgroundType::Wallpaper).is_valid());
    }

    #[test]
    fn test_default() {
        let default = BackgroundInfo::default();
        assert_eq!(default.background_id(), 0);
        assert_eq!(default.background_type(), BackgroundType::Wallpaper);
        assert!(!default.is_valid());
    }

    #[test]
    fn test_equality() {
        let info1 = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
        let info2 = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_inequality_different_id() {
        let info1 = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
        let info2 = BackgroundInfo::new(99999, BackgroundType::Wallpaper);
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_inequality_different_type() {
        let info1 = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
        let info2 = BackgroundInfo::new(12345, BackgroundType::Pattern);
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_clone_semantics() {
        let info1 = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_display_format() {
        let info = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
        assert_eq!(
            format!("{}", info),
            "BackgroundInfo(id: 12345, type: Wallpaper)"
        );

        let info2 = BackgroundInfo::new(999, BackgroundType::Pattern);
        assert_eq!(
            format!("{}", info2),
            "BackgroundInfo(id: 999, type: Pattern)"
        );
    }

    #[test]
    fn test_from_tuple() {
        let info: BackgroundInfo = (12345, BackgroundType::Fill).into();
        assert_eq!(info.background_id(), 12345);
        assert_eq!(info.background_type(), BackgroundType::Fill);
    }

    #[test]
    fn test_debug_format() {
        let info = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("BackgroundInfo"));
        assert!(debug_str.contains("12345"));
    }

    #[test]
    fn test_negative_id() {
        let info = BackgroundInfo::new(-12345, BackgroundType::Wallpaper);
        assert!(info.is_valid());
        assert_eq!(info.background_id(), -12345);
    }

    #[test]
    fn test_all_background_types() {
        let types = [
            BackgroundType::Pattern,
            BackgroundType::Fill,
            BackgroundType::Wallpaper,
        ];

        for bg_type in types {
            let info = BackgroundInfo::new(12345, bg_type);
            assert_eq!(info.background_type(), bg_type);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = BackgroundInfo::new(12345, BackgroundType::Wallpaper);

        // Test JSON serialization
        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(
            json,
            r#"{"background_id":12345,"background_type":"Wallpaper"}"#
        );

        let deserialized: BackgroundInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);

        // Test binary serialization
        let encoded = bincode::serialize(&original).unwrap();
        let decoded: BackgroundInfo = bincode::deserialize(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_pattern() {
        let original = BackgroundInfo::new(999, BackgroundType::Pattern);

        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(json, r#"{"background_id":999,"background_type":"Pattern"}"#);

        let deserialized: BackgroundInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_fill() {
        let original = BackgroundInfo::new(555, BackgroundType::Fill);

        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(json, r#"{"background_id":555,"background_type":"Fill"}"#);

        let deserialized: BackgroundInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_negative_id() {
        let original = BackgroundInfo::new(-12345, BackgroundType::Wallpaper);

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: BackgroundInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
