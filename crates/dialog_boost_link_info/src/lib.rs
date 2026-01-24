// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Dialog boost link information type for Telegram MTProto client.
//!
//! This module implements the dialog boost link info from TDLib.
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
//!
//! let info = DialogBoostLinkInfo::Username("mychannel".to_string());
//! assert!(info.is_username());
//!
//! let channel_info = DialogBoostLinkInfo::ChannelId(1234567890);
//! assert!(channel_info.is_channel_id());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_types::ChannelId;
use std::fmt;

/// Dialog boost link information.
///
/// Contains information about a boost link for a dialog.
/// Can be either a username or a channel ID.
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
///
/// let username_info = DialogBoostLinkInfo::Username("channel".to_string());
/// assert!(username_info.is_username());
/// assert_eq!(username_info.username().unwrap(), "channel");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogBoostLinkInfo {
    /// Username variant
    Username(String),
    /// Channel ID variant
    ChannelId(i64),
}

impl DialogBoostLinkInfo {
    /// Creates a new boost link info from a username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username without @ prefix
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
    ///
    /// let info = DialogBoostLinkInfo::new_username("mychannel");
    /// assert!(info.is_username());
    /// ```
    pub fn new_username(username: impl Into<String>) -> Self {
        Self::Username(username.into())
    }

    /// Creates a new boost link info from a channel ID.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The raw channel ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
    ///
    /// let info = DialogBoostLinkInfo::new_channel_id(1234567890);
    /// assert!(info.is_channel_id());
    /// ```
    pub fn new_channel_id(channel_id: i64) -> Self {
        Self::ChannelId(channel_id)
    }

    /// Creates a new boost link info from a ChannelId type.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The ChannelId
    ///
    /// # Returns
    ///
    /// Returns `Some(DialogBoostLinkInfo)` if the ChannelId is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
    /// use rustgram_types::ChannelId;
    ///
    /// let channel_id = ChannelId::new(1234567890).unwrap();
    /// let info = DialogBoostLinkInfo::from_channel_id(channel_id);
    /// assert!(info.is_some());
    /// ```
    pub fn from_channel_id(channel_id: ChannelId) -> Option<Self> {
        match channel_id.0 {
            id if id > 0 => Some(Self::ChannelId(id)),
            _ => None,
        }
    }

    /// Creates boost link info from a mock telegram_api object.
    ///
    /// This is a simplified version for testing. The real implementation would
    /// parse the actual MTProto object.
    ///
    /// # Arguments
    ///
    /// * `username` - Optional username
    /// * `channel_id` - Optional channel ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
    ///
    /// let info = DialogBoostLinkInfo::from_telegram_api(
    ///     Some("mychannel".to_string()),
    ///     None
    /// );
    /// assert!(info.unwrap().is_username());
    /// ```
    pub fn from_telegram_api(username: Option<String>, channel_id: Option<i64>) -> Option<Self> {
        match (username, channel_id) {
            (Some(u), None) if !u.is_empty() => Some(Self::Username(u)),
            (None, Some(id)) if id > 0 => Some(Self::ChannelId(id)),
            (Some(u), Some(id)) if !u.is_empty() && id > 0 => Some(Self::Username(u)),
            _ => None,
        }
    }

    /// Checks if this is a username variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
    ///
    /// let info = DialogBoostLinkInfo::Username("channel".to_string());
    /// assert!(info.is_username());
    /// ```
    pub fn is_username(&self) -> bool {
        matches!(self, Self::Username(_))
    }

    /// Checks if this is a channel ID variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
    ///
    /// let info = DialogBoostLinkInfo::ChannelId(1234567890);
    /// assert!(info.is_channel_id());
    /// ```
    pub fn is_channel_id(&self) -> bool {
        matches!(self, Self::ChannelId(_))
    }

    /// Returns the username if this is a username variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
    ///
    /// let info = DialogBoostLinkInfo::Username("mychannel".to_string());
    /// assert_eq!(info.username(), Some("mychannel"));
    /// ```
    pub fn username(&self) -> Option<&str> {
        match self {
            Self::Username(u) => Some(u),
            _ => None,
        }
    }

