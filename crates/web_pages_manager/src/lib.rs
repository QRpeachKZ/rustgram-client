// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Web Pages Manager
//!
//! Web pages (link previews) manager for Telegram MTProto client.
//!
//! ## Overview
//!
//! This module provides functionality for managing web pages and link previews.
//! It handles web page caching, instant view management, and URL-to-webpage mapping.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `WebPagesManager` class from
//! `td/telegram/WebPagesManager.h`.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_web_pages_manager::{WebPagesManager, WebPage};
//! use rustgram_web_page_id::WebPageId;
//!
//! #[tokio::main]
//! async fn main() {
//!     let manager = WebPagesManager::new();
//!     let page = WebPage::new(
//!         WebPageId::new(1),
//!         "https://example.com".to_string()
//!     );
//!     manager.add_page(page).await;
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

use rustgram_file_id::FileId;
use rustgram_user_id::UserId;
use rustgram_web_page_id::WebPageId;

/// Stub for ChannelId.
/// TODO: Replace with proper rustgram-channel-id crate when available.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ChannelId(i64);

impl ChannelId {
    /// Creates a new ChannelId.
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the raw ID value.
    #[must_use]
    pub const fn get(&self) -> i64 {
        self.0
    }
}

/// Web page instant view.
///
/// Represents an instant view of a web page (AMP/Instant Article).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WebPageInstantView {
    content: String,
    view_count: i32,
    hash: i32,
    is_full: bool,
}

impl Default for WebPageInstantView {
    fn default() -> Self {
        Self::new()
    }
}

impl WebPageInstantView {
    /// Creates a new empty instant view.
    #[must_use]
    pub fn new() -> Self {
        Self {
            content: String::new(),
            view_count: 0,
            hash: 0,
            is_full: false,
        }
    }

    /// Returns the instant view content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Returns the view count.
    #[must_use]
    pub const fn view_count(&self) -> i32 {
        self.view_count
    }

    /// Returns the hash.
    #[must_use]
    pub const fn hash(&self) -> i32 {
        self.hash
    }

    /// Returns whether this is a full instant view.
    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.is_full
    }

    /// Sets the instant view content.
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    /// Sets the view count.
    pub fn set_view_count(&mut self, count: i32) {
        self.view_count = count;
    }

    /// Sets the hash.
    pub fn set_hash(&mut self, hash: i32) {
        self.hash = hash;
    }

    /// Sets whether this is a full instant view.
    pub fn set_full(&mut self, full: bool) {
        self.is_full = full;
    }

    /// Returns whether the instant view has content.
    #[must_use]
    pub fn has_content(&self) -> bool {
        !self.content.is_empty()
    }
}

/// Web page information.
///
/// Represents a web page with link preview information.
///
/// ## TDLib Mapping
///
/// Corresponds to TDLib `WebPagesManager::WebPage` class.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WebPage {
    id: WebPageId,
    url: String,
    display_url: String,
    title: String,
    description: String,
    photo: FileId,
    instant_view: Option<WebPageInstantView>,
}

impl WebPage {
    /// Creates a new web page.
    ///
    /// # Arguments
    ///
    /// * `id` - Web page ID
    /// * `url` - Page URL
    #[must_use]
    pub fn new(id: WebPageId, url: String) -> Self {
        Self {
            id,
            url,
            display_url: String::new(),
            title: String::new(),
            description: String::new(),
            photo: FileId::new(0, 0),
            instant_view: None,
        }
    }

    /// Returns the web page ID.
    #[must_use]
    pub const fn id(&self) -> WebPageId {
        self.id
    }

    /// Returns the URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the display URL.
    #[must_use]
    pub fn display_url(&self) -> &str {
        &self.display_url
    }

    /// Returns the title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the photo file ID.
    #[must_use]
    pub const fn photo(&self) -> FileId {
        self.photo
    }

    /// Returns the instant view.
    #[must_use]
    pub const fn instant_view(&self) -> Option<&WebPageInstantView> {
        self.instant_view.as_ref()
    }

    /// Sets the display URL.
    pub fn set_display_url(&mut self, url: String) {
        self.display_url = url;
    }

    /// Sets the title.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Sets the description.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Sets the photo file ID.
    pub fn set_photo(&mut self, photo: FileId) {
        self.photo = photo;
    }

