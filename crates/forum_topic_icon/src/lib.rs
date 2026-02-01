// rustgram_forum_topic_icon
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

//! # Forum Topic Icon
//!
//! Represents the icon for a forum topic with color and optional custom emoji.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_forum_topic_icon::ForumTopicIcon;
//!
//! let icon = ForumTopicIcon::new();
//! assert_eq!(icon.color(), 0x6FB9F0);
//!
//! let icon_with_emoji = ForumTopicIcon::with_color_and_emoji(0xFF0000, 12345);
//! assert_eq!(icon_with_emoji.color(), 0xFF0000);
//! assert_eq!(icon_with_emoji.custom_emoji_id(), Some(12345));
//! ```

use std::fmt;

/// Custom emoji identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomEmojiId(i64);

impl CustomEmojiId {
    /// Creates a new custom emoji ID.
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the underlying i64 value.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

/// Default color for forum topic icons (light blue)
const DEFAULT_COLOR: i32 = 0x6FB9F0;

/// Color mask to ensure 24-bit RGB values
const COLOR_MASK: i32 = 0xFFFFFF;

/// Icon for a forum topic with color and optional custom emoji.
///
/// Contains an RGB color value and an optional custom emoji identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForumTopicIcon {
    /// RGB color in format 0xRRGGBB (24-bit)
    color: i32,
    /// Optional custom emoji identifier
    custom_emoji_id: CustomEmojiId,
}

impl Default for ForumTopicIcon {
    fn default() -> Self {
        Self::new()
    }
}

impl ForumTopicIcon {
    /// Creates a new forum topic icon with default color and no custom emoji.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_icon::ForumTopicIcon;
    ///
    /// let icon = ForumTopicIcon::new();
    /// assert_eq!(icon.color(), 0x6FB9F0);
    /// assert_eq!(icon.custom_emoji_id(), None);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            color: DEFAULT_COLOR,
            custom_emoji_id: CustomEmojiId::new(0),
        }
    }

    /// Creates a new forum topic icon with the specified color and custom emoji.
    ///
    /// The color is masked to 24 bits (0xRRGGBB format).
    ///
    /// # Arguments
    ///
    /// * `color` - RGB color value
    /// * `custom_emoji_id` - Custom emoji identifier
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_icon::ForumTopicIcon;
    ///
    /// let icon = ForumTopicIcon::with_color_and_emoji(0xFF0000, 12345);
    /// assert_eq!(icon.color(), 0xFF0000);
    /// assert_eq!(icon.custom_emoji_id(), Some(12345));
    /// ```
    #[must_use]
    pub fn with_color_and_emoji(color: i32, custom_emoji_id: i64) -> Self {
        Self {
            color: color & COLOR_MASK,
            custom_emoji_id: CustomEmojiId::new(custom_emoji_id),
        }
    }

    /// Returns the RGB color value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_icon::ForumTopicIcon;
    ///
    /// let icon = ForumTopicIcon::with_color_and_emoji(0x00FF00, 0);
    /// assert_eq!(icon.color(), 0x00FF00);
    /// ```
    #[must_use]
    pub const fn color(&self) -> i32 {
        self.color
    }

    /// Returns the custom emoji identifier if valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_icon::ForumTopicIcon;
    ///
    /// let icon = ForumTopicIcon::new();
    /// assert_eq!(icon.custom_emoji_id(), None);
    ///
    /// let icon_with_emoji = ForumTopicIcon::with_color_and_emoji(0, 12345);
    /// assert_eq!(icon_with_emoji.custom_emoji_id(), Some(12345));
    /// ```
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<i64> {
        let id = self.custom_emoji_id.get();
        if id != 0 {
            Some(id)
        } else {
            None
        }
    }

    /// Edits the custom emoji identifier.
    ///
    /// Returns `true` if the value changed, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `custom_emoji_id` - New custom emoji identifier
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_icon::{ForumTopicIcon, CustomEmojiId};
    ///
    /// let mut icon = ForumTopicIcon::new();
    /// assert!(icon.edit_custom_emoji_id(CustomEmojiId::new(12345))); // changed
    /// assert!(!icon.edit_custom_emoji_id(CustomEmojiId::new(12345))); // not changed
    /// ```
    pub fn edit_custom_emoji_id(&mut self, custom_emoji_id: CustomEmojiId) -> bool {
        if self.custom_emoji_id != custom_emoji_id {
            self.custom_emoji_id = custom_emoji_id;
            true
        } else {
            false
        }
    }

    /// Checks if this icon is equal to another.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_icon::ForumTopicIcon;
    ///
    /// let a = ForumTopicIcon::with_color_and_emoji(0xFF0000, 123);
    /// let b = ForumTopicIcon::with_color_and_emoji(0xFF0000, 123);
    /// assert!(a.is_equal(&b));
    /// ```
    #[must_use]
    pub fn is_equal(&self, other: &Self) -> bool {
        self.color == other.color && self.custom_emoji_id == other.custom_emoji_id
    }
}

