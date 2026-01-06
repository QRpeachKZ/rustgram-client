// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Peer Color
//!
//! Color configuration for peers (users/chats) in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`PeerColor`] struct, which represents
//! the color configuration for peers in Telegram. It includes
//! an accent color ID and an optional custom emoji ID for the background.
//! It mirrors TDLib's `PeerColor` struct.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_peer_color::PeerColor;
//!
//! // Create a peer color
//! let color = PeerColor::new(5, 12345);
//! assert_eq!(color.accent_color_id(), 5);
//! assert_eq!(color.background_custom_emoji_id(), 12345);
//!
//! // Create with no custom emoji
//! let no_emoji = PeerColor::new(3, 0);
//! assert_eq!(no_emoji.background_custom_emoji_id(), 0);
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Color configuration for a peer (user/chat).
///
/// This type provides color information for Telegram peers, including
/// the accent color and an optional custom emoji background.
///
/// # Fields
///
/// - `accent_color_id` - The accent color ID (0-7 in Telegram)
/// - `background_custom_emoji_id` - Optional custom emoji ID for background (0 if none)
///
/// # Example
///
/// ```rust
/// use rustgram_peer_color::PeerColor;
///
/// // Create a peer color with custom emoji
/// let color = PeerColor::new(5, 12345);
/// assert_eq!(color.accent_color_id(), 5);
/// assert_eq!(color.background_custom_emoji_id(), 12345);
///
/// // Create a peer color without custom emoji
/// let no_emoji = PeerColor::new(3, 0);
/// assert_eq!(no_emoji.background_custom_emoji_id(), 0);
///
/// // Default is all zeros
/// let default = PeerColor::default();
/// assert_eq!(default.accent_color_id(), 0);
/// assert_eq!(default.background_custom_emoji_id(), 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PeerColor {
    /// The accent color ID.
    ///
    /// In Telegram, accent colors are typically in the range 0-7.
    accent_color_id: i32,

    /// The custom emoji ID for the background.
    ///
    /// A value of 0 indicates no custom emoji background.
    background_custom_emoji_id: i64,
}

impl PeerColor {
    /// Creates a new peer color.
    ///
    /// # Arguments
    ///
    /// * `accent_color_id` - The accent color ID (typically 0-7)
    /// * `background_custom_emoji_id` - The custom emoji ID (0 for none)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_peer_color::PeerColor;
    ///
    /// let color = PeerColor::new(5, 12345);
    /// assert_eq!(color.accent_color_id(), 5);
    /// assert_eq!(color.background_custom_emoji_id(), 12345);
    /// ```
    #[must_use]
    pub const fn new(accent_color_id: i32, background_custom_emoji_id: i64) -> Self {
        Self {
            accent_color_id,
            background_custom_emoji_id,
        }
    }

    /// Returns the accent color ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_peer_color::PeerColor;
    ///
    /// let color = PeerColor::new(5, 12345);
    /// assert_eq!(color.accent_color_id(), 5);
    /// ```
    #[must_use]
    pub const fn accent_color_id(&self) -> i32 {
        self.accent_color_id
    }

    /// Returns the background custom emoji ID.
    ///
    /// A value of 0 indicates no custom emoji background.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_peer_color::PeerColor;
    ///
    /// let color = PeerColor::new(5, 12345);
    /// assert_eq!(color.background_custom_emoji_id(), 12345);
    ///
    /// let no_emoji = PeerColor::new(3, 0);
    /// assert_eq!(no_emoji.background_custom_emoji_id(), 0);
    /// ```
    #[must_use]
    pub const fn background_custom_emoji_id(&self) -> i64 {
        self.background_custom_emoji_id
    }

    /// Returns both color values as a tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_peer_color::PeerColor;
    ///
    /// let color = PeerColor::new(5, 12345);
    /// assert_eq!(color.get(), (5, 12345));
    /// ```
    #[must_use]
    pub const fn get(&self) -> (i32, i64) {
        (self.accent_color_id, self.background_custom_emoji_id)
    }

    /// Checks if this peer has a custom emoji background.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_peer_color::PeerColor;
    ///
    /// let with_emoji = PeerColor::new(5, 12345);
    /// assert!(with_emoji.has_custom_emoji());
    ///
    /// let without_emoji = PeerColor::new(3, 0);
    /// assert!(!without_emoji.has_custom_emoji());
    /// ```
    #[must_use]
    pub const fn has_custom_emoji(&self) -> bool {
        self.background_custom_emoji_id != 0
    }
}

impl Default for PeerColor {
    /// Creates a default peer color with zeros.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_peer_color::PeerColor;
    ///
    /// let default = PeerColor::default();
    /// assert_eq!(default.accent_color_id(), 0);
    /// assert_eq!(default.background_custom_emoji_id(), 0);
    /// ```
    fn default() -> Self {
        Self {
            accent_color_id: 0,
            background_custom_emoji_id: 0,
        }
    }
}

impl fmt::Display for PeerColor {
    /// Formats the peer color for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_peer_color::PeerColor;
    ///
    /// let color = PeerColor::new(5, 12345);
    /// assert_eq!(format!("{}", color), "PeerColor(accent: 5, emoji: 12345)");
    ///
    /// let no_emoji = PeerColor::new(3, 0);
    /// assert_eq!(format!("{}", no_emoji), "PeerColor(accent: 3, emoji: none)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.has_custom_emoji() {
            write!(
                f,
                "PeerColor(accent: {}, emoji: {})",
                self.accent_color_id, self.background_custom_emoji_id
            )
        } else {
            write!(
                f,
                "PeerColor(accent: {}, emoji: none)",
                self.accent_color_id
            )
        }
    }
}

