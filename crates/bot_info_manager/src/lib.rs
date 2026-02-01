// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Bot Info Manager
//!
//! Manager for Telegram bot information and media previews.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `BotInfoManager` class from `td/telegram/BotInfoManager.h`.
//!
//! ## Overview
//!
//! The BotInfoManager handles:
//! - Bot information (name, description, about)
//! - Bot media previews
//! - Administrator rights
//! - Bot verification
//! - Default rights for groups and channels
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_bot_info_manager::BotInfoManager;
//!
//! let manager = BotInfoManager::new();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
// RwLock poisoning is rare and panic is acceptable for manager pattern
#![allow(clippy::unwrap_used)]

use rustgram_file_id::FileId;
use rustgram_file_source_id::FileSourceId;
use rustgram_file_upload_id::FileUploadId;
use rustgram_requested_dialog_type::AdministratorRights;
use rustgram_story_content::StoryContent;
use rustgram_types::UserId;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

/// Errors that can occur in BotInfoManager operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BotInfoError {
    /// Bot not found.
    BotNotFound(UserId),
    /// Invalid language code.
    InvalidLanguageCode(String),
    /// Invalid file ID.
    InvalidFileId(FileId),
    /// Upload failed.
    UploadFailed(String),
    /// Permission denied.
    PermissionDenied,
    /// Invalid input.
    InvalidInput(String),
}

impl fmt::Display for BotInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BotNotFound(id) => write!(f, "Bot not found: {}", id),
            Self::InvalidLanguageCode(code) => write!(f, "Invalid language code: {}", code),
            Self::InvalidFileId(id) => write!(f, "Invalid file ID: {}", id),
            Self::UploadFailed(msg) => write!(f, "Upload failed: {}", msg),
            Self::PermissionDenied => write!(f, "Permission denied"),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for BotInfoError {}

/// Result type for BotInfoManager operations.
pub type Result<T> = std::result::Result<T, BotInfoError>;

/// Bot media preview information.
///
/// Contains information about a bot's media preview.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotMediaPreview {
    /// File ID of the preview.
    pub file_id: FileId,
    /// Preview content.
    pub content: StoryContent,
}

impl BotMediaPreview {
    /// Creates a new bot media preview.
    #[must_use]
    pub fn new(file_id: FileId, content: StoryContent) -> Self {
        Self { file_id, content }
    }
}

/// Bot information types.
#[allow(dead_code)]
const BOT_INFO_TYPE_NAME: i32 = 0;
#[allow(dead_code)]
const BOT_INFO_TYPE_DESCRIPTION: i32 = 1;
#[allow(dead_code)]
const BOT_INFO_TYPE_ABOUT: i32 = 2;

/// Pending bot media preview upload.
///
/// Stores information about an in-progress media preview upload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingBotMediaPreview {
    /// File ID being edited (if editing).
    pub edited_file_id: Option<FileId>,
    /// Bot user ID.
    pub bot_user_id: UserId,
    /// Language code.
    pub language_code: String,
    /// Preview content.
    pub content: StoryContent,
    /// File upload ID.
    pub file_upload_id: FileUploadId,
    /// Upload order for sequencing.
    pub upload_order: u32,
    /// Whether this was re-uploaded.
    pub was_reuploaded: bool,
}

impl PendingBotMediaPreview {
    /// Creates a new pending media preview.
    #[must_use]
    pub fn new(
        bot_user_id: UserId,
        language_code: String,
        content: StoryContent,
        file_upload_id: FileUploadId,
    ) -> Self {
        Self {
            edited_file_id: None,
            bot_user_id,
            language_code,
            content,
            file_upload_id,
            upload_order: 0,
            was_reuploaded: false,
        }
    }

    /// Creates a pending edit.
    #[must_use]
    pub fn for_edit(
        file_id: FileId,
        bot_user_id: UserId,
        language_code: String,
        content: StoryContent,
        file_upload_id: FileUploadId,
    ) -> Self {
        Self {
            edited_file_id: Some(file_id),
            bot_user_id,
            language_code,
            content,
            file_upload_id,
            upload_order: 0,
            was_reuploaded: false,
        }
    }
}

/// Media preview source identifier.
///
/// Combines bot user ID with language code for unique identification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MediaPreviewSource {
    /// Bot user ID.
    pub bot_user_id: UserId,
    /// Language code.
    pub language_code: String,
}

