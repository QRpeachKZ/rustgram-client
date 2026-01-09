// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Reaction List Type
//!
//! Type identifier for reaction lists in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`ReactionListType`] enum, which represents
//! the different types of reaction lists used for filtering and sorting
//! reactions in Telegram. It mirrors TDLib's `ReactionListType` enum,
//! providing type-safe list type identifiers.
//!
//! ## List Types
//!
//! - [`Recent`](Self::Recent) - Recently used reactions
//! - [`Top`](Self::Top) - Most popular reactions
//! - [`DefaultTag`](Self::DefaultTag) - Default tag reactions
//!
//! ## Example
//!
//! ```rust
//! use rustgram_reaction_list_type::ReactionListType;
//!
//! // Default is Recent
//! let default = ReactionListType::default();
//! assert_eq!(default, ReactionListType::Recent);
//!
//! // Create different list types
//! let recent = ReactionListType::Recent;
//! let top = ReactionListType::Top;
//! let default_tag = ReactionListType::DefaultTag;
//!
//! // Convert to/from i32
//! assert_eq!(ReactionListType::Recent.as_i32(), 0);
//! assert_eq!(ReactionListType::from_i32(1), Some(ReactionListType::Top));
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Maximum value for reaction list type.
///
/// This constant defines the maximum valid value for reaction list types,
/// matching TDLib's `MAX_REACTION_LIST_TYPE` constant.
pub const MAX_REACTION_LIST_TYPE: i32 = 3;

/// Type of reaction list for filtering/sorting reactions.
///
/// This enum defines the different types of reaction lists in Telegram.
/// It corresponds to TDLib's `ReactionListType` enum and is used for
/// filtering and sorting reactions in various contexts.
///
/// # Variants
///
/// - [`Recent`](Self::Recent) - Recently used reactions
/// - [`Top`](Self::Top) - Most popular/top reactions
/// - [`DefaultTag`](Self::DefaultTag) - Default tag for reactions
///
/// # Example
///
/// ```rust
/// use rustgram_reaction_list_type::ReactionListType;
///
/// // Default list type is Recent
/// let default = ReactionListType::default();
/// assert_eq!(default, ReactionListType::Recent);
///
/// // Create specific list types
/// let recent = ReactionListType::Recent;
/// let top = ReactionListType::Top;
/// let default_tag = ReactionListType::DefaultTag;
///
/// // Display format
/// assert_eq!(format!("{}", ReactionListType::Recent), "Recent");
/// assert_eq!(format!("{}", ReactionListType::Top), "Top");
/// assert_eq!(format!("{}", ReactionListType::DefaultTag), "DefaultTag");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ReactionListType {
    /// Recently used reactions.
    ///
    /// This is the default list type, showing reactions that have been
    /// used recently in the chat.
    ///
    /// Corresponds to `ReactionListType::Recent` in TDLib.
    Recent = 0,

    /// Most popular reactions.
    ///
    /// Shows the most frequently used reactions across all chats.
    ///
    /// Corresponds to `ReactionListType::Top` in TDLib.
    Top = 1,

    /// Default tag reactions.
    ///
    /// Shows reactions associated with the default tag.
    ///
    /// Corresponds to `ReactionListType::DefaultTag` in TDLib.
    DefaultTag = 2,
}

impl Default for ReactionListType {
    /// Returns the default reaction list type (Recent).
    ///
    /// This matches TDLib's behavior.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_reaction_list_type::ReactionListType;
    ///
    /// let default = ReactionListType::default();
    /// assert_eq!(default, ReactionListType::Recent);
    /// ```
    fn default() -> Self {
        Self::Recent
    }
}

impl fmt::Display for ReactionListType {
    /// Formats the reaction list type for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_reaction_list_type::ReactionListType;
    ///
    /// assert_eq!(format!("{}", ReactionListType::Recent), "Recent");
    /// assert_eq!(format!("{}", ReactionListType::Top), "Top");
    /// assert_eq!(format!("{}", ReactionListType::DefaultTag), "DefaultTag");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Recent => write!(f, "Recent"),
            Self::Top => write!(f, "Top"),
            Self::DefaultTag => write!(f, "DefaultTag"),
        }
    }
}

