// rustgram_star_gift_settings
// Copyright (C) 2025 rustgram-client contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # Star Gift Settings
//!
//! Settings for controlling star gifts display and restrictions.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_star_gift_settings::{StarGiftSettings, DisallowedGiftsSettings};
//!
//! let settings = StarGiftSettings::new();
//! assert!(!settings.display_gifts_button());
//! assert!(settings.is_default());
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Settings for which types of gifts are disallowed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisallowedGiftsSettings {
    /// Disallow unlimited star gifts
    #[serde(default)]
    pub disallow_unlimited_stargifts: bool,
    /// Disallow limited star gifts
    #[serde(default)]
    pub disallow_limited_stargifts: bool,
    /// Disallow unique star gifts
    #[serde(default)]
    pub disallow_unique_stargifts: bool,
    /// Disallow premium gifts
    #[serde(default)]
    pub disallow_premium_gifts: bool,
    /// Disallow gifts from channels
    #[serde(default)]
    pub disallow_gifts_from_channels: bool,
}

impl Default for DisallowedGiftsSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl DisallowedGiftsSettings {
    /// Creates a new disallowed gifts settings with all flags set to false.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::DisallowedGiftsSettings;
    ///
    /// let settings = DisallowedGiftsSettings::new();
    /// assert!(settings.is_default());
    /// assert!(!settings.disallow_unlimited_stargifts());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            disallow_unlimited_stargifts: false,
            disallow_limited_stargifts: false,
            disallow_unique_stargifts: false,
            disallow_premium_gifts: false,
            disallow_gifts_from_channels: false,
        }
    }

    /// Creates a settings object that disallows all gift types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::DisallowedGiftsSettings;
    ///
    /// let settings = DisallowedGiftsSettings::allow_nothing();
    /// assert!(!settings.is_default());
    /// assert!(settings.disallow_unlimited_stargifts());
    /// assert!(settings.disallow_limited_stargifts());
    /// ```
    #[must_use]
    pub fn allow_nothing() -> Self {
        Self {
            disallow_unlimited_stargifts: true,
            disallow_limited_stargifts: true,
            disallow_unique_stargifts: true,
            disallow_premium_gifts: true,
            disallow_gifts_from_channels: true,
        }
    }

    /// Returns whether unlimited star gifts are disallowed.
    #[must_use]
    pub const fn disallow_unlimited_stargifts(&self) -> bool {
        self.disallow_unlimited_stargifts
    }

    /// Returns whether limited star gifts are disallowed.
    #[must_use]
    pub const fn disallow_limited_stargifts(&self) -> bool {
        self.disallow_limited_stargifts
    }

    /// Returns whether unique star gifts are disallowed.
    #[must_use]
    pub const fn disallow_unique_stargifts(&self) -> bool {
        self.disallow_unique_stargifts
    }

    /// Returns whether premium gifts are disallowed.
    #[must_use]
    pub const fn disallow_premium_gifts(&self) -> bool {
        self.disallow_premium_gifts
    }

    /// Returns whether gifts from channels are disallowed.
    #[must_use]
    pub const fn disallow_gifts_from_channels(&self) -> bool {
        self.disallow_gifts_from_channels
    }

    /// Checks if all settings are at their default values (all false).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::DisallowedGiftsSettings;
    ///
    /// let settings = DisallowedGiftsSettings::new();
    /// assert!(settings.is_default());
    ///
    /// let settings2 = DisallowedGiftsSettings::allow_nothing();
    /// assert!(!settings2.is_default());
    /// ```
    #[must_use]
    pub fn is_default(&self) -> bool {
        !self.disallow_unlimited_stargifts
            && !self.disallow_limited_stargifts
            && !self.disallow_unique_stargifts
            && !self.disallow_premium_gifts
            && !self.disallow_gifts_from_channels
    }

    /// Checks if any gift type is disallowed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::DisallowedGiftsSettings;
    ///
    /// let settings = DisallowedGiftsSettings::new();
    /// assert!(!settings.has_restrictions());
    ///
    /// let mut settings2 = settings.clone();
    /// settings2.disallow_unlimited_stargifts = true;
    /// assert!(settings2.has_restrictions());
    /// ```
    #[must_use]
    pub fn has_restrictions(&self) -> bool {
        self.disallow_unlimited_stargifts
            || self.disallow_limited_stargifts
            || self.disallow_unique_stargifts
            || self.disallow_premium_gifts
            || self.disallow_gifts_from_channels
    }

    /// Returns the count of restricted gift types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::DisallowedGiftsSettings;
    ///
    /// let settings = DisallowedGiftsSettings::new();
    /// assert_eq!(settings.restriction_count(), 0);
    ///
    /// let settings2 = DisallowedGiftsSettings::allow_nothing();
    /// assert_eq!(settings2.restriction_count(), 5);
    /// ```
    #[must_use]
    pub fn restriction_count(&self) -> usize {
        let mut count = 0;
        if self.disallow_unlimited_stargifts {
            count += 1;
        }
        if self.disallow_limited_stargifts {
            count += 1;
        }
        if self.disallow_unique_stargifts {
            count += 1;
        }
        if self.disallow_premium_gifts {
            count += 1;
        }
        if self.disallow_gifts_from_channels {
            count += 1;
        }
        count
    }
}

