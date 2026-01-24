// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Target Dialog Types
//!
//! Target dialog type filters for Telegram inline queries and bot commands.
//!
//! ## Overview
//!
//! This module provides the [`TargetDialogTypes`] type, which represents
//! a filter for which types of chats a bot command or inline query can be used in.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_target_dialog_types::TargetDialogTypes;
//!
//! // Create a filter for users and bots only
//! let types = TargetDialogTypes::users() | TargetDialogTypes::bots();
//! assert!(types.has_users());
//! assert!(types.has_bots());
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Bitmask flags for dialog types.
const USERS_MASK: i64 = 1;
const BOTS_MASK: i64 = 2;
const CHATS_MASK: i64 = 4;
const BROADCASTS_MASK: i64 = 8;
const FULL_MASK: i64 = USERS_MASK | BOTS_MASK | CHATS_MASK | BROADCASTS_MASK;

/// Target dialog type filter.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib's `TargetDialogTypes` class.
///
/// # Example
///
/// ```rust
/// use rustgram_target_dialog_types::TargetDialogTypes;
///
/// let types = TargetDialogTypes::all();
/// assert!(types.has_users());
/// assert!(types.has_bots());
/// assert!(types.has_groups());
/// assert!(types.has_channels());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TargetDialogTypes {
    mask: i64,
}

impl TargetDialogTypes {
    /// Creates a new target dialog types from a bitmask.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// let types = TargetDialogTypes::from_mask(1 | 2);
    /// assert!(types.has_users());
    /// assert!(types.has_bots());
    /// ```
    #[must_use]
    pub const fn from_mask(mask: i64) -> Self {
        Self { mask }
    }

    /// Creates an empty filter (no dialog types).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// let types = TargetDialogTypes::none();
    /// assert_eq!(types.mask(), 0);
    /// ```
    #[must_use]
    pub const fn none() -> Self {
        Self { mask: 0 }
    }

    /// Creates a filter for private chats with users.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// let types = TargetDialogTypes::users();
    /// assert!(types.has_users());
    /// assert!(!types.has_bots());
    /// ```
    #[must_use]
    pub const fn users() -> Self {
        Self { mask: USERS_MASK }
    }

    /// Creates a filter for private chats with bots.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// let types = TargetDialogTypes::bots();
    /// assert!(types.has_bots());
    /// assert!(!types.has_users());
    /// ```
    #[must_use]
    pub const fn bots() -> Self {
        Self { mask: BOTS_MASK }
    }

    /// Creates a filter for group chats and supergroups.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// let types = TargetDialogTypes::groups();
    /// assert!(types.has_groups());
    /// assert!(!types.has_channels());
    /// ```
    #[must_use]
    pub const fn groups() -> Self {
        Self { mask: CHATS_MASK }
    }

    /// Creates a filter for channels (broadcasts).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// let types = TargetDialogTypes::channels();
    /// assert!(types.has_channels());
    /// assert!(!types.has_groups());
    /// ```
    #[must_use]
    pub const fn channels() -> Self {
        Self {
            mask: BROADCASTS_MASK,
        }
    }

    /// Creates a filter for all dialog types.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// let types = TargetDialogTypes::all();
    /// assert!(types.has_users());
    /// assert!(types.has_bots());
    /// assert!(types.has_groups());
    /// assert!(types.has_channels());
    /// ```
    #[must_use]
    pub const fn all() -> Self {
        Self { mask: FULL_MASK }
    }

    /// Returns the raw bitmask.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// let types = TargetDialogTypes::users() | TargetDialogTypes::bots();
    /// assert_eq!(types.mask(), 3);
    /// ```
    #[must_use]
    pub const fn mask(&self) -> i64 {
        self.mask
    }

    /// Returns the full mask (defaults to all types if mask is 0).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// let none = TargetDialogTypes::none();
    /// assert!(none.full_mask() != 0); // full_mask returns 15 when mask is 0
    ///
    /// let users = TargetDialogTypes::users();
    /// assert_eq!(users.full_mask(), 1); // users only
    /// ```
    #[must_use]
    pub const fn full_mask(&self) -> i64 {
        if self.mask == 0 {
            FULL_MASK
        } else {
            self.mask
        }
    }

    /// Checks if users are included.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// assert!(TargetDialogTypes::users().has_users());
    /// assert!(!TargetDialogTypes::bots().has_users());
    /// ```
    #[must_use]
    pub const fn has_users(&self) -> bool {
        (self.mask & USERS_MASK) != 0
    }

