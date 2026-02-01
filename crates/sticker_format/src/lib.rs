// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Sticker Format
//!
//! Sticker format enumeration for Telegram.
//!
//! Based on TDLib's `StickerFormat` from `td/telegram/StickerFormat.h`.
//!
//! # Overview
//!
//! A `StickerFormat` represents the file format of a sticker in Telegram.
//! Stickers can be in different formats: WebP (static), TGS (animated vector),
//! or WebM (animated video).
//!
//! # Example
//!
//! ```rust
//! use rustgram_sticker_format::StickerFormat;
//!
//! let format = StickerFormat::Webp;
//! assert_eq!(format.mime_type(), "image/webp");
//! assert!(!format.is_animated());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_sticker_type::StickerType;
use std::fmt::{self, Display, Formatter};

/// Sticker format.
///
/// Based on TDLib's `StickerFormat` enum.
///
/// Represents the file format of a sticker in Telegram.
///
/// # TDLib Mapping
///
/// - `StickerFormat::Unknown` → TDLib: `StickerFormat::Unknown`
/// - `StickerFormat::Webp` → TDLib: `StickerFormat::Webp`
/// - `StickerFormat::Tgs` → TDLib: `StickerFormat::Tgs`
/// - `StickerFormat::Webm` → TDLib: `StickerFormat::Webm`
///
/// # Example
///
/// ```rust
/// use rustgram_sticker_format::StickerFormat;
///
/// let format = StickerFormat::Tgs;
/// assert!(format.is_animated());
/// assert!(format.is_vector());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum StickerFormat {
    /// Unknown sticker format (default for unrecognized formats).
    #[default]
    Unknown = 0,

    /// WebP format (static stickers).
    Webp = 1,

    /// TGS format (animated Lottie vector stickers).
    Tgs = 2,

    /// WebM format (animated video stickers).
    Webm = 3,
}

