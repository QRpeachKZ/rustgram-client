// Copyright 2025 rustgram-client contributors
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

//! Type definitions for attach menu bots.

use rustgram_file_id::FileId;
use rustgram_types::UserId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Current cache format version for AttachMenuBot.
pub const CACHE_VERSION: u32 = 3;

/// Color pair for attach menu bot theming (light/dark modes).
///
/// Corresponds to TDLib `AttachMenuBotColor` struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachMenuBotColor {
    /// RGB color value for light theme (-1 if not set).
    pub light_color: i32,
    /// RGB color value for dark theme (-1 if not set).
    pub dark_color: i32,
}

impl Default for AttachMenuBotColor {
    fn default() -> Self {
        Self {
            light_color: -1,
            dark_color: -1,
        }
    }
}

impl AttachMenuBotColor {
    /// Creates a new attach menu bot color.
    ///
    /// # Arguments
    ///
    /// * `light_color` - RGB color value for light theme (-1 if not set)
    /// * `dark_color` - RGB color value for dark theme (-1 if not set)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuBotColor;
    ///
    /// let color = AttachMenuBotColor::new(0x123456, 0x654321);
    /// assert_eq!(color.light_color, 0x123456);
    /// assert_eq!(color.dark_color, 0x654321);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(light_color: i32, dark_color: i32) -> Self {
        Self {
            light_color,
            dark_color,
        }
    }

    /// Returns `true` if this color is empty (both colors are -1).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuBotColor;
    ///
    /// let empty = AttachMenuBotColor::new(-1, -1);
    /// assert!(empty.is_empty());
    ///
    /// let color = AttachMenuBotColor::new(0x123456, 0x654321);
    /// assert!(!color.is_empty());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.light_color == -1 && self.dark_color == -1
    }

    /// Checks if this color is valid (both colors are set).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuBotColor;
    ///
    /// let color = AttachMenuBotColor::new(0x123456, 0x654321);
    /// assert!(color.is_valid());
    ///
    /// let empty = AttachMenuBotColor::new(-1, -1);
    /// assert!(!empty.is_valid());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.light_color != -1 && self.dark_color != -1
    }
}

/// Complete data for an attachment menu bot.
///
/// Corresponds to TDLib `AttachMenuBot` struct.
///
/// This contains all platform-specific icons, colors, and capability flags
/// for a Telegram bot that can be invoked from the attachment menu.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachMenuBot {
    // Basic bot info
    /// Whether bot is added to user's attachment menu.
    pub is_added: bool,
    /// Bot user identifier.
    pub user_id: UserId,
    /// Short name displayed in attachment menu.
    pub name: String,

    // Capability flags
    /// Bot supports being invoked in Saved Messages.
    pub supports_self_dialog: bool,
    /// Bot supports private chats with users.
    pub supports_user_dialogs: bool,
    /// Bot supports private chats with other bots.
    pub supports_bot_dialogs: bool,
    /// Bot supports group chats.
    pub supports_group_dialogs: bool,
    /// Bot supports channels/broadcasts.
    pub supports_broadcast_dialogs: bool,

    // Display flags
    /// Bot requests permission to send messages.
    pub request_write_access: bool,
    /// Bot should be shown in attachment menu.
    pub show_in_attach_menu: bool,
    /// Bot should be shown in side menu.
    pub show_in_side_menu: bool,
    /// Side menu requires disclaimer display.
    pub side_menu_disclaimer_needed: bool,

    // Colors
    /// Color for bot name display (light/dark).
    pub name_color: AttachMenuBotColor,
    /// Color for bot icon display (light/dark).
    pub icon_color: AttachMenuBotColor,

    // Icons (platform-specific)
    /// Default icon (fallback for all platforms).
    pub default_icon_file_id: FileId,
    /// iOS static icon.
    pub ios_static_icon_file_id: FileId,
    /// iOS animated icon.
    pub ios_animated_icon_file_id: FileId,
    /// Android animated icon.
    pub android_icon_file_id: FileId,
    /// macOS animated icon.
    pub macos_icon_file_id: FileId,
    /// Android side menu icon.
    pub android_side_menu_icon_file_id: FileId,
    /// iOS side menu icon.
    pub ios_side_menu_icon_file_id: FileId,
    /// macOS side menu icon.
    pub macos_side_menu_icon_file_id: FileId,
    /// Placeholder icon while loading.
    pub placeholder_file_id: FileId,

    /// Cache format version (must equal CACHE_VERSION = 3).
    pub cache_version: u32,
}