impl ReactionListType {
    /// Returns the integer representation of this reaction list type.
    ///
    /// This matches TDLib's internal representation where the enum values
    /// are represented as integers (0 = Recent, 1 = Top, 2 = DefaultTag).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_reaction_list_type::ReactionListType;
    ///
    /// assert_eq!(ReactionListType::Recent.as_i32(), 0);
    /// assert_eq!(ReactionListType::Top.as_i32(), 1);
    /// assert_eq!(ReactionListType::DefaultTag.as_i32(), 2);
    /// ```
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Creates a reaction list type from its integer representation.
    ///
    /// Returns `None` if the value is not a valid list type (must be 0, 1, or 2).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_reaction_list_type::ReactionListType;
    ///
    /// assert_eq!(ReactionListType::from_i32(0), Some(ReactionListType::Recent));
    /// assert_eq!(ReactionListType::from_i32(1), Some(ReactionListType::Top));
    /// assert_eq!(ReactionListType::from_i32(2), Some(ReactionListType::DefaultTag));
    /// assert_eq!(ReactionListType::from_i32(99), None);
    /// ```
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Recent),
            1 => Some(Self::Top),
            2 => Some(Self::DefaultTag),
            _ => None,
        }
    }

    /// Returns the name of this reaction list type as a string slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_reaction_list_type::ReactionListType;
    ///
    /// assert_eq!(ReactionListType::Recent.as_str(), "Recent");
    /// assert_eq!(ReactionListType::Top.as_str(), "Top");
    /// assert_eq!(ReactionListType::DefaultTag.as_str(), "DefaultTag");
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recent => "Recent",
            Self::Top => "Top",
            Self::DefaultTag => "DefaultTag",
        }
    }

    /// Returns all possible reaction list types.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_reaction_list_type::ReactionListType;
    ///
    /// let all = ReactionListType::all();
    /// assert_eq!(all.len(), 3);
    /// assert!(all.contains(&ReactionListType::Recent));
    /// assert!(all.contains(&ReactionListType::Top));
    /// assert!(all.contains(&ReactionListType::DefaultTag));
    /// ```
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[Self::Recent, Self::Top, Self::DefaultTag]
    }
}

impl TryFrom<i32> for ReactionListType {
    type Error = &'static str;

    /// Creates a reaction list type from its integer representation.
    ///
    /// Returns an error if the value is not valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_reaction_list_type::ReactionListType;
    /// use std::convert::TryFrom;
    ///
    /// assert_eq!(ReactionListType::try_from(0), Ok(ReactionListType::Recent));
    /// assert_eq!(ReactionListType::try_from(1), Ok(ReactionListType::Top));
    /// assert_eq!(ReactionListType::try_from(2), Ok(ReactionListType::DefaultTag));
    /// assert!(ReactionListType::try_from(99).is_err());
    /// ```
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_i32(value).ok_or("Invalid reaction list type value")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_recent() {
        let default = ReactionListType::default();
        assert_eq!(default, ReactionListType::Recent);
    }

    #[test]
    fn test_equality() {
        assert_eq!(ReactionListType::Recent, ReactionListType::Recent);
        assert_eq!(ReactionListType::Top, ReactionListType::Top);
        assert_eq!(ReactionListType::DefaultTag, ReactionListType::DefaultTag);
    }

    #[test]
    fn test_inequality() {
        assert_ne!(ReactionListType::Recent, ReactionListType::Top);
        assert_ne!(ReactionListType::Top, ReactionListType::DefaultTag);
        assert_ne!(ReactionListType::DefaultTag, ReactionListType::Recent);
    }

    #[test]
    fn test_copy_semantics() {
        let original = ReactionListType::Top;
        let copied = original;
        assert_eq!(original, copied);
        assert_eq!(original, ReactionListType::Top);
    }

    #[test]
    fn test_display_format() {
        assert_eq!(format!("{}", ReactionListType::Recent), "Recent");
        assert_eq!(format!("{}", ReactionListType::Top), "Top");
        assert_eq!(format!("{}", ReactionListType::DefaultTag), "DefaultTag");
    }

    #[test]
    fn test_debug_format() {
        let debug_str = format!("{:?}", ReactionListType::Top);
        assert!(debug_str.contains("Top"));
    }

    #[test]
    fn test_as_i32() {
        assert_eq!(ReactionListType::Recent.as_i32(), 0);
        assert_eq!(ReactionListType::Top.as_i32(), 1);
        assert_eq!(ReactionListType::DefaultTag.as_i32(), 2);
    }

    #[test]
    fn test_from_i32_valid() {
        assert_eq!(
            ReactionListType::from_i32(0),
            Some(ReactionListType::Recent)
        );
        assert_eq!(ReactionListType::from_i32(1), Some(ReactionListType::Top));
        assert_eq!(
            ReactionListType::from_i32(2),
            Some(ReactionListType::DefaultTag)
        );
    }

