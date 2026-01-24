// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Suggested Action Manager
//!
//! Manages suggested actions shown to users in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`SuggestedActionManager`], which tracks
//! and manages suggested actions shown to users (setup prompts, feature suggestions, etc.).
//!
//! ## Example
//!
//! ```rust
//! use rustgram_suggested_action_manager::SuggestedActionManager;
//! use rustgram_suggested_action::SuggestedAction;
//!
//! let manager = SuggestedActionManager::new();
//! manager.update_suggested_actions(vec![SuggestedAction::CheckPassword]);
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use rustgram_dialog_id::DialogId;
use rustgram_suggested_action::SuggestedAction;

/// Manager for suggested actions.
///
/// Tracks suggested actions and dialog-specific suggestions.
///
/// # TDLib Alignment
///
/// Aligns with TDLib's `SuggestedActionManager` class in `SuggestedActionManager.h`.
///
/// # Example
///
/// ```rust
/// use rustgram_suggested_action_manager::SuggestedActionManager;
/// use rustgram_suggested_action::SuggestedAction;
///
/// # #[tokio::main]
/// # async fn main() {
/// let manager = SuggestedActionManager::new();
/// manager.update_suggested_actions(vec![SuggestedAction::CheckPassword]);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SuggestedActionManager {
    /// Inner state protected by RwLock for concurrent access.
    inner: Arc<RwLock<SuggestedActionManagerInner>>,
}