impl From<(i32, i64)> for PeerColor {
    /// Creates a peer color from a tuple of (accent_color_id, background_custom_emoji_id).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_peer_color::PeerColor;
    ///
    /// let color: PeerColor = (5, 12345).into();
    /// assert_eq!(color.accent_color_id(), 5);
    /// assert_eq!(color.background_custom_emoji_id(), 12345);
    /// ```
    fn from((accent_color_id, background_custom_emoji_id): (i32, i64)) -> Self {
        Self::new(accent_color_id, background_custom_emoji_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let color = PeerColor::new(5, 12345);
        assert_eq!(color.accent_color_id(), 5);
        assert_eq!(color.background_custom_emoji_id(), 12345);
    }

    #[test]
    fn test_accent_color_id() {
        let color = PeerColor::new(3, 999);
        assert_eq!(color.accent_color_id(), 3);
    }

    #[test]
    fn test_background_custom_emoji_id() {
        let color = PeerColor::new(5, 12345);
        assert_eq!(color.background_custom_emoji_id(), 12345);

        let no_emoji = PeerColor::new(3, 0);
        assert_eq!(no_emoji.background_custom_emoji_id(), 0);
    }

    #[test]
    fn test_get() {
        let color = PeerColor::new(5, 12345);
        assert_eq!(color.get(), (5, 12345));

        let color2 = PeerColor::new(0, 0);
        assert_eq!(color2.get(), (0, 0));
    }

    #[test]
    fn test_has_custom_emoji_true() {
        let color = PeerColor::new(5, 12345);
        assert!(color.has_custom_emoji());

        let color2 = PeerColor::new(3, -1);
        assert!(color2.has_custom_emoji());
    }

    #[test]
    fn test_has_custom_emoji_false() {
        let color = PeerColor::new(3, 0);
        assert!(!color.has_custom_emoji());
    }

    #[test]
    fn test_default() {
        let default = PeerColor::default();
        assert_eq!(default.accent_color_id(), 0);
        assert_eq!(default.background_custom_emoji_id(), 0);
        assert!(!default.has_custom_emoji());
    }

    #[test]
    fn test_equality() {
        let color1 = PeerColor::new(5, 12345);
        let color2 = PeerColor::new(5, 12345);
        assert_eq!(color1, color2);
    }

    #[test]
    fn test_inequality() {
        let color1 = PeerColor::new(5, 12345);
        let color2 = PeerColor::new(3, 12345);
        assert_ne!(color1, color2);

        let color3 = PeerColor::new(5, 999);
        assert_ne!(color1, color3);
    }

    #[test]
    fn test_copy_semantics() {
        let color1 = PeerColor::new(5, 12345);
        let color2 = color1;
        assert_eq!(color1, color2);
        assert_eq!(color1.accent_color_id(), 5);
        assert_eq!(color2.accent_color_id(), 5);
    }

    #[test]
    #[test]
    fn test_display_format_with_emoji() {
        let color = PeerColor::new(5, 12345);
        assert_eq!(format!("{}", color), "PeerColor(accent: 5, emoji: 12345)");
    }

    #[test]
    fn test_display_format_without_emoji() {
        let color = PeerColor::new(3, 0);
        assert_eq!(format!("{}", color), "PeerColor(accent: 3, emoji: none)");
    }

    #[test]
    fn test_from_tuple() {
        let color: PeerColor = (5, 12345).into();
        assert_eq!(color.accent_color_id(), 5);
        assert_eq!(color.background_custom_emoji_id(), 12345);

        let color2: PeerColor = (0, 0).into();
        assert_eq!(color2.accent_color_id(), 0);
        assert_eq!(color2.background_custom_emoji_id(), 0);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(PeerColor::new(5, 12345));
        set.insert(PeerColor::new(3, 999));
        set.insert(PeerColor::new(5, 12345)); // Duplicate

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_debug_format() {
        let color = PeerColor::new(5, 12345);
        let debug_str = format!("{:?}", color);
        assert!(debug_str.contains("PeerColor"));
        assert!(debug_str.contains("5"));
        assert!(debug_str.contains("12345"));
    }

    #[test]
    fn test_negative_values() {
        let color = PeerColor::new(-1, -999);
        assert_eq!(color.accent_color_id(), -1);
        assert_eq!(color.background_custom_emoji_id(), -999);
        assert!(color.has_custom_emoji());
    }

    #[test]
    fn test_large_values() {
        let color = PeerColor::new(i32::MAX, i64::MAX);
        assert_eq!(color.accent_color_id(), i32::MAX);
        assert_eq!(color.background_custom_emoji_id(), i64::MAX);
        assert!(color.has_custom_emoji());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = PeerColor::new(5, 12345);

        // Test JSON serialization
        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(
            json,
            r#"{"accent_color_id":5,"background_custom_emoji_id":12345}"#
        );

        let deserialized: PeerColor = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);

        // Test binary serialization
        let encoded = bincode::serialize(&original).unwrap();
        let decoded: PeerColor = bincode::deserialize(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_zero() {
        let original = PeerColor::new(0, 0);

        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(
            json,
            r#"{"accent_color_id":0,"background_custom_emoji_id":0}"#
        );

        let deserialized: PeerColor = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_negative() {
        let original = PeerColor::new(-1, -999);

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PeerColor = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