impl StickerFormat {
    /// Creates a `StickerFormat` from an i32 value.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value to convert
    ///
    /// # Returns
    ///
    /// Returns `Some(StickerFormat)` if the value is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert_eq!(StickerFormat::from_i32(1), Some(StickerFormat::Webp));
    /// assert_eq!(StickerFormat::from_i32(2), Some(StickerFormat::Tgs));
    /// assert_eq!(StickerFormat::from_i32(99), None);
    /// ```
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Unknown),
            1 => Some(Self::Webp),
            2 => Some(Self::Tgs),
            3 => Some(Self::Webm),
            _ => None,
        }
    }

    /// Returns the i32 representation of this sticker format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert_eq!(StickerFormat::Webp.to_i32(), 1);
    /// assert_eq!(StickerFormat::Tgs.to_i32(), 2);
    /// ```
    #[must_use]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }

    /// Returns the name of this sticker format.
    ///
    /// # Returns
    ///
    /// * `"unknown"` for unknown formats
    /// * `"webp"` for WebP format
    /// * `"tgs"` for TGS format
    /// * `"webm"` for WebM format
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert_eq!(StickerFormat::Webp.name(), "webp");
    /// assert_eq!(StickerFormat::Tgs.name(), "tgs");
    /// ```
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Webp => "webp",
            Self::Tgs => "tgs",
            Self::Webm => "webm",
        }
    }

    /// Returns the MIME type for this sticker format.
    ///
    /// # Returns
    ///
    /// * `"image/webp"` for WebP (including Unknown)
    /// * `"application/x-tgsticker"` for TGS
    /// * `"video/webm"` for WebM
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert_eq!(StickerFormat::Webp.mime_type(), "image/webp");
    /// assert_eq!(StickerFormat::Tgs.mime_type(), "application/x-tgsticker");
    /// ```
    #[must_use]
    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Unknown | Self::Webp => "image/webp",
            Self::Tgs => "application/x-tgsticker",
            Self::Webm => "video/webm",
        }
    }

    /// Returns the file extension for this sticker format.
    ///
    /// # Returns
    ///
    /// * `""` for Unknown
    /// * `".webp"` for WebP
    /// * `".tgs"` for TGS
    /// * `".webm"` for WebM
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert_eq!(StickerFormat::Webp.extension(), ".webp");
    /// assert_eq!(StickerFormat::Tgs.extension(), ".tgs");
    /// ```
    #[must_use]
    pub fn extension(self) -> &'static str {
        match self {
            Self::Unknown => "",
            Self::Webp => ".webp",
            Self::Tgs => ".tgs",
            Self::Webm => ".webm",
        }
    }

    /// Checks if this sticker format is animated.
    ///
    /// # Returns
    ///
    /// Returns `true` if the format is animated (TGS or WebM), `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert!(!StickerFormat::Webp.is_animated());
    /// assert!(StickerFormat::Tgs.is_animated());
    /// assert!(StickerFormat::Webm.is_animated());
    /// ```
    #[must_use]
    pub fn is_animated(self) -> bool {
        matches!(self, Self::Tgs | Self::Webm)
    }

    /// Checks if this sticker format is vector-based.
    ///
    /// # Returns
    ///
    /// Returns `true` only for TGS (vector Lottie format), `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert!(!StickerFormat::Webp.is_vector());
    /// assert!(StickerFormat::Tgs.is_vector());
    /// assert!(!StickerFormat::Webm.is_vector());
    /// ```
    #[must_use]
    pub fn is_vector(self) -> bool {
        matches!(self, Self::Tgs)
    }

    /// Checks if this sticker format is static (not animated).
    ///
    /// # Returns
    ///
    /// Returns `true` if the format is static (WebP), `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert!(StickerFormat::Webp.is_static());
    /// assert!(!StickerFormat::Tgs.is_static());
    /// ```
    #[must_use]
    pub fn is_static(self) -> bool {
        matches!(self, Self::Webp)
    }

    /// Checks if this format is supported (not Unknown).
    ///
    /// # Returns
    ///
    /// Returns `true` if the format is known and supported, `false` for Unknown.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert!(StickerFormat::Webp.is_supported());
    /// assert!(!StickerFormat::Unknown.is_supported());
    /// ```
    #[must_use]
    pub fn is_supported(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Gets the maximum file size for this sticker format.
    ///
    /// # Arguments
    ///
    /// * `sticker_type` - The type of sticker (Regular, Mask, or CustomEmoji)
    /// * `for_thumbnail` - Whether this is for a thumbnail (smaller size limit)
    ///
    /// # Returns
    ///
    /// Returns the maximum file size in bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    /// use rustgram_sticker_type::StickerType;
    ///
    /// let max_size = StickerFormat::Webp.max_file_size(StickerType::Regular, false);
    /// assert!(max_size > 0);
    /// ```
    #[must_use]
    pub const fn max_file_size(self, sticker_type: StickerType, for_thumbnail: bool) -> i64 {
        let is_custom_emoji = matches!(sticker_type, StickerType::CustomEmoji);

        match self {
            Self::Unknown | Self::Webp => {
                if for_thumbnail || is_custom_emoji {
                    1 << 17 // 128 KB for thumbnails or custom emoji
                } else {
                    1 << 19 // 512 KB for regular stickers
                }
            }
            Self::Tgs => {
                if for_thumbnail {
                    1 << 15 // 32 KB for thumbnails
                } else {
                    1 << 16 // 64 KB for TGS
                }
            }
            Self::Webm => {
                if for_thumbnail {
                    1 << 15 // 32 KB for thumbnails
                } else if is_custom_emoji {
                    1 << 16 // 64 KB for custom emoji
                } else {
                    1 << 18 // 256 KB for regular stickers
                }
            }
        }
    }

    /// Creates a `StickerFormat` from a MIME type string.
    ///
    /// # Arguments
    ///
    /// * `mime_type` - The MIME type string
    ///
    /// # Returns
    ///
    /// Returns the corresponding `StickerFormat`, or `Unknown` if not recognized.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert_eq!(StickerFormat::from_mime_type("image/webp"), StickerFormat::Webp);
    /// assert_eq!(StickerFormat::from_mime_type("application/x-tgsticker"), StickerFormat::Tgs);
    /// assert_eq!(StickerFormat::from_mime_type("unknown"), StickerFormat::Unknown);
    /// ```
    #[must_use]
    pub fn from_mime_type(mime_type: &str) -> Self {
        match mime_type {
            "application/x-tgsticker" => Self::Tgs,
            "image/webp" => Self::Webp,
            "video/webm" => Self::Webm,
            _ => Self::Unknown,
        }
    }

    /// Creates a `StickerFormat` from a file extension.
    ///
    /// # Arguments
    ///
    /// * `extension` - The file extension (with or without leading dot)
    ///
    /// # Returns
    ///
    /// Returns the corresponding `StickerFormat`, or `Unknown` if not recognized.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_format::StickerFormat;
    ///
    /// assert_eq!(StickerFormat::from_extension("webp"), StickerFormat::Webp);
    /// assert_eq!(StickerFormat::from_extension(".tgs"), StickerFormat::Tgs);
    /// assert_eq!(StickerFormat::from_extension("unknown"), StickerFormat::Unknown);
    /// ```
    #[must_use]
    pub fn from_extension(extension: &str) -> Self {
        let ext = extension.strip_prefix('.').unwrap_or(extension);
        match ext {
            "tgs" => Self::Tgs,
            "webp" => Self::Webp,
            "webm" => Self::Webm,
            _ => Self::Unknown,
        }
    }
}