    /// Checks if bots are included.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// assert!(TargetDialogTypes::bots().has_bots());
    /// assert!(!TargetDialogTypes::users().has_bots());
    /// ```
    #[must_use]
    pub const fn has_bots(&self) -> bool {
        (self.mask & BOTS_MASK) != 0
    }

    /// Checks if groups are included.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// assert!(TargetDialogTypes::groups().has_groups());
    /// assert!(!TargetDialogTypes::channels().has_groups());
    /// ```
    #[must_use]
    pub const fn has_groups(&self) -> bool {
        (self.mask & CHATS_MASK) != 0
    }

    /// Checks if channels are included.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// assert!(TargetDialogTypes::channels().has_channels());
    /// assert!(!TargetDialogTypes::groups().has_channels());
    /// ```
    #[must_use]
    pub const fn has_channels(&self) -> bool {
        (self.mask & BROADCASTS_MASK) != 0
    }

    /// Checks if this is empty (no dialog types selected).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// assert!(TargetDialogTypes::none().is_empty());
    /// assert!(!TargetDialogTypes::users().is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.mask == 0
    }
}

impl std::ops::BitOr for TargetDialogTypes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            mask: self.mask | rhs.mask,
        }
    }
}

impl std::ops::BitAnd for TargetDialogTypes {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            mask: self.mask & rhs.mask,
        }
    }
}

impl std::ops::BitOrAssign for TargetDialogTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.mask |= rhs.mask;
    }
}

impl std::ops::BitAndAssign for TargetDialogTypes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.mask &= rhs.mask;
    }
}

impl fmt::Display for TargetDialogTypes {
    /// Formats the target dialog types for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_target_dialog_types::TargetDialogTypes;
    ///
    /// let types = TargetDialogTypes::users() | TargetDialogTypes::groups();
    /// let s = format!("{}", types);
    /// assert!(s.contains("(users)") || s.contains("(groups)"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mask = self.full_mask();
        let mut parts = Vec::new();

        if (mask & USERS_MASK) != 0 {
            parts.push("(users)");
        }
        if (mask & BOTS_MASK) != 0 {
            parts.push("(bots)");
        }
        if (mask & CHATS_MASK) != 0 {
            parts.push("(groups)");
        }
        if (mask & BROADCASTS_MASK) != 0 {
            parts.push("(channels)");
        }

