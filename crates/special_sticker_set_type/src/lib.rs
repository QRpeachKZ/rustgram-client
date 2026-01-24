// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Special sticker set type for Telegram MTProto client.
//!
//! This module implements TDLib's special sticker set type identifiers.
//!
//! # Example
//!
//! ```rust
//! use rustgram_special_sticker_set_type::SpecialStickerSetType;
//!
//! let animated_emoji = SpecialStickerSetType::animated_emoji();
//! assert_eq!(animated_emoji.as_str(), "animated_emoji_sticker_set");
//!
//! let dice = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string()).unwrap();
//! assert_eq!(dice.as_str(), "animated_dice_sticker_set#\u{1F3B2}");
//! assert_eq!(dice.get_dice_emoji(), Some("\u{1F3B2}".to_string()));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod tl;

pub use tl::InputStickerSet;

use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};

/// Special sticker set type identifier.
///
/// Wraps a String containing the special sticker set type identifier.
/// Based on TDLib's special sticker set type implementation.
///
/// # Example
///
/// ```rust
/// use rustgram_special_sticker_set_type::SpecialStickerSetType;
///
/// let emoji = SpecialStickerSetType::animated_emoji();
/// assert!(!emoji.is_empty());
/// assert_eq!(emoji.as_str(), "animated_emoji_sticker_set");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpecialStickerSetType {
    /// Inner string value
    inner: String,
}

impl SpecialStickerSetType {
    /// Creates an animated emoji sticker set type.
    ///
    /// # Returns
    ///
    /// Returns a `SpecialStickerSetType` for animated emoji.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let emoji = SpecialStickerSetType::animated_emoji();
    /// assert_eq!(emoji.as_str(), "animated_emoji_sticker_set");
    /// ```
    pub fn animated_emoji() -> Self {
        Self {
            inner: String::from("animated_emoji_sticker_set"),
        }
    }

    /// Creates an animated emoji click sticker set type.
    ///
    /// # Returns
    ///
    /// Returns a `SpecialStickerSetType` for animated emoji click interactions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let click = SpecialStickerSetType::animated_emoji_click();
    /// assert_eq!(click.as_str(), "animated_emoji_click_sticker_set");
    /// ```
    pub fn animated_emoji_click() -> Self {
        Self {
            inner: String::from("animated_emoji_click_sticker_set"),
        }
    }

    /// Creates an animated dice sticker set type.
    ///
    /// # Arguments
    ///
    /// * `emoji` - The emoji character for the dice type (e.g., "\u{1F3B2}", "\u{26BD}", "\u{1F3C0}")
    ///
    /// # Returns
    ///
    /// Returns `Some(SpecialStickerSetType)` if emoji is non-empty,
    /// `None` if emoji is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let dice = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string());
    /// assert!(dice.is_some());
    /// assert_eq!(dice.unwrap().as_str(), "animated_dice_sticker_set#\u{1F3B2}");
    ///
    /// let empty = SpecialStickerSetType::animated_dice(String::new());
    /// assert!(empty.is_none());
    /// ```
    pub fn animated_dice(emoji: String) -> Option<Self> {
        if emoji.is_empty() {
            return None;
        }
        Some(Self {
            inner: format!("animated_dice_sticker_set#{emoji}"),
        })
    }

    /// Creates a premium gifts sticker set type.
    ///
    /// # Returns
    ///
    /// Returns a `SpecialStickerSetType` for premium gifts.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let gifts = SpecialStickerSetType::premium_gifts();
    /// assert_eq!(gifts.as_str(), "premium_gifts_sticker_set");
    /// ```
    pub fn premium_gifts() -> Self {
        Self {
            inner: String::from("premium_gifts_sticker_set"),
        }
    }

    /// Creates a generic animations sticker set type.
    ///
    /// # Returns
    ///
    /// Returns a `SpecialStickerSetType` for generic animations.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let animations = SpecialStickerSetType::generic_animations();
    /// assert_eq!(animations.as_str(), "generic_animations_sticker_set");
    /// ```
    pub fn generic_animations() -> Self {
        Self {
            inner: String::from("generic_animations_sticker_set"),
        }
    }

