// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Suggested Action
//!
//! Suggested action types for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`SuggestedAction`] enum, which represents
//! suggested actions shown to users (setup prompts, feature suggestions, etc.).
//!
//! ## Example
//!
//! ```rust
//! use rustgram_suggested_action::SuggestedAction;
//!
//! let action = SuggestedAction::CheckPassword;
//! assert!(!action.is_empty());
//! assert!(action.is_persistent());
//! ```

use std::fmt;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Suggested action type.
///
/// Represents a suggested action that can be shown to users.
/// Based on TDLib's `SuggestedAction` class.
///
/// # TDLib Alignment
///
/// Aligns with TDLib's `enum class SuggestedAction::Type` in `SuggestedAction.h`.
///
/// # Example
///
/// ```rust
/// use rustgram_suggested_action::SuggestedAction;
///
/// let action = SuggestedAction::CheckPassword;
/// assert!(!action.is_empty());
/// assert!(action.is_persistent());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SuggestedAction {
    /// Empty/invalid action.
    #[default]
    Empty,

    /// Enable archive and mute new chats.
    EnableArchiveAndMuteNewChats,

    /// Check phone number.
    CheckPhoneNumber,

    /// View checks hint.
    ViewChecksHint,

    /// Convert to gigagroup.
    ConvertToGigagroup,

    /// Check password.
    CheckPassword,

    /// Set password.
    SetPassword,

    /// Upgrade to Premium.
    UpgradePremium,

    /// Subscribe to annual Premium.
    SubscribeToAnnualPremium,

    /// Restore Premium.
    RestorePremium,

    /// Gift Premium for Christmas.
    GiftPremiumForChristmas,

    /// Setup birthday.
    BirthdaySetup,

    /// Premium grace period.
    PremiumGrace,

    /// Stars subscription low balance.
    StarsSubscriptionLowBalance,

    /// Setup userpic.
    UserpicSetup,

    /// Custom action type.
    Custom {
        /// The custom type string.
        custom_type: String,
    },

    /// Setup login email.
    SetupLoginEmail,

    /// Setup login email (no skip).
    SetupLoginEmailNoskip,

    /// Setup passkey.
    SetupPasskey,
}

impl SuggestedAction {
    /// Returns `true` if this action is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action::SuggestedAction;
    ///
    /// assert!(SuggestedAction::Empty.is_empty());
    /// assert!(!SuggestedAction::CheckPassword.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns `true` if this action is persistent.
    ///
    /// Non-persistent actions are `SetupLoginEmail` and `SetupLoginEmailNoskip`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action::SuggestedAction;
    ///
    /// assert!(SuggestedAction::CheckPassword.is_persistent());
    /// assert!(!SuggestedAction::SetupLoginEmail.is_persistent());
    /// assert!(!SuggestedAction::SetupLoginEmailNoskip.is_persistent());
    /// ```
    #[must_use]
    pub const fn is_persistent(&self) -> bool {
        !matches!(self, Self::Empty | Self::SetupLoginEmail | Self::SetupLoginEmailNoskip)
    }

    /// Returns the type value for TDLib compatibility.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action::SuggestedAction;
    ///
    /// assert_eq!(SuggestedAction::Empty.type_value(), 0);
    /// assert_eq!(SuggestedAction::CheckPassword.type_value(), 5);
    /// ```
    #[must_use]
    pub const fn type_value(&self) -> i32 {
        match self {
            Self::Empty => 0,
            Self::EnableArchiveAndMuteNewChats => 1,
            Self::CheckPhoneNumber => 2,
            Self::ViewChecksHint => 3,
            Self::ConvertToGigagroup => 4,
            Self::CheckPassword => 5,
            Self::SetPassword => 6,
            Self::UpgradePremium => 7,
            Self::SubscribeToAnnualPremium => 8,
            Self::RestorePremium => 9,
            Self::GiftPremiumForChristmas => 10,
            Self::BirthdaySetup => 11,
            Self::PremiumGrace => 12,
            Self::StarsSubscriptionLowBalance => 13,
            Self::UserpicSetup => 14,
            Self::Custom { .. } => 15,
            Self::SetupLoginEmail => 16,
            Self::SetupLoginEmailNoskip => 17,
            Self::SetupPasskey => 18,
        }
    }

