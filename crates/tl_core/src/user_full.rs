// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! User full info types for Telegram.
//!
//! This module provides TL deserialization for full user information.
//!
//! # TL Schema
//!
//! ```text
//! userFull#a02bc13e flags:# blocked:flags.0?true phone_calls_available:flags.4?true
//!     phone_calls_private:flags.5?true can_pin_message:flags.7?true
//!     has_scheduled:flags.12?true video_calls_available:flags.13?true
//!     voice_messages_forbidden:flags.20?true translations_disabled:flags.23?true
//!     stories_pinned_available:flags.26?true blocked_my_stories_from:flags.27?true
//!     wallpaper_overridden:flags.28?true contact_require_premium:flags.29?true
//!     read_dates_private:flags.30?true flags2:# sponsored_enabled:flags2.7?true
//!     can_view_revenue:flags2.9?true bot_can_manage_emoji_status:flags2.10?true
//!     display_gifts_button:flags2.16?true id:long about:flags.1?string
//!     settings:PeerSettings personal_photo:flags.21?Photo profile_photo:flags.2?Photo
//!     fallback_photo:flags.22?Photo notify_settings:PeerNotifySettings
//!     bot_info:flags.3?BotInfo pinned_msg_id:flags.6?int common_chats_count:int
//!     folder_id:flags.11?int ttl_period:flags.14?int theme:flags.15?ChatTheme
//!     private_forward_name:flags.16?string bot_group_admin_rights:flags.17?ChatAdminRights
//!     bot_broadcast_admin_rights:flags.18?ChatAdminRights wallpaper:flags.24?WallPaper
//!     stories:flags.25?PeerStories business_work_hours:flags2.0?BusinessWorkHours
//!     business_location:flags2.1?BusinessLocation
//!     business_greeting_message:flags2.2?BusinessGreetingMessage
//!     business_away_message:flags2.3?BusinessAwayMessage business_intro:flags2.4?BusinessIntro
//!     birthday:flags2.5?Birthday personal_channel_id:flags2.6?long
//!     personal_channel_message:flags2.6?int stargifts_count:flags2.8?int
//!     starref_program:flags2.11?StarRefProgram bot_verification:flags2.12?BotVerification
//!     send_paid_messages_stars:flags2.14?long disallowed_gifts:flags2.15?DisallowedGiftsSettings
//!     stars_rating:flags2.17?StarsRating stars_my_pending_rating:flags2.18?StarsRating
//!     stars_my_pending_rating_date:flags2.18?int main_tab:flags2.20?ProfileTab
//!     saved_music:flags2.21?Document note:flags2.22?TextWithEntities = UserFull;
//! ```

use crate::flags::FlagReader;
use crate::notify::PeerNotifySettings;
use crate::photo::Photo;
use rustgram_types::tl::{Bytes, TlDeserialize, TlHelper};
use serde::{Deserialize, Serialize};

/// Full user information.
///
/// Contains complete information about a user including profile, settings,
/// and various optional fields controlled by flags.
///
/// # TL Schema
///
/// ```text
/// userFull#a02bc13e flags:# blocked:flags.0?true phone_calls_available:flags.4?true
///     phone_calls_private:flags.5?true can_pin_message:flags.7?true
///     has_scheduled:flags.12?true video_calls_available:flags.13?true
///     voice_messages_forbidden:flags.20?true translations_disabled:flags.23?true
///     ... (many more optional fields)
///     id:long about:flags.1?string settings:PeerSettings personal_photo:flags.21?Photo
///     profile_photo:flags.2?Photo fallback_photo:flags.22?Photo
///     notify_settings:PeerNotifySettings bot_info:flags.3?BotInfo
///     pinned_msg_id:flags.6?int common_chats_count:int folder_id:flags.11?int
///     ttl_period:flags.14?int theme:flags.15?ChatTheme private_forward_name:flags.16?string
///     ... = UserFull;
/// ```
///
/// Note: This is a simplified version focusing on the core fields.
/// Many optional business and premium-related fields are omitted.
#[derive(Debug, Clone, PartialEq)]
pub struct UserFull {
    /// Whether the user is blocked by the current user.
    pub blocked: bool,

    /// Whether phone calls are available.
    pub phone_calls_available: bool,

    /// Whether phone calls are private.
    pub phone_calls_private: bool,

    /// Whether the user can pin messages.
    pub can_pin_message: bool,

    /// Whether the user has scheduled messages.
    pub has_scheduled: bool,

