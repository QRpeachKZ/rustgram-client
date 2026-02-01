// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Rustgram Chat Reactions
//!
//! Chat reactions container for Telegram MTProto client.
//!
//! This crate provides a container type for managing reaction settings in a chat,
//! including which reactions are allowed, limits, and paid reaction availability.
//!
//! ## Overview
//!
//! Telegram chats can have different reaction policies:
//!
//! - **Specific reactions**: Only certain emoji/custom reactions allowed
//! - **All regular reactions**: Any standard emoji reaction allowed
//! - **All custom reactions**: Any custom emoji reaction allowed
//! - **Paid reactions**: Telegram Premium star reactions available
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_chat_reactions::ChatReactions;
//! use rustgram_reaction_type::ReactionType;
//!
//! // Create with specific reactions
//! let reactions = ChatReactions::with_reactions(vec![
//!     ReactionType::emoji("👍"),
//!     ReactionType::emoji("❤️"),
//! ]);
//!
//! // Allow all regular reactions
//! let all_regular = ChatReactions::allow_all(false);
//! assert!(all_regular.allows_all_regular());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::iter_without_into_iter)]

use rustgram_reaction_type::ReactionType;
use std::fmt;

/// Container for chat reaction settings.
///
/// Controls which reactions are available in a chat and any limits on their use.
///
/// # Fields
///
/// * `reaction_types` - Specific allowed reaction types
/// * `allow_all_regular` - Whether all regular emoji reactions are allowed
/// * `allow_all_custom` - Whether all custom emoji reactions are allowed
/// * `reactions_limit` - Maximum number of unique reactions per message (0 = unlimited)
/// * `paid_reactions_available` - Whether paid reactions are available
///
/// # Examples
///
/// ```
/// use rustgram_chat_reactions::ChatReactions;
/// use rustgram_reaction_type::ReactionType;
///
/// // Create empty reactions
/// let empty = ChatReactions::new();
/// assert!(empty.is_empty());
///
/// // Create with specific reactions
/// let specific = ChatReactions::with_reactions(vec![
///     ReactionType::emoji("👍"),
/// ]);
/// assert!(!specific.is_empty());
///
/// // Allow all regular reactions
/// let all = ChatReactions::allow_all(false);
/// assert!(all.allows_all_regular());
/// ```
#[derive(Debug, Clone, Eq)]
pub struct ChatReactions {
    /// Specific allowed reaction types.
    reaction_types: Vec<ReactionType>,
    /// Whether all regular emoji reactions are allowed.
    allow_all_regular: bool,
    /// Whether all custom emoji reactions are allowed.
    allow_all_custom: bool,
    /// Maximum unique reactions per message (0 = unlimited).
    reactions_limit: i32,
    /// Whether paid reactions are available.
    paid_reactions_available: bool,
}

impl ChatReactions {
    /// Maximum possible reactions limit.
    pub const MAX_LIMIT: i32 = 100;

