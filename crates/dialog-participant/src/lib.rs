//! Dialog participant information.
//!
//! This module provides the `DialogParticipant` type, which represents
//! a participant in a group chat or channel.
//!
//! # TDLib Alignment
//!
//! - TDLib type: `DialogParticipant` (td/telegram/DialogParticipant.h)
//! - Contains dialog_id, inviter_user_id, joined_date, and status
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
//! use rustgram_types::UserId;
//!
//! // Create a participant
//! let user_id = UserId::new(123).unwrap();
//! let inviter_id = UserId::new(456).unwrap();
//! let participant = DialogParticipant::new(user_id, Some(inviter_id), 1234567890, ParticipantStatus::Member);
//! ```

use rustgram_types::{DialogId, UserId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Participant status in a dialog.
///
/// Represents the role and permissions of a participant.
/// This is a simplified version of TDLib's DialogParticipantStatus.
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_participant::ParticipantStatus;
///
/// let creator = ParticipantStatus::Creator;
/// let admin = ParticipantStatus::Administrator;
/// let member = ParticipantStatus::Member;
/// let left = ParticipantStatus::Left;
/// let banned = ParticipantStatus::Banned { until_date: 0 };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ParticipantStatus {
    /// Chat creator (owner).
    Creator,
    /// Administrator with permissions.
    Administrator,
    /// Regular member.
    Member,
    /// User left the chat.
    #[default]
    Left,
    /// User was banned from the chat.
    Banned {
        /// Unix timestamp when the ban will be lifted (0 = permanent).
        until_date: i32,
    },
    /// Restricted user with limited permissions.
    Restricted {
        /// Unix timestamp when restrictions will be lifted (0 = permanent).
        until_date: i32,
    },
}

impl ParticipantStatus {
    /// Checks if the participant is a creator.
    pub fn is_creator(&self) -> bool {
        matches!(self, Self::Creator)
    }

    /// Checks if the participant is an administrator.
    pub fn is_administrator(&self) -> bool {
        matches!(self, Self::Creator | Self::Administrator)
    }

    /// Checks if the participant is a member.
    pub fn is_member(&self) -> bool {
        matches!(
            self,
            Self::Creator | Self::Administrator | Self::Member | Self::Restricted { .. }
        )
    }

    /// Checks if the participant has left.
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left)
    }

    /// Checks if the participant is banned.
    pub fn is_banned(&self) -> bool {
        matches!(self, Self::Banned { .. })
    }

    /// Checks if the participant is restricted.
    pub fn is_restricted(&self) -> bool {
        matches!(self, Self::Restricted { .. })
    }
}

impl fmt::Display for ParticipantStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Creator => write!(f, "creator"),
            Self::Administrator => write!(f, "administrator"),
            Self::Member => write!(f, "member"),
            Self::Left => write!(f, "left"),
            Self::Banned { until_date } => {
                if *until_date == 0 {
                    write!(f, "banned permanently")
                } else {
                    write!(f, "banned until {}", until_date)
                }
            }
            Self::Restricted { until_date } => {
                if *until_date == 0 {
                    write!(f, "restricted permanently")
                } else {
                    write!(f, "restricted until {}", until_date)
                }
            }
        }
    }
}

impl Serialize for ParticipantStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Creator => (0u8,).serialize(serializer),
            Self::Administrator => (1u8,).serialize(serializer),
            Self::Member => (2u8,).serialize(serializer),
            Self::Left => (3u8,).serialize(serializer),
            Self::Banned { until_date } => (4u8, until_date).serialize(serializer),
            Self::Restricted { until_date } => (5u8, until_date).serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ParticipantStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ParticipantStatusVisitor;

        impl<'de> serde::de::Visitor<'de> for ParticipantStatusVisitor {
            type Value = ParticipantStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a participant status tuple")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let tag: u8 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                Ok(match tag {
                    0 => ParticipantStatus::Creator,
                    1 => ParticipantStatus::Administrator,
                    2 => ParticipantStatus::Member,
                    3 => ParticipantStatus::Left,
                    4 => {
                        let until_date: i32 = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        ParticipantStatus::Banned { until_date }
                    }
                    5 => {
                        let until_date: i32 = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        ParticipantStatus::Restricted { until_date }
                    }
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "invalid participant status tag: {tag}"
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_any(ParticipantStatusVisitor)
    }
}

