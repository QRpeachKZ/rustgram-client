// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Chain ID
//!
//! Chain identifier for dependency tracking in Telegram.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_types::{ChannelId, ChatId, DialogId, UserId};
use std::hash::{Hash, Hasher};

/// Chain identifier for dependency tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChainId {
    inner: u64,
}

impl Default for ChainId {
    fn default() -> Self {
        Self { inner: 0 }
    }
}

impl ChainId {
    /// Base offset for dialog chains.
    const DIALOG_OFFSET: u64 = 10;

    /// Creates a ChainId from a u64 value.
    #[must_use]
    pub const fn from_u64(value: u64) -> Self {
        Self { inner: value }
    }

    /// Creates a ChainId from a DialogId.
    #[must_use]
    pub fn from_dialog(dialog_id: DialogId) -> Self {
        let id = dialog_id.to_encoded();
        let id = id as u64;
        Self {
            inner: (id << Self::DIALOG_OFFSET) | Self::DIALOG_OFFSET,
        }
    }

    /// Creates a ChainId from a ChannelId.
    #[must_use]
    pub fn from_channel(channel_id: ChannelId) -> Self {
        Self::from_dialog(DialogId::from_channel(channel_id))
    }

    /// Creates a ChainId from a ChatId.
    #[must_use]
    pub fn from_chat(chat_id: ChatId) -> Self {
        Self::from_dialog(DialogId::from_chat(chat_id))
    }

    /// Creates a ChainId from a UserId.
    #[must_use]
    pub fn from_user(user_id: UserId) -> Self {
        Self::from_dialog(DialogId::from_user(user_id))
    }

    /// Creates a ChainId from a string hash.
    #[must_use]
    pub fn from_string(s: &str) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut hasher);
        Self {
            inner: hasher.finish(),
        }
    }

    /// Returns the inner u64 value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let chain_id = ChainId::default();
        assert_eq!(chain_id.get(), 0);
    }

    #[test]
    fn test_from_user() {
        let user_id = UserId::new(123).unwrap();
        let chain_id = ChainId::from_user(user_id);
        let expected = (123 << 10) | 10;
        assert_eq!(chain_id.get(), expected);
    }

    #[test]
    fn test_from_chat() {
        let chat_id = ChatId::new(456).unwrap();
        let chain_id = ChainId::from_chat(chat_id);
        let expected = (456 << 10) | 10;
        assert_eq!(chain_id.get(), expected);
    }

    #[test]
    fn test_from_string() {
        let chain1 = ChainId::from_string("test");
        let chain2 = ChainId::from_string("test");
        let chain3 = ChainId::from_string("other");
        assert_eq!(chain1, chain2);
        assert_ne!(chain1, chain3);
    }

    #[test]
    fn test_from_u64() {
        let chain_id = ChainId::from_u64(999);
        assert_eq!(chain_id.get(), 999);
    }
}
