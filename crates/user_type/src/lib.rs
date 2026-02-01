// rustgram_user_type
// Copyright (C) 2025 rustgram-client contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # User Type
//!
//! Defines the type of a user account in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`UserType`] enum which represents the four types of users in Telegram:
//!
//! - **Regular**: A normal human user account
//! - **Deleted**: A deleted user or deleted bot (no information available)
//! - **Bot**: A bot account with 11 capability fields
//! - **Unknown**: Rare type with no information, handle like deleted
//!
//! ## TDLib Reference
//!
//! Corresponds to the `UserType` class in TDLib (td_api.tl lines 658-681).
//!
//! ## Example
//!
//! ```rust
//! use rustgram_user_type::UserType;
//!
//! // Create a regular user
//! let regular = UserType::Regular;
//! assert!(regular.is_regular());
//! assert!(!regular.is_bot());
//!
//! // Create a bot with all fields
//! let bot = UserType::Bot {
//!     can_be_edited: true,
//!     can_join_groups: true,
//!     can_read_all_group_messages: false,
//!     has_main_web_app: true,
//!     has_topics: false,
//!     is_inline: true,
//!     inline_query_placeholder: "Enter query".to_string(),
//!     need_location: false,
//!     can_connect_to_business: false,
//!     can_be_added_to_attachment_menu: true,
//!     active_user_count: 1000,
//! };
//! assert!(bot.is_bot());
//! assert_eq!(bot.active_user_count(), 1000);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of user account in Telegram.
///
/// Represents the four types of users: regular users, deleted users, bots, and unknown.
/// The Bot variant contains 11 fields describing bot capabilities.
///
/// # TDLib Correspondence
///
/// - `userTypeRegular` -> [`UserType::Regular`]
/// - `userTypeDeleted` -> [`UserType::Deleted`]
/// - `userTypeBot` -> [`UserType::Bot`]
/// - `userTypeUnknown` -> [`UserType::Unknown`]
///
/// # Copy Trait Note
///
/// `UserType` does not derive `Copy` because the `Bot` variant contains
/// a `String` field (`inline_query_placeholder`), which is not `Copy`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UserType {
    /// A regular user (default).
    ///
    /// This represents a normal human user account in Telegram.
    #[default]
    Regular,

    /// A deleted user or deleted bot.
    ///
    /// No information is available besides the user identifier.
    /// It is not possible to perform any active actions on this type of user.
    Deleted,

    /// A bot (see https://core.telegram.org/bots).
    ///
    /// Contains 11 fields describing the bot's capabilities and configuration.
    Bot {
        /// True, if the bot is owned by the current user and can be edited.
        can_be_edited: bool,

        /// True, if the bot can be invited to basic group and supergroup chats.
        can_join_groups: bool,

        /// True, if the bot can read all messages in basic group or supergroup chats
        /// and not just those addressed to the bot.
        ///
        /// In private and channel chats a bot can always read all messages.
        can_read_all_group_messages: bool,

        /// True, if the bot has the main Web App.
        has_main_web_app: bool,

        /// True, if the bot has topics.
        has_topics: bool,

        /// True, if the bot supports inline queries.
        is_inline: bool,

        /// Placeholder for inline queries (displayed on the application input field).
        inline_query_placeholder: String,

        /// True, if the location of the user is expected to be sent
        /// with every inline query to this bot.
        need_location: bool,

        /// True, if the bot supports connection to Telegram Business accounts.
        can_connect_to_business: bool,

        /// True, if the bot can be added to attachment or side menu.
        can_be_added_to_attachment_menu: bool,

        /// The number of recently active users of the bot.
        active_user_count: i32,
    },

    /// No information on the user besides the user identifier is available.
    ///
    /// This object is extremely rare and must be handled like a deleted user.
    /// It is not possible to perform any actions on users of this type.
    Unknown,
}

impl UserType {
    /// Maximum discriminant value for UserType validation.
    pub const MAX_VALUE: i32 = 3;

