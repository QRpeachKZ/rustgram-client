// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Audio manager for Telegram MTProto client.
//!
//! This module implements TDLib's AudiosManager class.
//!
//! # Example
//!
//! ```rust
//! use rustgram_audios_manager::{Audio, AudiosManager};
//! use rustgram_file_id::FileId;
//!
//! let mut manager = AudiosManager::new();
//! let file_id = FileId::new(1, 0);
//! manager.create_audio(file_id, "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_file_id::FileId;
use rustgram_photo_size::PhotoSize;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

/// Audio metadata.
///
/// Based on TDLib's `AudiosManager::Audio` struct.
///
/// Contains information about an audio file including metadata
/// and thumbnail information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Audio {
    /// File name.
    pub file_name: String,

    /// MIME type.
    pub mime_type: String,

    /// Duration in seconds.
    pub duration: i32,

    /// Date of creation.
    pub date: i32,

    /// Audio title.
    pub title: String,

    /// Performer/artist name.
    pub performer: String,

    /// Minithumbnail data.
    pub minithumbnail: String,

    /// Photo thumbnail.
    pub thumbnail: Option<PhotoSize>,

    /// File ID.
    pub file_id: FileId,
}

impl Audio {
    /// Creates a new Audio.
    ///
    /// # Arguments
    ///
    /// * `file_id` - File ID
    /// * `file_name` - File name
    /// * `mime_type` - MIME type
    /// * `duration` - Duration in seconds
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::Audio;
    /// use rustgram_file_id::FileId;
    ///
    /// let audio = Audio::new(
    ///     FileId::new(1, 0),
    ///     "test.mp3".to_string(),
    ///     "audio/mpeg".to_string(),
    ///     120
    /// );
    /// ```
    pub fn new(file_id: FileId, file_name: String, mime_type: String, duration: i32) -> Self {
        Self {
            file_name,
            mime_type,
            duration,
            date: 0,
            title: String::new(),
            performer: String::new(),
            minithumbnail: String::new(),
            thumbnail: None,
            file_id,
        }
    }

    /// Returns the audio duration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::Audio;
    /// use rustgram_file_id::FileId;
    ///
    /// let audio = Audio::new(FileId::new(1, 0), "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
    /// assert_eq!(audio.duration(), 120);
    /// ```
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// Sets the audio title.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::Audio;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut audio = Audio::new(FileId::new(1, 0), "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
    /// audio.set_title("My Song".to_string());
    /// assert_eq!(audio.title, "My Song");
    /// ```
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Sets the performer name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::Audio;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut audio = Audio::new(FileId::new(1, 0), "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
    /// audio.set_performer("Artist Name".to_string());
    /// assert_eq!(audio.performer, "Artist Name");
    /// ```
    pub fn set_performer(&mut self, performer: String) {
        self.performer = performer;
    }

    /// Sets the thumbnail.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::Audio;
    /// use rustgram_file_id::FileId;
    /// use rustgram_photo_size::PhotoSize;
    ///
    /// let mut audio = Audio::new(FileId::new(1, 0), "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
    /// audio.set_thumbnail(Some(PhotoSize::new("small".to_string())));
    /// assert!(audio.thumbnail.is_some());
    /// ```
    pub fn set_thumbnail(&mut self, thumbnail: Option<PhotoSize>) {
        self.thumbnail = thumbnail;
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            file_name: String::new(),
            mime_type: String::new(),
            duration: 0,
            date: 0,
            title: String::new(),
            performer: String::new(),
            minithumbnail: String::new(),
            thumbnail: None,
            file_id: FileId::empty(),
        }
    }
}

impl Display for Audio {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Audio(file={}, duration={}, title={})",
            self.file_name, self.duration, self.title
        )
    }
}

/// Audio manager.
///
/// Based on TDLib's `AudiosManager` class.
///
/// Manages audio files and their metadata.
#[derive(Debug, Clone)]
pub struct AudiosManager {
    /// Map of file IDs to audio metadata.
    audios: HashMap<FileId, Audio>,
}

