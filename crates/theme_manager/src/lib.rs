// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Theme Manager
//!
//! Theme management for Telegram chat themes and accent colors.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_accent_color_id::AccentColorId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Accent color entry with ID and colors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccentColorEntry {
    /// Accent color ID
    pub id: i32,
    /// Light theme colors
    pub light_colors: Vec<i32>,
    /// Dark theme colors
    pub dark_colors: Vec<i32>,
}

impl AccentColorEntry {
    /// Creates a new accent color entry
    #[must_use]
    pub fn new(id: i32) -> Self {
        Self {
            id,
            light_colors: Vec::new(),
            dark_colors: Vec::new(),
        }
    }

    /// Returns the accent color ID
    #[must_use]
    pub const fn accent_color_id(&self) -> AccentColorId {
        AccentColorId::new(self.id)
    }
}

/// Chat theme with emoji and settings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmojiChatTheme {
    /// Emoji representing the theme
    pub emoji: String,
    /// Unique theme ID
    pub id: i64,
    /// Light theme settings
    pub light_theme: ThemeSettings,
    /// Dark theme settings
    pub dark_theme: ThemeSettings,
}

impl EmojiChatTheme {
    /// Creates a new emoji chat theme
    #[must_use]
    pub fn new(emoji: String, id: i64) -> Self {
        Self {
            emoji,
            id,
            light_theme: ThemeSettings::default(),
            dark_theme: ThemeSettings::default(),
        }
    }

    /// Returns true if the theme has valid settings
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.light_theme.is_empty() && !self.dark_theme.is_empty()
    }
}

/// Theme settings for a chat theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeSettings {
    /// Background color (RGB)
    pub background_color: i32,
    /// Secondary background color (RGB)
    pub secondary_background_color: i32,
    /// Text color (RGB)
    pub text_color: i32,
    /// Hint color (RGB)
    pub hint_color: i32,
    /// Link color (RGB)
    pub link_color: i32,
    /// Button color (RGB)
    pub button_color: i32,
    /// Button text color (RGB)
    pub button_text_color: i32,
    /// Whether this is a dark theme
    pub is_dark: bool,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            background_color: 0xFFFFFF,
            secondary_background_color: 0xE6E6E6,
            text_color: 0x000000,
            hint_color: 0x888888,
            link_color: 0x2481CC,
            button_color: 0x2481CC,
            button_text_color: 0xFFFFFF,
            is_dark: false,
        }
    }
}

impl ThemeSettings {
    /// Creates a new empty theme settings
    #[must_use]
    pub const fn new() -> Self {
        Self {
            background_color: 0,
            secondary_background_color: 0,
            text_color: 0,
            hint_color: 0,
            link_color: 0,
            button_color: 0,
            button_text_color: 0,
            is_dark: false,
        }
    }

    /// Returns true if the theme is empty (no colors set)
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.background_color == 0 && self.secondary_background_color == 0 && self.text_color == 0
    }

    /// Returns true if this is a dark theme
    #[must_use]
    pub const fn is_dark_theme(&self) -> bool {
        self.is_dark
    }
}

/// Collection of chat themes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmojiChatThemes {
    /// Hash for cache validation
    pub hash: i64,
    /// List of themes
    pub themes: Vec<EmojiChatTheme>,
}

impl Default for EmojiChatThemes {
    fn default() -> Self {
        Self::new()
    }
}

impl EmojiChatThemes {
    /// Creates a new empty chat themes collection
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hash: 0,
            themes: Vec::new(),
        }
    }

    /// Returns the number of themes
    #[must_use]
    pub fn len(&self) -> usize {
        self.themes.len()
    }

    /// Returns true if there are no themes
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }

    /// Adds a theme to the collection
    pub fn add_theme(&mut self, theme: EmojiChatTheme) {
        self.themes.push(theme);
    }

    /// Finds a theme by emoji
    #[must_use]
    pub fn find_by_emoji(&self, emoji: &str) -> Option<&EmojiChatTheme> {
        self.themes.iter().find(|t| t.emoji == emoji)
    }
}

