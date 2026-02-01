//! # Common Dialog Manager
//!
//! Manages common dialogs between users.
//!
//! This module provides functionality for:
//! - Caching common dialogs between two users
//! - Fetching common dialogs from server
//! - Pagination support
//!
//! # Overview
//!
//! The CommonDialogManager is responsible for tracking and managing dialogs
//! that are shared between two users (e.g., groups where both are members).
//!
//! # Main Components
//!
//! - [`CommonDialogManager`]: Main manager for common dialog operations
//! - [`CommonDialogs`]: Cached common dialogs for a user
//!
//! # Examples
//!
//! ```rust
//! use rustgram_common_dialog_manager::CommonDialogManager;
//! use rustgram_types::{UserId, DialogId};
//!
//! // Create a manager
//! let mut manager = CommonDialogManager::new();
//!
//! // Add common dialogs for a user
//! let user_id = UserId::new(123).unwrap();
//! let dialogs = vec![DialogId::from_chat(rustgram_types::ChatId::new(1).unwrap())];
//!
//! manager.on_get_common_dialogs(user_id, dialogs, 100);
//! ```
//!
//! # Thread Safety
//!
//! The manager uses `Arc<RwLock<T>>` for shared state, allowing safe concurrent access.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use rustgram_types::{DialogId, UserId};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

pub mod error;

pub use error::{CommonDialogError, Result};

/// Maximum number of dialogs to fetch per request (server-side limit).
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `CommonDialogManager::MAX_GET_DIALOGS`.
pub const MAX_GET_DIALOGS: i32 = 100;

/// Cached common dialogs for a user.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `CommonDialogManager::CommonDialogs` struct.
#[derive(Debug, Clone)]
pub struct CommonDialogs {
    /// List of dialog IDs.
    pub dialog_ids: Vec<DialogId>,
    /// Timestamp when this data was received.
    pub receive_time: f64,
    /// Total count of common dialogs.
    pub total_count: i32,
    /// Whether this cache is outdated.
    pub is_outdated: bool,
}

impl CommonDialogs {
    /// Creates new common dialogs cache.
    ///
    /// # Arguments
    ///
    /// * `dialog_ids` - List of dialog IDs
    /// * `total_count` - Total count
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogs;
    /// use rustgram_types::{DialogId, ChatId};
    ///
    /// let dialogs = vec![DialogId::from_chat(ChatId::new(1).unwrap())];
    /// let cache = CommonDialogs::new(dialogs, 100);
    ///
    /// assert_eq!(cache.total_count, 100);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(dialog_ids: Vec<DialogId>, total_count: i32) -> Self {
        Self {
            dialog_ids,
            receive_time: 0.0,
            total_count,
            is_outdated: false,
        }
    }

    /// Creates new common dialogs cache with receive time.
    ///
    /// # Arguments
    ///
    /// * `dialog_ids` - List of dialog IDs
    /// * `receive_time` - Timestamp when received
    /// * `total_count` - Total count
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogs;
    /// use rustgram_types::{DialogId, ChatId};
    ///
    /// let dialogs = vec![DialogId::from_chat(ChatId::new(1).unwrap())];
    /// let cache = CommonDialogs::with_receive_time(dialogs, 1234567890.0, 100);
    ///
    /// assert_eq!(cache.receive_time, 1234567890.0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn with_receive_time(
        dialog_ids: Vec<DialogId>,
        receive_time: f64,
        total_count: i32,
    ) -> Self {
        Self {
            dialog_ids,
            receive_time,
            total_count,
            is_outdated: false,
        }
    }

    /// Marks the cache as outdated.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogs;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let dialogs = vec![DialogId::from_chat(ChatId::new(1).unwrap())];
    /// let mut cache = CommonDialogs::new(dialogs, 100);
    ///
    /// cache.mark_outdated();
    /// assert!(cache.is_outdated);
    /// ```
    #[inline]
    pub fn mark_outdated(&mut self) {
        self.is_outdated = true;
    }

    /// Returns the number of cached dialogs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogs;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let dialogs = vec![
    ///     DialogId::from_chat(ChatId::new(1).unwrap()),
    ///     DialogId::from_chat(ChatId::new(2).unwrap()),
    /// ];
    /// let cache = CommonDialogs::new(dialogs, 100);
    ///
    /// assert_eq!(cache.len(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.dialog_ids.len()
    }

    /// Checks if the cache is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogs;
    /// use rustgram_types::DialogId;
    ///
    /// let cache = CommonDialogs::new(vec![], 0);
    /// assert!(cache.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dialog_ids.is_empty()
    }
}

