// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0;

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct FileId(i32);

impl FileId {
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Photo;

impl Photo {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Dimensions {
    width: i32,
    height: i32,
}

impl Dimensions {
    #[must_use]
    pub const fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    #[must_use]
    pub const fn width(&self) -> i32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> i32 {
        self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum MessageExtendedMediaType {
    Empty = 0,
    Unsupported = 1,
    Preview = 2,
    Photo = 3,
    Video = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageExtendedMedia {
    #[serde(rename = "type")]
    media_type: MessageExtendedMediaType,
    duration: i32,
    dimensions: Dimensions,
    photo_data: Option<Photo>,
    video_file_id: Option<FileId>,
}

impl MessageExtendedMedia {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_photo(photo: Photo) -> Self {
        Self {
            media_type: MessageExtendedMediaType::Photo,
            duration: 0,
            dimensions: Dimensions::default(),
            photo_data: Some(photo),
            video_file_id: None,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self.media_type, MessageExtendedMediaType::Empty)
    }

    #[must_use]
    pub fn is_media(&self) -> bool {
        matches!(
            self.media_type,
            MessageExtendedMediaType::Photo | MessageExtendedMediaType::Video
        )
    }

    #[must_use]
    pub const fn duration(&self) -> i32 {
        self.duration
    }
}

impl Default for MessageExtendedMedia {
    fn default() -> Self {
        Self {
            media_type: MessageExtendedMediaType::Empty,
            duration: 0,
            dimensions: Dimensions::default(),
            photo_data: None,
            video_file_id: None,
        }
    }
}

impl fmt::Display for MessageExtendedMedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.media_type {
            MessageExtendedMediaType::Empty => write!(f, "empty"),
            MessageExtendedMediaType::Photo => write!(f, "photo"),
            MessageExtendedMediaType::Video => write!(f, "video"),
            _ => write!(f, "other"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let media = MessageExtendedMedia::new();
        assert!(media.is_empty());
    }

    #[test]
    fn test_photo() {
        let photo = Photo::new();
        let media = MessageExtendedMedia::with_photo(photo);
        assert!(media.is_media());
    }
}
