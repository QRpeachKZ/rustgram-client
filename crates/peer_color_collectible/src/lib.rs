//! # Peer Color Collectible
//!
//! Represents collectible colors for peer profiles (upgraded gift colors).
//!
//! ## TDLib Reference
//!
//! - TDLib header: `td/telegram/PeerColorCollectible.h`
//! - TDLib class: `PeerColorCollectible`
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_peer_color_collectible::PeerColorCollectible;
//! use rustgram_custom_emoji_id::CustomEmojiId;
//!
//! let collectible = PeerColorCollectible::new(
//!     CustomEmojiId::new(123),
//!     CustomEmojiId::new(456),
//!     0xFF5733,
//!     vec![0xFF5733, 0x33FF57],
//! );
//! ```

use core::fmt;
use rustgram_custom_emoji_id::CustomEmojiId;

/// Represents collectible colors for peer profiles.
///
/// This is used for upgraded gift colors that can be applied to user profiles.
///
/// TDLib: `class PeerColorCollectible`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerColorCollectible {
    unique_gift_id: i64,
    gift_custom_emoji_id: CustomEmojiId,
    background_custom_emoji_id: CustomEmojiId,
    light_accent_color: i32,
    light_colors: Vec<i32>,
    dark_accent_color: i32,
    dark_colors: Vec<i32>,
}

impl PeerColorCollectible {
    /// Create a new PeerColorCollectible.
    ///
    /// # Arguments
    ///
    /// * `gift_custom_emoji_id` - The custom emoji ID for the gift
    /// * `background_custom_emoji_id` - The custom emoji ID for the background
    /// * `light_accent_color` - The accent color for light theme (RGB)
    /// * `light_colors` - The colors for light theme (RGB values)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_peer_color_collectible::PeerColorCollectible;
    /// use rustgram_custom_emoji_id::CustomEmojiId;
    ///
    /// let collectible = PeerColorCollectible::new(
    ///     CustomEmojiId::new(123),
    ///     CustomEmojiId::new(456),
    ///     0xFF5733,
    ///     vec![0xFF5733, 0x33FF57],
    /// );
    /// ```
    pub fn new(
        gift_custom_emoji_id: CustomEmojiId,
        background_custom_emoji_id: CustomEmojiId,
        light_accent_color: i32,
        light_colors: Vec<i32>,
    ) -> Self {
        Self {
            unique_gift_id: 0,
            gift_custom_emoji_id,
            background_custom_emoji_id,
            light_accent_color,
            light_colors,
            dark_accent_color: light_accent_color,
            dark_colors: light_colors.clone(),
        }
    }

    /// Create a new PeerColorCollectible with separate dark theme colors.
    ///
    /// # Arguments
    ///
    /// * `gift_custom_emoji_id` - The custom emoji ID for the gift
    /// * `background_custom_emoji_id` - The custom emoji ID for the background
    /// * `light_accent_color` - The accent color for light theme (RGB)
    /// * `light_colors` - The colors for light theme (RGB values)
    /// * `dark_accent_color` - The accent color for dark theme (RGB)
    /// * `dark_colors` - The colors for dark theme (RGB values)
    pub fn new_with_dark_theme(
        gift_custom_emoji_id: CustomEmojiId,
        background_custom_emoji_id: CustomEmojiId,
        light_accent_color: i32,
        light_colors: Vec<i32>,
        dark_accent_color: i32,
        dark_colors: Vec<i32>,
    ) -> Self {
        Self {
            unique_gift_id: 0,
            gift_custom_emoji_id,
            background_custom_emoji_id,
            light_accent_color,
            light_colors,
            dark_accent_color,
            dark_colors,
        }
    }

    /// Get the unique gift ID.
    pub fn unique_gift_id(&self) -> i64 {
        self.unique_gift_id
    }

    /// Get the gift custom emoji ID.
    pub fn gift_custom_emoji_id(&self) -> CustomEmojiId {
        self.gift_custom_emoji_id
    }

    /// Get the background custom emoji ID.
    pub fn background_custom_emoji_id(&self) -> CustomEmojiId {
        self.background_custom_emoji_id
    }

    /// Get the light theme accent color.
    pub fn light_accent_color(&self) -> i32 {
        self.light_accent_color
    }

    /// Get the light theme colors.
    pub fn light_colors(&self) -> &[i32] {
        &self.light_colors
    }

    /// Get the dark theme accent color.
    pub fn dark_accent_color(&self) -> i32 {
        self.dark_accent_color
    }

    /// Get the dark theme colors.
    pub fn dark_colors(&self) -> &[i32] {
        &self.dark_colors
    }

    /// Set the unique gift ID.
    pub fn set_unique_gift_id(&mut self, id: i64) {
        self.unique_gift_id = id;
    }

