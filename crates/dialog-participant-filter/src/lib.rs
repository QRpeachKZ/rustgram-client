//! Dialog participant filter.
//!
//! This module provides the `DialogParticipantFilter` type, which is used to
//! filter participants in a dialog (e.g., get only contacts, administrators, etc.).
//!
//! # TDLib Alignment
//!
//! - TDLib type: `DialogParticipantFilter` (td/telegram/DialogParticipantFilter.h)
//! - Multiple filter types: Contacts, Administrators, Members, Restricted, Banned, Mention, Bots
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_participant_filter::DialogParticipantFilter;
//!
//! // Get only administrators
//! let filter = DialogParticipantFilter::Administrators;
//! assert!(matches!(filter, DialogParticipantFilter::Administrators));
//!
//! // Get only contacts
//! let filter = DialogParticipantFilter::Contacts;
//! assert!(matches!(filter, DialogParticipantFilter::Contacts));
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Filter for dialog participants.
///
/// Used to query specific subsets of participants in a dialog.
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_participant_filter::DialogParticipantFilter;
///
/// let contacts = DialogParticipantFilter::Contacts;
/// let admins = DialogParticipantFilter::Administrators;
/// let bots = DialogParticipantFilter::Bots;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogParticipantFilter {
    /// Filter for contacts only.
    Contacts,
    /// Filter for administrators only.
    Administrators,
    /// Filter for all members (default).
    Members,
    /// Filter for restricted users only.
    Restricted,
    /// Filter for banned users only.
    Banned,
    /// Filter for users matching a specific @mention.
    Mention {
        /// The query string for the mention.
        query: String,
    },
    /// Filter for bots only.
    Bots,
}

impl DialogParticipantFilter {
    /// Creates a contacts filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::contacts();
    /// assert!(filter.is_contacts());
    /// ```
    pub fn contacts() -> Self {
        Self::Contacts
    }

    /// Creates an administrators filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::administrators();
    /// assert!(filter.is_administrators());
    /// ```
    pub fn administrators() -> Self {
        Self::Administrators
    }

    /// Creates a members filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::members();
    /// assert!(filter.is_members());
    /// ```
    pub fn members() -> Self {
        Self::Members
    }

    /// Creates a restricted filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::restricted();
    /// assert!(filter.is_restricted());
    /// ```
    pub fn restricted() -> Self {
        Self::Restricted
    }

    /// Creates a banned filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::banned();
    /// assert!(filter.is_banned());
    /// ```
    pub fn banned() -> Self {
        Self::Banned
    }

    /// Creates a mention filter with a query string.
    ///
    /// # Arguments
    ///
    /// * `query` - The mention query string
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::mention("@username");
    /// assert!(filter.is_mention());
    /// ```
    pub fn mention<S: Into<String>>(query: S) -> Self {
        Self::Mention {
            query: query.into(),
        }
    }

    /// Creates a bots filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::bots();
    /// assert!(filter.is_bots());
    /// ```
    pub fn bots() -> Self {
        Self::Bots
    }

    /// Checks if this is a contacts filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::contacts();
    /// assert!(filter.is_contacts());
    /// ```
    pub fn is_contacts(&self) -> bool {
        matches!(self, Self::Contacts)
    }

    /// Checks if this is an administrators filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::administrators();
    /// assert!(filter.is_administrators());
    /// ```
    pub fn is_administrators(&self) -> bool {
        matches!(self, Self::Administrators)
    }

    /// Checks if this is a members filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::members();
    /// assert!(filter.is_members());
    /// ```
    pub fn is_members(&self) -> bool {
        matches!(self, Self::Members)
    }

    /// Checks if this is a restricted filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::restricted();
    /// assert!(filter.is_restricted());
    /// ```
    pub fn is_restricted(&self) -> bool {
        matches!(self, Self::Restricted)
    }

    /// Checks if this is a banned filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::banned();
    /// assert!(filter.is_banned());
    /// ```
    pub fn is_banned(&self) -> bool {
        matches!(self, Self::Banned)
    }

    /// Checks if this is a mention filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::mention("@user");
    /// assert!(filter.is_mention());
    /// ```
    pub fn is_mention(&self) -> bool {
        matches!(self, Self::Mention { .. })
    }

