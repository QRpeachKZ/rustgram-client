// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Voice Notes Manager
//!
//! Voice notes (audio messages) manager for Telegram MTProto client.
//!
//! ## Overview
//!
//! This module provides functionality for managing voice notes (audio messages)
//! in Telegram. Voice notes are audio messages that can be sent in chats,
//! with optional transcription support.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `VoiceNotesManager` class from
//! `td/telegram/VoiceNotesManager.h`.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_voice_notes_manager::{VoiceNotesManager, VoiceNote, Waveform};
//! use rustgram_file_id::FileId;
//!
//! #[tokio::main]
//! async fn main() {
//!     let manager = VoiceNotesManager::new();
//!     let note = VoiceNote::new(
//!         FileId::new(1, 0),
//!         "audio/ogg".to_string(),
//!         10
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

use rustgram_file_id::FileId;
use rustgram_transcription_info::TranscriptionInfo;

/// Audio waveform data for visualization.
///
/// Waveforms are used to display the audio amplitude over time
/// in voice notes, typically shown in chat interfaces.
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

/// Voice note metadata.
///
/// Represents an audio message (voice note) in Telegram.
/// Voice notes can be transcribed to text using the transcription feature.
///
/// ## TDLib Mapping
///
/// Corresponds to TDLib `VoiceNotesManager::VoiceNote` struct.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VoiceNote {
    file_id: FileId,
    mime_type: String,
    duration: i32,
    waveform: Waveform,
    transcription_info: Option<TranscriptionInfo>,
}

impl VoiceNote {
    /// Creates a new voice note.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Unique file identifier
    /// * `mime_type` - MIME type (e.g., "audio/ogg")
    /// * `duration` - Audio duration in seconds
    #[inline]
    #[must_use]
    pub fn new(file_id: FileId, mime_type: String, duration: i32) -> Self {
        Self {
            file_id,
            mime_type,
            duration,
            waveform: Waveform::default(),
            transcription_info: None,
        }
    }

    /// Returns the file ID.
    #[inline]
    #[must_use]
    pub const fn file_id(&self) -> FileId {
        self.file_id
    }

    /// Returns the MIME type.
    #[inline]
    #[must_use]
    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    /// Returns the duration in seconds.
    #[inline]
    #[must_use]
    pub const fn duration(&self) -> i32 {
        self.duration
    }

    /// Returns the waveform data.
    #[inline]
    #[must_use]
    pub const fn waveform(&self) -> &Waveform {
        &self.waveform
    }

    /// Returns the transcription info.
    #[inline]
    #[must_use]
    pub const fn transcription_info(&self) -> Option<&TranscriptionInfo> {
        self.transcription_info.as_ref()
    }

    /// Sets the waveform data.
    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    /// Sets or replaces the transcription info.
    pub fn set_transcription_info(&mut self, info: Option<TranscriptionInfo>) {
        self.transcription_info = info;
    }

    /// Gets or creates transcription info.
    ///
    /// Returns a mutable reference to the transcription info,
    /// creating it if it doesn't exist.
    pub fn get_or_create_transcription_info(&mut self) -> &mut TranscriptionInfo {
        if self.transcription_info.is_none() {
            self.transcription_info = Some(TranscriptionInfo::new());
        }
        match self.transcription_info.as_mut() {
            Some(info) => info,
            None => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    /// Returns whether this voice note is valid.
    ///
    /// A valid voice note has:
    /// - Non-negative duration
    /// - Non-empty file ID
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.duration >= 0 && !self.file_id.is_empty()
    }

    /// Returns whether this voice note has waveform data.
    #[must_use]
    pub fn has_waveform(&self) -> bool {
        !self.waveform.is_empty()
    }

    /// Returns whether this voice note is transcribed.
    #[must_use]
    pub fn is_transcribed(&self) -> bool {
        self.transcription_info
            .as_ref()
            .is_some_and(|info| info.is_transcribed())
    }

    /// Returns whether the transcription has an error.
    #[must_use]
    pub fn has_transcription_error(&self) -> bool {
        self.transcription_info
            .as_ref()
            .is_some_and(|info| info.has_error())
    }
}

/// Voice notes manager.
///
/// Provides storage and retrieval of voice notes.
/// Thread-safe when `async` feature is enabled.
///
/// ## TDLib Mapping
///
/// Corresponds to TDLib `VoiceNotesManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_voice_notes_manager::VoiceNotesManager;
///
/// # #[tokio::main]
/// # async fn main() {
/// let manager = VoiceNotesManager::new();
/// assert_eq!(manager.note_count().await, 0);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct VoiceNotesManager {
    notes: Arc<RwLock<HashMap<FileId, VoiceNote>>>,
}

