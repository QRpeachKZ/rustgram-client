// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Dialog administrator types.
//!
//! This module provides the [`DialogAdministrator`] type which represents
//! a chat administrator with their custom rank/title and owner status.

use rustgram_types::UserId;
use std::fmt;

/// Dialog administrator information.
///
/// Represents an administrator in a Telegram chat/group/channel with their
/// custom title (rank) and whether they are the owner/creator.
///
/// # TDLib Correspondence
///
/// | Rust type | TDLib type | File |
/// |-----------|------------|------|
/// | [`DialogAdministrator`] | `td::DialogAdministrator` | `DialogAdministrator.h/cpp` |
///
/// # Examples
///
/// ```
/// use rustgram_dialog_administrator::DialogAdministrator;
/// use rustgram_types::UserId;
///
/// // Create an administrator with a custom rank
/// let admin = DialogAdministrator::new(
///     UserId(12345678),
///     "Moderator".to_string(),
///     false,
/// );
///
/// assert_eq!(admin.user_id(), UserId(12345678));
/// assert_eq!(admin.rank(), "Moderator");
/// assert!(!admin.is_creator());
/// ```
///
/// # TL Correspondence
///
/// ### TD API
///
/// ```text
/// chatAdministrator
///   user_id: int53
///   custom_title: string
///   is_owner: Bool
/// = ChatAdministrator
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct DialogAdministrator {
    /// User ID of the administrator.
    user_id: UserId,
    /// Custom rank/title (e.g., "Moderator", "Admin").
    rank: String,
    /// Whether this user is the creator/owner of the chat.
    is_creator: bool,
}

impl DialogAdministrator {
    /// Creates a new [`DialogAdministrator`].
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the user who is an administrator
    /// * `rank` - Custom title/rank displayed in the chat
    /// * `is_creator` - Whether this user is the chat owner/creator
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_administrator::DialogAdministrator;
    /// use rustgram_types::UserId;
    ///
    /// let admin = DialogAdministrator::new(
    ///     UserId(12345678),
    ///     "Super Admin".to_string(),
    ///     false,
    /// );
    /// ```
    #[must_use]
    pub const fn new(user_id: UserId, rank: String, is_creator: bool) -> Self {
        Self {
            user_id,
            rank,
            is_creator,
        }
    }

    /// Returns the user ID of this administrator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_administrator::DialogAdministrator;
    /// use rustgram_types::UserId;
    ///
    /// let admin = DialogAdministrator::new(
    ///     UserId(12345678),
    ///     "Admin".to_string(),
    ///     false,
    /// );
    /// assert_eq!(admin.user_id(), UserId(12345678));
    /// ```
    #[must_use]
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Returns the custom rank/title of this administrator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_administrator::DialogAdministrator;
    /// use rustgram_types::UserId;
    ///
    /// let admin = DialogAdministrator::new(
    ///     UserId(12345678),
    ///     "Chief Moderator".to_string(),
    ///     false,
    /// );
    /// assert_eq!(admin.rank(), "Chief Moderator");
    /// ```
    #[must_use]
    pub fn rank(&self) -> &str {
        &self.rank
    }

    /// Returns whether this administrator is the creator/owner of the chat.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_administrator::DialogAdministrator;
    /// use rustgram_types::UserId;
    ///
    /// let owner = DialogAdministrator::new(
    ///     UserId(12345678),
    ///     "Owner".to_string(),
    ///     true,
    /// );
    /// assert!(owner.is_creator());
    ///
    /// let admin = DialogAdministrator::new(
    ///     UserId(87654321),
    ///     "Admin".to_string(),
    ///     false,
    /// );
    /// assert!(!admin.is_creator());
    /// ```
    #[must_use]
    pub const fn is_creator(&self) -> bool {
        self.is_creator
    }
}

