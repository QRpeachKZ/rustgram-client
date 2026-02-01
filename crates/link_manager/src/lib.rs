// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Link Manager
//!
//! Manages links and previews for Telegram clients.
//!
//! Based on TDLib's `LinkManager` from `td/telegram/LinkManager.h`.
//!
//! ## Overview
//!
//! The link manager handles:
//! - Validation and canonicalization of URLs
//! - Parsing internal Telegram links (t.me, tg://)
//! - Managing deep links
//! - Link preview generation
//! - Autologin token management
//! - Dialog invite link parsing
//!
//! ## Architecture
//!
//! The manager maintains:
//! - Autologin token for external links
//! - Autologin domains whitelist
//! - URL authentication domains
//! - Recent t.me URLs cache
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_link_manager::LinkManager;
//!
//! // Check if a link is valid
//! let link = "https://t.me/example";
//! assert!(LinkManager::check_link(link).is_ok());
//!
//! // Check if it's an internal link
//! assert!(LinkManager::is_internal_link(link));
//! ```
//!
//! ```rust
//! use rustgram_link_manager::LinkManager;
//!
//! // Parse an internal link
//! let link = "https://t.me/telegram";
//! if let Some(internal_link) = LinkManager::parse_internal_link(link) {
//!     println!("Internal link type: {:?}", internal_link.type_name());
//! }
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod error;

use crate::error::{Error, Result};
use rustgram_types::UserId;
use url::Url;

/// Maximum length for a URL.
const MAX_URL_LENGTH: usize = 65536;

/// Default t.me URL.
const T_ME_URL: &str = "https://t.me/";

/// Internal link type.
///
/// TDLib reference: `td_api::InternalLinkType`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InternalLinkType {
    /// Link to active sessions.
    ActiveSessions,
    /// Link to attach menu bot.
    AttachMenuBot {
        /// Bot username.
        username: String,
        /// Target chat.
        target_chat: Option<String>,
    },
    /// Link to authentication code.
    AuthenticationCode {
        /// The code.
        code: String,
        /// Phone number.
        phone_number: String,
    },
    /// Link to background.
    Background {
        /// Background name.
        name: String,
    },
    /// Link to bot start.
    BotStart {
        /// Bot username.
        username: String,
        /// Start parameter.
        start_parameter: String,
    },
    /// Link to bot start in group.
    BotStartInGroup {
        /// Bot username.
        username: String,
        /// Start parameter.
        start_parameter: String,
    },
    /// Link to business chat.
    BusinessChat {
        /// Business username.
        username: String,
    },
    /// Link to dialog invite.
    DialogInvite {
        /// Invite hash.
        invite_hash: String,
    },
    /// Link to public dialog.
    PublicDialog {
        /// Dialog username.
        username: String,
        /// Message text.
        message_text: Option<String>,
    },
    /// Link to settings.
    Settings,
    /// Link to unknown deep link.
    UnknownDeepLink {
        /// The link.
        link: String,
    },
    /// Unsupported link type.
    Unsupported,
}

impl InternalLinkType {
    /// Returns the type name for this link.
    #[must_use]
    pub const fn type_name(&self) -> &str {
        match self {
            Self::ActiveSessions => "activeSessions",
            Self::AttachMenuBot { .. } => "attachMenuBot",
            Self::AuthenticationCode { .. } => "authenticationCode",
            Self::Background { .. } => "background",
            Self::BotStart { .. } => "botStart",
            Self::BotStartInGroup { .. } => "botStartInGroup",
            Self::BusinessChat { .. } => "businessChat",
            Self::DialogInvite { .. } => "dialogInvite",
            Self::PublicDialog { .. } => "publicDialog",
            Self::Settings => "settings",
            Self::UnknownDeepLink { .. } => "unknownDeepLink",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Link type classification.
///
/// TDLib reference: `LinkManager.h:189`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkType {
    /// External link.
    External,
    /// t.me link.
    TMe,
    /// tg:// link.
    Tg,
    /// Telegraph link.
    Telegraph,
}

/// Information about a parsed link.
///
/// TDLib reference: `LinkManager.h:191-195`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkInfo {
    /// The link type.
    pub type_: LinkType,
    /// The query string.
    pub query: String,
}

impl LinkInfo {
    /// Creates new link info.
    ///
    /// # Arguments
    ///
    /// * `type_` - The link type
    /// * `query` - The query string
    #[must_use]
    pub const fn new(type_: LinkType, query: String) -> Self {
        Self { type_, query }
    }

