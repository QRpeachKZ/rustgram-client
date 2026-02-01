// Copyright 2024 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Bot Verification
//!
//! Bot verification types for Telegram.
//!
//! Based on TDLib's BotVerification implementation.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use rustgram_formatted_text::FormattedText;
use rustgram_types::UserId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Custom emoji identifier.
///
/// Represents a custom emoji ID used for bot verification icons.
pub type CustomEmojiId = i64;

/// Bot verifier settings for bot verification.
///
/// Contains settings for bot verification including icon, company name,
/// description, and whether custom description can be modified.
///
/// # Example
///
/// ```rust
/// use rustgram_bot_verification::BotVerifierSettings;
///
/// let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
/// assert!(settings.is_valid());
/// assert_eq!(settings.icon(), 1234567890);
/// assert_eq!(settings.company(), "Telegram");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotVerifierSettings {
    /// Custom emoji ID for the verification icon
    icon: CustomEmojiId,
    /// Company name
    company: String,
    /// Description text
    description: String,
    /// Whether custom description can be modified
    can_modify_custom_description: bool,
}

impl BotVerifierSettings {
    /// Creates a new bot verifier settings.
    ///
    /// # Arguments
    ///
    /// * `icon` - Custom emoji ID for the verification icon
    /// * `company` - Company name
    /// * `description` - Description text
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
    /// assert_eq!(settings.icon(), 1234567890);
    /// assert_eq!(settings.company(), "Telegram");
    /// assert!(!settings.can_modify_custom_description());
    /// ```
    pub fn new(icon: CustomEmojiId, company: &str, description: &str) -> Self {
        Self {
            icon,
            company: company.to_string(),
            description: description.to_string(),
            can_modify_custom_description: false,
        }
    }

    /// Creates a new bot verifier settings with all fields.
    ///
    /// # Arguments
    ///
    /// * `icon` - Custom emoji ID for the verification icon
    /// * `company` - Company name
    /// * `description` - Description text
    /// * `can_modify_custom_description` - Whether custom description can be modified
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let settings = BotVerifierSettings::with_data(
    ///     1234567890,
    ///     "Telegram",
    ///     "Official bot",
    ///     true
    /// );
    /// assert!(settings.can_modify_custom_description());
    /// ```
    pub fn with_data(
        icon: CustomEmojiId,
        company: &str,
        description: &str,
        can_modify_custom_description: bool,
    ) -> Self {
        Self {
            icon,
            company: company.to_string(),
            description: description.to_string(),
            can_modify_custom_description,
        }
    }

    /// Returns the custom emoji ID for the verification icon.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
    /// assert_eq!(settings.icon(), 1234567890);
    /// ```
    pub fn icon(&self) -> CustomEmojiId {
        self.icon
    }

    /// Returns the company name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
    /// assert_eq!(settings.company(), "Telegram");
    /// ```
    pub fn company(&self) -> &str {
        &self.company
    }

    /// Returns the description text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
    /// assert_eq!(settings.description(), "Official bot");
    /// ```
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns whether custom description can be modified.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
    /// assert!(!settings.can_modify_custom_description());
    /// ```
    pub fn can_modify_custom_description(&self) -> bool {
        self.can_modify_custom_description
    }

    /// Sets the description text.
    ///
    /// # Arguments
    ///
    /// * `description` - The description text to set
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let mut settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
    /// settings.set_description("Updated description");
    /// assert_eq!(settings.description(), "Updated description");
    /// ```
    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    /// Sets whether custom description can be modified.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether custom description can be modified
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let mut settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
    /// settings.set_can_modify_custom_description(true);
    /// assert!(settings.can_modify_custom_description());
    /// ```
    pub fn set_can_modify_custom_description(&mut self, value: bool) {
        self.can_modify_custom_description = value;
    }

    /// Returns `true` if this bot verifier settings is valid.
    ///
    /// A settings is valid if the icon is non-zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
    /// assert!(settings.is_valid());
    ///
    /// let invalid = BotVerifierSettings::new(0, "Telegram", "Official bot");
    /// assert!(!invalid.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.icon != 0
    }