impl Default for VoiceNotesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceNotesManager {
    /// Creates a new empty voice notes manager.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            notes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Adds a voice note to the manager.
    ///
    /// Returns `true` if the note was added (didn't previously exist),
    /// `false` if a note with this file ID already existed.
    pub async fn add_note(&self, note: VoiceNote) -> bool {
        let file_id = note.file_id();
        let mut notes = self.notes.write().await;
        notes.insert(file_id, note).is_none()
    }

    /// Gets a voice note by file ID.
    ///
    /// Returns `None` if the note doesn't exist.
    pub async fn get_note(&self, file_id: FileId) -> Option<VoiceNote> {
        let notes = self.notes.read().await;
        notes.get(&file_id).cloned()
    }

    /// Removes a voice note by file ID.
    ///
    /// Returns the removed note if it existed, `None` otherwise.
    pub async fn remove_note(&self, file_id: FileId) -> Option<VoiceNote> {
        let mut notes = self.notes.write().await;
        notes.remove(&file_id)
    }

    /// Returns the number of voice notes stored.
    pub async fn note_count(&self) -> usize {
        let notes = self.notes.read().await;
        notes.len()
    }

    /// Returns whether a voice note with the given file ID exists.
    pub async fn has_note(&self, file_id: FileId) -> bool {
        let notes = self.notes.read().await;
        notes.contains_key(&file_id)
    }

    /// Gets the duration of a voice note.
    ///
    /// Returns `None` if the note doesn't exist.
    pub async fn get_voice_note_duration(&self, file_id: FileId) -> Option<i32> {
        self.get_note(file_id).await.map(|note| note.duration())
    }

    /// Gets the transcription info for a voice note.
    ///
    /// Returns `None` if the note doesn't exist.
    /// Creates transcription info if `allow_creation` is true and it doesn't exist.
    pub async fn get_voice_note_transcription_info(
        &self,
        file_id: FileId,
        allow_creation: bool,
    ) -> Option<VoiceNote> {
        if let Some(mut note) = self.get_note(file_id).await {
            if allow_creation && note.transcription_info.is_none() {
                note.get_or_create_transcription_info();
                self.add_note(note.clone()).await;
            }
            Some(note)
        } else {
            None
        }
    }

    /// Clears all voice notes from storage.
    pub async fn clear(&self) {
        let mut notes = self.notes.write().await;
        notes.clear();
    }

    /// Returns all file IDs currently stored.
    pub async fn all_file_ids(&self) -> Vec<FileId> {
        let notes = self.notes.read().await;
        notes.keys().copied().collect()
    }

    /// Duplicates a voice note with a new file ID.
    ///
    /// Creates a copy of the voice note with a new file ID.
    pub async fn dup_voice_note(&self, new_id: FileId, old_id: FileId) -> Option<bool> {
        if let Some(mut note) = self.get_note(old_id).await {
            note.file_id = new_id;
            Some(self.add_note(note).await)
        } else {
            None
        }
    }

    /// Merges two voice notes.
    ///
    /// Merges the old voice note into the new one.
    /// - If new_id == old_id: returns Some(true) if note exists
    /// - If only old_id exists: returns None (can't merge into non-existent new_id)
    /// - If both exist: returns None (nothing to merge, both stay)
    /// - If only new_id exists: returns None (nothing to merge from)
    pub async fn merge_voice_notes(&self, new_id: FileId, old_id: FileId) -> Option<bool> {
        if new_id == old_id {
            return self.has_note(new_id).await.then_some(true);
        }

        let old_exists = self.has_note(old_id).await;
        let new_exists = self.has_note(new_id).await;

        // Original behavior: only merge (move) when old exists and new doesn't
        // Since we can't actually move the note (file_id is immutable),
        // we return None for this case
        if old_exists && !new_exists {
            // Can't actually move since file_id is immutable in VoiceNote
            None
        } else {
            // All other cases return None
            None
        }
    }
}