impl AttachMenuBot {
    /// Creates a new attach menu bot with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuBot;
    /// use rustgram_types::UserId;
    /// use rustgram_file_id::FileId;
    ///
    /// let bot = AttachMenuBot::new(
    ///     UserId(123),
    ///     "MyBot",
    ///     FileId::new(1, 0),
    /// );
    /// assert_eq!(bot.user_id, UserId(123));
    /// assert_eq!(bot.name, "MyBot");
    /// assert_eq!(bot.cache_version, 3);
    /// ```
    #[must_use]
    pub fn new(user_id: UserId, name: &str, default_icon: FileId) -> Self {
        Self {
            is_added: false,
            user_id,
            name: name.to_string(),
            supports_self_dialog: false,
            supports_user_dialogs: false,
            supports_bot_dialogs: false,
            supports_group_dialogs: false,
            supports_broadcast_dialogs: false,
            request_write_access: false,
            show_in_attach_menu: false,
            show_in_side_menu: false,
            side_menu_disclaimer_needed: false,
            name_color: AttachMenuBotColor::default(),
            icon_color: AttachMenuBotColor::default(),
            default_icon_file_id: default_icon,
            ios_static_icon_file_id: FileId::empty(),
            ios_animated_icon_file_id: FileId::empty(),
            android_icon_file_id: FileId::empty(),
            macos_icon_file_id: FileId::empty(),
            android_side_menu_icon_file_id: FileId::empty(),
            ios_side_menu_icon_file_id: FileId::empty(),
            macos_side_menu_icon_file_id: FileId::empty(),
            placeholder_file_id: FileId::empty(),
            cache_version: CACHE_VERSION,
        }
    }

    /// Returns `true` if the bot is empty (invalid user ID).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuBot;
    /// use rustgram_types::UserId;
    /// use rustgram_file_id::FileId;
    ///
    /// let bot = AttachMenuBot::new(UserId(0), "bot", FileId::empty());
    /// assert!(bot.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.user_id.is_valid()
    }

    /// Returns `true` if the bot is valid.
    ///
    /// A valid bot must have:
    /// - Valid user ID
    /// - Valid default icon
    /// - Correct cache version
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuBot;
    /// use rustgram_types::UserId;
    /// use rustgram_file_id::FileId;
    ///
    /// let bot = AttachMenuBot::new(UserId(123), "bot", FileId::new(1, 0));
    /// assert!(bot.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.user_id.is_valid()
            && self.default_icon_file_id.is_valid()
            && self.cache_version == CACHE_VERSION
    }

    /// Gets all icon file IDs for this bot.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuBot;
    /// use rustgram_types::UserId;
    /// use rustgram_file_id::FileId;
    ///
    /// let bot = AttachMenuBot::new(UserId(123), "bot", FileId::new(1, 0));
    /// let icons = bot.get_all_icons();
    /// assert_eq!(icons.len(), 9);
    /// ```
    #[must_use]
    pub fn get_all_icons(&self) -> Vec<FileId> {
        vec![
            self.default_icon_file_id,
            self.ios_static_icon_file_id,
            self.ios_animated_icon_file_id,
            self.android_icon_file_id,
            self.macos_icon_file_id,
            self.android_side_menu_icon_file_id,
            self.ios_side_menu_icon_file_id,
            self.macos_side_menu_icon_file_id,
            self.placeholder_file_id,
        ]
    }

    /// Gets all valid icon file IDs for this bot.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_attach_menu_manager::AttachMenuBot;
    /// use rustgram_types::UserId;
    /// use rustgram_file_id::FileId;
    ///
    /// let bot = AttachMenuBot::new(UserId(123), "bot", FileId::new(1, 0));
    /// let icons = bot.get_valid_icons();
    /// assert_eq!(icons.len(), 1); // Only default icon is valid
    /// ```
    #[must_use]
    pub fn get_valid_icons(&self) -> Vec<FileId> {
        self.get_all_icons()
            .into_iter()
            .filter(|id| id.is_valid())
            .collect()
    }
}

impl Default for AttachMenuBot {
    fn default() -> Self {
        Self {
            is_added: false,
            user_id: UserId::default(),
            name: String::new(),
            supports_self_dialog: false,
            supports_user_dialogs: false,
            supports_bot_dialogs: false,
            supports_group_dialogs: false,
            supports_broadcast_dialogs: false,
            request_write_access: false,
            show_in_attach_menu: false,
            show_in_side_menu: false,
            side_menu_disclaimer_needed: false,
            name_color: AttachMenuBotColor::default(),
            icon_color: AttachMenuBotColor::default(),
            default_icon_file_id: FileId::empty(),
            ios_static_icon_file_id: FileId::empty(),
            ios_animated_icon_file_id: FileId::empty(),
            android_icon_file_id: FileId::empty(),
            macos_icon_file_id: FileId::empty(),
            android_side_menu_icon_file_id: FileId::empty(),
            ios_side_menu_icon_file_id: FileId::empty(),
            macos_side_menu_icon_file_id: FileId::empty(),
            placeholder_file_id: FileId::empty(),
            cache_version: 0,
        }
    }
}