    /// Sets or replaces the instant view.
    pub fn set_instant_view(&mut self, view: Option<WebPageInstantView>) {
        self.instant_view = view;
    }

    /// Returns whether this web page has an instant view.
    #[must_use]
    pub fn has_instant_view(&self) -> bool {
        self.instant_view.as_ref().is_some_and(|v| v.has_content())
    }

    /// Returns whether this web page has a photo.
    #[must_use]
    pub fn has_photo(&self) -> bool {
        !self.photo.is_empty()
    }
}

/// Web pages manager.
///
/// Provides storage and retrieval of web pages.
/// Thread-safe when `async` feature is enabled.
///
/// ## TDLib Mapping
///
/// Corresponds to TDLib `WebPagesManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_web_pages_manager::WebPagesManager;
///
/// # #[tokio::main]
/// # async fn main() {
/// let manager = WebPagesManager::new();
/// assert_eq!(manager.page_count().await, 0);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct WebPagesManager {
    pages: Arc<RwLock<HashMap<WebPageId, WebPage>>>,
    url_to_id: Arc<RwLock<HashMap<String, WebPageId>>>,
}

impl Default for WebPagesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WebPagesManager {
    /// Creates a new web pages manager.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            pages: Arc::new(RwLock::new(HashMap::new())),
            url_to_id: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Adds a web page to the manager.
    ///
    /// Returns `true` if the page was added (didn't previously exist),
    /// `false` if a page with this ID already existed.
    pub async fn add_page(&self, page: WebPage) -> bool {
        let id = page.id();
        let url = page.url().to_string();

        let mut pages = self.pages.write().await;
        let mut url_to_id = self.url_to_id.write().await;

        if pages.contains_key(&id) {
            return false;
        }

        pages.insert(id, page);
        url_to_id.insert(url, id);
        true
    }

    /// Gets a web page by ID.
    pub async fn get_page(&self, id: WebPageId) -> Option<WebPage> {
        let pages = self.pages.read().await;
        pages.get(&id).cloned()
    }

    /// Gets a web page by URL.
    pub async fn get_page_by_url(&self, url: &str) -> Option<WebPage> {
        let url_to_id = self.url_to_id.read().await;
        if let Some(id) = url_to_id.get(url) {
            let pages = self.pages.read().await;
            pages.get(id).cloned()
        } else {
            None
        }
    }

    /// Removes a web page.
    pub async fn remove_page(&self, id: WebPageId) -> Option<WebPage> {
        let mut pages = self.pages.write().await;
        let page = pages.remove(&id);

        if let Some(ref p) = page {
            let mut url_to_id = self.url_to_id.write().await;
            url_to_id.remove(p.url());
        }

        page
    }

    /// Returns whether a web page exists.
    pub async fn has_page(&self, id: WebPageId) -> bool {
        let pages = self.pages.read().await;
        pages.contains_key(&id)
    }

    /// Returns whether a web page exists for the given URL.
    pub async fn has_page_with_url(&self, url: &str) -> bool {
        let url_to_id = self.url_to_id.read().await;
        url_to_id.contains_key(url)
    }

    /// Returns the number of web pages stored.
    pub async fn page_count(&self) -> usize {
        let pages = self.pages.read().await;
        pages.len()
    }

    /// Gets the web page ID for a URL.
    pub async fn get_web_page_id_by_url(&self, url: &str) -> Option<WebPageId> {
        let url_to_id = self.url_to_id.read().await;
        url_to_id.get(url).copied()
    }

    /// Gets the web page instant view.
    pub async fn get_instant_view(&self, id: WebPageId) -> Option<WebPageInstantView> {
        self.get_page(id)
            .await
            .and_then(|p| p.instant_view().cloned())
    }

    /// Gets the web page URL by ID.
    pub async fn get_web_page_url(&self, id: WebPageId) -> Option<String> {
        self.get_page(id).await.map(|p| p.url().to_string())
    }

    /// Gets the web page search text (title + description).
    pub async fn get_web_page_search_text(&self, id: WebPageId) -> Option<String> {
        self.get_page(id).await.map(|p| {
            let mut text = p.title().to_string();
            if !p.description().is_empty() {
                if !text.is_empty() {
                    text.push(' ');
                }
                text.push_str(p.description());
            }
            text
        })
    }

