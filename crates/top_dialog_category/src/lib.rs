// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Top Dialog Category
//!
//! Categories for top dialogs in Telegram.
//!
//! ## Overview
//!
//! Defines categories used to organize top/recent dialogs.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_top_dialog_category::TopDialogCategory;
//!
//! let category = TopDialogCategory::Channel;
//! assert_eq!(category.name(), "channel");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Category for top dialogs
///
/// Used to categorize top/recent dialogs in Telegram.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum TopDialogCategory {
    /// Regular users (PM)
    Correspondent = 0,
    /// Bot PMs
    BotPm = 1,
    /// Bot inline queries
    BotInline = 2,
    /// Groups
    Group = 3,
    /// Channels
    Channel = 4,
    /// Calls
    Call = 5,
    /// Forwarded users
    ForwardUsers = 6,
    /// Forwarded chats
    ForwardChats = 7,
    /// Bot apps
    BotApp = 8,
}

impl TopDialogCategory {
    /// Total number of categories
    pub const SIZE: i32 = 9;

    /// Returns the name of the category
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Correspondent => "users",
            Self::BotPm => "bot_pm",
            Self::BotInline => "bot_inline",
            Self::Group => "groups",
            Self::Channel => "channels",
            Self::Call => "calls",
            Self::ForwardUsers => "forward_users",
            Self::ForwardChats => "forward_chats",
            Self::BotApp => "bot_apps",
        }
    }

    /// Creates a TopDialogCategory from an i32 value
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Correspondent),
            1 => Some(Self::BotPm),
            2 => Some(Self::BotInline),
            3 => Some(Self::Group),
            4 => Some(Self::Channel),
            5 => Some(Self::Call),
            6 => Some(Self::ForwardUsers),
            7 => Some(Self::ForwardChats),
            8 => Some(Self::BotApp),
            _ => None,
        }
    }

    /// Returns the i32 value of the category
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }
}

impl Default for TopDialogCategory {
    fn default() -> Self {
        Self::Correspondent
    }
}

impl fmt::Display for TopDialogCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(TopDialogCategory::Channel.name(), "channels");
        assert_eq!(TopDialogCategory::Group.name(), "groups");
        assert_eq!(TopDialogCategory::Call.name(), "calls");
    }

    #[test]
    fn test_from_i32_valid() {
        assert_eq!(
            TopDialogCategory::from_i32(0),
            Some(TopDialogCategory::Correspondent)
        );
        assert_eq!(
            TopDialogCategory::from_i32(4),
            Some(TopDialogCategory::Channel)
        );
        assert_eq!(
            TopDialogCategory::from_i32(8),
            Some(TopDialogCategory::BotApp)
        );
    }

    #[test]
    fn test_from_i32_invalid() {
        assert_eq!(TopDialogCategory::from_i32(-1), None);
        assert_eq!(TopDialogCategory::from_i32(9), None);
        assert_eq!(TopDialogCategory::from_i32(100), None);
    }

    #[test]
    fn test_as_i32() {
        assert_eq!(TopDialogCategory::Correspondent.as_i32(), 0);
        assert_eq!(TopDialogCategory::Channel.as_i32(), 4);
        assert_eq!(TopDialogCategory::BotApp.as_i32(), 8);
    }

    #[test]
    fn test_default() {
        assert_eq!(
            TopDialogCategory::default(),
            TopDialogCategory::Correspondent
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", TopDialogCategory::Channel), "channels");
    }

    #[test]
    fn test_size() {
        assert_eq!(TopDialogCategory::SIZE, 9);
    }

    #[test]
    fn test_equality() {
        assert_eq!(TopDialogCategory::Channel, TopDialogCategory::Channel);
        assert_ne!(TopDialogCategory::Channel, TopDialogCategory::Group);
    }
}
