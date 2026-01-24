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

//! TL type stubs for attach menu bot operations.
//!
//! This module provides stub implementations for TL types that are needed
//! for attach menu bot functionality. These are simplified versions that
//! will be replaced when the full TL layer is implemented.

use crate::types::AttachMenuBot;
use rustgram_file_id::FileId;
use rustgram_types::UserId;
use std::fmt;

/// Color for attach menu bot icon.
///
/// Stub for TL `attachMenuBotIconColor` type.
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachMenuBotIconColor {
    /// Color name (e.g., "light_icon", "light_text", "dark_icon", "dark_text").
    pub name: String,
    /// RGB color value.
    pub color: i32,
}

impl AttachMenuBotIconColor {
    /// Creates a new attach menu bot icon color.
    pub const fn new(name: String, color: i32) -> Self {
        Self { name, color }
    }
}

/// Icon for attach menu bot.
///
/// Stub for TL `attachMenuBotIcon` type.
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachMenuBotIcon {
    /// Icon name.
    pub name: String,
    /// Icon file ID.
    pub icon_file_id: FileId,
    /// Optional colors for the icon.
    pub colors: Vec<AttachMenuBotIconColor>,
}

impl AttachMenuBotIcon {
    /// Creates a new attach menu bot icon.
    pub fn new(name: String, icon_file_id: FileId, colors: Vec<AttachMenuBotIconColor>) -> Self {
        Self {
            name,
            icon_file_id,
            colors,
        }
    }
}

/// Peer types supported by attach menu bot.
///
/// Stub for TL `AttachMenuPeerType` enum.
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttachMenuPeerType {
    /// Saved Messages (SameBotPM).
    SameBotPm,
    /// Bot private chats.
    BotPm,
    /// User private chats.
    Pm,
    /// Group chats.
    Chat,
    /// Channel/broadcast chats.
    Broadcast,
}

impl AttachMenuPeerType {
    /// Creates an AttachMenuPeerType from a TL constructor ID.
    pub fn from_constructor_id(id: u32) -> Option<Self> {
        match id {
            0x77759166 => Some(Self::SameBotPm), // attachMenuPeerTypeSameBotPM
            0x1a821ec8 => Some(Self::BotPm),     // attachMenuPeerTypeBotPM
            0x6cbe2894 => Some(Self::Pm),        // attachMenuPeerTypePM
            0x1c9422c6 => Some(Self::Chat),      // attachMenuPeerTypeChat
            0x511c1502 => Some(Self::Broadcast), // attachMenuPeerTypeBroadcast
            _ => None,
        }
    }

    /// Returns the TL constructor ID for this peer type.
    pub const fn constructor_id(self) -> u32 {
        match self {
            Self::SameBotPm => 0x77759166,
            Self::BotPm => 0x1a821ec8,
            Self::Pm => 0x6cbe2894,
            Self::Chat => 0x1c9422c6,
            Self::Broadcast => 0x511c1502,
        }
    }
}

/// Attach menu bot from TL.
///
/// Stub for TL `attachMenuBot` type.
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlAttachMenuBot {
    /// Whether the bot is inactive (not added).
    pub inactive: bool,
    /// Whether bot requests write access.
    pub request_write_access: bool,
    /// Whether to show in attach menu.
    pub show_in_attach_menu: bool,
    /// Whether to show in side menu.
    pub show_in_side_menu: bool,
    /// Whether side menu disclaimer is needed.
    pub side_menu_disclaimer_needed: bool,
    /// Bot user ID.
    pub bot_id: i64,
    /// Short bot name.
    pub short_name: String,
    /// Supported peer types.
    pub peer_types: Vec<AttachMenuPeerType>,
    /// Bot icons.
    pub icons: Vec<AttachMenuBotIcon>,
}

impl TlAttachMenuBot {
    /// Creates a new TL attach menu bot.
    pub fn new(
        bot_id: i64,
        short_name: String,
        inactive: bool,
        request_write_access: bool,
        show_in_attach_menu: bool,
    ) -> Self {
        Self {
            inactive,
            request_write_access,
            show_in_attach_menu,
            show_in_side_menu: false,
            side_menu_disclaimer_needed: false,
            bot_id,
            short_name,
            peer_types: Vec::new(),
            icons: Vec::new(),
        }
    }

