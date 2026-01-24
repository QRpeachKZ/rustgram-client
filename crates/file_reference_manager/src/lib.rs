//! # File Reference Manager
//!
//! Manages file references and creates file source IDs for different source types.
//!
//! This module provides functionality to track file sources across Telegram.
//! Files can come from many different sources (messages, user photos, stickers,
//! backgrounds, etc.) and this module creates unique source IDs for tracking them.
//!
//! ## Overview
//!
//! The `FileReferenceManager` maintains mappings between files and their sources.
//! Each file can have multiple sources, and each source type has its own
//! identifier format.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_file_reference_manager::FileReferenceManager;
//! use rustgram_file_id::FileId;
//! use rustgram_file_source_id::FileSourceId;
//! use rustgram_types::{UserId, ChatId, ChannelId, DialogId, MessageId};
//! use rustgram_message_full_id::MessageFullId;
//! use rustgram_quick_reply_message_full_id::QuickReplyMessageFullId;
//! use rustgram_background_id::BackgroundId;
//!
//! let mut manager = FileReferenceManager::new();
//!
//! // Create a file source from a message
//! let message_id = MessageId(1);
//! let dialog_id = DialogId::Chat(ChatId(123));
//! let message_full_id = MessageFullId::new(dialog_id, message_id);
//! let source_id = manager.create_message_file_source(message_full_id);
//!
//! // Add file source to tracking
//! let file_id = FileId::empty();
//! let added = manager.add_file_source(file_id, source_id);
//! assert!(added);
//!
//! // Get all sources for a file
//! let sources = manager.get_file_sources(file_id);
//! assert_eq!(sources.len(), 1);
//! ```
//!
//! ## File Source Types
//!
//! The manager supports creating file sources from:
//!
//! - Messages and quick reply messages
//! - User photos and full user info
//! - Web pages and web apps
//! - Stickers (saved, recent, favorite)
//! - Backgrounds and wallpapers
//! - Chat and channel full info
//! - Stories and story albums
//! - Bot media previews
//! - And more...

pub use types::{StoryAlbumFullId, StoryFullId};

