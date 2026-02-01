// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Story Content
//!
//! Story media content types for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`StoryContent`] struct, which represents
//! the media content of a Telegram story. It is a simplified stub
//! implementation that supports basic content type detection.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_story_content::StoryContent;
//!
//! // Create a photo story
//! let photo = StoryContent::photo();
//! assert_eq!(photo.content_type(), "photo");
//! assert!(!photo.is_video());
//!
//! // Create a video story
//! let video = StoryContent::video(30);
//! assert_eq!(video.content_type(), "video");
//! assert!(video.is_video());
//! assert_eq!(video.duration(), Some(30));
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Story media content.
///
/// Represents the media content of a Telegram story.
/// This is a simplified stub implementation.
///
/// # Fields
///
/// - `content_type` - Type of content ("photo", "video", etc.)
/// - `duration` - Duration in seconds for video stories
///
/// # Example
///
/// ```rust
/// use rustgram_story_content::StoryContent;
///
/// let photo = StoryContent::photo();
/// assert_eq!(photo.content_type(), "photo");
///
/// let video = StoryContent::video(30);
/// assert_eq!(video.content_type(), "video");
/// assert_eq!(video.duration(), Some(30));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoryContent {
    /// Type of content ("photo", "video", etc.)
    content_type: String,

    /// Duration in seconds (for video stories)
    duration: Option<i32>,
}

impl StoryContent {
    /// Creates a new story content with the specified type.
    ///
    /// # Arguments
    ///
    /// * `content_type` - The type of content (e.g., "photo", "video")
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content::StoryContent;
    ///
    /// let content = StoryContent::new("photo".to_string());
    /// assert_eq!(content.content_type(), "photo");
    /// ```
    #[must_use]
    pub fn new(content_type: String) -> Self {
        Self {
            content_type,
            duration: None,
        }
    }

    /// Creates a photo story content.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content::StoryContent;
    ///
    /// let photo = StoryContent::photo();
    /// assert_eq!(photo.content_type(), "photo");
    /// assert!(!photo.is_video());
    /// ```
    #[must_use]
    pub fn photo() -> Self {
        Self {
            content_type: String::from("photo"),
            duration: None,
        }
    }

    /// Creates a video story content.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration in seconds
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content::StoryContent;
    ///
    /// let video = StoryContent::video(30);
    /// assert_eq!(video.content_type(), "video");
    /// assert!(video.is_video());
    /// assert_eq!(video.duration(), Some(30));
    /// ```
    #[must_use]
    pub fn video(duration: i32) -> Self {
        Self {
            content_type: String::from("video"),
            duration: Some(duration),
        }
    }

    /// Returns the content type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content::StoryContent;
    ///
    /// let photo = StoryContent::photo();
    /// assert_eq!(photo.content_type(), "photo");
    /// ```
    #[must_use]
    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    /// Returns the duration for video stories.
    ///
    /// Returns `None` for non-video content.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content::StoryContent;
    ///
    /// let photo = StoryContent::photo();
    /// assert_eq!(photo.duration(), None);
    ///
    /// let video = StoryContent::video(30);
    /// assert_eq!(video.duration(), Some(30));
    /// ```
    #[must_use]
    pub fn duration(&self) -> Option<i32> {
        self.duration
    }

    /// Returns `true` if this is video content.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content::StoryContent;
    ///
    /// assert!(!StoryContent::photo().is_video());
    /// assert!(StoryContent::video(30).is_video());
    /// ```
    #[must_use]
    pub fn is_video(&self) -> bool {
        self.content_type == "video"
    }

    /// Returns `true` if this is photo content.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_content::StoryContent;
    ///
    /// assert!(StoryContent::photo().is_photo());
    /// assert!(!StoryContent::video(30).is_photo());
    /// ```
    #[must_use]
    pub fn is_photo(&self) -> bool {
        self.content_type == "photo"
    }
}

impl fmt::Display for StoryContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.duration {
            None => write!(f, "{}", self.content_type),
            Some(d) => write!(f, "{}({}s)", self.content_type, d),
        }
    }
}

