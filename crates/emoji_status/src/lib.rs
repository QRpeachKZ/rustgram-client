// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0;

//! # Emoji Status
//!
//! Represents emoji status (profile badge) for Telegram users.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `EmojiStatus` class from
//! `td/telegram/EmojiStatus.h`.
//!
//! ## Structure
//!
//! - `custom_emoji_id`: Custom emoji ID for the status
//! - `collectible_id`: Collectible badge ID
//! - `title`: Status title
//! - `slug`: URL slug for the status
//! - Colors and optional gift parameters

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_custom_emoji_id::CustomEmojiId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Emoji status (profile badge).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmojiStatus {
    custom_emoji_id: CustomEmojiId,
    collectible_id: i64,
    title: String,
    slug: String,
    model_custom_emoji_id: CustomEmojiId,
    pattern_custom_emoji_id: CustomEmojiId,
    center_color: i32,
    edge_color: i32,
    pattern_color: i32,
    text_color: i32,
    until_date: i32,
}

impl EmojiStatus {
    /// Creates a new basic EmojiStatus.
    #[must_use]
    pub fn new(custom_emoji_id: CustomEmojiId) -> Self {
        Self {
            custom_emoji_id,
            collectible_id: 0,
            title: String::new(),
            slug: String::new(),
            model_custom_emoji_id: CustomEmojiId::new(0),
            pattern_custom_emoji_id: CustomEmojiId::new(0),
            center_color: 0,
            edge_color: 0,
            pattern_color: 0,
            text_color: 0,
            until_date: 0,
        }
    }

    /// Returns `true` if the status is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.custom_emoji_id.is_valid()
            && (self.collectible_id == 0
                || self.title.is_empty()
                || !self.model_custom_emoji_id.is_valid()
                || !self.pattern_custom_emoji_id.is_valid())
    }

    /// Returns the custom emoji ID.
    #[must_use]
    pub const fn custom_emoji_id(&self) -> CustomEmojiId {
        self.custom_emoji_id
    }

    /// Returns the until date.
    #[must_use]
    pub const fn until_date(&self) -> i32 {
        self.until_date
    }

    /// Clears the until date.
    pub fn clear_until_date(&mut self) {
        self.until_date = 0;
    }
}

impl Default for EmojiStatus {
    fn default() -> Self {
        Self::new(CustomEmojiId::new(0))
    }
}

impl fmt::Display for EmojiStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmojiStatus({})", self.custom_emoji_id.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let status = EmojiStatus::new(CustomEmojiId::new(123));
        assert_eq!(status.custom_emoji_id().get(), 123);
    }

    #[test]
    fn test_default() {
        let status = EmojiStatus::default();
        assert!(status.is_empty());
    }

    #[test]
    fn test_is_empty_true() {
        let status = EmojiStatus::new(CustomEmojiId::new(0));
        assert!(status.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let status = EmojiStatus::new(CustomEmojiId::new(123));
        assert!(!status.is_empty());
    }

    #[test]
    fn test_until_date() {
        let mut status = EmojiStatus::new(CustomEmojiId::new(123));
        assert_eq!(status.until_date(), 0);
    }

    #[test]
    fn test_clear_until_date() {
        let mut status = EmojiStatus::new(CustomEmojiId::new(123));
        status.clear_until_date();
        assert_eq!(status.until_date(), 0);
    }

    #[test]
    fn test_equality() {
        let s1 = EmojiStatus::new(CustomEmojiId::new(123));
        let s2 = EmojiStatus::new(CustomEmojiId::new(123));
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_inequality() {
        let s1 = EmojiStatus::new(CustomEmojiId::new(123));
        let s2 = EmojiStatus::new(CustomEmojiId::new(456));
        assert_ne!(s1, s2);
    }
}