/// Accent color palette
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccentColors {
    /// List of accent colors
    pub colors: Vec<AccentColorEntry>,
    /// Minimum boost levels for broadcast channels
    pub min_broadcast_boost_levels: Vec<i32>,
    /// Minimum boost levels for megagroups
    pub min_megagroup_boost_levels: Vec<i32>,
    /// Hash for cache validation
    pub hash: i32,
}

impl Default for AccentColors {
    fn default() -> Self {
        Self::new()
    }
}

impl AccentColors {
    /// Creates a new empty accent colors collection
    #[must_use]
    pub const fn new() -> Self {
        Self {
            colors: Vec::new(),
            min_broadcast_boost_levels: Vec::new(),
            min_megagroup_boost_levels: Vec::new(),
            hash: 0,
        }
    }

    /// Returns true if there are no accent colors
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Returns the number of accent colors
    #[must_use]
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Gets light colors for an accent color ID
    #[must_use]
    pub fn get_light_colors(&self, accent_color_id: AccentColorId) -> Option<&[i32]> {
        let id = accent_color_id.get();
        self.colors
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.light_colors.as_slice())
    }

    /// Gets dark colors for an accent color ID
    #[must_use]
    pub fn get_dark_colors(&self, accent_color_id: AccentColorId) -> Option<&[i32]> {
        let id = accent_color_id.get();
        self.colors
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.dark_colors.as_slice())
    }

    /// Checks if an accent color ID exists
    #[must_use]
    pub fn contains_key(&self, accent_color_id: AccentColorId) -> bool {
        let id = accent_color_id.get();
        self.colors.iter().any(|e| e.id == id)
    }

    /// Gets available accent color IDs
    #[must_use]
    pub fn accent_color_ids(&self) -> Vec<AccentColorId> {
        self.colors
            .iter()
            .map(|e| AccentColorId::new(e.id))
            .collect()
    }
}

/// Profile accent color with multiple palettes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileAccentColor {
    /// Accent color ID
    pub id: i32,
    /// Palette colors
    pub palette_colors: Vec<i32>,
    /// Background colors
    pub background_colors: Vec<i32>,
    /// Story colors
    pub story_colors: Vec<i32>,
}

impl ProfileAccentColor {
    /// Creates a new profile accent color
    #[must_use]
    pub fn new(id: i32) -> Self {
        Self {
            id,
            palette_colors: Vec::new(),
            background_colors: Vec::new(),
            story_colors: Vec::new(),
        }
    }

    /// Returns the accent color ID
    #[must_use]
    pub const fn accent_color_id(&self) -> AccentColorId {
        AccentColorId::new(self.id)
    }

    /// Returns true if the color is valid
    #[must_use]
    pub fn is_valid(&self) -> bool {
        let valid_colors = |colors: &[i32]| -> bool {
            (1..=2).contains(&colors.len()) && colors.iter().all(|&c| c >= 0 && c <= 0xFFFFFF)
        };
        valid_colors(&self.palette_colors)
            && valid_colors(&self.background_colors)
            && self.story_colors.len() == 2
            && self.story_colors.iter().all(|&c| c >= 0 && c <= 0xFFFFFF)
    }

    /// Returns true if empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.palette_colors.is_empty()
            && self.background_colors.is_empty()
            && self.story_colors.is_empty()
    }
}

impl Default for ProfileAccentColor {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Collection of profile accent colors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileAccentColors {
    /// List of profile accent colors
    pub colors: Vec<ProfileAccentColor>,
    /// Minimum boost levels for broadcast channels
    pub min_broadcast_boost_levels: Vec<i32>,
    /// Minimum boost levels for megagroups
    pub min_megagroup_boost_levels: Vec<i32>,
    /// Hash for cache validation
    pub hash: i32,
}

impl Default for ProfileAccentColors {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileAccentColors {
    /// Creates a new empty profile accent colors collection
    #[must_use]
    pub const fn new() -> Self {
        Self {
            colors: Vec::new(),
            min_broadcast_boost_levels: Vec::new(),
            min_megagroup_boost_levels: Vec::new(),
            hash: 0,
        }
    }

