// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Story Album
//!
//! Story album for Telegram.
//!
//! Based on TDLib's `StoryAlbum` from `td/telegram/StoryAlbum.h`.
//!
//! # Overview
//!
//! A `StoryAlbum` represents a story album in Telegram.
//!
//! # Example
//!
//! ```rust
//! use rustgram_story_album::StoryAlbum;
//!
//! let album = StoryAlbum::new();
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_file_id::FileId;
use rustgram_story_album_id::StoryAlbumId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stub for Photo.
/// TODO: Replace with full Photo type when available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Photo;

/// Story album.
///
/// Represents a story album in Telegram.
///
/// # TDLib Mapping
///
/// TDLib: `StoryAlbum`
///
/// # Example
///
/// ```rust
/// use rustgram_story_album::StoryAlbum;
///
/// let album = StoryAlbum::new();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StoryAlbum {
    /// Album ID
    album_id: Option<StoryAlbumId>,
    /// Title
    title: Option<String>,
    /// Icon photo
    icon_photo: Option<Photo>,
    /// Icon video file ID
    icon_video_file_id: Option<FileId>,
}

impl StoryAlbum {
    /// Creates a new story album.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_album::StoryAlbum;
    ///
    /// let album = StoryAlbum::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if this album is valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_album::StoryAlbum;
    ///
    /// let album = StoryAlbum::new();
    /// assert!(!album.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.album_id.is_some()
    }
}

impl fmt::Display for StoryAlbum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StoryAlbum")?;
        if let Some(title) = &self.title {
            write!(f, ": {}", title)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let album = StoryAlbum::new();
        assert!(!album.is_valid());
    }

    #[test]
    fn test_default() {
        let album = StoryAlbum::default();
        assert!(!album.is_valid());
    }

    #[test]
    fn test_is_valid() {
        let album = StoryAlbum::new();
        assert!(!album.is_valid());
    }

    #[test]
    fn test_display() {
        let album = StoryAlbum::new();
        assert_eq!(format!("{album}"), "StoryAlbum");
    }

    #[test]
    fn test_display_with_title() {
        let mut album = StoryAlbum::new();
        album.title = Some("My Album".to_string());
        assert!(format!("{album}").contains("My Album"));
    }

    #[test]
    fn test_equality() {
        let album1 = StoryAlbum::new();
        let album2 = StoryAlbum::new();
        assert_eq!(album1, album2);
    }

    #[test]
    fn test_clone() {
        let album1 = StoryAlbum::new();
        let album2 = album1.clone();
        assert_eq!(album1, album2);
    }

    #[test]
    fn test_serialization() {
        let album = StoryAlbum::new();
        let json = serde_json::to_string(&album).expect("Failed to serialize");
        let deserialized: StoryAlbum =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, album);
    }
}