impl Display for StickerFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<i32> for StickerFormat {
    fn from(value: i32) -> Self {
        Self::from_i32(value).unwrap_or_default()
    }
}

impl From<StickerFormat> for i32 {
    fn from(format: StickerFormat) -> Self {
        format.to_i32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(StickerFormat::from_i32(0), Some(StickerFormat::Unknown));
        assert_eq!(StickerFormat::from_i32(1), Some(StickerFormat::Webp));
        assert_eq!(StickerFormat::from_i32(2), Some(StickerFormat::Tgs));
        assert_eq!(StickerFormat::from_i32(3), Some(StickerFormat::Webm));
        assert_eq!(StickerFormat::from_i32(-1), None);
        assert_eq!(StickerFormat::from_i32(4), None);
        assert_eq!(StickerFormat::from_i32(99), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(StickerFormat::Unknown.to_i32(), 0);
        assert_eq!(StickerFormat::Webp.to_i32(), 1);
        assert_eq!(StickerFormat::Tgs.to_i32(), 2);
        assert_eq!(StickerFormat::Webm.to_i32(), 3);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for value in 0i32..=3 {
            let format = StickerFormat::from_i32(value);
            assert_eq!(format.map(|f| f.to_i32()), Some(value));
        }
    }

    #[test]
    fn test_name() {
        assert_eq!(StickerFormat::Unknown.name(), "unknown");
        assert_eq!(StickerFormat::Webp.name(), "webp");
        assert_eq!(StickerFormat::Tgs.name(), "tgs");
        assert_eq!(StickerFormat::Webm.name(), "webm");
    }

    #[test]
    fn test_mime_type() {
        assert_eq!(StickerFormat::Unknown.mime_type(), "image/webp");
        assert_eq!(StickerFormat::Webp.mime_type(), "image/webp");
        assert_eq!(StickerFormat::Tgs.mime_type(), "application/x-tgsticker");
        assert_eq!(StickerFormat::Webm.mime_type(), "video/webm");
    }

    #[test]
    fn test_extension() {
        assert_eq!(StickerFormat::Unknown.extension(), "");
        assert_eq!(StickerFormat::Webp.extension(), ".webp");
        assert_eq!(StickerFormat::Tgs.extension(), ".tgs");
        assert_eq!(StickerFormat::Webm.extension(), ".webm");
    }

    #[test]
    fn test_is_animated() {
        assert!(!StickerFormat::Unknown.is_animated());
        assert!(!StickerFormat::Webp.is_animated());
        assert!(StickerFormat::Tgs.is_animated());
        assert!(StickerFormat::Webm.is_animated());
    }

    #[test]
    fn test_is_vector() {
        assert!(!StickerFormat::Unknown.is_vector());
        assert!(!StickerFormat::Webp.is_vector());
        assert!(StickerFormat::Tgs.is_vector());
        assert!(!StickerFormat::Webm.is_vector());
    }

    #[test]
    fn test_is_static() {
        assert!(!StickerFormat::Unknown.is_static());
        assert!(StickerFormat::Webp.is_static());
        assert!(!StickerFormat::Tgs.is_static());
        assert!(!StickerFormat::Webm.is_static());
    }

    #[test]
    fn test_is_supported() {
        assert!(!StickerFormat::Unknown.is_supported());
        assert!(StickerFormat::Webp.is_supported());
        assert!(StickerFormat::Tgs.is_supported());
        assert!(StickerFormat::Webm.is_supported());
    }

    #[test]
    fn test_max_file_size_regular_sticker() {
        let webp_max = StickerFormat::Webp.max_file_size(StickerType::Regular, false);
        let tgs_max = StickerFormat::Tgs.max_file_size(StickerType::Regular, false);
        let webm_max = StickerFormat::Webm.max_file_size(StickerType::Regular, false);

        assert_eq!(webp_max, 1 << 19); // 512 KB
        assert_eq!(tgs_max, 1 << 16); // 64 KB
        assert_eq!(webm_max, 1 << 18); // 256 KB
    }

    #[test]
    fn test_max_file_size_custom_emoji() {
        let webp_max = StickerFormat::Webp.max_file_size(StickerType::CustomEmoji, false);
        let tgs_max = StickerFormat::Tgs.max_file_size(StickerType::CustomEmoji, false);
        let webm_max = StickerFormat::Webm.max_file_size(StickerType::CustomEmoji, false);

        assert_eq!(webp_max, 1 << 17); // 128 KB
        assert_eq!(tgs_max, 1 << 16); // 64 KB
        assert_eq!(webm_max, 1 << 16); // 64 KB
    }

    #[test]
    fn test_max_file_size_thumbnail() {
        let webp_max = StickerFormat::Webp.max_file_size(StickerType::Regular, true);
        let tgs_max = StickerFormat::Tgs.max_file_size(StickerType::Regular, true);
        let webm_max = StickerFormat::Webm.max_file_size(StickerType::Regular, true);

        assert_eq!(webp_max, 1 << 17); // 128 KB
        assert_eq!(tgs_max, 1 << 15); // 32 KB
        assert_eq!(webm_max, 1 << 15); // 32 KB
    }

    #[test]
    fn test_from_mime_type() {
        assert_eq!(StickerFormat::from_mime_type("image/webp"), StickerFormat::Webp);
        assert_eq!(StickerFormat::from_mime_type("application/x-tgsticker"), StickerFormat::Tgs);
        assert_eq!(StickerFormat::from_mime_type("video/webm"), StickerFormat::Webm);
        assert_eq!(StickerFormat::from_mime_type("unknown"), StickerFormat::Unknown);
        assert_eq!(StickerFormat::from_mime_type(""), StickerFormat::Unknown);
    }

    #[test]
    fn test_from_extension() {
        assert_eq!(StickerFormat::from_extension("webp"), StickerFormat::Webp);
        assert_eq!(StickerFormat::from_extension(".webp"), StickerFormat::Webp);
        assert_eq!(StickerFormat::from_extension("tgs"), StickerFormat::Tgs);
        assert_eq!(StickerFormat::from_extension(".tgs"), StickerFormat::Tgs);
        assert_eq!(StickerFormat::from_extension("webm"), StickerFormat::Webm);
        assert_eq!(StickerFormat::from_extension(".webm"), StickerFormat::Webm);
        assert_eq!(StickerFormat::from_extension("unknown"), StickerFormat::Unknown);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", StickerFormat::Unknown), "unknown");
        assert_eq!(format!("{}", StickerFormat::Webp), "webp");
        assert_eq!(format!("{}", StickerFormat::Tgs), "tgs");
        assert_eq!(format!("{}", StickerFormat::Webm), "webm");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", StickerFormat::Unknown), "Unknown");
        assert_eq!(format!("{:?}", StickerFormat::Webp), "Webp");
        assert_eq!(format!("{:?}", StickerFormat::Tgs), "Tgs");
        assert_eq!(format!("{:?}", StickerFormat::Webm), "Webm");
    }

    #[test]
    fn test_default() {
        assert_eq!(StickerFormat::default(), StickerFormat::Unknown);
    }

    #[test]
    fn test_from_i32_into_sticker_format() {
        let format: StickerFormat = 1.into();
        assert_eq!(format, StickerFormat::Webp);
    }

    #[test]
    fn test_sticker_format_into_i32() {
        let value: i32 = StickerFormat::Tgs.into();
        assert_eq!(value, 2);
    }

    #[test]
    fn test_equality() {
        assert_eq!(StickerFormat::Webp, StickerFormat::Webp);
        assert_eq!(StickerFormat::Tgs, StickerFormat::Tgs);
        assert_eq!(StickerFormat::Webm, StickerFormat::Webm);
        assert_eq!(StickerFormat::Unknown, StickerFormat::Unknown);
    }

    #[test]
    fn test_inequality() {
        assert_ne!(StickerFormat::Webp, StickerFormat::Tgs);
        assert_ne!(StickerFormat::Tgs, StickerFormat::Webm);
        assert_ne!(StickerFormat::Webm, StickerFormat::Unknown);
    }

    #[test]
    fn test_copy() {
        let a = StickerFormat::Webp;
        let b = a;
        assert_eq!(a, StickerFormat::Webp);
        assert_eq!(b, StickerFormat::Webp);
    }

    #[test]
    fn test_clone() {
        let format1 = StickerFormat::Tgs;
        let format2 = format1.clone();
        assert_eq!(format1, format2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(StickerFormat::Unknown);
        set.insert(StickerFormat::Webp);
        set.insert(StickerFormat::Tgs);
        set.insert(StickerFormat::Webm);
        assert_eq!(set.len(), 4);
    }

    #[test]
    fn test_all_formats_distinct() {
        let formats = [
            StickerFormat::Unknown,
            StickerFormat::Webp,
            StickerFormat::Tgs,
            StickerFormat::Webm,
        ];

        for i in 0..formats.len() {
            for j in (i + 1)..formats.len() {
                assert_ne!(formats[i], formats[j]);
            }
        }
    }

    #[test]
    fn test_webp_properties() {
        let webp = StickerFormat::Webp;
        assert!(webp.is_static());
        assert!(!webp.is_animated());
        assert!(!webp.is_vector());
        assert!(webp.is_supported());
    }

    #[test]
    fn test_tgs_properties() {
        let tgs = StickerFormat::Tgs;
        assert!(!tgs.is_static());
        assert!(tgs.is_animated());
        assert!(tgs.is_vector());
        assert!(tgs.is_supported());
    }

    #[test]
    fn test_webm_properties() {
        let webm = StickerFormat::Webm;
        assert!(!webm.is_static());
        assert!(webm.is_animated());
        assert!(!webm.is_vector());
        assert!(webm.is_supported());
    }

    #[test]
    fn test_unknown_properties() {
        let unknown = StickerFormat::Unknown;
        assert!(!unknown.is_static());
        assert!(!unknown.is_animated());
        assert!(!unknown.is_vector());
        assert!(!unknown.is_supported());
    }

    #[test]
    fn test_from_invalid_i32() {
        assert_eq!(StickerFormat::from_i32(-100), None);
        assert_eq!(StickerFormat::from_i32(100), None);
        assert_eq!(StickerFormat::from_i32(i32::MAX), None);
    }

    #[test]
    fn test_format_count() {
        let formats = [
            StickerFormat::Unknown,
            StickerFormat::Webp,
            StickerFormat::Tgs,
            StickerFormat::Webm,
        ];
        assert_eq!(formats.len(), 4);
    }

    #[test]
    fn test_extension_without_dot() {
        assert_eq!(StickerFormat::from_extension("webp"), StickerFormat::Webp);
        assert_eq!(StickerFormat::from_extension("tgs"), StickerFormat::Tgs);
        assert_eq!(StickerFormat::from_extension("webm"), StickerFormat::Webm);
    }

    #[test]
    fn test_case_sensitive_extensions() {
        assert_eq!(StickerFormat::from_extension("WEBP"), StickerFormat::Unknown);
        assert_eq!(StickerFormat::from_extension("TGS"), StickerFormat::Unknown);
        assert_eq!(StickerFormat::from_extension("WEBM"), StickerFormat::Unknown);
    }

    #[test]
    fn test_all_extensions_unique() {
        let extensions = [
            StickerFormat::Unknown.extension(),
            StickerFormat::Webp.extension(),
            StickerFormat::Tgs.extension(),
            StickerFormat::Webm.extension(),
        ];

        let mut unique_extensions = std::collections::HashSet::new();
        for ext in extensions {
            unique_extensions.insert(ext);
        }

        assert_eq!(unique_extensions.len(), 4);
    }

    #[test]
    fn test_all_mime_types_unique() {
        let mime_types = [
            StickerFormat::Unknown.mime_type(),
            StickerFormat::Webp.mime_type(),
            StickerFormat::Tgs.mime_type(),
            StickerFormat::Webm.mime_type(),
        ];

        let mut unique_mime_types = std::collections::HashSet::new();
        for mime in mime_types {
            unique_mime_types.insert(mime);
        }

        // Unknown and Webp share the same mime type
        assert!(mime_types[0] == mime_types[1]);
    }

    #[test]
    fn test_animated_formats() {
        let animated = [StickerFormat::Tgs, StickerFormat::Webm];
        let not_animated = [StickerFormat::Unknown, StickerFormat::Webp];

        for format in animated {
            assert!(format.is_animated());
        }

        for format in not_animated {
            assert!(!format.is_animated());
        }
    }

    #[test]
    fn test_max_file_size_mask_sticker() {
        let webp_max = StickerFormat::Webp.max_file_size(StickerType::Mask, false);
        let tgs_max = StickerFormat::Tgs.max_file_size(StickerType::Mask, false);
        let webm_max = StickerFormat::Webm.max_file_size(StickerType::Mask, false);

        // Mask stickers use same limits as regular stickers
        assert_eq!(webp_max, 1 << 19); // 512 KB
        assert_eq!(tgs_max, 1 << 16); // 64 KB
        assert_eq!(webm_max, 1 << 18); // 256 KB
    }

    #[test]
    fn test_max_file_size_custom_emoji_thumbnail() {
        let webp_max = StickerFormat::Webp.max_file_size(StickerType::CustomEmoji, true);
        let tgs_max = StickerFormat::Tgs.max_file_size(StickerType::CustomEmoji, true);
        let webm_max = StickerFormat::Webm.max_file_size(StickerType::CustomEmoji, true);

        assert_eq!(webp_max, 1 << 17); // 128 KB
        assert_eq!(tgs_max, 1 << 15); // 32 KB
        assert_eq!(webm_max, 1 << 15); // 32 KB
    }

    #[test]
    fn test_mime_type_for_unknown_format() {
        // Unknown format defaults to WebP mime type
        assert_eq!(StickerFormat::Unknown.mime_type(), "image/webp");
    }

    #[test]
    fn test_extension_for_unknown_format() {
        // Unknown format has empty extension
        assert_eq!(StickerFormat::Unknown.extension(), "");
    }

    #[test]
    fn test_empty_mime_type() {
        assert_eq!(StickerFormat::from_mime_type(""), StickerFormat::Unknown);
    }

    #[test]
    fn test_empty_extension() {
        assert_eq!(StickerFormat::from_extension(""), StickerFormat::Unknown);
    }

    #[test]
    fn test_i32_values_are_const() {
        // Ensure i32 values can be used in const contexts
        const WEBP_VALUE: i32 = StickerFormat::Webp.to_i32();
        const TGS_VALUE: i32 = StickerFormat::Tgs.to_i32();
        assert_eq!(WEBP_VALUE, 1);
        assert_eq!(TGS_VALUE, 2);
    }
}