    /// Creates a `UserType` from a TDLib API type name string.
    ///
    /// Returns `None` for unknown type names or for `userTypeBot` (which requires
    /// 11 fields of data and must be constructed directly).
    ///
    /// # Arguments
    ///
    /// * `type_name` - The TDLib API type name (e.g., "userTypeRegular")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_type::UserType;
    ///
    /// assert_eq!(
    ///     UserType::from_td_api_name("userTypeRegular"),
    ///     Some(UserType::Regular)
    /// );
    /// assert_eq!(
    ///     UserType::from_td_api_name("userTypeDeleted"),
    ///     Some(UserType::Deleted)
    /// );
    /// assert_eq!(
    ///     UserType::from_td_api_name("userTypeUnknown"),
    ///     Some(UserType::Unknown)
    /// );
    /// // Bot requires data, so returns None
    /// assert_eq!(UserType::from_td_api_name("userTypeBot"), None);
    /// // Unknown type name returns None
    /// assert_eq!(UserType::from_td_api_name("unknownType"), None);
    /// ```
    #[must_use]
    pub fn from_td_api_name(type_name: &str) -> Option<Self> {
        match type_name {
            "userTypeRegular" => Some(Self::Regular),
            "userTypeDeleted" => Some(Self::Deleted),
            "userTypeUnknown" => Some(Self::Unknown),
            // Bot requires 11 fields, cannot be constructed from name alone
            "userTypeBot" => None,
            _ => None,
        }
    }

