// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Background Manager
//!
//! Manager for Telegram chat backgrounds.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `BackgroundManager` class from `td/telegram/BackgroundManager.h`.
//!
//! ## Overview
//!
//! The BackgroundManager handles:
//! - Background storage and retrieval
//! - Background installation and removal
//! - Dialog-specific backgrounds
//! - File upload for custom backgrounds
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_background_manager::{BackgroundManager, Background, BackgroundError};
//! use rustgram_background_type::BackgroundType;
//! use rustgram_background_id::BackgroundId;
//!
//! let manager = BackgroundManager::new();
//! let bg_id = BackgroundId::new(12345);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
// RwLock poisoning is rare and panic is acceptable for manager pattern
#![allow(clippy::unwrap_used)]

use rustgram_background_id::BackgroundId;
use rustgram_background_type::BackgroundType;
use rustgram_dialog_id::DialogId;
use rustgram_file_id::FileId;
use rustgram_file_source_id::FileSourceId;
use rustgram_file_upload_id::FileUploadId;
use rustgram_logevent::LogEvent;
use rustgram_types::MessageId;
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, RwLock};

/// Errors that can occur in BackgroundManager operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundError {
    /// Background not found.
    BackgroundNotFound(BackgroundId),
    /// Invalid background type.
    InvalidBackgroundType,
    /// File operation failed.
    FileOperationFailed(String),
    /// Upload failed.
    UploadFailed(String),
    /// Dialog not found.
    DialogNotFound(DialogId),
    /// Invalid access hash.
    InvalidAccessHash,
}

impl fmt::Display for BackgroundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackgroundNotFound(id) => write!(f, "Background not found: {}", id.get()),
            Self::InvalidBackgroundType => write!(f, "Invalid background type"),
            Self::FileOperationFailed(msg) => write!(f, "File operation failed: {}", msg),
            Self::UploadFailed(msg) => write!(f, "Upload failed: {}", msg),
            Self::DialogNotFound(id) => write!(f, "Dialog not found: {}", id),
            Self::InvalidAccessHash => write!(f, "Invalid access hash"),
        }
    }
}

impl std::error::Error for BackgroundError {}

/// Result type for BackgroundManager operations.
pub type Result<T> = std::result::Result<T, BackgroundError>;

/// A Telegram chat background.
///
/// Represents a background that can be applied to chats.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib `Background` struct.
///
/// # Example
///
/// ```rust
/// use rustgram_background_manager::Background;
/// use rustgram_background_id::BackgroundId;
/// use rustgram_background_type::BackgroundType;
/// use rustgram_file_id::FileId;
/// use rustgram_file_source_id::FileSourceId;
///
/// let background = Background::new(
///     BackgroundId::new(12345),
///     67890,
///     "solid".to_string(),
///     FileId::new(1, 0),
///     BackgroundType::wallpaper(false, false, 0),
///     FileSourceId::new(42),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Background {
    /// Unique background identifier.
    pub id: BackgroundId,
    /// Access hash for API calls.
    pub access_hash: i64,
    /// Background name.
    pub name: String,
    /// Associated file ID.
    pub file_id: FileId,
    /// Whether the current user created this background.
    pub is_creator: bool,
    /// Whether this is a default background.
    pub is_default: bool,
    /// Whether this is a dark background.
    pub is_dark: bool,
    /// Whether this background has a new local ID.
    pub has_new_local_id: bool,
    /// Background type configuration.
    pub type_: BackgroundType,
    /// File source ID for downloads.
    pub file_source_id: FileSourceId,
}

impl Background {
    /// Creates a new background.
    ///
    /// # Arguments
    ///
    /// * `id` - Background ID
    /// * `access_hash` - Access hash for API calls
    /// * `name` - Background name
    /// * `file_id` - Associated file ID
    /// * `type_` - Background type
    /// * `file_source_id` - File source ID
    #[must_use]
    pub fn new(
        id: BackgroundId,
        access_hash: i64,
        name: String,
        file_id: FileId,
        type_: BackgroundType,
        file_source_id: FileSourceId,
    ) -> Self {
        Self {
            id,
            access_hash,
            name,
            file_id,
            is_creator: false,
            is_default: false,
            is_dark: false,
            has_new_local_id: true,
            type_,
            file_source_id,
        }
    }