    /// Converts this TL bot to an AttachMenuBot.
    pub fn to_attach_menu_bot(&self) -> AttachMenuBot {
        let mut bot = AttachMenuBot::new(UserId(self.bot_id), &self.short_name, FileId::empty());

        bot.is_added = !self.inactive;
        bot.request_write_access = self.request_write_access;
        bot.show_in_attach_menu = self.show_in_attach_menu;
        bot.show_in_side_menu = self.show_in_side_menu;
        bot.side_menu_disclaimer_needed = self.side_menu_disclaimer_needed;

        // Process peer types
        for peer_type in &self.peer_types {
            match peer_type {
                AttachMenuPeerType::SameBotPm => bot.supports_self_dialog = true,
                AttachMenuPeerType::BotPm => bot.supports_bot_dialogs = true,
                AttachMenuPeerType::Pm => bot.supports_user_dialogs = true,
                AttachMenuPeerType::Chat => bot.supports_group_dialogs = true,
                AttachMenuPeerType::Broadcast => bot.supports_broadcast_dialogs = true,
            }
        }

        // Process icons
        for icon in &self.icons {
            match icon.name.as_str() {
                "default_static" => bot.default_icon_file_id = icon.icon_file_id,
                "ios_static" => bot.ios_static_icon_file_id = icon.icon_file_id,
                "ios_animated" => bot.ios_animated_icon_file_id = icon.icon_file_id,
                "android_animated" => {
                    bot.android_icon_file_id = icon.icon_file_id;
                    // Extract colors from android_animated icon
                    for color in &icon.colors {
                        match color.name.as_str() {
                            "light_icon" => bot.icon_color.light_color = color.color,
                            "light_text" => bot.name_color.light_color = color.color,
                            "dark_icon" => bot.icon_color.dark_color = color.color,
                            "dark_text" => bot.name_color.dark_color = color.color,
                            _ => {}
                        }
                    }
                }
                "macos_animated" => bot.macos_icon_file_id = icon.icon_file_id,
                "android_side_menu_static" => {
                    bot.android_side_menu_icon_file_id = icon.icon_file_id
                }
                "ios_side_menu_static" => bot.ios_side_menu_icon_file_id = icon.icon_file_id,
                "macos_side_menu_static" => bot.macos_side_menu_icon_file_id = icon.icon_file_id,
                "placeholder_static" => bot.placeholder_file_id = icon.icon_file_id,
                _ => {}
            }
        }

        bot
    }
}

/// Response for getAttachMenuBots query.
///
/// Stub for TL `AttachMenuBots` type.
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TlAttachMenuBots {
    /// Bots not modified (hash matches).
    NotModified {
        /// Current hash.
        hash: i64,
    },
    /// Full bot list.
    Bots {
        /// Hash for change detection.
        hash: i64,
        /// List of attach menu bots.
        bots: Vec<TlAttachMenuBot>,
    },
}

impl TlAttachMenuBots {
    /// Creates a "not modified" response.
    pub const fn not_modified(hash: i64) -> Self {
        Self::NotModified { hash }
    }

    /// Creates a "bots" response.
    pub fn bots(hash: i64, bots: Vec<TlAttachMenuBot>) -> Self {
        Self::Bots { hash, bots }
    }

    /// Returns the hash value.
    pub const fn hash(&self) -> i64 {
        match self {
            Self::NotModified { hash } => *hash,
            Self::Bots { hash, .. } => *hash,
        }
    }

    /// Returns `true` if this is a "not modified" response.
    pub const fn is_not_modified(&self) -> bool {
        matches!(self, Self::NotModified { .. })
    }
}

/// Response for getAttachMenuBot query.
///
/// Stub for TL `attachMenuBotsBot` type.
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlAttachMenuBotsBot {
    /// The attach menu bot.
    pub bot: TlAttachMenuBot,
}

impl TlAttachMenuBotsBot {
    /// Creates a new response.
    pub fn new(bot: TlAttachMenuBot) -> Self {
        Self { bot }
    }
}

/// User data stub.
///
/// Simplified user type for attach menu operations.
/// TODO: Replace with full User type when available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlUser {
    /// User ID.
    pub id: i64,
    /// Whether this is a bot.
    pub is_bot: bool,
    /// Whether bot can be added to attach menu.
    pub can_be_added_to_attach_menu: bool,
}

impl TlUser {
    /// Creates a new user.
    pub const fn new(id: i64, is_bot: bool, can_be_added_to_attach_menu: bool) -> Self {
        Self {
            id,
            is_bot,
            can_be_added_to_attach_menu,
        }
    }
}

