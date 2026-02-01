// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification settings types for Telegram.
//!
//! This module provides TL deserialization for notification-related types.
//!
//! # TL Schema
//!
//! ```text
//! peerNotifySettings#99622c0c flags:# show_previews:flags.0?Bool silent:flags.1?Bool
//!     mute_until:flags.2?int ios_sound:flags.3?NotificationSound
//!     android_sound:flags.4?NotificationSound other_sound:flags.5?NotificationSound
//!     stories_muted:flags.6?Bool stories_hide_sender:flags.7?Bool
//!     stories_ios_sound:flags.8?NotificationSound
//!     stories_android_sound:flags.9?NotificationSound
//!     stories_other_sound:flags.10?NotificationSound = PeerNotifySettings;
//!
//! inputPeerNotifySettings#cacb6ae2 flags:# show_previews:flags.0?Bool silent:flags.1?Bool
//!     mute_until:flags.2?int sound:flags.3?NotificationSound
//!     stories_muted:flags.6?Bool stories_hide_sender:flags.7?Bool
//!     stories_sound:flags.8?NotificationSound = InputPeerNotifySettings;
//!
//! notificationSoundDefault#97e8bebe = NotificationSound;
//! notificationSoundNone#6f0c34df = NotificationSound;
//! notificationSoundLocal#830b9ae4 title:string data:string = NotificationSound;
//! notificationSoundRingtone#ff6c8049 id:long = NotificationSound;
//!
//! boolFalse#bc799737 = Bool;
//! boolTrue#997275b5 = Bool;
//! ```

use crate::flags::FlagReader;
use rustgram_types::tl::{Bytes, TlDeserialize, TlHelper};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Peer notification settings.
///
/// Controls how notifications are displayed for a specific peer.
///
/// # TL Schema
///
/// ```text
/// peerNotifySettings#99622c0c flags:# show_previews:flags.0?Bool silent:flags.1?Bool
///     mute_until:flags.2?int ios_sound:flags.3?NotificationSound
///     android_sound:flags.4?NotificationSound other_sound:flags.5?NotificationSound
///     stories_muted:flags.6?Bool stories_hide_sender:flags.7?Bool
///     stories_ios_sound:flags.8?NotificationSound
///     stories_android_sound:flags.9?NotificationSound
///     stories_other_sound:flags.10?NotificationSound = PeerNotifySettings;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_tl_core::PeerNotifySettings;
///
/// let settings = PeerNotifySettings {
///     show_previews: Some(true),
///     silent: Some(false),
///     mute_until: Some(0),
///     ios_sound: None,
///     android_sound: None,
///     other_sound: None,
///     stories_muted: None,
///     stories_hide_sender: None,
///     stories_ios_sound: None,
///     stories_android_sound: None,
///     stories_other_sound: None,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerNotifySettings {
    /// Whether to show message previews in notifications.
    pub show_previews: Option<bool>,

    /// Whether to silently deliver messages without sound.
    pub silent: Option<bool>,

    /// Unix timestamp until which notifications are muted (0 = not muted).
    pub mute_until: Option<i32>,

    /// iOS notification sound.
    pub ios_sound: Option<NotificationSound>,

    /// Android notification sound.
    pub android_sound: Option<NotificationSound>,

    /// Other platforms notification sound.
    pub other_sound: Option<NotificationSound>,

    /// Whether to mute stories.
    pub stories_muted: Option<bool>,

    /// Whether to hide the sender in story notifications.
    pub stories_hide_sender: Option<bool>,

    /// Stories iOS notification sound.
    pub stories_ios_sound: Option<NotificationSound>,

    /// Stories Android notification sound.
    pub stories_android_sound: Option<NotificationSound>,

    /// Stories other platforms notification sound.
    pub stories_other_sound: Option<NotificationSound>,
}

impl PeerNotifySettings {
    /// Constructor ID for peerNotifySettings.
    pub const CONSTRUCTOR: u32 = 0x99622c0c;

