// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Minimal channel information for Telegram MTProto client.
//!
//! This module implements TDLib's MinChannel struct.
//!
//! # Example
//!
//! ```rust
//! use rustgram_min_channel::MinChannel;
//! use rustgram_accent_color_id::AccentColorId;
//! use rustgram_dialog_photo::DialogPhoto;
//!
//! let channel = MinChannel::new(
//!     "Test Channel".to_string(),
//!     DialogPhoto::default(),
//!     AccentColorId::default(),
//!     true
//! );
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_accent_color_id::AccentColorId;
use rustgram_dialog_photo::DialogPhoto;
use std::fmt::{self, Display, Formatter};

/// Minimal channel information.
///
/// Based on TDLib's `MinChannel` struct.
///
/// Contains basic information about a channel/megagroup, including title,
/// photo, accent color, and whether it's a megagroup.
///
/// # Example
///
/// ```rust
/// use rustgram_min_channel::MinChannel;
///
/// let channel = MinChannel::default();
/// assert_eq!(channel.title(), "");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinChannel {
    /// Channel title.
    title: String,

    /// Channel photo.
    photo: DialogPhoto,

    /// Accent color ID.
    accent_color_id: AccentColorId,

    /// Whether this is a megagroup (as opposed to a broadcast channel).
    is_megagroup: bool,
}