        write!(f, "{}", parts.join(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let types = TargetDialogTypes::none();
        assert!(types.is_empty());
        assert!(!types.has_users());
        assert!(!types.has_bots());
        assert!(!types.has_groups());
        assert!(!types.has_channels());
    }

    #[test]
    fn test_users() {
        let types = TargetDialogTypes::users();
        assert!(!types.is_empty());
        assert!(types.has_users());
        assert!(!types.has_bots());
        assert!(!types.has_groups());
        assert!(!types.has_channels());
    }

    #[test]
    fn test_bots() {
        let types = TargetDialogTypes::bots();
        assert!(!types.is_empty());
        assert!(!types.has_users());
        assert!(types.has_bots());
        assert!(!types.has_groups());
        assert!(!types.has_channels());
    }

    #[test]
    fn test_groups() {
        let types = TargetDialogTypes::groups();
        assert!(!types.is_empty());
        assert!(!types.has_users());
        assert!(!types.has_bots());
        assert!(types.has_groups());
        assert!(!types.has_channels());
    }

    #[test]
    fn test_channels() {
        let types = TargetDialogTypes::channels();
        assert!(!types.is_empty());
        assert!(!types.has_users());
        assert!(!types.has_bots());
        assert!(!types.has_groups());
        assert!(types.has_channels());
    }

    #[test]
    fn test_all() {
        let types = TargetDialogTypes::all();
        assert!(!types.is_empty());
        assert!(types.has_users());
        assert!(types.has_bots());
        assert!(types.has_groups());
        assert!(types.has_channels());
    }

    #[test]
    fn test_from_mask() {
        let types = TargetDialogTypes::from_mask(USERS_MASK | BOTS_MASK);
        assert!(types.has_users());
        assert!(types.has_bots());
        assert!(!types.has_groups());
        assert!(!types.has_channels());
    }

    #[test]
    fn test_mask() {
        assert_eq!(TargetDialogTypes::none().mask(), 0);
        assert_eq!(TargetDialogTypes::users().mask(), USERS_MASK);
        assert_eq!(TargetDialogTypes::bots().mask(), BOTS_MASK);
        assert_eq!(TargetDialogTypes::groups().mask(), CHATS_MASK);
        assert_eq!(TargetDialogTypes::channels().mask(), BROADCASTS_MASK);
    }

    #[test]
    fn test_full_mask() {
        assert_eq!(TargetDialogTypes::none().full_mask(), FULL_MASK);
        assert_eq!(TargetDialogTypes::users().full_mask(), USERS_MASK);
    }

    #[test]
    fn test_bitor() {
        let types = TargetDialogTypes::users() | TargetDialogTypes::bots();
        assert!(types.has_users());
        assert!(types.has_bots());
        assert!(!types.has_groups());
    }

    #[test]
    fn test_bitand() {
        let types1 = TargetDialogTypes::users() | TargetDialogTypes::bots();
        let types2 = TargetDialogTypes::users() | TargetDialogTypes::groups();
        let result = types1 & types2;
        assert!(result.has_users());
        assert!(!result.has_bots());
        assert!(!result.has_groups());
    }

    #[test]
    fn test_bitor_assign() {
        let mut types = TargetDialogTypes::users();
        types |= TargetDialogTypes::bots();
        assert!(types.has_users());
        assert!(types.has_bots());
    }

    #[test]
    fn test_bitand_assign() {
        let mut types = TargetDialogTypes::users() | TargetDialogTypes::bots();
        types &= TargetDialogTypes::users() | TargetDialogTypes::groups();
        assert!(types.has_users());
        assert!(!types.has_bots());
    }

    #[test]
    fn test_equality() {
        assert_eq!(TargetDialogTypes::users(), TargetDialogTypes::users());
        assert_eq!(TargetDialogTypes::all(), TargetDialogTypes::all());
    }

    #[test]
    fn test_inequality() {
        assert_ne!(TargetDialogTypes::users(), TargetDialogTypes::bots());
        assert_ne!(TargetDialogTypes::groups(), TargetDialogTypes::channels());
    }

    #[test]
    fn test_copy_semantics() {
        let types1 = TargetDialogTypes::users();
        let types2 = types1;
        assert_eq!(types1, types2);
        assert!(types1.has_users());
    }

    #[test]
    fn test_clone_semantics() {
        let types1 = TargetDialogTypes::all();
        let types2 = types1.clone();
        assert_eq!(types1, types2);
    }

    #[test]
    fn test_default() {
        let types = TargetDialogTypes::default();
        assert!(types.is_empty());
        assert_eq!(types, TargetDialogTypes::none());
    }

    #[test]
    fn test_combined() {
        let types =
            TargetDialogTypes::users() | TargetDialogTypes::bots() | TargetDialogTypes::groups();
        assert!(types.has_users());
        assert!(types.has_bots());
        assert!(types.has_groups());
        assert!(!types.has_channels());
    }

    #[test]
    fn test_display_users() {
        let s = format!("{}", TargetDialogTypes::users());
        assert!(s.contains("(users)"));
    }

    #[test]
    fn test_display_bots() {
        let s = format!("{}", TargetDialogTypes::bots());
        assert!(s.contains("(bots)"));
    }

    #[test]
    fn test_display_groups() {
        let s = format!("{}", TargetDialogTypes::groups());
        assert!(s.contains("(groups)"));
    }

    #[test]
    fn test_display_channels() {
        let s = format!("{}", TargetDialogTypes::channels());
        assert!(s.contains("(channels)"));
    }

    #[test]
    fn test_display_combined() {
        let types = TargetDialogTypes::users() | TargetDialogTypes::bots();
        let s = format!("{}", types);
        assert!(s.contains("(users)") && s.contains("(bots)"));
    }

    #[test]
    fn test_debug_format() {
        let types = TargetDialogTypes::users();
        let debug_str = format!("{:?}", types);
        assert!(debug_str.contains("TargetDialogTypes"));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = TargetDialogTypes::users() | TargetDialogTypes::bots();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TargetDialogTypes = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_binary() {
        let original = TargetDialogTypes::all();
        let encoded = bincode::serialize(&original).unwrap();
        let decoded: TargetDialogTypes = bincode::deserialize(&encoded).unwrap();
        assert_eq!(original, decoded);
    }
}