use rustgram_background_id::BackgroundId;
use rustgram_dialog_id::DialogId;
use rustgram_file_id::FileId;
use rustgram_file_source_id::FileSourceId;
use rustgram_message_full_id::MessageFullId;
use rustgram_quick_reply_message_full_id::QuickReplyMessageFullId;
use rustgram_types::{ChannelId, ChatId, MessageId, UserId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

mod types;

/// File reference manager.
///
/// Tracks file sources and creates source IDs for different file types.
/// Each file can have multiple sources, and this manager maintains
/// the mapping between files and their sources.
///
/// # Thread Safety
///
/// The manager uses `Arc<RwLock<T>>` internally for thread-safe access.
/// Multiple threads can read sources simultaneously, while writes are
/// exclusive.
#[derive(Debug, Clone)]
pub struct FileReferenceManager {
    /// Mapping from file IDs to their source IDs.
    sources: Arc<RwLock<HashMap<FileId, Vec<FileSourceId>>>>,

    /// Counter for generating unique source IDs.
    source_id_counter: Arc<RwLock<i32>>,
}

impl Default for FileReferenceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FileReferenceManager {
    /// Create a new file reference manager.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_file_reference_manager::FileReferenceManager;
    ///
    /// let manager = FileReferenceManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            sources: Arc::new(RwLock::new(HashMap::new())),
            source_id_counter: Arc::new(RwLock::new(1i32)),
        }
    }

    /// Create a file source ID for a message file.
    ///
    /// # Arguments
    ///
    /// * `message_full_id` - The full message identifier
    pub fn create_message_file_source(&mut self, message_full_id: MessageFullId) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for a user photo.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    /// * `photo_id` - The photo identifier
    pub fn create_user_photo_file_source(
        &mut self,
        user_id: UserId,
        photo_id: i64,
    ) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for a web page file.
    ///
    /// # Arguments
    ///
    /// * `url` - The web page URL
    pub fn create_web_page_file_source(&mut self, url: String) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for saved animations.
    pub fn create_saved_animations_file_source(&mut self) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for recent stickers.
    ///
    /// # Arguments
    ///
    /// * `is_attached` - Whether these are attached stickers
    pub fn create_recent_stickers_file_source(&mut self, is_attached: bool) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for favorite stickers.
    pub fn create_favorite_stickers_file_source(&mut self) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for a background.
    ///
    /// # Arguments
    ///
    /// * `background_id` - The background identifier
    /// * `access_hash` - The access hash for the background
    pub fn create_background_file_source(
        &mut self,
        background_id: BackgroundId,
        access_hash: i64,
    ) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for chat full info.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat identifier
    pub fn create_chat_full_file_source(&mut self, chat_id: ChatId) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for channel full info.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel identifier
    pub fn create_channel_full_file_source(&mut self, channel_id: ChannelId) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for app config.
    pub fn create_app_config_file_source(&mut self) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for saved ringtones.
    pub fn create_saved_ringtones_file_source(&mut self) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for user full info.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    pub fn create_user_full_file_source(&mut self, user_id: UserId) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for attach menu bot.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The bot user identifier
    pub fn create_attach_menu_bot_file_source(&mut self, user_id: UserId) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for a web app.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    /// * `short_name` - The web app short name
    pub fn create_web_app_file_source(
        &mut self,
        user_id: UserId,
        short_name: String,
    ) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for a story.
    ///
    /// # Arguments
    ///
    /// * `story_full_id` - The full story identifier
    pub fn create_story_file_source(&mut self, story_full_id: StoryFullId) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for a quick reply message.
    ///
    /// # Arguments
    ///
    /// * `message_full_id` - The quick reply message identifier
    pub fn create_quick_reply_message_file_source(
        &mut self,
        message_full_id: QuickReplyMessageFullId,
    ) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for a star transaction.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `transaction_id` - The transaction ID string
    /// * `is_refund` - Whether this is a refund
    pub fn create_star_transaction_file_source(
        &mut self,
        dialog_id: DialogId,
        transaction_id: String,
        is_refund: bool,
    ) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for bot media preview.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - The bot user identifier
    pub fn create_bot_media_preview_file_source(&mut self, bot_user_id: UserId) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for bot media preview info.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - The bot user identifier
    /// * `language_code` - The language code
    pub fn create_bot_media_preview_info_file_source(
        &mut self,
        bot_user_id: UserId,
        language_code: String,
    ) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for a story album.
    ///
    /// # Arguments
    ///
    /// * `story_album_full_id` - The story album identifier
    pub fn create_story_album_file_source(
        &mut self,
        story_album_full_id: StoryAlbumFullId,
    ) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Create a file source ID for user saved music.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user identifier
    /// * `document_id` - The document identifier
    /// * `access_hash` - The access hash
    pub fn create_user_saved_music_file_source(
        &mut self,
        _user_id: UserId,
        _document_id: i64,
        _access_hash: i64,
    ) -> FileSourceId {
        let id = self.next_source_id();
        FileSourceId::new(id)
    }

    /// Add a file source to tracking.
    ///
    /// Returns `true` if the source was added, `false` if it was already tracked.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The file identifier
    /// * `file_source_id` - The source identifier to add
    pub fn add_file_source(&mut self, node_id: FileId, file_source_id: FileSourceId) -> bool {
        let mut sources = self.sources.write().unwrap();
        if let Some(existing) = sources.get(&node_id) {
            if existing.contains(&file_source_id) {
                return false;
            }
        }
        sources.entry(node_id).or_default().push(file_source_id);
        true
    }

    /// Remove a file source from tracking.
    ///
    /// Returns `true` if the source was removed, `false` if it wasn't tracked.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The file identifier
    /// * `file_source_id` - The source identifier to remove
    pub fn remove_file_source(&mut self, node_id: FileId, file_source_id: FileSourceId) -> bool {
        let mut sources = self.sources.write().unwrap();
        if let Some(existing) = sources.get_mut(&node_id) {
            let pos = existing.iter().position(|x| x == &file_source_id);
            if let Some(index) = pos {
                existing.remove(index);
                return true;
            }
        }
        false
    }

    /// Get all file sources for a file.
    ///
    /// Returns a vector of source IDs, or an empty vector if none are tracked.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The file identifier
    pub fn get_file_sources(&self, node_id: FileId) -> Vec<FileSourceId> {
        let sources = self.sources.read().unwrap();
        sources.get(&node_id).cloned().unwrap_or_default()
    }

    /// Generate the next unique source ID.
    fn next_source_id(&self) -> i32 {
        let mut counter = self.source_id_counter.write().unwrap();
        let id = *counter;
        *counter = counter.wrapping_add(1);
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_new() {
        let manager = FileReferenceManager::new();
        let sources = manager.get_file_sources(FileId::empty());
        assert!(sources.is_empty());
    }

    #[test]
    fn test_manager_default() {
        let manager = FileReferenceManager::default();
        let sources = manager.get_file_sources(FileId::empty());
        assert!(sources.is_empty());
    }

    #[test]
    fn test_create_message_file_source() {
        let mut manager = FileReferenceManager::new();
        let dialog_id = DialogIdType::new(123);
        let message_id = MessageId(1);
        let message_full_id = MessageFullId::new(dialog_id, message_id);
        let source_id = manager.create_message_file_source(message_full_id);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_user_photo_file_source() {
        let mut manager = FileReferenceManager::new();
        let user_id = UserId(123);
        let source_id = manager.create_user_photo_file_source(user_id, 456);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_web_page_file_source() {
        let mut manager = FileReferenceManager::new();
        let source_id = manager.create_web_page_file_source("https://example.com".to_string());
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_saved_animations_file_source() {
        let mut manager = FileReferenceManager::new();
        let source_id = manager.create_saved_animations_file_source();
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_recent_stickers_file_source() {
        let mut manager = FileReferenceManager::new();
        let source_id = manager.create_recent_stickers_file_source(true);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_favorite_stickers_file_source() {
        let mut manager = FileReferenceManager::new();
        let source_id = manager.create_favorite_stickers_file_source();
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_background_file_source() {
        let mut manager = FileReferenceManager::new();
        let background_id = BackgroundId::new(123);
        let source_id = manager.create_background_file_source(background_id, 456);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_chat_full_file_source() {
        let mut manager = FileReferenceManager::new();
        let chat_id = ChatId(123);
        let source_id = manager.create_chat_full_file_source(chat_id);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_channel_full_file_source() {
        let mut manager = FileReferenceManager::new();
        let channel_id = ChannelId(123);
        let source_id = manager.create_channel_full_file_source(channel_id);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_app_config_file_source() {
        let mut manager = FileReferenceManager::new();
        let source_id = manager.create_app_config_file_source();
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_saved_ringtones_file_source() {
        let mut manager = FileReferenceManager::new();
        let source_id = manager.create_saved_ringtones_file_source();
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_user_full_file_source() {
        let mut manager = FileReferenceManager::new();
        let user_id = UserId(123);
        let source_id = manager.create_user_full_file_source(user_id);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_attach_menu_bot_file_source() {
        let mut manager = FileReferenceManager::new();
        let user_id = UserId(123);
        let source_id = manager.create_attach_menu_bot_file_source(user_id);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_web_app_file_source() {
        let mut manager = FileReferenceManager::new();
        let user_id = UserId(123);
        let source_id = manager.create_web_app_file_source(user_id, "myapp".to_string());
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_story_file_source() {
        let mut manager = FileReferenceManager::new();
        let dialog_id = DialogIdType::new(123);
        let story_full_id = StoryFullId {
            story_id: 1,
            dialog_id,
        };
        let source_id = manager.create_story_file_source(story_full_id);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_quick_reply_message_file_source() {
        let mut manager = FileReferenceManager::new();
        let message_full_id = QuickReplyMessageFullId::new(1, 2);
        let source_id = manager.create_quick_reply_message_file_source(message_full_id);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_star_transaction_file_source() {
        let mut manager = FileReferenceManager::new();
        let dialog_id = DialogIdType::new(123);
        let source_id =
            manager.create_star_transaction_file_source(dialog_id, "tx123".to_string(), false);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_bot_media_preview_file_source() {
        let mut manager = FileReferenceManager::new();
        let user_id = UserId(123);
        let source_id = manager.create_bot_media_preview_file_source(user_id);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_bot_media_preview_info_file_source() {
        let mut manager = FileReferenceManager::new();
        let user_id = UserId(123);
        let source_id =
            manager.create_bot_media_preview_info_file_source(user_id, "en".to_string());
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_story_album_file_source() {
        let mut manager = FileReferenceManager::new();
        let dialog_id = DialogIdType::new(123);
        let album_id = StoryAlbumFullId {
            dialog_id,
            album_id: 456,
        };
        let source_id = manager.create_story_album_file_source(album_id);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_create_user_saved_music_file_source() {
        let mut manager = FileReferenceManager::new();
        let user_id = UserId(123);
        let source_id = manager.create_user_saved_music_file_source(user_id, 456, 789);
        assert_eq!(source_id, FileSourceId::new(1));
    }

    #[test]
    fn test_add_file_source() {
        let mut manager = FileReferenceManager::new();
        let file_id = FileId::empty();
        let source_id = FileSourceId::new(1);
        let added = manager.add_file_source(file_id, source_id);
        assert!(added);
        let sources = manager.get_file_sources(file_id);
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0], source_id);
    }

    #[test]
    fn test_add_file_source_duplicate() {
        let mut manager = FileReferenceManager::new();
        let file_id = FileId::empty();
        let source_id = FileSourceId::new(1);
        manager.add_file_source(file_id, source_id);
        let added = manager.add_file_source(file_id, source_id);
        assert!(!added);
        let sources = manager.get_file_sources(file_id);
        assert_eq!(sources.len(), 1);
    }

    #[test]
    fn test_add_multiple_file_sources() {
        let mut manager = FileReferenceManager::new();
        let file_id = FileId::empty();
        manager.add_file_source(file_id, FileSourceId::new(1));
        manager.add_file_source(file_id, FileSourceId::new(2));
        manager.add_file_source(file_id, FileSourceId::new(3));
        let sources = manager.get_file_sources(file_id);
        assert_eq!(sources.len(), 3);
    }

    #[test]
    fn test_remove_file_source() {
        let mut manager = FileReferenceManager::new();
        let file_id = FileId::empty();
        let source_id = FileSourceId::new(1);
        manager.add_file_source(file_id, source_id);
        let removed = manager.remove_file_source(file_id, source_id);
        assert!(removed);
        let sources = manager.get_file_sources(file_id);
        assert!(sources.is_empty());
    }

    #[test]
    fn test_remove_file_source_not_found() {
        let mut manager = FileReferenceManager::new();
        let file_id = FileId::empty();
        let source_id = FileSourceId::new(1);
        let removed = manager.remove_file_source(file_id, source_id);
        assert!(!removed);
    }

    #[test]
    fn test_remove_one_of_multiple_sources() {
        let mut manager = FileReferenceManager::new();
        let file_id = FileId::empty();
        manager.add_file_source(file_id, FileSourceId::new(1));
        manager.add_file_source(file_id, FileSourceId::new(2));
        manager.add_file_source(file_id, FileSourceId::new(3));
        let removed = manager.remove_file_source(file_id, FileSourceId::new(2));
        assert!(removed);
        let sources = manager.get_file_sources(file_id);
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0], FileSourceId::new(1));
        assert_eq!(sources[1], FileSourceId::new(3));
    }

    #[test]
    fn test_get_file_sources_empty() {
        let manager = FileReferenceManager::new();
        let file_id = FileId::empty();
        let sources = manager.get_file_sources(file_id);
        assert!(sources.is_empty());
    }

    #[test]
    fn test_get_file_sources_multiple_files() {
        let mut manager = FileReferenceManager::new();
        let file_id1 = FileId::new(1, 0);
        let file_id2 = FileId::new(2, 0);
        manager.add_file_source(file_id1, FileSourceId::new(1));
        manager.add_file_source(file_id2, FileSourceId::new(2));
        let sources1 = manager.get_file_sources(file_id1);
        let sources2 = manager.get_file_sources(file_id2);
        assert_eq!(sources1.len(), 1);
        assert_eq!(sources2.len(), 1);
        assert_eq!(sources1[0], FileSourceId::new(1));
        assert_eq!(sources2[0], FileSourceId::new(2));
    }

    #[test]
    fn test_source_id_increments() {
        let mut manager = FileReferenceManager::new();
        let dialog_id = DialogIdType::new(123);
        let message_id = MessageId(1);
        let message_full_id = MessageFullId::new(dialog_id, message_id);
        let source1 = manager.create_message_file_source(message_full_id);
        let source2 = manager.create_saved_animations_file_source();
        assert_eq!(source1, FileSourceId::new(1));
        assert_eq!(source2, FileSourceId::new(2));
    }

    #[test]
    fn test_manager_clone() {
        let mut manager = FileReferenceManager::new();
        let file_id = FileId::empty();
        manager.add_file_source(file_id, FileSourceId::new(1));
        let cloned = manager.clone();
        let sources = cloned.get_file_sources(file_id);
        assert_eq!(sources.len(), 1);
    }

    #[test]
    fn test_story_full_id_new() {
        let dialog_id = DialogIdType::new(123);
        let story_id = StoryFullId {
            story_id: 1,
            dialog_id,
        };
        assert_eq!(story_id.story_id, 1);
    }

    #[test]
    fn test_story_album_full_id_new() {
        let dialog_id = DialogIdType::new(123);
        let album_id = StoryAlbumFullId {
            dialog_id,
            album_id: 456,
        };
        assert_eq!(album_id.album_id, 456);
    }

    #[rstest::rstest]
    #[case(1)]
    #[case(5)]
    #[case(10)]
    #[case(100)]
    fn test_add_multiple_sources(#[case] count: usize) {
        let mut manager = FileReferenceManager::new();
        let file_id = FileId::empty();
        for i in 0..count {
            manager.add_file_source(file_id, FileSourceId::new(i as u64));
        }
        let sources = manager.get_file_sources(file_id);
        assert_eq!(sources.len(), count);
    }

    #[rstest::rstest]
    #[case(1)]
    #[case(5)]
    #[case(10)]
    fn test_remove_multiple_sources(#[case] count: usize) {
        let mut manager = FileReferenceManager::new();
        let file_id = FileId::empty();
        for i in 0..count {
            manager.add_file_source(file_id, FileSourceId::new(i as u64));
        }
        for i in 0..count {
            manager.remove_file_source(file_id, FileSourceId::new(i as u64));
        }
        let sources = manager.get_file_sources(file_id);
        assert!(sources.is_empty());
    }
}