impl Default for CommonDialogs {
    fn default() -> Self {
        Self {
            dialog_ids: Vec::new(),
            receive_time: 0.0,
            total_count: 0,
            is_outdated: false,
        }
    }
}

/// Common dialog manager state.
///
/// Internal state managed by the CommonDialogManager.
/// Uses Arc<RwLock<T>> for thread-safe shared access.
#[derive(Debug, Clone)]
pub struct CommonDialogManagerState {
    /// Map of user IDs to their cached common dialogs.
    pub found_common_dialogs: HashMap<UserId, CommonDialogs>,
}

impl Default for CommonDialogManagerState {
    fn default() -> Self {
        Self {
            found_common_dialogs: HashMap::new(),
        }
    }
}

/// Common dialog manager for Telegram.
///
/// Manages caching and retrieval of common dialogs between users.
///
/// # Thread Safety
///
/// This manager uses `Arc<RwLock<T>>` internally, making it safe to share across threads.
///
/// # Examples
///
/// ```
/// use rustgram_common_dialog_manager::CommonDialogManager;
/// use rustgram_types::{UserId, DialogId, ChatId};
///
/// let mut manager = CommonDialogManager::new();
///
/// let user_id = UserId::new(123).unwrap();
/// let dialogs = vec![DialogId::from_chat(ChatId::new(1).unwrap())];
///
/// manager.on_get_common_dialogs(user_id, dialogs.clone(), 100);
///
/// let result = manager.get_common_dialogs(user_id, DialogId::from_chat(ChatId::new(1).unwrap()), 100, false);
/// assert!(result.is_ok());
/// ```
#[derive(Debug, Clone)]
pub struct CommonDialogManager {
    /// Internal state protected by RwLock for thread-safe access.
    state: Arc<std::sync::RwLock<CommonDialogManagerState>>,
}

