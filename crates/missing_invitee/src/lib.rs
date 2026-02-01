// Copyright 2024 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Missing Invitee
//!
//! Types for missing invitees in Telegram group chats.
//!
//! Based on TDLib's MissingInvitee implementation.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use rustgram_types::UserId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A missing invitee in a group chat.
///
/// Represents a user who could not be invited to a group chat,
/// along with information about whether premium would allow the invite.
///
/// # Example
///
/// ```rust
/// use rustgram_missing_invitee::MissingInvitee;
/// use rustgram_types::UserId;
///
/// let user_id = UserId::new(12345).unwrap();
/// let invitee = MissingInvitee::new(user_id, true, false);
/// assert!(invitee.is_valid());
/// assert!(invitee.premium_would_allow_invite());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MissingInvitee {
    /// User ID of the missing invitee
    user_id: UserId,
    /// Whether premium subscription would allow inviting this user
    premium_would_allow_invite: bool,
    /// Whether premium subscription is required for private messages
    premium_required_for_pm: bool,
}

impl MissingInvitee {
    /// Creates a new missing invitee.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID of the missing invitee
    /// * `premium_would_allow_invite` - Whether premium would allow the invite
    /// * `premium_required_for_pm` - Whether premium is required for PM
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_missing_invitee::MissingInvitee;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(12345).unwrap();
    /// let invitee = MissingInvitee::new(user_id, true, false);
    /// assert_eq!(invitee.user_id(), user_id);
    /// ```
    pub fn new(
        user_id: UserId,
        premium_would_allow_invite: bool,
        premium_required_for_pm: bool,
    ) -> Self {
        Self {
            user_id,
            premium_would_allow_invite,
            premium_required_for_pm,
        }
    }

    /// Returns the user ID of the missing invitee.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_missing_invitee::MissingInvitee;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(12345).unwrap();
    /// let invitee = MissingInvitee::new(user_id, true, false);
    /// assert_eq!(invitee.user_id(), user_id);
    /// ```
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Returns `true` if premium subscription would allow inviting this user.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_missing_invitee::MissingInvitee;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(12345).unwrap();
    /// let invitee = MissingInvitee::new(user_id, true, false);
    /// assert!(invitee.premium_would_allow_invite());
    /// ```
    pub fn premium_would_allow_invite(&self) -> bool {
        self.premium_would_allow_invite
    }

    /// Returns `true` if premium subscription is required for private messages.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_missing_invitee::MissingInvitee;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(12345).unwrap();
    /// let invitee = MissingInvitee::new(user_id, false, true);
    /// assert!(invitee.premium_required_for_pm());
    /// ```
    pub fn premium_required_for_pm(&self) -> bool {
        self.premium_required_for_pm
    }

    /// Returns `true` if this missing invitee is valid (has a valid user ID).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_missing_invitee::MissingInvitee;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(12345).unwrap();
    /// let invitee = MissingInvitee::new(user_id, true, false);
    /// assert!(invitee.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.user_id.is_valid()
    }
}

impl fmt::Display for MissingInvitee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} {} {}]",
            self.user_id, self.premium_would_allow_invite, self.premium_required_for_pm
        )
    }
}

/// A collection of missing invitees.
///
/// Represents multiple users who could not be invited to a group chat.
///
/// # Example
///
/// ```rust
/// use rustgram_missing_invitee::{MissingInvitee, MissingInvitees};
/// use rustgram_types::UserId;
///
/// let user1 = UserId::new(12345).unwrap();
/// let user2 = UserId::new(67890).unwrap();
///
/// let invitee1 = MissingInvitee::new(user1, true, false);
/// let invitee2 = MissingInvitee::new(user2, false, true);
///
/// let invitees = MissingInvitees::new(vec![invitee1, invitee2]);
/// assert_eq!(invitees.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct MissingInvitees {
    /// List of missing invitees
    invitees: Vec<MissingInvitee>,
}

impl MissingInvitees {
    /// Creates a new collection of missing invitees.
    ///
    /// # Arguments
    ///
    /// * `invitees` - Vector of missing invitees
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_missing_invitee::{MissingInvitee, MissingInvitees};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(12345).unwrap();
    /// let invitee = MissingInvitee::new(user_id, true, false);
    /// let invitees = MissingInvitees::new(vec![invitee]);
    /// assert_eq!(invitees.len(), 1);
    /// ```
    pub fn new(invitees: Vec<MissingInvitee>) -> Self {
        // Filter out invalid invitees
        let valid_invitees = invitees
            .into_iter()
            .filter(|invitee| invitee.is_valid())
            .collect();

        Self {
            invitees: valid_invitees,
        }
    }