    /// Creates a new empty `ChatReactions`.
    ///
    /// All fields are set to default/false values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_reactions::ChatReactions;
    ///
    /// let reactions = ChatReactions::new();
    /// assert!(reactions.is_empty());
    /// assert!(!reactions.allows_all_regular());
    /// assert!(!reactions.allows_all_custom());
    /// assert_eq!(reactions.reactions_limit(), 0);
    /// assert!(!reactions.paid_available());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            reaction_types: Vec::new(),
            allow_all_regular: false,
            allow_all_custom: false,
            reactions_limit: 0,
            paid_reactions_available: false,
        }
    }

    /// Creates `ChatReactions` with specific reaction types.
    ///
    /// # Arguments
    ///
    /// * `reactions` - Vector of allowed reaction types
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_reactions::ChatReactions;
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let reactions = ChatReactions::with_reactions(vec![
    ///     ReactionType::emoji("👍"),
    ///     ReactionType::emoji("❤️"),
    /// ]);
    /// assert_eq!(reactions.reaction_types().len(), 2);
    /// ```
    #[must_use]
    pub fn with_reactions(reactions: Vec<ReactionType>) -> Self {
        Self {
            reaction_types: reactions,
            ..Self::new()
        }
    }

    /// Creates `ChatReactions` that allow all reactions.
    ///
    /// # Arguments
    ///
    /// * `allow_custom` - Whether to also allow all custom emoji reactions
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_reactions::ChatReactions;
    ///
    /// let all = ChatReactions::allow_all(false);
    /// assert!(all.allows_all_regular());
    /// assert!(!all.allows_all_custom());
    ///
    /// let all_with_custom = ChatReactions::allow_all(true);
    /// assert!(all_with_custom.allows_all_regular());
    /// assert!(all_with_custom.allows_all_custom());
    /// ```
    #[must_use]
    pub fn allow_all(allow_custom: bool) -> Self {
        Self {
            allow_all_regular: true,
            allow_all_custom: allow_custom,
            ..Self::new()
        }
    }

    /// Creates `ChatReactions` with specific reactions and a limit.
    ///
    /// # Arguments
    ///
    /// * `reactions` - Vector of allowed reaction types
    /// * `limit` - Maximum unique reactions per message (0 = unlimited)
    /// * `paid_available` - Whether paid reactions are available
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_reactions::ChatReactions;
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let reactions = ChatReactions::with_limit(
    ///     vec![ReactionType::emoji("👍")],
    ///     5,
    ///     true,
    /// );
    /// assert_eq!(reactions.reactions_limit(), 5);
    /// assert!(reactions.paid_available());
    /// ```
    #[must_use]
    pub fn with_limit(reactions: Vec<ReactionType>, limit: i32, paid_available: bool) -> Self {
        Self {
            reaction_types: reactions,
            reactions_limit: limit,
            paid_reactions_available: paid_available,
            ..Self::new()
        }
    }

    /// Creates `ChatReactions` using the legacy format (reactions only).
    ///
    /// This is for compatibility with older versions.
    ///
    /// # Arguments
    ///
    /// * `reactions` - Vector of allowed reaction types
    #[must_use]
    pub fn legacy(reactions: Vec<ReactionType>) -> Self {
        Self::with_reactions(reactions)
    }

    /// Checks if this is empty (no reactions available).
    ///
    /// Returns true if:
    /// - No specific reaction types
    /// - Not allowing all regular reactions
    /// - Not allowing all custom reactions
    /// - No paid reactions available
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_reactions::ChatReactions;
    ///
    /// assert!(ChatReactions::new().is_empty());
    /// assert!(!ChatReactions::allow_all(false).is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.reaction_types.is_empty()
            && !self.allow_all_regular
            && !self.allow_all_custom
            && !self.paid_reactions_available
    }

    /// Returns whether all regular emoji reactions are allowed.
    #[must_use]
    pub const fn allows_all_regular(&self) -> bool {
        self.allow_all_regular
    }

    /// Returns whether all custom emoji reactions are allowed.
    #[must_use]
    pub const fn allows_all_custom(&self) -> bool {
        self.allow_all_custom
    }

    /// Returns the reactions limit (0 = unlimited).
    #[must_use]
    pub const fn reactions_limit(&self) -> i32 {
        self.reactions_limit
    }

    /// Returns whether paid reactions are available.
    #[must_use]
    pub const fn paid_available(&self) -> bool {
        self.paid_reactions_available
    }

    /// Returns a slice of the specific reaction types.
    #[must_use]
    pub fn reaction_types(&self) -> &[ReactionType] {
        &self.reaction_types
    }

    /// Sets the reactions limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum unique reactions per message (0 = unlimited)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_reactions::ChatReactions;
    ///
    /// let mut reactions = ChatReactions::new();
    /// reactions.set_reactions_limit(10);
    /// assert_eq!(reactions.reactions_limit(), 10);
    /// ```
    pub fn set_reactions_limit(&mut self, limit: i32) {
        self.reactions_limit = limit.clamp(0, Self::MAX_LIMIT);
    }

    /// Ignores non-paid reactions (removes them from the list).
    ///
    /// Also clears allow_all_regular and allow_all_custom flags.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_reactions::ChatReactions;
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let mut reactions = ChatReactions::with_reactions(vec![
    ///     ReactionType::emoji("👍"),
    ///     ReactionType::paid(),
    /// ]);
    /// reactions.ignore_non_paid();
    /// assert_eq!(reactions.reaction_types().len(), 1);
    /// assert!(reactions.reaction_types()[0].is_paid());
    /// ```
    pub fn ignore_non_paid(&mut self) {
        self.reaction_types.retain(|r| r.is_paid());
        self.allow_all_regular = false;
        self.allow_all_custom = false;
    }

    /// Removes paid reactions from the list.
    ///
    /// Returns true if any paid reactions were removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_reactions::ChatReactions;
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let mut reactions = ChatReactions::with_reactions(vec![
    ///     ReactionType::emoji("👍"),
    ///     ReactionType::paid(),
    /// ]);
    /// let removed = reactions.remove_paid();
    /// assert!(removed);
    /// assert_eq!(reactions.reaction_types().len(), 1);
    /// ```
    pub fn remove_paid(&mut self) -> bool {
        let original_len = self.reaction_types.len();
        self.reaction_types.retain(|r| !r.is_paid());
        self.paid_reactions_available = false;
        original_len != self.reaction_types.len()
    }

    /// Checks if a specific reaction is allowed.
    ///
    /// A reaction is allowed if:
    /// - It's in the specific reaction types list, OR
    /// - It's a regular emoji and allow_all_regular is true, OR
    /// - It's a custom emoji and allow_all_custom is true, OR
    /// - It's a paid reaction and paid_reactions_available is true
    ///
    /// # Arguments
    ///
    /// * `reaction` - The reaction to check
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_reactions::ChatReactions;
    /// use rustgram_reaction_type::ReactionType;
    ///
    /// let reactions = ChatReactions::with_reactions(vec![
    ///     ReactionType::emoji("👍"),
    /// ]);
    /// assert!(reactions.is_allowed(&ReactionType::emoji("👍")));
    /// assert!(!reactions.is_allowed(&ReactionType::emoji("❤️")));
    /// ```
    #[must_use]
    pub fn is_allowed(&self, reaction: &ReactionType) -> bool {
        // Check if in specific list
        if self.reaction_types.contains(reaction) {
            return true;
        }

        // Check if all regular allowed
        if self.allow_all_regular && reaction.is_emoji() {
            return true;
        }

        // Check if all custom allowed
        if self.allow_all_custom && reaction.is_custom() {
            return true;
        }

        // Check if paid allowed
        if self.paid_reactions_available && reaction.is_paid() {
            return true;
        }

        false
    }
}

/// PartialEq implementation that ignores `allow_all_custom` field.
///
/// This matches TDLib's behavior where the `allow_all_custom_` field
/// is not considered when comparing ChatReactions for equality.
impl PartialEq for ChatReactions {
    fn eq(&self, other: &Self) -> bool {
        self.reaction_types == other.reaction_types
            && self.allow_all_regular == other.allow_all_regular
            && self.reactions_limit == other.reactions_limit
            && self.paid_reactions_available == other.paid_reactions_available
    }
}

impl Default for ChatReactions {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ChatReactions {
    /// Formats the `ChatReactions` for display.
    ///
    /// Shows a summary of the reaction settings.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if self.reactions_limit > 0 {
            parts.push(format!("limit: {}", self.reactions_limit));
        }

        if self.allow_all_regular {
            parts.push("all_regular".to_string());
        }

        if self.allow_all_custom {
            parts.push("all_custom".to_string());
        }

        if self.paid_reactions_available {
            parts.push("paid".to_string());
        }

        if !self.reaction_types.is_empty() {
            parts.push(format!("{} types", self.reaction_types.len()));
        }

        if parts.is_empty() {
            write!(f, "ChatReactions(empty)")
        } else {
            write!(f, "ChatReactions({})", parts.join(", "))
        }
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-chat-reactions";

#[cfg(test)]
mod tests {
    use super::*;

    // === Construction Tests ===

    #[test]
    fn test_default_empty() {
        let reactions = ChatReactions::default();
        assert!(reactions.is_empty());
        assert!(!reactions.allows_all_regular());
        assert!(!reactions.allows_all_custom());
        assert_eq!(reactions.reactions_limit(), 0);
        assert!(!reactions.paid_available());
    }

    #[test]
    fn test_new_empty() {
        let reactions = ChatReactions::new();
        assert!(reactions.is_empty());
        assert_eq!(reactions.reaction_types().len(), 0);
    }

    #[test]
    fn test_with_reactions() {
        let reactions = ChatReactions::with_reactions(vec![
            ReactionType::emoji("👍"),
            ReactionType::emoji("❤️"),
        ]);
        assert_eq!(reactions.reaction_types().len(), 2);
        assert!(!reactions.is_empty());
    }

    #[test]
    fn test_allow_all_regular() {
        let reactions = ChatReactions::allow_all(false);
        assert!(reactions.allows_all_regular());
        assert!(!reactions.allows_all_custom());
        assert!(!reactions.is_empty());
    }

    #[test]
    fn test_allow_all_custom() {
        let reactions = ChatReactions::allow_all(true);
        assert!(reactions.allows_all_regular());
        assert!(reactions.allows_all_custom());
    }

    #[test]
    fn test_with_limit() {
        let reactions = ChatReactions::with_limit(vec![ReactionType::emoji("👍")], 10, true);
        assert_eq!(reactions.reactions_limit(), 10);
        assert!(reactions.paid_available());
        assert_eq!(reactions.reaction_types().len(), 1);
    }

    #[test]
    fn test_legacy_constructor() {
        let reactions =
            ChatReactions::legacy(vec![ReactionType::emoji("👍"), ReactionType::emoji("❤️")]);
        assert_eq!(reactions.reaction_types().len(), 2);
    }

    // === is_empty Tests ===

    #[test]
    fn test_is_empty_true() {
        let reactions = ChatReactions::new();
        assert!(reactions.is_empty());
    }

    #[test]
    fn test_is_empty_false_with_reactions() {
        let reactions = ChatReactions::with_reactions(vec![ReactionType::emoji("👍")]);
        assert!(!reactions.is_empty());
    }

    #[test]
    fn test_is_empty_false_all_regular() {
        let reactions = ChatReactions::allow_all(false);
        assert!(!reactions.is_empty());
    }

    #[test]
    fn test_is_empty_false_all_custom() {
        let reactions = ChatReactions::allow_all(true);
        assert!(!reactions.is_empty());
    }

    #[test]
    fn test_is_empty_false_paid() {
        let reactions = ChatReactions::with_limit(vec![], 0, true);
        assert!(!reactions.is_empty());
    }

    // === Query Methods Tests ===

    #[test]
    fn test_allows_all_regular() {
        let reactions = ChatReactions::allow_all(false);
        assert!(reactions.allows_all_regular());
    }

    #[test]
    fn test_allows_all_custom() {
        let reactions = ChatReactions::allow_all(true);
        assert!(reactions.allows_all_custom());
    }

    #[test]
    fn test_reactions_limit() {
        let reactions = ChatReactions::with_limit(vec![], 25, false);
        assert_eq!(reactions.reactions_limit(), 25);
    }

    #[test]
    fn test_paid_available() {
        let reactions = ChatReactions::with_limit(vec![], 0, true);
        assert!(reactions.paid_available());
    }

    #[test]
    fn test_reaction_types_slice() {
        let reactions = ChatReactions::with_reactions(vec![
            ReactionType::emoji("👍"),
            ReactionType::emoji("❤️"),
        ]);
        let types = reactions.reaction_types();
        assert_eq!(types.len(), 2);
    }

    // === Mutation Tests ===

    #[test]
    fn test_set_reactions_limit() {
        let mut reactions = ChatReactions::new();
        reactions.set_reactions_limit(15);
        assert_eq!(reactions.reactions_limit(), 15);
    }

    #[test]
    fn test_set_reactions_limit_clamp_max() {
        let mut reactions = ChatReactions::new();
        reactions.set_reactions_limit(999);
        assert_eq!(reactions.reactions_limit(), ChatReactions::MAX_LIMIT);
    }

    #[test]
    fn test_set_reactions_limit_clamp_min() {
        let mut reactions = ChatReactions::new();
        reactions.set_reactions_limit(-10);
        assert_eq!(reactions.reactions_limit(), 0);
    }

    #[test]
    fn test_ignore_non_paid() {
        let mut reactions = ChatReactions::with_reactions(vec![
            ReactionType::emoji("👍"),
            ReactionType::paid(),
            ReactionType::emoji("❤️"),
        ]);
        reactions.ignore_non_paid();
        assert_eq!(reactions.reaction_types().len(), 1);
        assert!(reactions.reaction_types()[0].is_paid());
        assert!(!reactions.allows_all_regular());
        assert!(!reactions.allows_all_custom());
    }

    #[test]
    fn test_remove_paid_true() {
        let mut reactions =
            ChatReactions::with_reactions(vec![ReactionType::emoji("👍"), ReactionType::paid()]);
        let removed = reactions.remove_paid();
        assert!(removed);
        assert_eq!(reactions.reaction_types().len(), 1);
        assert!(!reactions.paid_available());
    }

    #[test]
    fn test_remove_paid_false() {
        let mut reactions = ChatReactions::with_reactions(vec![
            ReactionType::emoji("👍"),
            ReactionType::emoji("❤️"),
        ]);
        let removed = reactions.remove_paid();
        assert!(!removed);
        assert_eq!(reactions.reaction_types().len(), 2);
    }

    // === is_allowed Tests ===

    #[test]
    fn test_is_allowed_specific() {
        let reactions = ChatReactions::with_reactions(vec![ReactionType::emoji("👍")]);
        assert!(reactions.is_allowed(&ReactionType::emoji("👍")));
        assert!(!reactions.is_allowed(&ReactionType::emoji("❤️")));
    }

    #[test]
    fn test_is_allowed_all_regular() {
        let reactions = ChatReactions::allow_all(false);
        assert!(reactions.is_allowed(&ReactionType::emoji("👍")));
        assert!(reactions.is_allowed(&ReactionType::emoji("❤️")));
        assert!(!reactions.is_allowed(&ReactionType::custom_emoji(&[1, 2, 3])));
    }

    #[test]
    fn test_is_allowed_all_custom() {
        let reactions = ChatReactions::allow_all(true);
        assert!(reactions.is_allowed(&ReactionType::custom_emoji(&[1, 2, 3])));
        assert!(reactions.is_allowed(&ReactionType::emoji("👍")));
    }

    #[test]
    fn test_is_allowed_paid() {
        let reactions = ChatReactions::with_limit(vec![], 0, true);
        assert!(reactions.is_allowed(&ReactionType::paid()));
        assert!(!reactions.is_allowed(&ReactionType::emoji("👍")));
    }

    #[test]
    fn test_is_allowed_combination() {
        let reactions = ChatReactions::with_limit(vec![ReactionType::emoji("👍")], 0, true);
        // Specific reaction
        assert!(reactions.is_allowed(&ReactionType::emoji("👍")));
        // Paid
        assert!(reactions.is_allowed(&ReactionType::paid()));
        // Not in list and not all regular
        assert!(!reactions.is_allowed(&ReactionType::emoji("❤️")));
    }

    // === Equality Tests ===

    #[test]
    fn test_eq_same_reactions() {
        let r1 = ChatReactions::with_reactions(vec![
            ReactionType::emoji("👍"),
            ReactionType::emoji("❤️"),
        ]);
        let r2 = ChatReactions::with_reactions(vec![
            ReactionType::emoji("👍"),
            ReactionType::emoji("❤️"),
        ]);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_eq_different_reactions() {
        let r1 = ChatReactions::with_reactions(vec![ReactionType::emoji("👍")]);
        let r2 = ChatReactions::with_reactions(vec![ReactionType::emoji("❤️")]);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_eq_ignores_custom_flag() {
        let r1 = ChatReactions::allow_all(false);
        let mut r2 = ChatReactions::allow_all(false);
        r2.allow_all_custom = true; // This is ignored in PartialEq per TDLib
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_eq_different_limits() {
        let r1 = ChatReactions::with_limit(vec![], 5, false);
        let r2 = ChatReactions::with_limit(vec![], 10, false);
        assert_ne!(r1, r2);
    }

    // === Display Tests ===

    #[test]
    fn test_display_empty() {
        let reactions = ChatReactions::new();
        assert_eq!(format!("{}", reactions), "ChatReactions(empty)");
    }

    #[test]
    fn test_display_all() {
        let reactions = ChatReactions::allow_all(false);
        let s = format!("{}", reactions);
        assert!(s.contains("all_regular"));
    }

    #[test]
    fn test_display_specific() {
        let reactions = ChatReactions::with_reactions(vec![
            ReactionType::emoji("👍"),
            ReactionType::emoji("❤️"),
        ]);
        let s = format!("{}", reactions);
        assert!(s.contains("2 types"));
    }

    #[test]
    fn test_display_paid() {
        let reactions = ChatReactions::with_limit(vec![], 0, true);
        let s = format!("{}", reactions);
        assert!(s.contains("paid"));
    }

    #[test]
    fn test_display_limit() {
        let reactions = ChatReactions::with_limit(vec![], 25, false);
        let s = format!("{}", reactions);
        assert!(s.contains("limit: 25"));
    }

    #[test]
    fn test_display_combined() {
        let reactions = ChatReactions::with_limit(vec![ReactionType::emoji("👍")], 10, true);
        let s = format!("{}", reactions);
        assert!(s.contains("limit: 10"));
        assert!(s.contains("paid"));
        assert!(s.contains("1 types"));
    }

    // === Clone Tests ===

    #[test]
    fn test_clone() {
        let r1 = ChatReactions::with_reactions(vec![ReactionType::emoji("👍")]);
        let r2 = r1.clone();
        assert_eq!(r1, r2);
    }

    // === Boundary Tests ===

    #[test]
    fn test_reactions_limit_boundary_zero() {
        let reactions = ChatReactions::with_limit(vec![], 0, false);
        assert_eq!(reactions.reactions_limit(), 0);
    }

    #[test]
    fn test_reactions_limit_boundary_max() {
        let reactions = ChatReactions::with_limit(vec![], 100, false);
        assert_eq!(reactions.reactions_limit(), 100);
    }

    #[test]
    fn test_empty_when_all_false() {
        let reactions = ChatReactions::new();
        assert!(reactions.is_empty());
    }

    #[test]
    fn test_non_empty_when_reactions_present() {
        let reactions = ChatReactions::with_reactions(vec![ReactionType::emoji("👍")]);
        assert!(!reactions.is_empty());
    }

    #[test]
    fn test_non_empty_when_all_regular() {
        let reactions = ChatReactions::allow_all(false);
        assert!(!reactions.is_empty());
    }

    #[test]
    fn test_non_empty_when_paid_available() {
        let reactions = ChatReactions::with_limit(vec![], 0, true);
        assert!(!reactions.is_empty());
    }

    // === Version Info Tests ===

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-chat-reactions");
    }
}
