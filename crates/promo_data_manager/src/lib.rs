// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Promo Data Manager
//!
//! Manages promotional and sponsored message data for Telegram MTProto.
//!
//! Based on TDLib's `PromoDataManager` from `td/telegram/PromoDataManager.h`.
//!
//! # Overview
//!
//! The `PromoDataManager` handles the retrieval and management of promotional
//! content from Telegram, including sponsored messages and service announcements.
//! It periodically reloads promo data and provides methods to hide promo content
//! for specific dialogs.
//!
//! # Example
//!
//! ```rust
//! use rustgram_promo_data_manager::PromoDataManager;
//!
//! let mut manager = PromoDataManager::new();
//! manager.init();
//!
//! // Reload promo data
//! manager.reload_promo_data();
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_dialog_id::DialogId;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Error type for PromoDataManager operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PromoDataManagerError {
    /// Manager not initialized.
    #[error("manager not initialized")]
    NotInitialized,

    /// Operation already in progress.
    #[error("operation already in progress")]
    Busy,

    /// Invalid dialog ID.
    #[error("invalid dialog ID")]
    InvalidDialogId,

    /// Network error.
    #[error("network error: {0}")]
    NetworkError(String),
}

/// Result type for PromoDataManager operations.
pub type Result<T> = std::result::Result<T, PromoDataManagerError>;

/// Promo data received from Telegram.
///
/// Contains information about promotional content including
/// the sponsored dialog, expiration time, and suggested actions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromoData {
    /// The dialog ID containing sponsored content, if any.
    pub dialog_id: Option<DialogId>,
    /// Unix timestamp when this promo data expires.
    pub expires_at: i32,
    /// Whether this is a proxy-related promo.
    pub is_proxy: bool,
    /// Public service announcement type, if any.
    pub psa_type: Option<String>,
    /// Public service announcement message, if any.
    pub psa_message: Option<String>,
}

impl PromoData {
    /// Creates empty promo data.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            dialog_id: None,
            expires_at: 0,
            is_proxy: false,
            psa_type: None,
            psa_message: None,
        }
    }

    /// Checks if this promo data is empty (no content).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dialog_id.is_none()
    }

    /// Creates promo data with a dialog ID.
    #[must_use]
    pub fn with_dialog(dialog_id: DialogId) -> Self {
        Self {
            dialog_id: Some(dialog_id),
            ..Self::default()
        }
    }
}

/// Promo data manager for Telegram.
///
/// Manages the retrieval and caching of promotional content from Telegram.
/// This includes sponsored messages and public service announcements.
///
/// # Example
///
/// ```rust
/// use rustgram_promo_data_manager::PromoDataManager;
///
/// let mut manager = PromoDataManager::new();
/// assert!(!manager.is_inited());
///
/// manager.init();
/// assert!(manager.is_inited());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoDataManager {
    /// Whether the manager has been initialized.
    #[serde(default)]
    is_inited: bool,
    /// Whether promo data is currently being reloaded.
    #[serde(default)]
    reloading_promo_data: bool,
    /// Whether a reload is needed after the current operation.
    #[serde(default)]
    need_reload_promo_data: bool,
    /// Current promo data.
    #[serde(default)]
    promo_data: PromoData,
}

