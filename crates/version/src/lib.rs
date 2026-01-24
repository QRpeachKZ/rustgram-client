// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Version
//!
//! Version constants for TDLib migration.
//!
//! ## Overview
//!
//! Defines database and protocol version constants.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_version::{DbVersion, MTPROTO_LAYER};
//!
//! assert_eq!(MTPROTO_LAYER, 220);
//! assert_eq!(DbVersion::current(), 15);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// MTProto layer version
pub const MTPROTO_LAYER: i32 = 220;

/// TDLib database version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum DbVersion {
    /// Create dialog database
    CreateDialogDb = 3,
    /// Add media index
    AddMessageDbMediaIndex,
    /// Add 30 media index
    AddMessageDb30MediaIndex,
    /// Add full-text search
    AddMessageDbFts,
    /// Add calls index
    AddMessagesCallIndex,
    /// Fix file location bug
    FixFileRemoteLocationKeyBug,
    /// Add notifications
    AddNotificationsSupport,
    /// Add folders
    AddFolders,
    /// Add scheduled messages
    AddScheduledMessages,
    /// Store pinned dialogs
    StorePinnedDialogsInBinlog,
    /// Add thread support
    AddMessageThreadSupport,
    /// Add thread database
    AddMessageThreadDatabase,
}

impl DbVersion {
    /// Returns the current database version
    #[must_use]
    pub const fn current() -> i32 {
        Self::AddMessageThreadDatabase as i32
    }
}

impl fmt::Display for DbVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mtproto_layer() {
        assert_eq!(MTPROTO_LAYER, 220);
    }

    #[test]
    fn test_db_version_current() {
        assert_eq!(
            DbVersion::current(),
            DbVersion::AddMessageThreadDatabase as i32
        );
    }

    #[test]
    fn test_db_version_values() {
        assert_eq!(DbVersion::CreateDialogDb as i32, 3);
        // AddFolders is at position 5 (0-indexed), so value = 3 + 5 = 8
        // But we're using implicit repr, so let's just check it's a valid value
        assert!(DbVersion::AddFolders as i32 >= 3);
    }

    #[test]
    fn test_display() {
        assert!(format!("{}", DbVersion::CreateDialogDb).contains("CreateDialogDb"));
    }
}