    /// Creates external link info.
    #[must_use]
    pub const fn external(query: String) -> Self {
        Self {
            type_: LinkType::External,
            query,
        }
    }

    /// Creates t.me link info.
    #[must_use]
    pub const fn t_me(query: String) -> Self {
        Self {
            type_: LinkType::TMe,
            query,
        }
    }

    /// Creates tg:// link info.
    #[must_use]
    pub const fn tg(query: String) -> Self {
        Self {
            type_: LinkType::Tg,
            query,
        }
    }

    /// Creates telegraph link info.
    #[must_use]
    pub const fn telegraph(query: String) -> Self {
        Self {
            type_: LinkType::Telegraph,
            query,
        }
    }
}

/// Deep link information.
///
/// TDLib reference: `td_api::deepLinkInfo`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepLinkInfo {
    /// The deep link.
    pub link: String,
    /// The description.
    pub description: Option<String>,
}

impl DeepLinkInfo {
    /// Creates a new deep link info.
    ///
    /// # Arguments
    ///
    /// * `link` - The deep link
    #[must_use]
    pub fn new(link: String) -> Self {
        Self {
            link,
            description: None,
        }
    }

    /// Sets the description.
    #[must_use]
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

/// Login URL information.
///
/// TDLib reference: `td_api::LoginUrlInfo`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginUrlInfo {
    /// The login URL can be opened immediately.
    Open {
        /// The URL to open.
        url: String,
        /// Whether to skip confirmation.
        skip_confirmation: bool,
    },
    /// The login URL needs to be confirmed.
    RequestConfirmation {
        /// The URL to open.
        url: String,
        /// The domain requesting confirmation.
        domain: String,
        /// Whether to write access.
        request_write_access: bool,
    },
}

impl LoginUrlInfo {
    /// Creates an open login URL info.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL
    #[must_use]
    pub fn open(url: String) -> Self {
        Self::Open {
            url,
            skip_confirmation: false,
        }
    }

    /// Creates a request confirmation login URL info.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL
    /// * `domain` - The domain
    #[must_use]
    pub fn request_confirmation(url: String, domain: String) -> Self {
        Self::RequestConfirmation {
            url,
            domain,
            request_write_access: false,
        }
    }
}

/// Message link information.
///
/// TDLib reference: `td_api::messageLinkInfo`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageLinkInfo {
    /// The dialog username.
    pub username: Option<String>,
    /// The dialog invite link hash.
    pub invite_hash: Option<String>,
    /// The message thread ID.
    pub thread_id: i64,
    /// The message ID.
    pub message_id: i64,
}

impl MessageLinkInfo {
    /// Creates a new message link info.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The message ID
    #[must_use]
    pub const fn new(message_id: i64) -> Self {
        Self {
            username: None,
            invite_hash: None,
            thread_id: 0,
            message_id,
        }
    }

    /// Sets the username.
    #[must_use]
    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    /// Sets the invite hash.
    #[must_use]
    pub fn with_invite_hash(mut self, invite_hash: String) -> Self {
        self.invite_hash = Some(invite_hash);
        self
    }
}

/// Manages links and previews.
///
/// This manager handles URL validation, internal link parsing,
/// and link preview generation.
///
/// TDLib reference: `td::LinkManager` from `LinkManager.h`
#[derive(Debug, Clone)]
pub struct LinkManager {
    /// Autologin token.
    autologin_token: Option<String>,
    /// Autologin domains.
    autologin_domains: Vec<String>,
    /// URL authentication domains.
    url_auth_domains: Vec<String>,
    /// Whitelisted domains.
    whitelisted_domains: Vec<String>,
}