impl Default for CommonDialogManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CommonDialogManager {
    /// Creates a new common dialog manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogManager;
    ///
    /// let manager = CommonDialogManager::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(std::sync::RwLock::new(CommonDialogManagerState::default())),
        }
    }

    /// Updates common dialogs for a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    /// * `dialog_ids` - List of common dialog IDs
    /// * `total_count` - Total count of common dialogs
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `CommonDialogManager::on_get_common_dialogs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogManager;
    /// use rustgram_types::{UserId, DialogId, ChatId};
    ///
    /// let mut manager = CommonDialogManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialogs = vec![DialogId::from_chat(ChatId::new(1).unwrap())];
    ///
    /// manager.on_get_common_dialogs(user_id, dialogs.clone(), 100);
    /// ```
    pub fn on_get_common_dialogs(
        &mut self,
        user_id: UserId,
        dialog_ids: Vec<DialogId>,
        total_count: i32,
    ) -> Result<()> {
        let mut state = self
            .state
            .write()
            .map_err(|_| CommonDialogError::LockError)?;
        let common_dialogs = CommonDialogs::new(dialog_ids, total_count);
        state.found_common_dialogs.insert(user_id, common_dialogs);
        Ok(())
    }

    /// Drops the cache for a user's common dialogs.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `CommonDialogManager::drop_common_dialogs_cache`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogManager;
    /// use rustgram_types::{UserId, DialogId, ChatId};
    ///
    /// let mut manager = CommonDialogManager::new();
    /// let user_id = UserId::new(123).unwrap();
    ///
    /// manager.on_get_common_dialogs(user_id, vec![], 0);
    /// manager.drop_common_dialogs_cache(user_id);
    /// ```
    pub fn drop_common_dialogs_cache(&mut self, user_id: UserId) -> Result<()> {
        let mut state = self
            .state
            .write()
            .map_err(|_| CommonDialogError::LockError)?;
        state.found_common_dialogs.remove(&user_id);
        Ok(())
    }

    /// Gets common dialogs for a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    /// * `offset_dialog_id` - Dialog ID to offset from
    /// * `limit` - Maximum number of dialogs to return
    /// * `force` - Whether to force a refresh
    ///
    /// # Returns
    ///
    /// `Ok((total_count, dialog_ids))` if successful
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `CommonDialogManager::get_common_dialogs`.
    /// This is a stub implementation that only returns cached data.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogManager;
    /// use rustgram_types::{UserId, DialogId, ChatId};
    ///
    /// let mut manager = CommonDialogManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialogs = vec![
    ///     DialogId::from_chat(ChatId::new(1).unwrap()),
    ///     DialogId::from_chat(ChatId::new(2).unwrap()),
    /// ];
    ///
    /// manager.on_get_common_dialogs(user_id, dialogs.clone(), 100);
    ///
    /// let (total, ids) = manager.get_common_dialogs(user_id, DialogId::from_chat(ChatId::new(1).unwrap()), 10, false).unwrap();
    /// assert_eq!(total, 100);
    /// ```
    pub fn get_common_dialogs(
        &self,
        user_id: UserId,
        _offset_dialog_id: DialogId,
        limit: i32,
        _force: bool,
    ) -> Result<(i32, Vec<DialogId>)> {
        if limit > MAX_GET_DIALOGS {
            return Err(CommonDialogError::LimitExceeded {
                requested: limit,
                max: MAX_GET_DIALOGS,
            });
        }

        if limit <= 0 {
            return Err(CommonDialogError::InvalidLimit(limit));
        }

        let state = self
            .state
            .read()
            .map_err(|_| CommonDialogError::LockError)?;

        let common_dialogs = state
            .found_common_dialogs
            .get(&user_id)
            .ok_or_else(|| CommonDialogError::UserNotFound(user_id.get()))?;

        // TODO: Implement offset pagination when needed
        // For now, return all cached dialogs up to limit
        let end_idx = std::cmp::min(limit as usize, common_dialogs.dialog_ids.len());
        let dialog_ids = common_dialogs.dialog_ids[0..end_idx].to_vec();

        Ok((common_dialogs.total_count, dialog_ids))
    }

    /// Checks if common dialogs are cached for a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    ///
    /// # Returns
    ///
    /// `true` if cache exists
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogManager;
    /// use rustgram_types::UserId;
    ///
    /// let mut manager = CommonDialogManager::new();
    /// let user_id = UserId::new(123).unwrap();
    ///
    /// assert!(!manager.has_cached_dialogs(user_id));
    ///
    /// manager.on_get_common_dialogs(user_id, vec![], 0);
    /// assert!(manager.has_cached_dialogs(user_id));
    /// ```
    #[inline]
    #[must_use]
    pub fn has_cached_dialogs(&self, user_id: UserId) -> bool {
        self.state
            .read()
            .map(|s| s.found_common_dialogs.contains_key(&user_id))
            .unwrap_or(false)
    }

    /// Gets the number of users with cached common dialogs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogManager;
    /// use rustgram_types::UserId;
    ///
    /// let mut manager = CommonDialogManager::new();
    /// assert_eq!(manager.cache_count(), 0);
    ///
    /// manager.on_get_common_dialogs(UserId::new(1).unwrap(), vec![], 0);
    /// manager.on_get_common_dialogs(UserId::new(2).unwrap(), vec![], 0);
    ///
    /// assert_eq!(manager.cache_count(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn cache_count(&self) -> usize {
        self.state
            .read()
            .map(|s| s.found_common_dialogs.len())
            .unwrap_or(0)
    }

    /// Clears all cached common dialogs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_common_dialog_manager::CommonDialogManager;
    /// use rustgram_types::UserId;
    ///
    /// let mut manager = CommonDialogManager::new();
    /// manager.on_get_common_dialogs(UserId::new(1).unwrap(), vec![], 0);
    ///
    /// manager.clear().unwrap();
    /// assert_eq!(manager.cache_count(), 0);
    /// ```
    pub fn clear(&mut self) -> Result<()> {
        let mut state = self
            .state
            .write()
            .map_err(|_| CommonDialogError::LockError)?;
        state.found_common_dialogs.clear();
        Ok(())
    }
}