impl fmt::Display for AttachMenuPeerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SameBotPm => write!(f, "SameBotPM"),
            Self::BotPm => write!(f, "BotPM"),
            Self::Pm => write!(f, "PM"),
            Self::Chat => write!(f, "Chat"),
            Self::Broadcast => write!(f, "Broadcast"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === AttachMenuPeerType tests ===

    #[test]
    fn test_peer_type_from_constructor_id() {
        assert_eq!(
            AttachMenuPeerType::from_constructor_id(0x77759166),
            Some(AttachMenuPeerType::SameBotPm)
        );
        assert_eq!(
            AttachMenuPeerType::from_constructor_id(0x1a821ec8),
            Some(AttachMenuPeerType::BotPm)
        );
        assert_eq!(
            AttachMenuPeerType::from_constructor_id(0x6cbe2894),
            Some(AttachMenuPeerType::Pm)
        );
        assert_eq!(
            AttachMenuPeerType::from_constructor_id(0x1c9422c6),
            Some(AttachMenuPeerType::Chat)
        );
        assert_eq!(
            AttachMenuPeerType::from_constructor_id(0x511c1502),
            Some(AttachMenuPeerType::Broadcast)
        );
        assert_eq!(AttachMenuPeerType::from_constructor_id(0xFFFFFFFF), None);
    }

    #[test]
    fn test_peer_type_constructor_id() {
        assert_eq!(AttachMenuPeerType::SameBotPm.constructor_id(), 0x77759166);
        assert_eq!(AttachMenuPeerType::BotPm.constructor_id(), 0x1a821ec8);
        assert_eq!(AttachMenuPeerType::Pm.constructor_id(), 0x6cbe2894);
        assert_eq!(AttachMenuPeerType::Chat.constructor_id(), 0x1c9422c6);
        assert_eq!(AttachMenuPeerType::Broadcast.constructor_id(), 0x511c1502);
    }

    #[test]
    fn test_peer_type_display() {
        assert_eq!(format!("{}", AttachMenuPeerType::SameBotPm), "SameBotPM");
        assert_eq!(format!("{}", AttachMenuPeerType::BotPm), "BotPM");
        assert_eq!(format!("{}", AttachMenuPeerType::Pm), "PM");
        assert_eq!(format!("{}", AttachMenuPeerType::Chat), "Chat");
        assert_eq!(format!("{}", AttachMenuPeerType::Broadcast), "Broadcast");
    }

    // === TlAttachMenuBot tests ===

    #[test]
    fn test_tl_bot_new() {
        let bot = TlAttachMenuBot::new(123, "TestBot".to_string(), false, true, true);
        assert_eq!(bot.bot_id, 123);
        assert_eq!(bot.short_name, "TestBot");
        assert!(!bot.inactive);
        assert!(bot.request_write_access);
        assert!(bot.show_in_attach_menu);
    }

    #[test]
    fn test_tl_bot_to_attach_menu_bot() {
        let mut tl_bot = TlAttachMenuBot::new(123, "TestBot".to_string(), false, true, true);
        tl_bot.peer_types = vec![
            AttachMenuPeerType::SameBotPm,
            AttachMenuPeerType::Pm,
            AttachMenuPeerType::Chat,
        ];
        tl_bot.icons = vec![AttachMenuBotIcon::new(
            "default_static".to_string(),
            FileId::new(1, 0),
            Vec::new(),
        )];

        let bot = tl_bot.to_attach_menu_bot();
        assert_eq!(bot.user_id, UserId(123));
        assert_eq!(bot.name, "TestBot");
        assert!(bot.is_added);
        assert!(bot.supports_self_dialog);
        assert!(bot.supports_user_dialogs);
        assert!(bot.supports_group_dialogs);
        assert!(!bot.supports_bot_dialogs);
        assert!(!bot.supports_broadcast_dialogs);
        assert_eq!(bot.default_icon_file_id, FileId::new(1, 0));
    }

    #[test]
    fn test_tl_bot_with_colors() {
        let mut tl_bot = TlAttachMenuBot::new(123, "TestBot".to_string(), false, false, true);
        tl_bot.icons = vec![AttachMenuBotIcon::new(
            "android_animated".to_string(),
            FileId::new(1, 0),
            vec![
                AttachMenuBotIconColor::new("light_icon".to_string(), 0x111111),
                AttachMenuBotIconColor::new("light_text".to_string(), 0x222222),
                AttachMenuBotIconColor::new("dark_icon".to_string(), 0x333333),
                AttachMenuBotIconColor::new("dark_text".to_string(), 0x444444),
            ],
        )];

        let bot = tl_bot.to_attach_menu_bot();
        assert_eq!(bot.android_icon_file_id, FileId::new(1, 0));
        assert_eq!(bot.icon_color.light_color, 0x111111);
        assert_eq!(bot.name_color.light_color, 0x222222);
        assert_eq!(bot.icon_color.dark_color, 0x333333);
        assert_eq!(bot.name_color.dark_color, 0x444444);
    }

    // === TlAttachMenuBots tests ===

    #[test]
    fn test_tl_bots_not_modified() {
        let bots = TlAttachMenuBots::not_modified(12345);
        assert!(bots.is_not_modified());
        assert_eq!(bots.hash(), 12345);
    }

    #[test]
    fn test_tl_bots_list() {
        let tl_bot = TlAttachMenuBot::new(123, "TestBot".to_string(), false, true, true);
        let bots = TlAttachMenuBots::bots(54321, vec![tl_bot]);
        assert!(!bots.is_not_modified());
        assert_eq!(bots.hash(), 54321);
    }

    // === TlUser tests ===

    #[test]
    fn test_tl_user_new() {
        let user = TlUser::new(123, true, true);
        assert_eq!(user.id, 123);
        assert!(user.is_bot);
        assert!(user.can_be_added_to_attach_menu);
    }

    // === AttachMenuBotIconColor tests ===

    #[test]
    fn test_icon_color_new() {
        let color = AttachMenuBotIconColor::new("light_icon".to_string(), 0x123456);
        assert_eq!(color.name, "light_icon");
        assert_eq!(color.color, 0x123456);
    }

    // === AttachMenuBotIcon tests ===

    #[test]
    fn test_icon_new() {
        let icon = AttachMenuBotIcon::new("default_static".to_string(), FileId::new(1, 0), vec![]);
        assert_eq!(icon.name, "default_static");
        assert_eq!(icon.icon_file_id, FileId::new(1, 0));
        assert!(icon.colors.is_empty());
    }

    // === Integration tests ===

    #[test]
    fn test_full_bot_conversion() {
        let mut tl_bot = TlAttachMenuBot::new(123, "FullBot".to_string(), false, true, true);
        tl_bot.peer_types = vec![
            AttachMenuPeerType::SameBotPm,
            AttachMenuPeerType::BotPm,
            AttachMenuPeerType::Pm,
            AttachMenuPeerType::Chat,
            AttachMenuPeerType::Broadcast,
        ];
        tl_bot.icons = vec![
            AttachMenuBotIcon::new("default_static".to_string(), FileId::new(1, 0), vec![]),
            AttachMenuBotIcon::new("ios_static".to_string(), FileId::new(2, 0), vec![]),
            AttachMenuBotIcon::new("ios_animated".to_string(), FileId::new(3, 0), vec![]),
            AttachMenuBotIcon::new(
                "android_animated".to_string(),
                FileId::new(4, 0),
                vec![
                    AttachMenuBotIconColor::new("light_icon".to_string(), 0x111111),
                    AttachMenuBotIconColor::new("light_text".to_string(), 0x222222),
                    AttachMenuBotIconColor::new("dark_icon".to_string(), 0x333333),
                    AttachMenuBotIconColor::new("dark_text".to_string(), 0x444444),
                ],
            ),
            AttachMenuBotIcon::new("macos_animated".to_string(), FileId::new(5, 0), vec![]),
            AttachMenuBotIcon::new(
                "android_side_menu_static".to_string(),
                FileId::new(6, 0),
                vec![],
            ),
            AttachMenuBotIcon::new(
                "ios_side_menu_static".to_string(),
                FileId::new(7, 0),
                vec![],
            ),
            AttachMenuBotIcon::new(
                "macos_side_menu_static".to_string(),
                FileId::new(8, 0),
                vec![],
            ),
            AttachMenuBotIcon::new("placeholder_static".to_string(), FileId::new(9, 0), vec![]),
        ];

        let bot = tl_bot.to_attach_menu_bot();
        assert!(bot.is_valid());
        assert!(bot.supports_self_dialog);
        assert!(bot.supports_bot_dialogs);
        assert!(bot.supports_user_dialogs);
        assert!(bot.supports_group_dialogs);
        assert!(bot.supports_broadcast_dialogs);
        assert_eq!(bot.get_valid_icons().len(), 9);
        assert!(bot.icon_color.is_valid());
        assert!(bot.name_color.is_valid());
    }
}