    /// Check if this collectible is valid.
    ///
    /// A collectible is valid if it has at least one color for both themes.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_peer_color_collectible::PeerColorCollectible;
    /// use rustgram_custom_emoji_id::CustomEmojiId;
    ///
    /// let collectible = PeerColorCollectible::new(
    ///     CustomEmojiId::new(123),
    ///     CustomEmojiId::new(456),
    ///     0xFF5733,
    ///     vec![0xFF5733, 0x33FF57],
    /// );
    /// assert!(collectible.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.light_colors.is_empty() && !self.dark_colors.is_empty()
    }

    /// Get the number of light colors.
    pub fn light_color_count(&self) -> usize {
        self.light_colors.len()
    }

    /// Get the number of dark colors.
    pub fn dark_color_count(&self) -> usize {
        self.dark_colors.len()
    }
}

impl fmt::Display for PeerColorCollectible {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PeerColorCollectible(gift_emoji: {}, bg_emoji: {}, light_colors: {}, dark_colors: {})",
            self.gift_custom_emoji_id.get(),
            self.background_custom_emoji_id.get(),
            self.light_colors.len(),
            self.dark_colors.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_emoji_id(id: i64) -> CustomEmojiId {
        CustomEmojiId::new(id)
    }

    // Basic trait tests (8 tests)
    #[test]
    fn test_clone() {
        let a = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_partial_eq() {
        let a = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        let b = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(a, b);

        let c = PeerColorCollectible::new(
            make_emoji_id(999),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_ne!(a, c);
    }

    #[test]
    fn test_debug() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        let debug_str = format!("{:?}", collectible);
        assert!(debug_str.contains("PeerColorCollectible"));
    }

    // Constructor tests (6 tests)
    #[test]
    fn test_new() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(collectible.gift_custom_emoji_id().get(), 123);
        assert_eq!(collectible.background_custom_emoji_id().get(), 456);
        assert_eq!(collectible.light_accent_color(), 0xFF5733);
        assert_eq!(collectible.light_colors(), &[0xFF5733, 0x33FF57]);
    }

    #[test]
    fn test_new_dark_theme_same() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(collectible.dark_accent_color(), 0xFF5733);
        assert_eq!(collectible.dark_colors(), &[0xFF5733, 0x33FF57]);
    }

    #[test]
    fn test_new_with_dark_theme() {
        let collectible = PeerColorCollectible::new_with_dark_theme(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
            0x3357FF,
            vec![0x3357FF, 0x57FF33],
        );
        assert_eq!(collectible.light_accent_color(), 0xFF5733);
        assert_eq!(collectible.dark_accent_color(), 0x3357FF);
        assert_eq!(collectible.light_colors(), &[0xFF5733, 0x33FF57]);
        assert_eq!(collectible.dark_colors(), &[0x3357FF, 0x57FF33]);
    }

    #[test]
    fn test_new_defaults() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(collectible.unique_gift_id(), 0);
    }

    // Getter tests (10 tests)
    #[test]
    fn test_unique_gift_id() {
        let mut collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(collectible.unique_gift_id(), 0);
        collectible.set_unique_gift_id(999);
        assert_eq!(collectible.unique_gift_id(), 999);
    }

    #[test]
    fn test_gift_custom_emoji_id() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(collectible.gift_custom_emoji_id().get(), 123);
    }

    #[test]
    fn test_background_custom_emoji_id() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(collectible.background_custom_emoji_id().get(), 456);
    }

    #[test]
    fn test_light_accent_color() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(collectible.light_accent_color(), 0xFF5733);
    }

    #[test]
    fn test_light_colors() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57, 0x5733FF],
        );
        assert_eq!(collectible.light_colors(), &[0xFF5733, 0x33FF57, 0x5733FF]);
    }

    #[test]
    fn test_dark_accent_color() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(collectible.dark_accent_color(), 0xFF5733);
    }

    #[test]
    fn test_dark_colors() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(collectible.dark_colors(), &[0xFF5733, 0x33FF57]);
    }

    #[test]
    fn test_light_color_count() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57, 0x5733FF],
        );
        assert_eq!(collectible.light_color_count(), 3);
    }

    #[test]
    fn test_dark_color_count() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert_eq!(collectible.dark_color_count(), 2);
    }

    // Method tests (6 tests)
    #[test]
    fn test_is_valid_true() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        assert!(collectible.is_valid());
    }

    #[test]
    fn test_is_valid_empty_light() {
        let collectible = PeerColorCollectible::new_with_dark_theme(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![],
            0x3357FF,
            vec![0x3357FF, 0x57FF33],
        );
        assert!(!collectible.is_valid());
    }

    #[test]
    fn test_is_valid_empty_dark() {
        let collectible = PeerColorCollectible::new_with_dark_theme(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
            0x3357FF,
            vec![],
        );
        assert!(!collectible.is_valid());
    }

    #[test]
    fn test_set_unique_gift_id() {
        let mut collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        collectible.set_unique_gift_id(12345);
        assert_eq!(collectible.unique_gift_id(), 12345);
    }

    // Display tests (2 tests)
    #[test]
    fn test_display() {
        let collectible = PeerColorCollectible::new(
            make_emoji_id(123),
            make_emoji_id(456),
            0xFF5733,
            vec![0xFF5733, 0x33FF57],
        );
        let display_str = format!("{}", collectible);
        assert!(display_str.contains("123"));
        assert!(display_str.contains("456"));
        assert!(display_str.contains("light_colors: 2"));
    }
}