    /// Returns the channel ID if this is a channel ID variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
    ///
    /// let info = DialogBoostLinkInfo::ChannelId(1234567890);
    /// assert_eq!(info.channel_id(), Some(1234567890));
    /// ```
    pub fn channel_id(&self) -> Option<i64> {
        match self {
            Self::ChannelId(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the channel ID as a ChannelId type if valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
    /// use rustgram_types::ChannelId;
    ///
    /// let info = DialogBoostLinkInfo::ChannelId(1234567890);
    /// let channel_id = info.as_channel_id();
    /// assert!(channel_id.is_ok());
    /// ```
    pub fn as_channel_id(&self) -> Result<ChannelId, String> {
        match self {
            Self::ChannelId(id) => ChannelId::new(*id).map_err(|e| e.to_string()),
            Self::Username(_) => Err("Not a channel ID variant".to_string()),
        }
    }

    /// Checks if the boost link info is valid.
    ///
    /// A username must be non-empty, and a channel ID must be positive.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_boost_link_info::DialogBoostLinkInfo;
    ///
    /// let username_info = DialogBoostLinkInfo::Username("channel".to_string());
    /// assert!(username_info.is_valid());
    ///
    /// let channel_info = DialogBoostLinkInfo::ChannelId(1234567890);
    /// assert!(channel_info.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Username(u) => !u.is_empty(),
            Self::ChannelId(id) => *id > 0,
        }
    }
}

impl fmt::Display for DialogBoostLinkInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Username(u) => write!(f, "@{}", u),
            Self::ChannelId(id) => write!(f, "channel/{}", id),
        }
    }
}

