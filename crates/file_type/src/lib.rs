// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # File Type
//!
//! File type enumeration for different kinds of files in Telegram.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `FileType` enum from `td/telegram/files/FileType.h`.
//!
//! ## Structure
//!
//! - `FileType`: Enum of all file types (Photo, Video, Document, etc.)
//! - `FileTypeClass`: Higher-level classification (Photo, Document, Secure, Encrypted, Temp)
//! - `FileDirType`: Directory type (Secure, Common)
//!
//! ## Usage
//!
//! ```
//! use rustgram_file_type::{FileType, FileTypeClass};
//!
//! let file_type = FileType::Photo;
//! assert_eq!(file_type.class(), FileTypeClass::Photo);
//! assert_eq!(file_type.dir_name(), "photos");
//! assert!(file_type.is_photo());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Maximum file type value (used for array sizing).
pub const MAX_FILE_TYPE: usize = FileType::Size as usize;

/// File type enumeration matching TDLib.
///
/// Corresponds to TDLib `FileType` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[repr(i32)]
pub enum FileType {
    /// Thumbnail image
    Thumbnail = 0,
    /// Profile photo
    ProfilePhoto = 1,
    /// Regular photo
    Photo = 2,
    /// Voice note
    VoiceNote = 3,
    /// Video file
    Video = 4,
    /// Generic document
    Document = 5,
    /// Encrypted file
    Encrypted = 6,
    /// Temporary file
    Temp = 7,
    /// Sticker
    Sticker = 8,
    /// Audio file
    Audio = 9,
    /// Animation (GIF)
    Animation = 10,
    /// Encrypted thumbnail
    EncryptedThumbnail = 11,
    /// Wallpaper (deprecated, use Background)
    Wallpaper = 12,
    /// Video note (round video)
    VideoNote = 13,
    /// Secure decrypted file
    SecureDecrypted = 14,
    /// Secure encrypted file
    SecureEncrypted = 15,
    /// Background/wallpaper
    Background = 16,
    /// Document sent as file
    DocumentAsFile = 17,
    /// Ringtone/notification sound
    Ringtone = 18,
    /// Call log
    CallLog = 19,
    /// Photo story
    PhotoStory = 20,
    /// Video story
    VideoStory = 21,
    /// Self-destructing photo
    SelfDestructingPhoto = 22,
    /// Self-destructing video
    SelfDestructingVideo = 23,
    /// Self-destructing video note
    SelfDestructingVideoNote = 24,
    /// Self-destructing voice note
    SelfDestructingVoiceNote = 25,
    /// Sentinel value for array size
    Size = 26,
    /// Invalid/unknown file type
    #[default]
    None = 27,
}

impl FileType {
    /// Returns the main/primary file type, collapsing aliases.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_type::FileType;
    ///
    /// assert_eq!(FileType::Wallpaper.main_type(), FileType::Background);
    /// assert_eq!(FileType::SecureDecrypted.main_type(), FileType::SecureEncrypted);
    /// assert_eq!(FileType::DocumentAsFile.main_type(), FileType::Document);
    /// ```
    #[must_use]
    pub const fn main_type(self) -> FileType {
        match self {
            FileType::Wallpaper => FileType::Background,
            FileType::SecureDecrypted => FileType::SecureEncrypted,
            FileType::DocumentAsFile => FileType::Document,
            FileType::CallLog => FileType::Document,
            _ => self,
        }
    }

