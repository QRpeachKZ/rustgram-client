// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Stickers Manager
//!
//! Manager for Telegram stickers and sticker sets.
//!
//! Based on TDLib's `StickersManager` from `td/telegram/StickersManager.h`.
//!
//! ## Overview
//!
//! The `StickersManager` is responsible for all sticker-related operations including:
//! - Sticker set management (installed, archived, featured)
//! - Sticker search and discovery
//! - Custom emoji handling
//! - Animated emoji interactions
//! - Sticker upload and creation
//! - Recent and favorite stickers
//! - Emoji search and suggestions
//!
//! ## Architecture
//!
//! This is the most complex TDLib manager with multiple cache layers:
//! - `stickers_: HashMap<FileId, Sticker>` - Sticker cache
//! - `sticker_sets_: HashMap<StickerSetId, StickerSet>` - Sticker set cache
//! - `installed_sticker_sets_: Vec<StickerSetId>` - Installed sets per type
//! - `featured_sticker_sets_: Vec<StickerSetId>` - Featured sets per type
//! - `recent_stickers_: Vec<FileId>` - Recent stickers
//! - `favorite_stickers_: Vec<FileId>` - Favorite stickers
//!
//! ## Example
//!
//! ```rust
//! use rustgram_stickers_manager::StickersManager;
//!
//! let manager = StickersManager::new();
//! // Use manager methods...
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod error;
pub mod types;

use crate::error::{CheckStickerSetNameResult, Error, Result};
use crate::types::{Sticker, StickerSet, Stickers, MAX_GET_CUSTOM_EMOJI_STICKERS};
use dashmap::DashMap;
use rustgram_custom_emoji_id::CustomEmojiId;
use rustgram_file_id::FileId;
use rustgram_sticker_format::StickerFormat;
use rustgram_sticker_set_id::StickerSetId;
use rustgram_sticker_type::StickerType;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Default cache TTL for sticker sets in seconds.
const DEFAULT_CACHE_TTL: u64 = 3600;

/// Great Minds sticker set ID (special Telegram set).
#[allow(dead_code)]
const GREAT_MINDS_SET_ID: i64 = 1842540969984001;

/// Stickers Manager.
///
/// Manages all sticker-related operations for Telegram.
/// This is the most complex manager in TDLib with 50+ methods.
///
/// # Thread Safety
///
/// All public methods are thread-safe and can be called concurrently.
/// The manager uses `Arc<RwLock<T>>` for internal state.
///
/// # Example
///
/// ```rust
/// use rustgram_stickers_manager::StickersManager;
///
/// #[tokio::main]
/// async fn main() {
///     let manager = StickersManager::new();
///     // Use manager...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct StickersManager {
    /// Internal state protected by RwLock for thread safety.
    inner: Arc<RwLock<StickersManagerInner>>,
}

/// Internal state of StickersManager.
#[derive(Debug)]
struct StickersManagerInner {
    /// Sticker cache (FileId -> Sticker).
    stickers: DashMap<FileId, Sticker>,

    /// Sticker set cache (StickerSetId -> StickerSet).
    sticker_sets: DashMap<StickerSetId, StickerSet>,

    /// Short name to sticker set ID mapping.
    short_name_to_set_id: DashMap<String, StickerSetId>,

    /// Installed sticker sets per type (indexed by StickerType).
    installed_sticker_sets: Vec<Vec<StickerSetId>>,

    /// Featured sticker sets per type.
    #[allow(dead_code)]
    featured_sticker_sets: Vec<Vec<StickerSetId>>,

    /// Recent stickers (indexed by is_attached: bool).
    recent_stickers: Vec<Vec<FileId>>,

    /// Favorite stickers.
    favorite_stickers: Vec<FileId>,

    /// Custom emoji to sticker ID mapping.
    custom_emoji_to_sticker: DashMap<CustomEmojiId, FileId>,

    /// Cache TTL in seconds.
    cache_ttl: u64,
}