    /// Creates a default background.
    #[must_use]
    pub fn default_background() -> Self {
        Self {
            id: BackgroundId::default(),
            access_hash: 0,
            name: String::new(),
            file_id: FileId::empty(),
            is_creator: false,
            is_default: true,
            is_dark: false,
            has_new_local_id: false,
            type_: BackgroundType::default(),
            file_source_id: FileSourceId::new(0),
        }
    }

    /// Sets the creator flag.
    #[must_use]
    pub const fn with_creator(mut self, is_creator: bool) -> Self {
        self.is_creator = is_creator;
        self
    }

    /// Sets the default flag.
    #[must_use]
    pub const fn with_default(mut self, is_default: bool) -> Self {
        self.is_default = is_default;
        self
    }

    /// Sets the dark flag.
    #[must_use]
    pub const fn with_dark(mut self, is_dark: bool) -> Self {
        self.is_dark = is_dark;
        self
    }

    /// Checks if this background is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.id.is_valid() && self.file_id.is_valid()
    }

    /// Returns the MIME type for this background.
    #[must_use]
    pub fn mime_type(&self) -> &'static str {
        self.type_.mime_type()
    }
}

impl LogEvent for Background {
    fn log_event_id(&self) -> u64 {
        self.id.get() as u64
    }

    fn set_log_event_id(&mut self, id: u64) {
        self.id = BackgroundId::new(id as i64);
    }
}

impl Default for Background {
    fn default() -> Self {
        Self::default_background()
    }
}

/// Dialog background configuration.
///
/// Stores background settings for a specific dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogBackground {
    /// Dialog ID.
    pub dialog_id: DialogId,
    /// Background ID.
    pub background_id: BackgroundId,
    /// Background type.
    pub type_: BackgroundType,
    /// Dark theme dimming (0-100).
    pub dark_theme_dimming: i32,
    /// Message ID of the background message.
    pub message_id: MessageId,
}

impl DialogBackground {
    /// Creates a new dialog background.
    #[must_use]
    pub fn new(
        dialog_id: DialogId,
        background_id: BackgroundId,
        type_: BackgroundType,
        dark_theme_dimming: i32,
        message_id: MessageId,
    ) -> Self {
        Self {
            dialog_id,
            background_id,
            type_,
            dark_theme_dimming,
            message_id,
        }
    }
}

/// Background upload callback info.
///
/// Stores information about an in-progress background upload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadedFileInfo {
    /// Background type.
    pub type_: BackgroundType,
    /// Dialog ID (if setting dialog background).
    pub dialog_id: Option<DialogId>,
    /// For dark theme.
    pub for_dark_theme: bool,
}

impl UploadedFileInfo {
    /// Creates new upload info.
    #[must_use]
    pub fn new(type_: BackgroundType, for_dark_theme: bool) -> Self {
        Self {
            type_,
            dialog_id: None,
            for_dark_theme,
        }
    }

    /// Creates upload info for dialog background.
    #[must_use]
    pub fn for_dialog(type_: BackgroundType, dialog_id: DialogId, for_dark_theme: bool) -> Self {
        Self {
            type_,
            dialog_id: Some(dialog_id),
            for_dark_theme,
        }
    }
}

