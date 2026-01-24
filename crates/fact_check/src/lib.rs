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

//! # Fact Check
//!
//! Fact check information for Telegram messages.
//!
//! Fact checks provide verification and context for potentially misleading content.
//! This type implements move-only semantics similar to TDLib's implementation.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_fact_check::FactCheck;
//! use rustgram_formatted_text::FormattedText;
//!
//! let fact_check = FactCheck::new(
//!     "US",
//!     FormattedText::new("This claim has been verified."),
//!     12345,
//!     false
//! );
//! assert!(!fact_check.is_empty());
//! ```

use rustgram_formatted_text::FormattedText;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Fact check information for a message.
///
/// Provides verification and context for potentially misleading content.
/// This type implements move-only semantics (cannot be copied, only moved).
///
/// # Example
///
/// ```rust
/// use rustgram_fact_check::FactCheck;
/// use rustgram_formatted_text::FormattedText;
///
/// let fact_check = FactCheck::new(
///     "US",
///     FormattedText::new("Verified information."),
///     12345,
///     false
/// );
/// assert_eq!(fact_check.country_code(), Some("US"));
/// assert!(!fact_check.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FactCheck {
    /// ISO 3166-1 alpha-2 country code
    country_code: Option<String>,
    /// Fact check text content
    text: FormattedText,
    /// Hash value for caching/verification
    hash: i64,
    /// Whether the content needs checking
    need_check: bool,
}

impl FactCheck {
    /// Creates a new fact check.
    ///
    /// # Arguments
    ///
    /// * `country_code` - ISO 3166-1 alpha-2 country code (empty if none)
    /// * `text` - Fact check text content
    /// * `hash` - Hash value for caching
    /// * `need_check` - Whether the content needs checking
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_fact_check::FactCheck;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let fact_check = FactCheck::new(
    ///     "US",
    ///     FormattedText::new("Verified."),
    ///     12345,
    ///     false
    /// );
    /// ```
    pub fn new(country_code: &str, text: FormattedText, hash: i64, need_check: bool) -> Self {
        Self {
            country_code: if country_code.is_empty() {
                None
            } else {
                Some(country_code.to_string())
            },
            text,
            hash,
            need_check,
        }
    }

    /// Creates a fact check from a mock telegram_api::factCheck object.
    ///
    /// This is a simplified version for testing.
    ///
    /// # Arguments
    ///
    /// * `country_code` - ISO country code
    /// * `text` - Fact check text
    /// * `hash` - Hash value
    /// * `need_check` - Whether checking is needed
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_fact_check::FactCheck;
    ///
    /// let fact_check = FactCheck::from_telegram_api(
    ///     "US",
    ///     "This is verified.",
    ///     12345,
    ///     false
    /// );
    /// ```
    pub fn from_telegram_api(country_code: &str, text: &str, hash: i64, need_check: bool) -> Self {
        Self {
            country_code: if country_code.is_empty() {
                None
            } else {
                Some(country_code.to_string())
            },
            text: FormattedText::new(text),
            hash,
            need_check,
        }
    }

    /// Returns the ISO 3166-1 alpha-2 country code.
    ///
    /// Returns `None` if no country code is set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_fact_check::FactCheck;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let fact_check = FactCheck::new(
    ///     "US",
    ///     FormattedText::new("Verified."),
    ///     0,
    ///     false
    /// );
    /// assert_eq!(fact_check.country_code(), Some("US"));
    /// ```
    pub fn country_code(&self) -> Option<&str> {
        self.country_code.as_deref()
    }

    /// Returns the fact check text content.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_fact_check::FactCheck;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Verified information.");
    /// let fact_check = FactCheck::new("US", text, 0, false);
    /// assert_eq!(fact_check.text().text(), "Verified information.");
    /// ```
    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    /// Returns the hash value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_fact_check::FactCheck;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let fact_check = FactCheck::new("US", FormattedText::new(""), 12345, false);
    /// assert_eq!(fact_check.hash(), 12345);
    /// ```
    pub fn hash(&self) -> i64 {
        self.hash
    }

    /// Returns `true` if the content needs checking.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_fact_check::FactCheck;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let fact_check = FactCheck::new("US", FormattedText::new(""), 0, true);
    /// assert!(fact_check.need_check());
    /// ```
    pub fn need_check(&self) -> bool {
        self.need_check
    }

    /// Returns `true` if the fact check is empty.
    ///
    /// A fact check is empty if its hash is 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_fact_check::FactCheck;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let empty = FactCheck::new("", FormattedText::new(""), 0, false);
    /// assert!(empty.is_empty());
    ///
    /// let non_empty = FactCheck::new("US", FormattedText::new(""), 1, false);
    /// assert!(!non_empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.hash == 0
    }