impl LinkManager {
    /// Creates a new link manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_link_manager::LinkManager;
    ///
    /// let manager = LinkManager::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            autologin_token: None,
            autologin_domains: Vec::new(),
            url_auth_domains: Vec::new(),
            whitelisted_domains: Vec::new(),
        }
    }

    /// Checks if a link is valid.
    ///
    /// # Arguments
    ///
    /// * `link` - The link to check
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The link is too long
    /// - The link is not a valid URL
    ///
    /// TDLib reference: `LinkManager.h:53`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_link_manager::LinkManager;
    ///
    /// assert!(LinkManager::check_link("https://t.me/example").is_ok());
    /// assert!(LinkManager::check_link("https://example.com").is_ok());
    /// ```
    pub fn check_link(link: &str) -> Result<String> {
        if link.len() > MAX_URL_LENGTH {
            return Err(Error::LinkTooLong(link.len()));
        }

        let trimmed = link.trim();
        if trimmed.is_empty() {
            return Err(Error::EmptyLink);
        }

        // Try to parse as URL
        let url = Url::parse(trimmed).map_err(|_| Error::InvalidLink(trimmed.to_string()))?;

        // Ensure it has a scheme
        let canonical = if url.scheme().is_empty() {
            format!("https://{}", trimmed)
        } else {
            trimmed.to_string()
        };

        Ok(canonical)
    }

    /// Gets a checked link or empty string on error.
    ///
    /// # Arguments
    ///
    /// * `link` - The link to check
    ///
    /// Returns the canonical link or empty string if invalid.
    ///
    /// TDLib reference: `LinkManager.h:56`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_link_manager::LinkManager;
    ///
    /// assert!(!LinkManager::get_checked_link("https://t.me/example", false, false).is_empty());
    /// assert!(LinkManager::get_checked_link("not a url", false, false).is_empty());
    /// ```
    pub fn get_checked_link(link: &str, http_only: bool, https_only: bool) -> String {
        match Self::check_link(link) {
            Ok(canonical) => {
                if http_only && !canonical.starts_with("http://") {
                    return String::new();
                }
                if https_only && !canonical.starts_with("https://") {
                    return String::new();
                }
                canonical
            }
            Err(_) => String::new(),
        }
    }

    /// Checks if a link is an internal link.
    ///
    /// # Arguments
    ///
    /// * `link` - The link to check
    ///
    /// Returns true if this is a Telegram internal link.
    ///
    /// TDLib reference: `LinkManager.h:59`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_link_manager::LinkManager;
    ///
    /// assert!(LinkManager::is_internal_link("https://t.me/example"));
    /// assert!(LinkManager::is_internal_link("tg://resolve?domain=example"));
    /// assert!(!LinkManager::is_internal_link("https://google.com"));
    /// ```
    pub fn is_internal_link(link: &str) -> bool {
        let lower = link.to_lowercase();
        lower.starts_with("tg://")
            || lower.starts_with("https://t.me/")
            || lower.starts_with("http://t.me/")
            || lower.starts_with("https://telegram.me/")
            || lower.starts_with("http://telegram.me/")
            || lower.starts_with("https://telegram.dog/")
            || lower.starts_with("http://telegram.dog/")
    }

    /// Parses an internal link.
    ///
    /// # Arguments
    ///
    /// * `link` - The link to parse
    ///
    /// Returns the internal link type if parsing succeeds.
    ///
    /// TDLib reference: `LinkManager.h:62`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_link_manager::LinkManager;
    ///
    /// let link = "https://t.me/telegram";
    /// if let Some(internal) = LinkManager::parse_internal_link(link) {
    ///     println!("Parsed: {:?}", internal.type_name());
    /// }
    /// ```
    pub fn parse_internal_link(link: &str) -> Option<InternalLinkType> {
        if !Self::is_internal_link(link) {
            return None;
        }

        let lower = link.to_lowercase();

        // tg:// links
        if lower.starts_with("tg://") {
            return Self::parse_tg_link(link);
        }

        // t.me links
        if lower.starts_with("https://t.me/") || lower.starts_with("http://t.me/") {
            return Self::parse_t_me_link(link);
        }

        None
    }

    /// Gets the t.me URL.
    ///
    /// TDLib reference: `LinkManager.h:117`
    #[must_use]
    pub const fn get_t_me_url() -> &'static str {
        T_ME_URL
    }

    /// Extracts user ID from a link.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to parse
    ///
    /// Returns the user ID if found.
    ///
    /// TDLib reference: `LinkManager.h:115`
    pub fn get_link_user_id(url: &str) -> Option<UserId> {
        // Try to extract user ID from various link formats
        if url.contains("/u/") {
            let parts: Vec<&str> = url.split('/').collect();
            if let Some(user_str) = parts
                .iter()
                .position(|&p| p == "u")
                .and_then(|i| parts.get(i + 1))
            {
                if let Ok(id) = user_str.parse::<i64>() {
                    return UserId::new(id).ok();
                }
            }
        }
        None
    }

    /// Gets dialog invite link hash.
    ///
    /// # Arguments
    ///
    /// * `invite_link` - The invite link
    ///
    /// Returns the invite hash if found.
    ///
    /// TDLib reference: `LinkManager.h:97`
    pub fn get_dialog_invite_link_hash(invite_link: &str) -> String {
        // Extract hash from invite link
        // Format: https://t.me/+<hash>
        if let Some(hash_start) = invite_link.find("/+") {
            if let Some(hash_end) = invite_link[hash_start + 2..].find(&['?', '#'][..]) {
                return invite_link[hash_start + 2..hash_start + 2 + hash_end].to_string();
            }
            return invite_link[hash_start + 2..].to_string();
        }
        String::new()
    }

    /// Gets a dialog invite link.
    ///
    /// # Arguments
    ///
    /// * `invite_hash` - The invite hash
    /// * `is_internal` - Whether to use internal format
    ///
    /// TDLib reference: `LinkManager.h:99`
    #[must_use]
    pub fn get_dialog_invite_link(invite_hash: &str, is_internal: bool) -> String {
        if is_internal {
            format!("tg://join?invite={}", invite_hash)
        } else {
            format!("https://t.me/+{}", invite_hash)
        }
    }

    /// Gets public dialog link.
    ///
    /// # Arguments
    ///
    /// * `username` - The dialog username
    /// * `draft_text` - Optional draft message text
    /// * `open_profile` - Whether to open profile
    /// * `is_internal` - Whether to use internal format
    ///
    /// TDLib reference: `LinkManager.h:111`
    #[must_use]
    pub fn get_public_dialog_link(
        username: &str,
        draft_text: Option<&str>,
        open_profile: bool,
        is_internal: bool,
    ) -> String {
        if is_internal {
            let mut link = format!("tg://resolve?domain={}", username);
            if open_profile {
                link.push_str("&post=1");
            }
            if let Some(text) = draft_text {
                link.push_str(&format!("&text={}", urlencoding::encode(text)));
            }
            link
        } else {
            format!("https://t.me/{}", username)
        }
    }

    /// Parses a tg:// link.
    fn parse_tg_link(link: &str) -> Option<InternalLinkType> {
        // Extract the path part (after tg://)
        let path_start = link.find("tg://")?;
        let rest = &link[path_start + 5..];

        // Split path and query
        let (path, query) = if let Some(query_start) = rest.find('?') {
            (&rest[..query_start], Some(&rest[query_start + 1..]))
        } else {
            (rest, None)
        };

        // Parse query parameters
        let params: std::collections::HashMap<String, String> = query
            .unwrap_or("")
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                    _ => None,
                }
            })
            .collect();

        match path {
            "resolve" => {
                if let Some(domain) = params.get("domain") {
                    Some(InternalLinkType::PublicDialog {
                        username: domain.clone(),
                        message_text: params.get("text").cloned(),
                    })
                } else {
                    Some(InternalLinkType::Unsupported)
                }
            }
            "join" => {
                if let Some(invite) = params.get("invite") {
                    Some(InternalLinkType::DialogInvite {
                        invite_hash: invite.clone(),
                    })
                } else {
                    Some(InternalLinkType::Unsupported)
                }
            }
            "settings" => Some(InternalLinkType::Settings),
            _ => Some(InternalLinkType::UnknownDeepLink {
                link: link.to_string(),
            }),
        }
    }

    /// Parses a t.me link.
    fn parse_t_me_link(link: &str) -> Option<InternalLinkType> {
        // Parse https://t.me/* format
        let url = Url::parse(link).ok()?;
        let path = url.path();

        // Check for invite link first (starts with /+)
        if let Some(hash) = path.strip_prefix("/+") {
            return Some(InternalLinkType::DialogInvite {
                invite_hash: hash.to_string(),
            });
        }

        // Check for public dialog link
        if let Some(username) = path.strip_prefix("/") {
            if !username.is_empty() {
                return Some(InternalLinkType::PublicDialog {
                    username: username.to_string(),
                    message_text: None,
                });
            }
        }

        Some(InternalLinkType::UnknownDeepLink {
            link: link.to_string(),
        })
    }

    /// Updates the autologin token.
    ///
    /// # Arguments
    ///
    /// * `token` - The new autologin token
    ///
    /// TDLib reference: `LinkManager.h:66`
    pub fn update_autologin_token(&mut self, token: String) {
        self.autologin_token = Some(token);
    }

    /// Updates autologin domains.
    ///
    /// # Arguments
    ///
    /// * `autologin_domains` - Autologin domains
    /// * `url_auth_domains` - URL auth domains
    /// * `whitelisted_domains` - Whitelisted domains
    ///
    /// TDLib reference: `LinkManager.h:68-69`
    pub fn update_autologin_domains(
        &mut self,
        autologin_domains: Vec<String>,
        url_auth_domains: Vec<String>,
        whitelisted_domains: Vec<String>,
    ) {
        self.autologin_domains = autologin_domains;
        self.url_auth_domains = url_auth_domains;
        self.whitelisted_domains = whitelisted_domains;
    }
}

