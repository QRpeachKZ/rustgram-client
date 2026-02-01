// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0;

//! # Emoji Group
//!
//! Represents a group of emojis for Telegram emoji categories.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `EmojiGroup` class from
//! `td/telegram/EmojiGroup.h`.
//!
//! ## Structure
//!
//! - `title`: Group title
//! - `icon_custom_emoji_id`: Custom emoji for the group icon
//! - `emojis`: Vector of emoji strings in the group
//! - `is_greeting`: Whether this is a greeting group
//! - `is_premium`: Whether this is a premium-only group

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_custom_emoji_id::CustomEmojiId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Emoji group for organizing emoji categories.
///
/// Contains a collection of related emojis with metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmojiGroup {
    /// Group title.
    title: String,
    /// Custom emoji ID for the group icon.
    icon_custom_emoji_id: CustomEmojiId,
    /// Emojis in this group.
    emojis: Vec<String>,
    /// Whether this is a greeting group.
    is_greeting: bool,
    /// Whether this is a premium-only group.
    is_premium: bool,
}

impl EmojiGroup {
    /// Creates a new EmojiGroup.
    #[must_use]
    pub fn new(
        title: String,
        icon_custom_emoji_id: CustomEmojiId,
        emojis: Vec<String>,
        is_greeting: bool,
        is_premium: bool,
    ) -> Self {
        Self {
            title,
            icon_custom_emoji_id,
            emojis,
            is_greeting,
            is_premium,
        }
    }

    /// Returns the group title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the icon custom emoji ID.
    #[must_use]
    pub const fn icon_custom_emoji_id(&self) -> CustomEmojiId {
        self.icon_custom_emoji_id
    }

    /// Returns the emojis in this group.
    #[must_use]
    pub fn emojis(&self) -> &[String] {
        &self.emojis
    }

    /// Returns `true` if this is a greeting group.
    #[must_use]
    pub const fn is_greeting(&self) -> bool {
        self.is_greeting
    }

    /// Returns `true` if this is a premium-only group.
    #[must_use]
    pub const fn is_premium(&self) -> bool {
        self.is_premium
    }
}

impl Default for EmojiGroup {
    fn default() -> Self {
        Self {
            title: String::new(),
            icon_custom_emoji_id: CustomEmojiId::new(0),
            emojis: Vec::new(),
            is_greeting: false,
            is_premium: false,
        }
    }
}

impl fmt::Display for EmojiGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmojiGroup({})", self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let group = EmojiGroup::new(
            "Smileys".to_string(),
            CustomEmojiId::new(123),
            vec!["😀".to_string(), "😃".to_string()],
            false,
            false,
        );
        assert_eq!(group.title(), "Smileys");
        assert_eq!(group.emojis().len(), 2);
    }

    #[test]
    fn test_default() {
        let group = EmojiGroup::default();
        assert!(group.title().is_empty());
        assert!(group.emojis().is_empty());
    }

    #[test]
    fn test_is_greeting() {
        let group = EmojiGroup::new(
            "Greeting".to_string(),
            CustomEmojiId::new(0),
            vec![],
            true,
            false,
        );
        assert!(group.is_greeting());
    }

    #[test]
    fn test_is_premium() {
        let group = EmojiGroup::new(
            "Premium".to_string(),
            CustomEmojiId::new(0),
            vec![],
            false,
            true,
        );
        assert!(group.is_premium());
    }

    #[test]
    fn test_icon_custom_emoji_id() {
        let group = EmojiGroup::new(
            "Test".to_string(),
            CustomEmojiId::new(456),
            vec![],
            false,
            false,
        );
        assert_eq!(group.icon_custom_emoji_id().get(), 456);
    }

    #[test]
    fn test_emojis() {
        let emojis = vec!["😀".to_string(), "😃".to_string(), "😄".to_string()];
        let group = EmojiGroup::new(
            "Test".to_string(),
            CustomEmojiId::new(0),
            emojis.clone(),
            false,
            false,
        );
        assert_eq!(group.emojis(), &emojis);
    }

    #[test]
    fn test_equality() {
        let g1 = EmojiGroup::new(
            "Test".to_string(),
            CustomEmojiId::new(123),
            vec!["😀".to_string()],
            false,
            false,
        );
        let g2 = EmojiGroup::new(
            "Test".to_string(),
            CustomEmojiId::new(123),
            vec!["😀".to_string()],
            false,
            false,
        );
        assert_eq!(g1, g2);
    }

    #[test]
    fn test_inequality() {
        let g1 = EmojiGroup::new(
            "Test1".to_string(),
            CustomEmojiId::new(123),
            vec!["😀".to_string()],
            false,
            false,
        );
        let g2 = EmojiGroup::new(
            "Test2".to_string(),
            CustomEmojiId::new(123),
            vec!["😀".to_string()],
            false,
            false,
        );
        assert_ne!(g1, g2);
    }
}