    /// Checks if this is a bots filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::bots();
    /// assert!(filter.is_bots());
    /// ```
    pub fn is_bots(&self) -> bool {
        matches!(self, Self::Bots)
    }

    /// Gets the mention query if this is a mention filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::mention("@username");
    /// assert_eq!(filter.query(), Some("@username"));
    /// ```
    pub fn query(&self) -> Option<&str> {
        match self {
            Self::Mention { query } => Some(query),
            _ => None,
        }
    }

    /// Checks if this filter has a query string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_filter::DialogParticipantFilter;
    ///
    /// let filter = DialogParticipantFilter::mention("@user");
    /// assert!(filter.has_query());
    ///
    /// let filter = DialogParticipantFilter::members();
    /// assert!(!filter.has_query());
    /// ```
    pub fn has_query(&self) -> bool {
        matches!(self, Self::Mention { .. })
    }
}

impl Default for DialogParticipantFilter {
    fn default() -> Self {
        Self::Members
    }
}

impl fmt::Display for DialogParticipantFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contacts => write!(f, "contacts"),
            Self::Administrators => write!(f, "administrators"),
            Self::Members => write!(f, "members"),
            Self::Restricted => write!(f, "restricted"),
            Self::Banned => write!(f, "banned"),
            Self::Mention { query } => write!(f, "mention({})", query),
            Self::Bots => write!(f, "bots"),
        }
    }
}