impl SuggestedActionManager {
    /// Creates a new suggested action manager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action_manager::SuggestedActionManager;
    ///
    /// let manager = SuggestedActionManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(SuggestedActionManagerInner::new())),
        }
    }

    /// Updates the list of suggested actions.
    ///
    /// # Arguments
    ///
    /// * `actions` - New list of suggested actions
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action_manager::SuggestedActionManager;
    /// use rustgram_suggested_action::SuggestedAction;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = SuggestedActionManager::new();
    /// manager.update_suggested_actions(vec![
    ///     SuggestedAction::CheckPassword,
    ///     SuggestedAction::SetPassword,
    /// ]);
    /// # }
    /// ```
    pub async fn update_suggested_actions(&self, actions: Vec<SuggestedAction>) {
        let mut inner = self.inner.write().await;
        inner.suggested_actions = actions;
    }

    /// Returns the current suggested actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action_manager::SuggestedActionManager;
    /// use rustgram_suggested_action::SuggestedAction;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = SuggestedActionManager::new();
    /// manager.update_suggested_actions(vec![SuggestedAction::CheckPassword]).await;
    /// let actions = manager.get_suggested_actions().await;
    /// assert_eq!(actions.len(), 1);
    /// # }
    /// ```
    #[must_use]
    pub async fn get_suggested_actions(&self) -> Vec<SuggestedAction> {
        let inner = self.inner.read().await;
        inner.suggested_actions.clone()
    }

    /// Hides a suggested action temporarily.
    ///
    /// # Arguments
    ///
    /// * `action` - The action to hide
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action_manager::SuggestedActionManager;
    /// use rustgram_suggested_action::SuggestedAction;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = SuggestedActionManager::new();
    /// manager.update_suggested_actions(vec![SuggestedAction::CheckPassword]);
    /// manager.hide_suggested_action(SuggestedAction::CheckPassword);
    /// let actions = manager.get_suggested_actions().await;
    /// assert!(actions.is_empty());
    /// # }
    /// ```
    pub async fn hide_suggested_action(&self, action: SuggestedAction) {
        let mut inner = self.inner.write().await;
        inner.suggested_actions.retain(|a| a != &action);
    }

    /// Removes a dialog-specific suggested action.
    ///
    /// # Arguments
    ///
    /// * `action` - The action to remove
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action_manager::SuggestedActionManager;
    /// use rustgram_suggested_action::SuggestedAction;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = SuggestedActionManager::new();
    /// manager.set_dialog_pending_suggestions(DialogId::new(123), vec!["hint".to_string()]);
    /// manager.remove_dialog_suggested_action(SuggestedAction::ViewChecksHint);
    /// # }
    /// ```
    pub async fn remove_dialog_suggested_action(&self, action: SuggestedAction) {
        let mut inner = self.inner.write().await;
        for actions in inner.dialog_suggested_actions.values_mut() {
            actions.retain(|a| a != &action);
        }
    }

    /// Sets pending suggestions for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `suggestions` - List of suggestion strings
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action_manager::SuggestedActionManager;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = SuggestedActionManager::new();
    /// manager.set_dialog_pending_suggestions(DialogId::new(123), vec!["hint1".to_string(), "hint2".to_string()]);
    /// # }
    /// ```
    pub async fn set_dialog_pending_suggestions(
        &self,
        dialog_id: DialogId,
        suggestions: Vec<String>,
    ) {
        let mut inner = self.inner.write().await;
        inner
            .dialog_pending_suggestions
            .insert(dialog_id, suggestions);
    }

    /// Returns pending suggestions for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    ///
    /// # Returns
    ///
    /// The list of pending suggestions, or an empty vector if none exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action_manager::SuggestedActionManager;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = SuggestedActionManager::new();
    /// manager.set_dialog_pending_suggestions(DialogId::new(123), vec!["hint1".to_string()]).await;
    /// let suggestions = manager.get_dialog_pending_suggestions(DialogId::new(123)).await;
    /// assert_eq!(suggestions.len(), 1);
    /// # }
    /// ```
    #[must_use]
    pub async fn get_dialog_pending_suggestions(&self, dialog_id: DialogId) -> Vec<String> {
        let inner = self.inner.read().await;
        inner
            .dialog_pending_suggestions
            .get(&dialog_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Returns all suggested actions including dialog-specific ones.
    ///
    /// # Returns
    ///
    /// A combined list of global and dialog-specific suggested actions
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action_manager::SuggestedActionManager;
    /// use rustgram_suggested_action::SuggestedAction;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = SuggestedActionManager::new();
    /// manager.update_suggested_actions(vec![SuggestedAction::CheckPassword]).await;
    /// let all_actions = manager.get_all_suggested_actions().await;
    /// assert_eq!(all_actions.len(), 1);
    /// # }
    /// ```
    #[must_use]
    pub async fn get_all_suggested_actions(&self) -> Vec<SuggestedAction> {
        let inner = self.inner.read().await;
        let mut all_actions = inner.suggested_actions.clone();
        for actions in inner.dialog_suggested_actions.values() {
            all_actions.extend(actions.clone());
        }
        all_actions
    }

    /// Clears all suggested actions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action_manager::SuggestedActionManager;
    /// use rustgram_suggested_action::SuggestedAction;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = SuggestedActionManager::new();
    /// manager.update_suggested_actions(vec![SuggestedAction::CheckPassword]);
    /// manager.clear();
    /// let actions = manager.get_suggested_actions().await;
    /// assert!(actions.is_empty());
    /// # }
    /// ```
    pub async fn clear(&self) {
        let mut inner = self.inner.write().await;
        inner.suggested_actions.clear();
        inner.dialog_suggested_actions.clear();
        inner.dialog_pending_suggestions.clear();
    }
}

impl Default for SuggestedActionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Inner state of the suggested action manager.
#[derive(Debug, Default)]
struct SuggestedActionManagerInner {
    /// Current suggested actions.
    suggested_actions: Vec<SuggestedAction>,

    /// Dialog-specific suggested actions.
    dialog_suggested_actions: HashMap<DialogId, Vec<SuggestedAction>>,

    /// Dialog pending suggestions.
    dialog_pending_suggestions: HashMap<DialogId, Vec<String>>,
}

impl SuggestedActionManagerInner {
    /// Creates a new inner state.
    #[must_use]
    fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== new Tests ==========

    #[tokio::test]
    async fn test_new() {
        let manager = SuggestedActionManager::new();
        let actions = manager.get_suggested_actions().await;
        assert!(actions.is_empty());
    }

    #[tokio::test]
    async fn test_default() {
        let manager = SuggestedActionManager::default();
        let actions = manager.get_suggested_actions().await;
        assert!(actions.is_empty());
    }

    // ========== update_suggested_actions Tests ==========

    #[tokio::test]
    async fn test_update_suggested_actions_empty() {
        let manager = SuggestedActionManager::new();
        manager.update_suggested_actions(vec![]).await;
        let actions = manager.get_suggested_actions().await;
        assert!(actions.is_empty());
    }

    #[tokio::test]
    async fn test_update_suggested_actions_single() {
        let manager = SuggestedActionManager::new();
        manager
            .update_suggested_actions(vec![SuggestedAction::CheckPassword])
            .await;
        let actions = manager.get_suggested_actions().await;
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], SuggestedAction::CheckPassword);
    }

    #[tokio::test]
    async fn test_update_suggested_actions_multiple() {
        let manager = SuggestedActionManager::new();
        manager
            .update_suggested_actions(vec![
                SuggestedAction::CheckPassword,
                SuggestedAction::SetPassword,
                SuggestedAction::UpgradePremium,
            ])
            .await;
        let actions = manager.get_suggested_actions().await;
        assert_eq!(actions.len(), 3);
    }

    #[tokio::test]
    async fn test_update_suggested_actions_replace() {
        let manager = SuggestedActionManager::new();
        manager
            .update_suggested_actions(vec![SuggestedAction::CheckPassword])
            .await;
        manager
            .update_suggested_actions(vec![SuggestedAction::SetPassword])
            .await;
        let actions = manager.get_suggested_actions().await;
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], SuggestedAction::SetPassword);
    }

    // ========== get_suggested_actions Tests ==========

    #[tokio::test]
    async fn test_get_suggested_actions_returns_clone() {
        let manager = SuggestedActionManager::new();
        manager
            .update_suggested_actions(vec![SuggestedAction::CheckPassword])
            .await;
        let actions1 = manager.get_suggested_actions().await;
        let actions2 = manager.get_suggested_actions().await;
        assert_eq!(actions1, actions2);
    }

    // ========== hide_suggested_action Tests ==========

    #[tokio::test]
    async fn test_hide_suggested_action_removes_action() {
        let manager = SuggestedActionManager::new();
        manager
            .update_suggested_actions(vec![
                SuggestedAction::CheckPassword,
                SuggestedAction::SetPassword,
            ])
            .await;
        manager.hide_suggested_action(SuggestedAction::CheckPassword).await;
        let actions = manager.get_suggested_actions().await;
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], SuggestedAction::SetPassword);
    }

    #[tokio::test]
    async fn test_hide_suggested_action_not_present() {
        let manager = SuggestedActionManager::new();
        manager
            .update_suggested_actions(vec![SuggestedAction::CheckPassword])
            .await;
        manager.hide_suggested_action(SuggestedAction::SetPassword).await;
        let actions = manager.get_suggested_actions().await;
        assert_eq!(actions.len(), 1);
    }

    #[tokio::test]
    async fn test_hide_suggested_action_duplicates() {
        let manager = SuggestedActionManager::new();
        manager
            .update_suggested_actions(vec![
                SuggestedAction::CheckPassword,
                SuggestedAction::CheckPassword,
            ])
            .await;
        manager.hide_suggested_action(SuggestedAction::CheckPassword).await;
        let actions = manager.get_suggested_actions().await;
        assert!(actions.is_empty());
    }

    // ========== remove_dialog_suggested_action Tests ==========

    #[tokio::test]
    async fn test_remove_dialog_suggested_action() {
        let manager = SuggestedActionManager::new();
        let _dialog_id = DialogId::new(123);
        // Note: This is a simplified test - in real implementation, dialog actions
        // would be set through a different mechanism
        manager.remove_dialog_suggested_action(SuggestedAction::CheckPassword).await;
        // Test passes if no panic occurs
    }

    // ========== set_dialog_pending_suggestions Tests ==========

    #[tokio::test]
    async fn test_set_dialog_pending_suggestions() {
        let manager = SuggestedActionManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .set_dialog_pending_suggestions(dialog_id, vec!["hint1".to_string(), "hint2".to_string()])
            .await;
        let suggestions = manager.get_dialog_pending_suggestions(dialog_id).await;
        assert_eq!(suggestions.len(), 2);
    }

    #[tokio::test]
    async fn test_set_dialog_pending_suggestions_replace() {
        let manager = SuggestedActionManager::new();
        let dialog_id = DialogId::new(123);
        manager
            .set_dialog_pending_suggestions(dialog_id, vec!["hint1".to_string()])
            .await;
        manager
            .set_dialog_pending_suggestions(dialog_id, vec!["hint2".to_string()])
            .await;
        let suggestions = manager.get_dialog_pending_suggestions(dialog_id).await;
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0], "hint2");
    }

    // ========== get_dialog_pending_suggestions Tests ==========

    #[tokio::test]
    async fn test_get_dialog_pending_suggestions_empty() {
        let manager = SuggestedActionManager::new();
        let dialog_id = DialogId::new(123);
        let suggestions = manager.get_dialog_pending_suggestions(dialog_id).await;
        assert!(suggestions.is_empty());
    }

    #[tokio::test]
    async fn test_get_dialog_pending_suggestions_different_dialog() {
        let manager = SuggestedActionManager::new();
        let dialog_id1 = DialogId::new(123);
        let dialog_id2 = DialogId::new(456);
        manager
            .set_dialog_pending_suggestions(dialog_id1, vec!["hint1".to_string()])
            .await;
        let suggestions = manager.get_dialog_pending_suggestions(dialog_id2).await;
        assert!(suggestions.is_empty());
    }

    // ========== get_all_suggested_actions Tests ==========

    #[tokio::test]
    async fn test_get_all_suggested_actions_only_global() {
        let manager = SuggestedActionManager::new();
        manager
            .update_suggested_actions(vec![SuggestedAction::CheckPassword])
            .await;
        let all_actions = manager.get_all_suggested_actions().await;
        assert_eq!(all_actions.len(), 1);
    }

    #[tokio::test]
    async fn test_get_all_suggested_actions_empty() {
        let manager = SuggestedActionManager::new();
        let all_actions = manager.get_all_suggested_actions().await;
        assert!(all_actions.is_empty());
    }

    // ========== clear Tests ==========

    #[tokio::test]
    async fn test_clear_clears_actions() {
        let manager = SuggestedActionManager::new();
        manager
            .update_suggested_actions(vec![SuggestedAction::CheckPassword])
            .await;
        let dialog_id = DialogId::new(123);
        manager
            .set_dialog_pending_suggestions(dialog_id, vec!["hint1".to_string()])
            .await;

        manager.clear().await;

        let actions = manager.get_suggested_actions().await;
        assert!(actions.is_empty());

        let suggestions = manager.get_dialog_pending_suggestions(dialog_id).await;
        assert!(suggestions.is_empty());
    }

    // ========== concurrent access Tests ==========

    #[tokio::test]
    async fn test_concurrent_update_and_read() {
        let manager = Arc::new(SuggestedActionManager::new());
        let manager_clone = Arc::clone(&manager);

        tokio::spawn(async move {
            for _ in 0..10 {
                manager_clone
                    .update_suggested_actions(vec![SuggestedAction::CheckPassword])
                    .await;
            }
        });

        for _ in 0..10 {
            let _actions = manager.get_suggested_actions().await;
        }
        // Test passes if no deadlock or panic occurs
    }

    // ========== clone Tests ==========

    #[tokio::test]
    async fn test_clone_shares_state() {
        let manager1 = SuggestedActionManager::new();
        manager1
            .update_suggested_actions(vec![SuggestedAction::CheckPassword])
            .await;
        let manager2 = manager1.clone();
        let actions = manager2.get_suggested_actions().await;
        assert_eq!(actions.len(), 1);
    }
}