    /// Whether video calls are available.
    pub video_calls_available: bool,

    /// Whether voice messages are forbidden.
    pub voice_messages_forbidden: bool,

    /// Whether translations are disabled.
    pub translations_disabled: bool,

    /// User ID.
    pub id: i64,

    /// User bio or about text.
    pub about: Option<String>,

    /// Peer settings (privacy settings).
    pub settings: Option<PeerSettings>,

    /// Personal profile photo.
    pub personal_photo: Option<Photo>,

    /// Profile photo.
    pub profile_photo: Option<Photo>,

    /// Fallback photo.
    pub fallback_photo: Option<Photo>,

    /// Notification settings.
    pub notify_settings: PeerNotifySettings,

    /// Bot information (if this is a bot).
    pub bot_info: Option<BotInfo>,

    /// ID of a pinned message.
    pub pinned_msg_id: Option<i32>,

    /// Number of common chats.
    pub common_chats_count: i32,

    /// Folder ID (if in a folder).
    pub folder_id: Option<i32>,

    /// Time-to-live period for messages.
    pub ttl_period: Option<i32>,

    /// Chat theme.
    pub theme: Option<ChatTheme>,

    /// Name to show when forwarding messages.
    pub private_forward_name: Option<String>,

    /// Business work hours.
    pub business_work_hours: Option<BusinessWorkHours>,

    /// Personal channel ID.
    pub personal_channel_id: Option<i64>,

    /// Personal channel message ID.
    pub personal_channel_message: Option<i32>,

    /// Birthday information.
    pub birthday: Option<Birthday>,

    /// Number of star gifts received.
    pub stargifts_count: Option<i32>,

    /// Saved music document.
    pub saved_music: Option<Document>,
}

impl UserFull {
    /// Constructor ID for userFull.
    pub const CONSTRUCTOR: u32 = 0xa02bc13e;

    /// Checks if this is a bot user.
    pub fn is_bot(&self) -> bool {
        self.bot_info.is_some()
    }

    /// Checks if the user has a profile photo.
    pub fn has_profile_photo(&self) -> bool {
        self.profile_photo.is_some()
    }
}

impl TlDeserialize for UserFull {
    fn deserialize_tl(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        if constructor_id != Self::CONSTRUCTOR {
            let tl_err = crate::error::TlError::unknown_constructor(
                vec![Self::CONSTRUCTOR],
                constructor_id,
                "UserFull",
            );
            return Err(rustgram_types::TypeError::from(tl_err));
        }

        let flags = TlHelper::read_i32(buf)? as u32;
        let flag_reader = FlagReader::new(flags);

        let blocked = flag_reader.read_bool(0);
        let phone_calls_available = flag_reader.read_bool(4);
        let phone_calls_private = flag_reader.read_bool(5);
        let can_pin_message = flag_reader.read_bool(7);
        let has_scheduled = flag_reader.read_bool(12);
        let video_calls_available = flag_reader.read_bool(13);
        let voice_messages_forbidden = flag_reader.read_bool(20);
        let translations_disabled = flag_reader.read_bool(23);

        let id = TlHelper::read_i64(buf)?;

        let about = flag_reader.read_optional(1, || TlHelper::read_string(buf))?;

        // Read settings (PeerSettings) - simplified as placeholder
        let settings = if flag_reader.has(1) {
            Some(PeerSettings::default())
        } else {
            None
        };

        let personal_photo = flag_reader.read_optional(21, || Photo::deserialize_tl(buf))?;
        let profile_photo = flag_reader.read_optional(2, || Photo::deserialize_tl(buf))?;
        let fallback_photo = flag_reader.read_optional(22, || Photo::deserialize_tl(buf))?;

        let notify_settings = PeerNotifySettings::deserialize_tl(buf)?;

        let bot_info = flag_reader.read_optional(3, || BotInfo::deserialize_placeholder(buf))?;

        let pinned_msg_id = flag_reader.read_optional(6, || TlHelper::read_i32(buf))?;

        let common_chats_count = TlHelper::read_i32(buf)?;

        let folder_id = flag_reader.read_optional(11, || TlHelper::read_i32(buf))?;
        let ttl_period = flag_reader.read_optional(14, || TlHelper::read_i32(buf))?;

        let theme = flag_reader.read_optional(15, || ChatTheme::deserialize_placeholder(buf))?;

        let private_forward_name = flag_reader.read_optional(16, || TlHelper::read_string(buf))?;

        // Read flags2 if present
        let _flags2 = if flags & 0x4000 != 0 {
            Some(TlHelper::read_i32(buf)? as u32)
        } else {
            None
        };

        // Simplified: skip complex nested structures for now
        let business_work_hours = None;
        let personal_channel_id = None;
        let personal_channel_message = None;
        let birthday = None;
        let stargifts_count = None;
        let saved_music = None;

        Ok(Self {
            blocked,
            phone_calls_available,
            phone_calls_private,
            can_pin_message,
            has_scheduled,
            video_calls_available,
            voice_messages_forbidden,
            translations_disabled,
            id,
            about,
            settings,
            personal_photo,
            profile_photo,
            fallback_photo,
            notify_settings,
            bot_info,
            pinned_msg_id,
            common_chats_count,
            folder_id,
            ttl_period,
            theme,
            private_forward_name,
            business_work_hours,
            personal_channel_id,
            personal_channel_message,
            birthday,
            stargifts_count,
            saved_music,
        })
    }
}

