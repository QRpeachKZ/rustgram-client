// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Rustgram User Privacy Setting
//!
//! User privacy setting types for Telegram MTProto client.
//!
//! This crate provides type-safe representations of Telegram's user privacy settings,
//! controlling who can interact with a user's account data in various ways.
//!
//! ## Overview
//!
//! Telegram allows users to control privacy for various aspects of their account:
//!
//! - **User Status**: Who can see when the user is online
//! - **Chat Invites**: Who can send chat invitations
//! - **Calls**: Who can make voice/video calls
//! - **Peer to Peer Calls**: Whether to use peer-to-peer connections for calls
//! - **Profile Photo**: Who can see the profile photo
//! - **Phone Number**: Who can see the phone number
//! - **Finding by Phone**: Who can find the user by phone number
//! - **Voice Messages**: Who can send voice messages
//! - **Bio**: Who can see the user bio
//! - **Birthdate**: Who can see the user's birthdate
//! - And more...
//!
//! ## Representation
//!
//! Each privacy setting is represented as an enum variant with a specific i32 value
//! that maps to Telegram's API.
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_user_privacy_setting::UserPrivacySetting;
//!
//! // Create privacy settings
//! let status = UserPrivacySetting::UserStatus;
//! let chat_invite = UserPrivacySetting::ChatInvite;
//!
//! // Convert to/from i32
//! assert_eq!(status.to_i32(), 0);
//! assert_eq!(UserPrivacySetting::from_i32(1), UserPrivacySetting::ChatInvite);
//!
//! // Get human-readable name
//! assert_eq!(status.name(), "UserStatus");
//! ```
//!
//! ## MTProto Alignment
//!
//! This implementation aligns with TDLib's `td::td_api::UserPrivacySetting`:
//! - `userPrivacySettingShowStatus` → `UserPrivacySetting::UserStatus`
//! - `userPrivacySettingAllowChatInvites` → `UserPrivacySetting::ChatInvite`
//! - And 12 more variants...

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]

/// User privacy setting types for Telegram.
///
/// Represents the various privacy settings a user can configure in Telegram.
/// Each variant has a specific i32 value that maps to Telegram's API.
///
/// # Variants
///
/// | Variant | i32 Value | Telegram API | Description |
/// |---------|-----------|--------------|-------------|
/// | `UserStatus` | 0 | `userPrivacySettingShowStatus` | Who can see when user is online |
/// | `ChatInvite` | 1 | `userPrivacySettingAllowChatInvites` | Who can send chat invites |
/// | `Call` | 2 | `userPrivacySettingAllowCalls` | Who can make calls |
/// | `PeerToPeerCall` | 3 | `userPrivacySettingAllowPeerToPeerCalls` | P2P call setting |
/// | `LinkInForwardedMessages` | 4 | `userPrivacySettingShowLinkInForwardedMessages` | Link in forwards |
/// | `UserProfilePhoto` | 5 | `userPrivacySettingShowProfilePhoto` | Profile photo visibility |
/// | `UserPhoneNumber` | 6 | `userPrivacySettingShowPhoneNumber` | Phone number visibility |
/// | `FindByPhoneNumber` | 7 | `userPrivacySettingAllowFindingByPhoneNumber` | Find by phone |
/// | `VoiceMessages` | 8 | `userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages` | Voice messages |
/// | `UserBio` | 9 | `userPrivacySettingShowBio` | Bio visibility |
/// | `UserBirthdate` | 10 | `userPrivacySettingShowBirthdate` | Birthdate visibility |
/// | `StarGiftAutosave` | 11 | `userPrivacySettingAutosaveGifts` | Gift autosave |
/// | `NoPaidMessages` | 12 | `userPrivacySettingAllowUnpaidMessages` | Allow unpaid messages |
/// | `SavedMusic` | 13 | `userPrivacySettingShowProfileAudio` | Saved music |
/// | `Unknown` | 14+ | N/A | Unknown/invalid value |
///
/// # Examples
///
/// ```
/// use rustgram_user_privacy_setting::UserPrivacySetting;
///
/// // Create a privacy setting
/// let setting = UserPrivacySetting::UserStatus;
///
/// // Get its i32 value
/// assert_eq!(setting.to_i32(), 0);
///
/// // Parse from i32
/// assert_eq!(UserPrivacySetting::from_i32(1), UserPrivacySetting::ChatInvite);
///
/// // Unknown values map to Unknown
/// assert_eq!(UserPrivacySetting::from_i32(999), UserPrivacySetting::Unknown);
///
/// // Get human-readable name
/// assert_eq!(UserPrivacySetting::UserStatus.name(), "UserStatus");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum UserPrivacySetting {
    /// Who can see when the user is online (0)
    UserStatus = 0,
    /// Who can send chat invitations to the user (1)
    ChatInvite = 1,
    /// Who can make voice calls to the user (2)
    Call = 2,
    /// Who can make peer-to-peer calls to the user (3)
    PeerToPeerCall = 3,
    /// Whether to show a link to the user's profile in forwarded messages (4)
    LinkInForwardedMessages = 4,
    /// Who can see the user's profile photo (5)
    UserProfilePhoto = 5,
    /// Who can see the user's phone number (6)
    UserPhoneNumber = 6,
    /// Who can find the user by their phone number (7)
    FindByPhoneNumber = 7,
    /// Who can send voice and video note messages to the user (8)
    VoiceMessages = 8,
    /// Who can see the user's bio (9)
    UserBio = 9,
    /// Who can see the user's birthdate (10)
    UserBirthdate = 10,
    /// Whether to automatically save gifts from the user (11)
    StarGiftAutosave = 11,
    /// Whether to allow unpaid messages to the user (12)
    NoPaidMessages = 12,
    /// Who can see the user's saved music (13)
    SavedMusic = 13,
    /// Unknown or invalid privacy setting (14+)
    Unknown = 14,
}