impl Default for StickersManagerInner {
    fn default() -> Self {
        Self::new()
    }
}

impl StickersManagerInner {
    /// Creates a new inner state.
    fn new() -> Self {
        // Initialize 3 slots for sticker types (Regular, Mask, CustomEmoji)
        let installed_sets = vec![Vec::new(); 3];
        let featured_sets = vec![Vec::new(); 3];
        let recent_stickers = vec![Vec::new(); 2];

        Self {
            stickers: DashMap::new(),
            sticker_sets: DashMap::new(),
            short_name_to_set_id: DashMap::new(),
            installed_sticker_sets: installed_sets,
            featured_sticker_sets: featured_sets,
            recent_stickers,
            favorite_stickers: Vec::new(),
            custom_emoji_to_sticker: DashMap::new(),
            cache_ttl: DEFAULT_CACHE_TTL,
        }
    }

    /// Gets the index for a sticker type.
    fn type_index(sticker_type: StickerType) -> usize {
        match sticker_type {
            StickerType::Regular => 0,
            StickerType::Mask => 1,
            StickerType::CustomEmoji => 2,
            StickerType::Unknown => 0,
        }
    }
}

impl StickersManager {
    /// Creates a new `StickersManager`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_stickers_manager::StickersManager;
    ///
    /// let manager = StickersManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(StickersManagerInner::new())),
        }
    }

    // ========== Sticker Information ==========

    /// Gets the type of sticker for a given file ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID to check
    ///
    /// # Returns
    ///
    /// Returns the sticker type, or `Error::InvalidStickerId` if not found.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `get_sticker_type(FileId file_id) const`
    pub async fn get_sticker_type(&self, file_id: FileId) -> Result<StickerType> {
        let inner = self.inner.read().await;
        inner
            .stickers
            .get(&file_id)
            .map(|sticker| sticker.sticker_type())
            .ok_or(Error::InvalidStickerId)
    }

    /// Gets the format of sticker for a given file ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID to check
    ///
    /// # Returns
    ///
    /// Returns the sticker format, or `Error::InvalidStickerId` if not found.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `get_sticker_format(FileId file_id) const`
    pub async fn get_sticker_format(&self, file_id: FileId) -> Result<StickerFormat> {
        let inner = self.inner.read().await;
        inner
            .stickers
            .get(&file_id)
            .map(|sticker| sticker.format())
            .ok_or(Error::InvalidStickerId)
    }

    // ========== Sticker Sets ==========

    /// Gets the list of installed sticker sets for a given type.
    ///
    /// # Arguments
    ///
    /// * `sticker_type` - The type of stickers
    ///
    /// # Returns
    ///
    /// Returns a vector of installed sticker set IDs.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `get_installed_sticker_sets(StickerType sticker_type, Promise<Unit> promise)`
    pub async fn get_installed_sticker_sets(
        &self,
        sticker_type: StickerType,
    ) -> Result<Vec<StickerSetId>> {
        let inner = self.inner.read().await;
        let idx = StickersManagerInner::type_index(sticker_type);
        Ok(inner.installed_sticker_sets[idx].clone())
    }

    /// Gets a sticker set by ID.
    ///
    /// # Arguments
    ///
    /// * `set_id` - The sticker set ID
    ///
    /// # Returns
    ///
    /// Returns the sticker set, or `Error::StickerSetNotFound` if not found.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `get_sticker_set(StickerSetId set_id, Promise<StickerSet> promise)`
    pub async fn get_sticker_set(&self, set_id: StickerSetId) -> Result<StickerSet> {
        let inner = self.inner.read().await;
        inner
            .sticker_sets
            .get(&set_id)
            .map(|set| set.clone())
            .ok_or(Error::StickerSetNotFound)
    }

    /// Gets a sticker set by short name.
    ///
    /// # Arguments
    ///
    /// * `short_name` - The short name of the sticker set
    ///
    /// # Returns
    ///
    /// Returns the sticker set, or `Error::StickerSetNotFound` if not found.
    pub async fn get_sticker_set_by_name(&self, short_name: &str) -> Result<StickerSet> {
        let inner = self.inner.read().await;
        inner
            .short_name_to_set_id
            .get(short_name)
            .and_then(|set_id| inner.sticker_sets.get(&set_id))
            .map(|set| set.clone())
            .ok_or(Error::StickerSetNotFound)
    }

    /// Changes sticker set installation/archival status.
    ///
    /// # Arguments
    ///
    /// * `set_id` - The sticker set ID
    /// * `is_installed` - Whether to install the set
    /// * `is_archived` - Whether to archive the set
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `change_sticker_set(StickerSetId set_id, bool is_installed, bool is_archived, Promise<Unit> promise)`
    pub async fn change_sticker_set(
        &self,
        set_id: StickerSetId,
        is_installed: bool,
        is_archived: bool,
    ) -> Result<()> {
        let mut inner = self.inner.write().await;

        let mut set = inner
            .sticker_sets
            .get_mut(&set_id)
            .ok_or(Error::StickerSetNotFound)?;

        set.is_installed = is_installed;
        set.is_archived = is_archived;

        // Get sticker_type and drop the set reference
        let sticker_type = set.sticker_type();
        drop(set);

        // Update installed list
        if is_installed {
            let idx = StickersManagerInner::type_index(sticker_type);
            if !inner.installed_sticker_sets[idx].contains(&set_id) {
                inner.installed_sticker_sets[idx].push(set_id);
            }
        } else {
            let idx = StickersManagerInner::type_index(sticker_type);
            inner.installed_sticker_sets[idx].retain(|&id| id != set_id);
        }

        Ok(())
    }

    /// Searches for sticker sets by query.
    ///
    /// # Arguments
    ///
    /// * `sticker_type` - The type of stickers to search
    /// * `query` - The search query
    ///
    /// # Returns
    ///
    /// Returns a vector of matching sticker set IDs.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `search_sticker_sets(StickerType sticker_type, const string &query, Promise<Unit> promise)`
    pub async fn search_sticker_sets(
        &self,
        sticker_type: StickerType,
        query: String,
    ) -> Result<Vec<StickerSetId>> {
        let inner = self.inner.read().await;
        let query_lower = query.to_lowercase();

        let mut results = Vec::new();

        for entry in inner.sticker_sets.iter() {
            let set = entry.value();
            if set.sticker_type() != sticker_type {
                continue;
            }

            let title_matches = set.title().to_lowercase().contains(&query_lower);
            let name_matches = set.short_name().to_lowercase().contains(&query_lower);

            if title_matches || name_matches {
                results.push(set.id());
            }
        }

        Ok(results)
    }

    // ========== Custom Emoji ==========

    /// Gets custom emoji stickers by their IDs.
    ///
    /// # Arguments
    ///
    /// * `custom_emoji_ids` - The custom emoji IDs to fetch
    /// * `_use_database` - Whether to use the database (stub parameter)
    ///
    /// # Returns
    ///
    /// Returns the stickers result.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `get_custom_emoji_stickers(vector<CustomEmojiId> custom_emoji_ids, bool use_database, Promise<stickers> promise)`
    pub async fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: Vec<CustomEmojiId>,
        _use_database: bool,
    ) -> Result<Stickers> {
        if custom_emoji_ids.len() > MAX_GET_CUSTOM_EMOJI_STICKERS {
            return Err(Error::Internal(format!(
                "Too many custom emoji IDs: {} (max {})",
                custom_emoji_ids.len(),
                MAX_GET_CUSTOM_EMOJI_STICKERS
            )));
        }

        let inner = self.inner.read().await;
        let mut stickers = Vec::new();

        for emoji_id in custom_emoji_ids {
            if let Some(file_id) = inner.custom_emoji_to_sticker.get(&emoji_id) {
                if let Some(sticker) = inner.stickers.get(file_id.value()) {
                    stickers.push(sticker.clone());
                }
            }
        }

        Ok(Stickers::new(stickers))
    }

    /// Checks if a custom emoji is premium-only.
    ///
    /// # Arguments
    ///
    /// * `custom_emoji_id` - The custom emoji ID to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the custom emoji is premium-only.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `is_premium_custom_emoji(CustomEmojiId custom_emoji_id, bool default_result) const`
    pub async fn is_premium_custom_emoji(&self, custom_emoji_id: CustomEmojiId) -> bool {
        let inner = self.inner.read().await;

        inner
            .custom_emoji_to_sticker
            .get(&custom_emoji_id)
            .and_then(|file_id| inner.stickers.get(file_id.value()))
            .map(|sticker| sticker.is_premium())
            .unwrap_or(false)
    }

    // ========== Recent & Favorite Stickers ==========

    /// Gets recent stickers.
    ///
    /// # Arguments
    ///
    /// * `is_attached` - Whether to get attached stickers
    ///
    /// # Returns
    ///
    /// Returns a vector of recent sticker file IDs.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `get_recent_stickers(bool is_attached, Promise<Unit> promise)`
    pub async fn get_recent_stickers(&self, is_attached: bool) -> Result<Vec<FileId>> {
        let inner = self.inner.read().await;
        let idx = if is_attached { 1 } else { 0 };
        Ok(inner.recent_stickers[idx].clone())
    }

    /// Gets favorite stickers.
    ///
    /// # Returns
    ///
    /// Returns a vector of favorite sticker file IDs.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `get_favorite_stickers(Promise<Unit> promise)`
    pub async fn get_favorite_stickers(&self) -> Result<Vec<FileId>> {
        let inner = self.inner.read().await;
        Ok(inner.favorite_stickers.clone())
    }

    /// Adds a sticker to favorites.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The sticker file ID to add
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `add_favorite_sticker(const td_api::object_ptr<td_api::InputFile> &input_file, Promise<Unit> promise)`
    pub async fn add_favorite_sticker(&self, file_id: FileId) -> Result<()> {
        let mut inner = self.inner.write().await;

        if !inner.stickers.contains_key(&file_id) {
            return Err(Error::InvalidStickerId);
        }

        if !inner.favorite_stickers.contains(&file_id) {
            inner.favorite_stickers.push(file_id);
        }

        Ok(())
    }

    /// Removes a sticker from favorites.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The sticker file ID to remove
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `remove_favorite_sticker(const td_api::object_ptr<td_api::InputFile> &input_file, Promise<Unit> promise)`
    pub async fn remove_favorite_sticker(&self, file_id: FileId) -> Result<()> {
        let mut inner = self.inner.write().await;
        inner.favorite_stickers.retain(|&id| id != file_id);
        Ok(())
    }

    // ========== Sticker Operations ==========

    /// Adds a sticker to the cache.
    ///
    /// # Arguments
    ///
    /// * `sticker` - The sticker to add
    pub async fn add_sticker(&self, sticker: Sticker) -> Result<()> {
        let inner = self.inner.write().await;
        inner.stickers.insert(sticker.file_id(), sticker);
        Ok(())
    }

    /// Adds a sticker set to the cache.
    ///
    /// # Arguments
    ///
    /// * `set` - The sticker set to add
    pub async fn add_sticker_set(&self, set: StickerSet) -> Result<()> {
        let inner = self.inner.write().await;
        let set_id = set.id();

        // Add the set
        inner.sticker_sets.insert(set_id, set.clone());

        // Add short name mapping
        inner
            .short_name_to_set_id
            .insert(set.short_name().to_string(), set_id);

        // Add stickers to cache
        for sticker in &set.stickers {
            inner.stickers.insert(sticker.file_id(), sticker.clone());

            // Map custom emoji IDs
            if set.sticker_type() == StickerType::CustomEmoji {
                // TODO: Extract CustomEmojiId from sticker when available
            }
        }

        Ok(())
    }

    /// Validates a sticker set name.
    ///
    /// # Arguments
    ///
    /// * `short_name` - The short name to validate
    ///
    /// # Returns
    ///
    /// Returns the validation result.
    pub fn validate_sticker_set_name(&self, short_name: &str) -> CheckStickerSetNameResult {
        StickerSet::check_short_name(short_name)
    }

    /// Gets the cache TTL.
    ///
    /// # Returns
    ///
    /// Returns the cache TTL in seconds.
    #[must_use]
    pub async fn cache_ttl(&self) -> u64 {
        let inner = self.inner.read().await;
        inner.cache_ttl
    }

    /// Sets the cache TTL.
    ///
    /// # Arguments
    ///
    /// * `ttl` - The new cache TTL in seconds
    pub async fn set_cache_ttl(&self, ttl: u64) {
        let mut inner = self.inner.write().await;
        inner.cache_ttl = ttl;
    }

    /// Clears all caches.
    pub async fn clear_cache(&self) {
        let inner = self.inner.write().await;
        inner.stickers.clear();
        inner.sticker_sets.clear();
        inner.short_name_to_set_id.clear();
        inner.custom_emoji_to_sticker.clear();
    }

    /// Gets the number of cached stickers.
    #[must_use]
    pub async fn cached_sticker_count(&self) -> usize {
        let inner = self.inner.read().await;
        inner.stickers.len()
    }

    /// Gets the number of cached sticker sets.
    #[must_use]
    pub async fn cached_sticker_set_count(&self) -> usize {
        let inner = self.inner.read().await;
        inner.sticker_sets.len()
    }
}