/// Dialog participant information.
///
/// Contains information about a participant in a group chat or channel.
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
/// use rustgram_types::UserId;
///
/// let user_id = UserId::new(123).unwrap();
/// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
/// assert_eq!(participant.user_id(), user_id);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogParticipant {
    /// The dialog ID of the participant.
    dialog_id: DialogId,
    /// The user ID who invited this participant (if any).
    inviter_user_id: Option<UserId>,
    /// Unix timestamp when the participant joined.
    joined_date: i32,
    /// The participant's status.
    status: ParticipantStatus,
}

impl DialogParticipant {
    /// Creates a new dialog participant.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID of the participant
    /// * `inviter_user_id` - Optional user ID of who invited them
    /// * `joined_date` - Unix timestamp when they joined
    /// * `status` - Their status in the chat
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let participant = DialogParticipant::new(user_id, None, 1234567890, ParticipantStatus::Member);
    /// ```
    pub fn new(
        user_id: UserId,
        inviter_user_id: Option<UserId>,
        joined_date: i32,
        status: ParticipantStatus,
    ) -> Self {
        Self {
            dialog_id: DialogId::from_user(user_id),
            inviter_user_id,
            joined_date,
            status,
        }
    }

    /// Creates a participant who has left the chat.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant::DialogParticipant;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let left = DialogParticipant::left(user_id);
    /// assert!(left.status().is_left());
    /// ```
    pub fn left(user_id: UserId) -> Self {
        Self {
            dialog_id: DialogId::from_user(user_id),
            inviter_user_id: None,
            joined_date: 0,
            status: ParticipantStatus::Left,
        }
    }

    /// Returns the dialog ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// assert_eq!(participant.dialog_id(), &DialogId::from_user(user_id));
    /// ```
    pub fn dialog_id(&self) -> &DialogId {
        &self.dialog_id
    }

    /// Returns the user ID.
    ///
    /// Returns None if this is not a user dialog.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// assert_eq!(participant.user_id(), Some(user_id));
    /// ```
    pub fn user_id(&self) -> Option<UserId> {
        self.dialog_id.get_user_id()
    }

    /// Returns the inviter user ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let inviter_id = UserId::new(456).unwrap();
    /// let participant = DialogParticipant::new(user_id, Some(inviter_id), 0, ParticipantStatus::Member);
    /// assert_eq!(participant.inviter_user_id(), Some(inviter_id));
    /// ```
    pub fn inviter_user_id(&self) -> Option<UserId> {
        self.inviter_user_id
    }

    /// Returns the joined date.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let participant = DialogParticipant::new(user_id, None, 1234567890, ParticipantStatus::Member);
    /// assert_eq!(participant.joined_date(), 1234567890);
    /// ```
    pub fn joined_date(&self) -> i32 {
        self.joined_date
    }

    /// Returns the participant status.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Creator);
    /// assert!(participant.status().is_creator());
    /// ```
    pub fn status(&self) -> &ParticipantStatus {
        &self.status
    }

    /// Sets the participant status.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let mut participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// participant.set_status(ParticipantStatus::Administrator);
    /// assert!(participant.status().is_administrator());
    /// ```
    pub fn set_status(&mut self, status: ParticipantStatus) {
        self.status = status;
    }

    /// Checks if this participant is valid.
    ///
    /// A participant is valid if they have a valid dialog ID and
    /// are not in the "Left" state (unless specifically created as left).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// assert!(participant.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.dialog_id.is_valid()
    }
}

impl fmt::Display for DialogParticipant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Participant(dialog_id={}, status={})",
            self.dialog_id, self.status
        )
    }
}

