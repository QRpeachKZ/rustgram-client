// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Rustgram Reaction Type
//!
//! Reaction type representation for Telegram MTProto client.
//!
//! This crate provides type-safe representations of Telegram's reaction types,
//! including standard emoji reactions, custom emoji reactions, and paid reactions.
//!
//! ## Overview
//!
//! Telegram supports three types of reactions:
//!
//! - **Emoji reactions**: Standard Unicode emoji (e.g., "👍", "❤️", "😂")
//! - **Custom emoji reactions**: Custom sticker-based emoji (stored as base64-encoded document IDs)
//! - **Paid reactions**: Telegram Premium stars/reactions (stored as "$")
//!
//! ## Representation
//!
//! Internally, reactions are stored as strings with specific prefixes:
//! - Plain emoji: stored directly (e.g., "👍")
//! - Custom emoji: prefixed with "#" and base64-encoded (e.g., "#AAECAwQ=")
//! - Paid reactions: stored as "$"
//!
//! ## Examples
//!
//! Creating emoji reactions:
//!
//! ```rust
//! use rustgram_reaction_type::ReactionType;
//!
//! // Standard emoji reaction
//! let thumbs_up = ReactionType::emoji("👍");
//! assert!(thumbs_up.is_emoji());
//! assert_eq!(thumbs_up.as_str(), "👍");
//!
//! // Multiple emoji reactions
//! let heart = ReactionType::emoji("❤️");
//! let laugh = ReactionType::emoji("😂");
//! ```
//!
//! Creating custom emoji reactions:
//!
//! ```rust
//! use rustgram_reaction_type::ReactionType;
//!
//! // From raw bytes (document ID)
//! let doc_id = vec![0x00, 0x01, 0x02, 0x03];
//! let custom = ReactionType::custom_emoji(&doc_id);
//! assert!(custom.is_custom());
//! assert!(custom.as_str().starts_with('#'));
//!
//! // From base64-encoded string
//! let custom_b64 = ReactionType::custom_emoji_from_base64("AAECAwQ=");
//! assert!(custom_b64.is_custom());
//! ```
//!
//! Creating paid reactions:
//!
//! ```rust
//! use rustgram_reaction_type::ReactionType;
//!
//! let paid = ReactionType::paid();
//! assert!(paid.is_paid());
//! assert_eq!(paid.as_str(), "$");
//! ```
//!
//! Parsing reactions:
//!
//! ```rust
//! use rustgram_reaction_type::ReactionType;
//!
//! // Parse from string
//! let emoji = ReactionType::parse("👍").unwrap();
//! assert!(emoji.is_emoji());
//!
//! let custom = ReactionType::parse("#AAECAwQ=").unwrap();
//! assert!(custom.is_custom());
//!
//! let paid = ReactionType::parse("$").unwrap();
//! assert!(paid.is_paid());
//!
//! // Invalid input returns None
//! assert!(ReactionType::parse("").is_none());
//! ```
//!
//! ## MTProto Alignment
//!
//! This implementation aligns with TDLib's `td::td_api::reactionType`:
//! - `reactionTypeEmoji`: Maps to `ReactionType::emoji()`
//! - `reactionTypeCustomEmoji`: Maps to `ReactionType::custom_emoji()`
//! - `reactionTypePaid`: Maps to `ReactionType::paid()`

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]

mod error;

pub use error::ReactionError;

/// Reaction type for Telegram messages.
///
/// Represents three types of reactions:
/// - Standard emoji reactions (plain Unicode emoji)
/// - Custom emoji reactions (base64-encoded document IDs)
/// - Paid reactions (Telegram Premium stars)
///
/// # Internal Representation
///
/// - Emoji: stored directly as the emoji string (e.g., "👍", "❤️")
/// - Custom emoji: "#" + base64(document_id)
/// - Paid: "$"
///
/// # Examples
///
/// ```
/// use rustgram_reaction_type::ReactionType;
///
/// // Create an emoji reaction
/// let thumbs_up = ReactionType::emoji("👍");
/// assert!(thumbs_up.is_emoji());
/// assert_eq!(thumbs_up.as_str(), "👍");
///
/// // Create a custom emoji reaction
/// let custom = ReactionType::custom_emoji(&[0x00, 0x01, 0x02, 0x03]);
/// assert!(custom.is_custom());
/// assert!(custom.as_str().starts_with('#'));
///
/// // Create a paid reaction
/// let paid = ReactionType::paid();
/// assert!(paid.is_paid());
/// assert_eq!(paid.as_str(), "$");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReactionType {
    /// Internal string representation.
    ///
    /// - Plain emoji: the emoji itself
    /// - Custom emoji: "#" + base64(document_id)
    /// - Paid: "$"
    inner: String,
}

