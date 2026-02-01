// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Theme Settings
//!
//! Theme settings management for Telegram client.
//!
//! ## Overview
//!
//! This module provides the [`ThemeSettings`] struct, which represents
//! theme settings including colors, background, and base theme.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_theme_settings::ThemeSettings;
//! use rustgram_base_theme::BaseTheme;
//! use rustgram_background_info::{BackgroundInfo, BackgroundType};
//!
//! // Create theme settings
//! let settings = ThemeSettings::new(
//!     0x3D6DCC,  // accent color
//!     0x3D6DCC,  // message accent color
//!     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
//!     BaseTheme::Classic
//! );
//! ```

use std::fmt;

use rustgram_background_info::{BackgroundInfo, BackgroundType};
use rustgram_base_theme::BaseTheme;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Maximum number of message colors allowed.
const MAX_MESSAGE_COLORS: usize = 4;

/// Theme settings for Telegram.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib's `ThemeSettings` class.
///
/// # Example
///
/// ```rust
/// use rustgram_theme_settings::ThemeSettings;
/// use rustgram_base_theme::BaseTheme;
/// use rustgram_background_info::{BackgroundInfo, BackgroundType};
///
/// let settings = ThemeSettings::new(
///     0x3D6DCC,
///     0x3D6DCC,
///     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
///     BaseTheme::Classic
/// );
/// assert!(!settings.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThemeSettings {
    accent_color: i32,
    message_accent_color: i32,
    background_info: BackgroundInfo,
    base_theme: BaseTheme,
    message_colors: Vec<i32>,
    animate_message_colors: bool,
}