    /// Converts this settings to TD API representation.
    ///
    /// Returns a tuple of (icon, company, description, can_modify_custom_description).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
    /// let (icon, company, description, can_modify) = settings.to_td_api();
    /// assert_eq!(icon, 1234567890);
    /// assert_eq!(company, "Telegram");
    /// ```
    pub fn to_td_api(&self) -> (CustomEmojiId, String, String, bool) {
        (
            self.icon,
            self.company.clone(),
            self.description.clone(),
            self.can_modify_custom_description,
        )
    }

    /// Creates bot verifier settings from TD API representation.
    ///
    /// # Arguments
    ///
    /// * `icon` - Custom emoji ID for the verification icon
    /// * `company` - Company name
    /// * `description` - Description text
    /// * `can_modify_custom_description` - Whether custom description can be modified
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerifierSettings;
    ///
    /// let settings = BotVerifierSettings::from_td_api(
    ///     1234567890,
    ///     "Telegram".to_string(),
    ///     "Official bot".to_string(),
    ///     true
    /// );
    /// assert_eq!(settings.icon(), 1234567890);
    /// assert!(settings.can_modify_custom_description());
    /// ```
    pub fn from_td_api(
        icon: CustomEmojiId,
        company: String,
        description: String,
        can_modify_custom_description: bool,
    ) -> Self {
        Self {
            icon,
            company,
            description,
            can_modify_custom_description,
        }
    }
}

impl Default for BotVerifierSettings {
    fn default() -> Self {
        Self {
            icon: 0,
            company: String::new(),
            description: String::new(),
            can_modify_custom_description: false,
        }
    }
}

impl fmt::Display for BotVerifierSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BotVerifierSettings {{ icon: {}, company: {}, description: {}, can_modify: {} }}",
            self.icon, self.company, self.description, self.can_modify_custom_description
        )
    }
}

/// Bot verification information.
///
/// Represents verification information for a bot, including the bot user ID,
/// verification icon (custom emoji), and description text.
///
/// # Example
///
/// ```rust
/// use rustgram_bot_verification::BotVerification;
/// use rustgram_types::UserId;
///
/// let bot_id = UserId::new(12345).unwrap();
/// let verification = BotVerification::new(bot_id, 1234567890, Some("Verified bot"));
/// assert!(verification.is_valid());
/// assert_eq!(verification.bot_user_id(), bot_id);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct BotVerification {
    /// Bot user ID
    bot_user_id: UserId,
    /// Custom emoji ID for the verification icon
    icon: CustomEmojiId,
    /// Description text
    description: Option<String>,
}

impl BotVerification {
    /// Creates a new bot verification.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - Bot user ID
    /// * `icon` - Custom emoji ID for the verification icon
    /// * `description` - Optional description text
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerification;
    /// use rustgram_types::UserId;
    ///
    /// let bot_id = UserId::new(12345).unwrap();
    /// let verification = BotVerification::new(bot_id, 1234567890, Some("Verified bot"));
    /// assert_eq!(verification.bot_user_id(), bot_id);
    /// ```
    pub fn new(bot_user_id: UserId, icon: CustomEmojiId, description: Option<&str>) -> Self {
        Self {
            bot_user_id,
            icon,
            description: description.map(|s| s.to_string()),
        }
    }

    /// Returns the bot user ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerification;
    /// use rustgram_types::UserId;
    ///
    /// let bot_id = UserId::new(12345).unwrap();
    /// let verification = BotVerification::new(bot_id, 1234567890, None);
    /// assert_eq!(verification.bot_user_id(), bot_id);
    /// ```
    pub fn bot_user_id(&self) -> UserId {
        self.bot_user_id
    }

    /// Returns the custom emoji ID for the verification icon.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerification;
    /// use rustgram_types::UserId;
    ///
    /// let bot_id = UserId::new(12345).unwrap();
    /// let verification = BotVerification::new(bot_id, 1234567890, None);
    /// assert_eq!(verification.icon(), 1234567890);
    /// ```
    pub fn icon(&self) -> CustomEmojiId {
        self.icon
    }