impl MinChannel {
    /// Creates a new MinChannel.
    ///
    /// # Arguments
    ///
    /// * `title` - Channel title
    /// * `photo` - Channel photo
    /// * `accent_color_id` - Accent color ID
    /// * `is_megagroup` - Whether this is a megagroup
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_min_channel::MinChannel;
    /// use rustgram_accent_color_id::AccentColorId;
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let channel = MinChannel::new(
    ///     "My Channel".to_string(),
    ///     DialogPhoto::default(),
    ///     AccentColorId::default(),
    ///     false
    /// );
    /// ```
    pub fn new(
        title: String,
        photo: DialogPhoto,
        accent_color_id: AccentColorId,
        is_megagroup: bool,
    ) -> Self {
        Self {
            title,
            photo,
            accent_color_id,
            is_megagroup,
        }
    }

    /// Returns the channel title.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_min_channel::MinChannel;
    ///
    /// let channel = MinChannel::new(
    ///     "Test".to_string(),
    ///     rustgram_dialog_photo::DialogPhoto::default(),
    ///     rustgram_accent_color_id::AccentColorId::default(),
    ///     true
    /// );
    /// assert_eq!(channel.title(), "Test");
    /// ```
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the channel photo.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_min_channel::MinChannel;
    ///
    /// let channel = MinChannel::default();
    /// let photo = channel.photo();
    /// ```
    pub fn photo(&self) -> &DialogPhoto {
        &self.photo
    }

    /// Returns the accent color ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_min_channel::MinChannel;
    ///
    /// let channel = MinChannel::default();
    /// let color = channel.accent_color_id();
    /// ```
    pub fn accent_color_id(&self) -> AccentColorId {
        self.accent_color_id
    }

    /// Checks if this is a megagroup.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_min_channel::MinChannel;
    ///
    /// let channel = MinChannel::new(
    ///     "Group".to_string(),
    ///     rustgram_dialog_photo::DialogPhoto::default(),
    ///     rustgram_accent_color_id::AccentColorId::default(),
    ///     true
    /// );
    /// assert!(channel.is_megagroup());
    /// ```
    pub fn is_megagroup(&self) -> bool {
        self.is_megagroup
    }

    /// Sets the channel title.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_min_channel::MinChannel;
    ///
    /// let mut channel = MinChannel::default();
    /// channel.set_title("New Title".to_string());
    /// assert_eq!(channel.title(), "New Title");
    /// ```
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Sets the channel photo.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_min_channel::MinChannel;
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let mut channel = MinChannel::default();
    /// let new_photo = DialogPhoto::default();
    /// channel.set_photo(new_photo);
    /// ```
    pub fn set_photo(&mut self, photo: DialogPhoto) {
        self.photo = photo;
    }

    /// Sets the accent color ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_min_channel::MinChannel;
    /// use rustgram_accent_color_id::AccentColorId;
    ///
    /// let mut channel = MinChannel::default();
    /// channel.set_accent_color_id(AccentColorId::new(5));
    /// ```
    pub fn set_accent_color_id(&mut self, accent_color_id: AccentColorId) {
        self.accent_color_id = accent_color_id;
    }

    /// Sets whether this is a megagroup.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_min_channel::MinChannel;
    ///
    /// let mut channel = MinChannel::default();
    /// channel.set_is_megagroup(true);
    /// assert!(channel.is_megagroup());
    /// ```
    pub fn set_is_megagroup(&mut self, is_megagroup: bool) {
        self.is_megagroup = is_megagroup;
    }
}

impl Default for MinChannel {
    fn default() -> Self {
        Self {
            title: String::new(),
            photo: DialogPhoto::default(),
            accent_color_id: AccentColorId::default(),
            is_megagroup: false,
        }
    }
}

impl Display for MinChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MinChannel(title={}, is_megagroup={}, accent_color={:?})",
            self.title, self.is_megagroup, self.accent_color_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let channel = MinChannel::new(
            "Test Channel".to_string(),
            DialogPhoto::default(),
            AccentColorId::default(),
            true,
        );
        assert_eq!(channel.title(), "Test Channel");
        assert!(channel.is_megagroup());
    }

    #[test]
    fn test_default() {
        let channel = MinChannel::default();
        assert_eq!(channel.title(), "");
        assert!(!channel.is_megagroup());
    }

    #[test]
    fn test_title() {
        let channel = MinChannel::new(
            "My Title".to_string(),
            DialogPhoto::default(),
            AccentColorId::default(),
            false,
        );
        assert_eq!(channel.title(), "My Title");
    }

    #[test]
    fn test_photo() {
        let photo = DialogPhoto::default();
        let channel = MinChannel::new(
            "Test".to_string(),
            photo.clone(),
            AccentColorId::default(),
            false,
        );
        assert_eq!(channel.photo(), &photo);
    }

    #[test]
    fn test_accent_color_id() {
        let color_id = AccentColorId::new(5);
        let channel = MinChannel::new(
            "Test".to_string(),
            DialogPhoto::default(),
            color_id,
            false,
        );
        assert_eq!(channel.accent_color_id().get(), 5);
    }

    #[test]
    fn test_is_megagroup() {
        let megagroup = MinChannel::new(
            "Group".to_string(),
            DialogPhoto::default(),
            AccentColorId::default(),
            true,
        );
        assert!(megagroup.is_megagroup());

        let channel = MinChannel::new(
            "Channel".to_string(),
            DialogPhoto::default(),
            AccentColorId::default(),
            false,
        );
        assert!(!channel.is_megagroup());
    }

    #[test]
    fn test_set_title() {
        let mut channel = MinChannel::default();
        channel.set_title("New Title".to_string());
        assert_eq!(channel.title(), "New Title");
    }

    #[test]
    fn test_set_photo() {
        let mut channel = MinChannel::default();
        let new_photo = DialogPhoto::default();
        channel.set_photo(new_photo.clone());
        assert_eq!(channel.photo(), &new_photo);
    }

    #[test]
    fn test_set_accent_color_id() {
        let mut channel = MinChannel::default();
        channel.set_accent_color_id(AccentColorId::new(10));
        assert_eq!(channel.accent_color_id().get(), 10);
    }

    #[test]
    fn test_set_is_megagroup() {
        let mut channel = MinChannel::default();
        assert!(!channel.is_megagroup());
        channel.set_is_megagroup(true);
        assert!(channel.is_megagroup());
        channel.set_is_megagroup(false);
        assert!(!channel.is_megagroup());
    }

    #[test]
    fn test_equality() {
        let channel1 = MinChannel::new(
            "Test".to_string(),
            DialogPhoto::default(),
            AccentColorId::new(1),
            true,
        );
        let channel2 = MinChannel::new(
            "Test".to_string(),
            DialogPhoto::default(),
            AccentColorId::new(1),
            true,
        );
        let channel3 = MinChannel::new(
            "Other".to_string(),
            DialogPhoto::default(),
            AccentColorId::new(1),
            true,
        );

        assert_eq!(channel1, channel2);
        assert_ne!(channel1, channel3);
    }

    #[test]
    fn test_clone() {
        let channel1 = MinChannel::new(
            "Test".to_string(),
            DialogPhoto::default(),
            AccentColorId::new(3),
            false,
        );
        let channel2 = channel1.clone();
        assert_eq!(channel1, channel2);
    }

    #[test]
    fn test_display() {
        let channel = MinChannel::new(
            "Test Channel".to_string(),
            DialogPhoto::default(),
            AccentColorId::new(5),
            true,
        );
        let display = format!("{}", channel);
        assert!(display.contains("Test Channel"));
        assert!(display.contains("is_megagroup=true"));
    }

    #[test]
    fn test_debug() {
        let channel = MinChannel::new(
            "Debug".to_string(),
            DialogPhoto::default(),
            AccentColorId::default(),
            false,
        );
        let debug = format!("{:?}", channel);
        assert!(debug.contains("Debug"));
    }

    #[test]
    fn test_with_empty_title() {
        let channel = MinChannel::new(
            "".to_string(),
            DialogPhoto::default(),
            AccentColorId::default(),
            false,
        );
        assert_eq!(channel.title(), "");
    }

    #[test]
    fn test_channel_vs_megagroup() {
        let channel = MinChannel::new(
            "Broadcast Channel".to_string(),
            DialogPhoto::default(),
            AccentColorId::default(),
            false,
        );
        assert!(!channel.is_megagroup());

        let megagroup = MinChannel::new(
            "Megagroup".to_string(),
            DialogPhoto::default(),
            AccentColorId::default(),
            true,
        );
        assert!(megagroup.is_megagroup());
    }
}
