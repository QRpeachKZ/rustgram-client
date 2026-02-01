// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Block List ID - Block list identifier for Telegram MTProto client.
//!
//! This module provides the [`BlockListId`] type which represents different
//! block lists in Telegram (main block list and stories block list).
//!
//! ## Overview
//!
//! The block list ID type distinguishes between different block lists:
//! - **Main**: The main blocked users list
//! - **Stories**: The stories-specific block list
//! - **None**: No block list (default/invalid state)
//!
//! ## Examples
//!
//! ### Creating Block List IDs
//!
//! ```
//! use rustgram_block_list_id::BlockListId;
//!
//! // Create main block list
//! let main = BlockListId::main();
//! assert!(main.is_valid());
//! assert!(main.is_main());
//!
//! // Create stories block list
//! let stories = BlockListId::stories();
//! assert!(stories.is_valid());
//! assert!(stories.is_stories());
//!
//! // Default is invalid/none
//! let none = BlockListId::default();
//! assert!(!none.is_valid());
//! ```
//!
//! ### Creating from Flags
//!
//! ```
//! use rustgram_block_list_id::BlockListId;
//!
//! // From boolean flags (main takes priority)
//! let id = BlockListId::from_flags(true, false);
//! assert!(id.is_main());
//!
//! // Stories only
//! let id = BlockListId::from_flags(false, true);
//! assert!(id.is_stories());
//!
//! // Neither
//! let id = BlockListId::from_flags(false, false);
//! assert!(!id.is_valid());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::hash::{Hash, Hasher};

/// Block list type identifier.
///
/// Represents which block list a user is on in Telegram. Users can be blocked
/// from the main chat list, from stories only, or not blocked at all.
///
/// # Examples
///
/// ```
/// use rustgram_block_list_id::BlockListId;
///
/// let main = BlockListId::main();
/// assert!(main.is_valid());
/// assert!(main.is_main());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockListId {
    /// No block list (invalid/empty state)
    None,
    /// Main blocked users list
    Main,
    /// Stories-specific block list
    Stories,
}

impl Default for BlockListId {
    fn default() -> Self {
        Self::None
    }
}

impl BlockListId {
    /// Creates a [`BlockListId::Main`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_block_list_id::BlockListId;
    ///
    /// let main = BlockListId::main();
    /// assert!(main.is_main());
    /// ```
    pub const fn main() -> Self {
        Self::Main
    }

    /// Creates a [`BlockListId::Stories`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_block_list_id::BlockListId;
    ///
    /// let stories = BlockListId::stories();
    /// assert!(stories.is_stories());
    /// ```
    pub const fn stories() -> Self {
        Self::Stories
    }

    /// Creates a block list ID from boolean flags.
    ///
    /// The main flag takes priority over stories flag.
    ///
    /// # Arguments
    ///
    /// * `is_blocked` - If true, returns [`Main`](BlockListId::Main)
    /// * `is_blocked_for_stories` - If true and `is_blocked` is false, returns [`Stories`](BlockListId::Stories)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_block_list_id::BlockListId;
    ///
    /// assert_eq!(BlockListId::from_flags(true, false), BlockListId::Main);
    /// assert_eq!(BlockListId::from_flags(true, true), BlockListId::Main);
    /// assert_eq!(BlockListId::from_flags(false, true), BlockListId::Stories);
    /// assert_eq!(BlockListId::from_flags(false, false), BlockListId::None);
    /// ```
    pub const fn from_flags(is_blocked: bool, is_blocked_for_stories: bool) -> Self {
        if is_blocked {
            Self::Main
        } else if is_blocked_for_stories {
            Self::Stories
        } else {
            Self::None
        }
    }