impl Default for DialogBoostLinkInfo {
    fn default() -> Self {
        Self::Username(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_username() {
        let info = DialogBoostLinkInfo::new_username("mychannel");
        assert!(info.is_username());
        assert_eq!(info.username().unwrap(), "mychannel");
    }

    #[test]
    fn test_new_channel_id() {
        let info = DialogBoostLinkInfo::new_channel_id(1234567890);
        assert!(info.is_channel_id());
        assert_eq!(info.channel_id().unwrap(), 1234567890);
    }

    #[test]
    fn test_from_channel_id_valid() {
        let channel_id = ChannelId::new(1234567890).unwrap();
        let info = DialogBoostLinkInfo::from_channel_id(channel_id);
        assert!(info.is_some());
        assert!(info.unwrap().is_channel_id());
    }

    #[test]
    fn test_from_channel_id_invalid() {
        let channel_id = ChannelId::new(-1);
        assert!(channel_id.is_err());
    }

    #[test]
    fn test_from_telegram_api_username() {
        let info = DialogBoostLinkInfo::from_telegram_api(Some("mychannel".to_string()), None);
        assert!(info.is_some());
        assert!(info.unwrap().is_username());
    }

    #[test]
    fn test_from_telegram_api_channel_id() {
        let info = DialogBoostLinkInfo::from_telegram_api(None, Some(1234567890));
        assert!(info.is_some());
        assert!(info.unwrap().is_channel_id());
    }

    #[test]
    fn test_from_telegram_api_none() {
        let info = DialogBoostLinkInfo::from_telegram_api(None, None);
        assert!(info.is_none());
    }

    #[test]
    fn test_from_telegram_api_both() {
        let info =
            DialogBoostLinkInfo::from_telegram_api(Some("mychannel".to_string()), Some(1234567890));
        assert!(info.is_some());
    }

    #[test]
    fn test_from_telegram_api_empty_username() {
        let info = DialogBoostLinkInfo::from_telegram_api(Some(String::new()), None);
        assert!(info.is_none());
    }

    #[test]
    fn test_from_telegram_api_invalid_channel_id() {
        let info = DialogBoostLinkInfo::from_telegram_api(None, Some(-1));
        assert!(info.is_none());
    }

    #[test]
    fn test_is_username() {
        let info = DialogBoostLinkInfo::Username("channel".to_string());
        assert!(info.is_username());
        assert!(!info.is_channel_id());
    }

    #[test]
    fn test_is_channel_id() {
        let info = DialogBoostLinkInfo::ChannelId(1234567890);
        assert!(info.is_channel_id());
        assert!(!info.is_username());
    }

    #[test]
    fn test_username_some() {
        let info = DialogBoostLinkInfo::Username("mychannel".to_string());
        assert_eq!(info.username(), Some("mychannel"));
    }

    #[test]
    fn test_username_none() {
        let info = DialogBoostLinkInfo::ChannelId(1234567890);
        assert_eq!(info.username(), None);
    }

    #[test]
    fn test_channel_id_some() {
        let info = DialogBoostLinkInfo::ChannelId(1234567890);
        assert_eq!(info.channel_id(), Some(1234567890));
    }

    #[test]
    fn test_channel_id_none() {
        let info = DialogBoostLinkInfo::Username("channel".to_string());
        assert_eq!(info.channel_id(), None);
    }

    #[test]
    fn test_as_channel_id_valid() {
        let info = DialogBoostLinkInfo::ChannelId(1234567890);
        let result = info.as_channel_id();
        assert!(result.is_ok());
    }

    #[test]
    fn test_as_channel_id_from_username() {
        let info = DialogBoostLinkInfo::Username("channel".to_string());
        let result = info.as_channel_id();
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_username() {
        let info = DialogBoostLinkInfo::Username("channel".to_string());
        assert!(info.is_valid());
    }

    #[test]
    fn test_is_valid_empty_username() {
        let info = DialogBoostLinkInfo::Username(String::new());
        assert!(!info.is_valid());
    }

    #[test]
    fn test_is_valid_channel_id() {
        let info = DialogBoostLinkInfo::ChannelId(1234567890);
        assert!(info.is_valid());
    }

    #[test]
    fn test_is_valid_invalid_channel_id() {
        let info = DialogBoostLinkInfo::ChannelId(-1);
        assert!(!info.is_valid());
    }

    #[test]
    fn test_is_valid_zero_channel_id() {
        let info = DialogBoostLinkInfo::ChannelId(0);
        assert!(!info.is_valid());
    }

    #[test]
    fn test_display_username() {
        let info = DialogBoostLinkInfo::Username("mychannel".to_string());
        let display = format!("{}", info);
        assert_eq!(display, "@mychannel");
    }

    #[test]
    fn test_display_channel_id() {
        let info = DialogBoostLinkInfo::ChannelId(1234567890);
        let display = format!("{}", info);
        assert!(display.contains("1234567890"));
        assert!(display.contains("channel"));
    }

    #[test]
    fn test_default() {
        let info = DialogBoostLinkInfo::default();
        assert!(info.is_username());
        assert!(!info.is_valid());
    }

    #[test]
    fn test_equality_username() {
        let info1 = DialogBoostLinkInfo::Username("channel".to_string());
        let info2 = DialogBoostLinkInfo::Username("channel".to_string());
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_equality_channel_id() {
        let info1 = DialogBoostLinkInfo::ChannelId(1234567890);
        let info2 = DialogBoostLinkInfo::ChannelId(1234567890);
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_inequality_different_types() {
        let info1 = DialogBoostLinkInfo::Username("channel".to_string());
        let info2 = DialogBoostLinkInfo::ChannelId(1234567890);
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_inequality_different_usernames() {
        let info1 = DialogBoostLinkInfo::Username("channel1".to_string());
        let info2 = DialogBoostLinkInfo::Username("channel2".to_string());
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_inequality_different_channel_ids() {
        let info1 = DialogBoostLinkInfo::ChannelId(1234567890);
        let info2 = DialogBoostLinkInfo::ChannelId(9876543210);
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_clone_username() {
        let info1 = DialogBoostLinkInfo::Username("channel".to_string());
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_clone_channel_id() {
        let info1 = DialogBoostLinkInfo::ChannelId(1234567890);
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_debug_username() {
        let info = DialogBoostLinkInfo::Username("channel".to_string());
        let debug = format!("{:?}", info);
        assert!(debug.contains("Username"));
        assert!(debug.contains("channel"));
    }

    #[test]
    fn test_debug_channel_id() {
        let info = DialogBoostLinkInfo::ChannelId(1234567890);
        let debug = format!("{:?}", info);
        assert!(debug.contains("ChannelId"));
        assert!(debug.contains("1234567890"));
    }

    #[test]
    fn test_username_with_at_sign() {
        let info = DialogBoostLinkInfo::Username("@channel".to_string());
        assert!(info.is_valid());
        assert_eq!(info.username().unwrap(), "@channel");
    }

    #[test]
    fn test_large_channel_id() {
        let info = DialogBoostLinkInfo::ChannelId(i64::MAX);
        assert!(info.is_valid());
        assert_eq!(info.channel_id().unwrap(), i64::MAX);
    }

    #[test]
    fn test_multiple_variants() {
        let variants = [
            DialogBoostLinkInfo::Username("channel1".to_string()),
            DialogBoostLinkInfo::Username("channel2".to_string()),
            DialogBoostLinkInfo::ChannelId(1234567890),
            DialogBoostLinkInfo::ChannelId(9876543210),
        ];

        assert_eq!(variants.len(), 4);
        assert!(variants[0].is_username());
        assert!(variants[2].is_channel_id());
    }
}