    /// Returns the directory name for this file type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_type::FileType;
    ///
    /// assert_eq!(FileType::Photo.dir_name(), "photos");
    /// assert_eq!(FileType::Video.dir_name(), "videos");
    /// assert_eq!(FileType::Document.dir_name(), "documents");
    /// ```
    #[must_use]
    pub fn dir_name(self) -> &'static str {
        match self.main_type() {
            FileType::Thumbnail => "thumbnails",
            FileType::ProfilePhoto => "profile_photos",
            FileType::Photo | FileType::SelfDestructingPhoto => "photos",
            FileType::VoiceNote | FileType::SelfDestructingVoiceNote => "voice",
            FileType::Video | FileType::SelfDestructingVideo => "videos",
            FileType::Document => "documents",
            FileType::Encrypted => "secret",
            FileType::Temp => "temp",
            FileType::Sticker => "stickers",
            FileType::Audio => "music",
            FileType::Animation => "animations",
            FileType::EncryptedThumbnail => "secret_thumbnails",
            FileType::VideoNote | FileType::SelfDestructingVideoNote => "video_notes",
            FileType::SecureEncrypted => "passport",
            FileType::Background => "wallpapers",
            FileType::Ringtone => "notification_sounds",
            FileType::PhotoStory | FileType::VideoStory => "stories",
            _ => "none",
        }
    }

    /// Returns the unique directory name for this file type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_type::FileType;
    ///
    /// assert_eq!(FileType::PhotoStory.unique_dir_name(), "stories");
    /// assert_eq!(FileType::VideoStory.unique_dir_name(), "video_stories");
    /// ```
    #[must_use]
    pub fn unique_dir_name(self) -> &'static str {
        if self == FileType::VideoStory {
            return "video_stories";
        }
        self.dir_name()
    }

    /// Returns the file type class.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_type::{FileType, FileTypeClass};
    ///
    /// assert_eq!(FileType::Photo.class(), FileTypeClass::Photo);
    /// assert_eq!(FileType::Video.class(), FileTypeClass::Document);
    /// assert_eq!(FileType::Encrypted.class(), FileTypeClass::Encrypted);
    /// ```
    #[must_use]
    pub const fn class(self) -> FileTypeClass {
        match self {
            FileType::Photo
            | FileType::ProfilePhoto
            | FileType::Thumbnail
            | FileType::EncryptedThumbnail
            | FileType::Wallpaper
            | FileType::PhotoStory
            | FileType::SelfDestructingPhoto => FileTypeClass::Photo,
            FileType::Video
            | FileType::VoiceNote
            | FileType::Document
            | FileType::Sticker
            | FileType::Audio
            | FileType::Animation
            | FileType::VideoNote
            | FileType::Background
            | FileType::DocumentAsFile
            | FileType::Ringtone
            | FileType::CallLog
            | FileType::VideoStory
            | FileType::SelfDestructingVideo
            | FileType::SelfDestructingVideoNote
            | FileType::SelfDestructingVoiceNote => FileTypeClass::Document,
            FileType::SecureDecrypted | FileType::SecureEncrypted => FileTypeClass::Secure,
            FileType::Encrypted => FileTypeClass::Encrypted,
            FileType::Temp => FileTypeClass::Temp,
            FileType::Size | FileType::None => FileTypeClass::Temp,
        }
    }

    /// Returns the directory type (Secure or Common).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_type::{FileType, FileDirType};
    ///
    /// assert_eq!(FileType::Thumbnail.dir_type(), FileDirType::Secure);
    /// assert_eq!(FileType::Video.dir_type(), FileDirType::Common);
    /// ```
    #[must_use]
    pub const fn dir_type(self) -> FileDirType {
        match self {
            FileType::Thumbnail
            | FileType::ProfilePhoto
            | FileType::Encrypted
            | FileType::Sticker
            | FileType::Temp
            | FileType::Wallpaper
            | FileType::EncryptedThumbnail
            | FileType::SecureEncrypted
            | FileType::SecureDecrypted
            | FileType::Background
            | FileType::Ringtone
            | FileType::PhotoStory
            | FileType::VideoStory
            | FileType::SelfDestructingPhoto
            | FileType::SelfDestructingVideo
            | FileType::SelfDestructingVideoNote
            | FileType::SelfDestructingVoiceNote => FileDirType::Secure,
            _ => FileDirType::Common,
        }
    }

    /// Returns `true` if the file is considered "big" (>10MB).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_type::FileType;
    ///
    /// assert!(!FileType::Photo.is_big(100_000_000)); // Photos are never big
    /// assert!(FileType::Video.is_big(20_000_000));  // 20MB video is big
    /// assert!(!FileType::Video.is_big(5_000_000));  // 5MB video is not big
    /// ```
    #[must_use]
    pub fn is_big(self, expected_size: i64) -> bool {
        // Photos are never considered big
        if self.class() == FileTypeClass::Photo {
            return false;
        }
        // These types are never considered big
        match self {
            FileType::VideoNote | FileType::Ringtone | FileType::CallLog | FileType::VideoStory => {
                return false;
            }
            _ => {}
        }
        const SMALL_FILE_MAX_SIZE: i64 = 10 << 20; // 10MB
        expected_size > SMALL_FILE_MAX_SIZE
    }

    /// Returns `true` if remote files of this type can be reused.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_type::FileType;
    ///
    /// assert!(!FileType::Thumbnail.can_reuse_remote()); // Thumbnails can't be reused
    /// assert!(FileType::Photo.can_reuse_remote());      // Photos can be reused
    /// ```
    #[must_use]
    pub const fn can_reuse_remote(self) -> bool {
        !matches!(
            self,
            FileType::Thumbnail
                | FileType::EncryptedThumbnail
                | FileType::Background
                | FileType::CallLog
                | FileType::PhotoStory
                | FileType::VideoStory
                | FileType::SelfDestructingPhoto
                | FileType::SelfDestructingVideo
                | FileType::SelfDestructingVideoNote
                | FileType::SelfDestructingVoiceNote
        )
    }

    /// Returns `true` if this is a photo-type file.
    #[must_use]
    pub const fn is_photo(self) -> bool {
        matches!(
            self,
            FileType::Photo
                | FileType::ProfilePhoto
                | FileType::Thumbnail
                | FileType::EncryptedThumbnail
                | FileType::Wallpaper
                | FileType::PhotoStory
                | FileType::SelfDestructingPhoto
        )
    }

    /// Returns `true` if this is a document-type file.
    #[must_use]
    pub fn is_document(self) -> bool {
        self.class() == FileTypeClass::Document
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            FileType::Thumbnail => "Thumbnail",
            FileType::ProfilePhoto => "ChatPhoto",
            FileType::Photo => "Photo",
            FileType::VoiceNote => "VoiceNote",
            FileType::Video => "Video",
            FileType::Document => "Document",
            FileType::Encrypted => "Secret",
            FileType::Temp => "Temp",
            FileType::Sticker => "Sticker",
            FileType::Audio => "Audio",
            FileType::Animation => "Animation",
            FileType::EncryptedThumbnail => "SecretThumbnail",
            FileType::Wallpaper => "Wallpaper",
            FileType::VideoNote => "VideoNote",
            FileType::SecureDecrypted => "Passport",
            FileType::SecureEncrypted => "Passport",
            FileType::Background => "Background",
            FileType::DocumentAsFile => "DocumentAsFile",
            FileType::Ringtone => "NotificationSound",
            FileType::CallLog => "CallLog",
            FileType::PhotoStory => "PhotoStory",
            FileType::VideoStory => "VideoStory",
            FileType::SelfDestructingPhoto => "SelfDestructingPhoto",
            FileType::SelfDestructingVideo => "SelfDestructingVideo",
            FileType::SelfDestructingVideoNote => "SelfDestructingVideoNote",
            FileType::SelfDestructingVoiceNote => "SelfDestructingVoiceNote",
            FileType::Size | FileType::None => "<invalid>",
        };
        write!(f, "{name}")
    }
}

