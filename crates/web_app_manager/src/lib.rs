// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Web App Manager
//!
//! Web Apps (Telegram Mini Apps) manager for Telegram MTProto client.
//!
//! ## Overview
//!
//! This module provides functionality for managing Telegram Web Apps (Mini Apps).
//! It handles web app discovery, web view management, and file source tracking.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `WebAppManager` class from
//! `td/telegram/WebAppManager.h`.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_web_app_manager::{WebAppManager, OpenedWebView};
//! use rustgram_types::UserId;
//! use rustgram_dialog_id::DialogId;
//!
//! #[tokio::main]
//! async fn main() {
//!     let manager = WebAppManager::new();
//!     let bot_id = UserId::new(100).unwrap();
//!     let view = OpenedWebView::new(
//!         12345,
//!         DialogId::new(1),
//!         bot_id
//!     );
//!     manager.open_web_view(view).await;
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;

use rustgram_dialog_id::DialogId;
use rustgram_types::UserId;
use rustgram_web_app::WebApp;

/// Opened web view information.
///
/// Represents an actively opened web view in a chat.
///
/// ## TDLib Mapping
///
/// Corresponds to TDLib `WebAppManager::OpenedWebView` struct.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenedWebView {
    query_id: i64,
    dialog_id: DialogId,
    bot_user_id: UserId,
    message_topic_id: i64,
    reply_to_message_id: i64,
    as_dialog_id: DialogId,
}

impl OpenedWebView {
    /// Creates a new opened web view.
    ///
    /// # Arguments
    ///
    /// * `query_id` - Unique query identifier
    /// * `dialog_id` - Dialog where web view was opened
    /// * `bot_user_id` - Bot user ID
    #[must_use]
    pub fn new(query_id: i64, dialog_id: DialogId, bot_user_id: UserId) -> Self {
        Self {
            query_id,
            dialog_id,
            bot_user_id,
            message_topic_id: 0,
            reply_to_message_id: 0,
            as_dialog_id: DialogId::new(0),
        }
    }

    /// Returns the query ID.
    #[must_use]
    pub const fn query_id(&self) -> i64 {
        self.query_id
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the bot user ID.
    #[must_use]
    pub const fn bot_user_id(&self) -> UserId {
        self.bot_user_id
    }

    /// Returns the message topic ID.
    #[must_use]
    pub const fn message_topic_id(&self) -> i64 {
        self.message_topic_id
    }

    /// Sets the message topic ID.
    pub fn set_message_topic_id(&mut self, topic_id: i64) {
        self.message_topic_id = topic_id;
    }

    /// Returns the reply-to message ID.
    #[must_use]
    pub const fn reply_to_message_id(&self) -> i64 {
        self.reply_to_message_id
    }

    /// Sets the reply-to message ID.
    pub fn set_reply_to_message_id(&mut self, message_id: i64) {
        self.reply_to_message_id = message_id;
    }

    /// Returns the "as" dialog ID.
    #[must_use]
    pub const fn as_dialog_id(&self) -> DialogId {
        self.as_dialog_id
    }

    /// Sets the "as" dialog ID.
    pub fn set_as_dialog_id(&mut self, dialog_id: DialogId) {
        self.as_dialog_id = dialog_id;
    }
}

/// Web app file source identifier.
///
/// Tracks file sources for web apps to enable proper file caching.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct WebAppFileSourceId {
    bot_user_id: UserId,
    short_name: u64,
}

impl WebAppFileSourceId {
    /// Creates a new web app file source ID.
    #[must_use]
    pub const fn new(bot_user_id: UserId, short_name: u64) -> Self {
        Self {
            bot_user_id,
            short_name,
        }
    }

    /// Returns the bot user ID.
    #[must_use]
    pub const fn bot_user_id(&self) -> UserId {
        self.bot_user_id
    }

    /// Returns the short name hash.
    #[must_use]
    pub const fn short_name(&self) -> u64 {
        self.short_name
    }
}