/// Manager for Telegram chat backgrounds.
///
/// Handles background storage, installation, and management.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib `BackgroundManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_background_manager::BackgroundManager;
///
/// let manager = BackgroundManager::new();
/// ```
#[derive(Debug)]
pub struct BackgroundManager {
    /// Storage of all backgrounds by ID.
    backgrounds: Arc<RwLock<HashMap<BackgroundId, Background>>>,
    /// Background IDs by name.
    name_to_background_id: Arc<RwLock<HashMap<String, BackgroundId>>>,
    /// Background IDs by file ID.
    file_id_to_background_id: Arc<RwLock<HashMap<FileId, BackgroundId>>>,
    /// Background ID to file source ID mapping.
    background_id_to_file_source_id: Arc<RwLock<HashMap<BackgroundId, FileSourceId>>>,
    /// Current background ID for light/dark themes.
    set_background_id: Arc<RwLock<[BackgroundId; 2]>>,
    /// Current background type for light/dark themes.
    set_background_type: Arc<RwLock<[BackgroundType; 2]>>,
    /// Dialog-specific backgrounds.
    dialog_backgrounds: Arc<RwLock<HashMap<DialogId, DialogBackground>>>,
    /// Files currently being uploaded.
    being_uploaded_files: Arc<RwLock<HashMap<FileUploadId, UploadedFileInfo>>>,
    /// Local background ID counter.
    max_local_background_id: Arc<AtomicI64>,
}