impl ReactionType {
    /// Creates a new emoji reaction.
    ///
    /// # Arguments
    ///
    /// * `emoji` - The emoji character or string (e.g., "👍", "❤️")
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let reaction = ReactionType::emoji("👍");
    /// assert!(reaction.is_emoji());
    /// assert_eq!(reaction.as_str(), "👍");
    /// ```
    #[must_use]
    pub fn emoji(emoji: impl AsRef<str>) -> Self {
        Self {
            inner: emoji.as_ref().to_string(),
        }
    }

    /// Creates a new custom emoji reaction from raw bytes.
    ///
    /// The bytes are base64-encoded and prefixed with "#".
    ///
    /// # Arguments
    ///
    /// * `document_id` - The document ID bytes for the custom emoji
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let doc_id = vec![0x00, 0x01, 0x02, 0x03];
    /// let reaction = ReactionType::custom_emoji(&doc_id);
    /// assert!(reaction.is_custom());
    /// assert!(reaction.as_str().starts_with('#'));
    /// ```
    #[must_use]
    pub fn custom_emoji(document_id: &[u8]) -> Self {
        use base64::prelude::*;
        let engine = base64::engine::general_purpose::STANDARD;
        let encoded = engine.encode(document_id);
        Self {
            inner: format!("#{}", encoded),
        }
    }

    /// Creates a new custom emoji reaction from a base64-encoded string.
    ///
    /// The string is prefixed with "#" if not already present.
    ///
    /// # Arguments
    ///
    /// * `base64` - The base64-encoded document ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let reaction = ReactionType::custom_emoji_from_base64("AAECAwQ=");
    /// assert!(reaction.is_custom());
    /// assert_eq!(reaction.as_str(), "#AAECAwQ=");
    /// ```
    #[must_use]
    pub fn custom_emoji_from_base64(base64: impl AsRef<str>) -> Self {
        let s = base64.as_ref();
        if s.starts_with('#') {
            Self {
                inner: s.to_string(),
            }
        } else {
            Self {
                inner: format!("#{}", s),
            }
        }
    }

    /// Creates a new paid reaction.
    ///
    /// Paid reactions are represented by the "$" character.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let reaction = ReactionType::paid();
    /// assert!(reaction.is_paid());
    /// assert_eq!(reaction.as_str(), "$");
    /// ```
    #[must_use]
    pub fn paid() -> Self {
        Self {
            inner: "$".to_string(),
        }
    }

    /// Parses a reaction from its string representation.
    ///
    /// Returns `None` if the string is empty or invalid.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let emoji = ReactionType::parse("👍");
    /// assert!(emoji.is_some());
    /// assert!(emoji.unwrap().is_emoji());
    ///
    /// let custom = ReactionType::parse("#AAECAwQ=");
    /// assert!(custom.is_some());
    /// assert!(custom.unwrap().is_custom());
    ///
    /// let paid = ReactionType::parse("$");
    /// assert!(paid.is_some());
    /// assert!(paid.unwrap().is_paid());
    ///
    /// let invalid = ReactionType::parse("");
    /// assert!(invalid.is_none());
    /// ```
    #[must_use]
    pub fn parse(s: impl AsRef<str>) -> Option<Self> {
        let s = s.as_ref();
        if s.is_empty() {
            return None;
        }

        // Check if it's a valid reaction format
        if s == "$" {
            return Some(Self::paid());
        }

        if let Some(b64_part) = s.strip_prefix('#') {
            // Validate base64 after the #
            use base64::prelude::*;
            let engine = base64::engine::general_purpose::STANDARD;
            if engine.decode(b64_part).is_ok() {
                return Some(Self {
                    inner: s.to_string(),
                });
            }
            return None;
        }

        // Otherwise, treat as emoji
        Some(Self {
            inner: s.to_string(),
        })
    }