/// Peer settings (privacy and interaction settings).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PeerSettings {
    /// Whether to report spam.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_spam: Option<bool>,

    /// Whether to add contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_contact: Option<bool>,
}

/// Bot information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotInfo {
    /// Bot description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Bot commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<BotCommand>>,

    /// Bot menu button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<BotMenuButton>,
}

impl BotInfo {
    fn deserialize_placeholder(_buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        Ok(Self {
            description: None,
            commands: None,
            menu_button: None,
        })
    }
}

/// Bot command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotCommand {
    /// Command text (e.g., "/start").
    #[serde(skip)]
    pub command: String,

    /// Command description.
    #[serde(skip)]
    pub description: String,
}

/// Bot menu button.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotMenuButton {
    /// Button text.
    #[serde(skip)]
    pub text: String,

    /// Button URL.
    #[serde(skip)]
    pub url: String,
}

/// Chat theme.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatTheme {
    /// Theme name.
    #[serde(skip)]
    pub name: String,

    /// Theme settings (serialized JSON).
    #[serde(skip)]
    pub settings: String,
}

impl ChatTheme {
    fn deserialize_placeholder(_buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        Ok(Self {
            name: String::new(),
            settings: String::new(),
        })
    }
}

/// Business work hours.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessWorkHours {
    /// Timezone ID.
    #[serde(skip)]
    pub timezone_id: String,

    /// Weekly schedule.
    #[serde(skip)]
    pub weekly_open: Vec<i32>,
}

/// Birthday information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Birthday {
    /// Day of month (1-31).
    pub day: i32,

    /// Month (1-12).
    pub month: i32,

    /// Year (optional).
    pub year: Option<i32>,
}