    /// Returns the description text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerification;
    /// use rustgram_types::UserId;
    ///
    /// let bot_id = UserId::new(12345).unwrap();
    /// let verification = BotVerification::new(bot_id, 1234567890, Some("Verified bot"));
    /// assert_eq!(verification.description(), Some("Verified bot"));
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Sets the description text.
    ///
    /// # Arguments
    ///
    /// * `description` - The description text to set
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerification;
    /// use rustgram_types::UserId;
    ///
    /// let bot_id = UserId::new(12345).unwrap();
    /// let mut verification = BotVerification::new(bot_id, 1234567890, None);
    /// verification.set_description("New description");
    /// assert_eq!(verification.description(), Some("New description"));
    /// ```
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    /// Clears the description text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerification;
    /// use rustgram_types::UserId;
    ///
    /// let bot_id = UserId::new(12345).unwrap();
    /// let mut verification = BotVerification::new(bot_id, 1234567890, Some("Description"));
    /// verification.clear_description();
    /// assert_eq!(verification.description(), None);
    /// ```
    pub fn clear_description(&mut self) {
        self.description = None;
    }

    /// Returns `true` if this bot verification is valid.
    ///
    /// A verification is valid if the bot user ID is valid and the icon is non-zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerification;
    /// use rustgram_types::UserId;
    ///
    /// let bot_id = UserId::new(12345).unwrap();
    /// let verification = BotVerification::new(bot_id, 1234567890, None);
    /// assert!(verification.is_valid());
    ///
    /// let invalid = BotVerification::new(bot_id, 0, None);
    /// assert!(!invalid.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.bot_user_id.is_valid() && self.icon != 0
    }

    /// Converts this verification to a formatted text representation.
    ///
    /// Returns `None` if the verification is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_bot_verification::BotVerification;
    /// use rustgram_types::UserId;
    ///
    /// let bot_id = UserId::new(12345).unwrap();
    /// let verification = BotVerification::new(bot_id, 1234567890, Some("Verified bot"));
    /// let text = verification.to_formatted_text().unwrap();
    /// assert!(!text.is_empty());
    /// ```
    pub fn to_formatted_text(&self) -> Option<FormattedText> {
        if !self.is_valid() {
            return None;
        }

        let text = self.description.clone().unwrap_or_default();
        Some(FormattedText::new(&text))
    }
}

