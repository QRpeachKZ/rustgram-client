// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

use crate::{CustomEmojiId, PasskeyError, Result};
use std::fmt;

/// Builder for creating [`Passkey`] instances.
///
/// # Examples
///
/// ```
/// use rustgram_passkey::{Passkey, CustomEmojiId};
///
/// let passkey = Passkey::builder()
///     .with_id("credential-id".to_string())
///     .with_name("My Security Key".to_string())
///     .with_added_date(1704067200)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct PasskeyBuilder {
    id: Option<String>,
    name: Option<String>,
    added_date: Option<i32>,
    last_usage_date: Option<i32>,
    software_emoji_id: Option<CustomEmojiId>,
}

impl PasskeyBuilder {
    /// Creates a new passkey builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            name: None,
            added_date: None,
            last_usage_date: None,
            software_emoji_id: None,
        }
    }

    /// Sets the passkey credential ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique credential identifier (base64url-encoded)
    #[must_use]
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the passkey display name.
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable name for the passkey
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets when the passkey was added (Unix timestamp).
    ///
    /// # Arguments
    ///
    /// * `date` - Unix timestamp of when the passkey was registered
    #[must_use]
    pub fn with_added_date(mut self, date: i32) -> Self {
        self.added_date = Some(date);
        self
    }

    /// Sets the last usage date (Unix timestamp).
    ///
    /// # Arguments
    ///
    /// * `date` - Unix timestamp of last usage, or `None` if never used
    #[must_use]
    pub fn with_last_usage_date(mut self, date: Option<i32>) -> Self {
        self.last_usage_date = date;
        self
    }

    /// Sets the custom emoji ID for the software icon.
    ///
    /// # Arguments
    ///
    /// * `emoji_id` - Custom emoji identifier, or `None` for no icon
    #[must_use]
    pub fn with_software_emoji_id(mut self, emoji_id: Option<CustomEmojiId>) -> Self {
        self.software_emoji_id = emoji_id;
        self
    }

    /// Builds the passkey after validation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - ID is empty
    /// - Name is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::Passkey;
    ///
    /// let passkey = Passkey::builder()
    ///     .with_id("id".to_string())
    ///     .with_name("Key".to_string())
    ///     .with_added_date(1704067200)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Passkey> {
        let id = self.id.ok_or(PasskeyError::InvalidId)?;
        let name = self.name.ok_or(PasskeyError::InvalidName)?;
        let added_date = self.added_date.ok_or(PasskeyError::InvalidId)?;

        // Validate non-empty strings
        if id.is_empty() {
            return Err(PasskeyError::InvalidId);
        }
        if name.is_empty() {
            return Err(PasskeyError::InvalidName);
        }

        Ok(Passkey {
            id,
            name,
            added_date,
            last_usage_date: self.last_usage_date,
            software_emoji_id: self.software_emoji_id,
        })
    }
}

/// A WebAuthn passkey for Telegram authentication.
///
/// Contains metadata about a registered passkey including its credential ID,
/// display name, registration date, and optional usage tracking.
///
/// # Examples
///
/// ```
/// use rustgram_passkey::{Passkey, CustomEmojiId};
///
/// let passkey = Passkey::builder()
///     .with_id("credential-123".to_string())
///     .with_name("YubiKey 5".to_string())
///     .with_added_date(1704067200)
///     .with_software_emoji_id(Some(CustomEmojiId::new(12345)))
///     .build()
///     .unwrap();
///
/// assert_eq!(passkey.id(), "credential-123");
/// assert!(passkey.has_software_icon());
/// ```
#[derive(Debug, Clone)]
pub struct Passkey {
    id: String,
    name: String,
    added_date: i32,
    last_usage_date: Option<i32>,
    software_emoji_id: Option<CustomEmojiId>,
}

impl Passkey {
    /// Creates a new passkey builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::Passkey;
    ///
    /// let builder = Passkey::builder()
    ///     .with_id("id".to_string())
    ///     .with_name("Key".to_string())
    ///     .with_added_date(1704067200);
    /// ```
    #[must_use]
    pub const fn builder() -> PasskeyBuilder {
        PasskeyBuilder::new()
    }

    /// Returns the passkey credential ID.
    ///
    /// This is the base64url-encoded credential ID from WebAuthn.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::Passkey;
    ///
    /// let passkey = Passkey::builder()
    ///     .with_id("my-credential-id".to_string())
    ///     .with_name("Key".to_string())
    ///     .with_added_date(1704067200)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(passkey.id(), "my-credential-id");
    /// ```
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the passkey display name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::Passkey;
    ///
    /// let passkey = Passkey::builder()
    ///     .with_id("id".to_string())
    ///     .with_name("My YubiKey".to_string())
    ///     .with_added_date(1704067200)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(passkey.name(), "My YubiKey");
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns when the passkey was added (Unix timestamp).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::Passkey;
    ///
    /// let passkey = Passkey::builder()
    ///     .with_id("id".to_string())
    ///     .with_name("Key".to_string())
    ///     .with_added_date(1704067200)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(passkey.added_date(), 1704067200);
    /// ```
    #[must_use]
    pub const fn added_date(&self) -> i32 {
        self.added_date
    }