    /// Returns an iterator over the invitees.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_missing_invitee::{MissingInvitee, MissingInvitees};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(12345).unwrap();
    /// let invitee = MissingInvitee::new(user_id, true, false);
    /// let invitees = MissingInvitees::new(vec![invitee]);
    ///
    /// for invitee in invitees.iter() {
    ///     assert!(invitee.is_valid());
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &MissingInvitee> {
        self.invitees.iter()
    }

    /// Returns the number of invitees.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_missing_invitee::{MissingInvitee, MissingInvitees};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(12345).unwrap();
    /// let invitee = MissingInvitee::new(user_id, true, false);
    /// let invitees = MissingInvitees::new(vec![invitee]);
    /// assert_eq!(invitees.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.invitees.len()
    }

    /// Returns `true` if there are no invitees.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_missing_invitee::MissingInvitees;
    ///
    /// let invitees = MissingInvitees::new(vec![]);
    /// assert!(invitees.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.invitees.is_empty()
    }

    /// Adds a new invitee to the collection.
    ///
    /// # Arguments
    ///
    /// * `invitee` - The invitee to add
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_missing_invitee::{MissingInvitee, MissingInvitees};
    /// use rustgram_types::UserId;
    ///
    /// let mut invitees = MissingInvitees::new(vec![]);
    /// let user_id = UserId::new(12345).unwrap();
    /// let invitee = MissingInvitee::new(user_id, true, false);
    ///
    /// invitees.add(invitee);
    /// assert_eq!(invitees.len(), 1);
    /// ```
    pub fn add(&mut self, invitee: MissingInvitee) {
        if invitee.is_valid() {
            self.invitees.push(invitee);
        }
    }
}

impl fmt::Display for MissingInvitees {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.invitees)
    }
}

impl IntoIterator for MissingInvitees {
    type Item = MissingInvitee;
    type IntoIter = std::vec::IntoIter<MissingInvitee>;

    fn into_iter(self) -> Self::IntoIter {
        self.invitees.into_iter()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn test_missing_invitee_new() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);