    /// Creates a default statuses sticker set type.
    ///
    /// # Returns
    ///
    /// Returns a `SpecialStickerSetType` for default statuses.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let statuses = SpecialStickerSetType::default_statuses();
    /// assert_eq!(statuses.as_str(), "default_statuses_sticker_set");
    /// ```
    pub fn default_statuses() -> Self {
        Self {
            inner: String::from("default_statuses_sticker_set"),
        }
    }

    /// Creates a default channel statuses sticker set type.
    ///
    /// # Returns
    ///
    /// Returns a `SpecialStickerSetType` for default channel statuses.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let channel_statuses = SpecialStickerSetType::default_channel_statuses();
    /// assert_eq!(channel_statuses.as_str(), "default_channel_statuses_sticker_set");
    /// ```
    pub fn default_channel_statuses() -> Self {
        Self {
            inner: String::from("default_channel_statuses_sticker_set"),
        }
    }

    /// Creates a default topic icons sticker set type.
    ///
    /// # Returns
    ///
    /// Returns a `SpecialStickerSetType` for default topic icons.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let topics = SpecialStickerSetType::default_topic_icons();
    /// assert_eq!(topics.as_str(), "default_topic_icons_sticker_set");
    /// ```
    pub fn default_topic_icons() -> Self {
        Self {
            inner: String::from("default_topic_icons_sticker_set"),
        }
    }

    /// Creates a TON gifts sticker set type.
    ///
    /// # Returns
    ///
    /// Returns a `SpecialStickerSetType` for TON gifts.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let ton = SpecialStickerSetType::ton_gifts();
    /// assert_eq!(ton.as_str(), "ton_gifts_sticker_set");
    /// ```
    pub fn ton_gifts() -> Self {
        Self {
            inner: String::from("ton_gifts_sticker_set"),
        }
    }

    /// Checks if this sticker set type is empty.
    ///
    /// # Returns
    ///
    /// Returns `true` if the inner string is empty, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let emoji = SpecialStickerSetType::animated_emoji();
    /// assert!(!emoji.is_empty());
    ///
    /// let empty = SpecialStickerSetType::default();
    /// assert!(empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Extracts the dice emoji from this sticker set type.
    ///
    /// # Returns
    ///
    /// Returns `Some(emoji)` if this is a dice sticker set with a valid emoji,
    /// `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let dice = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string()).unwrap();
    /// assert_eq!(dice.get_dice_emoji(), Some("\u{1F3B2}".to_string()));
    ///
    /// let emoji = SpecialStickerSetType::animated_emoji();
    /// assert_eq!(emoji.get_dice_emoji(), None);
    /// ```
    pub fn get_dice_emoji(&self) -> Option<String> {
        if self.inner.starts_with("animated_dice_sticker_set#") {
            let emoji = self.inner.strip_prefix("animated_dice_sticker_set#")?;
            if emoji.is_empty() {
                return None;
            }
            Some(String::from(emoji))
        } else {
            None
        }
    }

    /// Returns a string slice reference to the inner value.
    ///
    /// # Returns
    ///
    /// Returns a `&str` reference to the inner string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let emoji = SpecialStickerSetType::animated_emoji();
    /// assert_eq!(emoji.as_str(), "animated_emoji_sticker_set");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Creates a special sticker set type from an InputStickerSet.
    ///
    /// # Arguments
    ///
    /// * `input_set` - The InputStickerSet to convert from
    ///
    /// # Returns
    ///
    /// Returns a `SpecialStickerSetType` matching the input set.
    ///
    /// # Warning
    ///
    /// This is a stub implementation that logs a warning. The full TL layer
    /// implementation will provide proper conversion logic.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::{SpecialStickerSetType, InputStickerSet};
    ///
    /// let input = InputStickerSet::AnimatedEmoji;
    /// let sticker_type = SpecialStickerSetType::from_input_sticker_set(input);
    /// // Note: This is a stub, returns empty in current implementation
    /// ```
    pub fn from_input_sticker_set(input_set: InputStickerSet) -> Self {
        match input_set {
            InputStickerSet::Empty => Self {
                inner: String::new(),
            },
            InputStickerSet::Id { .. } => Self {
                inner: String::new(),
            },
            InputStickerSet::ShortName { .. } => Self {
                inner: String::new(),
            },
            InputStickerSet::AnimatedEmoji => Self::animated_emoji(),
            InputStickerSet::AnimatedEmojiClick { emoji } => Self {
                inner: format!("animated_emoji_click_sticker_set#{emoji}"),
            },
            InputStickerSet::AnimatedDice { emoji } => Self {
                inner: format!("animated_dice_sticker_set#{emoji}"),
            },
            InputStickerSet::PremiumGifts => Self::premium_gifts(),
            InputStickerSet::GenericAnimations => Self::generic_animations(),
            InputStickerSet::GenericEffects => Self {
                inner: String::from("generic_effects_sticker_set"),
            },
        }
    }