impl Serialize for DialogParticipant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (
            &self.dialog_id,
            &self.inviter_user_id,
            self.joined_date,
            &self.status,
        )
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DialogParticipant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (dialog_id, inviter_user_id, joined_date, status) =
            <(DialogId, Option<UserId>, i32, ParticipantStatus)>::deserialize(deserializer)?;

        Ok(Self {
            dialog_id,
            inviter_user_id,
            joined_date,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ParticipantStatus tests (15)
    #[test]
    fn test_status_creator() {
        let status = ParticipantStatus::Creator;
        assert!(status.is_creator());
        assert!(status.is_administrator());
        assert!(status.is_member());
        assert!(!status.is_left());
        assert!(!status.is_banned());
    }

    #[test]
    fn test_status_administrator() {
        let status = ParticipantStatus::Administrator;
        assert!(!status.is_creator());
        assert!(status.is_administrator());
        assert!(status.is_member());
        assert!(!status.is_left());
    }

    #[test]
    fn test_status_member() {
        let status = ParticipantStatus::Member;
        assert!(!status.is_creator());
        assert!(!status.is_administrator());
        assert!(status.is_member());
        assert!(!status.is_left());
    }

    #[test]
    fn test_status_left() {
        let status = ParticipantStatus::Left;
        assert!(!status.is_member());
        assert!(status.is_left());
    }

    #[test]
    fn test_status_banned() {
        let status = ParticipantStatus::Banned { until_date: 0 };
        assert!(!status.is_member());
        assert!(status.is_banned());
    }

    #[test]
    fn test_status_restricted() {
        let status = ParticipantStatus::Restricted { until_date: 0 };
        assert!(status.is_member());
        assert!(status.is_restricted());
    }

    // DialogParticipant tests (35)
    #[test]
    fn test_new() {
        let user_id = UserId::new(123).unwrap();
        let inviter_id = UserId::new(456).unwrap();
        let participant = DialogParticipant::new(
            user_id,
            Some(inviter_id),
            1234567890,
            ParticipantStatus::Member,
        );
        assert_eq!(participant.user_id(), Some(user_id));
        assert_eq!(participant.inviter_user_id(), Some(inviter_id));
        assert_eq!(participant.joined_date(), 1234567890);
    }

    #[test]
    fn test_left() {
        let user_id = UserId::new(123).unwrap();
        let participant = DialogParticipant::left(user_id);
        assert!(participant.status().is_left());
        assert_eq!(participant.joined_date(), 0);
    }

    #[test]
    fn test_dialog_id() {
        let user_id = UserId::new(123).unwrap();
        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        assert_eq!(participant.dialog_id(), &DialogId::from_user(user_id));
    }

    #[test]
    fn test_user_id() {
        let user_id = UserId::new(123).unwrap();
        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        assert_eq!(participant.user_id(), Some(user_id));
    }

    #[test]
    fn test_inviter_user_id_some() {
        let user_id = UserId::new(123).unwrap();
        let inviter_id = UserId::new(456).unwrap();
        let participant =
            DialogParticipant::new(user_id, Some(inviter_id), 0, ParticipantStatus::Member);
        assert_eq!(participant.inviter_user_id(), Some(inviter_id));
    }

    #[test]
    fn test_inviter_user_id_none() {
        let user_id = UserId::new(123).unwrap();
        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        assert_eq!(participant.inviter_user_id(), None);
    }

    #[test]
    fn test_joined_date() {
        let user_id = UserId::new(123).unwrap();
        let participant =
            DialogParticipant::new(user_id, None, 1609459200, ParticipantStatus::Member);
        assert_eq!(participant.joined_date(), 1609459200);
    }

    #[test]
    fn test_status() {
        let user_id = UserId::new(123).unwrap();
        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Creator);
        assert!(participant.status().is_creator());
    }

    #[test]
    fn test_set_status() {
        let user_id = UserId::new(123).unwrap();
        let mut participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        participant.set_status(ParticipantStatus::Administrator);
        assert!(participant.status().is_administrator());
    }

    #[test]
    fn test_is_valid() {
        let user_id = UserId::new(123).unwrap();
        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        assert!(participant.is_valid());
    }

    #[test]
    fn test_clone() {
        let user_id = UserId::new(123).unwrap();
        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        let cloned = participant.clone();
        assert_eq!(participant, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let user_id = UserId::new(123).unwrap();
        let p1 = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        let p2 = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_display() {
        let user_id = UserId::new(123).unwrap();
        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        let display = format!("{}", participant);
        assert!(display.contains("Participant"));
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize_participant() {
        let user_id = UserId::new(123).unwrap();
        let inviter_id = UserId::new(456).unwrap();
        let participant = DialogParticipant::new(
            user_id,
            Some(inviter_id),
            1234567890,
            ParticipantStatus::Administrator,
        );

        let json = serde_json::to_string(&participant).unwrap();
        let deserialized: DialogParticipant = serde_json::from_str(&json).unwrap();
        assert_eq!(participant, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_status() {
        let status = ParticipantStatus::Banned { until_date: 12345 };
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: ParticipantStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

    // Status display tests (3)
    #[test]
    fn test_status_display_creator() {
        let status = ParticipantStatus::Creator;
        assert_eq!(format!("{}", status), "creator");
    }

    #[test]
    fn test_status_display_banned_permanent() {
        let status = ParticipantStatus::Banned { until_date: 0 };
        assert!(format!("{}", status).contains("permanently"));
    }

    #[test]
    fn test_status_display_banned_temporary() {
        let status = ParticipantStatus::Banned { until_date: 12345 };
        assert!(format!("{}", status).contains("12345"));
    }
}