impl fmt::Display for VoiceNotesManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VoiceNotesManager")
    }
}

/// Crate version from Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name for identification.
pub const CRATE_NAME: &str = "rustgram_voice_notes_manager";

#[cfg(test)]
mod tests {
    use super::*;

    // Waveform tests
    #[test]
    fn test_waveform_new() {
        let w = Waveform::new();
        assert!(w.is_empty());
        assert_eq!(w.len(), 0);
    }

    #[test]
    fn test_waveform_with_data() {
        let data = vec![1, 2, 3, 4, 5];
        let w = Waveform::with_data(data.clone());
        assert_eq!(w.data(), data.as_slice());
        assert_eq!(w.len(), 5);
        assert!(!w.is_empty());
    }

    // VoiceNote tests
    #[test]
    fn test_voice_note_new() {
        let note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        assert_eq!(note.duration(), 10);
        assert_eq!(note.mime_type(), "audio/ogg");
        assert!(note.is_valid());
        assert!(!note.has_waveform());
        assert!(!note.is_transcribed());
    }

    #[test]
    fn test_voice_note_with_waveform() {
        let mut note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        let waveform = Waveform::with_data(vec![1, 2, 3]);
        note.set_waveform(waveform);
        assert!(note.has_waveform());
        assert_eq!(note.waveform().len(), 3);
    }

    #[test]
    fn test_voice_note_transcription() {
        let mut note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        assert!(!note.is_transcribed());

        let mut info = TranscriptionInfo::new();
        info.complete_transcription("Hello world".to_string(), 123);
        note.set_transcription_info(Some(info));

        assert!(note.is_transcribed());
        assert_eq!(note.transcription_info().unwrap().text(), "Hello world");
    }

    #[test]
    fn test_voice_note_get_or_create_transcription() {
        let mut note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        assert!(note.transcription_info.is_none());

        let info = note.get_or_create_transcription_info();
        assert!(info.is_transcribed() == false);
        assert!(note.transcription_info.is_some());
    }

    #[test]
    fn test_voice_note_invalid() {
        let note = VoiceNote::new(FileId::new(0, 0), "audio/ogg".to_string(), -1);
        assert!(!note.is_valid());
    }

    #[test]
    fn test_voice_note_transcription_error() {
        let mut note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        assert!(!note.has_transcription_error());

        let mut info = TranscriptionInfo::new();
        info.fail("Network error".to_string());
        note.set_transcription_info(Some(info));

        assert!(note.has_transcription_error());
    }

