// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Recent Dialog List
//!
//! Recent dialog list for Telegram.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_types::DialogId;
use serde::{Deserialize, Serialize};

/// Recent dialog list.
///
/// Based on TDLib's `RecentDialogList` class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentDialogList {
    /// Maximum size of the list.
    max_size: usize,
    /// List of dialog IDs.
    dialog_ids: Vec<DialogId>,
    /// Whether the list is loaded.
    is_loaded: bool,
}

impl Default for RecentDialogList {
    fn default() -> Self {
        Self {
            max_size: 20,
            dialog_ids: Vec::new(),
            is_loaded: false,
        }
    }
}

impl RecentDialogList {
    /// Creates a new recent dialog list.
    #[must_use]
    pub const fn new(max_size: usize) -> Self {
        Self {
            max_size,
            dialog_ids: Vec::new(),
            is_loaded: false,
        }
    }

    /// Returns the maximum size.
    #[must_use]
    pub const fn max_size(&self) -> usize {
        self.max_size
    }

    /// Returns the dialog IDs.
    #[must_use]
    pub fn dialog_ids(&self) -> &[DialogId] {
        &self.dialog_ids
    }

    /// Returns whether the list is loaded.
    #[must_use]
    pub const fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    /// Adds a dialog to the list.
    pub fn add_dialog(&mut self, dialog_id: DialogId) {
        self.dialog_ids.insert(0, dialog_id);
        if self.dialog_ids.len() > self.max_size {
            self.dialog_ids.truncate(self.max_size);
        }
    }

    /// Removes a dialog from the list.
    pub fn remove_dialog(&mut self, dialog_id: DialogId) {
        self.dialog_ids.retain(|&id| id != dialog_id);
    }

    /// Clears all dialogs.
    pub fn clear(&mut self) {
        self.dialog_ids.clear();
    }

    /// Returns the number of dialogs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.dialog_ids.len()
    }

    /// Checks if the list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dialog_ids.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    #[test]
    fn test_default() {
        let list = RecentDialogList::default();
        assert_eq!(list.max_size(), 20);
        assert!(list.is_empty());
    }

    #[test]
    fn test_add_dialog() {
        let mut list = RecentDialogList::new(2);
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        list.add_dialog(dialog_id);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_max_size() {
        let mut list = RecentDialogList::new(2);
        let d1 = DialogId::from_user(UserId::new(1).unwrap());
        let d2 = DialogId::from_user(UserId::new(2).unwrap());
        let d3 = DialogId::from_user(UserId::new(3).unwrap());

        list.add_dialog(d3);
        list.add_dialog(d2);
        list.add_dialog(d1);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_remove_dialog() {
        let mut list = RecentDialogList::new(10);
        let d1 = DialogId::from_user(UserId::new(1).unwrap());

        list.add_dialog(d1);
        assert_eq!(list.len(), 1);

        list.remove_dialog(d1);
        assert!(list.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut list = RecentDialogList::new(10);
        let d1 = DialogId::from_user(UserId::new(1).unwrap());

        list.add_dialog(d1);
        assert!(!list.is_empty());

        list.clear();
        assert!(list.is_empty());
    }
}