    /// Creates default notification settings.
    pub fn default_settings() -> Self {
        Self {
            show_previews: Some(true),
            silent: Some(false),
            mute_until: Some(0),
            ios_sound: None,
            android_sound: None,
            other_sound: None,
            stories_muted: Some(false),
            stories_hide_sender: Some(false),
            stories_ios_sound: None,
            stories_android_sound: None,
            stories_other_sound: None,
        }
    }

    /// Checks if notifications are muted.
    pub fn is_muted(&self) -> bool {
        match self.mute_until {
            Some(until) => until > 0 && until > crate::utils::current_timestamp(),
            None => false,
        }
    }
}

impl TlDeserialize for PeerNotifySettings {
    fn deserialize_tl(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        if constructor_id != Self::CONSTRUCTOR {
            let tl_err = crate::error::TlError::unknown_constructor(
                vec![Self::CONSTRUCTOR],
                constructor_id,
                "PeerNotifySettings",
            );
            return Err(rustgram_types::TypeError::from(tl_err));
        }

        let flags = TlHelper::read_i32(buf)? as u32;
        let flag_reader = FlagReader::new(flags);

        let show_previews = flag_reader.read_optional(0, || read_bool(buf))?;
        let silent = flag_reader.read_optional(1, || read_bool(buf))?;
        let mute_until = flag_reader.read_optional(2, || TlHelper::read_i32(buf))?;
        let ios_sound = flag_reader.read_optional(3, || NotificationSound::deserialize_tl(buf))?;
        let android_sound =
            flag_reader.read_optional(4, || NotificationSound::deserialize_tl(buf))?;
        let other_sound =
            flag_reader.read_optional(5, || NotificationSound::deserialize_tl(buf))?;
        let stories_muted = flag_reader.read_optional(6, || read_bool(buf))?;
        let stories_hide_sender = flag_reader.read_optional(7, || read_bool(buf))?;
        let stories_ios_sound =
            flag_reader.read_optional(8, || NotificationSound::deserialize_tl(buf))?;
        let stories_android_sound =
            flag_reader.read_optional(9, || NotificationSound::deserialize_tl(buf))?;
        let stories_other_sound =
            flag_reader.read_optional(10, || NotificationSound::deserialize_tl(buf))?;

        Ok(Self {
            show_previews,
            silent,
            mute_until,
            ios_sound,
            android_sound,
            other_sound,
            stories_muted,
            stories_hide_sender,
            stories_ios_sound,
            stories_android_sound,
            stories_other_sound,
        })
    }
}

impl Default for PeerNotifySettings {
    fn default() -> Self {
        Self::default_settings()
    }
}

/// Input peer notification settings (for API requests).
///
/// # TL Schema
///
/// ```text
/// inputPeerNotifySettings#cacb6ae2 flags:# show_previews:flags.0?Bool silent:flags.1?Bool
///     mute_until:flags.2?int sound:flags.3?NotificationSound
///     stories_muted:flags.6?Bool stories_hide_sender:flags.7?Bool
///     stories_sound:flags.8?NotificationSound = InputPeerNotifySettings;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputPeerNotifySettings {
    /// Whether to show message previews in notifications.
    pub show_previews: Option<bool>,

    /// Whether to silently deliver messages without sound.
    pub silent: Option<bool>,

    /// Unix timestamp until which notifications are muted.
    pub mute_until: Option<i32>,

    /// Notification sound.
    pub sound: Option<NotificationSound>,

    /// Whether to mute stories.
    pub stories_muted: Option<bool>,

    /// Whether to hide the sender in story notifications.
    pub stories_hide_sender: Option<bool>,

    /// Stories notification sound.
    pub stories_sound: Option<NotificationSound>,
}

impl InputPeerNotifySettings {
    /// Constructor ID for inputPeerNotifySettings.
    pub const CONSTRUCTOR: u32 = 0xcacb6ae2;