    /// Returns `true` if this is a valid block list.
    ///
    /// A block list is valid if it is either [`Main`](BlockListId::Main) or
    /// [`Stories`](BlockListId::Stories). [`None`](BlockListId::None) is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_block_list_id::BlockListId;
    ///
    /// assert!(BlockListId::Main.is_valid());
    /// assert!(BlockListId::Stories.is_valid());
    /// assert!(!BlockListId::None.is_valid());
    /// ```
    pub const fn is_valid(self) -> bool {
        matches!(self, Self::Main | Self::Stories)
    }

    /// Returns `true` if this is the main block list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_block_list_id::BlockListId;
    ///
    /// assert!(BlockListId::Main.is_main());
    /// assert!(!BlockListId::Stories.is_main());
    /// assert!(!BlockListId::None.is_main());
    /// ```
    pub const fn is_main(self) -> bool {
        matches!(self, Self::Main)
    }

    /// Returns `true` if this is the stories block list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_block_list_id::BlockListId;
    ///
    /// assert!(!BlockListId::Main.is_stories());
    /// assert!(BlockListId::Stories.is_stories());
    /// assert!(!BlockListId::None.is_stories());
    /// ```
    pub const fn is_stories(self) -> bool {
        matches!(self, Self::Stories)
    }
}

impl Hash for BlockListId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Discriminant value for hashing
        let discriminant = match self {
            Self::None => -1i32,
            Self::Main => 0i32,
            Self::Stories => 1i32,
        };
        discriminant.hash(state);
    }
}

impl std::fmt::Display for BlockListId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Main => write!(f, "MainBlockList"),
            Self::Stories => write!(f, "StoriesBlockList"),
            Self::None => write!(f, "InvalidBlockList"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let id = BlockListId::default();
        assert_eq!(id, BlockListId::None);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_main() {
        let id = BlockListId::main();
        assert!(id.is_valid());
        assert!(id.is_main());
        assert!(!id.is_stories());
    }

    #[test]
    fn test_stories() {
        let id = BlockListId::stories();
        assert!(id.is_valid());
        assert!(!id.is_main());
        assert!(id.is_stories());
    }

    #[test]
    fn test_from_flags_main() {
        let id = BlockListId::from_flags(true, false);
        assert_eq!(id, BlockListId::Main);

        let id = BlockListId::from_flags(true, true);
        assert_eq!(id, BlockListId::Main);
    }

    #[test]
    fn test_from_flags_stories() {
        let id = BlockListId::from_flags(false, true);
        assert_eq!(id, BlockListId::Stories);
    }

    #[test]
    fn test_from_flags_none() {
        let id = BlockListId::from_flags(false, false);
        assert_eq!(id, BlockListId::None);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_is_valid() {
        assert!(BlockListId::Main.is_valid());
        assert!(BlockListId::Stories.is_valid());
        assert!(!BlockListId::None.is_valid());
    }

    #[test]
    fn test_equality() {
        assert_eq!(BlockListId::Main, BlockListId::Main);
        assert_eq!(BlockListId::Stories, BlockListId::Stories);
        assert_eq!(BlockListId::None, BlockListId::None);

        assert_ne!(BlockListId::Main, BlockListId::Stories);
        assert_ne!(BlockListId::Main, BlockListId::None);
        assert_ne!(BlockListId::Stories, BlockListId::None);
    }

    #[test]
    fn test_copy() {
        let id1 = BlockListId::Main;
        let id2 = id1;
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_clone() {
        let id = BlockListId::Stories;
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", BlockListId::Main), "MainBlockList");
        assert_eq!(format!("{}", BlockListId::Stories), "StoriesBlockList");
        assert_eq!(format!("{}", BlockListId::None), "InvalidBlockList");
    }

    #[test]
    fn test_debug() {
        let debug_str = format!("{:?}", BlockListId::Main);
        assert!(debug_str.contains("Main"));
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(BlockListId::Main);
        set.insert(BlockListId::Stories);
        set.insert(BlockListId::None);

        assert_eq!(set.len(), 3);
        assert!(set.contains(&BlockListId::Main));
        assert!(set.contains(&BlockListId::Stories));
        assert!(set.contains(&BlockListId::None));
    }
}