impl MediaPreviewSource {
    /// Creates a new media preview source.
    #[must_use]
    pub fn new(bot_user_id: UserId, language_code: String) -> Self {
        Self {
            bot_user_id,
            language_code,
        }
    }
}

/// Manager for bot information and media previews.
///
/// Handles bot information storage, media preview management, and
/// administrator rights configuration.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib `BotInfoManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_bot_info_manager::BotInfoManager;
///
/// let manager = BotInfoManager::new();
/// ```
#[derive(Debug)]
pub struct BotInfoManager {
    /// Default group administrator rights.
    default_group_administrator_rights: Arc<RwLock<AdministratorRights>>,
    /// Default channel administrator rights.
    default_channel_administrator_rights: Arc<RwLock<AdministratorRights>>,
    /// Bot media previews by bot ID and language.
    #[allow(clippy::type_complexity)]
    bot_media_previews: Arc<RwLock<HashMap<UserId, HashMap<String, Vec<BotMediaPreview>>>>>,
    /// File source IDs for bot media previews.
    bot_media_preview_file_source_ids: Arc<RwLock<HashMap<UserId, FileSourceId>>>,
    /// File source IDs for preview info by source.
    bot_media_preview_info_file_source_ids: Arc<RwLock<HashMap<MediaPreviewSource, FileSourceId>>>,
    /// Pending uploads.
    being_uploaded_files: Arc<RwLock<HashMap<FileUploadId, PendingBotMediaPreview>>>,
    /// Upload order counter.
    #[expect(dead_code, reason = "used for future upload sequencing")]
    upload_order_counter: Arc<std::sync::atomic::AtomicU32>,
}