    /// Returns the last usage date (Unix timestamp), if ever used.
    ///
    /// Returns `None` if the passkey has never been used.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::Passkey;
    ///
    /// let passkey = Passkey::builder()
    ///     .with_id("id".to_string())
    ///     .with_name("Key".to_string())
    ///     .with_added_date(1704067200)
    ///     .with_last_usage_date(Some(1704153600))
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(passkey.last_usage_date(), Some(1704153600));
    /// ```
    #[must_use]
    pub const fn last_usage_date(&self) -> Option<i32> {
        self.last_usage_date
    }

    /// Returns `true` if this passkey has been used at least once.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::Passkey;
    ///
    /// let mut passkey = Passkey::builder()
    ///     .with_id("id".to_string())
    ///     .with_name("Key".to_string())
    ///     .with_added_date(1704067200)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(!passkey.has_been_used());
    ///
    /// passkey.update_last_usage(1704153600);
    /// assert!(passkey.has_been_used());
    /// ```
    #[must_use]
    pub const fn has_been_used(&self) -> bool {
        self.last_usage_date.is_some()
    }

    /// Returns the custom emoji ID for the software icon, if set.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::{Passkey, CustomEmojiId};
    ///
    /// let passkey = Passkey::builder()
    ///     .with_id("id".to_string())
    ///     .with_name("Key".to_string())
    ///     .with_added_date(1704067200)
    ///     .with_software_emoji_id(Some(CustomEmojiId::new(12345)))
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(passkey.software_emoji_id().unwrap().get(), 12345);
    /// ```
    #[must_use]
    pub const fn software_emoji_id(&self) -> Option<CustomEmojiId> {
        self.software_emoji_id
    }

    /// Returns `true` if this passkey has a software icon.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::{Passkey, CustomEmojiId};
    ///
    /// let passkey = Passkey::builder()
    ///     .with_id("id".to_string())
    ///     .with_name("Key".to_string())
    ///     .with_added_date(1704067200)
    ///     .with_software_emoji_id(Some(CustomEmojiId::new(12345)))
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(passkey.has_software_icon());
    /// ```
    #[must_use]
    pub const fn has_software_icon(&self) -> bool {
        self.software_emoji_id.is_some()
    }

    /// Updates the last usage date to the current timestamp.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Unix timestamp to set as last usage
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::Passkey;
    ///
    /// let mut passkey = Passkey::builder()
    ///     .with_id("id".to_string())
    ///     .with_name("Key".to_string())
    ///     .with_added_date(1704067200)
    ///     .build()
    ///     .unwrap();
    ///
    /// passkey.update_last_usage(1704153600);
    /// assert_eq!(passkey.last_usage_date(), Some(1704153600));
    /// ```
    pub fn update_last_usage(&mut self, timestamp: i32) {
        self.last_usage_date = Some(timestamp);
    }

    /// Sets or clears the software emoji icon.
    ///
    /// # Arguments
    ///
    /// * `emoji_id` - Custom emoji ID, or `None` to clear
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_passkey::{Passkey, CustomEmojiId};
    ///
    /// let mut passkey = Passkey::builder()
    ///     .with_id("id".to_string())
    ///     .with_name("Key".to_string())
    ///     .with_added_date(1704067200)
    ///     .build()
    ///     .unwrap();
    ///
    /// passkey.set_software_emoji_id(Some(CustomEmojiId::new(12345)));
    /// assert!(passkey.has_software_icon());
    ///
    /// passkey.set_software_emoji_id(None);
    /// assert!(!passkey.has_software_icon());
    /// ```
    pub fn set_software_emoji_id(&mut self, emoji_id: Option<CustomEmojiId>) {
        self.software_emoji_id = emoji_id;
    }
}

impl fmt::Display for Passkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Passkey(id={}, name={}, added={}, used={})",
            self.id,
            self.name,
            self.added_date,
            self.last_usage_date
                .map_or("never".to_string(), |d| d.to_string())
        )
    }
}

impl PartialEq for Passkey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.added_date == other.added_date
            && self.last_usage_date == other.last_usage_date
            && self.software_emoji_id == other.software_emoji_id
    }
}

impl Eq for Passkey {}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_builder_minimum() {
        let passkey = Passkey::builder()
            .with_id("id".to_string())
            .with_name("Key".to_string())
            .with_added_date(1704067200)
            .build()
            .unwrap();

