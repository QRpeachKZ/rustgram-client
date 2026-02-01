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

//! # Attach Menu Manager
//!
//! Manager for Telegram attachment menu bots.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `AttachMenuManager` class from
//! `td/telegram/AttachMenuManager.h` and `td/telegram/AttachMenuManager.cpp`.
//!
//! ## Overview
//!
//! The AttachMenuManager manages Telegram attachment menu bots, which are bots
//! that can be invoked from the attachment menu in chats. This manager handles:
//!
//! - Caching of attach menu bot data (icons, colors, metadata)
//! - Network queries for fetching/updating attach menu bots
//! - File source tracking for bot icons
//! - Bot addition/removal from attachment menu
//! - Database persistence with versioning
//!
//! ## Usage
//!
//! ```rust,no_run
//! use rustgram_attach_menu_manager::{AttachMenuManager, AttachMenuBot};
//! use rustgram_types::UserId;
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() {
//!     let manager = AttachMenuManager::new();
//!     // Initialize and use the manager
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod error;
pub mod tl;
pub mod types;

use crate::error::Result;
use crate::tl::TlAttachMenuBot;
use dashmap::DashMap;
use rustgram_file_source_id::FileSourceId;
use rustgram_types::UserId;
use std::sync::Arc;
use tokio::sync::RwLock;

// Re-export commonly used types
pub use crate::error::{AttachMenuManagerError, AttachMenuManagerError as Error, Result as AttachMenuResult};
pub use crate::types::{AttachMenuBot, AttachMenuBotColor};

/// Database key for attach menu bots cache.
const DATABASE_KEY: &str = "attach_bots";

/// Manager for Telegram attachment menu bots.
///
/// This manager is responsible for:
/// - Maintaining a cache of attach menu bots
/// - Handling network queries for fetching/updating bots
/// - Tracking file sources for bot icon downloads
/// - Managing bot addition/removal from attachment menu
/// - Persisting state to database
///
/// Corresponds to TDLib `AttachMenuManager` class.
#[derive(Debug, Clone)]
pub struct AttachMenuManager {
    /// Whether the manager has been initialized.
    is_inited: Arc<RwLock<bool>>,
    /// Hash for change detection.
    hash: Arc<RwLock<i64>>,
    /// Cached attach menu bots.
    attach_menu_bots: Arc<RwLock<Vec<AttachMenuBot>>>,
    /// File source IDs for each bot's icons.
    file_source_ids: Arc<DashMap<UserId, FileSourceId>>,
    /// Whether the manager is active (authorized, not bot, not closing).
    is_active: Arc<RwLock<bool>>,
    /// Pending reload queries.
    #[allow(dead_code)]
    pending_reloads: Arc<RwLock<Vec<tokio::sync::oneshot::Sender<()>>>>,
}

impl Default for AttachMenuManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AttachMenuManager {
    /// Creates a new attach menu manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuManager;
    ///
    /// let manager = AttachMenuManager::new();
    /// assert!(!manager.is_inited());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_inited: Arc::new(RwLock::new(false)),
            hash: Arc::new(RwLock::new(0)),
            attach_menu_bots: Arc::new(RwLock::new(Vec::new())),
            file_source_ids: Arc::new(DashMap::new()),
            is_active: Arc::new(RwLock::new(false)),
            pending_reloads: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Initializes the manager.
    ///
    /// This should be called once when the client is authorized.
    /// It loads cached data from the database and triggers an initial reload.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuManager;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let manager = AttachMenuManager::new();
    ///     manager.init().await;
    ///     assert!(manager.is_inited());
    /// }
    /// ```
    pub async fn init(&self) {
        let mut is_inited = self.is_inited.write().await;
        if *is_inited {
            return;
        }
        *is_inited = true;

        // Set as active
        let mut is_active = self.is_active.write().await;
        *is_active = true;
        drop(is_active);

        // TODO: Load from database
        // TODO: Register file sources
        // TODO: Send initial update
        // TODO: Trigger initial reload
    }

    /// Checks if the manager is initialized.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuManager;
    ///
    /// let manager = AttachMenuManager::new();
    /// assert!(!manager.is_inited());
    /// ```
    #[must_use]
    pub fn is_inited(&self) -> bool {
        // Use try_read to avoid blocking in tests
        if let Ok(guard) = self.is_inited.try_read() {
            *guard
        } else {
            false
        }
    }

    /// Checks if the manager is active.
    ///
    /// An active manager is one that is authorized, not a bot, and not closing.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuManager;
    ///
    /// let manager = AttachMenuManager::new();
    /// assert!(!manager.is_active());
    /// ```
    #[must_use]
    pub fn is_active(&self) -> bool {
        if let Ok(guard) = self.is_active.try_read() {
            *guard
        } else {
            false
        }
    }

    /// Gets the database key for attach menu bots.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuManager;
    ///
    /// assert_eq!(AttachMenuManager::get_attach_menu_bots_database_key(), "attach_bots");
    /// ```
    #[must_use]
    pub const fn get_attach_menu_bots_database_key() -> &'static str {
        DATABASE_KEY
    }

    /// Gets the file source ID for a bot's icons.
    ///
    /// Creates a new source ID if one doesn't exist.
    /// Returns an invalid FileSourceId if the user_id is invalid or manager is not active.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The bot user ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuManager;
    /// use rustgram_types::UserId;
    ///
    /// let manager = AttachMenuManager::new();
    /// let source_id = manager.get_attach_menu_bot_file_source_id(UserId(123));
    /// ```
    #[must_use]
    pub fn get_attach_menu_bot_file_source_id(&self, user_id: UserId) -> FileSourceId {
        if !user_id.is_valid() || !self.is_active() {
            return FileSourceId::new(0);
        }

        // Try to get existing source ID
        if let Some(source_id) = self.file_source_ids.get(&user_id) {
            return *source_id;
        }

        // Create new source ID
        // TODO: Use FileReferenceManager to create real source ID
        let new_id = FileSourceId::new((user_id.get() % i32::MAX as i64) as i32);
        self.file_source_ids.insert(user_id, new_id);
        new_id
    }

    /// Reloads all attachment menu bots from the server.
    ///
    /// Uses hash-based diffing to avoid unnecessary data transfer.
    /// Queues concurrent requests.
    ///
    /// # Arguments
    ///
    /// * `promise` - Callback for completion notification
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_attach_menu_manager::AttachMenuManager;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = AttachMenuManager::new();
    /// manager.reload_attach_menu_bots().await;
    /// # }
    /// ```
    pub async fn reload_attach_menu_bots(&self) -> Result<()> {
        if !self.is_active() {
            return Err(crate::error::AttachMenuManagerError::NotActive);
        }

        // TODO: Implement actual network query
        // For now, just mark as complete
        Ok(())
    }

    /// Gets a specific attachment menu bot.
    ///
    /// Validates that the bot can be added to the attachment menu.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The bot user ID
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_attach_menu_manager::AttachMenuManager;
    /// # use rustgram_types::UserId;
    /// # use rustgram_attach_menu_manager::Error;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// let manager = AttachMenuManager::new();
    /// let bot = manager.get_attach_menu_bot(UserId(123)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_attach_menu_bot(&self, _user_id: UserId) -> Result<AttachMenuBot> {
        if !self.is_active() {
            return Err(crate::error::AttachMenuManagerError::NotActive);
        }

        // TODO: Check if user is accessible
        // TODO: Check if bot can_be_added_to_attach_menu

        // For now, return error
        Err(crate::error::AttachMenuManagerError::UserNotAccessible)
    }

    /// Reloads a specific attachment menu bot from the server.
    ///
    /// Updates the cache and sends an update if the bot data has changed.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The bot user ID
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_attach_menu_manager::AttachMenuManager;
    /// # use rustgram_types::UserId;
    /// # use rustgram_attach_menu_manager::Error;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// let manager = AttachMenuManager::new();
    /// manager.reload_attach_menu_bot(UserId(123)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reload_attach_menu_bot(&self, _user_id: UserId) -> Result<()> {
        if !self.is_active() {
            return Err(crate::error::AttachMenuManagerError::NotActive);
        }

        // TODO: Implement actual network query
        Ok(())
    }

    /// Toggles whether a bot is added to the attachment menu.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The bot user ID
    /// * `is_added` - Whether to add (true) or remove (false) the bot
    /// * `allow_write_access` - Whether to allow the bot to send messages
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_attach_menu_manager::AttachMenuManager;
    /// # use rustgram_types::UserId;
    /// # use rustgram_attach_menu_manager::Error;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// let manager = AttachMenuManager::new();
    /// manager.toggle_bot_is_added_to_attach_menu(UserId(123), true, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn toggle_bot_is_added_to_attach_menu(
        &self,
        user_id: UserId,
        is_added: bool,
        _allow_write_access: bool,
    ) -> Result<()> {
        if !self.is_active() {
            return Err(crate::error::AttachMenuManagerError::NotActive);
        }

        if is_added {
            // TODO: Check if bot can_be_added_to_attach_menu
            return Err(crate::error::AttachMenuManagerError::BotNotSupported);
        } else {
            self.remove_bot_from_attach_menu(user_id).await;
        }

        // TODO: Send network query
        Ok(())
    }

    /// Gets the current hash value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuManager;
    ///
    /// let manager = AttachMenuManager::new();
    /// let hash = manager.get_hash();
    /// ```
    #[must_use]
    pub async fn get_hash(&self) -> i64 {
        *self.hash.read().await
    }

    /// Gets the cached attach menu bots.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuManager;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let manager = AttachMenuManager::new();
    ///     let bots = manager.get_attach_menu_bots().await;
    ///     assert!(bots.is_empty());
    /// }
    /// ```
    #[must_use]
    pub async fn get_attach_menu_bots(&self) -> Vec<AttachMenuBot> {
        self.attach_menu_bots.read().await.clone()
    }

    /// Removes a bot from the attachment menu cache.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The bot user ID to remove
    async fn remove_bot_from_attach_menu(&self, user_id: UserId) {
        let mut bots = self.attach_menu_bots.write().await;
        if let Some(pos) = bots.iter().position(|b| b.user_id == user_id) {
            bots.remove(pos);
            // Reset hash to force reload
            let mut hash = self.hash.write().await;
            *hash = 0;
            // TODO: Send update
            // TODO: Save to database
        }
    }

    /// Processes a TL attach menu bot response.
    ///
    /// # Arguments
    ///
    /// * `tl_bot` - The TL bot response
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuManager;
    /// use rustgram_attach_menu_manager::tl::{TlAttachMenuBot, AttachMenuBotIcon};
    /// use rustgram_types::UserId;
    /// use rustgram_file_id::FileId;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let manager = AttachMenuManager::new();
    ///     let mut tl_bot = TlAttachMenuBot::new(123, "TestBot".to_string(), false, false, true);
    ///     tl_bot.icons.push(AttachMenuBotIcon::new(
    ///         "default_static".to_string(),
    ///         FileId::new(1, 0),
    ///         vec![],
    ///     ));
    ///     let bot = manager.process_tl_bot(&tl_bot).unwrap();
    ///     assert_eq!(bot.user_id, UserId(123));
    /// }
    /// ```
    pub fn process_tl_bot(&self, tl_bot: &TlAttachMenuBot) -> Result<AttachMenuBot> {
        // Validate user ID
        let user_id = UserId(tl_bot.bot_id);
        if !user_id.is_valid() {
            return Err(crate::error::AttachMenuManagerError::InvalidResponse);
        }

        // Convert TL bot to AttachMenuBot
        let bot = tl_bot.to_attach_menu_bot();

        // Validate default icon
        if !bot.default_icon_file_id.is_valid() {
            return Err(crate::error::AttachMenuManagerError::InvalidIcon);
        }

        // Register file source
        let file_source_id = self.get_attach_menu_bot_file_source_id(user_id);
        if file_source_id.is_valid() {
            // TODO: Register all icon file IDs with this source
        }

        Ok(bot)
    }

    /// Updates the attach menu bots cache.
    ///
    /// # Arguments
    ///
    /// * `new_bots` - The new list of bots
    /// * `new_hash` - The new hash value
    pub async fn update_cache(&self, new_bots: Vec<AttachMenuBot>, new_hash: i64) {
        let mut bots = self.attach_menu_bots.write().await;
        let mut hash = self.hash.write().await;

        let need_update = new_bots != *bots;

        if need_update || *hash != new_hash {
            *hash = new_hash;
            *bots = new_bots;

            if need_update {
                // TODO: Send update
            }

            // TODO: Save to database
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tl::{AttachMenuBotIcon, AttachMenuPeerType};
    use rustgram_file_id::FileId;
    use rstest::rstest;

    // === Constructor tests ===

    #[test]
    fn test_manager_new() {
        let manager = AttachMenuManager::new();
        assert!(!manager.is_inited());
        assert!(!manager.is_active());
    }

    #[test]
    fn test_manager_default() {
        let manager = AttachMenuManager::default();
        assert!(!manager.is_inited());
        assert!(!manager.is_active());
    }

    #[tokio::test]
    async fn test_manager_clone() {
        let manager = AttachMenuManager::new();
        let cloned = manager.clone();
        assert!(!cloned.is_inited());
    }

    // === Initialization tests ===

    #[tokio::test]
    async fn test_manager_init() {
        let manager = AttachMenuManager::new();
        assert!(!manager.is_inited());

        manager.init().await;
        assert!(manager.is_inited());
        assert!(manager.is_active());
    }

    #[tokio::test]
    async fn test_manager_init_idempotent() {
        let manager = AttachMenuManager::new();
        manager.init().await;
        manager.init().await; // Should not panic
        assert!(manager.is_inited());
    }

    // === State tests ===

    #[tokio::test]
    async fn test_get_hash_initial() {
        let manager = AttachMenuManager::new();
        let hash = manager.get_hash().await;
        assert_eq!(hash, 0);
    }

    #[tokio::test]
    async fn test_get_bots_initial() {
        let manager = AttachMenuManager::new();
        let bots = manager.get_attach_menu_bots().await;
        assert!(bots.is_empty());
    }

    #[tokio::test]
    async fn test_update_cache() {
        let manager = AttachMenuManager::new();
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        manager.update_cache(vec![bot.clone()], 12345).await;

        let bots = manager.get_attach_menu_bots().await;
        assert_eq!(bots.len(), 1);
        assert_eq!(bots[0], bot);

        let hash = manager.get_hash().await;
        assert_eq!(hash, 12345);
    }

    // === Database key tests ===

    #[test]
    fn test_database_key() {
        assert_eq!(
            AttachMenuManager::get_attach_menu_bots_database_key(),
            "attach_bots"
        );
    }

    // === File source ID tests ===

    #[test]
    fn test_file_source_id_invalid_user() {
        let manager = AttachMenuManager::new();
        let source_id = manager.get_attach_menu_bot_file_source_id(UserId(0));
        assert!(!source_id.is_valid());
    }

    #[test]
    fn test_file_source_id_not_active() {
        let manager = AttachMenuManager::new();
        let source_id = manager.get_attach_menu_bot_file_source_id(UserId(123));
        assert!(!source_id.is_valid());
    }

    #[tokio::test]
    async fn test_file_source_id_active() {
        let manager = AttachMenuManager::new();
        manager.init().await;

        let source_id = manager.get_attach_menu_bot_file_source_id(UserId(123));
        assert!(source_id.is_valid());
    }

    #[tokio::test]
    async fn test_file_source_id_cached() {
        let manager = AttachMenuManager::new();
        manager.init().await;

        let source_id1 = manager.get_attach_menu_bot_file_source_id(UserId(123));
        let source_id2 = manager.get_attach_menu_bot_file_source_id(UserId(123));
        assert_eq!(source_id1, source_id2);
    }

    #[tokio::test]
    async fn test_file_source_id_different_users() {
        let manager = AttachMenuManager::new();
        manager.init().await;

        let source_id1 = manager.get_attach_menu_bot_file_source_id(UserId(123));
        let source_id2 = manager.get_attach_menu_bot_file_source_id(UserId(456));
        assert_ne!(source_id1, source_id2);
    }

    // === Error handling tests ===

    #[tokio::test]
    async fn test_reload_not_active() {
        let manager = AttachMenuManager::new();
        let result = manager.reload_attach_menu_bots().await;
        assert_eq!(result, Err(crate::error::AttachMenuManagerError::NotActive));
    }

    #[tokio::test]
    async fn test_get_bot_not_active() {
        let manager = AttachMenuManager::new();
        let result = manager.get_attach_menu_bot(UserId(123)).await;
        assert_eq!(result, Err(crate::error::AttachMenuManagerError::NotActive));
    }

    #[tokio::test]
    async fn test_reload_bot_not_active() {
        let manager = AttachMenuManager::new();
        let result = manager.reload_attach_menu_bot(UserId(123)).await;
        assert_eq!(result, Err(crate::error::AttachMenuManagerError::NotActive));
    }

    #[tokio::test]
    async fn test_toggle_bot_not_active() {
        let manager = AttachMenuManager::new();
        let result = manager
            .toggle_bot_is_added_to_attach_menu(UserId(123), true, false)
            .await;
        assert_eq!(result, Err(crate::error::AttachMenuManagerError::NotActive));
    }

    #[tokio::test]
    async fn test_toggle_bot_add_not_supported() {
        let manager = AttachMenuManager::new();
        manager.init().await;

        let result = manager
            .toggle_bot_is_added_to_attach_menu(UserId(123), true, false)
            .await;
        assert_eq!(result, Err(crate::error::AttachMenuManagerError::BotNotSupported));
    }

    #[tokio::test]
    async fn test_toggle_bot_remove() {
        let manager = AttachMenuManager::new();
        manager.init().await;

        let mut bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        bot.is_added = true;
        manager.update_cache(vec![bot.clone()], 0).await;

        let result = manager
            .toggle_bot_is_added_to_attach_menu(UserId(123), false, false)
            .await;
        assert!(result.is_ok());

        let bots = manager.get_attach_menu_bots().await;
        assert!(bots.is_empty());
    }

    // === TL bot processing tests ===

    #[test]
    fn test_process_tl_bot_valid() {
        let manager = AttachMenuManager::new();
        let mut tl_bot = TlAttachMenuBot::new(123, "TestBot".to_string(), false, false, true);
        tl_bot.icons.push(AttachMenuBotIcon::new(
            "default_static".to_string(),
            FileId::new(1, 0),
            vec![],
        ));

        let result = manager.process_tl_bot(&tl_bot);
        assert!(result.is_ok());
        let bot = result.unwrap();
        assert_eq!(bot.user_id, UserId(123));
        assert_eq!(bot.name, "TestBot");
    }

    #[test]
    fn test_process_tl_bot_invalid_user_id() {
        let manager = AttachMenuManager::new();
        let tl_bot = TlAttachMenuBot::new(0, "TestBot".to_string(), false, false, true);

        let result = manager.process_tl_bot(&tl_bot);
        assert_eq!(result, Err(crate::error::AttachMenuManagerError::InvalidResponse));
    }

    #[test]
    fn test_process_tl_bot_missing_icon() {
        let manager = AttachMenuManager::new();
        let tl_bot = TlAttachMenuBot::new(123, "TestBot".to_string(), false, false, true);

        let result = manager.process_tl_bot(&tl_bot);
        assert_eq!(result, Err(crate::error::AttachMenuManagerError::InvalidIcon));
    }

    #[test]
    fn test_process_tl_bot_with_all_peer_types() {
        let manager = AttachMenuManager::new();
        let mut tl_bot = TlAttachMenuBot::new(123, "TestBot".to_string(), false, false, true);
        tl_bot.peer_types = vec![
            AttachMenuPeerType::SameBotPm,
            AttachMenuPeerType::BotPm,
            AttachMenuPeerType::Pm,
            AttachMenuPeerType::Chat,
            AttachMenuPeerType::Broadcast,
        ];
        tl_bot.icons.push(AttachMenuBotIcon::new(
            "default_static".to_string(),
            FileId::new(1, 0),
            vec![],
        ));

        let result = manager.process_tl_bot(&tl_bot);
        assert!(result.is_ok());
        let bot = result.unwrap();
        assert!(bot.supports_self_dialog);
        assert!(bot.supports_bot_dialogs);
        assert!(bot.supports_user_dialogs);
        assert!(bot.supports_group_dialogs);
        assert!(bot.supports_broadcast_dialogs);
    }

    #[test]
    fn test_process_tl_bot_with_colors() {
        let manager = AttachMenuManager::new();
        let mut tl_bot = TlAttachMenuBot::new(123, "TestBot".to_string(), false, false, true);
        // Add default_static icon (required for validity)
        tl_bot.icons.push(AttachMenuBotIcon::new(
            "default_static".to_string(),
            FileId::new(1, 0),
            vec![],
        ));
        // Add android_animated icon with colors
        tl_bot.icons.push(AttachMenuBotIcon::new(
            "android_animated".to_string(),
            FileId::new(2, 0),
            vec![
                crate::tl::AttachMenuBotIconColor::new("light_icon".to_string(), 0x111111),
                crate::tl::AttachMenuBotIconColor::new("light_text".to_string(), 0x222222),
                crate::tl::AttachMenuBotIconColor::new("dark_icon".to_string(), 0x333333),
                crate::tl::AttachMenuBotIconColor::new("dark_text".to_string(), 0x444444),
            ],
        ));

        let result = manager.process_tl_bot(&tl_bot);
        assert!(result.is_ok());
        let bot = result.unwrap();
        assert_eq!(bot.icon_color.light_color, 0x111111);
        assert_eq!(bot.name_color.light_color, 0x222222);
        assert_eq!(bot.icon_color.dark_color, 0x333333);
        assert_eq!(bot.name_color.dark_color, 0x444444);
    }

    // === Cache update tests ===

    #[tokio::test]
    async fn test_update_cache_no_change() {
        let manager = AttachMenuManager::new();
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        manager.update_cache(vec![bot.clone()], 12345).await;

        // Update with same data
        manager.update_cache(vec![bot.clone()], 12345).await;

        let bots = manager.get_attach_menu_bots().await;
        assert_eq!(bots.len(), 1);
    }

    #[tokio::test]
    async fn test_update_cache_hash_change_only() {
        let manager = AttachMenuManager::new();
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        manager.update_cache(vec![bot.clone()], 12345).await;

        // Update with same bots but different hash
        manager.update_cache(vec![bot.clone()], 54321).await;

        let hash = manager.get_hash().await;
        assert_eq!(hash, 54321);
    }

    #[tokio::test]
    async fn test_update_cache_bots_change() {
        let manager = AttachMenuManager::new();
        let bot1 = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        manager.update_cache(vec![bot1.clone()], 12345).await;

        let bot2 = AttachMenuBot::new(UserId(456), "TestBot2", FileId::new(2, 0));
        manager.update_cache(vec![bot2.clone()], 54321).await;

        let bots = manager.get_attach_menu_bots().await;
        assert_eq!(bots.len(), 1);
        assert_eq!(bots[0].user_id, UserId(456));
    }

    // === Remove bot tests ===

    #[tokio::test]
    async fn test_remove_bot_from_cache() {
        let manager = AttachMenuManager::new();
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        manager.update_cache(vec![bot.clone()], 12345).await;

        manager.remove_bot_from_attach_menu(UserId(123)).await;

        let bots = manager.get_attach_menu_bots().await;
        assert!(bots.is_empty());

        let hash = manager.get_hash().await;
        assert_eq!(hash, 0); // Hash reset to force reload
    }

    #[tokio::test]
    async fn test_remove_bot_not_in_cache() {
        let manager = AttachMenuManager::new();
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        manager.update_cache(vec![bot.clone()], 12345).await;

        // Try to remove different bot
        manager.remove_bot_from_attach_menu(UserId(456)).await;

        let bots = manager.get_attach_menu_bots().await;
        assert_eq!(bots.len(), 1); // Bot still there
    }

    // === Integration tests ===

    #[tokio::test]
    async fn test_full_workflow() {
        let manager = AttachMenuManager::new();
        manager.init().await;

        // Add bot to cache
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        manager.update_cache(vec![bot.clone()], 12345).await;

        // Verify bot is in cache
        let bots = manager.get_attach_menu_bots().await;
        assert_eq!(bots.len(), 1);

        // Get file source ID
        let source_id = manager.get_attach_menu_bot_file_source_id(UserId(123));
        assert!(source_id.is_valid());

        // Remove bot
        manager.remove_bot_from_attach_menu(UserId(123)).await;

        // Verify bot is removed
        let bots = manager.get_attach_menu_bots().await;
        assert!(bots.is_empty());
    }

    #[rstest]
    #[case(UserId(0), false)]
    #[case(UserId(123), true)]
    fn test_user_id_validity(#[case] user_id: UserId, #[case] expected: bool) {
        assert_eq!(user_id.is_valid(), expected);
    }
}