    /// Creates default input settings.
    pub fn default_settings() -> Self {
        Self {
            show_previews: None,
            silent: None,
            mute_until: None,
            sound: None,
            stories_muted: None,
            stories_hide_sender: None,
            stories_sound: None,
        }
    }
}

/// Notification sound type.
///
/// Represents different kinds of notification sounds.
///
/// # TL Schema
///
/// ```text
/// notificationSoundDefault#97e8bebe = NotificationSound;
/// notificationSoundNone#6f0c34df = NotificationSound;
/// notificationSoundLocal#830b9ae4 title:string data:string = NotificationSound;
/// notificationSoundRingtone#ff6c8049 id:long = NotificationSound;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationSound {
    /// Default notification sound.
    Default,

    /// No sound (silent).
    None,

    /// Local sound file.
    Local {
        /// Sound title.
        title: String,
        /// Sound data (file path or identifier).
        data: String,
    },

    /// Ringtone sound.
    Ringtone {
        /// Ringtone ID.
        id: i64,
    },
}

impl NotificationSound {
    /// Constructor ID for notificationSoundDefault.
    pub const DEFAULT_CONSTRUCTOR: u32 = 0x97e8bebe;

    /// Constructor ID for notificationSoundNone.
    pub const NONE_CONSTRUCTOR: u32 = 0x6f0c34df;

    /// Constructor ID for notificationSoundLocal.
    pub const LOCAL_CONSTRUCTOR: u32 = 0x830b9ae4;

    /// Constructor ID for notificationSoundRingtone.
    pub const RINGTONE_CONSTRUCTOR: u32 = 0xff6c8049;

    /// Checks if this is the default sound.
    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }

    /// Checks if this is silent (no sound).
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl TlDeserialize for NotificationSound {
    fn deserialize_tl(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            Self::DEFAULT_CONSTRUCTOR => Ok(Self::Default),
            Self::NONE_CONSTRUCTOR => Ok(Self::None),
            Self::LOCAL_CONSTRUCTOR => {
                let title = TlHelper::read_string(buf)?;
                let data = TlHelper::read_string(buf)?;
                Ok(Self::Local { title, data })
            }
            Self::RINGTONE_CONSTRUCTOR => {
                let id = TlHelper::read_i64(buf)?;
                Ok(Self::Ringtone { id })
            }
            _ => {
                let tl_err = crate::error::TlError::unknown_constructor(
                    vec![
                        Self::DEFAULT_CONSTRUCTOR,
                        Self::NONE_CONSTRUCTOR,
                        Self::LOCAL_CONSTRUCTOR,
                        Self::RINGTONE_CONSTRUCTOR,
                    ],
                    constructor_id,
                    "NotificationSound",
                );
                Err(rustgram_types::TypeError::from(tl_err))
            }
        }
    }
}

impl fmt::Display for NotificationSound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "Default"),
            Self::None => write!(f, "None"),
            Self::Local { title, .. } => write!(f, "Local({})", title),
            Self::Ringtone { id } => write!(f, "Ringtone({})", id),
        }
    }
}

/// TL Bool type.
///
/// Telegram uses a specific Bool type with constructor IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TlBool(pub bool);

impl TlBool {
    /// Constructor ID for boolTrue.
    pub const TRUE_CONSTRUCTOR: u32 = 0x997275b5;

    /// Constructor ID for boolFalse.
    pub const FALSE_CONSTRUCTOR: u32 = 0xbc799737;

    /// Returns the boolean value.
    pub const fn as_bool(&self) -> bool {
        self.0
    }
}

impl TlDeserialize for TlBool {
    fn deserialize_tl(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            Self::TRUE_CONSTRUCTOR => Ok(Self(true)),
            Self::FALSE_CONSTRUCTOR => Ok(Self(false)),
            _ => {
                let tl_err = crate::error::TlError::unknown_constructor(
                    vec![Self::TRUE_CONSTRUCTOR, Self::FALSE_CONSTRUCTOR],
                    constructor_id,
                    "Bool",
                );
                Err(rustgram_types::TypeError::from(tl_err))
            }
        }
    }
}