impl fmt::Display for AttachMenuBot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AttachMenuBot({}, {})", self.user_id.0, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // === AttachMenuBotColor tests ===

    #[test]
    fn test_color_new() {
        let color = AttachMenuBotColor::new(0x123456, 0x654321);
        assert_eq!(color.light_color, 0x123456);
        assert_eq!(color.dark_color, 0x654321);
    }

    #[test]
    fn test_color_default() {
        let color = AttachMenuBotColor::default();
        assert_eq!(color.light_color, -1);
        assert_eq!(color.dark_color, -1);
    }

    #[rstest]
    #[case(-1, -1, true)]
    #[case(0x123456, 0x654321, false)]
    #[case(-1, 0x654321, false)]
    #[case(0x123456, -1, false)]
    fn test_color_is_empty(#[case] light: i32, #[case] dark: i32, #[case] expected: bool) {
        let color = AttachMenuBotColor::new(light, dark);
        assert_eq!(color.is_empty(), expected);
    }

    #[rstest]
    #[case(-1, -1, false)]
    #[case(0x123456, 0x654321, true)]
    #[case(-1, 0x654321, false)]
    #[case(0x123456, -1, false)]
    fn test_color_is_valid(#[case] light: i32, #[case] dark: i32, #[case] expected: bool) {
        let color = AttachMenuBotColor::new(light, dark);
        assert_eq!(color.is_valid(), expected);
    }

    #[test]
    fn test_color_equality() {
        let a = AttachMenuBotColor::new(0x123456, 0x654321);
        let b = AttachMenuBotColor::new(0x123456, 0x654321);
        let c = AttachMenuBotColor::new(0x123456, 0x111111);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_color_clone() {
        let color = AttachMenuBotColor::new(0x123456, 0x654321);
        let cloned = color.clone();
        assert_eq!(color, cloned);
    }

    // === AttachMenuBot tests ===

    #[test]
    fn test_bot_new() {
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        assert_eq!(bot.user_id, UserId(123));
        assert_eq!(bot.name, "TestBot");
        assert_eq!(bot.default_icon_file_id, FileId::new(1, 0));
        assert_eq!(bot.cache_version, CACHE_VERSION);
        assert!(!bot.is_added);
        assert!(!bot.supports_self_dialog);
    }

    #[test]
    fn test_bot_default() {
        let bot = AttachMenuBot::default();
        assert_eq!(bot.user_id, UserId(0));
        assert!(bot.name.is_empty());
        assert!(!bot.default_icon_file_id.is_valid());
        assert_eq!(bot.cache_version, 0);
    }

    #[test]
    fn test_bot_is_empty() {
        let mut bot = AttachMenuBot::default();
        assert!(bot.is_empty());

        bot.user_id = UserId(123);
        assert!(!bot.is_empty());
    }

    #[test]
    fn test_bot_is_valid() {
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        assert!(bot.is_valid());

        let mut invalid = bot.clone();
        invalid.user_id = UserId(0);
        assert!(!invalid.is_valid());

        let mut invalid2 = bot.clone();
        invalid2.default_icon_file_id = FileId::empty();
        assert!(!invalid2.is_valid());

        let mut invalid3 = bot.clone();
        invalid3.cache_version = 2;
        assert!(!invalid3.is_valid());
    }

    #[test]
    fn test_bot_get_all_icons() {
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        let icons = bot.get_all_icons();
        assert_eq!(icons.len(), 9);
        assert_eq!(icons[0], FileId::new(1, 0));
        assert_eq!(icons[1], FileId::empty());
    }

    #[test]
    fn test_bot_get_valid_icons() {
        let mut bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        bot.ios_static_icon_file_id = FileId::new(2, 0);
        bot.android_icon_file_id = FileId::new(3, 0);

        let icons = bot.get_valid_icons();
        assert_eq!(icons.len(), 3);
        assert!(icons.contains(&FileId::new(1, 0)));
        assert!(icons.contains(&FileId::new(2, 0)));
        assert!(icons.contains(&FileId::new(3, 0)));
    }

    #[test]
    fn test_bot_display() {
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        assert_eq!(format!("{bot}"), "AttachMenuBot(123, TestBot)");
    }

    #[test]
    fn test_bot_equality() {
        let bot1 = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        let bot2 = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        let bot3 = AttachMenuBot::new(UserId(124), "TestBot", FileId::new(1, 0));

        assert_eq!(bot1, bot2);
        assert_ne!(bot1, bot3);
    }

    #[test]
    fn test_bot_clone() {
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        let cloned = bot.clone();
        assert_eq!(bot, cloned);
    }

    #[test]
    fn test_bot_support_flags() {
        let mut bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));

        bot.supports_self_dialog = true;
        bot.supports_user_dialogs = true;
        bot.supports_bot_dialogs = true;
        bot.supports_group_dialogs = true;
        bot.supports_broadcast_dialogs = true;

        assert!(bot.supports_self_dialog);
        assert!(bot.supports_user_dialogs);
        assert!(bot.supports_bot_dialogs);
        assert!(bot.supports_group_dialogs);
        assert!(bot.supports_broadcast_dialogs);
    }

    #[test]
    fn test_bot_display_flags() {
        let mut bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));

        bot.request_write_access = true;
        bot.show_in_attach_menu = true;
        bot.show_in_side_menu = true;
        bot.side_menu_disclaimer_needed = true;

        assert!(bot.request_write_access);
        assert!(bot.show_in_attach_menu);
        assert!(bot.show_in_side_menu);
        assert!(bot.side_menu_disclaimer_needed);
    }

    #[test]
    fn test_bot_colors() {
        let mut bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));

        bot.name_color = AttachMenuBotColor::new(0x111111, 0x222222);
        bot.icon_color = AttachMenuBotColor::new(0x333333, 0x444444);

        assert_eq!(bot.name_color.light_color, 0x111111);
        assert_eq!(bot.name_color.dark_color, 0x222222);
        assert_eq!(bot.icon_color.light_color, 0x333333);
        assert_eq!(bot.icon_color.dark_color, 0x444444);
    }

    #[test]
    fn test_bot_all_nine_icons() {
        let mut bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        bot.default_icon_file_id = FileId::new(1, 0);
        bot.ios_static_icon_file_id = FileId::new(2, 0);
        bot.ios_animated_icon_file_id = FileId::new(3, 0);
        bot.android_icon_file_id = FileId::new(4, 0);
        bot.macos_icon_file_id = FileId::new(5, 0);
        bot.android_side_menu_icon_file_id = FileId::new(6, 0);
        bot.ios_side_menu_icon_file_id = FileId::new(7, 0);
        bot.macos_side_menu_icon_file_id = FileId::new(8, 0);
        bot.placeholder_file_id = FileId::new(9, 0);

        assert!(bot.get_valid_icons().len() == 9);
    }

    // === Serialization tests ===

    #[test]
    fn test_color_serialize() {
        let color = AttachMenuBotColor::new(0x123456, 0x654321);
        let json = serde_json::to_string(&color).unwrap();
        // 0x123456 = 1193046 in decimal
        // 0x654321 = 6636321 in decimal
        assert!(json.contains("1193046"));
        assert!(json.contains("6636321"));
    }

    #[test]
    fn test_color_deserialize() {
        let json = r#"{"light_color":305419896,"dark_color":-123}"#;
        let color: AttachMenuBotColor = serde_json::from_str(json).unwrap();
        assert_eq!(color.light_color, 305419896);
        assert_eq!(color.dark_color, -123);
    }

    #[test]
    fn test_bot_serialize() {
        let bot = AttachMenuBot::new(UserId(123), "TestBot", FileId::new(1, 0));
        let json = serde_json::to_string(&bot).unwrap();
        assert!(json.contains("123"));
        assert!(json.contains("TestBot"));
    }

    #[test]
    fn test_bot_deserialize() {
        let json = r#"{
            "is_added":false,
            "user_id":123,
            "name":"TestBot",
            "supports_self_dialog":false,
            "supports_user_dialogs":false,
            "supports_bot_dialogs":false,
            "supports_group_dialogs":false,
            "supports_broadcast_dialogs":false,
            "request_write_access":false,
            "show_in_attach_menu":false,
            "show_in_side_menu":false,
            "side_menu_disclaimer_needed":false,
            "name_color":{"light_color":-1,"dark_color":-1},
            "icon_color":{"light_color":-1,"dark_color":-1},
            "default_icon_file_id":{"id":1,"remote_id":0},
            "ios_static_icon_file_id":{"id":0,"remote_id":0},
            "ios_animated_icon_file_id":{"id":0,"remote_id":0},
            "android_icon_file_id":{"id":0,"remote_id":0},
            "macos_icon_file_id":{"id":0,"remote_id":0},
            "android_side_menu_icon_file_id":{"id":0,"remote_id":0},
            "ios_side_menu_icon_file_id":{"id":0,"remote_id":0},
            "macos_side_menu_icon_file_id":{"id":0,"remote_id":0},
            "placeholder_file_id":{"id":0,"remote_id":0},
            "cache_version":3
        }"#;
        let bot: AttachMenuBot = serde_json::from_str(json).unwrap();
        assert_eq!(bot.user_id, UserId(123));
        assert_eq!(bot.name, "TestBot");
        assert_eq!(bot.cache_version, 3);
    }
}