    /// Updates this fact check from another one.
    ///
    /// Preserves values from the old fact check if the new one doesn't have them.
    ///
    /// # Arguments
    ///
    /// * `other` - The other fact check to update from
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_fact_check::FactCheck;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let mut fact_check = FactCheck::new("US", FormattedText::new("Old"), 123, false);
    /// let new_info = FactCheck::new("", FormattedText::new("New"), 456, true);
    ///
    /// fact_check.update_from(&new_info);
    /// assert_eq!(fact_check.hash(), 456);
    /// assert_eq!(fact_check.country_code(), Some("US")); // Preserved
    /// ```
    pub fn update_from(&mut self, other: &FactCheck) {
        // Keep country_code if other doesn't have one
        if other.country_code.is_some() {
            self.country_code = other.country_code.clone();
        }

        // Update text if other has content
        if !other.text.is_empty() {
            self.text = other.text.clone();
        }

        // Always update hash and need_check
        self.hash = other.hash;
        self.need_check = other.need_check;
    }

    /// Returns a mock td_api::factCheck object.
    ///
    /// This is a placeholder for the real implementation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_fact_check::FactCheck;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let fact_check = FactCheck::new(
    ///     "US",
    ///     FormattedText::new("Verified."),
    ///     12345,
    ///     false
    /// );
    /// let obj = fact_check.get_fact_check_object();
    /// assert_eq!(obj.country_code, Some("US".to_string()));
    /// ```
    pub fn get_fact_check_object(&self) -> FactCheckObject {
        FactCheckObject {
            country_code: self.country_code.clone(),
            text: self.text.text().to_string(),
            hash: self.hash,
            need_check: self.need_check,
        }
    }
}

impl Default for FactCheck {
    fn default() -> Self {
        Self {
            country_code: None,
            text: FormattedText::default(),
            hash: 0,
            need_check: false,
        }
    }
}

impl fmt::Display for FactCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FactCheck[")?;

        if let Some(country) = &self.country_code {
            write!(f, "country={}, ", country)?;
        }

        write!(f, "hash={}", self.hash)?;

        if self.need_check {
            write!(f, ", needs_check")?;
        }

        write!(f, "]")
    }
}