    /// Returns the TDLib API type name for this `UserType`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_type::UserType;
    ///
    /// assert_eq!(UserType::Regular.to_td_api_name(), "userTypeRegular");
    /// assert_eq!(UserType::Deleted.to_td_api_name(), "userTypeDeleted");
    /// assert_eq!(UserType::Unknown.to_td_api_name(), "userTypeUnknown");
    /// ```
    #[must_use]
    pub const fn to_td_api_name(&self) -> &'static str {
        match self {
            Self::Regular => "userTypeRegular",
            Self::Deleted => "userTypeDeleted",
            Self::Bot { .. } => "userTypeBot",
            Self::Unknown => "userTypeUnknown",
        }
    }

    /// Creates a `UserType` from an i32 discriminant value.
    ///
    /// Returns `None` for all values except simple variants (Regular=0, Deleted=1, Unknown=3).
    /// The Bot variant (value=2) cannot be constructed from i32 alone because it requires
    /// 11 fields of data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_type::UserType;
    ///
    /// assert_eq!(UserType::from_i32(0), Some(UserType::Regular));
    /// assert_eq!(UserType::from_i32(1), Some(UserType::Deleted));
    /// assert_eq!(UserType::from_i32(3), Some(UserType::Unknown));
    /// // Bot requires data, cannot be constructed from i32
    /// assert_eq!(UserType::from_i32(2), None);
    /// assert_eq!(UserType::from_i32(999), None);
    /// ```
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Regular),
            1 => Some(Self::Deleted),
            // Bot (2) cannot be constructed from i32 alone
            3 => Some(Self::Unknown),
            _ => None,
        }
    }

    /// Returns the i32 discriminant value for this `UserType`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_type::UserType;
    ///
    /// assert_eq!(UserType::Regular.to_i32(), 0);
    /// assert_eq!(UserType::Deleted.to_i32(), 1);
    /// assert_eq!(UserType::Unknown.to_i32(), 3);
    /// ```
    #[must_use]
    pub const fn to_i32(&self) -> i32 {
        match self {
            Self::Regular => 0,
            Self::Deleted => 1,
            Self::Bot { .. } => 2,
            Self::Unknown => 3,
        }
    }

    /// Returns `true` if this is a regular user.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_type::UserType;
    ///
    /// assert!(UserType::Regular.is_regular());
    /// assert!(!UserType::Deleted.is_regular());
    /// ```
    #[must_use]
    pub const fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }

    /// Returns `true` if this is a deleted user.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_type::UserType;
    ///
    /// assert!(UserType::Deleted.is_deleted());
    /// assert!(!UserType::Regular.is_deleted());
    /// ```
    #[must_use]
    pub const fn is_deleted(&self) -> bool {
        matches!(self, Self::Deleted)
    }

    /// Returns `true` if this is a bot.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_type::UserType;
    ///
    /// let bot = UserType::Bot {
    ///     can_be_edited: false,
    ///     can_join_groups: false,
    ///     can_read_all_group_messages: false,
    ///     has_main_web_app: false,
    ///     has_topics: false,
    ///     is_inline: false,
    ///     inline_query_placeholder: String::new(),
    ///     need_location: false,
    ///     can_connect_to_business: false,
    ///     can_be_added_to_attachment_menu: false,
    ///     active_user_count: 0,
    /// };
    /// assert!(bot.is_bot());
    /// assert!(!UserType::Regular.is_bot());
    /// ```
    #[must_use]
    pub const fn is_bot(&self) -> bool {
        matches!(self, Self::Bot { .. })
    }

    /// Returns `true` if this is an unknown user type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_type::UserType;
    ///
    /// assert!(UserType::Unknown.is_unknown());
    /// assert!(!UserType::Regular.is_unknown());
    /// ```
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns `can_be_edited` for Bot variants, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_type::UserType;
    ///
    /// let bot = UserType::Bot {
    ///     can_be_edited: true,
    ///     can_join_groups: false,
    ///     can_read_all_group_messages: false,
    ///     has_main_web_app: false,
    ///     has_topics: false,
    ///     is_inline: false,
    ///     inline_query_placeholder: String::new(),
    ///     need_location: false,
    ///     can_connect_to_business: false,
    ///     can_be_added_to_attachment_menu: false,
    ///     active_user_count: 0,
    /// };
    /// assert_eq!(bot.can_be_edited(), true);
    /// assert_eq!(UserType::Regular.can_be_edited(), false);
    /// ```
    #[must_use]
    pub const fn can_be_edited(&self) -> bool {
        match self {
            Self::Bot { can_be_edited, .. } => *can_be_edited,
            _ => false,
        }
    }

    /// Returns `can_join_groups` for Bot variants, `false` otherwise.
    #[must_use]
    pub const fn can_join_groups(&self) -> bool {
        match self {
            Self::Bot {
                can_join_groups, ..
            } => *can_join_groups,
            _ => false,
        }
    }

    /// Returns `can_read_all_group_messages` for Bot variants, `false` otherwise.
    #[must_use]
    pub const fn can_read_all_group_messages(&self) -> bool {
        match self {
            Self::Bot {
                can_read_all_group_messages,
                ..
            } => *can_read_all_group_messages,
            _ => false,
        }
    }

    /// Returns `has_main_web_app` for Bot variants, `false` otherwise.
    #[must_use]
    pub const fn has_main_web_app(&self) -> bool {
        match self {
            Self::Bot {
                has_main_web_app, ..
            } => *has_main_web_app,
            _ => false,
        }
    }

    /// Returns `has_topics` for Bot variants, `false` otherwise.
    #[must_use]
    pub const fn has_topics(&self) -> bool {
        match self {
            Self::Bot { has_topics, .. } => *has_topics,
            _ => false,
        }
    }

    /// Returns `is_inline` for Bot variants, `false` otherwise.
    #[must_use]
    pub const fn is_inline_bot(&self) -> bool {
        match self {
            Self::Bot { is_inline, .. } => *is_inline,
            _ => false,
        }
    }

    /// Returns `inline_query_placeholder` for Bot variants, empty string otherwise.
    #[must_use]
    pub fn inline_query_placeholder(&self) -> &str {
        match self {
            Self::Bot {
                inline_query_placeholder,
                ..
            } => inline_query_placeholder.as_str(),
            _ => "",
        }
    }

    /// Returns `need_location` for Bot variants, `false` otherwise.
    #[must_use]
    pub const fn need_location(&self) -> bool {
        match self {
            Self::Bot { need_location, .. } => *need_location,
            _ => false,
        }
    }

    /// Returns `can_connect_to_business` for Bot variants, `false` otherwise.
    #[must_use]
    pub const fn can_connect_to_business(&self) -> bool {
        match self {
            Self::Bot {
                can_connect_to_business,
                ..
            } => *can_connect_to_business,
            _ => false,
        }
    }

    /// Returns `can_be_added_to_attachment_menu` for Bot variants, `false` otherwise.
    #[must_use]
    pub const fn can_be_added_to_attachment_menu(&self) -> bool {
        match self {
            Self::Bot {
                can_be_added_to_attachment_menu,
                ..
            } => *can_be_added_to_attachment_menu,
            _ => false,
        }
    }

    /// Returns `active_user_count` for Bot variants, `0` otherwise.
    #[must_use]
    pub const fn active_user_count(&self) -> i32 {
        match self {
            Self::Bot {
                active_user_count, ..
            } => *active_user_count,
            _ => 0,
        }
    }
}