impl Default for LinkManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_manager() -> LinkManager {
        LinkManager::new()
    }

    #[test]
    fn test_manager_creation() {
        let _manager = create_test_manager();
        assert_eq!(LinkManager::get_t_me_url(), "https://t.me/");
    }

    #[test]
    fn test_default_manager() {
        let manager = LinkManager::default();
        assert!(manager.autologin_token.is_none());
    }

    #[test]
    fn test_check_link_valid() {
        assert!(LinkManager::check_link("https://t.me/example").is_ok());
        assert!(LinkManager::check_link("https://telegram.me/example").is_ok());
        assert!(LinkManager::check_link("tg://resolve?domain=example").is_ok());
    }

    #[test]
    fn test_check_link_invalid() {
        assert!(LinkManager::check_link("not a url").is_err());
        assert!(LinkManager::check_link("").is_err());
        assert!(LinkManager::check_link("   ").is_err());
    }

    #[test]
    fn test_get_checked_link_valid() {
        let result = LinkManager::get_checked_link("https://t.me/example", false, false);
        assert!(!result.is_empty());
        assert!(result.contains("t.me"));
    }

    #[test]
    fn test_get_checked_link_invalid() {
        let result = LinkManager::get_checked_link("not a url", false, false);
        assert!(result.is_empty());
    }

    #[test]
    fn test_get_checked_link_http_only() {
        let result = LinkManager::get_checked_link("https://t.me/example", true, false);
        assert!(result.is_empty());

        let result = LinkManager::get_checked_link("http://t.me/example", true, false);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_get_checked_link_https_only() {
        let result = LinkManager::get_checked_link("http://t.me/example", false, true);
        assert!(result.is_empty());

        let result = LinkManager::get_checked_link("https://t.me/example", false, true);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_is_internal_link_true() {
        assert!(LinkManager::is_internal_link("tg://resolve?domain=example"));
        assert!(LinkManager::is_internal_link("https://t.me/example"));
        assert!(LinkManager::is_internal_link("http://t.me/example"));
        assert!(LinkManager::is_internal_link("https://telegram.me/example"));
        assert!(LinkManager::is_internal_link(
            "https://telegram.dog/example"
        ));
    }

    #[test]
    fn test_is_internal_link_false() {
        assert!(!LinkManager::is_internal_link("https://google.com"));
        assert!(!LinkManager::is_internal_link("https://example.com"));
        assert!(!LinkManager::is_internal_link("not a url"));
    }

    #[test]
    fn test_parse_internal_link_tg_resolve() {
        let link = "tg://resolve?domain=telegram";
        let result = LinkManager::parse_internal_link(link);
        assert!(result.is_some());
        match result {
            Some(InternalLinkType::PublicDialog { username, .. }) => {
                assert_eq!(username, "telegram");
            }
            _ => panic!("Expected PublicDialog"),
        }
    }

    #[test]
    fn test_parse_internal_link_tg_join() {
        let link = "tg://join?invite=abc123";
        let result = LinkManager::parse_internal_link(link);
        assert!(result.is_some());
        match result {
            Some(InternalLinkType::DialogInvite { invite_hash }) => {
                assert_eq!(invite_hash, "abc123");
            }
            _ => panic!("Expected DialogInvite"),
        }
    }

    #[test]
    fn test_parse_internal_link_tg_settings() {
        let link = "tg://settings";
        let result = LinkManager::parse_internal_link(link);
        assert!(result.is_some());
        assert_eq!(result, Some(InternalLinkType::Settings));
    }

    #[test]
    fn test_parse_internal_link_t_me_public() {
        let link = "https://t.me/telegram";
        let result = LinkManager::parse_internal_link(link);
        assert!(result.is_some());
        match result {
            Some(InternalLinkType::PublicDialog { username, .. }) => {
                assert_eq!(username, "telegram");
            }
            _ => panic!("Expected PublicDialog"),
        }
    }

    #[test]
    fn test_parse_internal_link_t_me_invite() {
        let link = "https://t.me/+abc123";
        let result = LinkManager::parse_internal_link(link);
        assert!(result.is_some());
        match result {
            Some(InternalLinkType::DialogInvite { invite_hash }) => {
                assert_eq!(invite_hash, "abc123");
            }
            _ => panic!("Expected DialogInvite"),
        }
    }

    #[test]
    fn test_parse_internal_link_external() {
        let link = "https://google.com";
        let result = LinkManager::parse_internal_link(link);
        assert!(result.is_none());
    }

    #[test]
    fn test_internal_link_type_type_name() {
        assert_eq!(
            InternalLinkType::ActiveSessions.type_name(),
            "activeSessions"
        );
        assert_eq!(InternalLinkType::Settings.type_name(), "settings");
        assert_eq!(InternalLinkType::Unsupported.type_name(), "unsupported");
    }

    #[test]
    fn test_link_info_new() {
        let info = LinkInfo::new(LinkType::External, "test".to_string());
        assert_eq!(info.type_, LinkType::External);
        assert_eq!(info.query, "test");
    }

    #[test]
    fn test_link_info_constructors() {
        let external = LinkInfo::external("query".to_string());
        assert_eq!(external.type_, LinkType::External);

        let t_me = LinkInfo::t_me("query".to_string());
        assert_eq!(t_me.type_, LinkType::TMe);

        let tg = LinkInfo::tg("query".to_string());
        assert_eq!(tg.type_, LinkType::Tg);

        let telegraph = LinkInfo::telegraph("query".to_string());
        assert_eq!(telegraph.type_, LinkType::Telegraph);
    }

    #[test]
    fn test_deep_link_info_new() {
        let info = DeepLinkInfo::new("https://t.me/example".to_string());
        assert_eq!(info.link, "https://t.me/example");
        assert!(info.description.is_none());
    }

    #[test]
    fn test_deep_link_info_with_description() {
        let info =
            DeepLinkInfo::new("link".to_string()).with_description("description".to_string());
        assert_eq!(info.description.as_deref(), Some("description"));
    }

    #[test]
    fn test_login_url_info_open() {
        let info = LoginUrlInfo::open("https://example.com".to_string());
        match info {
            LoginUrlInfo::Open {
                url,
                skip_confirmation,
            } => {
                assert_eq!(url, "https://example.com");
                assert!(!skip_confirmation);
            }
            _ => panic!("Expected Open"),
        }
    }

    #[test]
    fn test_login_url_info_request_confirmation() {
        let info = LoginUrlInfo::request_confirmation(
            "https://example.com".to_string(),
            "example.com".to_string(),
        );
        match info {
            LoginUrlInfo::RequestConfirmation {
                url,
                domain,
                request_write_access,
            } => {
                assert_eq!(url, "https://example.com");
                assert_eq!(domain, "example.com");
                assert!(!request_write_access);
            }
            _ => panic!("Expected RequestConfirmation"),
        }
    }

    #[test]
    fn test_message_link_info_new() {
        let info = MessageLinkInfo::new(123);
        assert_eq!(info.message_id, 123);
        assert!(info.username.is_none());
        assert!(info.invite_hash.is_none());
    }

    #[test]
    fn test_message_link_info_with_username() {
        let info = MessageLinkInfo::new(123).with_username("telegram".to_string());
        assert_eq!(info.username.as_deref(), Some("telegram"));
    }

    #[test]
    fn test_message_link_info_with_invite_hash() {
        let info = MessageLinkInfo::new(123).with_invite_hash("hash".to_string());
        assert_eq!(info.invite_hash.as_deref(), Some("hash"));
    }

    #[test]
    fn test_get_t_me_url() {
        assert_eq!(LinkManager::get_t_me_url(), "https://t.me/");
    }

    #[test]
    fn test_get_dialog_invite_link_hash() {
        let hash = LinkManager::get_dialog_invite_link_hash("https://t.me/+abc123");
        assert_eq!(hash, "abc123");

        let hash = LinkManager::get_dialog_invite_link_hash("https://t.me/+abc123?start=foo");
        assert_eq!(hash, "abc123");
    }

    #[test]
    fn test_get_dialog_invite_link() {
        let link = LinkManager::get_dialog_invite_link("abc123", false);
        assert_eq!(link, "https://t.me/+abc123");

        let link = LinkManager::get_dialog_invite_link("abc123", true);
        assert_eq!(link, "tg://join?invite=abc123");
    }

    #[test]
    fn test_get_public_dialog_link() {
        let link = LinkManager::get_public_dialog_link("telegram", None, false, false);
        assert_eq!(link, "https://t.me/telegram");

        let link = LinkManager::get_public_dialog_link("telegram", None, false, true);
        assert_eq!(link, "tg://resolve?domain=telegram");

        let link = LinkManager::get_public_dialog_link("telegram", None, true, true);
        assert!(link.contains("post=1"));

        let link = LinkManager::get_public_dialog_link("telegram", Some("hello"), false, true);
        assert!(link.contains("text="));
        assert!(link.contains("hello"));
    }

    #[test]
    fn test_update_autologin_token() {
        let mut manager = create_test_manager();
        manager.update_autologin_token("token123".to_string());
        assert_eq!(manager.autologin_token.as_deref(), Some("token123"));
    }

    #[test]
    fn test_update_autologin_domains() {
        let mut manager = create_test_manager();
        manager.update_autologin_domains(
            vec!["example.com".to_string()],
            vec!["auth.com".to_string()],
            vec!["whitelist.com".to_string()],
        );
        assert_eq!(manager.autologin_domains.len(), 1);
        assert_eq!(manager.url_auth_domains.len(), 1);
        assert_eq!(manager.whitelisted_domains.len(), 1);
    }

    #[test]
    fn test_max_url_length_const() {
        assert_eq!(MAX_URL_LENGTH, 65536);
    }

    #[test]
    fn test_get_link_user_id() {
        let user_id = LinkManager::get_link_user_id("https://t.me/u/123456");
        assert!(user_id.is_some());

        let user_id = LinkManager::get_link_user_id("https://t.me/telegram");
        assert!(user_id.is_none());
    }

    #[test]
    fn test_error_display() {
        let err = Error::LinkTooLong(100000);
        assert!(err.to_string().contains("too long"));
    }
}