impl Default for PromoDataManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PromoDataManager {
    /// Creates a new `PromoDataManager`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_promo_data_manager::PromoDataManager;
    ///
    /// let manager = PromoDataManager::new();
    /// assert!(!manager.is_inited());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            is_inited: false,
            reloading_promo_data: false,
            need_reload_promo_data: false,
            promo_data: PromoData::empty(),
        }
    }

    /// Initializes the manager.
    ///
    /// Should be called once the client is authorized.
    /// Sets up the initial state and schedules the first promo data reload.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_promo_data_manager::PromoDataManager;
    ///
    /// let mut manager = PromoDataManager::new();
    /// manager.init();
    /// assert!(manager.is_inited());
    /// ```
    pub fn init(&mut self) {
        if self.is_inited {
            return;
        }
        self.is_inited = true;
        // In a full implementation, this would schedule the first reload
    }

    /// Reloads promo data from Telegram.
    ///
    /// Initiates an asynchronous request to fetch the latest promotional content.
    /// If a reload is already in progress, marks that a reload is needed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_promo_data_manager::PromoDataManager;
    ///
    /// let mut manager = PromoDataManager::new();
    /// manager.init();
    /// manager.reload_promo_data();
    /// ```
    pub fn reload_promo_data(&mut self) -> Result<()> {
        if !self.is_inited {
            return Err(PromoDataManagerError::NotInitialized);
        }
        if self.reloading_promo_data {
            self.need_reload_promo_data = true;
            return Err(PromoDataManagerError::Busy);
        }
        // In a full implementation, this would send a network request
        self.reloading_promo_data = true;
        Ok(())
    }

    /// Removes the current sponsored dialog.
    ///
    /// Clears any currently displayed promotional content.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_promo_data_manager::PromoDataManager;
    ///
    /// let mut manager = PromoDataManager::new();
    /// manager.init();
    /// manager.remove_sponsored_dialog();
    /// assert!(manager.promo_data().is_empty());
    /// ```
    pub fn remove_sponsored_dialog(&mut self) {
        self.promo_data = PromoData::empty();
    }

    /// Hides promotional data for a specific dialog.
    ///
    /// This method removes the sponsored dialog and sends a request
    /// to Telegram to hide promotional content for the specified dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog to hide promo data for
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_promo_data_manager::PromoDataManager;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let mut manager = PromoDataManager::new();
    /// manager.init();
    ///
    /// let dialog_id = DialogId::new(123456);
    /// manager.hide_promo_data(dialog_id);
    /// ```
    pub fn hide_promo_data(&mut self, dialog_id: DialogId) -> Result<()> {
        if !dialog_id.is_valid() {
            return Err(PromoDataManagerError::InvalidDialogId);
        }
        self.remove_sponsored_dialog();
        // In a full implementation, this would send a network request
        Ok(())
    }

    /// Returns whether the manager has been initialized.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_promo_data_manager::PromoDataManager;
    ///
    /// let manager = PromoDataManager::new();
    /// assert!(!manager.is_inited());
    /// ```
    #[must_use]
    pub const fn is_inited(&self) -> bool {
        self.is_inited
    }

    /// Returns whether promo data is currently being reloaded.
    #[must_use]
    pub const fn is_reloading(&self) -> bool {
        self.reloading_promo_data
    }

    /// Returns the current promo data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_promo_data_manager::PromoDataManager;
    ///
    /// let manager = PromoDataManager::new();
    /// let promo = manager.promo_data();
    /// assert!(promo.is_empty());
    /// ```
    #[must_use]
    pub const fn promo_data(&self) -> &PromoData {
        &self.promo_data
    }

    /// Handles the result of a promo data request.
    ///
    /// This method should be called when a network request completes.
    ///
    /// # Arguments
    ///
    /// * `result` - The promo data received from the server
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_promo_data_manager::{PromoDataManager, PromoData};
    ///
    /// let mut manager = PromoDataManager::new();
    /// manager.init();
    ///
    /// let promo = PromoData::empty();
    /// manager.on_get_promo_data(Ok(promo));
    /// ```
    pub fn on_get_promo_data(&mut self, result: Result<PromoData>) {
        self.reloading_promo_data = false;

        match result {
            Ok(data) => {
                self.promo_data = data;
                if self.need_reload_promo_data {
                    self.need_reload_promo_data = false;
                    // Trigger another reload
                }
            }
            Err(_) => {
                // In a full implementation, would schedule retry
            }
        }
    }

    /// Resets the manager to uninitialized state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_promo_data_manager::PromoDataManager;
    ///
    /// let mut manager = PromoDataManager::new();
    /// manager.init();
    /// assert!(manager.is_inited());
    ///
    /// manager.reset();
    /// assert!(!manager.is_inited());
    /// ```
    pub fn reset(&mut self) {
        self.is_inited = false;
        self.reloading_promo_data = false;
        self.need_reload_promo_data = false;
        self.promo_data = PromoData::empty();
    }
}