impl Default for StoryContent {
    fn default() -> Self {
        Self::photo()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let content = StoryContent::new("photo".to_string());
        assert_eq!(content.content_type(), "photo");
        assert_eq!(content.duration(), None);
    }

    #[test]
    fn test_photo() {
        let photo = StoryContent::photo();
        assert_eq!(photo.content_type(), "photo");
        assert!(photo.is_photo());
        assert!(!photo.is_video());
        assert_eq!(photo.duration(), None);
    }

    #[test]
    fn test_video() {
        let video = StoryContent::video(30);
        assert_eq!(video.content_type(), "video");
        assert!(!video.is_photo());
        assert!(video.is_video());
        assert_eq!(video.duration(), Some(30));
    }

    #[test]
    fn test_is_video() {
        assert!(!StoryContent::photo().is_video());
        assert!(StoryContent::video(10).is_video());
        assert!(StoryContent::video(60).is_video());
    }

    #[test]
    fn test_is_photo() {
        assert!(StoryContent::photo().is_photo());
        assert!(!StoryContent::video(10).is_photo());
    }

    #[test]
    fn test_duration_photo() {
        assert_eq!(StoryContent::photo().duration(), None);
    }

    #[test]
    fn test_duration_video() {
        assert_eq!(StoryContent::video(0).duration(), Some(0));
        assert_eq!(StoryContent::video(30).duration(), Some(30));
        assert_eq!(StoryContent::video(120).duration(), Some(120));
    }

    #[test]
    fn test_equality() {
        let photo1 = StoryContent::photo();
        let photo2 = StoryContent::photo();
        assert_eq!(photo1, photo2);

        let video1 = StoryContent::video(30);
        let video2 = StoryContent::video(30);
        assert_eq!(video1, video2);

        assert_ne!(photo1, video1);
    }

    #[test]
    fn test_clone() {
        let photo = StoryContent::photo();
        let cloned = photo.clone();
        assert_eq!(photo, cloned);

        let video = StoryContent::video(30);
        let cloned_video = video.clone();
        assert_eq!(video, cloned_video);
    }

    #[test]
    fn test_display_photo() {
        let photo = StoryContent::photo();
        assert_eq!(format!("{}", photo), "photo");
    }

    #[test]
    fn test_display_video() {
        let video = StoryContent::video(30);
        assert_eq!(format!("{}", video), "video(30s)");
    }

    #[test]
    fn test_default() {
        let default = StoryContent::default();
        assert!(default.is_photo());
    }

    #[test]
    fn test_custom_content_type() {
        let custom = StoryContent::new("animated".to_string());
        assert_eq!(custom.content_type(), "animated");
        assert!(!custom.is_photo());
        assert!(!custom.is_video());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let photo = StoryContent::photo();
        let json = serde_json::to_string(&photo).unwrap();
        let deserialized: StoryContent = serde_json::from_str(&json).unwrap();
        assert_eq!(photo, deserialized);

        let video = StoryContent::video(30);
        let json = serde_json::to_string(&video).unwrap();
        let deserialized: StoryContent = serde_json::from_str(&json).unwrap();
        assert_eq!(video, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_format() {
        let photo = StoryContent::photo();
        let json = serde_json::to_string(&photo).unwrap();
        assert!(json.contains("photo"));

        let video = StoryContent::video(30);
        let json = serde_json::to_string(&video).unwrap();
        assert!(json.contains("video"));
        assert!(json.contains("30"));
    }

    #[test]
    fn test_negative_duration() {
        // Negative duration is technically allowed in this stub
        let video = StoryContent::video(-10);
        assert_eq!(video.duration(), Some(-10));
    }

    #[test]
    fn test_zero_duration() {
        let video = StoryContent::video(0);
        assert_eq!(video.duration(), Some(0));
        assert!(video.is_video());
    }

    #[test]
    fn test_debug_format() {
        let photo = StoryContent::photo();
        let debug_str = format!("{:?}", photo);
        assert!(debug_str.contains("StoryContent"));
        assert!(debug_str.contains("photo"));

        let video = StoryContent::video(30);
        let debug_str = format!("{:?}", video);
        assert!(debug_str.contains("video"));
        assert!(debug_str.contains("30"));
    }
}