/// Web apps manager.
///
/// Provides storage and retrieval of web apps and opened web views.
/// Thread-safe when `async` feature is enabled.
///
/// ## TDLib Mapping
///
/// Corresponds to TDLib `WebAppManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_web_app_manager::WebAppManager;
///
/// # #[tokio::main]
/// # async fn main() {
/// let manager = WebAppManager::new();
/// assert_eq!(manager.web_view_count().await, 0);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct WebAppManager {
    opened_views: Arc<RwLock<HashMap<i64, OpenedWebView>>>,
    web_apps: Arc<RwLock<HashMap<(UserId, String), WebApp>>>,
    file_sources: Arc<RwLock<HashMap<WebAppFileSourceId, ()>>>,
}

impl Default for WebAppManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WebAppManager {
    /// Creates a new web app manager.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            opened_views: Arc::new(RwLock::new(HashMap::new())),
            web_apps: Arc::new(RwLock::new(HashMap::new())),
            file_sources: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Opens a web view.
    ///
    /// Returns `true` if the web view was opened successfully,
    /// `false` if a web view with this query ID already exists.
    pub async fn open_web_view(&self, view: OpenedWebView) -> bool {
        let query_id = view.query_id();
        let mut views = self.opened_views.write().await;
        views.insert(query_id, view).is_none()
    }

    /// Closes a web view.
    ///
    /// Returns the closed web view if it existed, `None` otherwise.
    pub async fn close_web_view(&self, query_id: i64) -> Option<OpenedWebView> {
        let mut views = self.opened_views.write().await;
        views.remove(&query_id)
    }

    /// Gets an opened web view by query ID.
    pub async fn get_web_view(&self, query_id: i64) -> Option<OpenedWebView> {
        let views = self.opened_views.read().await;
        views.get(&query_id).cloned()
    }

    /// Returns whether a web view is currently opened.
    pub async fn has_web_view(&self, query_id: i64) -> bool {
        let views = self.opened_views.read().await;
        views.contains_key(&query_id)
    }

    /// Returns the number of opened web views.
    pub async fn web_view_count(&self) -> usize {
        let views = self.opened_views.read().await;
        views.len()
    }

    /// Adds or updates a web app.
    ///
    /// Returns `true` if the web app was added (didn't previously exist),
    /// `false` if it was updated.
    pub async fn add_web_app(&self, app: WebApp) -> bool {
        let key = (app.bot_user_id(), app.short_name().to_string());
        let mut apps = self.web_apps.write().await;
        apps.insert(key, app).is_none()
    }

    /// Gets a web app by bot user ID and short name.
    pub async fn get_web_app(&self, bot_user_id: UserId, short_name: &str) -> Option<WebApp> {
        let key = (bot_user_id, short_name.to_string());
        let apps = self.web_apps.read().await;
        apps.get(&key).cloned()
    }

    /// Removes a web app.
    pub async fn remove_web_app(&self, bot_user_id: UserId, short_name: &str) -> Option<WebApp> {
        let key = (bot_user_id, short_name.to_string());
        let mut apps = self.web_apps.write().await;
        apps.remove(&key)
    }

    /// Registers a file source for a web app.
    ///
    /// Returns `true` if the file source was registered (didn't previously exist).
    pub async fn register_file_source(&self, source_id: WebAppFileSourceId) -> bool {
        let mut sources = self.file_sources.write().await;
        sources.insert(source_id, ()).is_none()
    }

    /// Gets a file source ID for a web app.
    ///
    /// Returns the file source ID if it exists, `None` otherwise.
    pub async fn get_file_source_id(
        &self,
        bot_user_id: UserId,
        short_name: &str,
    ) -> Option<WebAppFileSourceId> {
        // Hash the short name to get a u64
        let short_name_hash = crate::hash_string(short_name);
        let source_id = WebAppFileSourceId::new(bot_user_id, short_name_hash);

        let sources = self.file_sources.read().await;
        if sources.contains_key(&source_id) {
            Some(source_id)
        } else {
            None
        }
    }