impl fmt::Display for ForumTopicIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "icon color {:06X}", self.color)?;
        if let Some(emoji_id) = self.custom_emoji_id() {
            write!(f, " and {}", emoji_id)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let icon = ForumTopicIcon::default();
        assert_eq!(icon.color(), DEFAULT_COLOR);
        assert_eq!(icon.custom_emoji_id(), None);
    }

    #[test]
    fn test_new() {
        let icon = ForumTopicIcon::new();
        assert_eq!(icon.color(), DEFAULT_COLOR);
        assert_eq!(icon.custom_emoji_id(), None);
    }

    #[test]
    fn test_with_color_and_emoji() {
        let icon = ForumTopicIcon::with_color_and_emoji(0xFF0000, 12345);
        assert_eq!(icon.color(), 0xFF0000);
        assert_eq!(icon.custom_emoji_id(), Some(12345));
    }

    #[test]
    fn test_color_masking() {
        // Test that color is masked to 24 bits
        let icon = ForumTopicIcon::with_color_and_emoji(0xFFFFFFFFu32 as i32, 0);
        assert_eq!(icon.color(), 0xFFFFFF);

        let icon2 = ForumTopicIcon::with_color_and_emoji(0x1ABCDEF, 0);
        assert_eq!(icon2.color(), 0xABCDEF);
    }

    #[test]
    fn test_edit_custom_emoji_id() {
        let mut icon = ForumTopicIcon::new();
        assert_eq!(icon.custom_emoji_id(), None);

        assert!(icon.edit_custom_emoji_id(CustomEmojiId::new(12345)));
        assert_eq!(icon.custom_emoji_id(), Some(12345));

        assert!(!icon.edit_custom_emoji_id(CustomEmojiId::new(12345)));
        assert_eq!(icon.custom_emoji_id(), Some(12345));

        assert!(icon.edit_custom_emoji_id(CustomEmojiId::new(67890)));
        assert_eq!(icon.custom_emoji_id(), Some(67890));
    }

    #[test]
    fn test_is_equal() {
        let a = ForumTopicIcon::with_color_and_emoji(0xFF0000, 123);
        let b = ForumTopicIcon::with_color_and_emoji(0xFF0000, 123);
        let c = ForumTopicIcon::with_color_and_emoji(0xFF0000, 456);
        let d = ForumTopicIcon::with_color_and_emoji(0x00FF00, 123);

        assert!(a.is_equal(&b));
        assert!(!a.is_equal(&c));
        assert!(!a.is_equal(&d));
    }

    #[test]
    fn test_equality_trait() {
        let a = ForumTopicIcon::with_color_and_emoji(0xFF0000, 123);
        let b = ForumTopicIcon::with_color_and_emoji(0xFF0000, 123);
        let c = ForumTopicIcon::with_color_and_emoji(0xFF0000, 456);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_display_no_emoji() {
        let icon = ForumTopicIcon::new();
        assert_eq!(format!("{}", icon), "icon color 6FB9F0");
    }

    #[test]
    fn test_display_with_emoji() {
        let icon = ForumTopicIcon::with_color_and_emoji(0xFF0000, 12345);
        assert_eq!(format!("{}", icon), "icon color FF0000 and 12345");
    }

    #[test]
    fn test_display_various_colors() {
        let icon1 = ForumTopicIcon::with_color_and_emoji(0x000000, 0);
        assert_eq!(format!("{}", icon1), "icon color 000000");

        let icon2 = ForumTopicIcon::with_color_and_emoji(0xFFFFFF, 0);
        assert_eq!(format!("{}", icon2), "icon color FFFFFF");

        let icon3 = ForumTopicIcon::with_color_and_emoji(0x6FB9F0, 0);
        assert_eq!(format!("{}", icon3), "icon color 6FB9F0");
    }
}