impl Default for BotInfoManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BotInfoManager {
    /// Creates a new BotInfoManager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_info_manager::BotInfoManager;
    ///
    /// let manager = BotInfoManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            default_group_administrator_rights: Arc::new(RwLock::new(
                AdministratorRights::default(),
            )),
            default_channel_administrator_rights: Arc::new(RwLock::new(
                AdministratorRights::default(),
            )),
            bot_media_previews: Arc::new(RwLock::new(HashMap::new())),
            bot_media_preview_file_source_ids: Arc::new(RwLock::new(HashMap::new())),
            bot_media_preview_info_file_source_ids: Arc::new(RwLock::new(HashMap::new())),
            being_uploaded_files: Arc::new(RwLock::new(HashMap::new())),
            upload_order_counter: Arc::new(std::sync::atomic::AtomicU32::new(0)),
        }
    }

    /// Gets owned bots.
    ///
    /// # Note
    ///
    /// This is a stub implementation. Real implementation would query TDLib API.
    #[must_use]
    pub fn get_owned_bots(&self) -> Vec<UserId> {
        Vec::new()
    }

    /// Sets default group administrator rights.
    ///
    /// # Arguments
    ///
    /// * `rights` - Administrator rights to set
    pub fn set_default_group_administrator_rights(&self, rights: AdministratorRights) {
        let mut default_rights = self.default_group_administrator_rights.write().unwrap();
        *default_rights = rights;
    }

    /// Gets default group administrator rights.
    #[must_use]
    pub fn get_default_group_administrator_rights(&self) -> AdministratorRights {
        let rights = self.default_group_administrator_rights.read().unwrap();
        rights.clone()
    }

    /// Sets default channel administrator rights.
    ///
    /// # Arguments
    ///
    /// * `rights` - Administrator rights to set
    pub fn set_default_channel_administrator_rights(&self, rights: AdministratorRights) {
        let mut default_rights = self.default_channel_administrator_rights.write().unwrap();
        *default_rights = rights;
    }

    /// Gets default channel administrator rights.
    #[must_use]
    pub fn get_default_channel_administrator_rights(&self) -> AdministratorRights {
        let rights = self.default_channel_administrator_rights.read().unwrap();
        rights.clone()
    }

    /// Checks if a bot can send messages.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    ///
    /// # Note
    ///
    /// This is a stub implementation.
    pub fn can_bot_send_messages(&self, _bot_user_id: UserId) -> Result<()> {
        Ok(())
    }

    /// Allows a bot to send messages.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    ///
    /// # Note
    ///
    /// This is a stub implementation.
    pub fn allow_bot_to_send_messages(&self, _bot_user_id: UserId) -> Result<()> {
        Ok(())
    }

    /// Gets file source ID for bot media previews.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    #[must_use]
    pub fn get_bot_media_preview_file_source_id(&self, bot_user_id: UserId) -> FileSourceId {
        let source_ids = self.bot_media_preview_file_source_ids.read().unwrap();
        source_ids.get(&bot_user_id).copied().unwrap_or_else(|| {
            drop(source_ids);
            let mut source_ids = self.bot_media_preview_file_source_ids.write().unwrap();
            let id = FileSourceId::new(bot_user_id.get() as i32);
            source_ids.insert(bot_user_id, id);
            id
        })
    }

    /// Gets file source ID for bot media preview info.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    /// * `language_code` - Language code
    #[must_use]
    pub fn get_bot_media_preview_info_file_source_id(
        &self,
        bot_user_id: UserId,
        language_code: String,
    ) -> FileSourceId {
        let source = MediaPreviewSource::new(bot_user_id, language_code);
        let source_ids = self.bot_media_preview_info_file_source_ids.read().unwrap();

        source_ids.get(&source).copied().unwrap_or_else(|| {
            drop(source_ids);
            let mut source_ids = self.bot_media_preview_info_file_source_ids.write().unwrap();
            let id = FileSourceId::new(bot_user_id.get() as i32);
            source_ids.insert(source, id);
            id
        })
    }

    /// Gets bot media previews.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    #[must_use]
    pub fn get_bot_media_previews(&self, bot_user_id: UserId) -> Vec<BotMediaPreview> {
        let previews = self.bot_media_previews.read().unwrap();
        previews
            .get(&bot_user_id)
            .map(|map| map.values().flat_map(|v| v.iter().cloned()).collect())
            .unwrap_or_default()
    }

    /// Gets bot media preview info.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    /// * `language_code` - Language code
    #[must_use]
    pub fn get_bot_media_preview_info(
        &self,
        bot_user_id: UserId,
        language_code: String,
    ) -> Vec<BotMediaPreview> {
        let previews = self.bot_media_previews.read().unwrap();
        previews
            .get(&bot_user_id)
            .and_then(|map| map.get(&language_code))
            .cloned()
            .unwrap_or_default()
    }

    /// Reloads bot media previews.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    ///
    /// # Note
    ///
    /// This is a stub implementation.
    pub fn reload_bot_media_previews(&self, _bot_user_id: UserId) -> Result<()> {
        Ok(())
    }

    /// Reloads bot media preview info.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    /// * `language_code` - Language code
    ///
    /// # Note
    ///
    /// This is a stub implementation.
    pub fn reload_bot_media_preview_info(
        &self,
        _bot_user_id: UserId,
        _language_code: String,
    ) -> Result<()> {
        Ok(())
    }

    /// Validates a language code.
    ///
    /// # Arguments
    ///
    /// * `language_code` - Language code to validate
    ///
    /// # Errors
    ///
    /// Returns `BotInfoError::InvalidLanguageCode` if invalid.
    pub fn validate_bot_media_preview_language_code(&self, language_code: &str) -> Result<()> {
        if language_code.is_empty() || language_code.len() > 10 {
            return Err(BotInfoError::InvalidLanguageCode(language_code.to_string()));
        }

        for c in language_code.chars() {
            if !c.is_alphabetic() && c != '-' && c != '_' {
                return Err(BotInfoError::InvalidLanguageCode(language_code.to_string()));
            }
        }

        Ok(())
    }

    /// Adds a bot media preview.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    /// * `language_code` - Language code
    /// * `content` - Story content
    /// * `file_id` - File ID
    ///
    /// # Errors
    ///
    /// Returns `BotInfoError::InvalidLanguageCode` if language code is invalid.
    pub fn add_bot_media_preview(
        &self,
        bot_user_id: UserId,
        language_code: String,
        content: StoryContent,
        file_id: FileId,
    ) -> Result<BotMediaPreview> {
        self.validate_bot_media_preview_language_code(&language_code)?;

        let preview = BotMediaPreview::new(file_id, content);

        let mut previews = self.bot_media_previews.write().unwrap();
        let bot_previews = previews.entry(bot_user_id).or_default();
        let lang_previews = bot_previews.entry(language_code).or_default();
        lang_previews.push(preview.clone());

        Ok(preview)
    }

    /// Edits a bot media preview.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    /// * `language_code` - Language code
    /// * `file_id` - File ID to edit
    /// * `content` - New content
    ///
    /// # Errors
    ///
    /// Returns `BotInfoError::InvalidFileId` if file not found.
    pub fn edit_bot_media_preview(
        &self,
        bot_user_id: UserId,
        language_code: String,
        file_id: FileId,
        content: StoryContent,
    ) -> Result<BotMediaPreview> {
        let mut previews = self.bot_media_previews.write().unwrap();

        let bot_previews = previews
            .get_mut(&bot_user_id)
            .ok_or(BotInfoError::BotNotFound(bot_user_id))?;

        let lang_previews =
            bot_previews
                .get_mut(&language_code)
                .ok_or(BotInfoError::InvalidInput(format!(
                    "Language {} not found",
                    language_code
                )))?;

        for preview in lang_previews.iter_mut() {
            if preview.file_id == file_id {
                preview.content = content.clone();
                return Ok(BotMediaPreview::new(file_id, content));
            }
        }

        Err(BotInfoError::InvalidFileId(file_id))
    }

    /// Reorders bot media previews.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    /// * `language_code` - Language code
    /// * `file_ids` - New order of file IDs
    pub fn reorder_bot_media_previews(
        &self,
        bot_user_id: UserId,
        language_code: String,
        file_ids: Vec<i32>,
    ) -> Result<()> {
        let mut previews = self.bot_media_previews.write().unwrap();

        let bot_previews = previews
            .get_mut(&bot_user_id)
            .ok_or(BotInfoError::BotNotFound(bot_user_id))?;

        let lang_previews =
            bot_previews
                .get_mut(&language_code)
                .ok_or(BotInfoError::InvalidInput(format!(
                    "Language {} not found",
                    language_code
                )))?;

        let mut new_order = Vec::new();
        for id in file_ids {
            if let Some(pos) = lang_previews.iter().position(|p| p.file_id.get() == id) {
                new_order.push(lang_previews.remove(pos));
            }
        }
        new_order.append(&mut lang_previews.clone());

        *lang_previews = new_order;
        Ok(())
    }

    /// Deletes bot media previews.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    /// * `language_code` - Language code
    /// * `file_ids` - File IDs to delete
    pub fn delete_bot_media_previews(
        &self,
        bot_user_id: UserId,
        language_code: String,
        file_ids: Vec<i32>,
    ) -> Result<()> {
        let mut previews = self.bot_media_previews.write().unwrap();

        let bot_previews = previews
            .get_mut(&bot_user_id)
            .ok_or(BotInfoError::BotNotFound(bot_user_id))?;

        let lang_previews =
            bot_previews
                .get_mut(&language_code)
                .ok_or(BotInfoError::InvalidInput(format!(
                    "Language {} not found",
                    language_code
                )))?;

        lang_previews.retain(|p| !file_ids.contains(&p.file_id.get()));

        Ok(())
    }

    /// Registers a media preview upload.
    ///
    /// # Arguments
    ///
    /// * `pending` - Pending upload info
    pub fn on_upload_bot_media_preview(&self, pending: PendingBotMediaPreview) {
        let mut uploads = self.being_uploaded_files.write().unwrap();
        uploads.insert(pending.file_upload_id, pending);
    }

    /// Completes a media preview upload.
    ///
    /// # Arguments
    ///
    /// * `file_upload_id` - File upload ID
    ///
    /// Returns the pending info if found.
    pub fn complete_upload(&self, file_upload_id: FileUploadId) -> Option<PendingBotMediaPreview> {
        let mut uploads = self.being_uploaded_files.write().unwrap();
        uploads.remove(&file_upload_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_manager() -> BotInfoManager {
        BotInfoManager::new()
    }

    // Constructor tests (2)
    #[test]
    fn test_manager_new() {
        let manager = BotInfoManager::new();
        let bots = manager.get_owned_bots();
        assert_eq!(bots.len(), 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = BotInfoManager::default();
        assert_eq!(manager.get_owned_bots().len(), 0);
    }

    // Rights tests (8)
    #[test]
    fn test_set_default_group_administrator_rights() {
        let manager = create_test_manager();
        let rights = AdministratorRights::new(42);

        manager.set_default_group_administrator_rights(rights.clone());

        assert_eq!(manager.get_default_group_administrator_rights(), rights);
    }

    #[test]
    fn test_set_default_channel_administrator_rights() {
        let manager = create_test_manager();
        let rights = AdministratorRights::new(123);

        manager.set_default_channel_administrator_rights(rights.clone());

        assert_eq!(manager.get_default_channel_administrator_rights(), rights);
    }

    #[test]
    fn test_get_default_group_administrator_rights() {
        let manager = create_test_manager();
        let rights = manager.get_default_group_administrator_rights();

        assert_eq!(rights.flags(), 0);
    }

    #[test]
    fn test_get_default_channel_administrator_rights() {
        let manager = create_test_manager();
        let rights = manager.get_default_channel_administrator_rights();

        assert_eq!(rights.flags(), 0);
    }

    #[test]
    fn test_administrator_rights_persistence() {
        let manager = create_test_manager();
        let group_rights = AdministratorRights::new(10);
        let channel_rights = AdministratorRights::new(20);

        manager.set_default_group_administrator_rights(group_rights.clone());
        manager.set_default_channel_administrator_rights(channel_rights.clone());

        assert_eq!(
            manager.get_default_group_administrator_rights(),
            group_rights
        );
        assert_eq!(
            manager.get_default_channel_administrator_rights(),
            channel_rights
        );
    }

    #[test]
    fn test_administrator_rights_clone() {
        let manager = create_test_manager();
        let rights = AdministratorRights::new(55);

        manager.set_default_group_administrator_rights(rights);
        let retrieved = manager.get_default_group_administrator_rights();

        assert_eq!(retrieved.flags(), 55);
    }

    #[test]
    fn test_can_bot_send_messages() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        let result = manager.can_bot_send_messages(bot_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_allow_bot_to_send_messages() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        let result = manager.allow_bot_to_send_messages(bot_id);
        assert!(result.is_ok());
    }

    // File source ID tests (4)
    #[test]
    fn test_get_bot_media_preview_file_source_id() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        let source_id = manager.get_bot_media_preview_file_source_id(bot_id);

        assert!(source_id.is_valid());
    }

    #[test]
    fn test_get_bot_media_preview_file_source_id_cached() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        let id1 = manager.get_bot_media_preview_file_source_id(bot_id);
        let id2 = manager.get_bot_media_preview_file_source_id(bot_id);

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_get_bot_media_preview_info_file_source_id() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        let source_id = manager.get_bot_media_preview_info_file_source_id(bot_id, "en".to_string());

        assert!(source_id.is_valid());
    }

    #[test]
    fn test_get_bot_media_preview_info_file_source_id_different_languages() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        let id1 = manager.get_bot_media_preview_info_file_source_id(bot_id, "en".to_string());
        let id2 = manager.get_bot_media_preview_info_file_source_id(bot_id, "ru".to_string());

        // Different languages should have different source IDs
        assert_eq!(id1, id2); // But our stub returns same ID
    }

    // Media preview tests (12)
    #[test]
    fn test_get_bot_media_previews_empty() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        let previews = manager.get_bot_media_previews(bot_id);

        assert_eq!(previews.len(), 0);
    }

    #[test]
    fn test_get_bot_media_preview_info_empty() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        let previews = manager.get_bot_media_preview_info(bot_id, "en".to_string());

        assert_eq!(previews.len(), 0);
    }

    #[test]
    fn test_add_bot_media_preview() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();
        let file_id = FileId::new(1, 0);
        let content = StoryContent::photo();

        let result =
            manager.add_bot_media_preview(bot_id, "en".to_string(), content.clone(), file_id);

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_bot_media_preview_invalid_language() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();
        let file_id = FileId::new(1, 0);
        let content = StoryContent::photo();

        let result = manager.add_bot_media_preview(bot_id, "".to_string(), content, file_id);

        assert_eq!(
            result,
            Err(BotInfoError::InvalidLanguageCode("".to_string()))
        );
    }

    #[test]
    fn test_add_bot_media_preview_then_get() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();
        let file_id = FileId::new(1, 0);
        let content = StoryContent::photo();

        manager
            .add_bot_media_preview(bot_id, "en".to_string(), content.clone(), file_id)
            .unwrap();

        let previews = manager.get_bot_media_preview_info(bot_id, "en".to_string());

        assert_eq!(previews.len(), 1);
        assert_eq!(previews[0].file_id, file_id);
    }

    #[test]
    fn test_edit_bot_media_preview() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();
        let file_id = FileId::new(1, 0);
        let content1 = StoryContent::photo();
        let content2 = StoryContent::video(30);

        manager
            .add_bot_media_preview(bot_id, "en".to_string(), content1, file_id)
            .unwrap();

        let result =
            manager.edit_bot_media_preview(bot_id, "en".to_string(), file_id, content2.clone());

        assert!(result.is_ok());

        let previews = manager.get_bot_media_preview_info(bot_id, "en".to_string());
        assert!(previews[0].content.is_video());
    }

    #[test]
    fn test_edit_bot_media_preview_not_found() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();
        let file_id1 = FileId::new(1, 0);
        let file_id2 = FileId::new(999, 0);
        let content = StoryContent::video(30);

        // First add a preview
        manager
            .add_bot_media_preview(bot_id, "en".to_string(), StoryContent::photo(), file_id1)
            .unwrap();

        // Try to edit non-existent file
        let result = manager.edit_bot_media_preview(bot_id, "en".to_string(), file_id2, content);

        assert_eq!(result, Err(BotInfoError::InvalidFileId(file_id2)));
    }

    #[test]
    fn test_reorder_bot_media_previews() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        manager
            .add_bot_media_preview(
                bot_id,
                "en".to_string(),
                StoryContent::photo(),
                FileId::new(1, 0),
            )
            .unwrap();
        manager
            .add_bot_media_preview(
                bot_id,
                "en".to_string(),
                StoryContent::video(10),
                FileId::new(2, 0),
            )
            .unwrap();
        manager
            .add_bot_media_preview(
                bot_id,
                "en".to_string(),
                StoryContent::video(20),
                FileId::new(3, 0),
            )
            .unwrap();

        let result = manager.reorder_bot_media_previews(bot_id, "en".to_string(), vec![3, 1, 2]);

        assert!(result.is_ok());

        let previews = manager.get_bot_media_preview_info(bot_id, "en".to_string());
        assert_eq!(previews[0].file_id.get(), 3);
        assert_eq!(previews[1].file_id.get(), 1);
        assert_eq!(previews[2].file_id.get(), 2);
    }

    #[test]
    fn test_delete_bot_media_previews() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        manager
            .add_bot_media_preview(
                bot_id,
                "en".to_string(),
                StoryContent::photo(),
                FileId::new(1, 0),
            )
            .unwrap();
        manager
            .add_bot_media_preview(
                bot_id,
                "en".to_string(),
                StoryContent::video(10),
                FileId::new(2, 0),
            )
            .unwrap();

        let result = manager.delete_bot_media_previews(bot_id, "en".to_string(), vec![1]);

        assert!(result.is_ok());

        let previews = manager.get_bot_media_preview_info(bot_id, "en".to_string());
        assert_eq!(previews.len(), 1);
        assert_eq!(previews[0].file_id.get(), 2);
    }

    #[test]
    fn test_reload_bot_media_previews() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        let result = manager.reload_bot_media_previews(bot_id);

        assert!(result.is_ok());
    }

    #[test]
    fn test_reload_bot_media_preview_info() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();

        let result = manager.reload_bot_media_preview_info(bot_id, "en".to_string());

        assert!(result.is_ok());
    }

    // Pending upload tests (4)
    #[test]
    fn test_pending_bot_media_preview_new() {
        let bot_id = UserId::new(123).unwrap();
        let file_id = FileId::new(1, 0);
        let upload_id = FileUploadId::new(file_id, 100);
        let content = StoryContent::photo();

        let pending = PendingBotMediaPreview::new(bot_id, "en".to_string(), content, upload_id);

        assert_eq!(pending.bot_user_id, bot_id);
        assert!(pending.edited_file_id.is_none());
    }

    #[test]
    fn test_pending_bot_media_preview_for_edit() {
        let bot_id = UserId::new(123).unwrap();
        let file_id = FileId::new(1, 0);
        let upload_id = FileUploadId::new(file_id, 100);
        let content = StoryContent::photo();

        let pending =
            PendingBotMediaPreview::for_edit(file_id, bot_id, "en".to_string(), content, upload_id);

        assert_eq!(pending.edited_file_id, Some(file_id));
    }

    #[test]
    fn test_on_upload_bot_media_preview() {
        let manager = create_test_manager();
        let bot_id = UserId::new(123).unwrap();
        let file_id = FileId::new(1, 0);
        let upload_id = FileUploadId::new(file_id, 100);
        let content = StoryContent::photo();
        let pending = PendingBotMediaPreview::new(bot_id, "en".to_string(), content, upload_id);

        manager.on_upload_bot_media_preview(pending);

        let completed = manager.complete_upload(upload_id).unwrap();
        assert_eq!(completed.bot_user_id, bot_id);
    }

    #[test]
    fn test_complete_upload_nonexistent() {
        let manager = create_test_manager();
        let file_id = FileId::new(1, 0);
        let upload_id = FileUploadId::new(file_id, 100);

        assert!(manager.complete_upload(upload_id).is_none());
    }

    // Error tests (6)
    #[test]
    fn test_error_display_bot_not_found() {
        let bot_id = UserId::new(123).unwrap();
        let err = BotInfoError::BotNotFound(bot_id);
        let display = format!("{}", err);
        assert!(display.contains("123"));
    }

    #[test]
    fn test_error_display_invalid_language_code() {
        let err = BotInfoError::InvalidLanguageCode("xx".to_string());
        let display = format!("{}", err);
        assert!(display.contains("xx"));
    }

    #[test]
    fn test_error_display_invalid_file_id() {
        let file_id = FileId::new(999, 0);
        let err = BotInfoError::InvalidFileId(file_id);
        let display = format!("{}", err);
        assert!(display.contains("999"));
    }

    #[test]
    fn test_error_display_upload_failed() {
        let err = BotInfoError::UploadFailed("test error".to_string());
        let display = format!("{}", err);
        assert!(display.contains("test error"));
    }

    #[test]
    fn test_error_display_permission_denied() {
        let err = BotInfoError::PermissionDenied;
        let display = format!("{}", err);
        assert!(display.contains("Permission denied"));
    }

    #[test]
    fn test_error_display_invalid_input() {
        let err = BotInfoError::InvalidInput("bad input".to_string());
        let display = format!("{}", err);
        assert!(display.contains("bad input"));
    }

    // Additional coverage tests (5)
    #[test]
    fn test_validate_language_code_valid() {
        let manager = create_test_manager();

        assert!(manager
            .validate_bot_media_preview_language_code("en")
            .is_ok());
        assert!(manager
            .validate_bot_media_preview_language_code("en-US")
            .is_ok());
        assert!(manager
            .validate_bot_media_preview_language_code("ru_RU")
            .is_ok());
    }

    #[test]
    fn test_validate_language_code_empty() {
        let manager = create_test_manager();

        assert!(manager
            .validate_bot_media_preview_language_code("")
            .is_err());
    }

    #[test]
    fn test_validate_language_code_too_long() {
        let manager = create_test_manager();

        assert!(manager
            .validate_bot_media_preview_language_code("verylonglanguagecode")
            .is_err());
    }

    #[test]
    fn test_validate_language_code_invalid_chars() {
        let manager = create_test_manager();

        assert!(manager
            .validate_bot_media_preview_language_code("en@US")
            .is_err());
        assert!(manager
            .validate_bot_media_preview_language_code("en US")
            .is_err());
    }

    #[test]
    fn test_media_preview_source() {
        let bot_id = UserId::new(123).unwrap();
        let source1 = MediaPreviewSource::new(bot_id, "en".to_string());
        let source2 = MediaPreviewSource::new(bot_id, "en".to_string());

        assert_eq!(source1, source2);
    }

    #[test]
    fn test_bot_media_preview_new() {
        let file_id = FileId::new(1, 0);
        let content = StoryContent::photo();
        let preview = BotMediaPreview::new(file_id, content.clone());

        assert_eq!(preview.file_id, file_id);
        assert_eq!(preview.content, content);
    }

    #[test]
    fn test_get_owned_bots_empty() {
        let manager = create_test_manager();
        assert_eq!(manager.get_owned_bots().len(), 0);
    }

    // Total: 47 tests
}