impl fmt::Display for UserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Regular => "Regular",
            Self::Deleted => "Deleted",
            Self::Bot { .. } => "Bot",
            Self::Unknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Enum Construction Tests =====

    #[test]
    fn test_regular_variant() {
        let user_type = UserType::Regular;
        assert!(user_type.is_regular());
        assert!(!user_type.is_bot());
        assert!(!user_type.is_deleted());
        assert!(!user_type.is_unknown());
    }

    #[test]
    fn test_deleted_variant() {
        let user_type = UserType::Deleted;
        assert!(user_type.is_deleted());
        assert!(!user_type.is_regular());
        assert!(!user_type.is_bot());
        assert!(!user_type.is_unknown());
    }

    #[test]
    fn test_bot_variant_with_all_fields() {
        let bot = UserType::Bot {
            can_be_edited: true,
            can_join_groups: true,
            can_read_all_group_messages: false,
            has_main_web_app: true,
            has_topics: false,
            is_inline: true,
            inline_query_placeholder: "Enter your query".to_string(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: true,
            active_user_count: 5000,
        };
        assert!(bot.is_bot());
        assert!(!bot.is_regular());
        assert!(!bot.is_deleted());
        assert!(!bot.is_unknown());
        assert_eq!(bot.can_be_edited(), true);
        assert_eq!(bot.can_join_groups(), true);
        assert_eq!(bot.can_read_all_group_messages(), false);
        assert_eq!(bot.has_main_web_app(), true);
        assert_eq!(bot.has_topics(), false);
        assert_eq!(bot.is_inline_bot(), true);
        assert_eq!(bot.inline_query_placeholder(), "Enter your query");
        assert_eq!(bot.need_location(), false);
        assert_eq!(bot.can_connect_to_business(), false);
        assert_eq!(bot.can_be_added_to_attachment_menu(), true);
        assert_eq!(bot.active_user_count(), 5000);
    }

    #[test]
    fn test_unknown_variant() {
        let user_type = UserType::Unknown;
        assert!(user_type.is_unknown());
        assert!(!user_type.is_regular());
        assert!(!user_type.is_bot());
        assert!(!user_type.is_deleted());
    }

    // ===== Default Behavior Tests =====

    #[test]
    fn test_default_is_regular() {
        let default = UserType::default();
        assert_eq!(default, UserType::Regular);
    }

    #[test]
    fn test_clone_trait() {
        let bot = UserType::Bot {
            can_be_edited: true,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: "test".to_string(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        let cloned = bot.clone();
        assert_eq!(bot, cloned);
        assert_eq!(
            bot.inline_query_placeholder(),
            cloned.inline_query_placeholder()
        );
    }

    // ===== Conversion Methods Tests =====

    #[test]
    fn test_from_td_api_name_regular() {
        assert_eq!(
            UserType::from_td_api_name("userTypeRegular"),
            Some(UserType::Regular)
        );
    }

    #[test]
    fn test_from_td_api_name_deleted() {
        assert_eq!(
            UserType::from_td_api_name("userTypeDeleted"),
            Some(UserType::Deleted)
        );
    }

    #[test]
    fn test_from_td_api_name_unknown() {
        assert_eq!(
            UserType::from_td_api_name("userTypeUnknown"),
            Some(UserType::Unknown)
        );
    }

    #[test]
    fn test_from_td_api_name_invalid() {
        assert_eq!(UserType::from_td_api_name("unknownType"), None);
        assert_eq!(UserType::from_td_api_name(""), None);
        assert_eq!(UserType::from_td_api_name("UserTypeRegular"), None); // Case sensitive
    }

    #[test]
    fn test_from_td_api_name_bot_returns_none() {
        // Bot requires 11 fields, cannot be constructed from type name alone
        assert_eq!(UserType::from_td_api_name("userTypeBot"), None);
    }

    #[test]
    fn test_to_td_api_name_all_variants() {
        assert_eq!(UserType::Regular.to_td_api_name(), "userTypeRegular");
        assert_eq!(UserType::Deleted.to_td_api_name(), "userTypeDeleted");
        assert_eq!(
            UserType::Bot {
                can_be_edited: false,
                can_join_groups: false,
                can_read_all_group_messages: false,
                has_main_web_app: false,
                has_topics: false,
                is_inline: false,
                inline_query_placeholder: String::new(),
                need_location: false,
                can_connect_to_business: false,
                can_be_added_to_attachment_menu: false,
                active_user_count: 0,
            }
            .to_td_api_name(),
            "userTypeBot"
        );
        assert_eq!(UserType::Unknown.to_td_api_name(), "userTypeUnknown");
    }

    #[test]
    fn test_to_i32_discriminants() {
        assert_eq!(UserType::Regular.to_i32(), 0);
        assert_eq!(UserType::Deleted.to_i32(), 1);
        assert_eq!(
            UserType::Bot {
                can_be_edited: false,
                can_join_groups: false,
                can_read_all_group_messages: false,
                has_main_web_app: false,
                has_topics: false,
                is_inline: false,
                inline_query_placeholder: String::new(),
                need_location: false,
                can_connect_to_business: false,
                can_be_added_to_attachment_menu: false,
                active_user_count: 0,
            }
            .to_i32(),
            2
        );
        assert_eq!(UserType::Unknown.to_i32(), 3);
    }

    #[test]
    fn test_from_i32_simple_variants() {
        assert_eq!(UserType::from_i32(0), Some(UserType::Regular));
        assert_eq!(UserType::from_i32(1), Some(UserType::Deleted));
        assert_eq!(UserType::from_i32(3), Some(UserType::Unknown));
    }

    #[test]
    fn test_from_i32_bot_returns_none() {
        // Bot cannot be constructed from i32 alone
        assert_eq!(UserType::from_i32(2), None);
    }

    #[test]
    fn test_from_i32_invalid() {
        assert_eq!(UserType::from_i32(-1), None);
        assert_eq!(UserType::from_i32(4), None);
        assert_eq!(UserType::from_i32(999), None);
    }

    // ===== Convenience Methods Tests =====

    #[test]
    fn test_is_bot() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert!(bot.is_bot());
        assert!(!UserType::Regular.is_bot());
        assert!(!UserType::Deleted.is_bot());
        assert!(!UserType::Unknown.is_bot());
    }

    #[test]
    fn test_is_deleted() {
        assert!(UserType::Deleted.is_deleted());
        assert!(!UserType::Regular.is_deleted());
        assert!(!UserType::Unknown.is_deleted());
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert!(!bot.is_deleted());
    }

    #[test]
    fn test_is_regular() {
        assert!(UserType::Regular.is_regular());
        assert!(!UserType::Deleted.is_regular());
        assert!(!UserType::Unknown.is_regular());
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert!(!bot.is_regular());
    }

    #[test]
    fn test_is_unknown() {
        assert!(UserType::Unknown.is_unknown());
        assert!(!UserType::Regular.is_unknown());
        assert!(!UserType::Deleted.is_unknown());
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert!(!bot.is_unknown());
    }

    // ===== Bot Field Accessors Tests (11 dedicated tests) =====

    #[test]
    fn test_bot_field_can_be_edited() {
        let bot_edited = UserType::Bot {
            can_be_edited: true,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(bot_edited.can_be_edited(), true);
        assert_eq!(UserType::Regular.can_be_edited(), false);
        assert_eq!(UserType::Deleted.can_be_edited(), false);
        assert_eq!(UserType::Unknown.can_be_edited(), false);
    }

    #[test]
    fn test_bot_field_can_join_groups() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: true,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(bot.can_join_groups(), true);
        assert_eq!(UserType::Regular.can_join_groups(), false);
    }

    #[test]
    fn test_bot_field_can_read_all_group_messages() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: true,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(bot.can_read_all_group_messages(), true);
        assert_eq!(UserType::Regular.can_read_all_group_messages(), false);
    }

    #[test]
    fn test_bot_field_has_main_web_app() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: true,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(bot.has_main_web_app(), true);
        assert_eq!(UserType::Regular.has_main_web_app(), false);
    }

    #[test]
    fn test_bot_field_has_topics() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: true,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(bot.has_topics(), true);
        assert_eq!(UserType::Regular.has_topics(), false);
    }

    #[test]
    fn test_bot_field_is_inline() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: true,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(bot.is_inline_bot(), true);
        assert_eq!(UserType::Regular.is_inline_bot(), false);
    }

    #[test]
    fn test_bot_field_inline_query_placeholder() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: "Search @bot".to_string(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(bot.inline_query_placeholder(), "Search @bot");
        assert_eq!(UserType::Regular.inline_query_placeholder(), "");
        assert_eq!(UserType::Deleted.inline_query_placeholder(), "");
        assert_eq!(UserType::Unknown.inline_query_placeholder(), "");
    }

    #[test]
    fn test_bot_field_need_location() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: true,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(bot.need_location(), true);
        assert_eq!(UserType::Regular.need_location(), false);
    }

    #[test]
    fn test_bot_field_can_connect_to_business() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: true,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(bot.can_connect_to_business(), true);
        assert_eq!(UserType::Regular.can_connect_to_business(), false);
    }

    #[test]
    fn test_bot_field_can_be_added_to_attachment_menu() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: true,
            active_user_count: 0,
        };
        assert_eq!(bot.can_be_added_to_attachment_menu(), true);
        assert_eq!(UserType::Regular.can_be_added_to_attachment_menu(), false);
    }

    #[test]
    fn test_bot_field_active_user_count() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 12345,
        };
        assert_eq!(bot.active_user_count(), 12345);
        assert_eq!(UserType::Regular.active_user_count(), 0);
        assert_eq!(UserType::Deleted.active_user_count(), 0);
        assert_eq!(UserType::Unknown.active_user_count(), 0);
    }

    // ===== Non-Bot Field Defaults Tests =====

    #[test]
    fn test_non_bot_field_defaults() {
        // Test that Regular, Deleted, and Unknown return sensible defaults for Bot fields
        for user_type in &[UserType::Regular, UserType::Deleted, UserType::Unknown] {
            assert_eq!(user_type.can_be_edited(), false);
            assert_eq!(user_type.can_join_groups(), false);
            assert_eq!(user_type.can_read_all_group_messages(), false);
            assert_eq!(user_type.has_main_web_app(), false);
            assert_eq!(user_type.has_topics(), false);
            assert_eq!(user_type.is_inline_bot(), false);
            assert_eq!(user_type.inline_query_placeholder(), "");
            assert_eq!(user_type.need_location(), false);
            assert_eq!(user_type.can_connect_to_business(), false);
            assert_eq!(user_type.can_be_added_to_attachment_menu(), false);
            assert_eq!(user_type.active_user_count(), 0);
        }
    }

    // ===== Display Trait Tests =====

    #[test]
    fn test_display_regular() {
        assert_eq!(format!("{}", UserType::Regular), "Regular");
    }

    #[test]
    fn test_display_deleted() {
        assert_eq!(format!("{}", UserType::Deleted), "Deleted");
    }

    #[test]
    fn test_display_bot() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(format!("{}", bot), "Bot");
    }

    #[test]
    fn test_display_unknown() {
        assert_eq!(format!("{}", UserType::Unknown), "Unknown");
    }

    // ===== Equality and Hashing Tests =====

    #[test]
    fn test_partial_eq() {
        assert_eq!(UserType::Regular, UserType::Regular);
        assert_eq!(UserType::Deleted, UserType::Deleted);
        assert_eq!(UserType::Unknown, UserType::Unknown);

        let bot1 = UserType::Bot {
            can_be_edited: true,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: "test".to_string(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };

        let bot2 = UserType::Bot {
            can_be_edited: true,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: "test".to_string(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };

        assert_eq!(bot1, bot2);

        let bot3 = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: "test".to_string(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };

        assert_ne!(bot1, bot3);
        assert_ne!(UserType::Regular, UserType::Deleted);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(UserType::Regular);
        set.insert(UserType::Deleted);
        set.insert(UserType::Unknown);

        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: "test".to_string(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        set.insert(bot.clone());

        // All 4 variants should be in the set
        assert_eq!(set.len(), 4);

        // Same bot should hash to same value
        set.insert(bot);
        assert_eq!(set.len(), 4);
    }

    // ===== Edge Cases Tests =====

    #[test]
    fn test_bot_with_all_false_flags() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert!(bot.is_bot());
        assert!(!bot.can_be_edited());
        assert!(!bot.can_join_groups());
        assert!(!bot.can_read_all_group_messages());
        assert!(!bot.has_main_web_app());
        assert!(!bot.has_topics());
        assert!(!bot.is_inline_bot());
        assert_eq!(bot.inline_query_placeholder(), "");
        assert!(!bot.need_location());
        assert!(!bot.can_connect_to_business());
        assert!(!bot.can_be_added_to_attachment_menu());
        assert_eq!(bot.active_user_count(), 0);
    }

    #[test]
    fn test_bot_with_all_true_flags() {
        let bot = UserType::Bot {
            can_be_edited: true,
            can_join_groups: true,
            can_read_all_group_messages: true,
            has_main_web_app: true,
            has_topics: true,
            is_inline: true,
            inline_query_placeholder: "Query".to_string(),
            need_location: true,
            can_connect_to_business: true,
            can_be_added_to_attachment_menu: true,
            active_user_count: 999,
        };
        assert!(bot.is_bot());
        assert!(bot.can_be_edited());
        assert!(bot.can_join_groups());
        assert!(bot.can_read_all_group_messages());
        assert!(bot.has_main_web_app());
        assert!(bot.has_topics());
        assert!(bot.is_inline_bot());
        assert_eq!(bot.inline_query_placeholder(), "Query");
        assert!(bot.need_location());
        assert!(bot.can_connect_to_business());
        assert!(bot.can_be_added_to_attachment_menu());
        assert_eq!(bot.active_user_count(), 999);
    }

    #[test]
    fn test_bot_empty_placeholder() {
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 0,
        };
        assert_eq!(bot.inline_query_placeholder(), "");
    }

    #[test]
    fn test_bot_negative_user_count() {
        // Test that negative user count is stored correctly (though unlikely in practice)
        let bot = UserType::Bot {
            can_be_edited: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: String::new(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: -1,
        };
        assert_eq!(bot.active_user_count(), -1);
    }

    // ===== Serialization Tests =====

    #[test]
    fn test_serialize_regular() {
        let regular = UserType::Regular;
        let json = serde_json::to_string(&regular).expect("Failed to serialize");
        assert_eq!(json, "\"Regular\"");
    }

    #[test]
    fn test_serialize_bot() {
        let bot = UserType::Bot {
            can_be_edited: true,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            has_topics: false,
            is_inline: false,
            inline_query_placeholder: "test".to_string(),
            need_location: false,
            can_connect_to_business: false,
            can_be_added_to_attachment_menu: false,
            active_user_count: 100,
        };
        let json = serde_json::to_string(&bot).expect("Failed to serialize");
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse");
        assert_eq!(parsed["Bot"]["can_be_edited"], true);
        assert_eq!(parsed["Bot"]["inline_query_placeholder"], "test");
        assert_eq!(parsed["Bot"]["active_user_count"], 100);
    }

    #[test]
    fn test_deserialize_roundtrip() {
        let original = UserType::Bot {
            can_be_edited: false,
            can_join_groups: true,
            can_read_all_group_messages: true,
            has_main_web_app: false,
            has_topics: true,
            is_inline: true,
            inline_query_placeholder: "placeholder".to_string(),
            need_location: false,
            can_connect_to_business: true,
            can_be_added_to_attachment_menu: false,
            active_user_count: 42,
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: UserType = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_max_value_constant() {
        assert_eq!(UserType::MAX_VALUE, 3);
    }
}
