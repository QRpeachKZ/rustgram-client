//! # Rustgram VideoNotesManager
//!
//! Video notes (round video messages) manager for Telegram MTProto client.
//!
//! ## Overview
//!
//! This module provides functionality for managing video notes (round videos)
//! in Telegram. Video notes are the circular video messages that can be sent
//! in chats.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_video_notes_manager::{VideoNotesManager, VideoNote, Waveform};
//! use rustgram_file_id::FileId;
//! use rustgram_dimensions::Dimensions;
//!
//! #[tokio::main]
//! async fn main() {
//!     let manager = VideoNotesManager::new();
//!     let note = VideoNote::new(
//!         FileId::new(1, 0),
//!         10,
//!         Dimensions::from_wh(300, 300)
//!     );
//!     manager.add_note(note).await;
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;

use rustgram_dimensions::Dimensions;
use rustgram_file_id::FileId;
use rustgram_photo_size::PhotoSize;

/// Audio waveform data for visualization.
///
/// Waveforms are used to display the audio amplitude over time
/// in video notes, typically shown in chat interfaces.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Waveform {
    data: Vec<u8>,
}

impl Default for Waveform {
    fn default() -> Self {
        Self::new()
    }
}

impl Waveform {
    /// Creates a new empty waveform.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Creates a waveform from the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw waveform byte data
    #[inline]
    #[must_use]
    pub fn with_data(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Returns the waveform data.
    #[inline]
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the length of the waveform data.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the waveform is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Video note metadata.
///
/// Represents a round video message (video note) in Telegram.
/// Video notes are typically short, circular videos that play
/// automatically in chat.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VideoNote {
    file_id: FileId,
    duration: i32,
    dimensions: Dimensions,
    waveform: Waveform,
    thumbnail: PhotoSize,
    minithumbnail: String,
}

impl VideoNote {
    /// Creates a new video note.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique file identifier
    /// * `duration` - Video duration in seconds
    /// * `dimensions` - Video dimensions (should be square for video notes)
    #[inline]
    #[must_use]
    pub fn new(file_id: FileId, duration: i32, dimensions: Dimensions) -> Self {
        Self {
            file_id,
            duration,
            dimensions,
            waveform: Waveform::default(),
            thumbnail: PhotoSize::new(String::new()),
            minithumbnail: String::new(),
        }
    }

    /// Returns the file ID.
    #[inline]
    #[must_use]
    pub const fn file_id(&self) -> FileId {
        self.file_id
    }

    /// Returns the duration in seconds.
    #[inline]
    #[must_use]
    pub const fn duration(&self) -> i32 {
        self.duration
    }

    /// Returns the video dimensions.
    #[inline]
    #[must_use]
    pub const fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    /// Returns the waveform data.
    #[inline]
    #[must_use]
    pub const fn waveform(&self) -> &Waveform {
        &self.waveform
    }

    /// Returns the thumbnail.
    #[inline]
    #[must_use]
    pub const fn thumbnail(&self) -> &PhotoSize {
        &self.thumbnail
    }

    /// Returns the minithumbnail (blurhash).
    #[inline]
    #[must_use]
    pub fn minithumbnail(&self) -> &str {
        &self.minithumbnail
    }

    /// Sets the waveform data.
    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    /// Sets the thumbnail.
    pub fn set_thumbnail(&mut self, thumbnail: PhotoSize) {
        self.thumbnail = thumbnail;
    }

    /// Sets the minithumbnail (blurhash).
    pub fn set_minithumbnail(&mut self, minithumbnail: String) {
        self.minithumbnail = minithumbnail;
    }

    /// Returns whether this video note is valid.
    ///
    /// A valid video note has:
    /// - Positive duration
    /// - Non-empty file ID
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.duration > 0 && !self.file_id.is_empty()
    }

    /// Returns whether this video note has waveform data.
    #[must_use]
    pub fn has_waveform(&self) -> bool {
        !self.waveform.is_empty()
    }
}

/// Video notes manager.
///
/// Provides thread-safe storage and retrieval of video notes.
/// Uses `Arc<RwLock<T>>` for concurrent access.
///
/// # Example
///
/// ```rust
/// use rustgram_video_notes_manager::VideoNotesManager;
///
/// # #[tokio::main]
/// # async fn main() {
/// let manager = VideoNotesManager::new();
/// assert_eq!(manager.note_count().await, 0);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct VideoNotesManager {
    notes: Arc<RwLock<HashMap<FileId, VideoNote>>>,
}