    /// Gets the media duration for a web page.
    pub async fn get_web_page_media_duration(&self, id: WebPageId) -> Option<i32> {
        // Web pages don't typically have media duration
        // This is a placeholder for compatibility
        self.get_page(id).await?;
        Some(0)
    }

    /// Gets user IDs mentioned in a web page.
    pub async fn get_web_page_user_ids(&self, id: WebPageId) -> Vec<UserId> {
        // Placeholder for extracting mentioned users
        // In a full implementation, this would parse the page content
        let _ = id;
        Vec::new()
    }

    /// Gets channel IDs mentioned in a web page.
    pub async fn get_web_page_channel_ids(&self, id: WebPageId) -> Vec<ChannelId> {
        // Placeholder for extracting mentioned channels
        // In a full implementation, this would parse the page content
        let _ = id;
        Vec::new()
    }

    /// Clears all web pages.
    pub async fn clear(&self) {
        let mut pages = self.pages.write().await;
        let mut url_to_id = self.url_to_id.write().await;
        pages.clear();
        url_to_id.clear();
    }

    /// Returns all web page IDs.
    pub async fn all_page_ids(&self) -> Vec<WebPageId> {
        let pages = self.pages.read().await;
        pages.keys().copied().collect()
    }
}

impl fmt::Display for WebPagesManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WebPagesManager")
    }
}

/// Crate version from Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name for identification.
pub const CRATE_NAME: &str = "rustgram_web_pages_manager";

#[cfg(test)]
mod tests {
    use super::*;

    // WebPageInstantView tests
    #[test]
    fn test_instant_view_new() {
        let view = WebPageInstantView::new();
        assert!(!view.has_content());
        assert_eq!(view.view_count(), 0);
        assert!(!view.is_full());
    }

    #[test]
    fn test_instant_view_with_content() {
        let mut view = WebPageInstantView::new();
        view.set_content("<p>Hello</p>".to_string());
        view.set_view_count(100);
        view.set_full(true);

        assert!(view.has_content());
        assert_eq!(view.content(), "<p>Hello</p>");
        assert_eq!(view.view_count(), 100);
        assert!(view.is_full());
    }

    // WebPage tests
    #[test]
    fn test_web_page_new() {
        let page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        assert_eq!(page.id(), WebPageId::new(1));
        assert_eq!(page.url(), "https://example.com");
        assert!(!page.has_instant_view());
        assert!(!page.has_photo());
    }

    #[test]
    fn test_web_page_with_instant_view() {
        let mut page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        let mut view = WebPageInstantView::new();
        view.set_content("<p>Content</p>".to_string());
        page.set_instant_view(Some(view));

        assert!(page.has_instant_view());
        assert!(page.instant_view().unwrap().has_content());
    }

    #[test]
    fn test_web_page_with_title() {
        let mut page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        page.set_title("Example Site".to_string());
        page.set_description("An example website".to_string());

        assert_eq!(page.title(), "Example Site");
        assert_eq!(page.description(), "An example website");
    }

    #[test]
    fn test_web_page_with_photo() {
        let mut page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        page.set_photo(FileId::new(1, 0));

        assert!(page.has_photo());
    }

    // Manager tests
    #[tokio::test]
    async fn test_manager_new() {
        let mgr = WebPagesManager::new();
        assert_eq!(mgr.page_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_add_page() {
        let mgr = WebPagesManager::new();
        let page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());

        assert!(mgr.add_page(page.clone()).await);
        assert_eq!(mgr.page_count().await, 1);
        assert!(mgr.has_page(WebPageId::new(1)).await);
        assert!(mgr.has_page_with_url("https://example.com").await);
    }

    #[tokio::test]
    async fn test_manager_add_duplicate() {
        let mgr = WebPagesManager::new();
        let page1 = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        let page2 = WebPage::new(WebPageId::new(1), "https://example.com".to_string());

        assert!(mgr.add_page(page1).await);
        assert!(!mgr.add_page(page2).await);
        assert_eq!(mgr.page_count().await, 1);
    }

    #[tokio::test]
    async fn test_manager_get_page() {
        let mgr = WebPagesManager::new();
        let page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        mgr.add_page(page).await;

        let retrieved = mgr.get_page(WebPageId::new(1)).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().url(), "https://example.com");
    }

