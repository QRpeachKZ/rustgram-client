// Copyright (c) 2025 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Privacy key type.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Privacy setting key.
///
/// Represents the different privacy settings that can be configured.
/// Based on TDLib's privacy keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum PrivacyKey {
    /// User status (online/offline).
    Status = 0,
    /// Chat invitations.
    ChatInvite = 1,
    /// Voice/video calls.
    Call = 2,
    /// Peer-to-peer calls.
    PeerToPeerCall = 3,
    /// Profile photo visibility.
    ProfilePhoto = 4,
    /// Phone number visibility.
    PhoneNumber = 5,
    /// Phone number status visibility.
    PhoneNumberStatus = 6,
    /// Bio visibility.
    Bio = 7,
    /// Username visibility.
    Username = 8,
    /// Birthdate visibility.
    Birthdate = 9,
}

impl PrivacyKey {
    /// All known privacy keys.
    pub const ALL_KNOWN: [Self; 10] = [
        Self::Status,
        Self::ChatInvite,
        Self::Call,
        Self::PeerToPeerCall,
        Self::ProfilePhoto,
        Self::PhoneNumber,
        Self::PhoneNumberStatus,
        Self::Bio,
        Self::Username,
        Self::Birthdate,
    ];

    /// Converts an i32 to a PrivacyKey.
    ///
    /// Returns None for unknown values.
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Status),
            1 => Some(Self::ChatInvite),
            2 => Some(Self::Call),
            3 => Some(Self::PeerToPeerCall),
            4 => Some(Self::ProfilePhoto),
            5 => Some(Self::PhoneNumber),
            6 => Some(Self::PhoneNumberStatus),
            7 => Some(Self::Bio),
            8 => Some(Self::Username),
            9 => Some(Self::Birthdate),
            _ => None,
        }
    }

    /// Converts this PrivacyKey to its i32 representation.
    #[must_use]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }

    /// Returns an iterator over all known privacy keys.
    #[must_use]
    pub fn all_known() -> impl Iterator<Item = Self> {
        Self::ALL_KNOWN.iter().copied()
    }

    /// Returns the name of this privacy key.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Status => "Status",
            Self::ChatInvite => "ChatInvite",
            Self::Call => "Call",
            Self::PeerToPeerCall => "PeerToPeerCall",
            Self::ProfilePhoto => "ProfilePhoto",
            Self::PhoneNumber => "PhoneNumber",
            Self::PhoneNumberStatus => "PhoneNumberStatus",
            Self::Bio => "Bio",
            Self::Username => "Username",
            Self::Birthdate => "Birthdate",
        }
    }
}

impl Default for PrivacyKey {
    fn default() -> Self {
        Self::Status
    }
}

impl fmt::Display for PrivacyKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_key_from_i32() {
        assert_eq!(PrivacyKey::from_i32(0), Some(PrivacyKey::Status));
        assert_eq!(PrivacyKey::from_i32(9), Some(PrivacyKey::Birthdate));
        assert_eq!(PrivacyKey::from_i32(999), None);
    }

    #[test]
    fn test_privacy_key_to_i32() {
        assert_eq!(PrivacyKey::Status.to_i32(), 0);
        assert_eq!(PrivacyKey::Birthdate.to_i32(), 9);
    }

    #[test]
    fn test_privacy_key_name() {
        assert_eq!(PrivacyKey::Status.name(), "Status");
        assert_eq!(PrivacyKey::Birthdate.name(), "Birthdate");
    }

    #[test]
    fn test_privacy_key_all_known() {
        let keys: Vec<_> = PrivacyKey::all_known().collect();
        assert_eq!(keys.len(), 10);
    }

    #[test]
    fn test_privacy_key_default() {
        assert_eq!(PrivacyKey::default(), PrivacyKey::Status);
    }

    #[test]
    fn test_privacy_key_display() {
        assert_eq!(format!("{}", PrivacyKey::Status), "Status");
        assert_eq!(format!("{}", PrivacyKey::Birthdate), "Birthdate");
    }

    #[test]
    fn test_privacy_key_equality() {
        assert_eq!(PrivacyKey::Status, PrivacyKey::Status);
        assert_ne!(PrivacyKey::Status, PrivacyKey::ChatInvite);
    }

    #[test]
    fn test_privacy_key_repr() {
        assert_eq!(PrivacyKey::Status as i32, 0);
        assert_eq!(PrivacyKey::Birthdate as i32, 9);
    }
}