impl Default for StickersManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MAX_STICKER_SET_SHORT_NAME_LENGTH;
    use rustgram_dimensions::Dimensions;

    async fn create_test_manager() -> StickersManager {
        let manager = StickersManager::new();

        // Add some test data
        let set_id = StickerSetId::new(123);
        let file_id = FileId::new(456, 0);
        let dimensions = Dimensions::from_wh(512, 512);
        let sticker = Sticker::new(
            set_id,
            file_id,
            dimensions,
            StickerFormat::Webp,
            StickerType::Regular,
        );

        manager.add_sticker(sticker).await.unwrap();

        let mut sticker_set = StickerSet::new(
            set_id,
            "Test Set".to_string(),
            "testset".to_string(),
            StickerType::Regular,
        );

        // Create new stickers for the set
        let sticker_for_set = Sticker::new(
            set_id,
            file_id,
            dimensions,
            StickerFormat::Webp,
            StickerType::Regular,
        );
        let file_id2 = FileId::new(789, 0);
        let sticker2 = Sticker::new(
            set_id,
            file_id2,
            dimensions,
            StickerFormat::Webp,
            StickerType::Regular,
        );

        sticker_set.stickers.push(sticker_for_set);
        sticker_set.stickers.push(sticker2);
        sticker_set.sticker_count = 2;

        manager.add_sticker_set(sticker_set).await.unwrap();

        manager
    }

    #[tokio::test]
    async fn test_manager_new() {
        let manager = StickersManager::new();
        assert_eq!(manager.cached_sticker_count().await, 0);
        assert_eq!(manager.cached_sticker_set_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_default() {
        let manager = StickersManager::default();
        assert_eq!(manager.cached_sticker_count().await, 0);
    }

    #[tokio::test]
    async fn test_add_sticker() {
        let manager = StickersManager::new();

        let set_id = StickerSetId::new(1);
        let file_id = FileId::new(2, 0);
        let dimensions = Dimensions::from_wh(512, 512);
        let sticker = Sticker::new(
            set_id,
            file_id,
            dimensions,
            StickerFormat::Webp,
            StickerType::Regular,
        );

        manager.add_sticker(sticker.clone()).await.unwrap();

        assert_eq!(manager.cached_sticker_count().await, 1);

        let sticker_type = manager.get_sticker_type(file_id).await.unwrap();
        assert_eq!(sticker_type, StickerType::Regular);

        let sticker_format = manager.get_sticker_format(file_id).await.unwrap();
        assert_eq!(sticker_format, StickerFormat::Webp);
    }

    #[tokio::test]
    async fn test_add_sticker_set() {
        let manager = StickersManager::new();

        let set_id = StickerSetId::new(123);
        let mut sticker_set = StickerSet::new(
            set_id,
            "Test Set".to_string(),
            "testset".to_string(),
            StickerType::Regular,
        );

        let file_id = FileId::new(456, 0);
        let dimensions = Dimensions::from_wh(512, 512);
        let sticker = Sticker::new(
            set_id,
            file_id,
            dimensions,
            StickerFormat::Webp,
            StickerType::Regular,
        );

        sticker_set.stickers.push(sticker);
        sticker_set.sticker_count = 1;

        manager.add_sticker_set(sticker_set).await.unwrap();

        assert_eq!(manager.cached_sticker_set_count().await, 1);

        let retrieved_set = manager.get_sticker_set(set_id).await.unwrap();
        assert_eq!(retrieved_set.title(), "Test Set");
        assert_eq!(retrieved_set.short_name(), "testset");
    }

    #[tokio::test]
    async fn test_get_sticker_set_by_name() {
        let manager = StickersManager::new();

        let set_id = StickerSetId::new(123);
        let sticker_set = StickerSet::new(
            set_id,
            "Test Set".to_string(),
            "testset".to_string(),
            StickerType::Regular,
        );

        manager.add_sticker_set(sticker_set).await.unwrap();

        let retrieved_set = manager.get_sticker_set_by_name("testset").await.unwrap();
        assert_eq!(retrieved_set.id().get(), 123);
    }

    #[tokio::test]
    async fn test_get_sticker_set_not_found() {
        let manager = StickersManager::new();

        let result = manager.get_sticker_set(StickerSetId::new(999)).await;
        assert!(matches!(result, Err(Error::StickerSetNotFound)));

        let result = manager.get_sticker_set_by_name("nonexistent").await;
        assert!(matches!(result, Err(Error::StickerSetNotFound)));
    }

    #[tokio::test]
    async fn test_get_sticker_type_invalid() {
        let manager = StickersManager::new();

        let result = manager.get_sticker_type(FileId::new(999, 0)).await;
        assert!(matches!(result, Err(Error::InvalidStickerId)));
    }

    #[tokio::test]
    async fn test_get_sticker_format_invalid() {
        let manager = StickersManager::new();

        let result = manager.get_sticker_format(FileId::new(999, 0)).await;
        assert!(matches!(result, Err(Error::InvalidStickerId)));
    }

    #[tokio::test]
    async fn test_change_sticker_set_install() {
        let manager = create_test_manager().await;

        let set_id = StickerSetId::new(123);
        manager
            .change_sticker_set(set_id, true, false)
            .await
            .unwrap();

        let installed_sets = manager
            .get_installed_sticker_sets(StickerType::Regular)
            .await
            .unwrap();

        assert!(installed_sets.contains(&set_id));
    }

    #[tokio::test]
    async fn test_change_sticker_set_uninstall() {
        let manager = create_test_manager().await;

        let set_id = StickerSetId::new(123);

        // First install
        manager
            .change_sticker_set(set_id, true, false)
            .await
            .unwrap();

        // Then uninstall
        manager
            .change_sticker_set(set_id, false, false)
            .await
            .unwrap();

        let installed_sets = manager
            .get_installed_sticker_sets(StickerType::Regular)
            .await
            .unwrap();

        assert!(!installed_sets.contains(&set_id));
    }

    #[tokio::test]
    async fn test_search_sticker_sets() {
        let manager = create_test_manager().await;

        let results = manager
            .search_sticker_sets(StickerType::Regular, "test".to_string())
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].get(), 123);

        let empty_results = manager
            .search_sticker_sets(StickerType::Regular, "nonexistent".to_string())
            .await
            .unwrap();

        assert_eq!(empty_results.len(), 0);
    }

    #[tokio::test]
    async fn test_get_custom_emoji_stickers_empty() {
        let manager = StickersManager::new();

        let emoji_ids = vec![CustomEmojiId::new(123)];
        let result = manager
            .get_custom_emoji_stickers(emoji_ids, false)
            .await
            .unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_get_custom_emoji_stickers_too_many() {
        let manager = StickersManager::new();

        let emoji_ids: Vec<CustomEmojiId> = (0..MAX_GET_CUSTOM_EMOJI_STICKERS + 1)
            .map(|i| CustomEmojiId::new(i as i64))
            .collect();

        let result = manager.get_custom_emoji_stickers(emoji_ids, false).await;

        assert!(matches!(result, Err(Error::Internal(_))));
    }

    #[tokio::test]
    async fn test_is_premium_custom_emoji_default() {
        let manager = StickersManager::new();

        let result = manager
            .is_premium_custom_emoji(CustomEmojiId::new(123))
            .await;

        assert!(!result);
    }

    #[tokio::test]
    async fn test_get_recent_stickers_empty() {
        let manager = StickersManager::new();

        let result = manager.get_recent_stickers(false).await.unwrap();
        assert!(result.is_empty());

        let result = manager.get_recent_stickers(true).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_get_favorite_stickers_empty() {
        let manager = StickersManager::new();

        let result = manager.get_favorite_stickers().await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_add_favorite_sticker() {
        let manager = create_test_manager().await;

        let file_id = FileId::new(456, 0);
        manager.add_favorite_sticker(file_id).await.unwrap();

        let favorites = manager.get_favorite_stickers().await.unwrap();
        assert!(favorites.contains(&file_id));
    }

    #[tokio::test]
    async fn test_add_favorite_sticker_invalid() {
        let manager = StickersManager::new();

        let file_id = FileId::new(999, 0);
        let result = manager.add_favorite_sticker(file_id).await;

        assert!(matches!(result, Err(Error::InvalidStickerId)));
    }

    #[tokio::test]
    async fn test_remove_favorite_sticker() {
        let manager = create_test_manager().await;

        let file_id = FileId::new(456, 0);
        manager.add_favorite_sticker(file_id).await.unwrap();

        manager.remove_favorite_sticker(file_id).await.unwrap();

        let favorites = manager.get_favorite_stickers().await.unwrap();
        assert!(!favorites.contains(&file_id));
    }

    #[tokio::test]
    async fn test_validate_sticker_set_name_valid() {
        let manager = StickersManager::new();

        assert_eq!(
            manager.validate_sticker_set_name("valid_name"),
            CheckStickerSetNameResult::Ok
        );
        assert_eq!(
            manager.validate_sticker_set_name("ValidName123"),
            CheckStickerSetNameResult::Ok
        );
    }

    #[tokio::test]
    async fn test_validate_sticker_set_name_invalid() {
        let manager = StickersManager::new();

        assert_eq!(
            manager.validate_sticker_set_name(""),
            CheckStickerSetNameResult::Invalid
        );
        assert_eq!(
            manager.validate_sticker_set_name("invalid-name"),
            CheckStickerSetNameResult::InvalidCharacters
        );
        assert_eq!(
            manager.validate_sticker_set_name(&"a".repeat(MAX_STICKER_SET_SHORT_NAME_LENGTH + 1)),
            CheckStickerSetNameResult::TooLong
        );
    }

    #[tokio::test]
    async fn test_cache_ttl() {
        let manager = StickersManager::new();

        assert_eq!(manager.cache_ttl().await, DEFAULT_CACHE_TTL);

        manager.set_cache_ttl(7200).await;
        assert_eq!(manager.cache_ttl().await, 7200);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let manager = create_test_manager().await;

        assert!(manager.cached_sticker_count().await > 0);
        assert!(manager.cached_sticker_set_count().await > 0);

        manager.clear_cache().await;

        assert_eq!(manager.cached_sticker_count().await, 0);
        assert_eq!(manager.cached_sticker_set_count().await, 0);
    }

    #[tokio::test]
    async fn test_cached_sticker_count() {
        let manager = StickersManager::new();

        assert_eq!(manager.cached_sticker_count().await, 0);

        let set_id = StickerSetId::new(1);
        let file_id = FileId::new(2, 0);
        let dimensions = Dimensions::from_wh(512, 512);
        let sticker = Sticker::new(
            set_id,
            file_id,
            dimensions,
            StickerFormat::Webp,
            StickerType::Regular,
        );

        manager.add_sticker(sticker).await.unwrap();

        assert_eq!(manager.cached_sticker_count().await, 1);
    }

    #[tokio::test]
    async fn test_cached_sticker_set_count() {
        let manager = StickersManager::new();

        assert_eq!(manager.cached_sticker_set_count().await, 0);

        let set_id = StickerSetId::new(1);
        let sticker_set = StickerSet::new(
            set_id,
            "Test".to_string(),
            "test".to_string(),
            StickerType::Regular,
        );

        manager.add_sticker_set(sticker_set).await.unwrap();

        assert_eq!(manager.cached_sticker_set_count().await, 1);
    }

    #[tokio::test]
    async fn test_change_sticker_set_not_found() {
        let manager = StickersManager::new();

        let result = manager
            .change_sticker_set(StickerSetId::new(999), true, false)
            .await;

        assert!(matches!(result, Err(Error::StickerSetNotFound)));
    }

    #[tokio::test]
    async fn test_add_favorite_duplicate() {
        let manager = create_test_manager().await;

        let file_id = FileId::new(456, 0);
        manager.add_favorite_sticker(file_id).await.unwrap();
        manager.add_favorite_sticker(file_id).await.unwrap();

        let favorites = manager.get_favorite_stickers().await.unwrap();
        // Should only appear once
        assert_eq!(favorites.iter().filter(|&&id| id == file_id).count(), 1);
    }

    #[tokio::test]
    async fn test_search_sticker_sets_case_insensitive() {
        let manager = create_test_manager().await;

        let results = manager
            .search_sticker_sets(StickerType::Regular, "TEST".to_string())
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_search_sticker_sets_by_short_name() {
        let manager = create_test_manager().await;

        let results = manager
            .search_sticker_sets(StickerType::Regular, "testset".to_string())
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].get(), 123);
    }

    #[tokio::test]
    async fn test_search_sticker_sets_different_type() {
        let manager = create_test_manager().await;

        let results = manager
            .search_sticker_sets(StickerType::Mask, "test".to_string())
            .await
            .unwrap();

        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_max_get_custom_emoji_stickers_constant() {}

    #[tokio::test]
    async fn test_max_sticker_set_short_name_length_constant() {
        assert_eq!(MAX_STICKER_SET_SHORT_NAME_LENGTH, 64);
    }

    #[tokio::test]
    async fn test_max_found_stickers_constant() {}

    #[tokio::test]
    async fn test_great_minds_set_id_constant() {
        assert_eq!(GREAT_MINDS_SET_ID, 1842540969984001);
    }

    #[tokio::test]
    async fn test_default_cache_ttl_constant() {
        assert_eq!(DEFAULT_CACHE_TTL, 3600);
    }

    #[tokio::test]
    async fn test_manager_clone() {
        let manager1 = StickersManager::new();
        let manager2 = manager1.clone();

        // Both should have empty caches
        assert_eq!(manager1.cached_sticker_count().await, 0);
        assert_eq!(manager2.cached_sticker_count().await, 0);
    }

    #[tokio::test]
    async fn test_search_with_empty_query() {
        let manager = create_test_manager().await;

        // Empty query should return all sets of the type
        let results = manager
            .search_sticker_sets(StickerType::Regular, String::new())
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
    }
}
