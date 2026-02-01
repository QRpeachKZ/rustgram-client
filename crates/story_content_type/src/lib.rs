// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Story content type enumeration.
//!
//! This module implements TDLib's StoryContentType from `td/telegram/StoryContentType.h`.
//!
//! # Example
//!
//! ```rust
//! use rustgram_story_content_type::StoryContentType;
//!
//! let content_type = StoryContentType::Photo;
//! assert!(content_type.can_send());
//! assert!(content_type.can_edit());
//!
//! let unsupported = StoryContentType::Unsupported;
//! assert!(!unsupported.can_send());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use std::fmt::{self, Display, Formatter};

/// Story content type.
///
/// Based on TDLib's `StoryContentType` enum from `td/telegram/StoryContentType.h`.
///
/// Represents the type of content in a Telegram story.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum StoryContentType {
    /// Photo story
    Photo = 0,

    /// Video story
    Video = 1,

    /// Unsupported story type
    #[default]
    Unsupported = 2,

    /// Live stream story
    LiveStream = 3,
}

impl StoryContentType {
    /// Creates a new StoryContentType from an i32 value.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value to convert
    ///
    /// # Returns
    ///
    /// Returns `Some(StoryContentType)` if the value is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content_type::StoryContentType;
    ///
    /// assert_eq!(StoryContentType::from_i32(0), Some(StoryContentType::Photo));
    /// assert_eq!(StoryContentType::from_i32(1), Some(StoryContentType::Video));
    /// assert_eq!(StoryContentType::from_i32(99), None);
    /// ```
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Photo),
            1 => Some(Self::Video),
            2 => Some(Self::Unsupported),
            3 => Some(Self::LiveStream),
            _ => None,
        }
    }

    /// Returns the i32 representation of this content type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content_type::StoryContentType;
    ///
    /// assert_eq!(StoryContentType::Photo.to_i32(), 0);
    /// assert_eq!(StoryContentType::Video.to_i32(), 1);
    /// ```
    pub fn to_i32(self) -> i32 {
        self as i32
    }

    /// Checks if this content type can be sent.
    ///
    /// Based on TDLib's `can_send_story_content` function.
    ///
    /// # Returns
    ///
    /// Returns `true` if the content type can be sent, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content_type::StoryContentType;
    ///
    /// assert!(StoryContentType::Photo.can_send());
    /// assert!(StoryContentType::Video.can_send());
    /// assert!(!StoryContentType::Unsupported.can_send());
    /// assert!(!StoryContentType::LiveStream.can_send());
    /// ```
    pub fn can_send(self) -> bool {
        matches!(self, Self::Photo | Self::Video)
    }

    /// Checks if this content type can be edited.
    ///
    /// Based on TDLib's `can_edit_story_content` function.
    ///
    /// # Returns
    ///
    /// Returns `true` if the content type can be edited, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content_type::StoryContentType;
    ///
    /// assert!(StoryContentType::Photo.can_edit());
    /// assert!(StoryContentType::Video.can_edit());
    /// assert!(!StoryContentType::Unsupported.can_edit());
    /// assert!(!StoryContentType::LiveStream.can_edit());
    /// ```
    pub fn can_edit(self) -> bool {
        matches!(self, Self::Photo | Self::Video)
    }

    /// Checks if this content type is supported.
    ///
    /// # Returns
    ///
    /// Returns `true` if the content type is supported, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content_type::StoryContentType;
    ///
    /// assert!(StoryContentType::Photo.is_supported());
    /// assert!(!StoryContentType::Unsupported.is_supported());
    /// ```
    pub fn is_supported(self) -> bool {
        !matches!(self, Self::Unsupported)
    }
}

impl Display for StoryContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Photo => write!(f, "Photo"),
            Self::Video => write!(f, "Video"),
            Self::Unsupported => write!(f, "Unsupported"),
            Self::LiveStream => write!(f, "LiveStream"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(StoryContentType::from_i32(0), Some(StoryContentType::Photo));
        assert_eq!(StoryContentType::from_i32(1), Some(StoryContentType::Video));
        assert_eq!(
            StoryContentType::from_i32(2),
            Some(StoryContentType::Unsupported)
        );
        assert_eq!(
            StoryContentType::from_i32(3),
            Some(StoryContentType::LiveStream)
        );
        assert_eq!(StoryContentType::from_i32(-1), None);
        assert_eq!(StoryContentType::from_i32(4), None);
        assert_eq!(StoryContentType::from_i32(99), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(StoryContentType::Photo.to_i32(), 0);
        assert_eq!(StoryContentType::Video.to_i32(), 1);
        assert_eq!(StoryContentType::Unsupported.to_i32(), 2);
        assert_eq!(StoryContentType::LiveStream.to_i32(), 3);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for value in 0..=3 {
            let content_type = StoryContentType::from_i32(value);
            assert_eq!(content_type.map(|ct| ct.to_i32()), Some(value));
        }
    }

    #[test]
    fn test_can_send() {
        assert!(StoryContentType::Photo.can_send());
        assert!(StoryContentType::Video.can_send());
        assert!(!StoryContentType::Unsupported.can_send());
        assert!(!StoryContentType::LiveStream.can_send());
    }

    #[test]
    fn test_can_edit() {
        assert!(StoryContentType::Photo.can_edit());
        assert!(StoryContentType::Video.can_edit());
        assert!(!StoryContentType::Unsupported.can_edit());
        assert!(!StoryContentType::LiveStream.can_edit());
    }

    #[test]
    fn test_is_supported() {
        assert!(StoryContentType::Photo.is_supported());
        assert!(StoryContentType::Video.is_supported());
        assert!(!StoryContentType::Unsupported.is_supported());
        assert!(StoryContentType::LiveStream.is_supported());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", StoryContentType::Photo), "Photo");
        assert_eq!(format!("{}", StoryContentType::Video), "Video");
        assert_eq!(format!("{}", StoryContentType::Unsupported), "Unsupported");
        assert_eq!(format!("{}", StoryContentType::LiveStream), "LiveStream");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", StoryContentType::Photo), "Photo");
        assert_eq!(format!("{:?}", StoryContentType::Video), "Video");
        assert_eq!(
            format!("{:?}", StoryContentType::Unsupported),
            "Unsupported"
        );
        assert_eq!(format!("{:?}", StoryContentType::LiveStream), "LiveStream");
    }

    #[test]
    fn test_default() {
        assert_eq!(StoryContentType::default(), StoryContentType::Unsupported);
    }

    #[test]
    fn test_equality() {
        assert_eq!(StoryContentType::Photo, StoryContentType::Photo);
        assert_eq!(StoryContentType::Video, StoryContentType::Video);
        assert_ne!(StoryContentType::Photo, StoryContentType::Video);
        assert_ne!(StoryContentType::Photo, StoryContentType::Unsupported);
    }

    #[test]
    fn test_copy() {
        let a = StoryContentType::Photo;
        let b = a;
        assert_eq!(a, StoryContentType::Photo);
        assert_eq!(b, StoryContentType::Photo);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(StoryContentType::Photo);
        set.insert(StoryContentType::Video);
        set.insert(StoryContentType::Unsupported);
        set.insert(StoryContentType::LiveStream);
        assert_eq!(set.len(), 4);
    }
}
