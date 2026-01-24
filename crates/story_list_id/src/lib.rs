// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Story List ID
//!
//! Story list identifier for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`StoryListId`] enum, which represents
//! different story lists in Telegram (main or archive).
//!
//! ## Example
//!
//! ```rust
//! use rustgram_story_list_id::StoryListId;
//!
//! let main = StoryListId::main();
//! assert!(main.is_main());
//! assert!(!main.is_archive());
//!
//! let archive = StoryListId::archive();
//! assert!(archive.is_archive());
//! assert!(!archive.is_main());
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Story list identifier.
///
/// Represents which story list a story belongs to.
/// Stories can be in the main list or archived.
///
/// # TDLib Alignment
///
/// Aligns with TDLib's `class StoryListId { int32 type = 0; }`.
/// - Type 0: Main
/// - Type 1: Archive
///
/// # Example
///
/// ```rust
/// use rustgram_story_list_id::StoryListId;
///
/// let main = StoryListId::main();
/// assert!(main.is_main());
///
/// let archive = StoryListId::archive();
/// assert!(archive.is_archive());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StoryListId {
    /// Main story list.
    #[default]
    Main,

    /// Archived stories.
    Archive,
}

impl StoryListId {
    /// Creates a main story list ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_list_id::StoryListId;
    ///
    /// let main = StoryListId::main();
    /// assert!(main.is_main());
    /// ```
    #[must_use]
    pub const fn main() -> Self {
        Self::Main
    }

    /// Creates an archive story list ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_list_id::StoryListId;
    ///
    /// let archive = StoryListId::archive();
    /// assert!(archive.is_archive());
    /// ```
    #[must_use]
    pub const fn archive() -> Self {
        Self::Archive
    }

    /// Returns `true` if this is the main story list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_list_id::StoryListId;
    ///
    /// assert!(StoryListId::main().is_main());
    /// assert!(!StoryListId::archive().is_main());
    /// ```
    #[must_use]
    pub const fn is_main(&self) -> bool {
        matches!(self, Self::Main)
    }

    /// Returns `true` if this is the archive story list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_list_id::StoryListId;
    ///
    /// assert!(StoryListId::archive().is_archive());
    /// assert!(!StoryListId::main().is_archive());
    /// ```
    #[must_use]
    pub const fn is_archive(&self) -> bool {
        matches!(self, Self::Archive)
    }

    /// Returns the TDLib type value.
    ///
    /// - 0 for Main
    /// - 1 for Archive
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_list_id::StoryListId;
    ///
    /// assert_eq!(StoryListId::main().type_value(), 0);
    /// assert_eq!(StoryListId::archive().type_value(), 1);
    /// ```
    #[must_use]
    pub const fn type_value(&self) -> i32 {
        match self {
            Self::Main => 0,
            Self::Archive => 1,
        }
    }

    /// Creates a story list ID from a TDLib type value.
    ///
    /// # Arguments
    ///
    /// * `value` - The TDLib type value (0 or 1)
    ///
    /// # Returns
    ///
    /// `Some(StoryListId)` if the value is valid, `None` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_list_id::StoryListId;
    ///
    /// assert_eq!(StoryListId::from_type_value(0), Some(StoryListId::main()));
    /// assert_eq!(StoryListId::from_type_value(1), Some(StoryListId::archive()));
    /// assert_eq!(StoryListId::from_type_value(2), None);
    /// ```
    #[must_use]
    pub const fn from_type_value(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Main),
            1 => Some(Self::Archive),
            _ => None,
        }
    }
}