impl fmt::Display for CommonDialogManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cache_count = self.cache_count();
        write!(f, "CommonDialogManager(cached_users: {})", cache_count)
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-common-dialog-manager";

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::ChatId;

    // ========== CommonDialogs Tests ==========

    #[test]
    fn test_common_dialogs_new() {
        let dialogs = vec![DialogId::from_chat(ChatId::new(1).unwrap())];
        let cache = CommonDialogs::new(dialogs.clone(), 100);

        assert_eq!(cache.dialog_ids, dialogs);
        assert_eq!(cache.total_count, 100);
        assert_eq!(cache.receive_time, 0.0);
        assert!(!cache.is_outdated);
    }

    #[test]
    fn test_common_dialogs_with_receive_time() {
        let dialogs = vec![DialogId::from_chat(ChatId::new(1).unwrap())];
        let cache = CommonDialogs::with_receive_time(dialogs, 1234567890.0, 100);

        assert_eq!(cache.receive_time, 1234567890.0);
    }

    #[test]
    fn test_common_dialogs_mark_outdated() {
        let dialogs = vec![DialogId::from_chat(ChatId::new(1).unwrap())];
        let mut cache = CommonDialogs::new(dialogs, 100);

        assert!(!cache.is_outdated);
        cache.mark_outdated();
        assert!(cache.is_outdated);
    }

    #[test]
    fn test_common_dialogs_len() {
        let dialogs = vec![
            DialogId::from_chat(ChatId::new(1).unwrap()),
            DialogId::from_chat(ChatId::new(2).unwrap()),
            DialogId::from_chat(ChatId::new(3).unwrap()),
        ];
        let cache = CommonDialogs::new(dialogs, 100);

        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_common_dialogs_is_empty() {
        let cache = CommonDialogs::new(vec![], 0);
        assert!(cache.is_empty());

        let dialogs = vec![DialogId::from_chat(ChatId::new(1).unwrap())];
        let cache = CommonDialogs::new(dialogs, 100);
        assert!(!cache.is_empty());
    }

    #[test]
    fn test_common_dialogs_default() {
        let cache = CommonDialogs::default();
        assert!(cache.is_empty());
        assert_eq!(cache.total_count, 0);
        assert!(!cache.is_outdated);
    }

    // ========== Constructor Tests ==========

    #[test]
    fn test_manager_new() {
        let manager = CommonDialogManager::new();
        assert_eq!(manager.cache_count(), 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = CommonDialogManager::default();
        assert_eq!(manager.cache_count(), 0);
    }

    // ========== on_get_common_dialogs Tests ==========

    #[test]
    fn test_on_get_common_dialogs() {
        let mut manager = CommonDialogManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialogs = vec![DialogId::from_chat(ChatId::new(1).unwrap())];

        let result = manager.on_get_common_dialogs(user_id, dialogs.clone(), 100);
        assert!(result.is_ok());
        assert!(manager.has_cached_dialogs(user_id));
    }

    #[test]
    fn test_on_get_common_dialogs_multiple_users() {
        let mut manager = CommonDialogManager::new();

        for i in 1..=5 {
            let user_id = UserId::new(i).unwrap();
            let dialogs = vec![DialogId::from_chat(ChatId::new(i).unwrap())];
            manager
                .on_get_common_dialogs(user_id, dialogs, (i * 10) as i32)
                .unwrap();
        }

        assert_eq!(manager.cache_count(), 5);
    }

    // ========== drop_common_dialogs_cache Tests ==========

    #[test]
    fn test_drop_common_dialogs_cache() {
        let mut manager = CommonDialogManager::new();
        let user_id = UserId::new(123).unwrap();

        manager.on_get_common_dialogs(user_id, vec![], 0).unwrap();
        assert!(manager.has_cached_dialogs(user_id));

        manager.drop_common_dialogs_cache(user_id).unwrap();
        assert!(!manager.has_cached_dialogs(user_id));
    }

    #[test]
    fn test_drop_common_dialogs_cache_nonexistent() {
        let mut manager = CommonDialogManager::new();
        let user_id = UserId::new(123).unwrap();

        // Should not fail even if user doesn't exist
        let result = manager.drop_common_dialogs_cache(user_id);
        assert!(result.is_ok());
    }

    // ========== get_common_dialogs Tests ==========

    #[test]
    fn test_get_common_dialogs() {
        let mut manager = CommonDialogManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialogs = vec![
            DialogId::from_chat(ChatId::new(1).unwrap()),
            DialogId::from_chat(ChatId::new(2).unwrap()),
        ];

        manager
            .on_get_common_dialogs(user_id, dialogs.clone(), 100)
            .unwrap();

        let (total, ids) = manager
            .get_common_dialogs(
                user_id,
                DialogId::from_chat(ChatId::new(1).unwrap()),
                10,
                false,
            )
            .unwrap();

        assert_eq!(total, 100);
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_get_common_dialogs_with_limit() {
        let mut manager = CommonDialogManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialogs = vec![
            DialogId::from_chat(ChatId::new(1).unwrap()),
            DialogId::from_chat(ChatId::new(2).unwrap()),
            DialogId::from_chat(ChatId::new(3).unwrap()),
        ];

        manager
            .on_get_common_dialogs(user_id, dialogs, 100)
            .unwrap();

        let (total, ids) = manager
            .get_common_dialogs(
                user_id,
                DialogId::from_chat(ChatId::new(1).unwrap()),
                2,
                false,
            )
            .unwrap();

        assert_eq!(total, 100);
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_get_common_dialogs_limit_exceeded() {
        let mut manager = CommonDialogManager::new();
        let user_id = UserId::new(123).unwrap();

        manager.on_get_common_dialogs(user_id, vec![], 0).unwrap();

        let result = manager.get_common_dialogs(
            user_id,
            DialogId::from_chat(ChatId::new(1).unwrap()),
            MAX_GET_DIALOGS + 1,
            false,
        );

        assert!(result.is_err());
        match result {
            Err(CommonDialogError::LimitExceeded { requested, max }) => {
                assert_eq!(requested, MAX_GET_DIALOGS + 1);
                assert_eq!(max, MAX_GET_DIALOGS);
            }
            _ => panic!("Expected LimitExceeded error"),
        }
    }

    #[test]
    fn test_get_common_dialogs_invalid_limit() {
        let mut manager = CommonDialogManager::new();
        let user_id = UserId::new(123).unwrap();

        manager.on_get_common_dialogs(user_id, vec![], 0).unwrap();

        let result = manager.get_common_dialogs(
            user_id,
            DialogId::from_chat(ChatId::new(1).unwrap()),
            0,
            false,
        );

        assert!(result.is_err());
        match result {
            Err(CommonDialogError::InvalidLimit(limit)) => {
                assert_eq!(limit, 0);
            }
            _ => panic!("Expected InvalidLimit error"),
        }
    }

    #[test]
    fn test_get_common_dialogs_user_not_found() {
        let manager = CommonDialogManager::new();
        let user_id = UserId::new(123).unwrap();

        let result = manager.get_common_dialogs(
            user_id,
            DialogId::from_chat(ChatId::new(1).unwrap()),
            10,
            false,
        );

        assert!(result.is_err());
        match result {
            Err(CommonDialogError::UserNotFound(id)) => {
                assert_eq!(id, 123);
            }
            _ => panic!("Expected UserNotFound error"),
        }
    }

    // ========== has_cached_dialogs Tests ==========

    #[test]
    fn test_has_cached_dialogs() {
        let mut manager = CommonDialogManager::new();
        let user_id = UserId::new(123).unwrap();

        assert!(!manager.has_cached_dialogs(user_id));

        manager.on_get_common_dialogs(user_id, vec![], 0).unwrap();
        assert!(manager.has_cached_dialogs(user_id));
    }

    // ========== cache_count Tests ==========

    #[test]
    fn test_cache_count() {
        let mut manager = CommonDialogManager::new();
        assert_eq!(manager.cache_count(), 0);

        for i in 1..=5 {
            let user_id = UserId::new(i).unwrap();
            manager.on_get_common_dialogs(user_id, vec![], 0).unwrap();
        }

        assert_eq!(manager.cache_count(), 5);
    }

    // ========== clear Tests ==========

    #[test]
    fn test_clear() {
        let mut manager = CommonDialogManager::new();

        manager
            .on_get_common_dialogs(UserId::new(1).unwrap(), vec![], 0)
            .unwrap();
        manager
            .on_get_common_dialogs(UserId::new(2).unwrap(), vec![], 0)
            .unwrap();

        assert_eq!(manager.cache_count(), 2);

        manager.clear().unwrap();
        assert_eq!(manager.cache_count(), 0);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let manager = CommonDialogManager::new();
        let display = format!("{}", manager);
        assert!(display.contains("CommonDialogManager"));
        assert!(display.contains("cached_users: 0"));
    }

    // ========== Constants Tests ==========

    #[test]
    fn test_max_get_dialogs() {
        assert_eq!(MAX_GET_DIALOGS, 100);
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_and_crate_name() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-common-dialog-manager");
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_full_workflow() {
        let mut manager = CommonDialogManager::new();
        let user_id = UserId::new(123).unwrap();

        // Initially no cache
        assert!(!manager.has_cached_dialogs(user_id));

        // Add dialogs
        let dialogs = vec![
            DialogId::from_chat(ChatId::new(1).unwrap()),
            DialogId::from_chat(ChatId::new(2).unwrap()),
            DialogId::from_chat(ChatId::new(3).unwrap()),
        ];
        manager
            .on_get_common_dialogs(user_id, dialogs.clone(), 100)
            .unwrap();

        // Now cached
        assert!(manager.has_cached_dialogs(user_id));

        // Get dialogs with limit
        let (total, ids) = manager
            .get_common_dialogs(
                user_id,
                DialogId::from_chat(ChatId::new(1).unwrap()),
                2,
                false,
            )
            .unwrap();
        assert_eq!(total, 100);
        assert_eq!(ids.len(), 2);

        // Drop cache
        manager.drop_common_dialogs_cache(user_id).unwrap();
        assert!(!manager.has_cached_dialogs(user_id));
    }

    #[test]
    fn test_multiple_users_workflow() {
        let mut manager = CommonDialogManager::new();

        // Add dialogs for multiple users
        for i in 1..=3 {
            let user_id = UserId::new(i).unwrap();
            let dialogs = vec![DialogId::from_chat(ChatId::new(i).unwrap())];
            manager
                .on_get_common_dialogs(user_id, dialogs, (i * 10) as i32)
                .unwrap();
        }

        assert_eq!(manager.cache_count(), 3);

        // Verify each user has correct total
        for i in 1..=3 {
            let user_id = UserId::new(i).unwrap();
            let (total, _) = manager
                .get_common_dialogs(
                    user_id,
                    DialogId::from_chat(ChatId::new(1).unwrap()),
                    10,
                    false,
                )
                .unwrap();
            assert_eq!(total, (i * 10) as i32);
        }
    }
}