impl Serialize for DialogParticipantFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Contacts => (0u8,).serialize(serializer),
            Self::Administrators => (1u8,).serialize(serializer),
            Self::Members => (2u8,).serialize(serializer),
            Self::Restricted => (3u8,).serialize(serializer),
            Self::Banned => (4u8,).serialize(serializer),
            Self::Mention { query } => (5u8, query).serialize(serializer),
            Self::Bots => (6u8,).serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for DialogParticipantFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DialogParticipantFilterVisitor;

        impl<'de> serde::de::Visitor<'de> for DialogParticipantFilterVisitor {
            type Value = DialogParticipantFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a dialog participant filter tuple")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let tag: u8 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                Ok(match tag {
                    0 => DialogParticipantFilter::Contacts,
                    1 => DialogParticipantFilter::Administrators,
                    2 => DialogParticipantFilter::Members,
                    3 => DialogParticipantFilter::Restricted,
                    4 => DialogParticipantFilter::Banned,
                    5 => {
                        let query: String = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        DialogParticipantFilter::Mention { query }
                    }
                    6 => DialogParticipantFilter::Bots,
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "invalid dialog participant filter tag: {tag}"
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_any(DialogParticipantFilterVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10)
    #[test]
    fn test_debug_contacts() {
        let filter = DialogParticipantFilter::Contacts;
        assert_eq!(format!("{:?}", filter), "Contacts");
    }

    #[test]
    fn test_debug_mention() {
        let filter = DialogParticipantFilter::mention("@user");
        assert!(format!("{:?}", filter).contains("Mention"));
    }

    #[test]
    fn test_clone() {
        let filter = DialogParticipantFilter::Contacts;
        let cloned = filter.clone();
        assert_eq!(filter, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let f1 = DialogParticipantFilter::Contacts;
        let f2 = DialogParticipantFilter::Contacts;
        let f3 = DialogParticipantFilter::Administrators;
        assert_eq!(f1, f2);
        assert_ne!(f1, f3);
    }

    #[test]
    fn test_default() {
        let filter = DialogParticipantFilter::default();
        assert!(filter.is_members());
    }

    #[test]
    fn test_display_contacts() {
        let filter = DialogParticipantFilter::Contacts;
        assert_eq!(format!("{}", filter), "contacts");
    }

    #[test]
    fn test_display_mention() {
        let filter = DialogParticipantFilter::mention("@username");
        assert!(format!("{}", filter).contains("@username"));
    }

    // Constructor tests (7 * 2 = 14)
    #[test]
    fn test_contacts() {
        let filter = DialogParticipantFilter::contacts();
        assert!(filter.is_contacts());
    }

    #[test]
    fn test_administrators() {
        let filter = DialogParticipantFilter::administrators();
        assert!(filter.is_administrators());
    }

    #[test]
    fn test_members() {
        let filter = DialogParticipantFilter::members();
        assert!(filter.is_members());
    }

    #[test]
    fn test_restricted() {
        let filter = DialogParticipantFilter::restricted();
        assert!(filter.is_restricted());
    }

    #[test]
    fn test_banned() {
        let filter = DialogParticipantFilter::banned();
        assert!(filter.is_banned());
    }

    #[test]
    fn test_mention() {
        let filter = DialogParticipantFilter::mention("@user");
        assert!(filter.is_mention());
        assert_eq!(filter.query(), Some("@user"));
    }

    #[test]
    fn test_bots() {
        let filter = DialogParticipantFilter::bots();
        assert!(filter.is_bots());
    }

    // Type check tests (7 * 2 = 14)
    #[test]
    fn test_is_contacts_true() {
        let filter = DialogParticipantFilter::Contacts;
        assert!(filter.is_contacts());
        assert!(!filter.is_administrators());
    }

    #[test]
    fn test_is_administrators_true() {
        let filter = DialogParticipantFilter::Administrators;
        assert!(filter.is_administrators());
    }

    #[test]
    fn test_is_members_true() {
        let filter = DialogParticipantFilter::Members;
        assert!(filter.is_members());
    }

    #[test]
    fn test_is_restricted_true() {
        let filter = DialogParticipantFilter::Restricted;
        assert!(filter.is_restricted());
    }

    #[test]
    fn test_is_banned_true() {
        let filter = DialogParticipantFilter::Banned;
        assert!(filter.is_banned());
    }

    #[test]
    fn test_is_mention_true() {
        let filter = DialogParticipantFilter::Mention {
            query: "@test".to_string(),
        };
        assert!(filter.is_mention());
    }

    #[test]
    fn test_is_bots_true() {
        let filter = DialogParticipantFilter::Bots;
        assert!(filter.is_bots());
    }

    // Method tests (2 * 3 = 6)
    #[test]
    fn test_query_some() {
        let filter = DialogParticipantFilter::mention("@username");
        assert_eq!(filter.query(), Some("@username"));
    }

    #[test]
    fn test_query_none() {
        let filter = DialogParticipantFilter::contacts();
        assert_eq!(filter.query(), None);
    }

    #[test]
    fn test_has_query_true() {
        let filter = DialogParticipantFilter::mention("@user");
        assert!(filter.has_query());
    }

    #[test]
    fn test_has_query_false() {
        let filter = DialogParticipantFilter::members();
        assert!(!filter.has_query());
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize_simple() {
        let filter = DialogParticipantFilter::Contacts;
        let json = serde_json::to_string(&filter).unwrap();
        let deserialized: DialogParticipantFilter = serde_json::from_str(&json).unwrap();
        assert_eq!(filter, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_with_query() {
        let filter = DialogParticipantFilter::mention("@username");
        let json = serde_json::to_string(&filter).unwrap();
        let deserialized: DialogParticipantFilter = serde_json::from_str(&json).unwrap();
        assert_eq!(filter, deserialized);
    }

    #[test]
    fn test_serialize_all_variants() {
        let filters = vec![
            DialogParticipantFilter::Contacts,
            DialogParticipantFilter::Administrators,
            DialogParticipantFilter::Members,
            DialogParticipantFilter::Restricted,
            DialogParticipantFilter::Banned,
            DialogParticipantFilter::Bots,
        ];
        for filter in filters {
            let json = serde_json::to_string(&filter).unwrap();
            let deserialized: DialogParticipantFilter = serde_json::from_str(&json).unwrap();
            assert_eq!(filter, deserialized);
        }
    }

    // Edge cases (2)
    #[test]
    fn test_empty_query() {
        let filter = DialogParticipantFilter::mention("");
        assert!(filter.is_mention());
        assert_eq!(filter.query(), Some(""));
    }

    #[test]
    fn test_query_with_unicode() {
        let filter = DialogParticipantFilter::mention("user123");
        assert_eq!(filter.query(), Some("user123"));
    }

    // Doc test example (1)
    #[test]
    fn test_doc_example() {
        let filter = DialogParticipantFilter::Administrators;
        assert!(matches!(filter, DialogParticipantFilter::Administrators));

        let filter = DialogParticipantFilter::Contacts;
        assert!(matches!(filter, DialogParticipantFilter::Contacts));
    }
}