impl AudiosManager {
    /// Creates a new AudiosManager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::AudiosManager;
    ///
    /// let manager = AudiosManager::new();
    /// assert_eq!(manager.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            audios: HashMap::new(),
        }
    }

    /// Creates a new audio entry.
    ///
    /// # Arguments
    ///
    /// * `file_id` - File ID
    /// * `file_name` - File name
    /// * `mime_type` - MIME type
    /// * `duration` - Duration in seconds
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::AudiosManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = AudiosManager::new();
    /// let file_id = FileId::new(1, 0);
    /// manager.create_audio(file_id, "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
    /// assert_eq!(manager.len(), 1);
    /// ```
    pub fn create_audio(
        &mut self,
        file_id: FileId,
        file_name: String,
        mime_type: String,
        duration: i32,
    ) {
        let audio = Audio::new(file_id, file_name, mime_type, duration);
        self.audios.insert(file_id, audio);
    }

    /// Gets the audio duration for a file ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - File ID
    ///
    /// # Returns
    ///
    /// The duration in seconds, or 0 if not found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::AudiosManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = AudiosManager::new();
    /// let file_id = FileId::new(1, 0);
    /// manager.create_audio(file_id, "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
    /// assert_eq!(manager.get_audio_duration(file_id), 120);
    /// ```
    pub fn get_audio_duration(&self, file_id: FileId) -> i32 {
        self.audios.get(&file_id).map_or(0, |audio| audio.duration())
    }

    /// Gets the audio for a file ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - File ID
    ///
    /// # Returns
    ///
    /// Option containing the audio if found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::AudiosManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = AudiosManager::new();
    /// let file_id = FileId::new(1, 0);
    /// manager.create_audio(file_id, "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
    /// let audio = manager.get_audio(file_id);
    /// assert!(audio.is_some());
    /// ```
    pub fn get_audio(&self, file_id: FileId) -> Option<&Audio> {
        self.audios.get(&file_id)
    }

    /// Returns the number of audios managed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::AudiosManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = AudiosManager::new();
    /// assert_eq!(manager.len(), 0);
    /// manager.create_audio(FileId::new(1, 0), "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
    /// assert_eq!(manager.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.audios.len()
    }

    /// Returns true if there are no audios.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::AudiosManager;
    ///
    /// let manager = AudiosManager::new();
    /// assert!(manager.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.audios.is_empty()
    }

    /// Removes an audio by file ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - File ID
    ///
    /// # Returns
    ///
    /// Option containing the removed audio if it existed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_audios_manager::AudiosManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = AudiosManager::new();
    /// let file_id = FileId::new(1, 0);
    /// manager.create_audio(file_id.clone(), "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
    /// let removed = manager.remove_audio(file_id);
    /// assert!(removed.is_some());
    /// assert!(manager.is_empty());
    /// ```
    pub fn remove_audio(&mut self, file_id: FileId) -> Option<Audio> {
        self.audios.remove(&file_id)
    }
}