impl fmt::Display for StoryListId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Main => write!(f, "main"),
            Self::Archive => write!(f, "archive"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let main = StoryListId::main();
        assert!(main.is_main());
        assert!(!main.is_archive());
    }

    #[test]
    fn test_archive() {
        let archive = StoryListId::archive();
        assert!(archive.is_archive());
        assert!(!archive.is_main());
    }

    #[test]
    fn test_is_main() {
        assert!(StoryListId::main().is_main());
        assert!(!StoryListId::archive().is_main());
    }

    #[test]
    fn test_is_archive() {
        assert!(StoryListId::archive().is_archive());
        assert!(!StoryListId::main().is_archive());
    }

    #[test]
    fn test_type_value_main() {
        assert_eq!(StoryListId::main().type_value(), 0);
    }

    #[test]
    fn test_type_value_archive() {
        assert_eq!(StoryListId::archive().type_value(), 1);
    }

    #[test]
    fn test_from_type_value_main() {
        assert_eq!(StoryListId::from_type_value(0), Some(StoryListId::Main));
    }

    #[test]
    fn test_from_type_value_archive() {
        assert_eq!(StoryListId::from_type_value(1), Some(StoryListId::Archive));
    }

    #[test]
    fn test_from_type_value_invalid() {
        assert_eq!(StoryListId::from_type_value(-1), None);
        assert_eq!(StoryListId::from_type_value(2), None);
        assert_eq!(StoryListId::from_type_value(100), None);
    }

    #[test]
    fn test_default() {
        assert_eq!(StoryListId::default(), StoryListId::Main);
    }

    #[test]
    fn test_equality() {
        assert_eq!(StoryListId::main(), StoryListId::main());
        assert_eq!(StoryListId::archive(), StoryListId::archive());
        assert_ne!(StoryListId::main(), StoryListId::archive());
    }

    #[test]
    fn test_clone() {
        let main = StoryListId::main();
        let cloned = main;
        assert_eq!(main, cloned);

        let archive = StoryListId::archive();
        let cloned_archive = archive;
        assert_eq!(archive, cloned_archive);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let main1 = StoryListId::main();
        let main2 = StoryListId::main();

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        main1.hash(&mut hasher1);
        main2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());

        let archive = StoryListId::archive();
        let mut hasher3 = DefaultHasher::new();
        archive.hash(&mut hasher3);

        assert_ne!(hasher1.finish(), hasher3.finish());
    }

    #[test]
    fn test_display_main() {
        assert_eq!(format!("{}", StoryListId::main()), "main");
    }

    #[test]
    fn test_display_archive() {
        assert_eq!(format!("{}", StoryListId::archive()), "archive");
    }

    #[test]
    fn test_debug_format() {
        let main = StoryListId::main();
        let debug_str = format!("{:?}", main);
        assert!(debug_str.contains("Main"));

        let archive = StoryListId::archive();
        let debug_str = format!("{:?}", archive);
        assert!(debug_str.contains("Archive"));
    }

    #[test]
    fn test_match() {
        match StoryListId::main() {
            StoryListId::Main => assert!(true),
            StoryListId::Archive => assert!(false),
        }

        match StoryListId::archive() {
            StoryListId::Main => assert!(false),
            StoryListId::Archive => assert!(true),
        }
    }

    #[test]
    fn test_roundtrip_type_value() {
        let main = StoryListId::main();
        let archive = StoryListId::archive();

        assert_eq!(StoryListId::from_type_value(main.type_value()), Some(main));
        assert_eq!(
            StoryListId::from_type_value(archive.type_value()),
            Some(archive)
        );
    }

    // ========== Serialization Tests ==========

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_main() {
        let main = StoryListId::main();
        let json = serde_json::to_string(&main).unwrap();
        let deserialized: StoryListId = serde_json::from_str(&json).unwrap();
        assert_eq!(main, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_archive() {
        let archive = StoryListId::archive();
        let json = serde_json::to_string(&archive).unwrap();
        let deserialized: StoryListId = serde_json::from_str(&json).unwrap();
        assert_eq!(archive, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_format() {
        let main = StoryListId::main();
        let json = serde_json::to_string(&main).unwrap();
        assert!(json.contains("Main") || json.contains("main"));

        let archive = StoryListId::archive();
        let json = serde_json::to_string(&archive).unwrap();
        assert!(json.contains("Archive") || json.contains("archive"));
    }
}
