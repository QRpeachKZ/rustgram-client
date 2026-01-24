// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification manager for Telegram MTProto client.
//!
//! This module implements TDLib's NotificationManager.
//!
//! # Overview
//!
//! The NotificationManager manages notification delivery, groups, and settings,
//! including notification grouping, temporary notifications, and call notifications.
//!
//! # Example
//!
//! ```rust
//! use rustgram_notification_manager::NotificationManager;
//!
//! let manager = NotificationManager::new();
//! let max_id = manager.get_max_notification_id();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_call_id::CallId;
use rustgram_dialog_id::DialogId;
use rustgram_notification::Notification;
use rustgram_notification_group_id::NotificationGroupId;
use rustgram_notification_group_type::NotificationGroupType;
use rustgram_notification_id::NotificationId;
use rustgram_notification_object_id::NotificationObjectId;
use rustgram_notification_type::NotificationType;
use rustgram_types::MessageId;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicI32, AtomicI64, Ordering};
use std::sync::Arc;
use std::sync::RwLock;

/// Notification group key.
///
/// Combines dialog ID and group type to uniquely identify a notification group.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotificationGroupKey {
    /// Dialog ID.
    dialog_id: DialogId,
    /// Group type.
    group_type: NotificationGroupType,
}

impl NotificationGroupKey {
    /// Creates a new notification group key.
    #[must_use]
    pub const fn new(dialog_id: DialogId, group_type: NotificationGroupType) -> Self {
        Self {
            dialog_id,
            group_type,
        }
    }

    /// Gets the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Gets the group type.
    #[must_use]
    pub const fn group_type(&self) -> NotificationGroupType {
        self.group_type
    }
}

/// Notification group.
///
/// Contains notifications grouped by dialog and type.
#[derive(Debug, Clone)]
pub struct NotificationGroup {
    /// Total count of notifications in the group.
    total_count: i32,
    /// Group type.
    group_type: NotificationGroupType,
    /// Whether the group is loaded from database.
    is_loaded: bool,
    /// Notifications in the group.
    notifications: Vec<Notification>,
}

impl Default for NotificationGroup {
    fn default() -> Self {
        Self {
            total_count: 0,
            group_type: NotificationGroupType::Messages,
            is_loaded: false,
            notifications: Vec::new(),
        }
    }
}

impl NotificationGroup {
    /// Creates a new notification group.
    #[must_use]
    pub fn new(group_type: NotificationGroupType) -> Self {
        Self {
            total_count: 0,
            group_type,
            is_loaded: false,
            notifications: Vec::new(),
        }
    }

    /// Gets the total count.
    #[must_use]
    pub const fn total_count(&self) -> i32 {
        self.total_count
    }

    /// Gets the group type.
    #[must_use]
    pub const fn group_type(&self) -> NotificationGroupType {
        self.group_type
    }

    /// Checks if loaded from database.
    #[must_use]
    pub const fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    /// Gets the notifications.
    #[must_use]
    pub fn notifications(&self) -> &[Notification] {
        &self.notifications
    }
}

/// Notification manager.
///
/// Based on TDLib's `NotificationManager` class.
///
/// Manages notification delivery, grouping, and settings including:
/// - Notification ID generation
/// - Notification group management
/// - Call notifications
/// - Push notifications
///
/// # Example
///
/// ```rust
/// use rustgram_notification_manager::NotificationManager;
///
/// let manager = NotificationManager::new();
/// let max_id = manager.get_max_notification_id();
/// assert_eq!(max_id.get(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct NotificationManager {
    /// Shared manager state.
    state: Arc<ManagerState>,
}

