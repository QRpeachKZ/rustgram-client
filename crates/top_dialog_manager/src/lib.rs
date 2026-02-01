// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Top Dialog Manager
//!
//! Top dialog tracking and management for Telegram.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_top_dialog_category::TopDialogCategory;
use rustgram_types::{DialogId, DialogType, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Maximum number of top dialogs to track
pub const MAX_TOP_DIALOGS_LIMIT: usize = 30;

/// Default rating e-decay value (in seconds)
pub const DEFAULT_RATING_E_DECAY: i32 = 241920;

/// Minimum story rating
pub const MIN_STORY_RATING: f64 = 10.0;

/// Top dialog entry with dialog ID and rating
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopDialog {
    /// Dialog ID
    pub dialog_id: DialogId,
    /// Dialog rating
    pub rating: f64,
}

impl TopDialog {
    /// Creates a new top dialog entry
    #[must_use]
    pub fn new(dialog_id: DialogId) -> Self {
        Self {
            dialog_id,
            rating: 0.0,
        }
    }

    /// Creates a new top dialog entry with rating
    #[must_use]
    pub fn with_rating(dialog_id: DialogId, rating: f64) -> Self {
        Self { dialog_id, rating }
    }
}

impl Eq for TopDialog {}

impl std::cmp::PartialOrd for TopDialog {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for TopDialog {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort by rating descending, then by dialog_id encoded value ascending
        match self.rating.partial_cmp(&other.rating) {
            Some(std::cmp::Ordering::Equal) => self
                .dialog_id
                .to_encoded()
                .cmp(&other.dialog_id.to_encoded()),
            Some(order) => order.reverse(),
            None => std::cmp::Ordering::Equal,
        }
    }
}

/// Top dialogs for a specific category
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopDialogs {
    /// Whether the data is dirty (needs saving)
    pub is_dirty: bool,
    /// Rating timestamp
    pub rating_timestamp: f64,
    /// List of top dialogs
    pub dialogs: Vec<TopDialog>,
}

impl Default for TopDialogs {
    fn default() -> Self {
        Self::new()
    }
}

impl TopDialogs {
    /// Creates a new empty top dialogs list
    #[must_use]
    pub const fn new() -> Self {
        Self {
            is_dirty: false,
            rating_timestamp: 0.0,
            dialogs: Vec::new(),
        }
    }

    /// Returns the number of dialogs
    #[must_use]
    pub fn len(&self) -> usize {
        self.dialogs.len()
    }

    /// Returns true if there are no dialogs
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dialogs.is_empty()
    }

    /// Adds a dialog to the list
    pub fn add(&mut self, dialog: TopDialog) {
        self.dialogs.push(dialog);
        self.is_dirty = true;
    }

    /// Removes a dialog by ID
    pub fn remove(&mut self, dialog_id: DialogId) -> bool {
        let initial_len = self.dialogs.len();
        self.dialogs.retain(|d| d.dialog_id != dialog_id);
        let removed = self.dialogs.len() != initial_len;
        if removed {
            self.is_dirty = true;
        }
        removed
    }

    /// Finds a dialog by ID
    #[must_use]
    pub fn find(&self, dialog_id: DialogId) -> Option<&TopDialog> {
        self.dialogs.iter().find(|d| d.dialog_id == dialog_id)
    }

    /// Finds a dialog by ID (mutable)
    #[must_use]
    pub fn find_mut(&mut self, dialog_id: DialogId) -> Option<&mut TopDialog> {
        self.dialogs.iter_mut().find(|d| d.dialog_id == dialog_id)
    }

    /// Sorts dialogs by rating
    pub fn sort(&mut self) {
        self.dialogs.sort_by(|a, b| {
            b.rating
                .partial_cmp(&a.rating)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}

/// Sync state for top dialogs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum SyncState {
    /// No sync pending
    None = 0,
    /// Sync pending
    Pending = 1,
    /// Sync completed
    Ok = 2,
}

impl Default for SyncState {
    fn default() -> Self {
        Self::None
    }
}

/// Top dialog manager
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopDialogManager {
    /// Whether top dialogs are enabled
    pub is_enabled: bool,
    /// Whether synchronized with server
    pub is_synchronized: bool,
    /// Rating e-decay value
    pub rating_e_decay: i32,
    /// Top dialogs by category
    pub by_category: HashMap<TopDialogCategory, TopDialogs>,
    /// Database sync state
    pub db_sync_state: SyncState,
    /// Server sync state
    pub server_sync_state: SyncState,
}

impl Default for TopDialogManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TopDialogManager {
    /// Creates a new top dialog manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            is_synchronized: false,
            rating_e_decay: DEFAULT_RATING_E_DECAY,
            by_category: HashMap::new(),
            db_sync_state: SyncState::None,
            server_sync_state: SyncState::None,
        }
    }

    /// Returns true if top dialogs are enabled
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    /// Sets whether top dialogs are enabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    /// Returns true if synchronized with server
    #[must_use]
    pub const fn is_synchronized(&self) -> bool {
        self.is_synchronized
    }

    /// Sets the synchronized state
    pub fn set_synchronized(&mut self, synchronized: bool) {
        self.is_synchronized = synchronized;
    }

    /// Returns the rating e-decay value
    #[must_use]
    pub const fn rating_e_decay(&self) -> i32 {
        self.rating_e_decay
    }

    /// Sets the rating e-decay value
    pub fn set_rating_e_decay(&mut self, decay: i32) {
        self.rating_e_decay = decay;
    }

    /// Gets top dialogs for a category
    #[must_use]
    pub fn get_top_dialogs(&self, category: TopDialogCategory) -> Option<&TopDialogs> {
        self.by_category.get(&category)
    }

    /// Gets mutable top dialogs for a category
    pub fn get_top_dialogs_mut(&mut self, category: TopDialogCategory) -> &mut TopDialogs {
        self.by_category
            .entry(category)
            .or_insert_with(TopDialogs::new)
    }

    /// Called when a dialog is used
    pub fn on_dialog_used(&mut self, category: TopDialogCategory, dialog_id: DialogId, date: i32) {
        if !self.is_enabled {
            return;
        }

        // Extract rating_e_decay before mutable borrow
        let rating_e_decay = self.rating_e_decay;
        let dialogs = self.get_top_dialogs_mut(category);
        let rating_timestamp = dialogs.rating_timestamp;
        let now = date as f64;
        let delta = Self::rating_add_impl(rating_e_decay, now, rating_timestamp);

        // Find or create the dialog entry
        if let Some(dialog) = dialogs.find_mut(dialog_id) {
            dialog.rating += delta;
        } else {
            let mut dialog = TopDialog::new(dialog_id);
            dialog.rating = delta;
            dialogs.add(dialog);
        }

        dialogs.is_dirty = true;
        dialogs.sort();
    }

    /// Removes a dialog from top dialogs
    pub fn remove_dialog(&mut self, category: TopDialogCategory, dialog_id: DialogId) -> bool {
        if !self.is_enabled {
            return false;
        }

        if let Some(dialogs) = self.by_category.get_mut(&category) {
            return dialogs.remove(dialog_id);
        }
        false
    }

    /// Checks if a dialog is in the top dialogs
    #[must_use]
    pub fn is_top_dialog(
        &self,
        category: TopDialogCategory,
        limit: usize,
        dialog_id: DialogId,
    ) -> bool {
        if !self.is_enabled {
            return false;
        }

        if let Some(dialogs) = self.by_category.get(&category) {
            let limit = limit.min(dialogs.len()).min(MAX_TOP_DIALOGS_LIMIT);
            dialogs
                .dialogs
                .iter()
                .take(limit)
                .any(|d| d.dialog_id == dialog_id)
        } else {
            false
        }
    }

    /// Gets all top dialog IDs across categories
    #[must_use]
    pub fn get_all_top_dialog_ids(&self) -> Vec<DialogId> {
        let mut ids = Vec::new();
        for dialogs in self.by_category.values() {
            for dialog in &dialogs.dialogs {
                if !ids.contains(&dialog.dialog_id) {
                    ids.push(dialog.dialog_id);
                }
            }
        }
        ids
    }

    /// Gets dialog IDs for stories (top correspondents with high rating)
    #[must_use]
    pub fn get_story_dialog_ids(&self) -> Vec<DialogId> {
        let mut ids = Vec::new();

        if let Some(dialogs) = self.by_category.get(&TopDialogCategory::Correspondent) {
            for dialog in &dialogs.dialogs {
                if dialog.rating >= MIN_STORY_RATING
                    && dialog.dialog_id.get_type() == DialogType::User
                    && ids.len() < MAX_TOP_DIALOGS_LIMIT
                {
                    ids.push(dialog.dialog_id);
                }
            }
        }

        ids
    }

    /// Normalizes all ratings
    pub fn normalize_ratings(&mut self, server_time: f64) {
        // Extract rating_e_decay before mutable borrow
        let rating_e_decay = self.rating_e_decay;
        for (_category, dialogs) in &mut self.by_category {
            let div_by = Self::current_rating_add_impl(
                rating_e_decay,
                server_time,
                dialogs.rating_timestamp,
            );
            dialogs.rating_timestamp = server_time;

            for dialog in &mut dialogs.dialogs {
                dialog.rating /= div_by;
            }

            dialogs.is_dirty = true;
        }
        self.db_sync_state = SyncState::None;
    }

    /// Clears all top dialogs
    pub fn clear(&mut self) {
        self.by_category.clear();
        self.is_synchronized = false;
    }

    /// Calculates rating add based on time difference
    fn rating_add_impl(rating_e_decay: i32, now: f64, rating_timestamp: f64) -> f64 {
        let diff = now - rating_timestamp;
        if diff <= 0.0 {
            return 1.0;
        }
        let decay = rating_e_decay as f64;
        (diff / decay).exp()
    }

    /// Calculates current rating add
    fn current_rating_add_impl(
        rating_e_decay: i32,
        server_time: f64,
        rating_timestamp: f64,
    ) -> f64 {
        Self::rating_add_impl(rating_e_decay, server_time, rating_timestamp)
    }

    /// Checks if a dialog needs stories
    #[must_use]
    pub fn need_dialog_stores(
        category: TopDialogCategory,
        dialog_id: DialogId,
        rating: f64,
    ) -> bool {
        if category != TopDialogCategory::Correspondent {
            return false;
        }
        // Only user dialogs need stories
        if dialog_id.get_type() != DialogType::User {
            return false;
        }
        rating >= MIN_STORY_RATING
    }
}