impl Default for VideoNotesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoNotesManager {
    /// Creates a new empty video notes manager.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            notes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Adds a video note to the manager.
    ///
    /// Returns `true` if the note was added (didn't previously exist),
    /// `false` if a note with this file ID already existed.
    pub async fn add_note(&self, note: VideoNote) -> bool {
        let file_id = note.file_id();
        let mut notes = self.notes.write().await;
        notes.insert(file_id, note).is_none()
    }

    /// Gets a video note by file ID.
    ///
    /// Returns `None` if the note doesn't exist.
    pub async fn get_note(&self, file_id: FileId) -> Option<VideoNote> {
        let notes = self.notes.read().await;
        notes.get(&file_id).cloned()
    }

    /// Removes a video note by file ID.
    ///
    /// Returns the removed note if it existed, `None` otherwise.
    pub async fn remove_note(&self, file_id: FileId) -> Option<VideoNote> {
        let mut notes = self.notes.write().await;
        notes.remove(&file_id)
    }

    /// Returns the number of video notes stored.
    pub async fn note_count(&self) -> usize {
        let notes = self.notes.read().await;
        notes.len()
    }

    /// Returns whether a video note with the given file ID exists.
    pub async fn has_note(&self, file_id: FileId) -> bool {
        let notes = self.notes.read().await;
        notes.contains_key(&file_id)
    }

    /// Clears all video notes from storage.
    pub async fn clear(&self) {
        let mut notes = self.notes.write().await;
        notes.clear();
    }

    /// Returns all file IDs currently stored.
    pub async fn all_file_ids(&self) -> Vec<FileId> {
        let notes = self.notes.read().await;
        notes.keys().copied().collect()
    }
}

impl fmt::Display for VideoNotesManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VideoNotesManager")
    }
}

/// Crate version from Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name for identification.
pub const CRATE_NAME: &str = "rustgram_video_notes_manager";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveform_new() {
        let w = Waveform::new();
        assert!(w.is_empty());
    }

    #[test]
    fn test_waveform_with_data() {
        let data = vec![1, 2, 3, 4, 5];
        let w = Waveform::with_data(data.clone());
        assert_eq!(w.data(), data.as_slice());
        assert_eq!(w.len(), 5);
        assert!(!w.is_empty());
    }

    #[test]
    fn test_video_note_new() {
        let note = VideoNote::new(FileId::new(1, 0), 10, Dimensions::from_wh(300, 300));
        assert_eq!(note.duration(), 10);
        assert!(note.is_valid());
        assert!(!note.has_waveform());
    }

    #[test]
    fn test_video_note_with_waveform() {
        let mut note = VideoNote::new(FileId::new(1, 0), 10, Dimensions::from_wh(300, 300));
        let waveform = Waveform::with_data(vec![1, 2, 3]);
        note.set_waveform(waveform);
        assert!(note.has_waveform());
        assert_eq!(note.waveform().len(), 3);
    }

    #[tokio::test]
    async fn test_manager_add_get() {
        let mgr = VideoNotesManager::new();
        let note = VideoNote::new(FileId::new(1, 0), 10, Dimensions::from_wh(300, 300));
        mgr.add_note(note).await;
        assert_eq!(mgr.note_count().await, 1);
        assert!(mgr.has_note(FileId::new(1, 0)).await);
    }

    #[tokio::test]
    async fn test_manager_remove() {
        let mgr = VideoNotesManager::new();
        let note = VideoNote::new(FileId::new(1, 0), 10, Dimensions::from_wh(300, 300));
        mgr.add_note(note).await;
        assert_eq!(mgr.note_count().await, 1);

        let removed = mgr.remove_note(FileId::new(1, 0)).await;
        assert!(removed.is_some());
        assert_eq!(mgr.note_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_clear() {
        let mgr = VideoNotesManager::new();
        mgr.add_note(VideoNote::new(
            FileId::new(1, 0),
            10,
            Dimensions::from_wh(300, 300),
        ))
        .await;
        mgr.add_note(VideoNote::new(
            FileId::new(2, 0),
            15,
            Dimensions::from_wh(300, 300),
        ))
        .await;
        assert_eq!(mgr.note_count().await, 2);

        mgr.clear().await;
        assert_eq!(mgr.note_count().await, 0);
    }

    #[tokio::test]
    async fn test_all_file_ids() {
        let mgr = VideoNotesManager::new();
        mgr.add_note(VideoNote::new(
            FileId::new(1, 0),
            10,
            Dimensions::from_wh(300, 300),
        ))
        .await;
        mgr.add_note(VideoNote::new(
            FileId::new(2, 0),
            15,
            Dimensions::from_wh(300, 300),
        ))
        .await;

        let ids = mgr.all_file_ids().await;
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram_video_notes_manager");
    }
}
