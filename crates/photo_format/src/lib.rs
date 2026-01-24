//! # Photo Format
//!
//! Photo format enumeration for different image formats.
//!
//! ## TDLib Reference
//!
//! - TDLib header: `td/telegram/PhotoFormat.h`
//! - TDLib enum: `PhotoFormat`
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_photo_format::PhotoFormat;
//!
//! let format = PhotoFormat::Jpeg;
//! ```

use core::fmt;

/// Photo format enumeration.
///
/// TDLib: `enum class PhotoFormat : int32`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
#[non_exhaustive]
pub enum PhotoFormat {
    /// JPEG format
    Jpeg = 0,
    /// PNG format
    #[default]
    Png = 1,
    /// WebP format
    Webp = 2,
    /// GIF format (animated)
    Gif = 3,
    /// TGS format (Telegram Stickers, Lottie JSON)
    Tgs = 4,
    /// MPEG-4 format (video)
    Mpeg4 = 5,
    /// WebM format (video)
    Webm = 6,
}

impl PhotoFormat {
    /// Get the i32 representation of this format.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_format::PhotoFormat;
    ///
    /// assert_eq!(PhotoFormat::Jpeg.as_i32(), 0);
    /// assert_eq!(PhotoFormat::Png.as_i32(), 1);
    /// ```
    #[inline]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Create a PhotoFormat from an i32 value.
    ///
    /// Returns `None` if the value is not a valid format.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_format::PhotoFormat;
    ///
    /// assert_eq!(PhotoFormat::from_i32(0), Some(PhotoFormat::Jpeg));
    /// assert_eq!(PhotoFormat::from_i32(1), Some(PhotoFormat::Png));
    /// assert_eq!(PhotoFormat::from_i32(99), None);
    /// ```
    #[inline]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(PhotoFormat::Jpeg),
            1 => Some(PhotoFormat::Png),
            2 => Some(PhotoFormat::Webp),
            3 => Some(PhotoFormat::Gif),
            4 => Some(PhotoFormat::Tgs),
            5 => Some(PhotoFormat::Mpeg4),
            6 => Some(PhotoFormat::Webm),
            _ => None,
        }
    }

    /// Check if this format is animated.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_format::PhotoFormat;
    ///
    /// assert!(!PhotoFormat::Jpeg.is_animated());
    /// assert!(PhotoFormat::Gif.is_animated());
    /// assert!(PhotoFormat::Tgs.is_animated());
    /// ```
    #[inline]
    pub const fn is_animated(self) -> bool {
        matches!(self, PhotoFormat::Gif | PhotoFormat::Tgs)
    }

    /// Check if this format is a video format.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_format::PhotoFormat;
    ///
    /// assert!(!PhotoFormat::Jpeg.is_video());
    /// assert!(PhotoFormat::Mpeg4.is_video());
    /// assert!(PhotoFormat::Webm.is_video());
    /// ```
    #[inline]
    pub const fn is_video(self) -> bool {
        matches!(self, PhotoFormat::Mpeg4 | PhotoFormat::Webm)
    }

    /// Get the file extension for this format.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_format::PhotoFormat;
    ///
    /// assert_eq!(PhotoFormat::Jpeg.extension(), "jpg");
    /// assert_eq!(PhotoFormat::Png.extension(), "png");
    /// assert_eq!(PhotoFormat::Tgs.extension(), "tgs");
    /// ```
    #[inline]
    pub const fn extension(self) -> &'static str {
        match self {
            PhotoFormat::Jpeg => "jpg",
            PhotoFormat::Png => "png",
            PhotoFormat::Webp => "webp",
            PhotoFormat::Gif => "gif",
            PhotoFormat::Tgs => "tgs",
            PhotoFormat::Mpeg4 => "mp4",
            PhotoFormat::Webm => "webm",
        }
    }

    /// Get the MIME type for this format.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_format::PhotoFormat;
    ///
    /// assert_eq!(PhotoFormat::Jpeg.mime_type(), "image/jpeg");
    /// assert_eq!(PhotoFormat::Png.mime_type(), "image/png");
    /// ```
    #[inline]
    pub const fn mime_type(self) -> &'static str {
        match self {
            PhotoFormat::Jpeg => "image/jpeg",
            PhotoFormat::Png => "image/png",
            PhotoFormat::Webp => "image/webp",
            PhotoFormat::Gif => "image/gif",
            PhotoFormat::Tgs => "application/x-tgsticker",
            PhotoFormat::Mpeg4 => "video/mp4",
            PhotoFormat::Webm => "video/webm",
        }
    }
}