impl fmt::Display for TopDialogManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TopDialogManager(enabled: {}, synced: {}, categories: {})",
            self.is_enabled,
            self.is_synchronized,
            self.by_category.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_dialog_new() {
        let user_id = UserId(12345);
        let dialog = TopDialog::new(DialogId::from(user_id));
        assert_eq!(dialog.dialog_id.to_encoded(), 12345);
        assert_eq!(dialog.rating, 0.0);
    }

    #[test]
    fn test_top_dialog_with_rating() {
        let user_id = UserId(12345);
        let dialog = TopDialog::with_rating(DialogId::from(user_id), 10.5);
        assert_eq!(dialog.rating, 10.5);
    }

    #[test]
    fn test_top_dialog_ord() {
        let id1 = UserId(1);
        let id2 = UserId(2);
        let d1 = TopDialog::with_rating(DialogId::from(id1), 10.0);
        let d2 = TopDialog::with_rating(DialogId::from(id2), 20.0);
        assert!(d2 > d1); // Higher rating comes first
    }

    #[test]
    fn test_top_dialogs_new() {
        let dialogs = TopDialogs::new();
        assert!(dialogs.is_empty());
        assert!(!dialogs.is_dirty);
    }

    #[test]
    fn test_top_dialogs_add() {
        let mut dialogs = TopDialogs::new();
        let user_id = UserId(12345);
        let dialog = TopDialog::new(DialogId::from(user_id));
        dialogs.add(dialog);
        assert_eq!(dialogs.len(), 1);
        assert!(dialogs.is_dirty);
    }

    #[test]
    fn test_top_dialogs_remove() {
        let mut dialogs = TopDialogs::new();
        let user_id = UserId(12345);
        let dialog = TopDialog::new(DialogId::from(user_id));
        dialogs.add(dialog);
        assert!(dialogs.remove(DialogId::from(user_id)));
        assert!(dialogs.is_empty());
    }

    #[test]
    fn test_top_dialogs_find() {
        let mut dialogs = TopDialogs::new();
        let user_id = UserId(12345);
        let dialog = TopDialog::new(DialogId::from(user_id));
        dialogs.add(dialog);
        assert!(dialogs.find(DialogId::from(user_id)).is_some());
        assert!(dialogs.find(DialogId::from(UserId(99999))).is_none());
    }

    #[test]
    fn test_top_dialogs_sort() {
        let mut dialogs = TopDialogs::new();
        dialogs.add(TopDialog::with_rating(DialogId::from(UserId(1)), 10.0));
        dialogs.add(TopDialog::with_rating(DialogId::from(UserId(2)), 30.0));
        dialogs.add(TopDialog::with_rating(DialogId::from(UserId(3)), 20.0));
        dialogs.sort();

        assert_eq!(dialogs.dialogs[0].rating, 30.0);
        assert_eq!(dialogs.dialogs[1].rating, 20.0);
        assert_eq!(dialogs.dialogs[2].rating, 10.0);
    }

    #[test]
    fn test_sync_state_default() {
        let state = SyncState::default();
        assert_eq!(state, SyncState::None);
    }

    #[test]
    fn test_top_dialog_manager_new() {
        let manager = TopDialogManager::new();
        assert!(manager.is_enabled());
        assert!(!manager.is_synchronized());
        assert_eq!(manager.rating_e_decay(), DEFAULT_RATING_E_DECAY);
    }

    #[test]
    fn test_set_enabled() {
        let mut manager = TopDialogManager::new();
        manager.set_enabled(false);
        assert!(!manager.is_enabled());
    }

    #[test]
    fn test_set_synchronized() {
        let mut manager = TopDialogManager::new();
        manager.set_synchronized(true);
        assert!(manager.is_synchronized());
    }

    #[test]
    fn test_set_rating_e_decay() {
        let mut manager = TopDialogManager::new();
        manager.set_rating_e_decay(100000);
        assert_eq!(manager.rating_e_decay(), 100000);
    }

    #[test]
    fn test_on_dialog_used() {
        let mut manager = TopDialogManager::new();
        manager.on_dialog_used(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(12345)),
            1234567890,
        );

        let dialogs = manager.get_top_dialogs(TopDialogCategory::Correspondent);
        assert!(dialogs.is_some());
        assert_eq!(dialogs.unwrap().len(), 1);
    }

    #[test]
    fn test_on_dialog_used_multiple() {
        let mut manager = TopDialogManager::new();
        manager.on_dialog_used(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(1)),
            1000000,
        );
        manager.on_dialog_used(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(2)),
            2000000,
        );
        manager.on_dialog_used(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(1)),
            3000000,
        );

        let dialogs = manager
            .get_top_dialogs(TopDialogCategory::Correspondent)
            .unwrap();
        assert_eq!(dialogs.len(), 2);
    }

    #[test]
    fn test_remove_dialog() {
        let mut manager = TopDialogManager::new();
        manager.on_dialog_used(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(12345)),
            1234567890,
        );
        assert!(manager.remove_dialog(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(12345))
        ));

        let dialogs = manager.get_top_dialogs(TopDialogCategory::Correspondent);
        assert!(dialogs.is_some() && dialogs.unwrap().is_empty());
    }

    #[test]
    fn test_is_top_dialog() {
        let mut manager = TopDialogManager::new();
        manager.on_dialog_used(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(12345)),
            1234567890,
        );

        assert!(manager.is_top_dialog(
            TopDialogCategory::Correspondent,
            10,
            DialogId::from(UserId(12345))
        ));
        assert!(!manager.is_top_dialog(
            TopDialogCategory::Correspondent,
            10,
            DialogId::from(UserId(99999))
        ));
    }

    #[test]
    fn test_is_top_dialog_disabled() {
        let mut manager = TopDialogManager::new();
        manager.on_dialog_used(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(12345)),
            1234567890,
        );
        manager.set_enabled(false);

        assert!(!manager.is_top_dialog(
            TopDialogCategory::Correspondent,
            10,
            DialogId::from(UserId(12345))
        ));
    }

    #[test]
    fn test_get_story_dialog_ids() {
        let mut manager = TopDialogManager::new();
        let dialogs = manager.get_top_dialogs_mut(TopDialogCategory::Correspondent);
        dialogs.add(TopDialog::with_rating(
            DialogId::from(UserId(1)),
            MIN_STORY_RATING + 1.0,
        ));
        dialogs.add(TopDialog::with_rating(
            DialogId::from(UserId(2)),
            MIN_STORY_RATING - 1.0,
        ));

        let story_ids = manager.get_story_dialog_ids();
        assert_eq!(story_ids.len(), 1);
        assert_eq!(story_ids[0].to_encoded(), 1);
    }

    #[test]
    fn test_normalize_ratings() {
        let mut manager = TopDialogManager::new();
        let mut dialogs = TopDialogs::new();
        dialogs.rating_timestamp = 1000.0;
        dialogs.add(TopDialog::with_rating(DialogId::from(UserId(1)), 100.0));
        manager
            .by_category
            .insert(TopDialogCategory::Correspondent, dialogs);

        manager.normalize_ratings(2000.0);

        let dialogs = manager
            .get_top_dialogs(TopDialogCategory::Correspondent)
            .unwrap();
        // Ratings should be normalized (divided by some factor)
        assert!(dialogs.dialogs[0].rating < 100.0);
    }

    #[test]
    fn test_clear() {
        let mut manager = TopDialogManager::new();
        manager.on_dialog_used(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(12345)),
            1234567890,
        );
        manager.clear();

        assert!(manager
            .get_top_dialogs(TopDialogCategory::Correspondent)
            .is_none());
        assert!(!manager.is_synchronized());
    }

    #[test]
    fn test_get_all_top_dialog_ids() {
        let mut manager = TopDialogManager::new();
        manager.on_dialog_used(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(1)),
            1000000,
        );
        manager.on_dialog_used(TopDialogCategory::BotPm, DialogId::from(UserId(2)), 2000000);

        let ids = manager.get_all_top_dialog_ids();
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_manager_display() {
        let manager = TopDialogManager::new();
        let display = format!("{}", manager);
        assert!(display.contains("TopDialogManager"));
        assert!(display.contains("enabled: true"));
    }

    #[test]
    fn test_rating_add() {
        // Same time should give rating_add of 1.0
        let add = TopDialogManager::rating_add_impl(DEFAULT_RATING_E_DECAY, 1000.0, 1000.0);
        assert_eq!(add, 1.0);
    }

    #[test]
    fn test_need_dialog_stores() {
        assert!(TopDialogManager::need_dialog_stores(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(1)),
            MIN_STORY_RATING + 1.0
        ));
        assert!(!TopDialogManager::need_dialog_stores(
            TopDialogCategory::Correspondent,
            DialogId::from(UserId(1)),
            MIN_STORY_RATING - 1.0
        ));
        assert!(!TopDialogManager::need_dialog_stores(
            TopDialogCategory::BotPm,
            DialogId::from(UserId(1)),
            MIN_STORY_RATING + 1.0
        ));
    }
}