    /// Attempts to convert this sticker set type to an InputStickerSet.
    ///
    /// # Returns
    ///
    /// Returns `Some(InputStickerSet)` if conversion is possible,
    /// `None` if the type is unknown or empty.
    ///
    /// # Note
    ///
    /// This is a stub implementation. The full TL layer will provide
    /// proper bidirectional conversion.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_special_sticker_set_type::SpecialStickerSetType;
    ///
    /// let emoji = SpecialStickerSetType::animated_emoji();
    /// let input = emoji.to_input_sticker_set();
    /// // Note: Stub implementation, currently returns None
    /// ```
    pub fn to_input_sticker_set(&self) -> Option<InputStickerSet> {
        if self.inner.is_empty() {
            return None;
        }

        match self.inner.as_str() {
            "animated_emoji_sticker_set" => Some(InputStickerSet::AnimatedEmoji),
            "animated_emoji_click_sticker_set" => Some(InputStickerSet::AnimatedEmojiClick {
                emoji: String::new(),
            }),
            "premium_gifts_sticker_set" => Some(InputStickerSet::PremiumGifts),
            "generic_animations_sticker_set" => Some(InputStickerSet::GenericAnimations),
            "generic_effects_sticker_set" => Some(InputStickerSet::GenericEffects),
            _ => {
                if self.inner.starts_with("animated_dice_sticker_set#") {
                    if let Some(emoji) = self.get_dice_emoji() {
                        return Some(InputStickerSet::AnimatedDice { emoji });
                    }
                }
                if self.inner.starts_with("animated_emoji_click_sticker_set#") {
                    if let Some(emoji) =
                        self.inner.strip_prefix("animated_emoji_click_sticker_set#")
                    {
                        return Some(InputStickerSet::AnimatedEmojiClick {
                            emoji: String::from(emoji),
                        });
                    }
                }
                None
            }
        }
    }
}

impl Display for SpecialStickerSetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Hash for SpecialStickerSetType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl From<String> for SpecialStickerSetType {
    fn from(inner: String) -> Self {
        Self { inner }
    }
}

impl From<&str> for SpecialStickerSetType {
    fn from(inner: &str) -> Self {
        Self {
            inner: String::from(inner),
        }
    }
}

impl From<SpecialStickerSetType> for String {
    fn from(sticker_type: SpecialStickerSetType) -> Self {
        sticker_type.inner
    }
}

impl AsRef<str> for SpecialStickerSetType {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animated_emoji() {
        let emoji = SpecialStickerSetType::animated_emoji();
        assert_eq!(emoji.as_str(), "animated_emoji_sticker_set");
        assert!(!emoji.is_empty());
    }

    #[test]
    fn test_animated_emoji_click() {
        let click = SpecialStickerSetType::animated_emoji_click();
        assert_eq!(click.as_str(), "animated_emoji_click_sticker_set");
        assert!(!click.is_empty());
    }

