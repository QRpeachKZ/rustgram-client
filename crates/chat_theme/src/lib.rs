// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Chat Theme
//!
//! Chat theme types for Telegram.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};

/// Chat theme type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChatThemeType {
    /// Default theme.
    Default,
    /// Emoji-based theme.
    Emoji,
    /// Gift-based theme.
    Gift,
}

impl Default for ChatThemeType {
    fn default() -> Self {
        Self::Default
    }
}

/// Chat theme.
///
/// Represents a chat theme in Telegram.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatTheme {
    /// Theme type.
    theme_type: ChatThemeType,
    /// Emoji (for emoji themes).
    emoji: Option<String>,
}

impl Default for ChatTheme {
    fn default() -> Self {
        Self {
            theme_type: ChatThemeType::Default,
            emoji: None,
        }
    }
}

impl ChatTheme {
    /// Creates a new emoji-based chat theme.
    #[must_use]
    pub fn with_emoji(emoji: String) -> Self {
        Self {
            theme_type: ChatThemeType::Emoji,
            emoji: Some(emoji),
        }
    }

    /// Returns the theme type.
    #[must_use]
    pub const fn theme_type(&self) -> ChatThemeType {
        self.theme_type
    }

    /// Returns the emoji if this is an emoji theme.
    #[must_use]
    pub fn get_emoji(&self) -> Option<&str> {
        self.emoji.as_deref()
    }

    /// Checks if this is the default theme.
    #[must_use]
    pub const fn is_default(&self) -> bool {
        matches!(self.theme_type, ChatThemeType::Default)
    }

    /// Checks if this is a gift theme.
    #[must_use]
    pub const fn is_gift(&self) -> bool {
        matches!(self.theme_type, ChatThemeType::Gift)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let theme = ChatTheme::default();
        assert!(theme.is_default());
        assert!(!theme.is_gift());
        assert_eq!(theme.theme_type(), ChatThemeType::Default);
    }

    #[test]
    fn test_with_emoji() {
        let theme = ChatTheme::with_emoji("🎨".to_string());
        assert!(!theme.is_default());
        assert!(!theme.is_gift());
        assert_eq!(theme.theme_type(), ChatThemeType::Emoji);
        assert_eq!(theme.get_emoji(), Some("🎨"));
    }
}