/// File type class (higher-level categorization).
///
/// Corresponds to TDLib `FileTypeClass` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileTypeClass {
    /// Photo files
    Photo,
    /// Document files (videos, audio, etc.)
    Document,
    /// Secure storage files
    Secure,
    /// Encrypted secret files
    Encrypted,
    /// Temporary files
    Temp,
}

impl fmt::Display for FileTypeClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            FileTypeClass::Photo => "Photo",
            FileTypeClass::Document => "Document",
            FileTypeClass::Secure => "Secure",
            FileTypeClass::Encrypted => "Encrypted",
            FileTypeClass::Temp => "Temp",
        };
        write!(f, "{name}")
    }
}

/// File directory type.
///
/// Corresponds to TDLib `FileDirType` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileDirType {
    /// Secure directory (encrypted)
    Secure,
    /// Common directory
    Common,
}

impl fmt::Display for FileDirType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            FileDirType::Secure => "Secure",
            FileDirType::Common => "Common",
        };
        write!(f, "{name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // === Basic trait tests ===

    #[test]
    fn test_clone() {
        let file_type = FileType::Photo;
        let cloned = file_type;
        assert_eq!(file_type, cloned);
    }

    #[test]
    fn test_copy() {
        let file_type = FileType::Photo;
        let copied = file_type;
        assert_eq!(file_type, copied);
    }

    #[test]
    fn test_default() {
        assert_eq!(FileType::default(), FileType::None);
    }

    // === main_type tests ===

    #[rstest]
    #[case(FileType::Wallpaper, FileType::Background)]
    #[case(FileType::SecureDecrypted, FileType::SecureEncrypted)]
    #[case(FileType::DocumentAsFile, FileType::Document)]
    #[case(FileType::CallLog, FileType::Document)]
    #[case(FileType::Photo, FileType::Photo)]
    fn test_main_type(#[case] input: FileType, #[case] expected: FileType) {
        assert_eq!(input.main_type(), expected);
    }

    // === dir_name tests ===

    #[rstest]
    #[case(FileType::Thumbnail, "thumbnails")]
    #[case(FileType::ProfilePhoto, "profile_photos")]
    #[case(FileType::Photo, "photos")]
    #[case(FileType::VoiceNote, "voice")]
    #[case(FileType::Video, "videos")]
    #[case(FileType::Document, "documents")]
    #[case(FileType::Encrypted, "secret")]
    #[case(FileType::Temp, "temp")]
    #[case(FileType::Sticker, "stickers")]
    #[case(FileType::Audio, "music")]
    #[case(FileType::Animation, "animations")]
    #[case(FileType::EncryptedThumbnail, "secret_thumbnails")]
    #[case(FileType::VideoNote, "video_notes")]
    #[case(FileType::SecureEncrypted, "passport")]
    #[case(FileType::Background, "wallpapers")]
    #[case(FileType::Ringtone, "notification_sounds")]
    #[case(FileType::PhotoStory, "stories")]
    fn test_dir_name(#[case] file_type: FileType, #[case] expected: &str) {
        assert_eq!(file_type.dir_name(), expected);
    }

    // === unique_dir_name tests ===

    #[test]
    fn test_unique_dir_name() {
        assert_eq!(FileType::PhotoStory.unique_dir_name(), "stories");
        assert_eq!(FileType::VideoStory.unique_dir_name(), "video_stories");
        assert_eq!(FileType::Photo.unique_dir_name(), "photos");
    }

    // === class tests ===

    #[rstest]
    #[case(FileType::Photo, FileTypeClass::Photo)]
    #[case(FileType::ProfilePhoto, FileTypeClass::Photo)]
    #[case(FileType::Thumbnail, FileTypeClass::Photo)]
    #[case(FileType::Video, FileTypeClass::Document)]
    #[case(FileType::Document, FileTypeClass::Document)]
    #[case(FileType::Encrypted, FileTypeClass::Encrypted)]
    #[case(FileType::SecureEncrypted, FileTypeClass::Secure)]
    #[case(FileType::Temp, FileTypeClass::Temp)]
    fn test_class(#[case] file_type: FileType, #[case] expected: FileTypeClass) {
        assert_eq!(file_type.class(), expected);
    }

    // === dir_type tests ===

    #[test]
    fn test_dir_type() {
        assert_eq!(FileType::Thumbnail.dir_type(), FileDirType::Secure);
        assert_eq!(FileType::Video.dir_type(), FileDirType::Common);
        assert_eq!(FileType::Document.dir_type(), FileDirType::Common);
        assert_eq!(FileType::Sticker.dir_type(), FileDirType::Secure);
    }

    // === is_big tests ===

    #[rstest]
    #[case(FileType::Photo, 100_000_000, false)] // Photos never big
    #[case(FileType::Video, 20_000_000, true)] // 20MB is big
    #[case(FileType::Video, 5_000_000, false)] // 5MB is not big
    #[case(FileType::Document, 15_000_000, true)] // 15MB is big
    #[case(FileType::Document, 8_000_000, false)] // 8MB is not big
    #[case(FileType::VideoNote, 50_000_000, false)] // VideoNotes never big
    fn test_is_big(#[case] file_type: FileType, #[case] size: i64, #[case] expected: bool) {
        assert_eq!(file_type.is_big(size), expected);
    }

    // === can_reuse_remote tests ===

    #[test]
    fn test_can_reuse_remote() {
        assert!(!FileType::Thumbnail.can_reuse_remote());
        assert!(!FileType::Background.can_reuse_remote());
        assert!(FileType::Photo.can_reuse_remote());
        assert!(FileType::Video.can_reuse_remote());
        assert!(FileType::Document.can_reuse_remote());
    }

    // === is_photo tests ===

    #[test]
    fn test_is_photo() {
        assert!(FileType::Photo.is_photo());
        assert!(FileType::ProfilePhoto.is_photo());
        assert!(FileType::Thumbnail.is_photo());
        assert!(!FileType::Video.is_photo());
        assert!(!FileType::Document.is_photo());
    }

    // === is_document tests ===

    #[test]
    fn test_is_document() {
        assert!(FileType::Video.is_document());
        assert!(FileType::Document.is_document());
        assert!(FileType::Audio.is_document());
        assert!(!FileType::Photo.is_document());
        assert!(!FileType::Encrypted.is_document());
    }

    // === Display tests ===

    #[rstest]
    #[case(FileType::Photo, "Photo")]
    #[case(FileType::Video, "Video")]
    #[case(FileType::Document, "Document")]
    #[case(FileType::Encrypted, "Secret")]
    fn test_display(#[case] file_type: FileType, #[case] expected: &str) {
        assert_eq!(format!("{file_type}"), expected);
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize() {
        let file_type = FileType::Photo;
        let json = serde_json::to_string(&file_type).unwrap();
        assert!(json.contains("Photo"));
    }

    #[test]
    fn test_deserialize() {
        let json = r#""Photo""#;
        let file_type: FileType = serde_json::from_str(json).unwrap();
        assert_eq!(file_type, FileType::Photo);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let original = FileType::Video;
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: FileType = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