    #[test]
    fn test_animated_dice_valid() {
        let dice = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string());
        assert!(dice.is_some());
        assert_eq!(
            dice.unwrap().as_str(),
            "animated_dice_sticker_set#\u{1F3B2}"
        );
    }

    #[test]
    fn test_animated_dice_empty_emoji() {
        let dice = SpecialStickerSetType::animated_dice(String::new());
        assert!(dice.is_none());
    }

    #[test]
    fn test_animated_dice_multiple_emojis() {
        let dice1 = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string());
        assert!(dice1.is_some());
        assert_eq!(
            dice1.unwrap().as_str(),
            "animated_dice_sticker_set#\u{1F3B2}"
        );

        let dice2 = SpecialStickerSetType::animated_dice("\u{26BD}".to_string());
        assert!(dice2.is_some());
        assert_eq!(
            dice2.unwrap().as_str(),
            "animated_dice_sticker_set#\u{26BD}"
        );

        let dice3 = SpecialStickerSetType::animated_dice("\u{1F3C0}".to_string());
        assert!(dice3.is_some());
        assert_eq!(
            dice3.unwrap().as_str(),
            "animated_dice_sticker_set#\u{1F3C0}"
        );
    }

    #[test]
    fn test_premium_gifts() {
        let gifts = SpecialStickerSetType::premium_gifts();
        assert_eq!(gifts.as_str(), "premium_gifts_sticker_set");
        assert!(!gifts.is_empty());
    }

    #[test]
    fn test_generic_animations() {
        let animations = SpecialStickerSetType::generic_animations();
        assert_eq!(animations.as_str(), "generic_animations_sticker_set");
        assert!(!animations.is_empty());
    }

    #[test]
    fn test_default_statuses() {
        let statuses = SpecialStickerSetType::default_statuses();
        assert_eq!(statuses.as_str(), "default_statuses_sticker_set");
        assert!(!statuses.is_empty());
    }

    #[test]
    fn test_default_channel_statuses() {
        let channel_statuses = SpecialStickerSetType::default_channel_statuses();
        assert_eq!(
            channel_statuses.as_str(),
            "default_channel_statuses_sticker_set"
        );
        assert!(!channel_statuses.is_empty());
    }

    #[test]
    fn test_default_topic_icons() {
        let topics = SpecialStickerSetType::default_topic_icons();
        assert_eq!(topics.as_str(), "default_topic_icons_sticker_set");
        assert!(!topics.is_empty());
    }

    #[test]
    fn test_ton_gifts() {
        let ton = SpecialStickerSetType::ton_gifts();
        assert_eq!(ton.as_str(), "ton_gifts_sticker_set");
        assert!(!ton.is_empty());
    }

    #[test]
    fn test_is_empty() {
        let emoji = SpecialStickerSetType::animated_emoji();
        assert!(!emoji.is_empty());

        let empty = SpecialStickerSetType {
            inner: String::new(),
        };
        assert!(empty.is_empty());
    }

    #[test]
    fn test_get_dice_emoji_valid() {
        let dice = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string()).unwrap();
        assert_eq!(dice.get_dice_emoji(), Some("\u{1F3B2}".to_string()));
    }

    #[test]
    fn test_get_dice_emoji_multiple_types() {
        let dice_d6 = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string()).unwrap();
        assert_eq!(dice_d6.get_dice_emoji(), Some("\u{1F3B2}".to_string()));

        let dice_soccer = SpecialStickerSetType::animated_dice("\u{26BD}".to_string()).unwrap();
        assert_eq!(dice_soccer.get_dice_emoji(), Some("\u{26BD}".to_string()));

        let dice_basketball =
            SpecialStickerSetType::animated_dice("\u{1F3C0}".to_string()).unwrap();
        assert_eq!(
            dice_basketball.get_dice_emoji(),
            Some("\u{1F3C0}".to_string())
        );
    }

    #[test]
    fn test_get_dice_emoji_invalid() {
        let emoji = SpecialStickerSetType::animated_emoji();
        assert_eq!(emoji.get_dice_emoji(), None);

        let gifts = SpecialStickerSetType::premium_gifts();
        assert_eq!(gifts.get_dice_emoji(), None);
    }

    #[test]
    fn test_get_dice_emoji_empty_prefix() {
        let invalid = SpecialStickerSetType {
            inner: String::from("animated_dice_sticker_set#"),
        };
        assert_eq!(invalid.get_dice_emoji(), None);
    }

    #[test]
    fn test_as_str() {
        let emoji = SpecialStickerSetType::animated_emoji();
        assert_eq!(emoji.as_str(), "animated_emoji_sticker_set");

        let dice = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string()).unwrap();
        assert_eq!(dice.as_str(), "animated_dice_sticker_set#\u{1F3B2}");
    }

    #[test]
    fn test_display() {
        let emoji = SpecialStickerSetType::animated_emoji();
        assert_eq!(format!("{}", emoji), "animated_emoji_sticker_set");

        let dice = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string()).unwrap();
        assert_eq!(format!("{}", dice), "animated_dice_sticker_set#\u{1F3B2}");
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;
        let mut set = HashSet::new();

        let emoji1 = SpecialStickerSetType::animated_emoji();
        let emoji2 = SpecialStickerSetType::animated_emoji();
        set.insert(emoji1);
        set.insert(emoji2);

        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_hash_uniqueness() {
        use std::collections::HashSet;
        let mut set = HashSet::new();

        set.insert(SpecialStickerSetType::animated_emoji());
        set.insert(SpecialStickerSetType::premium_gifts());
        set.insert(SpecialStickerSetType::ton_gifts());

        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_equality() {
        let emoji1 = SpecialStickerSetType::animated_emoji();
        let emoji2 = SpecialStickerSetType::animated_emoji();
        assert_eq!(emoji1, emoji2);

        let gifts1 = SpecialStickerSetType::premium_gifts();
        let gifts2 = SpecialStickerSetType::premium_gifts();
        assert_eq!(gifts1, gifts2);
    }

    #[test]
    fn test_inequality() {
        let emoji = SpecialStickerSetType::animated_emoji();
        let gifts = SpecialStickerSetType::premium_gifts();
        assert_ne!(emoji, gifts);
    }

    #[test]
    fn test_clone() {
        let original = SpecialStickerSetType::animated_emoji();
        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(original.as_str(), cloned.as_str());
    }

    #[test]
    fn test_debug() {
        let emoji = SpecialStickerSetType::animated_emoji();
        let debug = format!("{:?}", emoji);
        assert!(debug.contains("animated_emoji_sticker_set"));
    }

    #[test]
    fn test_default() {
        let default = SpecialStickerSetType::default();
        assert!(default.is_empty());
        assert_eq!(default.as_str(), "");
    }

    #[test]
    fn test_from_string() {
        let s = String::from("animated_emoji_sticker_set");
        let sticker_type = SpecialStickerSetType::from(s.clone());
        assert_eq!(sticker_type.as_str(), s);
    }

    #[test]
    fn test_from_str() {
        let s = "premium_gifts_sticker_set";
        let sticker_type = SpecialStickerSetType::from(s);
        assert_eq!(sticker_type.as_str(), s);
    }

    #[test]
    fn test_into_string() {
        let sticker_type = SpecialStickerSetType::animated_emoji();
        let s: String = sticker_type.into();
        assert_eq!(s, "animated_emoji_sticker_set");
    }

    #[test]
    fn test_as_ref() {
        let sticker_type = SpecialStickerSetType::animated_emoji();
        let s: &str = sticker_type.as_ref();
        assert_eq!(s, "animated_emoji_sticker_set");
    }

    #[test]
    fn test_from_input_sticker_set_animated_emoji() {
        let input = InputStickerSet::AnimatedEmoji;
        let sticker_type = SpecialStickerSetType::from_input_sticker_set(input);
        assert_eq!(sticker_type.as_str(), "animated_emoji_sticker_set");
    }

    #[test]
    fn test_from_input_sticker_set_premium_gifts() {
        let input = InputStickerSet::PremiumGifts;
        let sticker_type = SpecialStickerSetType::from_input_sticker_set(input);
        assert_eq!(sticker_type.as_str(), "premium_gifts_sticker_set");
    }

    #[test]
    fn test_from_input_sticker_set_animated_dice() {
        let input = InputStickerSet::AnimatedDice {
            emoji: String::from(""),
        };
        let sticker_type = SpecialStickerSetType::from_input_sticker_set(input);
        assert_eq!(sticker_type.as_str(), "animated_dice_sticker_set#");
    }

    #[test]
    fn test_to_input_sticker_set_animated_emoji() {
        let sticker_type = SpecialStickerSetType::animated_emoji();
        let input = sticker_type.to_input_sticker_set();
        assert!(input.is_some());
        assert_eq!(input.unwrap(), InputStickerSet::AnimatedEmoji);
    }

    #[test]
    fn test_to_input_sticker_set_premium_gifts() {
        let sticker_type = SpecialStickerSetType::premium_gifts();
        let input = sticker_type.to_input_sticker_set();
        assert!(input.is_some());
        assert_eq!(input.unwrap(), InputStickerSet::PremiumGifts);
    }

    #[test]
    fn test_to_input_sticker_set_animated_dice() {
        let sticker_type = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string()).unwrap();
        let input = sticker_type.to_input_sticker_set();
        assert!(input.is_some());
        match input.unwrap() {
            InputStickerSet::AnimatedDice { emoji } => {
                assert_eq!(emoji, "\u{1F3B2}");
            }
            _ => panic!("Expected AnimatedDice"),
        }
    }

    #[test]
    fn test_to_input_sticker_set_empty() {
        let sticker_type = SpecialStickerSetType::default();
        let input = sticker_type.to_input_sticker_set();
        assert!(input.is_none());
    }

    #[test]
    fn test_to_input_sticker_set_unknown() {
        let sticker_type = SpecialStickerSetType {
            inner: String::from("unknown_sticker_set"),
        };
        let input = sticker_type.to_input_sticker_set();
        assert!(input.is_none());
    }

    #[test]
    fn test_all_constructors_distinct() {
        let types = [
            SpecialStickerSetType::animated_emoji(),
            SpecialStickerSetType::animated_emoji_click(),
            SpecialStickerSetType::premium_gifts(),
            SpecialStickerSetType::generic_animations(),
            SpecialStickerSetType::default_statuses(),
            SpecialStickerSetType::default_channel_statuses(),
            SpecialStickerSetType::default_topic_icons(),
            SpecialStickerSetType::ton_gifts(),
        ];

        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j]);
            }
        }
    }

    #[test]
    fn test_all_non_empty_constructors() {
        let types = [
            SpecialStickerSetType::animated_emoji(),
            SpecialStickerSetType::animated_emoji_click(),
            SpecialStickerSetType::premium_gifts(),
            SpecialStickerSetType::generic_animations(),
            SpecialStickerSetType::default_statuses(),
            SpecialStickerSetType::default_channel_statuses(),
            SpecialStickerSetType::default_topic_icons(),
            SpecialStickerSetType::ton_gifts(),
        ];

        for sticker_type in types {
            assert!(!sticker_type.is_empty());
        }
    }

    #[test]
    fn test_roundtrip_conversion() {
        let original = SpecialStickerSetType::animated_emoji();
        let input = original.to_input_sticker_set();
        assert!(input.is_some());

        let converted = SpecialStickerSetType::from_input_sticker_set(input.unwrap());
        assert_eq!(original, converted);
    }

    #[test]
    fn test_dice_roundtrip() {
        let original = SpecialStickerSetType::animated_dice("\u{1F3B2}".to_string()).unwrap();
        let emoji = original.get_dice_emoji();
        assert!(emoji.is_some());
        assert_eq!(emoji.unwrap(), "\u{1F3B2}");
    }

    #[test]
    fn test_display_all_types() {
        let types = [
            SpecialStickerSetType::animated_emoji(),
            SpecialStickerSetType::animated_emoji_click(),
            SpecialStickerSetType::premium_gifts(),
            SpecialStickerSetType::generic_animations(),
            SpecialStickerSetType::default_statuses(),
            SpecialStickerSetType::default_channel_statuses(),
            SpecialStickerSetType::default_topic_icons(),
            SpecialStickerSetType::ton_gifts(),
        ];

        for sticker_type in types {
            let display = format!("{}", sticker_type);
            assert!(!display.is_empty());
            assert_eq!(display, sticker_type.as_str());
        }
    }
}