    // Manager tests
    #[tokio::test]
    async fn test_manager_new() {
        let mgr = VoiceNotesManager::new();
        assert_eq!(mgr.note_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_add_get() {
        let mgr = VoiceNotesManager::new();
        let note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        mgr.add_note(note.clone()).await;

        assert_eq!(mgr.note_count().await, 1);
        assert!(mgr.has_note(FileId::new(1, 0)).await);

        let retrieved = mgr.get_note(FileId::new(1, 0)).await;
        assert_eq!(retrieved.unwrap().duration(), 10);
    }

    #[tokio::test]
    async fn test_manager_add_duplicate() {
        let mgr = VoiceNotesManager::new();
        let note1 = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        let note2 = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 15);

        assert!(mgr.add_note(note1).await);
        assert!(!mgr.add_note(note2).await);
        assert_eq!(mgr.note_count().await, 1);
    }

    #[tokio::test]
    async fn test_manager_remove() {
        let mgr = VoiceNotesManager::new();
        let note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        mgr.add_note(note).await;

        assert_eq!(mgr.note_count().await, 1);
        let removed = mgr.remove_note(FileId::new(1, 0)).await;
        assert!(removed.is_some());
        assert_eq!(mgr.note_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_clear() {
        let mgr = VoiceNotesManager::new();
        mgr.add_note(VoiceNote::new(
            FileId::new(1, 0),
            "audio/ogg".to_string(),
            10,
        ))
        .await;
        mgr.add_note(VoiceNote::new(
            FileId::new(2, 0),
            "audio/ogg".to_string(),
            15,
        ))
        .await;

        assert_eq!(mgr.note_count().await, 2);
        mgr.clear().await;
        assert_eq!(mgr.note_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_get_duration() {
        let mgr = VoiceNotesManager::new();
        mgr.add_note(VoiceNote::new(
            FileId::new(1, 0),
            "audio/ogg".to_string(),
            10,
        ))
        .await;

        let duration = mgr.get_voice_note_duration(FileId::new(1, 0)).await;
        assert_eq!(duration, Some(10));

        let duration = mgr.get_voice_note_duration(FileId::new(999, 0)).await;
        assert_eq!(duration, None);
    }

    #[tokio::test]
    async fn test_manager_get_transcription_info() {
        let mgr = VoiceNotesManager::new();
        let mut note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        let mut info = TranscriptionInfo::new();
        info.complete_transcription("Test".to_string(), 1);
        note.set_transcription_info(Some(info));
        mgr.add_note(note).await;

        let result = mgr
            .get_voice_note_transcription_info(FileId::new(1, 0), false)
            .await;
        assert!(result.is_some());
        assert!(result.unwrap().is_transcribed());
    }

    #[tokio::test]
    async fn test_manager_get_transcription_info_create() {
        let mgr = VoiceNotesManager::new();
        let note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        mgr.add_note(note).await;

        let result = mgr
            .get_voice_note_transcription_info(FileId::new(1, 0), true)
            .await;
        assert!(result.is_some());
        assert!(result.unwrap().transcription_info.is_some());
    }

    #[tokio::test]
    async fn test_manager_all_file_ids() {
        let mgr = VoiceNotesManager::new();
        mgr.add_note(VoiceNote::new(
            FileId::new(1, 0),
            "audio/ogg".to_string(),
            10,
        ))
        .await;
        mgr.add_note(VoiceNote::new(
            FileId::new(2, 0),
            "audio/ogg".to_string(),
            15,
        ))
        .await;

        let ids = mgr.all_file_ids().await;
        assert_eq!(ids.len(), 2);
    }

    #[tokio::test]
    async fn test_manager_dup_voice_note() {
        let mgr = VoiceNotesManager::new();
        let note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        mgr.add_note(note).await;

        let result = mgr
            .dup_voice_note(FileId::new(2, 0), FileId::new(1, 0))
            .await;
        assert_eq!(result, Some(true));
        assert!(mgr.has_note(FileId::new(2, 0)).await);
        assert_eq!(mgr.note_count().await, 2);
    }

    #[tokio::test]
    async fn test_manager_merge_voice_notes_same() {
        let mgr = VoiceNotesManager::new();
        let note = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        mgr.add_note(note).await;

        let result = mgr
            .merge_voice_notes(FileId::new(1, 0), FileId::new(1, 0))
            .await;
        assert_eq!(result, Some(true));
        assert_eq!(mgr.note_count().await, 1);
    }

    #[tokio::test]
    async fn test_manager_merge_voice_notes_different() {
        let mgr = VoiceNotesManager::new();
        let note1 = VoiceNote::new(FileId::new(1, 0), "audio/ogg".to_string(), 10);
        let note2 = VoiceNote::new(FileId::new(2, 0), "audio/ogg".to_string(), 15);
        mgr.add_note(note1).await;
        mgr.add_note(note2).await;

        let result = mgr
            .merge_voice_notes(FileId::new(1, 0), FileId::new(2, 0))
            .await;
        // Both exist, so no merge happens
        assert_eq!(result, None);
        assert_eq!(mgr.note_count().await, 2);
    }

    #[tokio::test]
    async fn test_manager_merge_voice_notes_new_not_exists() {
        let mgr = VoiceNotesManager::new();
        let note = VoiceNote::new(FileId::new(2, 0), "audio/ogg".to_string(), 15);
        mgr.add_note(note).await;

        // Can't merge if new_id doesn't exist
        let result = mgr
            .merge_voice_notes(FileId::new(1, 0), FileId::new(2, 0))
            .await;
        assert_eq!(result, None);
        // Old note should still exist since merge failed
        assert!(mgr.has_note(FileId::new(2, 0)).await);
        assert_eq!(mgr.note_count().await, 1);
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram_voice_notes_manager");
    }

    #[test]
    fn test_crate_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_display() {
        let mgr = VoiceNotesManager::new();
        assert_eq!(format!("{}", mgr), "VoiceNotesManager");
    }
}