/// Settings for star gifts display and acceptance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarGiftSettings {
    /// Whether to display the gifts button
    display_gifts_button: bool,
    /// Settings for disallowed gift types
    disallowed_gifts: DisallowedGiftsSettings,
}

impl Default for StarGiftSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl StarGiftSettings {
    /// Creates a new star gift settings with default values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::StarGiftSettings;
    ///
    /// let settings = StarGiftSettings::new();
    /// assert!(!settings.display_gifts_button());
    /// assert!(settings.disallowed_gifts().is_default());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            display_gifts_button: false,
            disallowed_gifts: DisallowedGiftsSettings::new(),
        }
    }

    /// Creates a new star gift settings with the specified values.
    ///
    /// # Arguments
    ///
    /// * `display_gifts_button` - Whether to display the gifts button
    /// * `disallowed_gifts` - Settings for disallowed gift types
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::{StarGiftSettings, DisallowedGiftsSettings};
    ///
    /// let disallowed = DisallowedGiftsSettings::allow_nothing();
    /// let settings = StarGiftSettings::with_values(true, disallowed);
    /// assert!(settings.display_gifts_button());
    /// ```
    #[must_use]
    pub fn with_values(
        display_gifts_button: bool,
        disallowed_gifts: DisallowedGiftsSettings,
    ) -> Self {
        Self {
            display_gifts_button,
            disallowed_gifts,
        }
    }

    /// Creates settings that disallow all gifts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::StarGiftSettings;
    ///
    /// let settings = StarGiftSettings::allow_nothing();
    /// assert!(!settings.display_gifts_button());
    /// assert!(settings.disallowed_gifts().has_restrictions());
    /// ```
    #[must_use]
    pub fn allow_nothing() -> Self {
        Self {
            display_gifts_button: false,
            disallowed_gifts: DisallowedGiftsSettings::allow_nothing(),
        }
    }

    /// Returns whether to display the gifts button.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::StarGiftSettings;
    ///
    /// let settings = StarGiftSettings::new();
    /// assert!(!settings.display_gifts_button());
    /// ```
    #[must_use]
    pub const fn display_gifts_button(&self) -> bool {
        self.display_gifts_button
    }

    /// Returns a reference to the disallowed gifts settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::StarGiftSettings;
    ///
    /// let settings = StarGiftSettings::new();
    /// assert!(!settings.disallowed_gifts().has_restrictions());
    /// ```
    #[must_use]
    pub const fn disallowed_gifts(&self) -> &DisallowedGiftsSettings {
        &self.disallowed_gifts
    }

    /// Checks if settings are at default values.
    ///
    /// Default means:
    /// - `display_gifts_button` is false
    /// - `disallowed_gifts` is default (all flags false)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_gift_settings::StarGiftSettings;
    ///
    /// let settings = StarGiftSettings::new();
    /// assert!(settings.is_default());
    /// ```
    #[must_use]
    pub fn is_default(&self) -> bool {
        !self.display_gifts_button && self.disallowed_gifts.is_default()
    }
}

