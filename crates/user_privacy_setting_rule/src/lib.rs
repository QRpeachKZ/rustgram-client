// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # User Privacy Setting Rule
//!
//! Privacy rules for Telegram users.
//!
//! ## Overview
//!
//! Defines privacy settings and rules for user data access.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_user_privacy_setting_rule::{PrivacyRule, PrivacyKey};
//!
//! let rule = PrivacyRule::AllowAll;
//! assert!(rule.is_all_allowed());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Privacy setting key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum PrivacyKey {
    /// Status
    Status = 0,
    /// Profile photo
    ProfilePhoto = 1,
    /// Phone number
    PhoneNumber = 2,
    /// Phone number status
    PhoneNumberStatus = 3,
    /// Bio
    Bio = 4,
    /// Username
    Username = 5,
    /// Birthdate
    Birthdate = 6,
}

/// Privacy value type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum PrivacyValueType {
    /// Allow all
    AllowAll = 0,
    /// Allow contacts
    AllowContacts = 1,
    /// Allow premium users
    AllowPremium = 2,
    /// Allow bots
    AllowBots = 3,
    /// Allow users
    AllowUsers = 4,
    /// Allow chat members
    AllowChatMembers = 5,
    /// Disallow all
    DisallowAll = 6,
    /// Disallow contacts
    DisallowContacts = 7,
    /// Disallow bots
    DisallowBots = 8,
}

/// Privacy rule
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivacyRule {
    /// Allow all users
    AllowAll,
    /// Allow only contacts
    AllowContacts,
    /// Allow only premium users
    AllowPremium,
    /// Allow only bots
    AllowBots,
    /// Allow specific users
    AllowUsers(Vec<i64>),
    /// Allow chat members
    AllowChatMembers(i64),
    /// Disallow all users
    DisallowAll,
    /// Disallow specific users
    DisallowContacts,
    /// Disallow bots
    DisallowBots,
    /// Disallow specific users
    DisallowUsers(Vec<i64>),
}

impl PrivacyRule {
    /// Returns true if this rule allows all users
    #[must_use]
    pub fn is_all_allowed(&self) -> bool {
        matches!(self, Self::AllowAll)
    }

    /// Returns true if this rule disallows all users
    #[must_use]
    pub fn is_all_disallowed(&self) -> bool {
        matches!(self, Self::DisallowAll)
    }

    /// Returns the value type of this rule
    #[must_use]
    pub fn value_type(&self) -> PrivacyValueType {
        match self {
            Self::AllowAll => PrivacyValueType::AllowAll,
            Self::AllowContacts => PrivacyValueType::AllowContacts,
            Self::AllowPremium => PrivacyValueType::AllowPremium,
            Self::AllowBots => PrivacyValueType::AllowBots,
            Self::AllowUsers(_) => PrivacyValueType::AllowUsers,
            Self::AllowChatMembers(_) => PrivacyValueType::AllowChatMembers,
            Self::DisallowAll => PrivacyValueType::DisallowAll,
            Self::DisallowContacts => PrivacyValueType::DisallowContacts,
            Self::DisallowBots => PrivacyValueType::DisallowBots,
            Self::DisallowUsers(_) => PrivacyValueType::DisallowContacts,
        }
    }
}

impl fmt::Display for PrivacyRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllowAll => write!(f, "AllowAll"),
            Self::AllowContacts => write!(f, "AllowContacts"),
            Self::AllowPremium => write!(f, "AllowPremium"),
            Self::AllowBots => write!(f, "AllowBots"),
            Self::AllowUsers(ids) => write!(f, "AllowUsers({} ids)", ids.len()),
            Self::AllowChatMembers(id) => write!(f, "AllowChatMembers({})", id),
            Self::DisallowAll => write!(f, "DisallowAll"),
            Self::DisallowContacts => write!(f, "DisallowContacts"),
            Self::DisallowBots => write!(f, "DisallowBots"),
            Self::DisallowUsers(ids) => write!(f, "DisallowUsers({} ids)", ids.len()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_key_values() {
        assert_eq!(PrivacyKey::Status as i32, 0);
        assert_eq!(PrivacyKey::Birthdate as i32, 6);
    }

    #[test]
    fn test_privacy_value_type_values() {
        assert_eq!(PrivacyValueType::AllowAll as i32, 0);
        assert_eq!(PrivacyValueType::DisallowBots as i32, 8);
    }

    #[test]
    fn test_allow_all() {
        assert!(PrivacyRule::AllowAll.is_all_allowed());
        assert!(!PrivacyRule::AllowContacts.is_all_allowed());
    }

    #[test]
    fn test_disallow_all() {
        assert!(PrivacyRule::DisallowAll.is_all_disallowed());
        assert!(!PrivacyRule::AllowAll.is_all_disallowed());
    }

    #[test]
    fn test_value_type() {
        assert_eq!(
            PrivacyRule::AllowAll.value_type(),
            PrivacyValueType::AllowAll
        );
        assert_eq!(
            PrivacyRule::AllowContacts.value_type(),
            PrivacyValueType::AllowContacts
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", PrivacyRule::AllowAll), "AllowAll");
        assert!(format!("{}", PrivacyRule::AllowUsers(vec![1, 2, 3])).contains("3 ids"));
    }
}