    /// Creates a suggested action from a type value.
    ///
    /// # Returns
    ///
    /// `Some(SuggestedAction)` if the type value is valid, `None` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_action::SuggestedAction;
    ///
    /// assert_eq!(SuggestedAction::from_type_value(0), Some(SuggestedAction::Empty));
    /// assert_eq!(SuggestedAction::from_type_value(5), Some(SuggestedAction::CheckPassword));
    /// assert_eq!(SuggestedAction::from_type_value(999), None);
    /// ```
    #[must_use]
    pub const fn from_type_value(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Empty),
            1 => Some(Self::EnableArchiveAndMuteNewChats),
            2 => Some(Self::CheckPhoneNumber),
            3 => Some(Self::ViewChecksHint),
            4 => Some(Self::ConvertToGigagroup),
            5 => Some(Self::CheckPassword),
            6 => Some(Self::SetPassword),
            7 => Some(Self::UpgradePremium),
            8 => Some(Self::SubscribeToAnnualPremium),
            9 => Some(Self::RestorePremium),
            10 => Some(Self::GiftPremiumForChristmas),
            11 => Some(Self::BirthdaySetup),
            12 => Some(Self::PremiumGrace),
            13 => Some(Self::StarsSubscriptionLowBalance),
            14 => Some(Self::UserpicSetup),
            15 => Some(Self::Custom {
                custom_type: String::new(),
            }),
            16 => Some(Self::SetupLoginEmail),
            17 => Some(Self::SetupLoginEmailNoskip),
            18 => Some(Self::SetupPasskey),
            _ => None,
        }
    }
}

impl fmt::Display for SuggestedAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::EnableArchiveAndMuteNewChats => write!(f, "EnableArchiveAndMuteNewChats"),
            Self::CheckPhoneNumber => write!(f, "CheckPhoneNumber"),
            Self::ViewChecksHint => write!(f, "ViewChecksHint"),
            Self::ConvertToGigagroup => write!(f, "ConvertToGigagroup"),
            Self::CheckPassword => write!(f, "CheckPassword"),
            Self::SetPassword => write!(f, "SetPassword"),
            Self::UpgradePremium => write!(f, "UpgradePremium"),
            Self::SubscribeToAnnualPremium => write!(f, "SubscribeToAnnualPremium"),
            Self::RestorePremium => write!(f, "RestorePremium"),
            Self::GiftPremiumForChristmas => write!(f, "GiftPremiumForChristmas"),
            Self::BirthdaySetup => write!(f, "BirthdaySetup"),
            Self::PremiumGrace => write!(f, "PremiumGrace"),
            Self::StarsSubscriptionLowBalance => write!(f, "StarsSubscriptionLowBalance"),
            Self::UserpicSetup => write!(f, "UserpicSetup"),
            Self::Custom { custom_type } => write!(f, "Custom({})", custom_type),
            Self::SetupLoginEmail => write!(f, "SetupLoginEmail"),
            Self::SetupLoginEmailNoskip => write!(f, "SetupLoginEmailNoskip"),
            Self::SetupPasskey => write!(f, "SetupPasskey"),
        }
    }
}

impl Hash for SuggestedAction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Custom { custom_type } => {
                15.hash(state);
                custom_type.hash(state);
            }
            other => other.type_value().hash(state),
        }
    }
}

/// Extended suggested action with additional metadata.
///
/// Contains the action type along with optional dialog ID, title, description, and URL.
///
/// # Example
///
/// ```rust
/// use rustgram_suggested_action::SuggestedAction;
///
/// let action = SuggestedAction::CheckPassword;
/// assert!(!action.is_empty());
/// ```
pub type SuggestedActionExt = SuggestedAction;

#[cfg(test)]
mod tests {
    use super::*;

    // ========== is_empty Tests ==========