impl fmt::Debug for DialogAdministrator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DialogAdministrator")
            .field("user_id", &self.user_id)
            .field("rank", &self.rank)
            .field("is_creator", &self.is_creator)
            .finish()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for DialogAdministrator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("DialogAdministrator", 3)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("rank", &self.rank)?;
        state.serialize_field("is_creator", &self.is_creator)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DialogAdministrator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            UserId,
            Rank,
            IsCreator,
        }

        struct DialogAdministratorVisitor;

        impl<'de> serde::de::Visitor<'de> for DialogAdministratorVisitor {
            type Value = DialogAdministrator;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct DialogAdministrator")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let user_id = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let rank = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let is_creator = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                Ok(DialogAdministrator::new(user_id, rank, is_creator))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut user_id = None;
                let mut rank = None;
                let mut is_creator = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::UserId => {
                            if user_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_id"));
                            }
                            user_id = Some(map.next_value()?);
                        }
                        Field::Rank => {
                            if rank.is_some() {
                                return Err(serde::de::Error::duplicate_field("rank"));
                            }
                            rank = Some(map.next_value()?);
                        }
                        Field::IsCreator => {
                            if is_creator.is_some() {
                                return Err(serde::de::Error::duplicate_field("is_creator"));
                            }
                            is_creator = Some(map.next_value()?);
                        }
                    }
                }

                let user_id = user_id.ok_or_else(|| serde::de::Error::missing_field("user_id"))?;
                let rank = rank.ok_or_else(|| serde::de::Error::missing_field("rank"))?;
                let is_creator =
                    is_creator.ok_or_else(|| serde::de::Error::missing_field("is_creator"))?;

                Ok(DialogAdministrator::new(user_id, rank, is_creator))
            }
        }

        deserializer.deserialize_struct(
            "DialogAdministrator",
            &["user_id", "rank", "is_creator"],
            DialogAdministratorVisitor,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test construction with new()
    #[test]
    fn test_new_construction() {
        let admin = DialogAdministrator::new(UserId(12345678), "Admin".to_string(), false);
        assert_eq!(admin.user_id(), UserId(12345678));
        assert_eq!(admin.rank(), "Admin");
        assert!(!admin.is_creator());
    }

    // Test default construction
    #[test]
    fn test_default_construction() {
        let admin = DialogAdministrator::default();
        assert_eq!(admin.user_id(), UserId::default());
        assert_eq!(admin.user_id(), UserId(0));
        assert_eq!(admin.rank(), "");
        assert!(!admin.is_creator());
    }

    // Test user_id getter
    #[test]
    fn test_user_id_getter() {
        let admin = DialogAdministrator::new(UserId(99999999), "Mod".to_string(), false);
        assert_eq!(admin.user_id(), UserId(99999999));
    }

    // Test rank getter
    #[test]
    fn test_rank_getter() {
        let admin = DialogAdministrator::new(UserId(1), "Super Admin".to_string(), false);
        assert_eq!(admin.rank(), "Super Admin");
    }

    // Test empty rank
    #[test]
    fn test_empty_rank() {
        let admin = DialogAdministrator::new(UserId(1), String::new(), false);
        assert_eq!(admin.rank(), "");
        assert!(admin.rank().is_empty());
    }

    // Test is_creator getter returns true for owner
    #[test]
    fn test_is_creator_true() {
        let owner = DialogAdministrator::new(UserId(1), "Owner".to_string(), true);
        assert!(owner.is_creator());
    }

    // Test is_creator getter returns false for non-owner
    #[test]
    fn test_is_creator_false() {
        let admin = DialogAdministrator::new(UserId(2), "Admin".to_string(), false);
        assert!(!admin.is_creator());
    }

    // Test PartialEq with equal administrators
    #[test]
    fn test_partial_eq_equal() {
        let admin1 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        let admin2 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        assert_eq!(admin1, admin2);
    }

    // Test PartialEq with different user_id
    #[test]
    fn test_partial_eq_different_user_id() {
        let admin1 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        let admin2 = DialogAdministrator::new(UserId(456), "Admin".to_string(), false);
        assert_ne!(admin1, admin2);
    }

    // Test PartialEq with different rank
    #[test]
    fn test_partial_eq_different_rank() {
        let admin1 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        let admin2 = DialogAdministrator::new(UserId(123), "Mod".to_string(), false);
        assert_ne!(admin1, admin2);
    }

    // Test PartialEq with different is_creator
    #[test]
    fn test_partial_eq_different_is_creator() {
        let admin1 = DialogAdministrator::new(UserId(123), "Owner".to_string(), true);
        let admin2 = DialogAdministrator::new(UserId(123), "Owner".to_string(), false);
        assert_ne!(admin1, admin2);
    }

    // Test Eq trait
    #[test]
    fn test_eq_trait() {
        let admin1 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        let admin2 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        // Eq just requires PartialEq to be implemented, which we test above
        assert_eq!(admin1, admin2);
    }

    // Test Clone trait
    #[test]
    fn test_clone() {
        let admin = DialogAdministrator::new(UserId(12345), "Clone Test".to_string(), true);
        let cloned = admin.clone();
        assert_eq!(admin, cloned);
        assert_eq!(cloned.user_id(), UserId(12345));
        assert_eq!(cloned.rank(), "Clone Test");
        assert!(cloned.is_creator());
    }

    // Test Debug output
    #[test]
    fn test_debug_output() {
        let admin = DialogAdministrator::new(UserId(12345), "Debug Admin".to_string(), false);
        let debug_str = format!("{:?}", admin);
        assert!(debug_str.contains("DialogAdministrator"));
        assert!(debug_str.contains("12345"));
        assert!(debug_str.contains("Debug Admin"));
    }

    // Test Hash trait consistency
    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let admin1 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        let admin2 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);

        let mut hasher1 = DefaultHasher::new();
        admin1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        admin2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    // Test Hash trait difference
    #[test]
    fn test_hash_difference() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let admin1 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        let admin2 = DialogAdministrator::new(UserId(456), "Admin".to_string(), false);

        let mut hasher1 = DefaultHasher::new();
        admin1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        admin2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_ne!(hash1, hash2);
    }

    // Test with Unicode rank
    #[test]
    fn test_unicode_rank() {
        let admin = DialogAdministrator::new(UserId(1), "Администратор".to_string(), false);
        assert_eq!(admin.rank(), "Администратор");
    }

    // Test with emoji in rank
    #[test]
    fn test_emoji_rank() {
        let admin = DialogAdministrator::new(UserId(1), "👑 Owner".to_string(), true);
        assert_eq!(admin.rank(), "👑 Owner");
    }

    // Test very long rank
    #[test]
    fn test_long_rank() {
        let long_rank = "A".repeat(1000);
        let admin = DialogAdministrator::new(UserId(1), long_rank.clone(), false);
        assert_eq!(admin.rank(), long_rank);
        assert_eq!(admin.rank().len(), 1000);
    }

    // Test multiple fields different at once
    #[test]
    fn test_all_fields_different() {
        let admin1 = DialogAdministrator::new(UserId(111), "Admin".to_string(), false);
        let admin2 = DialogAdministrator::new(UserId(222), "Owner".to_string(), true);
        assert_ne!(admin1, admin2);
        assert_eq!(admin1.user_id(), UserId(111));
        assert_eq!(admin2.user_id(), UserId(222));
    }

    // Test owner with custom title
    #[test]
    fn test_owner_with_custom_title() {
        let owner = DialogAdministrator::new(UserId(1), "Founder".to_string(), true);
        assert!(owner.is_creator());
        assert_eq!(owner.rank(), "Founder");
    }

    // Test administrator without custom title
    #[test]
    fn test_admin_without_title() {
        let admin = DialogAdministrator::new(UserId(123), String::new(), false);
        assert!(!admin.is_creator());
        assert_eq!(admin.rank(), "");
    }

    // Serde tests (only compiled with serde feature)
    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;

        // Test JSON serialization
        #[test]
        fn test_json_serialize() {
            let admin = DialogAdministrator::new(UserId(12345), "Admin".to_string(), false);
            let json = serde_json::to_string(&admin)
                .expect("JSON serialization should succeed for valid DialogAdministrator");
            assert!(json.contains("\"user_id\":12345"));
            assert!(json.contains("\"rank\":\"Admin\""));
            assert!(json.contains("\"is_creator\":false"));
        }

        // Test JSON deserialization
        #[test]
        fn test_json_deserialize() {
            let json = r#"{"user_id":12345,"rank":"Admin","is_creator":false}"#;
            let admin: DialogAdministrator = serde_json::from_str(json)
                .expect("JSON deserialization should succeed for valid JSON");
            assert_eq!(admin.user_id(), UserId(12345));
            assert_eq!(admin.rank(), "Admin");
            assert!(!admin.is_creator());
        }

        // Test JSON round-trip
        #[test]
        fn test_json_round_trip() {
            let original = DialogAdministrator::new(UserId(99999), "Super Admin".to_string(), true);
            let json = serde_json::to_string(&original).expect("JSON serialization should succeed");
            let deserialized: DialogAdministrator = serde_json::from_str(&json)
                .expect("JSON deserialization should succeed for valid JSON");
            assert_eq!(original, deserialized);
        }

        // Test JSON with Unicode rank
        #[test]
        fn test_json_unicode_rank() {
            let admin = DialogAdministrator::new(UserId(1), "Модератор".to_string(), false);
            let json = serde_json::to_string(&admin)
                .expect("JSON serialization should succeed for valid DialogAdministrator");
            let deserialized: DialogAdministrator = serde_json::from_str(&json)
                .expect("JSON deserialization should succeed for valid JSON");
            assert_eq!(admin, deserialized);
            assert_eq!(deserialized.rank(), "Модератор");
        }

        // Test JSON with owner
        #[test]
        fn test_json_owner() {
            let owner = DialogAdministrator::new(UserId(1), "Creator".to_string(), true);
            let json = serde_json::to_string(&owner).expect("JSON serialization should succeed");
            let deserialized: DialogAdministrator = serde_json::from_str(&json)
                .expect("JSON deserialization should succeed for valid JSON");
            assert!(deserialized.is_creator());
        }

        // Test bincode round-trip
        #[test]
        fn test_bincode_round_trip() {
            let original =
                DialogAdministrator::new(UserId(77777), "Bincode Test".to_string(), false);
            let encoded =
                bincode::serialize(&original).expect("bincode serialization should succeed");
            let decoded: DialogAdministrator = bincode::deserialize(&encoded)
                .expect("bincode deserialization should succeed for valid data");
            assert_eq!(original, decoded);
        }

        // Test empty rank serialization
        #[test]
        fn test_serialize_empty_rank() {
            let admin = DialogAdministrator::new(UserId(123), String::new(), false);
            let json = serde_json::to_string(&admin)
                .expect("JSON serialization should succeed for valid DialogAdministrator");
            let deserialized: DialogAdministrator = serde_json::from_str(&json)
                .expect("JSON deserialization should succeed for valid JSON");
            assert_eq!(admin, deserialized);
            assert_eq!(deserialized.rank(), "");
        }
    }
}