impl fmt::Display for BotVerification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "verified by {} with {} and {}",
            self.bot_user_id,
            self.icon,
            self.description.as_deref().unwrap_or("(no description)")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_verification_new() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, 1234567890, None);

        assert_eq!(verification.bot_user_id(), bot_id);
        assert_eq!(verification.icon(), 1234567890);
        assert_eq!(verification.description(), None);
    }

    #[test]
    fn test_bot_verification_new_with_description() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, 1234567890, Some("Verified bot"));

        assert_eq!(verification.description(), Some("Verified bot"));
    }

    #[test]
    fn test_bot_verification_is_valid() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, 1234567890, None);
        assert!(verification.is_valid());

        let invalid_icon = BotVerification::new(bot_id, 0, None);
        assert!(!invalid_icon.is_valid());

        let invalid_bot = BotVerification::new(UserId(0), 1234567890, None);
        assert!(!invalid_bot.is_valid());
    }

    #[test]
    fn test_bot_verification_set_description() {
        let bot_id = UserId::new(12345).unwrap();
        let mut verification = BotVerification::new(bot_id, 1234567890, None);

        verification.set_description("New description");
        assert_eq!(verification.description(), Some("New description"));

        verification.set_description("Another description");
        assert_eq!(verification.description(), Some("Another description"));
    }

    #[test]
    fn test_bot_verification_clear_description() {
        let bot_id = UserId::new(12345).unwrap();
        let mut verification = BotVerification::new(bot_id, 1234567890, Some("Description"));

        verification.clear_description();
        assert_eq!(verification.description(), None);
    }

    #[test]
    fn test_bot_verification_to_formatted_text() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, 1234567890, Some("Verified bot"));

        let text = verification.to_formatted_text();
        assert!(text.is_some());
        assert_eq!(text.unwrap().text(), "Verified bot");
    }

    #[test]
    fn test_bot_verification_to_formatted_text_no_description() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, 1234567890, None);

        let text = verification.to_formatted_text();
        assert!(text.is_some());
        assert_eq!(text.unwrap().text(), "");
    }

    #[test]
    fn test_bot_verification_to_formatted_text_invalid() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, 0, None); // Invalid icon

        let text = verification.to_formatted_text();
        assert!(text.is_none());
    }

    #[test]
    fn test_bot_verification_equality() {
        let bot_id = UserId::new(12345).unwrap();
        let verification1 = BotVerification::new(bot_id, 1234567890, Some("Description"));
        let verification2 = BotVerification::new(bot_id, 1234567890, Some("Description"));
        assert_eq!(verification1, verification2);

        let verification3 = BotVerification::new(bot_id, 1234567890, None);
        assert_ne!(verification1, verification3);
    }

    #[test]
    fn test_bot_verification_default() {
        let verification = BotVerification::default();
        assert!(!verification.is_valid());
        assert_eq!(verification.bot_user_id(), UserId(0));
        assert_eq!(verification.icon(), 0);
    }

    #[test]
    fn test_bot_verification_clone() {
        let bot_id = UserId::new(12345).unwrap();
        let verification1 = BotVerification::new(bot_id, 1234567890, Some("Description"));
        let verification2 = verification1.clone();
        assert_eq!(verification1, verification2);
    }

    #[test]
    fn test_bot_verification_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let bot_id = UserId::new(12345).unwrap();
        let verification1 = BotVerification::new(bot_id, 1234567890, Some("Description"));
        let verification2 = BotVerification::new(bot_id, 1234567890, Some("Description"));

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        verification1.hash(&mut hasher1);
        verification2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_bot_verification_serialization() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, 1234567890, Some("Description"));

        let json = serde_json::to_string(&verification).unwrap();
        let parsed: BotVerification = serde_json::from_str(&json).unwrap();
        assert_eq!(verification, parsed);
    }

    #[test]
    fn test_bot_verification_display() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, 1234567890, Some("Verified bot"));

        let display = format!("{}", verification);
        assert!(display.contains("12345"));
        assert!(display.contains("1234567890"));
        assert!(display.contains("Verified bot"));
    }

    #[test]
    fn test_bot_verification_display_no_description() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, 1234567890, None);

        let display = format!("{}", verification);
        assert!(display.contains("12345"));
        assert!(display.contains("1234567890"));
        assert!(display.contains("no description"));
    }

    #[test]
    fn test_bot_verification_empty_description() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, 1234567890, Some(""));

        assert_eq!(verification.description(), Some(""));
        assert!(verification.is_valid());
    }

    #[test]
    fn test_bot_verification_negative_icon() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, -1234567890, None);

        assert_eq!(verification.icon(), -1234567890);
        assert!(verification.is_valid()); // Negative icons might be valid in some cases
    }

    #[test]
    fn test_bot_verification_large_icon() {
        let bot_id = UserId::new(12345).unwrap();
        let verification = BotVerification::new(bot_id, i64::MAX, None);

        assert_eq!(verification.icon(), i64::MAX);
        assert!(verification.is_valid());
    }

    #[test]
    fn test_bot_verification_update_description() {
        let bot_id = UserId::new(12345).unwrap();
        let mut verification = BotVerification::new(bot_id, 1234567890, None);

        verification.set_description("First");
        assert_eq!(verification.description(), Some("First"));

        verification.set_description("Second");
        assert_eq!(verification.description(), Some("Second"));

        verification.clear_description();
        assert_eq!(verification.description(), None);

        verification.set_description("Third");
        assert_eq!(verification.description(), Some("Third"));
    }

    #[test]
    fn test_bot_verification_zero_bot_id() {
        let verification = BotVerification::new(UserId(0), 1234567890, None);
        assert!(!verification.is_valid());
    }

    #[test]
    fn test_bot_verification_negative_bot_id() {
        let verification = BotVerification::new(UserId(-1), 1234567890, None);
        assert!(!verification.is_valid());
    }

    // ========== BotVerifierSettings Tests ==========

    #[test]
    fn test_bot_verifier_settings_new() {
        let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        assert_eq!(settings.icon(), 1234567890);
        assert_eq!(settings.company(), "Telegram");
        assert_eq!(settings.description(), "Official bot");
        assert!(!settings.can_modify_custom_description());
    }

    #[test]
    fn test_bot_verifier_settings_with_data() {
        let settings = BotVerifierSettings::with_data(1234567890, "Telegram", "Official bot", true);
        assert_eq!(settings.icon(), 1234567890);
        assert_eq!(settings.company(), "Telegram");
        assert_eq!(settings.description(), "Official bot");
        assert!(settings.can_modify_custom_description());
    }

    #[test]
    fn test_bot_verifier_settings_is_valid() {
        let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        assert!(settings.is_valid());

        let invalid = BotVerifierSettings::new(0, "Telegram", "Official bot");
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_bot_verifier_settings_icon() {
        let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        assert_eq!(settings.icon(), 1234567890);
    }

    #[test]
    fn test_bot_verifier_settings_company() {
        let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        assert_eq!(settings.company(), "Telegram");
    }

    #[test]
    fn test_bot_verifier_settings_description() {
        let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        assert_eq!(settings.description(), "Official bot");
    }

    #[test]
    fn test_bot_verifier_settings_can_modify_custom_description() {
        let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        assert!(!settings.can_modify_custom_description());
    }

    #[test]
    fn test_bot_verifier_settings_set_description() {
        let mut settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        settings.set_description("New description");
        assert_eq!(settings.description(), "New description");

        settings.set_description("Another description");
        assert_eq!(settings.description(), "Another description");
    }

    #[test]
    fn test_bot_verifier_settings_set_can_modify_custom_description() {
        let mut settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        assert!(!settings.can_modify_custom_description());

        settings.set_can_modify_custom_description(true);
        assert!(settings.can_modify_custom_description());

        settings.set_can_modify_custom_description(false);
        assert!(!settings.can_modify_custom_description());
    }

    #[test]
    fn test_bot_verifier_settings_to_td_api() {
        let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        let (icon, company, description, can_modify) = settings.to_td_api();
        assert_eq!(icon, 1234567890);
        assert_eq!(company, "Telegram");
        assert_eq!(description, "Official bot");
        assert!(!can_modify);
    }

    #[test]
    fn test_bot_verifier_settings_from_td_api() {
        let settings = BotVerifierSettings::from_td_api(
            1234567890,
            "Telegram".to_string(),
            "Official bot".to_string(),
            true,
        );
        assert_eq!(settings.icon(), 1234567890);
        assert_eq!(settings.company(), "Telegram");
        assert_eq!(settings.description(), "Official bot");
        assert!(settings.can_modify_custom_description());
    }

    #[test]
    fn test_bot_verifier_settings_default() {
        let settings = BotVerifierSettings::default();
        assert!(!settings.is_valid());
        assert_eq!(settings.icon(), 0);
        assert_eq!(settings.company(), "");
        assert_eq!(settings.description(), "");
        assert!(!settings.can_modify_custom_description());
    }

    #[test]
    fn test_bot_verifier_settings_equality() {
        let settings1 = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        let settings2 = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        assert_eq!(settings1, settings2);

        let settings3 = BotVerifierSettings::new(1234567890, "Telegram", "Different");
        assert_ne!(settings1, settings3);
    }

    #[test]
    fn test_bot_verifier_settings_clone() {
        let settings1 = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        let settings2 = settings1.clone();
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_bot_verifier_settings_display() {
        let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        let display = format!("{}", settings);
        assert!(display.contains("1234567890"));
        assert!(display.contains("Telegram"));
        assert!(display.contains("Official bot"));
    }

    #[test]
    fn test_bot_verifier_settings_serialization() {
        let settings = BotVerifierSettings::new(1234567890, "Telegram", "Official bot");
        let json = serde_json::to_string(&settings).unwrap();
        let parsed: BotVerifierSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, parsed);
    }

    #[test]
    fn test_bot_verifier_settings_empty_strings() {
        let settings = BotVerifierSettings::new(1234567890, "", "");
        assert!(settings.is_valid());
        assert_eq!(settings.company(), "");
        assert_eq!(settings.description(), "");
    }

    #[test]
    fn test_bot_verifier_settings_negative_icon() {
        let settings = BotVerifierSettings::new(-1234567890, "Telegram", "Official bot");
        assert!(settings.is_valid());
        assert_eq!(settings.icon(), -1234567890);
    }

    #[test]
    fn test_bot_verifier_settings_zero_icon() {
        let settings = BotVerifierSettings::new(0, "Telegram", "Official bot");
        assert!(!settings.is_valid());
    }

    #[test]
    fn test_bot_verifier_settings_max_icon() {
        let settings = BotVerifierSettings::new(i64::MAX, "Telegram", "Official bot");
        assert!(settings.is_valid());
        assert_eq!(settings.icon(), i64::MAX);
    }

    #[test]
    fn test_bot_verifier_settings_min_icon() {
        let settings = BotVerifierSettings::new(i64::MIN, "Telegram", "Official bot");
        assert!(settings.is_valid());
        assert_eq!(settings.icon(), i64::MIN);
    }
}