    /// Returns true if there are no profile accent colors
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Returns the number of profile accent colors
    #[must_use]
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Checks if an accent color ID exists
    #[must_use]
    pub fn contains_key(&self, accent_color_id: AccentColorId) -> bool {
        let id = accent_color_id.get();
        self.colors.iter().any(|e| e.id == id)
    }

    /// Gets available accent color IDs
    #[must_use]
    pub fn accent_color_ids(&self) -> Vec<AccentColorId> {
        self.colors
            .iter()
            .map(|e| AccentColorId::new(e.id))
            .collect()
    }
}

/// Dialog boost available counts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogBoostAvailableCounts {
    /// Number of title colors available
    pub title_color_count: i32,
    /// Number of accent colors available
    pub accent_color_count: i32,
    /// Number of profile accent colors available
    pub profile_accent_color_count: i32,
    /// Number of chat themes available
    pub chat_theme_count: i32,
}

impl Default for DialogBoostAvailableCounts {
    fn default() -> Self {
        Self {
            title_color_count: 0,
            accent_color_count: 0,
            profile_accent_color_count: 0,
            chat_theme_count: 0,
        }
    }
}

/// Theme manager for chat themes and accent colors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeManager {
    /// Chat themes
    chat_themes: EmojiChatThemes,
    /// Accent colors
    accent_colors: AccentColors,
    /// Profile accent colors
    profile_accent_colors: ProfileAccentColors,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeManager {
    /// Creates a new theme manager
    #[must_use]
    pub const fn new() -> Self {
        Self {
            chat_themes: EmojiChatThemes::new(),
            accent_colors: AccentColors::new(),
            profile_accent_colors: ProfileAccentColors::new(),
        }
    }

    /// Returns the number of chat themes
    #[must_use]
    pub fn chat_theme_count(&self) -> usize {
        self.chat_themes.len()
    }

    /// Returns the chat themes
    #[must_use]
    pub fn chat_themes(&self) -> &[EmojiChatTheme] {
        &self.chat_themes.themes
    }

    /// Returns the accent colors
    #[must_use]
    pub fn accent_colors(&self) -> &AccentColors {
        &self.accent_colors
    }

    /// Returns the profile accent colors
    #[must_use]
    pub fn profile_accent_colors(&self) -> &ProfileAccentColors {
        &self.profile_accent_colors
    }

    /// Gets the accent color ID for a given accent color
    #[must_use]
    pub fn get_accent_color_id_object(
        &self,
        accent_color_id: AccentColorId,
        fallback_accent_color_id: AccentColorId,
    ) -> i32 {
        if accent_color_id.is_valid()
            && (accent_color_id.is_built_in() || self.accent_colors.contains_key(accent_color_id))
        {
            return accent_color_id.get();
        }
        if fallback_accent_color_id.is_valid() {
            return fallback_accent_color_id.get();
        }
        5 // Default to blue
    }

    /// Gets the profile accent color ID
    #[must_use]
    pub fn get_profile_accent_color_id_object(
        &self,
        accent_color_id: AccentColorId,
    ) -> Option<i32> {
        if accent_color_id.is_valid()
            && (accent_color_id.is_built_in()
                || self.profile_accent_colors.contains_key(accent_color_id))
        {
            return Some(accent_color_id.get());
        }
        None
    }

    /// Gets available boost counts for a dialog
    #[must_use]
    pub fn get_dialog_boost_available_count(
        &self,
        level: i32,
        _for_megagroup: bool,
    ) -> DialogBoostAvailableCounts {
        let mut result = DialogBoostAvailableCounts::default();

        // Chat themes are available at certain boost levels
        if level >= 5 {
            result.chat_theme_count = self.chat_themes.len() as i32;
        }

        // Count accent colors available at this level
        for (i, &min_level) in self
            .accent_colors
            .min_broadcast_boost_levels
            .iter()
            .enumerate()
        {
            if level >= min_level && i < self.accent_colors.colors.len() {
                result.accent_color_count += 1;
                let entry = &self.accent_colors.colors[i];
                let accent_color_id = AccentColorId::new(entry.id);
                if accent_color_id.is_built_in() {
                    result.title_color_count += 1;
                } else if entry.light_colors.len() == 1 {
                    result.title_color_count += 1;
                }
            }
        }

        // Count profile accent colors available at this level
        for (i, &min_level) in self
            .profile_accent_colors
            .min_broadcast_boost_levels
            .iter()
            .enumerate()
        {
            if level >= min_level && i < self.profile_accent_colors.colors.len() {
                result.profile_accent_color_count += 1;
            }
        }

        result
    }

    /// Updates chat themes from server data
    pub fn update_chat_themes(&mut self, themes: Vec<EmojiChatTheme>, hash: i64) {
        self.chat_themes.themes = themes;
        self.chat_themes.hash = hash;
    }

    /// Updates accent colors from server data
    pub fn update_accent_colors(&mut self, accent_colors: AccentColors) {
        self.accent_colors = accent_colors;
    }

    /// Updates profile accent colors from server data
    pub fn update_profile_accent_colors(&mut self, colors: ProfileAccentColors) {
        self.profile_accent_colors = colors;
    }

    /// Finds a chat theme by emoji
    #[must_use]
    pub fn find_theme_by_emoji(&self, emoji: &str) -> Option<&EmojiChatTheme> {
        self.chat_themes.find_by_emoji(emoji)
    }

    /// Converts theme parameters to JSON string
    #[must_use]
    pub fn theme_parameters_to_json(
        background_color: i32,
        secondary_background_color: i32,
        text_color: i32,
        hint_color: i32,
        link_color: i32,
        button_color: i32,
        button_text_color: i32,
    ) -> String {
        format!(
            r#"{{"bg_color":"{}","secondary_bg_color":"{}","text_color":"{}","hint_color":"{}","link_color":"{}","button_color":"{}","button_text_color":"{}"}}"#,
            color_to_hex(background_color),
            color_to_hex(secondary_background_color),
            color_to_hex(text_color),
            color_to_hex(hint_color),
            color_to_hex(link_color),
            color_to_hex(button_color),
            color_to_hex(button_text_color)
        )
    }
}

/// Converts an RGB color to hex string
#[must_use]
fn color_to_hex(color: i32) -> String {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

impl fmt::Display for ThemeManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ThemeManager({} themes, {} accent colors, {} profile colors)",
            self.chat_themes.len(),
            self.accent_colors.len(),
            self.profile_accent_colors.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager_new() {
        let manager = ThemeManager::new();
        assert_eq!(manager.chat_theme_count(), 0);
        assert!(manager.chat_themes().is_empty());
    }

    #[test]
    fn test_emoji_chat_theme_new() {
        let theme = EmojiChatTheme::new("🎨".to_string(), 123);
        assert_eq!(theme.emoji, "🎨");
        assert_eq!(theme.id, 123);
    }

    #[test]
    fn test_theme_settings_new() {
        let settings = ThemeSettings::new();
        assert!(settings.is_empty());
        assert!(!settings.is_dark_theme());
    }

    #[test]
    fn test_theme_settings_default() {
        let settings = ThemeSettings::default();
        assert!(!settings.is_empty());
        assert_eq!(settings.background_color, 0xFFFFFF);
    }

    #[test]
    fn test_accent_colors_new() {
        let colors = AccentColors::new();
        assert!(colors.is_empty());
        assert_eq!(colors.len(), 0);
    }

    #[test]
    fn test_accent_color_entry() {
        let entry = AccentColorEntry::new(5);
        assert_eq!(entry.id, 5);
        assert_eq!(entry.accent_color_id().get(), 5);
    }

    #[test]
    fn test_profile_accent_color_new() {
        let color = ProfileAccentColor::new(1);
        assert!(color.is_empty());
        assert!(!color.is_valid()); // Empty is not valid
    }

    #[test]
    fn test_profile_accent_color_valid() {
        let mut color = ProfileAccentColor::new(1);
        color.palette_colors = vec![0xFF0000];
        color.background_colors = vec![0x00FF00];
        color.story_colors = vec![0x0000FF, 0xFFFF00];
        assert!(color.is_valid());
    }

    #[test]
    fn test_dialog_boost_counts_default() {
        let counts = DialogBoostAvailableCounts::default();
        assert_eq!(counts.title_color_count, 0);
        assert_eq!(counts.chat_theme_count, 0);
    }

    #[test]
    fn test_get_accent_color_id_object() {
        let manager = ThemeManager::new();
        let accent_id = AccentColorId::new(5);
        let fallback = AccentColorId::new(3);
        assert_eq!(manager.get_accent_color_id_object(accent_id, fallback), 5);
    }

    #[test]
    fn test_get_accent_color_id_object_fallback() {
        let manager = ThemeManager::new();
        let invalid_id = AccentColorId::new(-1);
        let fallback = AccentColorId::new(3);
        assert_eq!(manager.get_accent_color_id_object(invalid_id, fallback), 3);
    }

    #[test]
    fn test_get_profile_accent_color_id_object() {
        let manager = ThemeManager::new();
        let accent_id = AccentColorId::new(5);
        assert_eq!(
            manager.get_profile_accent_color_id_object(accent_id),
            Some(5)
        );
    }

    #[test]
    fn test_get_profile_accent_color_id_object_invalid() {
        let manager = ThemeManager::new();
        let invalid_id = AccentColorId::new(-1);
        assert_eq!(manager.get_profile_accent_color_id_object(invalid_id), None);
    }

    #[test]
    fn test_update_chat_themes() {
        let mut manager = ThemeManager::new();
        let theme = EmojiChatTheme::new("🎨".to_string(), 1);
        manager.update_chat_themes(vec![theme], 123);
        assert_eq!(manager.chat_theme_count(), 1);
    }

    #[test]
    fn test_get_dialog_boost_available_count() {
        let manager = ThemeManager::new();
        let counts = manager.get_dialog_boost_available_count(0, false);
        assert_eq!(counts.chat_theme_count, 0);
    }

    #[test]
    fn test_color_to_hex() {
        assert_eq!(color_to_hex(0xFF0000), "#FF0000");
        assert_eq!(color_to_hex(0x00FF00), "#00FF00");
        assert_eq!(color_to_hex(0x0000FF), "#0000FF");
        assert_eq!(color_to_hex(0xFFFFFF), "#FFFFFF");
    }

    #[test]
    fn test_theme_parameters_to_json() {
        let json = ThemeManager::theme_parameters_to_json(
            0xFF0000, 0x00FF00, 0x0000FF, 0x888888, 0x2481CC, 0x2481CC, 0xFFFFFF,
        );
        assert!(json.contains("\"bg_color\":\"#FF0000\""));
        assert!(json.contains("\"text_color\":\"#0000FF\""));
    }

    #[test]
    fn test_emoji_chat_themes_add_theme() {
        let mut themes = EmojiChatThemes::new();
        let theme = EmojiChatTheme::new("🎨".to_string(), 1);
        themes.add_theme(theme);
        assert_eq!(themes.len(), 1);
    }

    #[test]
    fn test_emoji_chat_themes_find_by_emoji() {
        let mut themes = EmojiChatThemes::new();
        let theme = EmojiChatTheme::new("🎨".to_string(), 1);
        themes.add_theme(theme);
        assert!(themes.find_by_emoji("🎨").is_some());
        assert!(themes.find_by_emoji("❌").is_none());
    }

    #[test]
    fn test_accent_colors_contains_key() {
        let mut colors = AccentColors::new();
        colors.colors.push(AccentColorEntry::new(5));
        assert!(colors.contains_key(AccentColorId::new(5)));
        assert!(!colors.contains_key(AccentColorId::new(3)));
    }

    #[test]
    fn test_manager_display() {
        let manager = ThemeManager::new();
        let display = format!("{}", manager);
        assert!(display.contains("ThemeManager"));
        assert!(display.contains("0 themes"));
    }

    #[test]
    fn test_find_theme_by_emoji() {
        let mut manager = ThemeManager::new();
        let theme = EmojiChatTheme::new("🎨".to_string(), 1);
        manager.update_chat_themes(vec![theme], 123);
        assert!(manager.find_theme_by_emoji("🎨").is_some());
        assert!(manager.find_theme_by_emoji("❌").is_none());
    }
}