impl fmt::Display for PhotoFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhotoFormat::Jpeg => write!(f, "JPEG"),
            PhotoFormat::Png => write!(f, "PNG"),
            PhotoFormat::Webp => write!(f, "WebP"),
            PhotoFormat::Gif => write!(f, "GIF"),
            PhotoFormat::Tgs => write!(f, "TGS"),
            PhotoFormat::Mpeg4 => write!(f, "MPEG-4"),
            PhotoFormat::Webm => write!(f, "WebM"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10 tests)
    #[test]
    fn test_clone() {
        let a = PhotoFormat::Jpeg;
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_copy() {
        let a = PhotoFormat::Png;
        let b = a;
        assert_eq!(a, PhotoFormat::Png);
        assert_eq!(b, PhotoFormat::Png);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(PhotoFormat::Jpeg, PhotoFormat::Jpeg);
        assert_ne!(PhotoFormat::Jpeg, PhotoFormat::Png);
    }

    #[test]
    fn test_default() {
        assert_eq!(PhotoFormat::default(), PhotoFormat::Png);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        PhotoFormat::Jpeg.hash(&mut hasher);
        let h1 = hasher.finish();

        hasher = DefaultHasher::new();
        PhotoFormat::Jpeg.hash(&mut hasher);
        let h2 = hasher.finish();

        assert_eq!(h1, h2);
    }

    // Constructor tests (2 tests)
    #[test]
    fn test_from_i32_valid() {
        assert_eq!(PhotoFormat::from_i32(0), Some(PhotoFormat::Jpeg));
        assert_eq!(PhotoFormat::from_i32(1), Some(PhotoFormat::Png));
        assert_eq!(PhotoFormat::from_i32(2), Some(PhotoFormat::Webp));
        assert_eq!(PhotoFormat::from_i32(3), Some(PhotoFormat::Gif));
        assert_eq!(PhotoFormat::from_i32(4), Some(PhotoFormat::Tgs));
        assert_eq!(PhotoFormat::from_i32(5), Some(PhotoFormat::Mpeg4));
        assert_eq!(PhotoFormat::from_i32(6), Some(PhotoFormat::Webm));
    }

    #[test]
    fn test_from_i32_invalid() {
        assert_eq!(PhotoFormat::from_i32(-1), None);
        assert_eq!(PhotoFormat::from_i32(7), None);
        assert_eq!(PhotoFormat::from_i32(99), None);
    }

    // Method tests (12 tests)
    #[test]
    fn test_as_i32() {
        assert_eq!(PhotoFormat::Jpeg.as_i32(), 0);
        assert_eq!(PhotoFormat::Png.as_i32(), 1);
        assert_eq!(PhotoFormat::Webp.as_i32(), 2);
        assert_eq!(PhotoFormat::Gif.as_i32(), 3);
        assert_eq!(PhotoFormat::Tgs.as_i32(), 4);
        assert_eq!(PhotoFormat::Mpeg4.as_i32(), 5);
        assert_eq!(PhotoFormat::Webm.as_i32(), 6);
    }

    #[test]
    fn test_is_animated() {
        assert!(!PhotoFormat::Jpeg.is_animated());
        assert!(!PhotoFormat::Png.is_animated());
        assert!(!PhotoFormat::Webp.is_animated());
        assert!(PhotoFormat::Gif.is_animated());
        assert!(PhotoFormat::Tgs.is_animated());
        assert!(!PhotoFormat::Mpeg4.is_animated());
        assert!(!PhotoFormat::Webm.is_animated());
    }

    #[test]
    fn test_is_video() {
        assert!(!PhotoFormat::Jpeg.is_video());
        assert!(!PhotoFormat::Png.is_video());
        assert!(!PhotoFormat::Webp.is_video());
        assert!(!PhotoFormat::Gif.is_video());
        assert!(!PhotoFormat::Tgs.is_video());
        assert!(PhotoFormat::Mpeg4.is_video());
        assert!(PhotoFormat::Webm.is_video());
    }

    #[test]
    fn test_extension() {
        assert_eq!(PhotoFormat::Jpeg.extension(), "jpg");
        assert_eq!(PhotoFormat::Png.extension(), "png");
        assert_eq!(PhotoFormat::Webp.extension(), "webp");
        assert_eq!(PhotoFormat::Gif.extension(), "gif");
        assert_eq!(PhotoFormat::Tgs.extension(), "tgs");
        assert_eq!(PhotoFormat::Mpeg4.extension(), "mp4");
        assert_eq!(PhotoFormat::Webm.extension(), "webm");
    }

    #[test]
    fn test_mime_type() {
        assert_eq!(PhotoFormat::Jpeg.mime_type(), "image/jpeg");
        assert_eq!(PhotoFormat::Png.mime_type(), "image/png");
        assert_eq!(PhotoFormat::Webp.mime_type(), "image/webp");
        assert_eq!(PhotoFormat::Gif.mime_type(), "image/gif");
        assert_eq!(PhotoFormat::Tgs.mime_type(), "application/x-tgsticker");
        assert_eq!(PhotoFormat::Mpeg4.mime_type(), "video/mp4");
        assert_eq!(PhotoFormat::Webm.mime_type(), "video/webm");
    }

    // Display tests (7 tests)
    #[test]
    fn test_display() {
        assert_eq!(format!("{}", PhotoFormat::Jpeg), "JPEG");
        assert_eq!(format!("{}", PhotoFormat::Png), "PNG");
        assert_eq!(format!("{}", PhotoFormat::Webp), "WebP");
        assert_eq!(format!("{}", PhotoFormat::Gif), "GIF");
        assert_eq!(format!("{}", PhotoFormat::Tgs), "TGS");
        assert_eq!(format!("{}", PhotoFormat::Mpeg4), "MPEG-4");
        assert_eq!(format!("{}", PhotoFormat::Webm), "WebM");
    }

    // Debug tests (7 tests)
    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", PhotoFormat::Jpeg), "Jpeg");
        assert_eq!(format!("{:?}", PhotoFormat::Png), "Png");
        assert_eq!(format!("{:?}", PhotoFormat::Webp), "Webp");
        assert_eq!(format!("{:?}", PhotoFormat::Gif), "Gif");
        assert_eq!(format!("{:?}", PhotoFormat::Tgs), "Tgs");
        assert_eq!(format!("{:?}", PhotoFormat::Mpeg4), "Mpeg4");
        assert_eq!(format!("{:?}", PhotoFormat::Webm), "Webm");
    }

    // Round-trip tests (7 tests)
    #[test]
    fn test_round_trip() {
        for format in [
            PhotoFormat::Jpeg,
            PhotoFormat::Png,
            PhotoFormat::Webp,
            PhotoFormat::Gif,
            PhotoFormat::Tgs,
            PhotoFormat::Mpeg4,
            PhotoFormat::Webm,
        ] {
            assert_eq!(PhotoFormat::from_i32(format.as_i32()), Some(format));
        }
    }
}