impl fmt::Display for PromoDataManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PromoDataManager(inited={}, reloading={})",
            self.is_inited, self.reloading_promo_data
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let manager = PromoDataManager::new();
        assert!(!manager.is_inited());
        assert!(!manager.is_reloading());
        assert!(manager.promo_data().is_empty());
    }

    #[test]
    fn test_default() {
        let manager = PromoDataManager::default();
        assert!(!manager.is_inited());
    }

    #[test]
    fn test_init() {
        let mut manager = PromoDataManager::new();
        manager.init();
        assert!(manager.is_inited());
    }

    #[test]
    fn test_init_idempotent() {
        let mut manager = PromoDataManager::new();
        manager.init();
        manager.init();
        assert!(manager.is_inited());
    }

    #[test]
    fn test_reload_promo_data() {
        let mut manager = PromoDataManager::new();
        manager.init();
        let result = manager.reload_promo_data();
        assert!(result.is_ok());
        assert!(manager.is_reloading());
    }

    #[test]
    fn test_reload_promo_data_not_initialized() {
        let mut manager = PromoDataManager::new();
        let result = manager.reload_promo_data();
        assert!(matches!(result, Err(PromoDataManagerError::NotInitialized)));
    }

    #[test]
    fn test_reload_promo_data_busy() {
        let mut manager = PromoDataManager::new();
        manager.init();
        let _ = manager.reload_promo_data();
        let result = manager.reload_promo_data();
        assert!(matches!(result, Err(PromoDataManagerError::Busy)));
    }

    #[test]
    fn test_remove_sponsored_dialog() {
        let mut manager = PromoDataManager::new();
        manager.init();

        // Set some promo data
        manager.promo_data = PromoData::with_dialog(DialogId::new(123456));
        assert!(!manager.promo_data().is_empty());

        manager.remove_sponsored_dialog();
        assert!(manager.promo_data().is_empty());
    }

    #[test]
    fn test_hide_promo_data() {
        let mut manager = PromoDataManager::new();
        manager.init();

        let dialog_id = DialogId::new(123456);
        let result = manager.hide_promo_data(dialog_id);
        assert!(result.is_ok());
        assert!(manager.promo_data().is_empty());
    }

    #[test]
    fn test_hide_promo_data_invalid_dialog() {
        let mut manager = PromoDataManager::new();
        manager.init();

        let dialog_id = DialogId::new(0); // Invalid
        let result = manager.hide_promo_data(dialog_id);
        assert!(matches!(
            result,
            Err(PromoDataManagerError::InvalidDialogId)
        ));
    }

    #[test]
    fn test_on_get_promo_data() {
        let mut manager = PromoDataManager::new();
        manager.init();
        manager.reloading_promo_data = true;

        let promo = PromoData::empty();
        manager.on_get_promo_data(Ok(promo));

        assert!(!manager.is_reloading());
        assert!(manager.promo_data().is_empty());
    }

    #[test]
    fn test_on_get_promo_data_with_content() {
        let mut manager = PromoDataManager::new();
        manager.init();
        manager.reloading_promo_data = true;

        let dialog_id = DialogId::new(123456);
        let promo = PromoData::with_dialog(dialog_id);
        manager.on_get_promo_data(Ok(promo));

        assert!(!manager.is_reloading());
        assert_eq!(manager.promo_data().dialog_id, Some(dialog_id));
    }

    #[test]
    fn test_on_get_promo_data_error() {
        let mut manager = PromoDataManager::new();
        manager.init();
        manager.reloading_promo_data = true;

        manager.on_get_promo_data(Err(PromoDataManagerError::NetworkError("test".to_string())));

        assert!(!manager.is_reloading());
    }

    #[test]
    fn test_reset() {
        let mut manager = PromoDataManager::new();
        manager.init();
        let _ = manager.reload_promo_data();

        manager.reset();
        assert!(!manager.is_inited());
        assert!(!manager.is_reloading());
        assert!(manager.promo_data().is_empty());
    }

    #[test]
    fn test_display() {
        let manager = PromoDataManager::new();
        let display = format!("{manager}");
        assert!(display.contains("PromoDataManager"));
        assert!(display.contains("inited=false"));
    }

    #[test]
    fn test_promo_data_default() {
        let data = PromoData::default();
        assert!(data.is_empty());
        assert!(data.dialog_id.is_none());
        assert_eq!(data.expires_at, 0);
        assert!(!data.is_proxy);
    }

    #[test]
    fn test_promo_data_empty() {
        let data = PromoData::empty();
        assert!(data.is_empty());
    }

    #[test]
    fn test_promo_data_with_dialog() {
        let dialog_id = DialogId::new(123456);
        let data = PromoData::with_dialog(dialog_id);
        assert!(!data.is_empty());
        assert_eq!(data.dialog_id, Some(dialog_id));
    }

    #[test]
    fn test_promo_data_clone() {
        let dialog_id = DialogId::new(123456);
        let data1 = PromoData::with_dialog(dialog_id);
        let data2 = data1.clone();
        assert_eq!(data1.dialog_id, data2.dialog_id);
    }

    #[test]
    fn test_serialization() {
        let manager = PromoDataManager::new();
        let json = serde_json::to_string(&manager).expect("Failed to serialize");
        assert!(json.contains("inited"));

        let deserialized: PromoDataManager =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.is_inited(), manager.is_inited());
    }

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", PromoDataManagerError::NotInitialized),
            "manager not initialized"
        );
        assert_eq!(
            format!("{}", PromoDataManagerError::Busy),
            "operation already in progress"
        );
        assert_eq!(
            format!("{}", PromoDataManagerError::InvalidDialogId),
            "invalid dialog ID"
        );
    }

    #[test]
    fn test_manager_clone() {
        let mut manager1 = PromoDataManager::new();
        manager1.init();

        let manager2 = manager1.clone();
        assert_eq!(manager1.is_inited(), manager2.is_inited());
    }
}
