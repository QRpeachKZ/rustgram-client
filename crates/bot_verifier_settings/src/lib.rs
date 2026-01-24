// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Bot Verifier Settings
//!
//! Bot verification settings for Telegram.
//!
//! ## Overview
//!
//! Bot verification settings contain information about verified bots,
//! including the icon, company name, and description.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_custom_emoji_id::CustomEmojiId;
use serde::{Deserialize, Serialize};

/// Bot verifier settings.
///
/// Contains verification information for a bot.
/// Based on TDLib's `BotVerifierSettings` class.
///
/// # Example
///
/// ```rust
/// use rustgram_bot_verifier_settings::BotVerifierSettings;
///
/// let settings = BotVerifierSettings::new(
///     1234567890,
///     "MyCompany".to_string(),
///     "Verified bot".to_string(),
///     true,
/// );
/// assert!(settings.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotVerifierSettings {
    /// Custom emoji ID for the verification icon.
    icon: CustomEmojiId,
    /// Company name.
    company: String,
    /// Custom description.
    description: String,
    /// Whether the user can modify the custom description.
    can_modify_custom_description: bool,
}

impl Default for BotVerifierSettings {
    fn default() -> Self {
        Self {
            icon: CustomEmojiId::default(),
            company: String::new(),
            description: String::new(),
            can_modify_custom_description: false,
        }
    }
}

impl BotVerifierSettings {
    /// Creates new bot verifier settings.
    ///
    /// # Arguments
    ///
    /// * `icon` - Custom emoji ID for the verification icon
    /// * `company` - Company name
    /// * `description` - Custom description
    /// * `can_modify_custom_description` - Whether description can be modified
    #[must_use]
    pub const fn new(
        icon: i64,
        company: String,
        description: String,
        can_modify_custom_description: bool,
    ) -> Self {
        Self {
            icon: CustomEmojiId::new(icon),
            company,
            description,
            can_modify_custom_description,
        }
    }

    /// Returns the verification icon ID.
    #[must_use]
    pub const fn icon(&self) -> CustomEmojiId {
        self.icon
    }

    /// Returns the company name.
    #[must_use]
    pub fn company(&self) -> &str {
        &self.company
    }

    /// Returns the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns whether the description can be modified.
    #[must_use]
    pub const fn can_modify_custom_description(&self) -> bool {
        self.can_modify_custom_description
    }

    /// Checks if these settings are valid.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.icon.is_valid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let settings =
            BotVerifierSettings::new(1234567890, "Company".to_string(), "Desc".to_string(), true);
        assert!(settings.is_valid());
        assert_eq!(settings.company(), "Company");
        assert_eq!(settings.description(), "Desc");
        assert!(settings.can_modify_custom_description());
    }

    #[test]
    fn test_default() {
        let settings = BotVerifierSettings::default();
        assert!(!settings.is_valid());
    }

    #[test]
    fn test_is_valid() {
        let valid = BotVerifierSettings::new(123, "C".to_string(), "D".to_string(), false);
        assert!(valid.is_valid());

        let invalid = BotVerifierSettings::new(0, "C".to_string(), "D".to_string(), false);
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_equality() {
        let s1 = BotVerifierSettings::new(123, "A".to_string(), "B".to_string(), true);
        let s2 = BotVerifierSettings::new(123, "A".to_string(), "B".to_string(), true);
        let s3 = BotVerifierSettings::new(456, "A".to_string(), "B".to_string(), true);
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }
}
