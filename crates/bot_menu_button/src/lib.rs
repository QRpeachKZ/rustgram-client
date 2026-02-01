// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! Bot menu button type for Telegram MTProto client.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::hash::{Hash, Hasher};

/// Bot menu button type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotMenuButton {
    text: String,
    url: String,
}

impl BotMenuButton {
    /// Create a new empty bot menu button.
    #[must_use]
    pub fn new() -> Self {
        Self {
            text: String::new(),
            url: String::new(),
        }
    }
    /// Create with text and URL.
    #[must_use]
    pub fn with_params(text: String, url: String) -> Self {
        Self { text, url }
    }
    /// Get the button text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
    /// Get the button URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }
    /// Check if empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty() && self.url.is_empty()
    }
}

impl Hash for BotMenuButton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text.hash(state);
        self.url.hash(state);
    }
}

impl Default for BotMenuButton {
    fn default() -> Self {
        Self::new()
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-bot-menu-button";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-bot-menu-button");
    }
    #[test]
    fn test_new() {
        assert!(BotMenuButton::new().is_empty());
    }
    #[test]
    fn test_with_params() {
        let b = BotMenuButton::with_params("H".into(), "U".into());
        assert_eq!(b.text(), "H");
    }
    #[test]
    fn test_is_empty() {
        assert!(!BotMenuButton::with_params("T".into(), String::new()).is_empty());
    }
    #[test]
    fn test_equality() {
        let b1 = BotMenuButton::with_params("X".into(), "Y".into());
        assert_eq!(b1, BotMenuButton::with_params("X".into(), "Y".into()));
    }
}