        assert_eq!(passkey.id(), "id");
        assert_eq!(passkey.name(), "Key");
        assert_eq!(passkey.added_date(), 1704067200);
        assert!(!passkey.has_been_used());
        assert!(!passkey.has_software_icon());
    }

    #[test]
    fn test_builder_full() {
        let emoji = CustomEmojiId::new(12345);
        let passkey = Passkey::builder()
            .with_id("id".to_string())
            .with_name("Key".to_string())
            .with_added_date(1704067200)
            .with_last_usage_date(Some(1704153600))
            .with_software_emoji_id(Some(emoji))
            .build()
            .unwrap();

        assert!(passkey.has_been_used());
        assert_eq!(passkey.last_usage_date(), Some(1704153600));
        assert!(passkey.has_software_icon());
        assert_eq!(passkey.software_emoji_id().unwrap().get(), 12345);
    }

    #[test]
    fn test_builder_missing_id() {
        let result = Passkey::builder()
            .with_name("Key".to_string())
            .with_added_date(1704067200)
            .build();

        assert!(matches!(result, Err(PasskeyError::InvalidId)));
    }

    #[test]
    fn test_builder_missing_name() {
        let result = Passkey::builder()
            .with_id("id".to_string())
            .with_added_date(1704067200)
            .build();

        assert!(matches!(result, Err(PasskeyError::InvalidName)));
    }

    #[test]
    fn test_builder_missing_added_date() {
        let result = Passkey::builder()
            .with_id("id".to_string())
            .with_name("Key".to_string())
            .build();

        assert!(matches!(result, Err(PasskeyError::InvalidId)));
    }

    #[test]
    fn test_builder_empty_id() {
        let result = Passkey::builder()
            .with_id("".to_string())
            .with_name("Key".to_string())
            .with_added_date(1704067200)
            .build();

        assert!(matches!(result, Err(PasskeyError::InvalidId)));
    }

    #[test]
    fn test_builder_empty_name() {
        let result = Passkey::builder()
            .with_id("id".to_string())
            .with_name("".to_string())
            .with_added_date(1704067200)
            .build();

        assert!(matches!(result, Err(PasskeyError::InvalidName)));
    }

    #[test]
    fn test_update_last_usage() {
        let mut passkey = Passkey::builder()
            .with_id("id".to_string())
            .with_name("Key".to_string())
            .with_added_date(1704067200)
            .build()
            .unwrap();

        assert!(!passkey.has_been_used());
        assert_eq!(passkey.last_usage_date(), None);

        passkey.update_last_usage(1704153600);
        assert!(passkey.has_been_used());
        assert_eq!(passkey.last_usage_date(), Some(1704153600));
    }

    #[test]
    fn test_set_software_emoji_id() {
        let mut passkey = Passkey::builder()
            .with_id("id".to_string())
            .with_name("Key".to_string())
            .with_added_date(1704067200)
            .build()
            .unwrap();

        assert!(!passkey.has_software_icon());

        passkey.set_software_emoji_id(Some(CustomEmojiId::new(12345)));
        assert!(passkey.has_software_icon());
        assert_eq!(passkey.software_emoji_id().unwrap().get(), 12345);

        passkey.set_software_emoji_id(None);
        assert!(!passkey.has_software_icon());
    }

    #[test]
    fn test_equality() {
        let emoji1 = CustomEmojiId::new(12345);
        let emoji2 = CustomEmojiId::new(54321);

        let a = Passkey::builder()
            .with_id("id".to_string())
            .with_name("Key".to_string())
            .with_added_date(1704067200)
            .with_last_usage_date(Some(1704153600))
            .with_software_emoji_id(Some(emoji1))
            .build()
            .unwrap();

        let b = Passkey::builder()
            .with_id("id".to_string())
            .with_name("Key".to_string())
            .with_added_date(1704067200)
            .with_last_usage_date(Some(1704153600))
            .with_software_emoji_id(Some(emoji1))
            .build()
            .unwrap();

        let c = Passkey::builder()
            .with_id("id".to_string())
            .with_name("Key".to_string())
            .with_added_date(1704067200)
            .with_last_usage_date(Some(1704153600))
            .with_software_emoji_id(Some(emoji2))
            .build()
            .unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_display() {
        let passkey = Passkey::builder()
            .with_id("cred-123".to_string())
            .with_name("My Key".to_string())
            .with_added_date(1704067200)
            .with_last_usage_date(Some(1704153600))
            .build()
            .unwrap();

        let s = format!("{}", passkey);
        assert!(s.contains("cred-123"));
        assert!(s.contains("My Key"));
        assert!(s.contains("1704067200"));
        assert!(s.contains("1704153600"));
    }

    #[test]
    fn test_display_never_used() {
        let passkey = Passkey::builder()
            .with_id("id".to_string())
            .with_name("Key".to_string())
            .with_added_date(1704067200)
            .build()
            .unwrap();

        let s = format!("{}", passkey);
        assert!(s.contains("never"));
    }
}