/// Reads a TL Bool value from the buffer.
fn read_bool(buf: &mut Bytes) -> rustgram_types::TypeResult<bool> {
    let tl_bool = TlBool::deserialize_tl(buf)?;
    Ok(tl_bool.as_bool())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_buffer(data: &[u8]) -> Bytes {
        Bytes::new(bytes::Bytes::copy_from_slice(data))
    }

    #[test]
    fn test_tl_bool_deserialize() {
        // boolTrue#997275b5 = Bool;
        let data = [0xb5, 0x75, 0x72, 0x99]; // boolTrue constructor

        let mut buf = create_buffer(&data);
        let result = read_bool(&mut buf).unwrap();
        assert!(result);

        // boolFalse#bc799737 = Bool;
        let data = [0x37, 0x97, 0x79, 0xbc]; // boolFalse constructor

        let mut buf = create_buffer(&data);
        let result = read_bool(&mut buf).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_tl_bool_constructors() {
        assert_eq!(TlBool::TRUE_CONSTRUCTOR, 0x997275b5);
        assert_eq!(TlBool::FALSE_CONSTRUCTOR, 0xbc799737);
    }

    #[test]
    fn test_notification_sound_default() {
        // notificationSoundDefault#97e8bebe = NotificationSound;
        let data = [0xbe, 0xbe, 0xe8, 0x97]; // constructor

        let mut buf = create_buffer(&data);
        let result = NotificationSound::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result, NotificationSound::Default);
        assert!(result.is_default());
        assert!(!result.is_none());
    }

    #[test]
    fn test_notification_sound_none() {
        // notificationSoundNone#6f0c34df = NotificationSound;
        let data = [0xdf, 0x34, 0x0c, 0x6f]; // constructor

        let mut buf = create_buffer(&data);
        let result = NotificationSound::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result, NotificationSound::None);
        assert!(!result.is_default());
        assert!(result.is_none());
    }

    #[test]
    fn test_notification_sound_ringtone() {
        // notificationSoundRingtone#ff6c8049 id:long = NotificationSound;
        let mut data = vec![0x49, 0x80, 0x6c, 0xff]; // constructor
        data.extend_from_slice(&12345i64.to_le_bytes()); // id

        let mut buf = create_buffer(&data);
        let result = NotificationSound::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result, NotificationSound::Ringtone { id: 12345 });
    }

    #[test]
    fn test_notification_sound_constructors() {
        assert_eq!(NotificationSound::DEFAULT_CONSTRUCTOR, 0x97e8bebe);
        assert_eq!(NotificationSound::NONE_CONSTRUCTOR, 0x6f0c34df);
        assert_eq!(NotificationSound::LOCAL_CONSTRUCTOR, 0x830b9ae4);
        assert_eq!(NotificationSound::RINGTONE_CONSTRUCTOR, 0xff6c8049);
    }

    #[test]
    fn test_notification_sound_display() {
        assert_eq!(format!("{}", NotificationSound::Default), "Default");
        assert_eq!(format!("{}", NotificationSound::None), "None");
        assert_eq!(
            format!(
                "{}",
                NotificationSound::Local {
                    title: "My Sound".to_string(),
                    data: "data".to_string()
                }
            ),
            "Local(My Sound)"
        );
        assert_eq!(
            format!("{}", NotificationSound::Ringtone { id: 123 }),
            "Ringtone(123)"
        );
    }

    #[test]
    fn test_peer_notify_settings_constructors() {
        assert_eq!(PeerNotifySettings::CONSTRUCTOR, 0x99622c0c);
        assert_eq!(InputPeerNotifySettings::CONSTRUCTOR, 0xcacb6ae2);
    }

    #[test]
    fn test_peer_notify_settings_default() {
        let settings = PeerNotifySettings::default_settings();

        assert_eq!(settings.show_previews, Some(true));
        assert_eq!(settings.silent, Some(false));
        assert_eq!(settings.mute_until, Some(0));
        assert!(settings.ios_sound.is_none());
        assert!(settings.android_sound.is_none());
        assert!(settings.other_sound.is_none());
    }

    #[test]
    fn test_input_peer_notify_settings_default() {
        let settings = InputPeerNotifySettings::default_settings();

        assert!(settings.show_previews.is_none());
        assert!(settings.silent.is_none());
        assert!(settings.mute_until.is_none());
        assert!(settings.sound.is_none());
    }

    #[test]
    fn test_peer_notify_settings_minimal() {
        // peerNotifySettings with no flags set (all None)
        let mut data = vec![0x0c, 0x2c, 0x62, 0x99]; // constructor
        data.extend_from_slice(&0i32.to_le_bytes()); // flags = 0

        let mut buf = create_buffer(&data);
        let result = PeerNotifySettings::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result.show_previews, None);
        assert_eq!(result.silent, None);
        assert_eq!(result.mute_until, None);
    }

    #[test]
    fn test_notification_sound_unknown_constructor() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF]; // invalid constructor
        let mut buf = create_buffer(&data);
        let result = NotificationSound::deserialize_tl(&mut buf);

        assert!(result.is_err());
    }

    #[test]
    fn test_tl_bool_unknown_constructor() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF]; // invalid constructor
        let mut buf = create_buffer(&data);
        let result = TlBool::deserialize_tl(&mut buf);

        assert!(result.is_err());
    }

    #[test]
    fn test_peer_notify_settings_unknown_constructor() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0]; // invalid constructor
        let mut buf = create_buffer(&data);
        let result = PeerNotifySettings::deserialize_tl(&mut buf);

        assert!(result.is_err());
    }

    #[test]
    fn test_tl_bool_copy() {
        let bool1 = TlBool(true);
        let bool2 = bool1;
        assert_eq!(bool1.0, bool2.0);
    }

    #[test]
    fn test_notification_sound_clone() {
        let sound1 = NotificationSound::Local {
            title: "Test".to_string(),
            data: "data".to_string(),
        };
        let sound2 = sound1.clone();
        assert_eq!(sound1, sound2);
    }

    // Additional tests to increase coverage

    #[test]
    fn test_notification_sound_is_default() {
        assert!(NotificationSound::Default.is_default());
        assert!(!NotificationSound::None.is_default());
        assert!(!NotificationSound::Local {
            title: "test".to_string(),
            data: "data".to_string()
        }.is_default());
        assert!(!NotificationSound::Ringtone { id: 123 }.is_default());
    }

    #[test]
    fn test_notification_sound_is_none() {
        assert!(!NotificationSound::Default.is_none());
        assert!(NotificationSound::None.is_none());
        assert!(!NotificationSound::Local {
            title: "test".to_string(),
            data: "data".to_string()
        }.is_none());
        assert!(!NotificationSound::Ringtone { id: 123 }.is_none());
    }

    #[test]
    fn test_tl_bool_as_bool() {
        let bool_true = TlBool(true);
        assert!(bool_true.as_bool());

        let bool_false = TlBool(false);
        assert!(!bool_false.as_bool());
    }

    #[test]
    fn test_tl_bool_from() {
        let bool1 = TlBool(true);
        assert!(bool1.as_bool());

        let bool2 = TlBool(false);
        assert!(!bool2.as_bool());
    }

    #[test]
    fn test_peer_notify_settings_is_muted_no_mute() {
        let settings = PeerNotifySettings {
            show_previews: None,
            silent: None,
            mute_until: Some(0),
            ios_sound: None,
            android_sound: None,
            other_sound: None,
            stories_muted: None,
            stories_hide_sender: None,
            stories_ios_sound: None,
            stories_android_sound: None,
            stories_other_sound: None,
        };

        assert!(!settings.is_muted());
    }

    #[test]
    fn test_peer_notify_settings_is_muted_past() {
        let settings = PeerNotifySettings {
            show_previews: None,
            silent: None,
            // Set mute_until to a past timestamp
            mute_until: Some(1600000000), // 2020-09-13
            ios_sound: None,
            android_sound: None,
            other_sound: None,
            stories_muted: None,
            stories_hide_sender: None,
            stories_ios_sound: None,
            stories_android_sound: None,
            stories_other_sound: None,
        };

        // Should not be muted since the time is in the past
        assert!(!settings.is_muted());
    }

    #[test]
    fn test_peer_notify_settings_is_muted_future() {
        let future_timestamp = crate::utils::current_timestamp() + 3600; // 1 hour from now

        let settings = PeerNotifySettings {
            show_previews: None,
            silent: None,
            mute_until: Some(future_timestamp),
            ios_sound: None,
            android_sound: None,
            other_sound: None,
            stories_muted: None,
            stories_hide_sender: None,
            stories_ios_sound: None,
            stories_android_sound: None,
            stories_other_sound: None,
        };

        assert!(settings.is_muted());
    }

    #[test]
    fn test_peer_notify_settings_is_muted_none() {
        let settings = PeerNotifySettings {
            show_previews: None,
            silent: None,
            mute_until: None,
            ios_sound: None,
            android_sound: None,
            other_sound: None,
            stories_muted: None,
            stories_hide_sender: None,
            stories_ios_sound: None,
            stories_android_sound: None,
            stories_other_sound: None,
        };

        assert!(!settings.is_muted());
    }

    #[test]
    fn test_input_peer_notify_settings_all_none() {
        let settings = InputPeerNotifySettings::default_settings();
        assert!(settings.show_previews.is_none());
        assert!(settings.silent.is_none());
        assert!(settings.mute_until.is_none());
        assert!(settings.sound.is_none());
        assert!(settings.stories_muted.is_none());
        assert!(settings.stories_hide_sender.is_none());
        assert!(settings.stories_sound.is_none());
    }

    #[test]
    fn test_peer_notify_settings_clone() {
        let settings1 = PeerNotifySettings {
            show_previews: Some(true),
            silent: Some(false),
            mute_until: Some(123),
            ios_sound: Some(NotificationSound::Default),
            android_sound: None,
            other_sound: None,
            stories_muted: None,
            stories_hide_sender: None,
            stories_ios_sound: None,
            stories_android_sound: None,
            stories_other_sound: None,
        };

        let settings2 = settings1.clone();
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_input_peer_notify_settings_clone() {
        let settings1 = InputPeerNotifySettings {
            show_previews: Some(true),
            silent: Some(false),
            mute_until: Some(123),
            sound: Some(NotificationSound::Default),
            stories_muted: None,
            stories_hide_sender: None,
            stories_sound: None,
        };

        let settings2 = settings1.clone();
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_notification_sound_equality() {
        let sound1 = NotificationSound::Default;
        let sound2 = NotificationSound::Default;
        assert_eq!(sound1, sound2);

        let sound3 = NotificationSound::None;
        assert_ne!(sound1, sound3);

        let sound4 = NotificationSound::Local {
            title: "Test".to_string(),
            data: "data".to_string()
        };
        let sound5 = NotificationSound::Local {
            title: "Test".to_string(),
            data: "data".to_string()
        };
        assert_eq!(sound4, sound5);

        let sound6 = NotificationSound::Local {
            title: "Other".to_string(),
            data: "data".to_string()
        };
        assert_ne!(sound4, sound6);
    }

    #[test]
    fn test_notification_sound_ringtone_equality() {
        let sound1 = NotificationSound::Ringtone { id: 123 };
        let sound2 = NotificationSound::Ringtone { id: 123 };
        assert_eq!(sound1, sound2);

        let sound3 = NotificationSound::Ringtone { id: 456 };
        assert_ne!(sound1, sound3);
    }

    #[test]
    fn test_tl_bool_equality() {
        assert_eq!(TlBool(true), TlBool(true));
        assert_eq!(TlBool(false), TlBool(false));
        assert_ne!(TlBool(true), TlBool(false));
    }
}