    #[test]
    fn test_from_i32_invalid() {
        assert_eq!(ReactionListType::from_i32(-1), None);
        assert_eq!(ReactionListType::from_i32(3), None);
        assert_eq!(ReactionListType::from_i32(99), None);
        assert_eq!(ReactionListType::from_i32(i32::MAX), None);
    }

    #[test]
    fn test_from_i32_roundtrip() {
        for list_type in ReactionListType::all() {
            let i32_value = list_type.as_i32();
            let restored = ReactionListType::from_i32(i32_value);
            assert_eq!(Some(*list_type), restored);
        }
    }

    #[test]
    fn test_as_str() {
        assert_eq!(ReactionListType::Recent.as_str(), "Recent");
        assert_eq!(ReactionListType::Top.as_str(), "Top");
        assert_eq!(ReactionListType::DefaultTag.as_str(), "DefaultTag");
    }

    #[test]
    fn test_all() {
        let all = ReactionListType::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&ReactionListType::Recent));
        assert!(all.contains(&ReactionListType::Top));
        assert!(all.contains(&ReactionListType::DefaultTag));
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ReactionListType::Recent);
        set.insert(ReactionListType::Top);
        set.insert(ReactionListType::DefaultTag);

        assert_eq!(set.len(), 3);

        // Duplicate insertions don't increase size
        set.insert(ReactionListType::Top);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_try_from_valid() {
        use std::convert::TryFrom;

        assert_eq!(ReactionListType::try_from(0), Ok(ReactionListType::Recent));
        assert_eq!(ReactionListType::try_from(1), Ok(ReactionListType::Top));
        assert_eq!(
            ReactionListType::try_from(2),
            Ok(ReactionListType::DefaultTag)
        );
    }

    #[test]
    fn test_try_from_invalid() {
        use std::convert::TryFrom;

        assert!(ReactionListType::try_from(-1).is_err());
        assert!(ReactionListType::try_from(3).is_err());
        assert!(ReactionListType::try_from(99).is_err());
    }

    #[test]
    fn test_max_value() {
        assert_eq!(MAX_REACTION_LIST_TYPE, 3);
        assert!((ReactionListType::DefaultTag.as_i32()) < MAX_REACTION_LIST_TYPE);
    }

    #[test]
    fn test_array_iteration() {
        let all = ReactionListType::all();
        let mut count = 0;

        for list_type in all {
            match list_type {
                ReactionListType::Recent => count += 1,
                ReactionListType::Top => count += 1,
                ReactionListType::DefaultTag => count += 1,
            }
        }

        assert_eq!(count, 3);
    }

    #[test]
    fn test_match_exhaustiveness() {
        fn check_exhaustive(list_type: ReactionListType) -> bool {
            matches!(
                list_type,
                ReactionListType::Recent | ReactionListType::Top | ReactionListType::DefaultTag
            )
        }

        assert!(check_exhaustive(ReactionListType::Recent));
        assert!(check_exhaustive(ReactionListType::Top));
        assert!(check_exhaustive(ReactionListType::DefaultTag));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let list_types = [
            ReactionListType::Recent,
            ReactionListType::Top,
            ReactionListType::DefaultTag,
        ];

        for list_type in list_types {
            // JSON serialization
            let json = serde_json::to_string(&list_type).unwrap();
            let deserialized: ReactionListType = serde_json::from_str(&json).unwrap();
            assert_eq!(list_type, deserialized);

            // Binary serialization
            let encoded = bincode::serialize(&list_type).unwrap();
            let decoded: ReactionListType = bincode::deserialize(&encoded).unwrap();
            assert_eq!(list_type, decoded);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_json_representation() {
        // JSON representation uses external tag for enums
        let json = serde_json::to_string(&ReactionListType::Recent).unwrap();
        assert_eq!(json, "\"Recent\"");

        let json = serde_json::to_string(&ReactionListType::Top).unwrap();
        assert_eq!(json, "\"Top\"");

        let json = serde_json::to_string(&ReactionListType::DefaultTag).unwrap();
        assert_eq!(json, "\"DefaultTag\"");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_invalid_json() {
        // Invalid JSON values should fail
        let result: Result<ReactionListType, _> = serde_json::from_str("\"Invalid\"");
        assert!(result.is_err());
    }
}