        assert_eq!(invitee.user_id(), user_id);
        assert!(invitee.premium_would_allow_invite());
        assert!(!invitee.premium_required_for_pm());
    }

    #[test]
    fn test_missing_invitee_is_valid() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);
        assert!(invitee.is_valid());
    }

    #[test]
    fn test_missing_invitee_invalid_user_id() {
        let user_id = UserId(0); // Invalid user ID
        let invitee = MissingInvitee::new(user_id, true, false);
        assert!(!invitee.is_valid());
    }

    #[test]
    fn test_missing_invitee_premium_would_allow() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);
        assert!(invitee.premium_would_allow_invite());
    }

    #[test]
    fn test_missing_invitee_premium_required_for_pm() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, false, true);
        assert!(invitee.premium_required_for_pm());
    }

    #[test]
    fn test_missing_invitee_both_flags() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, true);
        assert!(invitee.premium_would_allow_invite());
        assert!(invitee.premium_required_for_pm());
    }

    #[test]
    fn test_missing_invitee_equality() {
        let user_id = UserId::new(12345).unwrap();
        let invitee1 = MissingInvitee::new(user_id, true, false);
        let invitee2 = MissingInvitee::new(user_id, true, false);
        assert_eq!(invitee1, invitee2);

        let invitee3 = MissingInvitee::new(user_id, false, true);
        assert_ne!(invitee1, invitee3);
    }

    #[test]
    fn test_missing_invitee_clone() {
        let user_id = UserId::new(12345).unwrap();
        let invitee1 = MissingInvitee::new(user_id, true, false);
        let invitee2 = invitee1.clone();
        assert_eq!(invitee1, invitee2);
    }

    #[test]
    fn test_missing_invitee_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let user_id = UserId::new(12345).unwrap();
        let invitee1 = MissingInvitee::new(user_id, true, false);
        let invitee2 = MissingInvitee::new(user_id, true, false);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        invitee1.hash(&mut hasher1);
        invitee2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_missing_invitee_serialization() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);
        let json = serde_json::to_string(&invitee).unwrap();
        let parsed: MissingInvitee = serde_json::from_str(&json).unwrap();
        assert_eq!(invitee, parsed);
    }

    #[test]
    fn test_missing_invitees_new() {
        let user_id1 = UserId::new(12345).unwrap();
        let user_id2 = UserId::new(67890).unwrap();

        let invitee1 = MissingInvitee::new(user_id1, true, false);
        let invitee2 = MissingInvitee::new(user_id2, false, true);

        let invitees = MissingInvitees::new(vec![invitee1.clone(), invitee2.clone()]);
        assert_eq!(invitees.len(), 2);
    }

    #[test]
    fn test_missing_invitees_filter_invalid() {
        let user_id1 = UserId::new(12345).unwrap();
        let user_id2 = UserId(0); // Invalid

        let invitee1 = MissingInvitee::new(user_id1, true, false);
        let invitee2 = MissingInvitee::new(user_id2, true, false);

        let invitees = MissingInvitees::new(vec![invitee1, invitee2]);
        assert_eq!(invitees.len(), 1); // Only the valid one
    }

    #[test]
    fn test_missing_invitees_iter() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);

        let invitees = MissingInvitees::new(vec![invitee]);
        assert_eq!(invitees.iter().count(), 1);
    }

    #[test]
    fn test_missing_invitees_is_empty() {
        let invitees = MissingInvitees::new(vec![]);
        assert!(invitees.is_empty());

        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);
        let invitees = MissingInvitees::new(vec![invitee]);
        assert!(!invitees.is_empty());
    }

    #[test]
    fn test_missing_invitees_add() {
        let mut invitees = MissingInvitees::new(vec![]);
        assert_eq!(invitees.len(), 0);

        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);
        invitees.add(invitee);
        assert_eq!(invitees.len(), 1);
    }

    #[test]
    fn test_missing_invitees_add_invalid() {
        let mut invitees = MissingInvitees::new(vec![]);
        let user_id = UserId(0); // Invalid
        let invitee = MissingInvitee::new(user_id, true, false);
        invitees.add(invitee);
        assert_eq!(invitees.len(), 0); // Should not be added
    }

    #[test]
    fn test_missing_invitees_default() {
        let invitees = MissingInvitees::default();
        assert!(invitees.is_empty());
        assert_eq!(invitees.len(), 0);
    }

    #[test]
    fn test_missing_invitees_clone() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);
        let invitees1 = MissingInvitees::new(vec![invitee]);
        let invitees2 = invitees1.clone();
        assert_eq!(invitees1, invitees2);
    }

    #[test]
    fn test_missing_invitees_into_iter() {
        let user_id1 = UserId::new(12345).unwrap();
        let user_id2 = UserId::new(67890).unwrap();

        let invitee1 = MissingInvitee::new(user_id1, true, false);
        let invitee2 = MissingInvitee::new(user_id2, false, true);

        let invitees = MissingInvitees::new(vec![invitee1, invitee2]);
        let collected: Vec<_> = invitees.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }

    #[test]
    fn test_missing_invitees_serialization() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);
        let invitees = MissingInvitees::new(vec![invitee]);

        let json = serde_json::to_string(&invitees).unwrap();
        let parsed: MissingInvitees = serde_json::from_str(&json).unwrap();
        assert_eq!(invitees, parsed);
    }

    #[test]
    fn test_missing_invitees_equality() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);

        let invitees1 = MissingInvitees::new(vec![invitee.clone()]);
        let invitees2 = MissingInvitees::new(vec![invitee]);
        assert_eq!(invitees1, invitees2);
    }

    #[test]
    fn test_missing_invitees_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);

        let invitees1 = MissingInvitees::new(vec![invitee.clone()]);
        let invitees2 = MissingInvitees::new(vec![invitee]);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        invitees1.hash(&mut hasher1);
        invitees2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_display_missing_invitee() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);
        let display = format!("{}", invitee);
        assert!(display.contains("12345"));
        assert!(display.contains("true"));
        assert!(display.contains("false"));
    }

    #[test]
    fn test_display_missing_invitees() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);
        let invitees = MissingInvitees::new(vec![invitee]);
        // Just ensure it can be displayed
        let _display = format!("{}", invitees);
    }

    #[test]
    fn test_missing_invitee_needs_premium_for_invite() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, true, false);
        assert!(invitee.premium_would_allow_invite());
        assert!(!invitee.premium_required_for_pm());
    }

    #[test]
    fn test_missing_invitee_needs_premium_for_pm() {
        let user_id = UserId::new(12345).unwrap();
        let invitee = MissingInvitee::new(user_id, false, true);
        assert!(!invitee.premium_would_allow_invite());
        assert!(invitee.premium_required_for_pm());
    }
}