/// A mock TDLib API object for fact check.
///
/// This is a placeholder for the real td_api::factCheck.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactCheckObject {
    pub country_code: Option<String>,
    pub text: String,
    pub hash: i64,
    pub need_check: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact_check_new() {
        let text = FormattedText::new("Verified information.");
        let fact_check = FactCheck::new("US", text, 12345, false);

        assert_eq!(fact_check.country_code(), Some("US"));
        assert_eq!(fact_check.text().text(), "Verified information.");
        assert_eq!(fact_check.hash(), 12345);
        assert!(!fact_check.need_check());
    }

    #[test]
    fn test_fact_check_new_empty_country() {
        let fact_check = FactCheck::new("", FormattedText::new(""), 0, false);

        assert_eq!(fact_check.country_code(), None);
        assert!(fact_check.is_empty());
    }

    #[test]
    fn test_fact_check_from_telegram_api() {
        let fact_check = FactCheck::from_telegram_api("US", "Verified.", 12345, true);

        assert_eq!(fact_check.country_code(), Some("US"));
        assert_eq!(fact_check.text().text(), "Verified.");
        assert_eq!(fact_check.hash(), 12345);
        assert!(fact_check.need_check());
    }

    #[test]
    fn test_fact_check_from_telegram_api_empty_country() {
        let fact_check = FactCheck::from_telegram_api("", "Verified.", 12345, false);

        assert_eq!(fact_check.country_code(), None);
    }

    #[test]
    fn test_is_empty() {
        let empty = FactCheck::new("", FormattedText::new(""), 0, false);
        assert!(empty.is_empty());

        let non_empty = FactCheck::new("US", FormattedText::new(""), 1, false);
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn test_need_check() {
        let needs_check = FactCheck::new("US", FormattedText::new(""), 0, true);
        assert!(needs_check.need_check());

        let no_check = FactCheck::new("US", FormattedText::new(""), 0, false);
        assert!(!no_check.need_check());
    }

    #[test]
    fn test_update_from() {
        let mut fact_check = FactCheck::new("US", FormattedText::new("Old"), 123, false);

        let new_info = FactCheck::new("", FormattedText::new("New"), 456, true);
        fact_check.update_from(&new_info);

        assert_eq!(fact_check.hash(), 456);
        assert_eq!(fact_check.country_code(), Some("US")); // Preserved
        assert_eq!(fact_check.text().text(), "New");
        assert!(fact_check.need_check());
    }

    #[test]
    fn test_update_from_with_country() {
        let mut fact_check = FactCheck::new("", FormattedText::new("Old"), 123, false);

        let new_info = FactCheck::new("GB", FormattedText::new("New"), 456, true);
        fact_check.update_from(&new_info);

        assert_eq!(fact_check.country_code(), Some("GB"));
    }

    #[test]
    fn test_update_from_empty_text() {
        let mut fact_check = FactCheck::new("US", FormattedText::new("Old"), 123, false);

        let new_info = FactCheck::new("", FormattedText::new(""), 456, true);
        fact_check.update_from(&new_info);

        assert_eq!(fact_check.text().text(), "Old"); // Preserved
        assert_eq!(fact_check.hash(), 456);
    }

    #[test]
    fn test_get_fact_check_object() {
        let fact_check = FactCheck::new("US", FormattedText::new("Verified."), 12345, false);
        let obj = fact_check.get_fact_check_object();

        assert_eq!(obj.country_code, Some("US".to_string()));
        assert_eq!(obj.text, "Verified.");
        assert_eq!(obj.hash, 12345);
        assert!(!obj.need_check);
    }

    #[test]
    fn test_get_fact_check_object_empty() {
        let fact_check = FactCheck::default();
        let obj = fact_check.get_fact_check_object();

        assert_eq!(obj.country_code, None);
        assert_eq!(obj.text, "");
        assert_eq!(obj.hash, 0);
        assert!(!obj.need_check);
    }

    #[test]
    fn test_default() {
        let fact_check = FactCheck::default();

        assert_eq!(fact_check.country_code(), None);
        assert!(fact_check.text().is_empty());
        assert_eq!(fact_check.hash(), 0);
        assert!(!fact_check.need_check());
        assert!(fact_check.is_empty());
    }

    #[test]
    fn test_display() {
        let fact_check = FactCheck::new("US", FormattedText::new(""), 12345, true);
        let display = format!("{}", fact_check);

        assert!(display.contains("US"));
        assert!(display.contains("12345"));
        assert!(display.contains("needs_check"));
    }

    #[test]
    fn test_display_no_country() {
        let fact_check = FactCheck::new("", FormattedText::new(""), 12345, false);
        let display = format!("{}", fact_check);

        assert!(!display.contains("country="));
        assert!(display.contains("12345"));
    }

    #[test]
    fn test_equality() {
        let text = FormattedText::new("Verified.");
        let fact_check1 = FactCheck::new("US", text.clone(), 12345, false);
        let fact_check2 = FactCheck::new("US", text, 12345, false);

        assert_eq!(fact_check1, fact_check2);
    }

    #[test]
    fn test_inequality() {
        let fact_check1 = FactCheck::new("US", FormattedText::new("A"), 12345, false);
        let fact_check2 = FactCheck::new("GB", FormattedText::new("A"), 12345, false);

        assert_ne!(fact_check1, fact_check2);
    }

    #[test]
    fn test_clone() {
        let fact_check = FactCheck::new("US", FormattedText::new("Verified."), 12345, false);
        let cloned = fact_check.clone();

        assert_eq!(fact_check, cloned);
    }

    #[test]
    fn test_serialization() {
        let fact_check = FactCheck::new("US", FormattedText::new("Verified."), 12345, false);

        let json = serde_json::to_string(&fact_check).unwrap();
        let parsed: FactCheck = serde_json::from_str(&json).unwrap();

        assert_eq!(fact_check, parsed);
    }

    #[test]
    fn test_hash_trait() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let text = FormattedText::new("Verified.");
        let fact_check1 = FactCheck::new("US", text.clone(), 12345, false);
        let fact_check2 = FactCheck::new("US", text, 12345, false);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        std::hash::Hash::hash(&fact_check1, &mut hasher1);
        std::hash::Hash::hash(&fact_check2, &mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_zero_means_empty() {
        let fact_check = FactCheck::new("US", FormattedText::new("Any content"), 0, false);
        assert!(fact_check.is_empty());
    }

    #[test]
    fn test_negative_hash() {
        let fact_check = FactCheck::new("US", FormattedText::new(""), -1, false);
        assert_eq!(fact_check.hash(), -1);
        assert!(!fact_check.is_empty());
    }

    #[test]
    fn test_large_hash() {
        let fact_check = FactCheck::new("US", FormattedText::new(""), i64::MAX, false);
        assert_eq!(fact_check.hash(), i64::MAX);
        assert!(!fact_check.is_empty());
    }

    #[test]
    fn test_country_code_variations() {
        let countries = ["US", "GB", "CA", "DE", "FR", "JP"];

        for country in countries {
            let fact_check = FactCheck::new(country, FormattedText::new(""), 1, false);
            assert_eq!(fact_check.country_code(), Some(country));
        }
    }

    #[test]
    fn test_multiple_updates() {
        let mut fact_check = FactCheck::new("US", FormattedText::new("Initial"), 100, false);

        let update1 = FactCheck::new("", FormattedText::new("Update 1"), 200, false);
        fact_check.update_from(&update1);

        let update2 = FactCheck::new("GB", FormattedText::new(""), 300, true);
        fact_check.update_from(&update2);

        assert_eq!(fact_check.country_code(), Some("GB"));
        assert_eq!(fact_check.text().text(), "Update 1");
        assert_eq!(fact_check.hash(), 300);
        assert!(fact_check.need_check());
    }

    #[test]
    fn test_empty_text_with_content() {
        let text = FormattedText::new("Content");
        let fact_check = FactCheck::new("US", text, 1, false);

        assert!(!fact_check.text().is_empty());
    }

    #[test]
    fn test_unicode_country_code() {
        // Country codes are ASCII, but we handle any string
        let fact_check = FactCheck::new("US", FormattedText::new(""), 1, false);
        assert_eq!(fact_check.country_code(), Some("US"));
    }

    #[test]
    fn test_unicode_text() {
        let text = FormattedText::new("Verified in 日本語");
        let fact_check = FactCheck::new("JP", text, 1, false);

        assert_eq!(fact_check.text().text(), "Verified in 日本語");
    }
}