    #[tokio::test]
    async fn test_manager_get_page_by_url() {
        let mgr = WebPagesManager::new();
        let page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        mgr.add_page(page).await;

        let retrieved = mgr.get_page_by_url("https://example.com").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), WebPageId::new(1));
    }

    #[tokio::test]
    async fn test_manager_remove_page() {
        let mgr = WebPagesManager::new();
        let page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        mgr.add_page(page).await;

        assert_eq!(mgr.page_count().await, 1);
        let removed = mgr.remove_page(WebPageId::new(1)).await;
        assert!(removed.is_some());
        assert_eq!(mgr.page_count().await, 0);
        assert!(!mgr.has_page_with_url("https://example.com").await);
    }

    #[tokio::test]
    async fn test_manager_get_web_page_id_by_url() {
        let mgr = WebPagesManager::new();
        let page = WebPage::new(WebPageId::new(123), "https://example.com".to_string());
        mgr.add_page(page).await;

        let id = mgr.get_web_page_id_by_url("https://example.com").await;
        assert_eq!(id, Some(WebPageId::new(123)));
    }

    #[tokio::test]
    async fn test_manager_get_web_page_url() {
        let mgr = WebPagesManager::new();
        let page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        mgr.add_page(page).await;

        let url = mgr.get_web_page_url(WebPageId::new(1)).await;
        assert_eq!(url, Some("https://example.com".to_string()));
    }

    #[tokio::test]
    async fn test_manager_get_instant_view() {
        let mgr = WebPagesManager::new();
        let mut page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        let mut view = WebPageInstantView::new();
        view.set_content("<p>Content</p>".to_string());
        page.set_instant_view(Some(view));
        mgr.add_page(page).await;

        let instant_view = mgr.get_instant_view(WebPageId::new(1)).await;
        assert!(instant_view.is_some());
        assert!(instant_view.unwrap().has_content());
    }

    #[tokio::test]
    async fn test_manager_get_search_text() {
        let mgr = WebPagesManager::new();
        let mut page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        page.set_title("Example".to_string());
        page.set_description("Description".to_string());
        mgr.add_page(page).await;

        let text = mgr.get_web_page_search_text(WebPageId::new(1)).await;
        assert_eq!(text, Some("Example Description".to_string()));
    }

    #[tokio::test]
    async fn test_manager_get_media_duration() {
        let mgr = WebPagesManager::new();
        let page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        mgr.add_page(page).await;

        let duration = mgr.get_web_page_media_duration(WebPageId::new(1)).await;
        assert_eq!(duration, Some(0));
    }

    #[tokio::test]
    async fn test_manager_get_user_ids() {
        let mgr = WebPagesManager::new();
        let page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        mgr.add_page(page).await;

        let ids = mgr.get_web_page_user_ids(WebPageId::new(1)).await;
        assert!(ids.is_empty());
    }

    #[tokio::test]
    async fn test_manager_get_channel_ids() {
        let mgr = WebPagesManager::new();
        let page = WebPage::new(WebPageId::new(1), "https://example.com".to_string());
        mgr.add_page(page).await;

        let ids = mgr.get_web_page_channel_ids(WebPageId::new(1)).await;
        assert!(ids.is_empty());
    }

    #[tokio::test]
    async fn test_manager_clear() {
        let mgr = WebPagesManager::new();
        mgr.add_page(WebPage::new(
            WebPageId::new(1),
            "https://example.com".to_string(),
        ))
        .await;
        mgr.add_page(WebPage::new(
            WebPageId::new(2),
            "https://example.org".to_string(),
        ))
        .await;

        assert_eq!(mgr.page_count().await, 2);
        mgr.clear().await;
        assert_eq!(mgr.page_count().await, 0);
    }

    #[tokio::test]
    async fn test_manager_all_page_ids() {
        let mgr = WebPagesManager::new();
        mgr.add_page(WebPage::new(
            WebPageId::new(1),
            "https://example.com".to_string(),
        ))
        .await;
        mgr.add_page(WebPage::new(
            WebPageId::new(2),
            "https://example.org".to_string(),
        ))
        .await;

        let ids = mgr.all_page_ids().await;
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram_web_pages_manager");
    }

    #[test]
    fn test_crate_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_display() {
        let mgr = WebPagesManager::new();
        assert_eq!(format!("{}", mgr), "WebPagesManager");
    }
}