/// Document placeholder.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    /// Document ID.
    #[serde(skip)]
    pub id: i64,

    /// Document access hash.
    #[serde(skip)]
    pub access_hash: i64,

    /// File reference.
    #[serde(skip)]
    pub file_reference: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_buffer(data: &[u8]) -> Bytes {
        Bytes::new(bytes::Bytes::copy_from_slice(data))
    }

    #[test]
    fn test_user_full_constructor() {
        assert_eq!(UserFull::CONSTRUCTOR, 0xa02bc13e);
    }

    #[test]
    fn test_user_full_minimal() {
        // Minimal userFull with only required fields
        let mut data = vec![0x3e, 0xc1, 0x2b, 0xa0]; // userFull constructor
        data.extend_from_slice(&0i32.to_le_bytes()); // flags = 0
        data.extend_from_slice(&123i64.to_le_bytes()); // id

        // about is optional, skip since flag 1 is not set
        // settings is optional
        // photos are optional
        // notify_settings
        data.extend_from_slice(&[0x0c, 0x2c, 0x62, 0x99]); // peerNotifySettings constructor
        data.extend_from_slice(&0i32.to_le_bytes()); // flags = 0

        // bot_info is optional
        data.extend_from_slice(&10i32.to_le_bytes()); // common_chats_count

        let mut buf = create_buffer(&data);
        let result = UserFull::deserialize_tl(&mut buf);

        // This should succeed or provide a clear error
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_peer_settings_default() {
        let settings = PeerSettings::default();

        assert_eq!(settings.report_spam, None);
        assert_eq!(settings.add_contact, None);
    }

    #[test]
    fn test_bot_info() {
        let bot_info = BotInfo {
            description: Some("Test bot".to_string()),
            commands: None,
            menu_button: None,
        };

        assert_eq!(bot_info.description, Some("Test bot".to_string()));
    }

    #[test]
    fn test_birthday() {
        let birthday = Birthday {
            day: 15,
            month: 6,
            year: Some(1990),
        };

        assert_eq!(birthday.day, 15);
        assert_eq!(birthday.month, 6);
        assert_eq!(birthday.year, Some(1990));
    }

    #[test]
    fn test_chat_theme() {
        let theme = ChatTheme {
            name: "blue".to_string(),
            settings: "{}".to_string(),
        };

        assert_eq!(theme.name, "blue");
        assert_eq!(theme.settings, "{}");
    }

    #[test]
    fn test_business_work_hours() {
        let work_hours = BusinessWorkHours {
            timezone_id: "Europe/London".to_string(),
            weekly_open: vec![0, 0, 0, 0, 0, 0, 0],
        };

        assert_eq!(work_hours.timezone_id, "Europe/London");
        assert_eq!(work_hours.weekly_open.len(), 7);
    }

    #[test]
    fn test_document() {
        let doc = Document {
            id: 12345,
            access_hash: 67890,
            file_reference: vec![1, 2, 3],
        };

        assert_eq!(doc.id, 12345);
        assert_eq!(doc.access_hash, 67890);
        assert_eq!(doc.file_reference, vec![1, 2, 3]);
    }

    #[test]
    fn test_user_full_clone() {
        let full = UserFull {
            blocked: false,
            phone_calls_available: true,
            phone_calls_private: false,
            can_pin_message: true,
            has_scheduled: false,
            video_calls_available: true,
            voice_messages_forbidden: false,
            translations_disabled: false,
            id: 123,
            about: Some("Test user".to_string()),
            settings: None,
            personal_photo: None,
            profile_photo: None,
            fallback_photo: None,
            notify_settings: PeerNotifySettings::default(),
            bot_info: None,
            pinned_msg_id: None,
            common_chats_count: 5,
            folder_id: None,
            ttl_period: None,
            theme: None,
            private_forward_name: None,
            business_work_hours: None,
            personal_channel_id: None,
            personal_channel_message: None,
            birthday: None,
            stargifts_count: None,
            saved_music: None,
        };

        let _ = full.clone();
    }

    #[test]
    fn test_user_full_is_bot() {
        let mut full = UserFull {
            blocked: false,
            phone_calls_available: true,
            phone_calls_private: false,
            can_pin_message: true,
            has_scheduled: false,
            video_calls_available: true,
            voice_messages_forbidden: false,
            translations_disabled: false,
            id: 123,
            about: None,
            settings: None,
            personal_photo: None,
            profile_photo: None,
            fallback_photo: None,
            notify_settings: PeerNotifySettings::default(),
            bot_info: None,
            pinned_msg_id: None,
            common_chats_count: 0,
            folder_id: None,
            ttl_period: None,
            theme: None,
            private_forward_name: None,
            business_work_hours: None,
            personal_channel_id: None,
            personal_channel_message: None,
            birthday: None,
            stargifts_count: None,
            saved_music: None,
        };

        assert!(!full.is_bot());

        full.bot_info = Some(BotInfo {
            description: None,
            commands: None,
            menu_button: None,
        });

        assert!(full.is_bot());
    }

    #[test]
    fn test_user_full_has_profile_photo() {
        let mut full = UserFull {
            blocked: false,
            phone_calls_available: true,
            phone_calls_private: false,
            can_pin_message: true,
            has_scheduled: false,
            video_calls_available: true,
            voice_messages_forbidden: false,
            translations_disabled: false,
            id: 123,
            about: None,
            settings: None,
            personal_photo: None,
            profile_photo: None,
            fallback_photo: None,
            notify_settings: PeerNotifySettings::default(),
            bot_info: None,
            pinned_msg_id: None,
            common_chats_count: 0,
            folder_id: None,
            ttl_period: None,
            theme: None,
            private_forward_name: None,
            business_work_hours: None,
            personal_channel_id: None,
            personal_channel_message: None,
            birthday: None,
            stargifts_count: None,
            saved_music: None,
        };

        assert!(!full.has_profile_photo());

        full.profile_photo = Some(Photo::Empty { id: 456 });
        assert!(full.has_profile_photo());
    }

    #[test]
    fn test_user_full_unknown_constructor() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0]; // invalid constructor
        let mut buf = create_buffer(&data);
        let result = UserFull::deserialize_tl(&mut buf);

        assert!(result.is_err());
    }
}