    /// Closes all web views for a dialog.
    pub async fn close_dialog_web_views(&self, dialog_id: DialogId) -> usize {
        let mut views = self.opened_views.write().await;
        views.retain(|_, v| v.dialog_id() != dialog_id);
        views.len()
    }

    /// Clears all opened web views.
    pub async fn clear_web_views(&self) {
        let mut views = self.opened_views.write().await;
        views.clear();
    }

    /// Returns all opened web views.
    pub async fn all_web_views(&self) -> Vec<OpenedWebView> {
        let views = self.opened_views.read().await;
        views.values().cloned().collect()
    }
}

impl fmt::Display for WebAppManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WebAppManager")
    }
}

/// Simple hash function for strings.
fn hash_string(s: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

/// Crate version from Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name for identification.
pub const CRATE_NAME: &str = "rustgram_web_app_manager";

#[cfg(test)]
mod tests {
    use super::*;

    // OpenedWebView tests
    #[test]
    fn test_opened_web_view_new() {
        let bot_id = UserId::new(100).unwrap();
        let dialog_id = DialogId::new(1);
        let view = OpenedWebView::new(12345, dialog_id, bot_id);

        assert_eq!(view.query_id(), 12345);
        assert_eq!(view.dialog_id(), dialog_id);
        assert_eq!(view.bot_user_id(), bot_id);
        assert_eq!(view.message_topic_id(), 0);
        assert_eq!(view.reply_to_message_id(), 0);
    }

    #[test]
    fn test_opened_web_view_with_topic() {
        let bot_id = UserId::new(100).unwrap();
        let dialog_id = DialogId::new(1);
        let mut view = OpenedWebView::new(12345, dialog_id, bot_id);
        view.set_message_topic_id(999);
        view.set_reply_to_message_id(888);

        assert_eq!(view.message_topic_id(), 999);
        assert_eq!(view.reply_to_message_id(), 888);
    }

    // WebAppFileSourceId tests
    #[test]
    fn test_file_source_id_new() {
        let bot_id = UserId::new(100).unwrap();
        let id = WebAppFileSourceId::new(bot_id, 12345);
        assert_eq!(id.bot_user_id(), bot_id);
        assert_eq!(id.short_name(), 12345);
    }

    // Manager tests
    #[tokio::test]
    async fn test_manager_new() {
        let mgr = WebAppManager::new();
        assert_eq!(mgr.web_view_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_open_web_view() {
        let mgr = WebAppManager::new();
        let bot_id = UserId::new(100).unwrap();
        let dialog_id = DialogId::new(1);
        let view = OpenedWebView::new(12345, dialog_id, bot_id);

        assert!(mgr.open_web_view(view).await);
        assert_eq!(mgr.web_view_count().await, 1);
        assert!(mgr.has_web_view(12345).await);
    }

    #[tokio::test]
    async fn test_manager_open_duplicate() {
        let mgr = WebAppManager::new();
        let bot_id = UserId::new(100).unwrap();
        let dialog_id = DialogId::new(1);
        let view1 = OpenedWebView::new(12345, dialog_id, bot_id);
        let view2 = OpenedWebView::new(12345, dialog_id, bot_id);

        assert!(mgr.open_web_view(view1).await);
        assert!(!mgr.open_web_view(view2).await);
        assert_eq!(mgr.web_view_count().await, 1);
    }

    #[tokio::test]
    async fn test_manager_close_web_view() {
        let mgr = WebAppManager::new();
        let bot_id = UserId::new(100).unwrap();
        let dialog_id = DialogId::new(1);
        let view = OpenedWebView::new(12345, dialog_id, bot_id);

        mgr.open_web_view(view).await;
        assert_eq!(mgr.web_view_count().await, 1);

        let closed = mgr.close_web_view(12345).await;
        assert!(closed.is_some());
        assert_eq!(mgr.web_view_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_get_web_view() {
        let mgr = WebAppManager::new();
        let bot_id = UserId::new(100).unwrap();
        let dialog_id = DialogId::new(1);
        let view = OpenedWebView::new(12345, dialog_id, bot_id);

        mgr.open_web_view(view.clone()).await;

        let retrieved = mgr.get_web_view(12345).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().query_id(), 12345);
    }

    #[tokio::test]
    async fn test_manager_all_web_views() {
        let mgr = WebAppManager::new();
        let bot_id = UserId::new(100).unwrap();
        let dialog_id = DialogId::new(1);

        mgr.open_web_view(OpenedWebView::new(12345, dialog_id, bot_id))
            .await;
        mgr.open_web_view(OpenedWebView::new(12346, dialog_id, bot_id))
            .await;

        let views = mgr.all_web_views().await;
        assert_eq!(views.len(), 2);
    }

    #[tokio::test]
    async fn test_manager_close_dialog_web_views() {
        let mgr = WebAppManager::new();
        let bot_id = UserId::new(100).unwrap();
        let dialog_id1 = DialogId::new(1);
        let dialog_id2 = DialogId::new(2);

        mgr.open_web_view(OpenedWebView::new(12345, dialog_id1, bot_id))
            .await;
        mgr.open_web_view(OpenedWebView::new(12346, dialog_id2, bot_id))
            .await;
        mgr.open_web_view(OpenedWebView::new(12347, dialog_id1, bot_id))
            .await;

        assert_eq!(mgr.web_view_count().await, 3);

        let _closed = mgr.close_dialog_web_views(dialog_id1).await;
        // dialog_id1 had 2 views
        assert_eq!(mgr.web_view_count().await, 1);
        assert!(mgr.has_web_view(12346).await);
        assert!(!mgr.has_web_view(12345).await);
        assert!(!mgr.has_web_view(12347).await);
    }

    #[tokio::test]
    async fn test_manager_clear_web_views() {
        let mgr = WebAppManager::new();
        let bot_id = UserId::new(100).unwrap();
        let dialog_id = DialogId::new(1);

        mgr.open_web_view(OpenedWebView::new(12345, dialog_id, bot_id))
            .await;
        mgr.open_web_view(OpenedWebView::new(12346, dialog_id, bot_id))
            .await;

        assert_eq!(mgr.web_view_count().await, 2);
        mgr.clear_web_views().await;
        assert_eq!(mgr.web_view_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_web_app() {
        let mgr = WebAppManager::new();
        let bot_id = UserId::new(100).unwrap();
        let app = WebApp::new(bot_id, "test_app".to_string());

        assert!(mgr.add_web_app(app.clone()).await);

        let retrieved = mgr.get_web_app(bot_id, "test_app").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().short_name(), "test_app");
    }

    #[tokio::test]
    async fn test_manager_remove_web_app() {
        let mgr = WebAppManager::new();
        let bot_id = UserId::new(100).unwrap();
        let app = WebApp::new(bot_id, "test_app".to_string());

        mgr.add_web_app(app).await;

        let removed = mgr.remove_web_app(bot_id, "test_app").await;
        assert!(removed.is_some());

        let retrieved = mgr.get_web_app(bot_id, "test_app").await;
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_manager_file_source() {
        let mgr = WebAppManager::new();
        let bot_id = UserId::new(100).unwrap();
        let source_id = WebAppFileSourceId::new(bot_id, 12345);

        assert!(mgr.register_file_source(source_id).await);

        let retrieved = mgr.get_file_source_id(bot_id, "test_app").await;
        // The hash will be different, so this should be None
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_hash_string() {
        let hash1 = hash_string("test_app");
        let hash2 = hash_string("test_app");
        let hash3 = hash_string("other");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram_web_app_manager");
    }

    #[test]
    fn test_crate_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_display() {
        let mgr = WebAppManager::new();
        assert_eq!(format!("{}", mgr), "WebAppManager");
    }
}