    /// Returns the string representation of this reaction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let emoji = ReactionType::emoji("👍");
    /// assert_eq!(emoji.as_str(), "👍");
    ///
    /// let custom = ReactionType::custom_emoji(&[0x00, 0x01]);
    /// assert!(custom.as_str().starts_with('#'));
    ///
    /// let paid = ReactionType::paid();
    /// assert_eq!(paid.as_str(), "$");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Checks if this is an emoji reaction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// assert!(ReactionType::emoji("👍").is_emoji());
    /// assert!(!ReactionType::custom_emoji(&[0x00]).is_emoji());
    /// assert!(!ReactionType::paid().is_emoji());
    /// ```
    #[must_use]
    pub fn is_emoji(&self) -> bool {
        !self.inner.starts_with('#') && self.inner != "$"
    }

    /// Checks if this is a custom emoji reaction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// assert!(!ReactionType::emoji("👍").is_custom());
    /// assert!(ReactionType::custom_emoji(&[0x00]).is_custom());
    /// assert!(!ReactionType::paid().is_custom());
    /// ```
    #[must_use]
    pub fn is_custom(&self) -> bool {
        self.inner.starts_with('#')
    }

    /// Checks if this is a paid reaction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// assert!(!ReactionType::emoji("👍").is_paid());
    /// assert!(!ReactionType::custom_emoji(&[0x00]).is_paid());
    /// assert!(ReactionType::paid().is_paid());
    /// ```
    #[must_use]
    pub fn is_paid(&self) -> bool {
        self.inner == "$"
    }

    /// Gets the MD5 hash of the reaction string.
    ///
    /// Useful for caching and deduplication.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let reaction = ReactionType::emoji("👍");
    /// let hash = reaction.md5_hash();
    /// assert_eq!(hash.len(), 16); // MD5 produces 16 bytes
    /// ```
    #[must_use]
    pub fn md5_hash(&self) -> [u8; 16] {
        md5::compute(self.inner.as_bytes()).0
    }

    /// Converts the reaction to a base64-encoded string.
    ///
    /// For emoji reactions, the emoji string is encoded as UTF-8 bytes first.
    /// For custom emoji reactions, the base64 part is extracted.
    /// For paid reactions, returns "$".
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let emoji = ReactionType::emoji("👍");
    /// let encoded = emoji.to_base64();
    /// // "👍" as UTF-8, then base64-encoded
    ///
    /// let custom = ReactionType::custom_emoji(&[0x00, 0x01, 0x02, 0x03]);
    /// assert_eq!(custom.to_base64(), "AAECAw==");
    ///
    /// let paid = ReactionType::paid();
    /// assert_eq!(paid.to_base64(), "$");
    /// ```
    #[must_use]
    pub fn to_base64(&self) -> String {
        if self.is_custom() {
            // Remove the "#" prefix
            self.inner[1..].to_string()
        } else if self.is_paid() {
            "$".to_string()
        } else {
            use base64::prelude::*;
            let engine = base64::engine::general_purpose::STANDARD;
            engine.encode(self.inner.as_bytes())
        }
    }
}

impl Default for ReactionType {
    /// Returns an empty emoji reaction as the default.
    fn default() -> Self {
        Self::emoji("")
    }
}

impl std::fmt::Display for ReactionType {
    /// Formats the reaction for display.
    ///
    /// - Emoji reactions: shows the emoji
    /// - Custom emoji: shows "[Custom Emoji]"
    /// - Paid: shows "[Paid]"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_emoji() {
            write!(f, "{}", self.inner)
        } else if self.is_custom() {
            write!(f, "[Custom Emoji: {}]", &self.inner[1..])
        } else {
            write!(f, "[Paid]")
        }
    }
}