impl ThemeSettings {
    /// Creates new theme settings.
    ///
    /// # Arguments
    ///
    /// * `accent_color` - The accent color (RGB value)
    /// * `message_accent_color` - The accent color for messages
    /// * `background_info` - Background information
    /// * `base_theme` - The base theme
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    /// use rustgram_base_theme::BaseTheme;
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let settings = ThemeSettings::new(
    ///     0x3D6DCC,
    ///     0x3D6DCC,
    ///     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
    ///     BaseTheme::Classic
    /// );
    /// ```
    #[must_use]
    pub fn new(
        accent_color: i32,
        message_accent_color: i32,
        background_info: BackgroundInfo,
        base_theme: BaseTheme,
    ) -> Self {
        Self {
            accent_color,
            message_accent_color,
            background_info,
            base_theme,
            message_colors: Vec::new(),
            animate_message_colors: false,
        }
    }

    /// Returns the accent color.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    /// use rustgram_base_theme::BaseTheme;
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let settings = ThemeSettings::new(
    ///     0x3D6DCC,
    ///     0x3D6DCC,
    ///     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
    ///     BaseTheme::Classic
    /// );
    /// assert_eq!(settings.accent_color(), 0x3D6DCC);
    /// ```
    #[must_use]
    pub const fn accent_color(&self) -> i32 {
        self.accent_color
    }

    /// Returns the message accent color.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    /// use rustgram_base_theme::BaseTheme;
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let settings = ThemeSettings::new(
    ///     0x3D6DCC,
    ///     0x4A7DFF,
    ///     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
    ///     BaseTheme::Classic
    /// );
    /// assert_eq!(settings.message_accent_color(), 0x4A7DFF);
    /// ```
    #[must_use]
    pub const fn message_accent_color(&self) -> i32 {
        self.message_accent_color
    }

    /// Returns the background information.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    /// use rustgram_base_theme::BaseTheme;
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let bg = BackgroundInfo::new(12345, BackgroundType::Wallpaper);
    /// let settings = ThemeSettings::new(0x3D6DCC, 0x3D6DCC, bg.clone(), BaseTheme::Classic);
    /// assert_eq!(settings.background_info().background_id(), 12345);
    /// ```
    #[must_use]
    pub const fn background_info(&self) -> &BackgroundInfo {
        &self.background_info
    }

    /// Returns the base theme.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    /// use rustgram_base_theme::BaseTheme;
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let settings = ThemeSettings::new(
    ///     0x3D6DCC,
    ///     0x3D6DCC,
    ///     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
    ///     BaseTheme::Night
    /// );
    /// assert_eq!(settings.base_theme(), BaseTheme::Night);
    /// ```
    #[must_use]
    pub const fn base_theme(&self) -> BaseTheme {
        self.base_theme
    }

    /// Returns the message colors.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    /// use rustgram_base_theme::BaseTheme;
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let settings = ThemeSettings::new(
    ///     0x3D6DCC,
    ///     0x3D6DCC,
    ///     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
    ///     BaseTheme::Classic
    /// );
    /// assert!(settings.message_colors().is_empty());
    /// ```
    #[must_use]
    pub fn message_colors(&self) -> &[i32] {
        &self.message_colors
    }

    /// Returns whether to animate message colors.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    /// use rustgram_base_theme::BaseTheme;
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let settings = ThemeSettings::new(
    ///     0x3D6DCC,
    ///     0x3D6DCC,
    ///     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
    ///     BaseTheme::Classic
    /// );
    /// assert!(!settings.animate_message_colors());
    /// ```
    #[must_use]
    pub const fn animate_message_colors(&self) -> bool {
        self.animate_message_colors
    }

    /// Sets the message colors.
    ///
    /// # Arguments
    ///
    /// * `colors` - The message colors (max 4)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    /// use rustgram_base_theme::BaseTheme;
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let mut settings = ThemeSettings::new(
    ///     0x3D6DCC,
    ///     0x3D6DCC,
    ///     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
    ///     BaseTheme::Classic
    /// );
    /// settings.set_message_colors(vec![0xFF0000, 0x00FF00]);
    /// assert_eq!(settings.message_colors().len(), 2);
    /// ```
    pub fn set_message_colors(&mut self, mut colors: Vec<i32>) {
        if colors.len() > MAX_MESSAGE_COLORS {
            colors.truncate(MAX_MESSAGE_COLORS);
        }
        self.message_colors = colors;
    }

    /// Sets whether to animate message colors.
    ///
    /// # Arguments
    ///
    /// * `animate` - Whether to animate message colors
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    /// use rustgram_base_theme::BaseTheme;
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let mut settings = ThemeSettings::new(
    ///     0x3D6DCC,
    ///     0x3D6DCC,
    ///     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
    ///     BaseTheme::Classic
    /// );
    /// settings.set_animate_message_colors(true);
    /// assert!(settings.animate_message_colors());
    /// ```
    pub fn set_animate_message_colors(&mut self, animate: bool) {
        self.animate_message_colors = animate;
    }

    /// Checks if these theme settings are empty (default).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    ///
    /// let settings = ThemeSettings::default();
    /// assert!(settings.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.accent_color == 0
            && self.message_accent_color == 0
            && !self.background_info.is_valid()
            && self.base_theme == BaseTheme::default()
            && self.message_colors.is_empty()
            && !self.animate_message_colors
    }

    /// Checks if this is a dark theme.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    /// use rustgram_base_theme::BaseTheme;
    /// use rustgram_background_info::{BackgroundInfo, BackgroundType};
    ///
    /// let settings = ThemeSettings::new(
    ///     0x3D6DCC,
    ///     0x3D6DCC,
    ///     BackgroundInfo::new(12345, BackgroundType::Wallpaper),
    ///     BaseTheme::Night
    /// );
    /// assert!(settings.are_dark());
    /// ```
    #[must_use]
    pub fn are_dark(&self) -> bool {
        self.base_theme.is_dark()
    }
}

impl Default for ThemeSettings {
    /// Creates default theme settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    ///
    /// let settings = ThemeSettings::default();
    /// assert!(settings.is_empty());
    /// ```
    fn default() -> Self {
        Self {
            accent_color: 0,
            message_accent_color: 0,
            background_info: BackgroundInfo::default(),
            base_theme: BaseTheme::default(),
            message_colors: Vec::new(),
            animate_message_colors: false,
        }
    }
}

impl fmt::Display for ThemeSettings {
    /// Formats the theme settings for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_theme_settings::ThemeSettings;
    ///
    /// let settings = ThemeSettings::default();
    /// let s = format!("{}", settings);
    /// assert!(s.contains("ThemeSettings"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ThemeSettings(accent: #{:06X}, base: {:?}, dark: {})",
            self.accent_color as u32 & 0xFFFFFF,
            self.base_theme,
            self.are_dark()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let settings = ThemeSettings::new(
            0x3D6DCC,
            0x3D6DCC,
            BackgroundInfo::new(12345, BackgroundType::Wallpaper),
            BaseTheme::Classic,
        );
        assert_eq!(settings.accent_color(), 0x3D6DCC);
        assert_eq!(settings.message_accent_color(), 0x3D6DCC);
    }

    #[test]
    fn test_accent_color() {
        let settings =
            ThemeSettings::new(0xFF0000, 0, BackgroundInfo::default(), BaseTheme::default());
        assert_eq!(settings.accent_color(), 0xFF0000);
    }

    #[test]
    fn test_message_accent_color() {
        let settings =
            ThemeSettings::new(0, 0x00FF00, BackgroundInfo::default(), BaseTheme::default());
        assert_eq!(settings.message_accent_color(), 0x00FF00);
    }

    #[test]
    fn test_background_info() {
        let bg = BackgroundInfo::new(12345, BackgroundType::Pattern);
        let settings = ThemeSettings::new(0, 0, bg.clone(), BaseTheme::default());
        assert_eq!(settings.background_info().background_id(), 12345);
    }

    #[test]
    fn test_base_theme() {
        let settings = ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::Night);
        assert_eq!(settings.base_theme(), BaseTheme::Night);
    }

    #[test]
    fn test_message_colors_default() {
        let settings = ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::default());
        assert!(settings.message_colors().is_empty());
    }

    #[test]
    fn test_set_message_colors() {
        let mut settings =
            ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::default());
        settings.set_message_colors(vec![0xFF0000, 0x00FF00, 0x0000FF]);
        assert_eq!(settings.message_colors().len(), 3);
    }

    #[test]
    fn test_set_message_colors_truncate() {
        let mut settings =
            ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::default());
        let colors = vec![1, 2, 3, 4, 5, 6];
        settings.set_message_colors(colors);
        assert_eq!(settings.message_colors().len(), MAX_MESSAGE_COLORS);
    }

    #[test]
    fn test_animate_message_colors_default() {
        let settings = ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::default());
        assert!(!settings.animate_message_colors());
    }

    #[test]
    fn test_set_animate_message_colors() {
        let mut settings =
            ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::default());
        settings.set_animate_message_colors(true);
        assert!(settings.animate_message_colors());
    }