impl Default for BackgroundManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BackgroundManager {
    /// Creates a new BackgroundManager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_background_manager::BackgroundManager;
    ///
    /// let manager = BackgroundManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            backgrounds: Arc::new(RwLock::new(HashMap::new())),
            name_to_background_id: Arc::new(RwLock::new(HashMap::new())),
            file_id_to_background_id: Arc::new(RwLock::new(HashMap::new())),
            background_id_to_file_source_id: Arc::new(RwLock::new(HashMap::new())),
            set_background_id: Arc::new(RwLock::new([
                BackgroundId::default(),
                BackgroundId::default(),
            ])),
            set_background_type: Arc::new(RwLock::new([
                BackgroundType::default(),
                BackgroundType::default(),
            ])),
            dialog_backgrounds: Arc::new(RwLock::new(HashMap::new())),
            being_uploaded_files: Arc::new(RwLock::new(HashMap::new())),
            max_local_background_id: Arc::new(AtomicI64::new(0)),
        }
    }

    /// Gets all backgrounds for a theme.
    ///
    /// # Arguments
    ///
    /// * `for_dark_theme` - Whether to get backgrounds for dark theme
    #[must_use]
    pub fn get_backgrounds(&self, _for_dark_theme: bool) -> Vec<Background> {
        let backgrounds = self.backgrounds.read().unwrap();
        backgrounds.values().cloned().collect()
    }

    /// Gets a background by ID.
    ///
    /// # Arguments
    ///
    /// * `background_id` - Background ID
    ///
    /// # Errors
    ///
    /// Returns `BackgroundError::BackgroundNotFound` if the background doesn't exist.
    pub fn get_background(&self, background_id: BackgroundId) -> Result<Background> {
        let backgrounds = self.backgrounds.read().unwrap();
        backgrounds
            .get(&background_id)
            .cloned()
            .ok_or(BackgroundError::BackgroundNotFound(background_id))
    }

    /// Searches for a background by name.
    ///
    /// # Arguments
    ///
    /// * `name` - Background name
    ///
    /// # Errors
    ///
    /// Returns `BackgroundError::BackgroundNotFound` if not found.
    pub fn search_background(&self, name: &str) -> Result<(BackgroundId, BackgroundType)> {
        let name_map = self.name_to_background_id.read().unwrap();
        let bg_id = name_map
            .get(name)
            .ok_or_else(|| BackgroundError::BackgroundNotFound(BackgroundId::default()))?;
        let backgrounds = self.backgrounds.read().unwrap();
        let bg = backgrounds
            .get(bg_id)
            .ok_or(BackgroundError::BackgroundNotFound(*bg_id))?;
        Ok((*bg_id, bg.type_.clone()))
    }

    /// Sets a background as the current background.
    ///
    /// # Arguments
    ///
    /// * `background_id` - Background ID
    /// * `type_` - Background type
    /// * `for_dark_theme` - Whether this is for dark theme
    ///
    /// # Errors
    ///
    /// Returns `BackgroundError::BackgroundNotFound` if the background doesn't exist.
    pub fn set_background(
        &self,
        background_id: BackgroundId,
        type_: BackgroundType,
        for_dark_theme: bool,
    ) -> Result<()> {
        // Verify background exists
        let _bg = self.get_background(background_id)?;

        let theme_index = if for_dark_theme { 1 } else { 0 };

        let mut set_id = self.set_background_id.write().unwrap();
        let mut set_type = self.set_background_type.write().unwrap();

        set_id[theme_index] = background_id;
        set_type[theme_index] = type_;

        Ok(())
    }

    /// Gets the current background ID.
    ///
    /// # Arguments
    ///
    /// * `for_dark_theme` - Whether to get dark theme background
    #[must_use]
    pub fn get_current_background_id(&self, for_dark_theme: bool) -> BackgroundId {
        let set_id = self.set_background_id.read().unwrap();
        set_id[if for_dark_theme { 1 } else { 0 }]
    }

    /// Gets the current background type.
    ///
    /// # Arguments
    ///
    /// * `for_dark_theme` - Whether to get dark theme background type
    #[must_use]
    pub fn get_current_background_type(&self, for_dark_theme: bool) -> BackgroundType {
        let set_type = self.set_background_type.read().unwrap();
        set_type[if for_dark_theme { 1 } else { 0 }].clone()
    }

    /// Removes a background.
    ///
    /// # Arguments
    ///
    /// * `background_id` - Background ID to remove
    ///
    /// # Errors
    ///
    /// Returns `BackgroundError::BackgroundNotFound` if the background doesn't exist.
    pub fn remove_background(&self, background_id: BackgroundId) -> Result<()> {
        let mut backgrounds = self.backgrounds.write().unwrap();
        let bg = backgrounds
            .remove(&background_id)
            .ok_or(BackgroundError::BackgroundNotFound(background_id))?;

        // Remove from indexes
        let mut name_map = self.name_to_background_id.write().unwrap();
        name_map.remove(&bg.name);

        let mut file_map = self.file_id_to_background_id.write().unwrap();
        file_map.remove(&bg.file_id);

        let mut source_map = self.background_id_to_file_source_id.write().unwrap();
        source_map.remove(&background_id);

        Ok(())
    }

    /// Resets all backgrounds.
    pub fn reset_backgrounds(&self) {
        let mut backgrounds = self.backgrounds.write().unwrap();
        let mut name_map = self.name_to_background_id.write().unwrap();
        let mut file_map = self.file_id_to_background_id.write().unwrap();
        let mut source_map = self.background_id_to_file_source_id.write().unwrap();

        backgrounds.clear();
        name_map.clear();
        file_map.clear();
        source_map.clear();

        let mut set_id = self.set_background_id.write().unwrap();
        let mut set_type = self.set_background_type.write().unwrap();

        set_id[0] = BackgroundId::default();
        set_id[1] = BackgroundId::default();
        set_type[0] = BackgroundType::default();
        set_type[1] = BackgroundType::default();
    }

    /// Adds a background to storage.
    ///
    /// # Arguments
    ///
    /// * `background` - Background to add
    /// * `replace_type` - Whether to replace existing background type
    pub fn add_background(&self, background: Background, replace_type: bool) {
        let mut backgrounds = self.backgrounds.write().unwrap();
        let mut name_map = self.name_to_background_id.write().unwrap();
        let mut file_map = self.file_id_to_background_id.write().unwrap();
        let mut source_map = self.background_id_to_file_source_id.write().unwrap();

        let bg_id = background.id;

        if replace_type {
            backgrounds.insert(bg_id, background.clone());
        } else {
            backgrounds.entry(bg_id).or_insert(background.clone());
        }

        name_map.insert(background.name.clone(), bg_id);
        file_map.insert(background.file_id, bg_id);
        source_map.insert(bg_id, background.file_source_id);
    }

    /// Gets file source ID for a background.
    ///
    /// # Arguments
    ///
    /// * `background_id` - Background ID
    /// * `access_hash` - Access hash for the background
    ///
    /// # Errors
    ///
    /// Returns `BackgroundError::BackgroundNotFound` if not found.
    pub fn get_background_file_source_id(
        &self,
        background_id: BackgroundId,
        _access_hash: i64,
    ) -> Result<FileSourceId> {
        let source_map = self.background_id_to_file_source_id.read().unwrap();
        source_map
            .get(&background_id)
            .copied()
            .ok_or(BackgroundError::BackgroundNotFound(background_id))
    }

    /// Sets dialog background.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `background_id` - Background ID
    /// * `type_` - Background type
    /// * `dark_theme_dimming` - Dark theme dimming (0-100)
    /// * `message_id` - Message ID
    ///
    /// # Errors
    ///
    /// Returns `BackgroundError::BackgroundNotFound` if the background doesn't exist.
    pub fn set_dialog_background(
        &self,
        dialog_id: DialogId,
        background_id: BackgroundId,
        type_: BackgroundType,
        dark_theme_dimming: i32,
        message_id: MessageId,
    ) -> Result<()> {
        // Verify background exists
        let _bg = self.get_background(background_id)?;

        let dialog_bg = DialogBackground::new(
            dialog_id,
            background_id,
            type_,
            dark_theme_dimming,
            message_id,
        );

        let mut dialog_bgs = self.dialog_backgrounds.write().unwrap();
        dialog_bgs.insert(dialog_id, dialog_bg);

        Ok(())
    }

    /// Deletes dialog background.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    pub fn delete_dialog_background(&self, dialog_id: DialogId) {
        let mut dialog_bgs = self.dialog_backgrounds.write().unwrap();
        dialog_bgs.remove(&dialog_id);
    }

    /// Gets dialog background.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    #[must_use]
    pub fn get_dialog_background(&self, dialog_id: DialogId) -> Option<DialogBackground> {
        let dialog_bgs = self.dialog_backgrounds.read().unwrap();
        dialog_bgs.get(&dialog_id).cloned()
    }

    /// Registers a background file upload.
    ///
    /// # Arguments
    ///
    /// * `file_upload_id` - File upload ID
    /// * `info` - Upload info
    pub fn on_uploaded_background_file(
        &self,
        file_upload_id: FileUploadId,
        info: UploadedFileInfo,
    ) {
        if let Ok(mut uploads) = self.being_uploaded_files.write() {
            uploads.insert(file_upload_id, info);
        }
    }

    /// Completes a background file upload.
    ///
    /// # Arguments
    ///
    /// * `file_upload_id` - File upload ID
    ///
    /// Returns the upload info if found.
    pub fn complete_upload(&self, file_upload_id: FileUploadId) -> Option<UploadedFileInfo> {
        let mut uploads = self.being_uploaded_files.write().ok()?;
        uploads.remove(&file_upload_id)
    }

    /// Gets the next local background ID.
    #[must_use]
    pub fn get_next_local_background_id(&self) -> BackgroundId {
        let id = self.max_local_background_id.fetch_add(1, Ordering::SeqCst) + 1;
        BackgroundId::new(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_background_type::BackgroundFill;

    fn create_test_background(id: i64, name: &str) -> Background {
        Background::new(
            BackgroundId::new(id),
            id * 1000,
            name.to_string(),
            FileId::new(id as i32, 0),
            BackgroundType::wallpaper(false, false, 0),
            FileSourceId::new(id as i32),
        )
    }

    #[test]
    fn test_manager_new() {
        let manager = BackgroundManager::new();
        assert_eq!(manager.get_backgrounds(false).len(), 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = BackgroundManager::default();
        assert_eq!(manager.get_backgrounds(true).len(), 0);
    }

    #[test]
    fn test_add_background() {
        let manager = BackgroundManager::new();
        let bg = create_test_background(123, "test_bg");

        manager.add_background(bg.clone(), false);

        let retrieved = manager.get_background(bg.id).unwrap();
        assert_eq!(retrieved.id, bg.id);
        assert_eq!(retrieved.name, bg.name);
    }

    #[test]
    fn test_add_background_with_replace() {
        let manager = BackgroundManager::new();

        let bg1 = create_test_background(123, "test_bg");
        let bg2 = Background::new(
            bg1.id,
            bg1.access_hash,
            "renamed".to_string(),
            bg1.file_id,
            BackgroundType::fill(BackgroundFill::solid(0xFF0000), 50),
            bg1.file_source_id,
        );

        manager.add_background(bg1.clone(), false);
        manager.add_background(bg2, true);

        let retrieved = manager.get_background(bg1.id).unwrap();
        assert_eq!(retrieved.name, "renamed");
    }

    #[test]
    fn test_get_background_not_found() {
        let manager = BackgroundManager::new();
        let result = manager.get_background(BackgroundId::new(999));
        assert_eq!(
            result,
            Err(BackgroundError::BackgroundNotFound(BackgroundId::new(999)))
        );
    }

    #[test]
    fn test_search_background() {
        let manager = BackgroundManager::new();
        let bg = create_test_background(123, "solid");

        manager.add_background(bg, false);

        let (id, _type_) = manager.search_background("solid").unwrap();
        assert_eq!(id, BackgroundId::new(123));
    }

    #[test]
    fn test_search_background_not_found() {
        let manager = BackgroundManager::new();
        let result = manager.search_background("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_set_background() {
        let manager = BackgroundManager::new();
        let bg = create_test_background(123, "test_bg");

        manager.add_background(bg.clone(), false);

        manager
            .set_background(bg.id, bg.type_.clone(), false)
            .unwrap();

        assert_eq!(manager.get_current_background_id(false), bg.id);
        assert_eq!(manager.get_current_background_type(false), bg.type_);
    }

    #[test]
    fn test_set_background_not_found() {
        let manager = BackgroundManager::new();
        let result =
            manager.set_background(BackgroundId::new(999), BackgroundType::default(), false);
        assert_eq!(
            result,
            Err(BackgroundError::BackgroundNotFound(BackgroundId::new(999)))
        );
    }

    #[test]
    fn test_remove_background() {
        let manager = BackgroundManager::new();
        let bg = create_test_background(123, "test_bg");

        manager.add_background(bg.clone(), false);
        manager.remove_background(bg.id).unwrap();

        let result = manager.get_background(bg.id);
        assert_eq!(result, Err(BackgroundError::BackgroundNotFound(bg.id)));
    }

    #[test]
    fn test_remove_background_not_found() {
        let manager = BackgroundManager::new();
        let result = manager.remove_background(BackgroundId::new(999));
        assert_eq!(
            result,
            Err(BackgroundError::BackgroundNotFound(BackgroundId::new(999)))
        );
    }

    #[test]
    fn test_reset_backgrounds() {
        let manager = BackgroundManager::new();
        let bg1 = create_test_background(123, "bg1");
        let bg2 = create_test_background(456, "bg2");

        manager.add_background(bg1, false);
        manager.add_background(bg2, false);

        manager.reset_backgrounds();

        assert_eq!(manager.get_backgrounds(false).len(), 0);
        assert_eq!(
            manager.get_current_background_id(false),
            BackgroundId::default()
        );
    }

    #[test]
    fn test_set_dialog_background() {
        let manager = BackgroundManager::new();
        let bg = create_test_background(123, "test_bg");
        let dialog_id = DialogId::new(1234567890);

        manager.add_background(bg.clone(), false);

        manager
            .set_dialog_background(
                dialog_id,
                bg.id,
                bg.type_.clone(),
                50,
                MessageId::new(1048576).unwrap(),
            )
            .unwrap();

        let dialog_bg = manager.get_dialog_background(dialog_id).unwrap();
        assert_eq!(dialog_bg.dialog_id, dialog_id);
        assert_eq!(dialog_bg.background_id, bg.id);
    }

    #[test]
    fn test_delete_dialog_background() {
        let manager = BackgroundManager::new();
        let bg = create_test_background(123, "test_bg");
        let dialog_id = DialogId::new(1234567890);

        manager.add_background(bg.clone(), false);

        manager
            .set_dialog_background(
                dialog_id,
                bg.id,
                bg.type_.clone(),
                50,
                MessageId::new(1048576).unwrap(),
            )
            .unwrap();

        manager.delete_dialog_background(dialog_id);

        assert!(manager.get_dialog_background(dialog_id).is_none());
    }

    #[test]
    fn test_get_background_file_source_id() {
        let manager = BackgroundManager::new();
        let bg = create_test_background(123, "test_bg");

        manager.add_background(bg.clone(), false);

        let source_id = manager
            .get_background_file_source_id(bg.id, bg.access_hash)
            .unwrap();
        assert_eq!(source_id, bg.file_source_id);
    }

    #[test]
    fn test_get_background_file_source_id_not_found() {
        let manager = BackgroundManager::new();
        let result = manager.get_background_file_source_id(BackgroundId::new(999), 0);
        assert_eq!(
            result,
            Err(BackgroundError::BackgroundNotFound(BackgroundId::new(999)))
        );
    }

    #[test]
    fn test_uploaded_file_info() {
        let info = UploadedFileInfo::new(BackgroundType::wallpaper(true, false, 50), false);
        assert!(info.dialog_id.is_none());
        assert!(!info.for_dark_theme);
    }

    #[test]
    fn test_uploaded_file_info_for_dialog() {
        let dialog_id = DialogId::new(1234567890);
        let info = UploadedFileInfo::for_dialog(
            BackgroundType::wallpaper(false, true, 75),
            dialog_id,
            true,
        );
        assert_eq!(info.dialog_id, Some(dialog_id));
        assert!(info.for_dark_theme);
    }

    #[test]
    fn test_on_uploaded_background_file() {
        let manager = BackgroundManager::new();
        let file_id = FileId::new(1, 0);
        let upload_id = FileUploadId::new(file_id, 123);
        let info = UploadedFileInfo::new(BackgroundType::default(), false);

        manager.on_uploaded_background_file(upload_id, info.clone());

        let completed = manager.complete_upload(upload_id).unwrap();
        assert_eq!(completed.dialog_id, info.dialog_id);
    }

    #[test]
    fn test_get_next_local_background_id() {
        let manager = BackgroundManager::new();

        let id1 = manager.get_next_local_background_id();
        let id2 = manager.get_next_local_background_id();

        assert!(id2.get() > id1.get());
    }

    #[test]
    fn test_background_is_valid() {
        let bg = create_test_background(123, "test_bg");
        assert!(bg.is_valid());
    }

    #[test]
    fn test_background_default_is_valid() {
        let bg = Background::default_background();
        assert!(!bg.is_valid());
    }

    #[test]
    fn test_background_mime_type() {
        let bg = create_test_background(123, "test_bg");
        assert_eq!(bg.mime_type(), "image/jpeg");
    }

    #[test]
    fn test_background_with_creator() {
        let bg = create_test_background(123, "test_bg").with_creator(true);
        assert!(bg.is_creator);
    }

    #[test]
    fn test_background_with_default() {
        let bg = create_test_background(123, "test_bg").with_default(true);
        assert!(bg.is_default);
    }

    #[test]
    fn test_background_with_dark() {
        let bg = create_test_background(123, "test_bg").with_dark(true);
        assert!(bg.is_dark);
    }

    #[test]
    fn test_background_error_display() {
        let err = BackgroundError::BackgroundNotFound(BackgroundId::new(123));
        let display = format!("{}", err);
        assert!(display.contains("123"));
    }

    #[test]
    fn test_set_background_dark_theme() {
        let manager = BackgroundManager::new();
        let bg = create_test_background(123, "dark_bg");

        manager.add_background(bg.clone(), false);

        manager
            .set_background(bg.id, bg.type_.clone(), true)
            .unwrap();

        assert_eq!(manager.get_current_background_id(true), bg.id);
        assert_ne!(manager.get_current_background_id(false), bg.id);
    }

    #[test]
    fn test_multiple_dialogs() {
        let manager = BackgroundManager::new();
        let bg = create_test_background(123, "test_bg");
        let dialog1 = DialogId::new(111);
        let dialog2 = DialogId::new(222);

        manager.add_background(bg.clone(), false);

        manager
            .set_dialog_background(
                dialog1,
                bg.id,
                bg.type_.clone(),
                50,
                MessageId::new(1048576).unwrap(),
            )
            .unwrap();
        manager
            .set_dialog_background(
                dialog2,
                bg.id,
                bg.type_.clone(),
                75,
                MessageId::new(2097152).unwrap(),
            )
            .unwrap();

        let dialog_bg1 = manager.get_dialog_background(dialog1).unwrap();
        let dialog_bg2 = manager.get_dialog_background(dialog2).unwrap();

        assert_eq!(dialog_bg1.dark_theme_dimming, 50);
        assert_eq!(dialog_bg2.dark_theme_dimming, 75);
    }

    #[test]
    fn test_dialog_background_partial_eq() {
        let dialog_id = DialogId::new(1234567890);
        let bg1 = DialogBackground::new(
            dialog_id,
            BackgroundId::new(123),
            BackgroundType::default(),
            50,
            MessageId::new(1048576).unwrap(),
        );
        let bg2 = DialogBackground::new(
            dialog_id,
            BackgroundId::new(123),
            BackgroundType::default(),
            50,
            MessageId::new(1048576).unwrap(),
        );

        assert_eq!(bg1, bg2);
    }

    #[test]
    fn test_background_clone() {
        let bg = create_test_background(123, "test_bg");
        let cloned = bg.clone();
        assert_eq!(bg, cloned);
    }

    #[test]
    fn test_background_log_event() {
        let mut bg = create_test_background(123, "test_bg");
        assert_eq!(bg.log_event_id(), 123);

        bg.set_log_event_id(456);
        assert_eq!(bg.log_event_id(), 456);
        assert_eq!(bg.id, BackgroundId::new(456));
    }

    // Additional tests for coverage
    #[test]
    fn test_background_error_invalid_type() {
        let err = BackgroundError::InvalidBackgroundType;
        assert_eq!(format!("{}", err), "Invalid background type");
    }

    #[test]
    fn test_background_error_file_operation() {
        let err = BackgroundError::FileOperationFailed("test error".to_string());
        assert!(format!("{}", err).contains("test error"));
    }

    #[test]
    fn test_background_error_upload_failed() {
        let err = BackgroundError::UploadFailed("upload failed".to_string());
        assert!(format!("{}", err).contains("upload failed"));
    }

    #[test]
    fn test_background_error_dialog_not_found() {
        let dialog_id = DialogId::new(1234567890);
        let err = BackgroundError::DialogNotFound(dialog_id);
        assert!(format!("{}", err).contains("1234567890"));
    }

    #[test]
    fn test_background_error_invalid_access_hash() {
        let err = BackgroundError::InvalidAccessHash;
        assert_eq!(format!("{}", err), "Invalid access hash");
    }

    #[test]
    fn test_get_backgrounds_for_dark_theme() {
        let manager = BackgroundManager::new();
        let bg1 = create_test_background(123, "bg1");
        let bg2 = create_test_background(456, "bg2");

        manager.add_background(bg1, false);
        manager.add_background(bg2, false);

        let backgrounds = manager.get_backgrounds(true);
        assert_eq!(backgrounds.len(), 2);
    }

    #[test]
    fn test_complete_upload_nonexistent() {
        let manager = BackgroundManager::new();
        let file_id = FileId::new(1, 0);
        let upload_id = FileUploadId::new(file_id, 123);

        assert!(manager.complete_upload(upload_id).is_none());
    }
}