impl AsRef<str> for ReactionType {
    /// Allows easy conversion to a string slice.
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl From<ReactionType> for String {
    /// Converts a reaction type into its string representation.
    fn from(reaction: ReactionType) -> Self {
        reaction.inner
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-reaction-type";

#[cfg(test)]
mod tests {
    use super::*;

    // Test helper to check if base64 is valid
    fn is_valid_base64(s: &str) -> bool {
        use base64::prelude::*;
        let engine = base64::engine::general_purpose::STANDARD;
        engine.decode(s).is_ok()
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-reaction-type");
    }

    // === Emoji Tests ===

    #[test]
    fn test_emoji_creation() {
        let reaction = ReactionType::emoji("👍");
        assert!(reaction.is_emoji());
        assert!(!reaction.is_custom());
        assert!(!reaction.is_paid());
        assert_eq!(reaction.as_str(), "👍");
    }

    #[test]
    fn test_emoji_multiple() {
        let thumbs_up = ReactionType::emoji("👍");
        let heart = ReactionType::emoji("❤️");
        let laugh = ReactionType::emoji("😂");
        let cry = ReactionType::emoji("😢");

        assert!(thumbs_up.is_emoji());
        assert!(heart.is_emoji());
        assert!(laugh.is_emoji());
        assert!(cry.is_emoji());
    }

    #[test]
    fn test_emoji_empty() {
        let reaction = ReactionType::emoji("");
        assert!(reaction.is_emoji()); // Empty string is treated as emoji
        assert_eq!(reaction.as_str(), "");
    }

    #[test]
    fn test_emoji_equality() {
        let r1 = ReactionType::emoji("👍");
        let r2 = ReactionType::emoji("👍");
        let r3 = ReactionType::emoji("❤️");

        assert_eq!(r1, r2);
        assert_ne!(r1, r3);
    }

    // === Custom Emoji Tests ===

    #[test]
    fn test_custom_emoji_from_bytes() {
        let doc_id = vec![0x00, 0x01, 0x02, 0x03];
        let reaction = ReactionType::custom_emoji(&doc_id);

        assert!(reaction.is_custom());
        assert!(!reaction.is_emoji());
        assert!(!reaction.is_paid());
        assert!(reaction.as_str().starts_with('#'));

        // Check that the part after # is valid base64
        let b64_part = &reaction.as_str()[1..];
        assert!(is_valid_base64(b64_part));
    }

    #[test]
    fn test_custom_emoji_from_base64() {
        let reaction = ReactionType::custom_emoji_from_base64("AAECAwQ=");

        assert!(reaction.is_custom());
        assert_eq!(reaction.as_str(), "#AAECAwQ=");
    }

    #[test]
    fn test_custom_emoji_from_base64_with_prefix() {
        let reaction = ReactionType::custom_emoji_from_base64("#AAECAwQ=");

        assert!(reaction.is_custom());
        assert_eq!(reaction.as_str(), "#AAECAwQ=");
    }

    #[test]
    fn test_custom_emoji_to_base64() {
        let doc_id = vec![0x00, 0x01, 0x02, 0x03];
        let reaction = ReactionType::custom_emoji(&doc_id);

        assert_eq!(reaction.to_base64(), "AAECAw==");
    }

    #[test]
    fn test_custom_emoji_empty() {
        let reaction = ReactionType::custom_emoji(&[]);
        assert!(reaction.is_custom());
        assert_eq!(reaction.as_str(), "#");
    }

    #[test]
    fn test_custom_emoji_large_document() {
        // Test with a larger document ID
        let doc_id: Vec<u8> = (0..32).collect();
        let reaction = ReactionType::custom_emoji(&doc_id);

        assert!(reaction.is_custom());
        let b64_part = &reaction.as_str()[1..];
        assert!(is_valid_base64(b64_part));
    }

    // === Paid Reaction Tests ===

    #[test]
    fn test_paid_creation() {
        let reaction = ReactionType::paid();

        assert!(reaction.is_paid());
        assert!(!reaction.is_emoji());
        assert!(!reaction.is_custom());
        assert_eq!(reaction.as_str(), "$");
    }

    #[test]
    fn test_paid_singleton() {
        let r1 = ReactionType::paid();
        let r2 = ReactionType::paid();

        assert_eq!(r1, r2);
    }

    #[test]
    fn test_paid_to_base64() {
        let reaction = ReactionType::paid();
        assert_eq!(reaction.to_base64(), "$");
    }

    // === Parse Tests ===

    #[test]
    fn test_parse_emoji() {
        let reaction = ReactionType::parse("👍");
        assert!(reaction.is_some());
        assert!(reaction.unwrap().is_emoji());
    }

    #[test]
    fn test_parse_custom() {
        let reaction = ReactionType::parse("#AAECAwQ=");
        assert!(reaction.is_some());
        assert!(reaction.unwrap().is_custom());
    }

    #[test]
    fn test_parse_paid() {
        let reaction = ReactionType::parse("$");
        assert!(reaction.is_some());
        assert!(reaction.unwrap().is_paid());
    }

    #[test]
    fn test_parse_empty() {
        let reaction = ReactionType::parse("");
        assert!(reaction.is_none());
    }

    #[test]
    fn test_parse_invalid_custom() {
        // Invalid base64 after #
        let reaction = ReactionType::parse("#!@#$%");
        assert!(reaction.is_none());
    }

    #[test]
    fn test_parse_roundtrip() {
        // Test that parse and display work together
        let original = ReactionType::emoji("👍");
        let parsed = ReactionType::parse(original.as_str());
        assert_eq!(parsed, Some(original));

        let custom = ReactionType::custom_emoji(&[0x00, 0x01]);
        let parsed_custom = ReactionType::parse(custom.as_str());
        assert_eq!(parsed_custom, Some(custom));

        let paid = ReactionType::paid();
        let parsed_paid = ReactionType::parse(paid.as_str());
        assert_eq!(parsed_paid, Some(paid));
    }

    // === Hash Tests ===

    #[test]
    fn test_md5_hash() {
        let reaction = ReactionType::emoji("👍");
        let hash = reaction.md5_hash();
        assert_eq!(hash.len(), 16);
    }

    #[test]
    fn test_md5_hash_consistency() {
        let r1 = ReactionType::emoji("👍");
        let r2 = ReactionType::emoji("👍");
        assert_eq!(r1.md5_hash(), r2.md5_hash());
    }

    #[test]
    fn test_md5_hash_uniqueness() {
        let r1 = ReactionType::emoji("👍");
        let r2 = ReactionType::emoji("❤️");
        assert_ne!(r1.md5_hash(), r2.md5_hash());
    }

    // === Display Tests ===

    #[test]
    fn test_display_emoji() {
        let reaction = ReactionType::emoji("👍");
        assert_eq!(format!("{}", reaction), "👍");
    }

    #[test]
    fn test_display_custom() {
        let reaction = ReactionType::custom_emoji_from_base64("AAECAwQ=");
        let display = format!("{}", reaction);
        assert!(display.contains("[Custom Emoji:"));
        assert!(display.contains("AAECAwQ"));
    }

    #[test]
    fn test_display_paid() {
        let reaction = ReactionType::paid();
        assert_eq!(format!("{}", reaction), "[Paid]");
    }

    // === Conversion Tests ===

    #[test]
    fn test_as_ref() {
        let reaction = ReactionType::emoji("👍");
        let s: &str = reaction.as_ref();
        assert_eq!(s, "👍");
    }

    #[test]
    fn test_into_string() {
        let reaction = ReactionType::emoji("👍");
        let s: String = reaction.into();
        assert_eq!(s, "👍");
    }

    #[test]
    fn test_default() {
        let reaction = ReactionType::default();
        assert!(reaction.is_emoji());
        assert_eq!(reaction.as_str(), "");
    }

    // === Edge Cases ===

    #[test]
    fn test_emoji_with_hash_prefix() {
        // An emoji starting with # is treated as custom emoji (TDLib semantics)
        let reaction = ReactionType::emoji("#test");
        // Since it starts with "#", it's detected as custom emoji
        assert!(!reaction.is_emoji());
        assert!(reaction.is_custom());
    }

    #[test]
    fn test_custom_emoji_with_special_chars() {
        // Test custom emoji with various byte values
        let doc_id = vec![0xFF, 0xFE, 0x00, 0x01, 0x80];
        let reaction = ReactionType::custom_emoji(&doc_id);
        assert!(reaction.is_custom());

        let b64_part = &reaction.as_str()[1..];
        assert!(is_valid_base64(b64_part));
    }

    #[test]
    fn test_unicode_emoji() {
        // Test various Unicode emoji
        let emojis = vec!["👍", "❤️", "😂", "😢", "🎉", "🔥", "✨", "🙏"];

        for emoji in emojis {
            let reaction = ReactionType::emoji(emoji);
            assert!(reaction.is_emoji());
            assert_eq!(reaction.as_str(), emoji);
        }
    }

    #[test]
    fn test_skin_tone_emoji() {
        // Test emoji with skin tone modifiers
        let reaction = ReactionType::emoji("👍🏽");
        assert!(reaction.is_emoji());
        assert_eq!(reaction.as_str(), "👍🏽");
    }

    #[test]
    fn test_zwj_emoji() {
        // Test zero-width joiner sequences (compound emoji)
        let reaction = ReactionType::emoji("👨‍👩‍👧‍👦");
        assert!(reaction.is_emoji());
        assert_eq!(reaction.as_str(), "👨‍👩‍👧‍👦");
    }
}