impl fmt::Display for DisallowedGiftsSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_default() {
            return write!(f, "[all gifts allowed]");
        }

        let mut parts = Vec::new();
        if self.disallow_unlimited_stargifts {
            parts.push("unlimited stargifts");
        }
        if self.disallow_limited_stargifts {
            parts.push("limited stargifts");
        }
        if self.disallow_unique_stargifts {
            parts.push("unique stargifts");
        }
        if self.disallow_premium_gifts {
            parts.push("premium gifts");
        }
        if self.disallow_gifts_from_channels {
            parts.push("gifts from channels");
        }

        write!(f, "[disallow: {}]", parts.join(", "))
    }
}

impl fmt::Display for StarGiftSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if self.display_gifts_button {
            parts.push("(show button)");
        }

        let prefix = if parts.is_empty() { "" } else { " " };
        write!(f, "{}{}{}", parts.join(" "), prefix, self.disallowed_gifts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disallowed_gifts_default() {
        let settings = DisallowedGiftsSettings::default();
        assert!(settings.is_default());
        assert!(!settings.disallow_unlimited_stargifts());
        assert!(!settings.disallow_limited_stargifts());
        assert!(!settings.disallow_unique_stargifts());
        assert!(!settings.disallow_premium_gifts());
        assert!(!settings.disallow_gifts_from_channels());
    }

    #[test]
    fn test_disallowed_gifts_new() {
        let settings = DisallowedGiftsSettings::new();
        assert!(settings.is_default());
        assert_eq!(settings.restriction_count(), 0);
    }

    #[test]
    fn test_disallowed_gifts_allow_nothing() {
        let settings = DisallowedGiftsSettings::allow_nothing();
        assert!(!settings.is_default());
        assert!(settings.disallow_unlimited_stargifts());
        assert!(settings.disallow_limited_stargifts());
        assert!(settings.disallow_unique_stargifts());
        assert!(settings.disallow_premium_gifts());
        assert!(settings.disallow_gifts_from_channels());
        assert_eq!(settings.restriction_count(), 5);
    }

    #[test]
    fn test_disallowed_gifts_has_restrictions() {
        let settings = DisallowedGiftsSettings::new();
        assert!(!settings.has_restrictions());

        let mut settings2 = settings.clone();
        settings2.disallow_unlimited_stargifts = true;
        assert!(settings2.has_restrictions());
    }

    #[test]
    fn test_disallowed_gifts_restriction_count() {
        let settings = DisallowedGiftsSettings::new();
        assert_eq!(settings.restriction_count(), 0);

        let mut settings2 = settings.clone();
        settings2.disallow_unlimited_stargifts = true;
        assert_eq!(settings2.restriction_count(), 1);

        settings2.disallow_limited_stargifts = true;
        assert_eq!(settings2.restriction_count(), 2);
    }

    #[test]
    fn test_star_gift_settings_default() {
        let settings = StarGiftSettings::default();
        assert!(settings.is_default());
        assert!(!settings.display_gifts_button());
        assert!(settings.disallowed_gifts().is_default());
    }

    #[test]
    fn test_star_gift_settings_new() {
        let settings = StarGiftSettings::new();
        assert!(settings.is_default());
    }

    #[test]
    fn test_star_gift_settings_with_values() {
        let disallowed = DisallowedGiftsSettings::allow_nothing();
        let settings = StarGiftSettings::with_values(true, disallowed);
        assert!(settings.display_gifts_button());
        assert!(settings.disallowed_gifts().has_restrictions());
    }

    #[test]
    fn test_star_gift_settings_allow_nothing() {
        let settings = StarGiftSettings::allow_nothing();
        assert!(!settings.display_gifts_button());
        assert!(settings.disallowed_gifts().has_restrictions());
    }

    #[test]
    fn test_star_gift_settings_is_default() {
        let settings = StarGiftSettings::new();
        assert!(settings.is_default());

        let settings2 = StarGiftSettings::with_values(true, DisallowedGiftsSettings::new());
        assert!(!settings2.is_default());

        let settings3 =
            StarGiftSettings::with_values(false, DisallowedGiftsSettings::allow_nothing());
        assert!(!settings3.is_default());
    }

    #[test]
    fn test_display_disallowed_gifts_default() {
        let settings = DisallowedGiftsSettings::new();
        assert_eq!(format!("{}", settings), "[all gifts allowed]");
    }

    #[test]
    fn test_display_disallowed_gifts_with_restrictions() {
        let mut settings = DisallowedGiftsSettings::new();
        settings.disallow_unlimited_stargifts = true;
        settings.disallow_premium_gifts = true;
        let display = format!("{}", settings);
        assert!(display.contains("unlimited stargifts"));
        assert!(display.contains("premium gifts"));
    }

    #[test]
    fn test_display_star_gift_settings() {
        let settings = StarGiftSettings::new();
        assert_eq!(format!("{}", settings), "[all gifts allowed]");

        let settings2 = StarGiftSettings::with_values(true, DisallowedGiftsSettings::new());
        assert_eq!(
            format!("{}", settings2),
            "(show button) [all gifts allowed]"
        );
    }

    #[test]
    fn test_equality_disallowed_gifts() {
        let a = DisallowedGiftsSettings::new();
        let b = DisallowedGiftsSettings::new();
        assert_eq!(a, b);

        let mut c = DisallowedGiftsSettings::new();
        c.disallow_unlimited_stargifts = true;
        assert_ne!(a, c);
    }

    #[test]
    fn test_equality_star_gift_settings() {
        let a = StarGiftSettings::new();
        let b = StarGiftSettings::new();
        assert_eq!(a, b);

        let c = StarGiftSettings::with_values(true, DisallowedGiftsSettings::new());
        assert_ne!(a, c);
    }

    #[test]
    fn test_cloning() {
        let disallowed = DisallowedGiftsSettings::allow_nothing();
        let settings1 = StarGiftSettings::with_values(true, disallowed);
        let settings2 = settings1.clone();
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_serialization_disallowed_gifts() {
        let settings = DisallowedGiftsSettings::allow_nothing();
        let json = serde_json::to_string(&settings).unwrap();
        let parsed: DisallowedGiftsSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, parsed);
    }

    #[test]
    fn test_serialization_partial() {
        let mut settings = DisallowedGiftsSettings::new();
        settings.disallow_unlimited_stargifts = true;
        settings.disallow_premium_gifts = true;

        let json = serde_json::to_string(&settings).unwrap();
        let parsed: DisallowedGiftsSettings = serde_json::from_str(&json).unwrap();

        assert!(parsed.disallow_unlimited_stargifts());
        assert!(parsed.disallow_premium_gifts());
        assert!(!parsed.disallow_limited_stargifts());
    }

    #[test]
    fn test_display_gifts_button_getter() {
        let settings = StarGiftSettings::new();
        assert!(!settings.display_gifts_button());

        let settings2 = StarGiftSettings::with_values(true, DisallowedGiftsSettings::new());
        assert!(settings2.display_gifts_button());
    }

    #[test]
    fn test_disallowed_gifts_getters() {
        let settings = DisallowedGiftsSettings::allow_nothing();
        assert!(settings.disallow_unlimited_stargifts());
        assert!(settings.disallow_limited_stargifts());
        assert!(settings.disallow_unique_stargifts());
        assert!(settings.disallow_premium_gifts());
        assert!(settings.disallow_gifts_from_channels());
    }

    #[test]
    fn test_disallowed_gifts_reference() {
        let settings = StarGiftSettings::new();
        let disallowed = settings.disallowed_gifts();
        assert!(disallowed.is_default());
    }

    #[test]
    fn test_combination_restrictions() {
        let mut settings = DisallowedGiftsSettings::new();
        settings.disallow_unlimited_stargifts = true;
        settings.disallow_limited_stargifts = true;
        settings.disallow_unique_stargifts = false;
        settings.disallow_premium_gifts = true;
        settings.disallow_gifts_from_channels = false;

        assert_eq!(settings.restriction_count(), 3);
        assert!(settings.has_restrictions());
        assert!(!settings.is_default());
    }

    #[test]
    fn test_all_restrictions() {
        let settings = DisallowedGiftsSettings::allow_nothing();
        assert_eq!(settings.restriction_count(), 5);
        assert!(settings.has_restrictions());
        assert!(!settings.is_default());

        let display = format!("{}", settings);
        assert!(display.contains("unlimited stargifts"));
        assert!(display.contains("limited stargifts"));
        assert!(display.contains("unique stargifts"));
        assert!(display.contains("premium gifts"));
        assert!(display.contains("gifts from channels"));
    }
}