/// Shared manager state.
#[derive(Debug)]
struct ManagerState {
    /// Current notification ID.
    current_notification_id: AtomicI32,
    /// Current notification group ID.
    current_notification_group_id: AtomicI32,
    /// Maximum notification group size.
    max_notification_group_size: RwLock<usize>,
    /// Maximum notification group count.
    max_notification_group_count: RwLock<usize>,
    /// Notification groups.
    groups: RwLock<HashMap<NotificationGroupId, NotificationGroup>>,
    /// Group keys (group_id -> key).
    group_keys: RwLock<HashMap<NotificationGroupId, NotificationGroupKey>>,
    /// Call notification groups.
    call_notification_groups: RwLock<HashMap<DialogId, NotificationGroupId>>,
    /// Available call notification group IDs.
    available_call_group_ids: RwLock<HashSet<NotificationGroupId>>,
    /// Active call notifications.
    active_call_notifications: RwLock<HashMap<DialogId, Vec<(CallId, NotificationId)>>>,
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationManager {
    /// Minimum notification group count.
    pub const MIN_NOTIFICATION_GROUP_COUNT_MAX: i32 = 0;

    /// Maximum notification group count.
    pub const MAX_NOTIFICATION_GROUP_COUNT_MAX: i32 = 25;

    /// Minimum notification group size.
    pub const MIN_NOTIFICATION_GROUP_SIZE_MAX: i32 = 1;

    /// Maximum notification group size.
    pub const MAX_NOTIFICATION_GROUP_SIZE_MAX: i32 = 25;

    /// Default online cloud timeout in milliseconds.
    pub const DEFAULT_ONLINE_CLOUD_TIMEOUT_MS: i32 = 300_000;

    /// Default notification cloud delay in milliseconds.
    pub const DEFAULT_ONLINE_CLOUD_DELAY_MS: i32 = 30_000;

    /// Default notification delay in milliseconds.
    pub const DEFAULT_NOTIFICATION_DELAY_MS: i32 = 1500;

    /// Minimum notification delay in milliseconds.
    pub const MIN_NOTIFICATION_DELAY_MS: i32 = 1;

    /// Extra group size.
    pub const EXTRA_GROUP_SIZE: usize = 10;

    /// Maximum call notification groups.
    pub const MAX_CALL_NOTIFICATION_GROUPS: usize = 10;

    /// Maximum call notifications.
    pub const MAX_CALL_NOTIFICATIONS: usize = 10;

    /// Creates a new NotificationManager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(ManagerState {
                current_notification_id: AtomicI32::new(0),
                current_notification_group_id: AtomicI32::new(0),
                max_notification_group_size: RwLock::new(Self::DEFAULT_GROUP_SIZE_MAX()),
                max_notification_group_count: RwLock::new(Self::DEFAULT_GROUP_COUNT_MAX()),
                groups: RwLock::new(HashMap::new()),
                group_keys: RwLock::new(HashMap::new()),
                call_notification_groups: RwLock::new(HashMap::new()),
                available_call_group_ids: RwLock::new(HashSet::new()),
                active_call_notifications: RwLock::new(HashMap::new()),
            }),
        }
    }

    /// Gets the default group size max.
    const fn DEFAULT_GROUP_SIZE_MAX() -> usize {
        10
    }

    /// Gets the default group count max.
    const fn DEFAULT_GROUP_COUNT_MAX() -> usize {
        0
    }

    /// Initializes the manager.
    ///
    /// This method should be called after creating the manager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// manager.init();
    /// ```
    pub fn init(&self) {
        // TODO: Load from database when storage module is available
    }

    /// Gets the maximum notification group size.
    ///
    /// # Returns
    ///
    /// The maximum number of notifications per group.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// assert_eq!(manager.get_max_notification_group_size(), 10);
    /// ```
    #[must_use]
    pub fn get_max_notification_group_size(&self) -> usize {
        *self.state.max_notification_group_size.read().unwrap()
    }

    /// Gets the maximum notification ID.
    ///
    /// # Returns
    ///
    /// The current maximum notification ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// assert_eq!(manager.get_max_notification_id().get(), 0);
    /// ```
    #[must_use]
    pub fn get_max_notification_id(&self) -> NotificationId {
        NotificationId::new(self.state.current_notification_id.load(Ordering::Acquire))
    }

    /// Gets the next notification ID.
    ///
    /// # Returns
    ///
    /// A new unique notification ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// let id1 = manager.get_next_notification_id();
    /// let id2 = manager.get_next_notification_id();
    /// assert_eq!(id2.get(), id1.get() + 1);
    /// ```
    #[must_use]
    pub fn get_next_notification_id(&self) -> NotificationId {
        let id = self
            .state
            .current_notification_id
            .fetch_add(1, Ordering::AcqRel)
            + 1;
        NotificationId::new(id)
    }

    /// Gets the next notification group ID.
    ///
    /// # Returns
    ///
    /// A new unique notification group ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// let id1 = manager.get_next_notification_group_id();
    /// let id2 = manager.get_next_notification_group_id();
    /// assert_eq!(id2.get(), id1.get() + 1);
    /// ```
    #[must_use]
    pub fn get_next_notification_group_id(&self) -> NotificationGroupId {
        let id = self
            .state
            .current_notification_group_id
            .fetch_add(1, Ordering::AcqRel)
            + 1;
        NotificationGroupId::new(id)
    }

    /// Tries to reuse a notification group ID.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The group ID to reuse
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// let manager = NotificationManager::new();
    /// manager.try_reuse_notification_group_id(NotificationGroupId::new(123));
    /// ```
    pub fn try_reuse_notification_group_id(&self, group_id: NotificationGroupId) {
        self.state
            .available_call_group_ids
            .write()
            .unwrap()
            .insert(group_id);
    }

    /// Checks if a group exists and is loaded.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The group ID to check
    ///
    /// # Returns
    ///
    /// `true` if the group exists and is loaded.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// let manager = NotificationManager::new();
    /// assert!(!manager.have_group_force(NotificationGroupId::new(123)));
    /// ```
    #[must_use]
    pub fn have_group_force(&self, group_id: NotificationGroupId) -> bool {
        self.state
            .groups
            .read()
            .unwrap()
            .get(&group_id)
            .map_or(false, |g| g.is_loaded())
    }

    /// Adds a call notification.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `call_id` - The call ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_call_id::CallId;
    ///
    /// let manager = NotificationManager::new();
    /// manager.add_call_notification(DialogId::new(123), CallId::new(456));
    /// ```
    pub fn add_call_notification(&self, dialog_id: DialogId, call_id: CallId) {
        let notification_id = self.get_next_notification_id();
        self.state
            .active_call_notifications
            .write()
            .unwrap()
            .entry(dialog_id)
            .or_insert_with(Vec::new)
            .push((call_id, notification_id));
    }

    /// Removes a call notification.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `call_id` - The call ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_call_id::CallId;
    ///
    /// let manager = NotificationManager::new();
    /// manager.add_call_notification(DialogId::new(123), CallId::new(456));
    /// manager.remove_call_notification(DialogId::new(123), CallId::new(456));
    /// ```
    pub fn remove_call_notification(&self, dialog_id: DialogId, call_id: CallId) {
        let mut notifications = self.state.active_call_notifications.write().unwrap();
        if let Some(notifs) = notifications.get_mut(&dialog_id) {
            notifs.retain(|(id, _)| *id != call_id);
            if notifs.is_empty() {
                notifications.remove(&dialog_id);
            }
        }
    }

    /// Sets the maximum notification group count.
    ///
    /// # Arguments
    ///
    /// * `count` - The maximum number of notification groups
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// manager.set_max_notification_group_count(20);
    /// ```
    pub fn set_max_notification_group_count(&self, count: usize) {
        *self.state.max_notification_group_count.write().unwrap() = count;
    }

    /// Sets the maximum notification group size.
    ///
    /// # Arguments
    ///
    /// * `size` - The maximum number of notifications per group
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// manager.set_max_notification_group_size(15);
    /// ```
    pub fn set_max_notification_group_size(&self, size: usize) {
        *self.state.max_notification_group_size.write().unwrap() = size;
    }

    /// Gets the maximum notification group count.
    ///
    /// # Returns
    ///
    /// The maximum number of notification groups.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// assert_eq!(manager.get_max_notification_group_count(), 0);
    /// ```
    #[must_use]
    pub fn get_max_notification_group_count(&self) -> usize {
        *self.state.max_notification_group_count.read().unwrap()
    }

    /// Gets active call notifications for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    ///
    /// # Returns
    ///
    /// A vector of (call_id, notification_id) tuples.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let manager = NotificationManager::new();
    /// let calls = manager.get_active_call_notifications(DialogId::new(123));
    /// assert!(calls.is_empty());
    /// ```
    #[must_use]
    pub fn get_active_call_notifications(
        &self,
        dialog_id: DialogId,
    ) -> Vec<(CallId, NotificationId)> {
        self.state
            .active_call_notifications
            .read()
            .unwrap()
            .get(&dialog_id)
            .map_or_else(Vec::new, |v| v.clone())
    }

    /// Gets all notification group IDs.
    ///
    /// # Returns
    ///
    /// A vector of all notification group IDs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// let groups = manager.get_notification_group_ids();
    /// assert!(groups.is_empty());
    /// ```
    #[must_use]
    pub fn get_notification_group_ids(&self) -> Vec<NotificationGroupId> {
        self.state.groups.read().unwrap().keys().copied().collect()
    }

    /// Gets a notification group by ID.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The group ID
    ///
    /// # Returns
    ///
    /// `Some(NotificationGroup)` if found, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// let manager = NotificationManager::new();
    /// let group = manager.get_group(NotificationGroupId::new(123));
    /// assert!(group.is_none());
    /// ```
    #[must_use]
    pub fn get_group(&self, group_id: NotificationGroupId) -> Option<NotificationGroup> {
        self.state.groups.read().unwrap().get(&group_id).cloned()
    }

    /// Flushes all notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// manager.flush_all_notifications();
    /// ```
    pub fn flush_all_notifications(&self) {
        self.state.groups.write().unwrap().clear();
        self.state.group_keys.write().unwrap().clear();
        self.state.call_notification_groups.write().unwrap().clear();
        self.state
            .active_call_notifications
            .write()
            .unwrap()
            .clear();
    }

    /// Destroys all notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_manager::NotificationManager;
    ///
    /// let manager = NotificationManager::new();
    /// manager.destroy_all_notifications();
    /// ```
    pub fn destroy_all_notifications(&self) {
        self.flush_all_notifications();
        self.state
            .current_notification_id
            .store(0, Ordering::Release);
        self.state
            .current_notification_group_id
            .store(0, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let manager = NotificationManager::new();
        assert_eq!(manager.get_max_notification_id().get(), 0);
        assert_eq!(manager.get_max_notification_group_size(), 10);
        assert_eq!(manager.get_max_notification_group_count(), 0);
    }

    #[test]
    fn test_default() {
        let manager = NotificationManager::default();
        assert_eq!(manager.get_max_notification_id().get(), 0);
    }

    #[test]
    fn test_init() {
        let manager = NotificationManager::new();
        manager.init(); // Should not panic
    }

    #[test]
    fn test_get_max_notification_id() {
        let manager = NotificationManager::new();
        assert_eq!(manager.get_max_notification_id().get(), 0);
    }

    #[test]
    fn test_get_next_notification_id() {
        let manager = NotificationManager::new();
        let id1 = manager.get_next_notification_id();
        let id2 = manager.get_next_notification_id();
        let id3 = manager.get_next_notification_id();

        assert_eq!(id1.get(), 1);
        assert_eq!(id2.get(), 2);
        assert_eq!(id3.get(), 3);
    }

    #[test]
    fn test_get_next_notification_group_id() {
        let manager = NotificationManager::new();
        let id1 = manager.get_next_notification_group_id();
        let id2 = manager.get_next_notification_group_id();
        let id3 = manager.get_next_notification_group_id();

        assert_eq!(id1.get(), 1);
        assert_eq!(id2.get(), 2);
        assert_eq!(id3.get(), 3);
    }

    #[test]
    fn test_get_max_notification_group_size() {
        let manager = NotificationManager::new();
        assert_eq!(manager.get_max_notification_group_size(), 10);
    }

    #[test]
    fn test_set_max_notification_group_size() {
        let manager = NotificationManager::new();
        manager.set_max_notification_group_size(20);
        assert_eq!(manager.get_max_notification_group_size(), 20);
    }

    #[test]
    fn test_get_max_notification_group_count() {
        let manager = NotificationManager::new();
        assert_eq!(manager.get_max_notification_group_count(), 0);
    }

    #[test]
    fn test_set_max_notification_group_count() {
        let manager = NotificationManager::new();
        manager.set_max_notification_group_count(15);
        assert_eq!(manager.get_max_notification_group_count(), 15);
    }

    #[test]
    fn test_try_reuse_notification_group_id() {
        let manager = NotificationManager::new();
        let group_id = NotificationGroupId::new(123);
        manager.try_reuse_notification_group_id(group_id);

        assert!(manager
            .state
            .available_call_group_ids
            .read()
            .unwrap()
            .contains(&group_id));
    }

    #[test]
    fn test_have_group_force() {
        let manager = NotificationManager::new();
        let group_id = NotificationGroupId::new(123);
        assert!(!manager.have_group_force(group_id));
    }

    #[test]
    fn test_add_call_notification() {
        let manager = NotificationManager::new();
        let dialog_id = DialogId::new(123);
        let call_id = CallId::new(456);

        manager.add_call_notification(dialog_id, call_id);

        let calls = manager.get_active_call_notifications(dialog_id);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, call_id);
    }

    #[test]
    fn test_remove_call_notification() {
        let manager = NotificationManager::new();
        let dialog_id = DialogId::new(123);
        let call_id = CallId::new(456);

        manager.add_call_notification(dialog_id, call_id);
        assert_eq!(manager.get_active_call_notifications(dialog_id).len(), 1);

        manager.remove_call_notification(dialog_id, call_id);
        assert_eq!(manager.get_active_call_notifications(dialog_id).len(), 0);
    }

    #[test]
    fn test_add_multiple_call_notifications() {
        let manager = NotificationManager::new();
        let dialog_id = DialogId::new(123);

        manager.add_call_notification(dialog_id, CallId::new(1));
        manager.add_call_notification(dialog_id, CallId::new(2));
        manager.add_call_notification(dialog_id, CallId::new(3));

        let calls = manager.get_active_call_notifications(dialog_id);
        assert_eq!(calls.len(), 3);
    }

    #[test]
    fn test_remove_one_of_multiple_call_notifications() {
        let manager = NotificationManager::new();
        let dialog_id = DialogId::new(123);
        let call_id1 = CallId::new(1);
        let call_id2 = CallId::new(2);

        manager.add_call_notification(dialog_id, call_id1);
        manager.add_call_notification(dialog_id, call_id2);

        manager.remove_call_notification(dialog_id, call_id1);

        let calls = manager.get_active_call_notifications(dialog_id);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, call_id2);
    }

    #[test]
    fn test_get_active_call_notifications_empty() {
        let manager = NotificationManager::new();
        let dialog_id = DialogId::new(123);
        assert!(manager.get_active_call_notifications(dialog_id).is_empty());
    }

    #[test]
    fn test_get_notification_group_ids_empty() {
        let manager = NotificationManager::new();
        assert!(manager.get_notification_group_ids().is_empty());
    }

    #[test]
    fn test_get_group_nonexistent() {
        let manager = NotificationManager::new();
        let group_id = NotificationGroupId::new(123);
        assert!(manager.get_group(group_id).is_none());
    }

    #[test]
    fn test_flush_all_notifications() {
        let manager = NotificationManager::new();
        let dialog_id = DialogId::new(123);
        manager.add_call_notification(dialog_id, CallId::new(456));

        manager.flush_all_notifications();

        assert!(manager.get_active_call_notifications(dialog_id).is_empty());
    }

    #[test]
    fn test_destroy_all_notifications() {
        let manager = NotificationManager::new();
        let dialog_id = DialogId::new(123);
        manager.add_call_notification(dialog_id, CallId::new(456));

        manager.get_next_notification_id();
        manager.get_next_notification_group_id();

        manager.destroy_all_notifications();

        assert!(manager.get_active_call_notifications(dialog_id).is_empty());
        assert_eq!(manager.get_max_notification_id().get(), 0);
    }

    #[test]
    fn test_constants() {
        assert_eq!(NotificationManager::MIN_NOTIFICATION_GROUP_COUNT_MAX, 0);
        assert_eq!(NotificationManager::MAX_NOTIFICATION_GROUP_COUNT_MAX, 25);
        assert_eq!(NotificationManager::MIN_NOTIFICATION_GROUP_SIZE_MAX, 1);
        assert_eq!(NotificationManager::MAX_NOTIFICATION_GROUP_SIZE_MAX, 25);
        assert_eq!(
            NotificationManager::DEFAULT_ONLINE_CLOUD_TIMEOUT_MS,
            300_000
        );
        assert_eq!(NotificationManager::DEFAULT_ONLINE_CLOUD_DELAY_MS, 30_000);
        assert_eq!(NotificationManager::DEFAULT_NOTIFICATION_DELAY_MS, 1500);
        assert_eq!(NotificationManager::MIN_NOTIFICATION_DELAY_MS, 1);
        assert_eq!(NotificationManager::EXTRA_GROUP_SIZE, 10);
        assert_eq!(NotificationManager::MAX_CALL_NOTIFICATION_GROUPS, 10);
        assert_eq!(NotificationManager::MAX_CALL_NOTIFICATIONS, 10);
    }

    #[test]
    fn test_clone() {
        let manager1 = NotificationManager::new();
        manager1.set_max_notification_group_size(20);

        let manager2 = manager1.clone();
        assert_eq!(manager2.get_max_notification_group_size(), 20);
    }

    #[test]
    fn test_independent_managers() {
        let manager1 = NotificationManager::new();
        let manager2 = NotificationManager::new();

        manager1.set_max_notification_group_size(15);

        assert_eq!(manager1.get_max_notification_group_size(), 15);
        assert_eq!(manager2.get_max_notification_group_size(), 10);
    }

    #[test]
    fn test_debug_format() {
        let manager = NotificationManager::new();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("NotificationManager"));
    }

    #[test]
    fn test_notification_group_key() {
        let dialog_id = DialogId::new(123);
        let group_type = NotificationGroupType::Messages;
        let key = NotificationGroupKey::new(dialog_id, group_type);

        assert_eq!(key.dialog_id(), dialog_id);
        assert_eq!(key.group_type(), group_type);
    }

    #[test]
    fn test_notification_group_default() {
        let group = NotificationGroup::default();
        assert_eq!(group.total_count(), 0);
        assert_eq!(group.group_type(), NotificationGroupType::Messages);
        assert!(!group.is_loaded());
        assert!(group.notifications().is_empty());
    }

    #[test]
    fn test_notification_group_new() {
        let group = NotificationGroup::new(NotificationGroupType::Calls);
        assert_eq!(group.total_count(), 0);
        assert_eq!(group.group_type(), NotificationGroupType::Calls);
        assert!(!group.is_loaded());
        assert!(group.notifications().is_empty());
    }

    #[test]
    fn test_concurrent_notification_id_generation() {
        use std::thread;

        let manager = NotificationManager::new();
        let mut handles = vec![];

        for _ in 0..10 {
            let manager_clone = manager.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let _ = manager_clone.get_next_notification_id();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(manager.get_max_notification_id().get(), 1000);
    }
}
