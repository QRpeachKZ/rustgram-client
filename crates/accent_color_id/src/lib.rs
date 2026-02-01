// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! Accent color ID type for Telegram MTProto client.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_types::{ChannelId, ChatId, UserId};
use std::hash::{Hash, Hasher};

/// Accent color ID wrapper type (0-51). Built-in: 0-6, custom: 7-51.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccentColorId(pub i32);

impl AccentColorId {
    /// Create a new accent color ID.
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    /// Create an invalid accent color ID.
    #[must_use]
    pub const fn invalid() -> Self {
        Self(-1)
    }
    /// Create from user ID (mod 7).
    #[must_use]
    pub fn from_user_id(user_id: UserId) -> Self {
        Self((user_id.0 % 7) as i32)
    }
    /// Create from chat ID (mod 7).
    #[must_use]
    pub fn from_chat_id(chat_id: ChatId) -> Self {
        Self((chat_id.0 % 7) as i32)
    }
    /// Create from channel ID (mod 7).
    #[must_use]
    pub fn from_channel_id(channel_id: ChannelId) -> Self {
        Self((channel_id.0 % 7) as i32)
    }
    /// Check if valid (non-negative).
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.0 >= 0
    }
    /// Check if built-in (0-6).
    #[must_use]
    pub const fn is_built_in(&self) -> bool {
        self.0 >= 0 && self.0 < 7
    }
    /// Get the raw ID value.
    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0
    }
}

impl Default for AccentColorId {
    fn default() -> Self {
        Self(-1)
    }
}

impl Hash for AccentColorId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-accent-color-id";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-accent-color-id");
    }
    #[test]
    fn test_invalid() {
        assert!(!AccentColorId::invalid().is_valid());
    }
    #[test]
    fn test_default() {
        assert!(!AccentColorId::default().is_valid());
    }
    #[test]
    fn test_new() {
        let c = AccentColorId::new(5);
        assert!(c.is_valid() && c.is_built_in());
    }
    #[test]
    fn test_from_ids() {
        assert_eq!(AccentColorId::from_user_id(UserId(123)).get(), 123 % 7);
        assert_eq!(AccentColorId::from_chat_id(ChatId(456)).get(), 456 % 7);
        assert_eq!(
            AccentColorId::from_channel_id(ChannelId(789)).get(),
            789 % 7
        );
    }
    #[test]
    fn test_validation() {
        assert!(AccentColorId::new(0).is_built_in());
        assert!(!AccentColorId::new(7).is_built_in());
    }
    #[test]
    fn test_equality() {
        assert_eq!(AccentColorId::new(5), AccentColorId::new(5));
    }
}