impl Default for AudiosManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_new() {
        let audio = Audio::new(
            FileId::new(1, 0),
            "test.mp3".to_string(),
            "audio/mpeg".to_string(),
            120,
        );
        assert_eq!(audio.file_name, "test.mp3");
        assert_eq!(audio.mime_type, "audio/mpeg");
        assert_eq!(audio.duration, 120);
    }

    #[test]
    fn test_audio_default() {
        let audio = Audio::default();
        assert_eq!(audio.duration, 0);
        assert!(audio.title.is_empty());
    }

    #[test]
    fn test_audio_duration() {
        let audio = Audio::new(
            FileId::new(1, 0),
            "test.mp3".to_string(),
            "audio/mpeg".to_string(),
            240,
        );
        assert_eq!(audio.duration(), 240);
    }

    #[test]
    fn test_audio_set_title() {
        let mut audio = Audio::new(
            FileId::new(1, 0),
            "test.mp3".to_string(),
            "audio/mpeg".to_string(),
            120,
        );
        audio.set_title("My Song".to_string());
        assert_eq!(audio.title, "My Song");
    }

    #[test]
    fn test_audio_set_performer() {
        let mut audio = Audio::new(
            FileId::new(1, 0),
            "test.mp3".to_string(),
            "audio/mpeg".to_string(),
            120,
        );
        audio.set_performer("Artist Name".to_string());
        assert_eq!(audio.performer, "Artist Name");
    }

    #[test]
    fn test_audio_set_thumbnail() {
        let mut audio = Audio::new(
            FileId::new(1, 0),
            "test.mp3".to_string(),
            "audio/mpeg".to_string(),
            120,
        );
        audio.set_thumbnail(Some(PhotoSize::new("small".to_string())));
        assert!(audio.thumbnail.is_some());
    }

    #[test]
    fn test_audio_equality() {
        let audio1 = Audio::new(
            FileId::new(1, 0),
            "test.mp3".to_string(),
            "audio/mpeg".to_string(),
            120,
        );
        let mut audio2 = Audio::new(
            FileId::new(1, 0),
            "test.mp3".to_string(),
            "audio/mpeg".to_string(),
            120,
        );
        assert_eq!(audio1, audio2);

        audio2.set_title("Different".to_string());
        assert_ne!(audio1, audio2);
    }

    #[test]
    fn test_audio_clone() {
        let audio1 = Audio::new(
            FileId::new(1, 0),
            "test.mp3".to_string(),
            "audio/mpeg".to_string(),
            120,
        );
        let audio2 = audio1.clone();
        assert_eq!(audio1, audio2);
    }

    #[test]
    fn test_audios_manager_new() {
        let manager = AudiosManager::new();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_audios_manager_default() {
        let manager = AudiosManager::default();
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_create_audio() {
        let mut manager = AudiosManager::new();
        let file_id = FileId::new(1, 0);
        manager.create_audio(file_id, "test.mp3".to_string(), "audio/mpeg".to_string(), 120);
        assert_eq!(manager.len(), 1);
        assert!(!manager.is_empty());
    }

    #[test]
    fn test_get_audio_duration() {
        let mut manager = AudiosManager::new();
        let file_id = FileId::new(1, 0);
        manager.create_audio(file_id, "test.mp3".to_string(), "audio/mpeg".to_string(), 180);
        assert_eq!(manager.get_audio_duration(file_id), 180);
        assert_eq!(manager.get_audio_duration(FileId::new(999, 0)), 0);
    }

    #[test]
    fn test_get_audio() {
        let mut manager = AudiosManager::new();
        let file_id = FileId::new(1, 0);
        manager.create_audio(file_id, "test.mp3".to_string(), "audio/mpeg".to_string(), 120);

        let audio = manager.get_audio(file_id);
        assert!(audio.is_some());
        assert_eq!(audio.unwrap().duration, 120);

        let missing = manager.get_audio(FileId::new(999, 0));
        assert!(missing.is_none());
    }

    #[test]
    fn test_remove_audio() {
        let mut manager = AudiosManager::new();
        let file_id = FileId::new(1, 0);
        manager.create_audio(file_id, "test.mp3".to_string(), "audio/mpeg".to_string(), 120);

        let removed = manager.remove_audio(file_id);
        assert!(removed.is_some());
        assert!(manager.is_empty());

        let removed_again = manager.remove_audio(file_id);
        assert!(removed_again.is_none());
    }

    #[test]
    fn test_multiple_audios() {
        let mut manager = AudiosManager::new();
        manager.create_audio(FileId::new(1, 0), "test1.mp3".to_string(), "audio/mpeg".to_string(), 120);
        manager.create_audio(FileId::new(2, 0), "test2.mp3".to_string(), "audio/mpeg".to_string(), 180);
        manager.create_audio(FileId::new(3, 0), "test3.mp3".to_string(), "audio/mpeg".to_string(), 240);

        assert_eq!(manager.len(), 3);
        assert_eq!(manager.get_audio_duration(FileId::new(1, 0)), 120);
        assert_eq!(manager.get_audio_duration(FileId::new(2, 0)), 180);
        assert_eq!(manager.get_audio_duration(FileId::new(3, 0)), 240);
    }

    #[test]
    fn test_audio_display() {
        let audio = Audio::new(
            FileId::new(1, 0),
            "test.mp3".to_string(),
            "audio/mpeg".to_string(),
            120,
        );
        let display = format!("{}", audio);
        assert!(display.contains("test.mp3"));
        assert!(display.contains("120"));
    }

    #[test]
    fn test_manager_clone() {
        let mut manager1 = AudiosManager::new();
        let file_id = FileId::new(1, 0);
        manager1.create_audio(file_id, "test.mp3".to_string(), "audio/mpeg".to_string(), 120);

        let manager2 = manager1.clone();
        assert_eq!(manager1.len(), manager2.len());
        assert_eq!(manager2.get_audio_duration(file_id), 120);
    }

    #[test]
    fn test_audio_with_metadata() {
        let mut audio = Audio::new(
            FileId::new(1, 0),
            "song.mp3".to_string(),
            "audio/mpeg".to_string(),
            200,
        );
        audio.set_title("Beautiful Song".to_string());
        audio.set_performer("Great Artist".to_string());
        audio.set_thumbnail(Some(PhotoSize::new("large".to_string())));

        assert_eq!(audio.title, "Beautiful Song");
        assert_eq!(audio.performer, "Great Artist");
        assert!(audio.thumbnail.is_some());
    }

    #[test]
    fn test_replace_audio() {
        let mut manager = AudiosManager::new();
        let file_id = FileId::new(1, 0);
        manager.create_audio(file_id, "original.mp3".to_string(), "audio/mpeg".to_string(), 100);
        assert_eq!(manager.get_audio_duration(file_id), 100);

        manager.create_audio(file_id, "replaced.mp3".to_string(), "audio/mpeg".to_string(), 200);
        assert_eq!(manager.len(), 1); // Still 1, replaced
        assert_eq!(manager.get_audio_duration(file_id), 200);
    }
}