impl UserPrivacySetting {
    /// All known privacy setting variants.
    const ALL_KNOWN: [Self; 14] = [
        Self::UserStatus,
        Self::ChatInvite,
        Self::Call,
        Self::PeerToPeerCall,
        Self::LinkInForwardedMessages,
        Self::UserProfilePhoto,
        Self::UserPhoneNumber,
        Self::FindByPhoneNumber,
        Self::VoiceMessages,
        Self::UserBio,
        Self::UserBirthdate,
        Self::StarGiftAutosave,
        Self::NoPaidMessages,
        Self::SavedMusic,
    ];

    /// Converts an i32 value to a `UserPrivacySetting`.
    ///
    /// Returns `Unknown` for any value outside the known range (0-13).
    ///
    /// # Arguments
    ///
    /// * `value` - The i32 value to convert
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_user_privacy_setting::UserPrivacySetting;
    ///
    /// assert_eq!(UserPrivacySetting::from_i32(0), UserPrivacySetting::UserStatus);
    /// assert_eq!(UserPrivacySetting::from_i32(1), UserPrivacySetting::ChatInvite);
    /// assert_eq!(UserPrivacySetting::from_i32(13), UserPrivacySetting::SavedMusic);
    /// assert_eq!(UserPrivacySetting::from_i32(999), UserPrivacySetting::Unknown);
    /// assert_eq!(UserPrivacySetting::from_i32(-1), UserPrivacySetting::Unknown);
    /// ```
    #[must_use]
    pub const fn from_i32(value: i32) -> Self {
        match value {
            0 => Self::UserStatus,
            1 => Self::ChatInvite,
            2 => Self::Call,
            3 => Self::PeerToPeerCall,
            4 => Self::LinkInForwardedMessages,
            5 => Self::UserProfilePhoto,
            6 => Self::UserPhoneNumber,
            7 => Self::FindByPhoneNumber,
            8 => Self::VoiceMessages,
            9 => Self::UserBio,
            10 => Self::UserBirthdate,
            11 => Self::StarGiftAutosave,
            12 => Self::NoPaidMessages,
            13 => Self::SavedMusic,
            _ => Self::Unknown,
        }
    }

    /// Converts this `UserPrivacySetting` to its i32 representation.
    ///
    /// For `Unknown`, returns 14.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_user_privacy_setting::UserPrivacySetting;
    ///
    /// assert_eq!(UserPrivacySetting::UserStatus.to_i32(), 0);
    /// assert_eq!(UserPrivacySetting::ChatInvite.to_i32(), 1);
    /// assert_eq!(UserPrivacySetting::SavedMusic.to_i32(), 13);
    /// assert_eq!(UserPrivacySetting::Unknown.to_i32(), 14);
    /// ```
    #[must_use]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }

    /// Returns the name of this privacy setting as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_user_privacy_setting::UserPrivacySetting;
    ///
    /// assert_eq!(UserPrivacySetting::UserStatus.name(), "UserStatus");
    /// assert_eq!(UserPrivacySetting::ChatInvite.name(), "ChatInvite");
    /// assert_eq!(UserPrivacySetting::Unknown.name(), "Unknown");
    /// ```
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::UserStatus => "UserStatus",
            Self::ChatInvite => "ChatInvite",
            Self::Call => "Call",
            Self::PeerToPeerCall => "PeerToPeerCall",
            Self::LinkInForwardedMessages => "LinkInForwardedMessages",
            Self::UserProfilePhoto => "UserProfilePhoto",
            Self::UserPhoneNumber => "UserPhoneNumber",
            Self::FindByPhoneNumber => "FindByPhoneNumber",
            Self::VoiceMessages => "VoiceMessages",
            Self::UserBio => "UserBio",
            Self::UserBirthdate => "UserBirthdate",
            Self::StarGiftAutosave => "StarGiftAutosave",
            Self::NoPaidMessages => "NoPaidMessages",
            Self::SavedMusic => "SavedMusic",
            Self::Unknown => "Unknown",
        }
    }

    /// Returns an iterator over all known privacy setting variants.
    ///
    /// Does not include `Unknown`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_user_privacy_setting::UserPrivacySetting;
    ///
    /// let count = UserPrivacySetting::all_known().count();
    /// assert_eq!(count, 14);
    /// ```
    #[must_use]
    pub fn all_known() -> impl Iterator<Item = Self> {
        Self::ALL_KNOWN.iter().copied()
    }

    /// Checks if this is a known (not `Unknown`) privacy setting.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_user_privacy_setting::UserPrivacySetting;
    ///
    /// assert!(UserPrivacySetting::UserStatus.is_known());
    /// assert!(!UserPrivacySetting::Unknown.is_known());
    /// ```
    #[must_use]
    pub const fn is_known(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

impl Default for UserPrivacySetting {
    /// Returns `Unknown` as the default value.
    fn default() -> Self {
        Self::Unknown
    }
}

impl std::fmt::Display for UserPrivacySetting {
    /// Formats the privacy setting for display.
    ///
    /// Shows a human-readable description.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_user_privacy_setting::UserPrivacySetting;
    ///
    /// assert_eq!(format!("{}", UserPrivacySetting::UserStatus), "UserStatus");
    /// assert_eq!(format!("{}", UserPrivacySetting::ChatInvite), "ChatInvite");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<i32> for UserPrivacySetting {
    /// Converts an i32 to a `UserPrivacySetting`.
    ///
    /// Unknown values map to `Unknown`.
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl From<UserPrivacySetting> for i32 {
    /// Converts a `UserPrivacySetting` to its i32 representation.
    fn from(setting: UserPrivacySetting) -> Self {
        setting.to_i32()
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-user-privacy-setting";

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::hash::Hash;

    // === from_i32 Tests ===

    #[test]
    fn test_from_i32_all_known() {
        let expected = [
            (0, UserPrivacySetting::UserStatus),
            (1, UserPrivacySetting::ChatInvite),
            (2, UserPrivacySetting::Call),
            (3, UserPrivacySetting::PeerToPeerCall),
            (4, UserPrivacySetting::LinkInForwardedMessages),
            (5, UserPrivacySetting::UserProfilePhoto),
            (6, UserPrivacySetting::UserPhoneNumber),
            (7, UserPrivacySetting::FindByPhoneNumber),
            (8, UserPrivacySetting::VoiceMessages),
            (9, UserPrivacySetting::UserBio),
            (10, UserPrivacySetting::UserBirthdate),
            (11, UserPrivacySetting::StarGiftAutosave),
            (12, UserPrivacySetting::NoPaidMessages),
            (13, UserPrivacySetting::SavedMusic),
        ];

        for (value, setting) in expected {
            assert_eq!(UserPrivacySetting::from_i32(value), setting);
        }
    }

    #[test]
    fn test_from_i32_unknown() {
        assert_eq!(
            UserPrivacySetting::from_i32(-1),
            UserPrivacySetting::Unknown
        );
        assert_eq!(
            UserPrivacySetting::from_i32(14),
            UserPrivacySetting::Unknown
        );
        assert_eq!(
            UserPrivacySetting::from_i32(999),
            UserPrivacySetting::Unknown
        );
    }

    // === to_i32 Tests ===

    #[test]
    fn test_to_i32_all_known() {
        let expected = [
            (UserPrivacySetting::UserStatus, 0),
            (UserPrivacySetting::ChatInvite, 1),
            (UserPrivacySetting::Call, 2),
            (UserPrivacySetting::PeerToPeerCall, 3),
            (UserPrivacySetting::LinkInForwardedMessages, 4),
            (UserPrivacySetting::UserProfilePhoto, 5),
            (UserPrivacySetting::UserPhoneNumber, 6),
            (UserPrivacySetting::FindByPhoneNumber, 7),
            (UserPrivacySetting::VoiceMessages, 8),
            (UserPrivacySetting::UserBio, 9),
            (UserPrivacySetting::UserBirthdate, 10),
            (UserPrivacySetting::StarGiftAutosave, 11),
            (UserPrivacySetting::NoPaidMessages, 12),
            (UserPrivacySetting::SavedMusic, 13),
        ];

        for (setting, value) in expected {
            assert_eq!(setting.to_i32(), value);
        }
    }

    #[test]
    fn test_to_i32_unknown() {
        assert_eq!(UserPrivacySetting::Unknown.to_i32(), 14);
    }

    #[test]
    fn test_to_i32_roundtrip() {
        let values = [0, 1, 2, 5, 10, 13];
        for value in values {
            let setting = UserPrivacySetting::from_i32(value);
            assert_eq!(setting.to_i32(), value);
        }
    }

    // === name Tests ===

    #[test]
    fn test_name_all_variants() {
        assert_eq!(UserPrivacySetting::UserStatus.name(), "UserStatus");
        assert_eq!(UserPrivacySetting::ChatInvite.name(), "ChatInvite");
        assert_eq!(UserPrivacySetting::Call.name(), "Call");
        assert_eq!(UserPrivacySetting::PeerToPeerCall.name(), "PeerToPeerCall");
        assert_eq!(
            UserPrivacySetting::LinkInForwardedMessages.name(),
            "LinkInForwardedMessages"
        );
        assert_eq!(
            UserPrivacySetting::UserProfilePhoto.name(),
            "UserProfilePhoto"
        );
        assert_eq!(
            UserPrivacySetting::UserPhoneNumber.name(),
            "UserPhoneNumber"
        );
        assert_eq!(
            UserPrivacySetting::FindByPhoneNumber.name(),
            "FindByPhoneNumber"
        );
        assert_eq!(UserPrivacySetting::VoiceMessages.name(), "VoiceMessages");
        assert_eq!(UserPrivacySetting::UserBio.name(), "UserBio");
        assert_eq!(UserPrivacySetting::UserBirthdate.name(), "UserBirthdate");
        assert_eq!(
            UserPrivacySetting::StarGiftAutosave.name(),
            "StarGiftAutosave"
        );
        assert_eq!(UserPrivacySetting::NoPaidMessages.name(), "NoPaidMessages");
        assert_eq!(UserPrivacySetting::SavedMusic.name(), "SavedMusic");
        assert_eq!(UserPrivacySetting::Unknown.name(), "Unknown");
    }

    // === all_known Tests ===

    #[test]
    fn test_all_known_count() {
        let count = UserPrivacySetting::all_known().count();
        assert_eq!(count, 14);
    }

    #[test]
    fn test_all_known_no_unknown() {
        for setting in UserPrivacySetting::all_known() {
            assert!(setting.is_known());
        }
    }

    // === is_known Tests ===

    #[test]
    fn test_is_known_true() {
        assert!(UserPrivacySetting::UserStatus.is_known());
        assert!(UserPrivacySetting::ChatInvite.is_known());
        assert!(UserPrivacySetting::SavedMusic.is_known());
    }

    #[test]
    fn test_is_known_false() {
        assert!(!UserPrivacySetting::Unknown.is_known());
    }

    // === Default Tests ===

    #[test]
    fn test_default_is_unknown() {
        assert_eq!(UserPrivacySetting::default(), UserPrivacySetting::Unknown);
    }

    // === Display Tests ===

    #[test]
    fn test_display_all_variants() {
        let settings = [
            UserPrivacySetting::UserStatus,
            UserPrivacySetting::ChatInvite,
            UserPrivacySetting::Call,
            UserPrivacySetting::PeerToPeerCall,
            UserPrivacySetting::LinkInForwardedMessages,
            UserPrivacySetting::UserProfilePhoto,
            UserPrivacySetting::UserPhoneNumber,
            UserPrivacySetting::FindByPhoneNumber,
            UserPrivacySetting::VoiceMessages,
            UserPrivacySetting::UserBio,
            UserPrivacySetting::UserBirthdate,
            UserPrivacySetting::StarGiftAutosave,
            UserPrivacySetting::NoPaidMessages,
            UserPrivacySetting::SavedMusic,
        ];

        for setting in settings {
            assert_eq!(format!("{}", setting), setting.name());
        }
    }

    // === Copy/Send/Sync Tests ===

    #[test]
    fn test_copy_semantics() {
        let a = UserPrivacySetting::UserStatus;
        let b = a;
        assert_eq!(a, UserPrivacySetting::UserStatus);
        assert_eq!(b, UserPrivacySetting::UserStatus);
    }

    // === From/i32 Tests ===

    #[test]
    fn test_from_i32_trait() {
        let setting: UserPrivacySetting = 5.into();
        assert_eq!(setting, UserPrivacySetting::UserProfilePhoto);
    }

    #[test]
    fn test_into_i32_trait() {
        let value: i32 = UserPrivacySetting::ChatInvite.into();
        assert_eq!(value, 1);
    }

    // === Eq Tests ===

    #[test]
    fn test_eq_same() {
        assert_eq!(
            UserPrivacySetting::UserStatus,
            UserPrivacySetting::UserStatus
        );
        assert_eq!(
            UserPrivacySetting::ChatInvite,
            UserPrivacySetting::ChatInvite
        );
    }

    #[test]
    fn test_eq_different() {
        assert_ne!(
            UserPrivacySetting::UserStatus,
            UserPrivacySetting::ChatInvite
        );
        assert_ne!(UserPrivacySetting::Call, UserPrivacySetting::PeerToPeerCall);
    }

    // === Hash Tests ===

    #[test]
    fn test_hash_consistency() {
        use std::hash::Hasher;

        let a = UserPrivacySetting::UserStatus;
        let b = UserPrivacySetting::UserStatus;

        let mut hasher_a = std::collections::hash_map::DefaultHasher::new();
        let mut hasher_b = std::collections::hash_map::DefaultHasher::new();

        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);

        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn test_hash_in_set() {
        let mut set = HashSet::new();
        set.insert(UserPrivacySetting::UserStatus);
        set.insert(UserPrivacySetting::ChatInvite);
        set.insert(UserPrivacySetting::UserStatus); // Duplicate

        assert_eq!(set.len(), 2);
    }

    // === Version Info Tests ===

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-user-privacy-setting");
    }

    // === Specific Variant Tests ===

    #[test]
    fn test_user_status() {
        let setting = UserPrivacySetting::UserStatus;
        assert_eq!(setting.to_i32(), 0);
        assert_eq!(setting.name(), "UserStatus");
        assert!(setting.is_known());
    }

    #[test]
    fn test_chat_invite() {
        let setting = UserPrivacySetting::ChatInvite;
        assert_eq!(setting.to_i32(), 1);
        assert_eq!(setting.name(), "ChatInvite");
        assert!(setting.is_known());
    }

    #[test]
    fn test_call_settings() {
        let call = UserPrivacySetting::Call;
        let p2p = UserPrivacySetting::PeerToPeerCall;

        assert_eq!(call.to_i32(), 2);
        assert_eq!(p2p.to_i32(), 3);
        assert_ne!(call, p2p);
    }

    #[test]
    fn test_photo_settings() {
        let photo = UserPrivacySetting::UserProfilePhoto;
        assert_eq!(photo.to_i32(), 5);
        assert!(photo.is_known());
    }

    #[test]
    fn test_phone_settings() {
        let show_phone = UserPrivacySetting::UserPhoneNumber;
        let find_by_phone = UserPrivacySetting::FindByPhoneNumber;

        assert_eq!(show_phone.to_i32(), 6);
        assert_eq!(find_by_phone.to_i32(), 7);
        assert_ne!(show_phone, find_by_phone);
    }

    #[test]
    fn test_voice_messages() {
        let setting = UserPrivacySetting::VoiceMessages;
        assert_eq!(setting.to_i32(), 8);
        assert_eq!(setting.name(), "VoiceMessages");
    }

    #[test]
    fn test_newer_settings() {
        let birthdate = UserPrivacySetting::UserBirthdate;
        let gift_autosave = UserPrivacySetting::StarGiftAutosave;
        let paid_messages = UserPrivacySetting::NoPaidMessages;
        let saved_music = UserPrivacySetting::SavedMusic;

        assert_eq!(birthdate.to_i32(), 10);
        assert_eq!(gift_autosave.to_i32(), 11);
        assert_eq!(paid_messages.to_i32(), 12);
        assert_eq!(saved_music.to_i32(), 13);

        assert!(birthdate.is_known());
        assert!(gift_autosave.is_known());
        assert!(paid_messages.is_known());
        assert!(saved_music.is_known());
    }
}