    #[test]
    fn test_is_empty_true() {
        let settings = ThemeSettings::default();
        assert!(settings.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let settings =
            ThemeSettings::new(0x3D6DCC, 0, BackgroundInfo::default(), BaseTheme::default());
        assert!(!settings.is_empty());
    }

    #[test]
    fn test_are_dark_classic() {
        let settings = ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::Classic);
        assert!(!settings.are_dark());
    }

    #[test]
    fn test_are_dark_night() {
        let settings = ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::Night);
        assert!(settings.are_dark());
    }

    #[test]
    fn test_default() {
        let settings = ThemeSettings::default();
        assert_eq!(settings.accent_color(), 0);
        assert_eq!(settings.message_accent_color(), 0);
        assert!(settings.is_empty());
    }

    #[test]
    fn test_equality() {
        let settings1 = ThemeSettings::new(
            0x3D6DCC,
            0x3D6DCC,
            BackgroundInfo::new(12345, BackgroundType::Wallpaper),
            BaseTheme::Classic,
        );
        let settings2 = ThemeSettings::new(
            0x3D6DCC,
            0x3D6DCC,
            BackgroundInfo::new(12345, BackgroundType::Wallpaper),
            BaseTheme::Classic,
        );
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_inequality_accent() {
        let settings1 =
            ThemeSettings::new(0xFF0000, 0, BackgroundInfo::default(), BaseTheme::default());
        let settings2 =
            ThemeSettings::new(0x00FF00, 0, BackgroundInfo::default(), BaseTheme::default());
        assert_ne!(settings1, settings2);
    }

    #[test]
    fn test_inequality_base_theme() {
        let settings1 = ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::Classic);
        let settings2 = ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::Night);
        assert_ne!(settings1, settings2);
    }

    #[test]
    fn test_clone_semantics() {
        let settings1 =
            ThemeSettings::new(0x3D6DCC, 0, BackgroundInfo::default(), BaseTheme::Classic);
        let settings2 = settings1.clone();
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_display_format() {
        let settings =
            ThemeSettings::new(0xFF0000, 0, BackgroundInfo::default(), BaseTheme::Classic);
        let s = format!("{}", settings);
        assert!(s.contains("ThemeSettings"));
    }

    #[test]
    fn test_debug_format() {
        let settings =
            ThemeSettings::new(0xFF0000, 0, BackgroundInfo::default(), BaseTheme::Classic);
        let debug_str = format!("{:?}", settings);
        assert!(debug_str.contains("ThemeSettings"));
    }

    #[test]
    fn test_with_background() {
        let bg = BackgroundInfo::new(999, BackgroundType::Fill);
        let settings = ThemeSettings::new(0, 0, bg, BaseTheme::Day);
        assert!(!settings.is_empty());
    }

    #[test]
    fn test_with_message_colors_and_animation() {
        let mut settings =
            ThemeSettings::new(0, 0, BackgroundInfo::default(), BaseTheme::default());
        settings.set_message_colors(vec![0xFF0000, 0x00FF00]);
        settings.set_animate_message_colors(true);
        assert!(settings.animate_message_colors());
        assert_eq!(settings.message_colors().len(), 2);
    }

    #[test]
    fn test_all_base_themes() {
        for theme in [
            BaseTheme::Classic,
            BaseTheme::Day,
            BaseTheme::Night,
            BaseTheme::Tinted,
            BaseTheme::Arctic,
        ] {
            let settings = ThemeSettings::new(0, 0, BackgroundInfo::default(), theme);
            assert_eq!(settings.base_theme(), theme);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = ThemeSettings::new(
            0x3D6DCC,
            0x3D6DCC,
            BackgroundInfo::new(12345, BackgroundType::Wallpaper),
            BaseTheme::Classic,
        );
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ThemeSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_with_colors() {
        let mut original = ThemeSettings::new(
            0xFF0000,
            0x00FF00,
            BackgroundInfo::default(),
            BaseTheme::Night,
        );
        original.set_message_colors(vec![0x111111, 0x222222]);
        original.set_animate_message_colors(true);

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ThemeSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