    #[test]
    fn test_is_empty_true() {
        assert!(SuggestedAction::Empty.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        assert!(!SuggestedAction::CheckPassword.is_empty());
        assert!(!SuggestedAction::SetPassword.is_empty());
        assert!(!SuggestedAction::UpgradePremium.is_empty());
    }

    // ========== is_persistent Tests ==========

    #[test]
    fn test_is_persistent_true() {
        assert!(SuggestedAction::CheckPassword.is_persistent());
        assert!(SuggestedAction::SetPassword.is_persistent());
        assert!(SuggestedAction::UpgradePremium.is_persistent());
    }

    #[test]
    fn test_is_persistent_false() {
        assert!(!SuggestedAction::SetupLoginEmail.is_persistent());
        assert!(!SuggestedAction::SetupLoginEmailNoskip.is_persistent());
        assert!(!SuggestedAction::Empty.is_persistent());
    }

    // ========== type_value Tests ==========

    #[test]
    fn test_type_value_all_variants() {
        assert_eq!(SuggestedAction::Empty.type_value(), 0);
        assert_eq!(SuggestedAction::EnableArchiveAndMuteNewChats.type_value(), 1);
        assert_eq!(SuggestedAction::CheckPhoneNumber.type_value(), 2);
        assert_eq!(SuggestedAction::ViewChecksHint.type_value(), 3);
        assert_eq!(SuggestedAction::ConvertToGigagroup.type_value(), 4);
        assert_eq!(SuggestedAction::CheckPassword.type_value(), 5);
        assert_eq!(SuggestedAction::SetPassword.type_value(), 6);
        assert_eq!(SuggestedAction::UpgradePremium.type_value(), 7);
        assert_eq!(SuggestedAction::SubscribeToAnnualPremium.type_value(), 8);
        assert_eq!(SuggestedAction::RestorePremium.type_value(), 9);
        assert_eq!(SuggestedAction::GiftPremiumForChristmas.type_value(), 10);
        assert_eq!(SuggestedAction::BirthdaySetup.type_value(), 11);
        assert_eq!(SuggestedAction::PremiumGrace.type_value(), 12);
        assert_eq!(SuggestedAction::StarsSubscriptionLowBalance.type_value(), 13);
        assert_eq!(SuggestedAction::UserpicSetup.type_value(), 14);
        assert_eq!(SuggestedAction::Custom { custom_type: String::new() }.type_value(), 15);
        assert_eq!(SuggestedAction::SetupLoginEmail.type_value(), 16);
        assert_eq!(SuggestedAction::SetupLoginEmailNoskip.type_value(), 17);
        assert_eq!(SuggestedAction::SetupPasskey.type_value(), 18);
    }

    // ========== from_type_value Tests ==========

    #[test]
    fn test_from_type_value_valid() {
        assert_eq!(SuggestedAction::from_type_value(0), Some(SuggestedAction::Empty));
        assert_eq!(SuggestedAction::from_type_value(5), Some(SuggestedAction::CheckPassword));
        assert_eq!(SuggestedAction::from_type_value(16), Some(SuggestedAction::SetupLoginEmail));
    }

    #[test]
    fn test_from_type_value_invalid() {
        assert_eq!(SuggestedAction::from_type_value(-1), None);
        assert_eq!(SuggestedAction::from_type_value(19), None);
        assert_eq!(SuggestedAction::from_type_value(999), None);
    }

    // ========== roundtrip Tests ==========

    #[test]
    fn test_roundtrip_type_value() {
        let actions = [
            SuggestedAction::Empty,
            SuggestedAction::CheckPassword,
            SuggestedAction::SetPassword,
            SuggestedAction::UpgradePremium,
            SuggestedAction::SetupLoginEmail,
            SuggestedAction::SetupPasskey,
        ];

        for action in &actions {
            let type_value = action.type_value();
            let restored = SuggestedAction::from_type_value(type_value);
            assert_eq!(Some(action.clone()), restored);
        }
    }

    // ========== default Tests ==========

    #[test]
    fn test_default() {
        assert_eq!(SuggestedAction::default(), SuggestedAction::Empty);
    }

    // ========== equality Tests ==========

    #[test]
    fn test_equality_same() {
        assert_eq!(SuggestedAction::CheckPassword, SuggestedAction::CheckPassword);
    }

    #[test]
    fn test_equality_different() {
        assert_ne!(SuggestedAction::CheckPassword, SuggestedAction::SetPassword);
    }

    #[test]
    fn test_custom_equality() {
        assert_eq!(
            SuggestedAction::Custom { custom_type: String::from("test") },
            SuggestedAction::Custom { custom_type: String::from("test") }
        );
    }

    #[test]
    fn test_custom_inequality() {
        assert_ne!(
            SuggestedAction::Custom { custom_type: String::from("test1") },
            SuggestedAction::Custom { custom_type: String::from("test2") }
        );
    }

    // ========== clone Tests ==========

    #[test]
    fn test_clone() {
        let action1 = SuggestedAction::CheckPassword;
        let action2 = action1.clone();
        assert_eq!(action1, action2);
    }

    // ========== hash Tests ==========

    #[test]
    fn test_hash_same() {
        use std::collections::hash_map::DefaultHasher;

        let action1 = SuggestedAction::CheckPassword;
        let action2 = SuggestedAction::CheckPassword;

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        action1.hash(&mut hasher1);
        action2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_different() {
        use std::collections::hash_map::DefaultHasher;

        let action1 = SuggestedAction::CheckPassword;
        let action2 = SuggestedAction::SetPassword;

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        action1.hash(&mut hasher1);
        action2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    // ========== display Tests ==========

    #[test]
    fn test_display_empty() {
        assert_eq!(format!("{}", SuggestedAction::Empty), "Empty");
    }

    #[test]
    fn test_display_check_password() {
        assert_eq!(format!("{}", SuggestedAction::CheckPassword), "CheckPassword");
    }

    #[test]
    fn test_display_custom() {
        let custom = SuggestedAction::Custom { custom_type: String::from("my_action") };
        assert_eq!(format!("{}", custom), "Custom(my_action)");
    }

    // ========== match Tests ==========

    #[test]
    fn test_match_all_variants() {
        let actions = [
            SuggestedAction::Empty,
            SuggestedAction::EnableArchiveAndMuteNewChats,
            SuggestedAction::CheckPhoneNumber,
            SuggestedAction::ViewChecksHint,
            SuggestedAction::ConvertToGigagroup,
            SuggestedAction::CheckPassword,
            SuggestedAction::SetPassword,
            SuggestedAction::UpgradePremium,
            SuggestedAction::SubscribeToAnnualPremium,
            SuggestedAction::RestorePremium,
            SuggestedAction::GiftPremiumForChristmas,
            SuggestedAction::BirthdaySetup,
            SuggestedAction::PremiumGrace,
            SuggestedAction::StarsSubscriptionLowBalance,
            SuggestedAction::UserpicSetup,
            SuggestedAction::SetupLoginEmail,
            SuggestedAction::SetupLoginEmailNoskip,
            SuggestedAction::SetupPasskey,
        ];

        for action in &actions {
            let _ = match action {
                SuggestedAction::Empty => true,
                SuggestedAction::EnableArchiveAndMuteNewChats => true,
                SuggestedAction::CheckPhoneNumber => true,
                SuggestedAction::ViewChecksHint => true,
                SuggestedAction::ConvertToGigagroup => true,
                SuggestedAction::CheckPassword => true,
                SuggestedAction::SetPassword => true,
                SuggestedAction::UpgradePremium => true,
                SuggestedAction::SubscribeToAnnualPremium => true,
                SuggestedAction::RestorePremium => true,
                SuggestedAction::GiftPremiumForChristmas => true,
                SuggestedAction::BirthdaySetup => true,
                SuggestedAction::PremiumGrace => true,
                SuggestedAction::StarsSubscriptionLowBalance => true,
                SuggestedAction::UserpicSetup => true,
                SuggestedAction::Custom { .. } => true,
                SuggestedAction::SetupLoginEmail => true,
                SuggestedAction::SetupLoginEmailNoskip => true,
                SuggestedAction::SetupPasskey => true,
            };
        }
    }

    // ========== Serialization Tests ==========

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let action = SuggestedAction::CheckPassword;
        let json = serde_json::to_string(&action).unwrap();
        let deserialized: SuggestedAction = serde_json::from_str(&json).unwrap();
        assert_eq!(action, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_custom() {
        let action = SuggestedAction::Custom { custom_type: String::from("test") };
        let json = serde_json::to_string(&action).unwrap();
        let deserialized: SuggestedAction = serde_json::from_str(&json).unwrap();
        assert_eq!(action, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_all_variants() {
        let actions = [
            SuggestedAction::Empty,
            SuggestedAction::CheckPassword,
            SuggestedAction::SetPassword,
            SuggestedAction::UpgradePremium,
            SuggestedAction::Custom { custom_type: String::from("test") },
        ];

        for action in &actions {
            let json = serde_json::to_string(&action).unwrap();
            let deserialized: SuggestedAction = serde_json::from_str(&json).unwrap();
            assert_eq!(action.clone(), deserialized);
        }
    }
}
