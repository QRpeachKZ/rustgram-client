// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # UserInfo
//!
//! Public user profile information.
//!
//! ## TDLib Reference
//!
//! - TDLib schema: `telegram_api.tl` lines for `help.userInfo`
//! - TDLib types: `help.userInfoEmpty`, `help.userInfo`
//!
//! ## Overview
//!
//! `UserInfo` contains public profile information shown to other users, including:
//! - A message/bio text with entities
//! - Author information
//! - Publication date
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_user_info::UserInfo;
//!
//! // Create empty user info
//! let empty = UserInfo::empty();
//!
//! // Create user info with content
//! let info = UserInfo::builder()
//!     .with_message("Hello, I'm Alice!".to_string())
//!     .with_author("Alice Smith".to_string())
//!     .with_date(1234567890)
//!     .build()
//!     .unwrap();
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Stub for MessageEntity.
///
/// TDLib: `MessageEntity` type from `td_api.tl`
///
/// TODO: Full implementation when message_entity crate exists.
/// Currently a minimal placeholder for UserInfo type compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageEntity {
    /// Type of the entity.
    pub type_: String,
    /// Offset of the entity.
    pub offset: i32,
    /// Length of the entity.
    pub length: i32,
}

impl MessageEntity {
    /// Creates a new MessageEntity.
    #[must_use]
    pub const fn new(type_: String, offset: i32, length: i32) -> Self {
        Self {
            type_,
            offset,
            length,
        }
    }

    /// Returns the entity type.
    #[must_use]
    pub fn type_(&self) -> &str {
        &self.type_
    }

    /// Returns the offset.
    #[must_use]
    pub const fn offset(&self) -> i32 {
        self.offset
    }

    /// Returns the length.
    #[must_use]
    pub const fn length(&self) -> i32 {
        self.length
    }
}

/// Represents public user profile information.
///
/// TDLib: `help.userInfo` types from `telegram_api.tl`
///
/// This type has two variants:
/// - `Empty` - No user info available
/// - `Info` - User info with message, entities, author, and date
///
/// # Example
///
/// ```
/// use rustgram_user_info::UserInfo;
///
/// // Empty user info
/// let empty = UserInfo::empty();
/// assert!(empty.is_empty());
///
/// // User info with content
/// let info = UserInfo::builder()
///     .with_message("Hello!".to_string())
///     .with_author("Alice".to_string())
///     .with_date(1234567890)
///     .build()
///     .unwrap();
/// assert!(info.is_info());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserInfo {
    /// No user info available.
    ///
    /// TDLib: `help.userInfoEmpty`
    Empty,
    /// User info with content.
    ///
    /// TDLib: `help.userInfo`
    Info {
        /// Message/bio text.
        message: String,
        /// Message entities for formatting (bold, italic, links, etc.).
        entities: Vec<MessageEntity>,
        /// Author of the message.
        author: String,
        /// Date when the message was written (Unix timestamp).
        date: i32,
    },
}

impl UserInfo {
    /// Creates a new empty UserInfo.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_user_info::UserInfo;
    ///
    /// let info = UserInfo::empty();
    /// assert!(info.is_empty());
    /// ```
    #[must_use]
    pub const fn empty() -> Self {
        Self::Empty
    }

    /// Creates a builder for UserInfo.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_user_info::UserInfo;
    ///
    /// let info = UserInfo::builder()
    ///     .with_message("Hello, world!".to_string())
    ///     .with_author("Alice".to_string())
    ///     .with_date(1234567890)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn builder() -> UserInfoBuilder {
        UserInfoBuilder::new()
    }

    /// Returns true if this is an Empty UserInfo.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_user_info::UserInfo;
    ///
    /// let empty = UserInfo::empty();
    /// assert!(empty.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns true if this is an Info UserInfo.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_user_info::UserInfo;
    ///
    /// let info = UserInfo::builder()
    ///     .with_message("Hello".to_string())
    ///     .with_author("Alice".to_string())
    ///     .with_date(1234567890)
    ///     .build()
    ///     .unwrap();
    /// assert!(info.is_info());
    /// ```
    #[must_use]
    pub const fn is_info(&self) -> bool {
        matches!(self, Self::Info { .. })
    }

    /// Returns the message if available.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_user_info::UserInfo;
    ///
    /// let info = UserInfo::builder()
    ///     .with_message("Hello, world!".to_string())
    ///     .with_author("Alice".to_string())
    ///     .with_date(1234567890)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(info.message(), Some("Hello, world!"));
    /// ```
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Empty => None,
            Self::Info { message, .. } => Some(message),
        }
    }

    /// Returns the entities if available.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_user_info::UserInfo;
    /// use rustgram_user_info::MessageEntity;
    ///
    /// let entity = MessageEntity::new("bold".to_string(), 0, 5);
    /// let info = UserInfo::builder()
    ///     .with_message("Hello".to_string())
    ///     .with_entities(vec![entity])
    ///     .with_author("Alice".to_string())
    ///     .with_date(1234567890)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(info.entities().map(|v| v.len()), Some(1));
    /// ```
    #[must_use]
    pub const fn entities(&self) -> Option<&Vec<MessageEntity>> {
        match self {
            Self::Empty => None,
            Self::Info { entities, .. } => Some(entities),
        }
    }

    /// Returns the author if available.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_user_info::UserInfo;
    ///
    /// let info = UserInfo::builder()
    ///     .with_message("Hello".to_string())
    ///     .with_author("Alice".to_string())
    ///     .with_date(1234567890)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(info.author(), Some("Alice"));
    /// ```
    #[must_use]
    pub fn author(&self) -> Option<&str> {
        match self {
            Self::Empty => None,
            Self::Info { author, .. } => Some(author),
        }
    }

    /// Returns the date if available.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_user_info::UserInfo;
    ///
    /// let info = UserInfo::builder()
    ///     .with_message("Hello".to_string())
    ///     .with_author("Alice".to_string())
    ///     .with_date(1234567890)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(info.date(), Some(1234567890));
    /// ```
    #[must_use]
    pub const fn date(&self) -> Option<i32> {
        match self {
            Self::Empty => None,
            Self::Info { date, .. } => Some(*date),
        }
    }
}

impl Default for UserInfo {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for UserInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "UserInfo[Empty]"),
            Self::Info {
                message,
                author,
                date,
                ..
            } => write!(
                f,
                "UserInfo[message={}, author={}, date={}]",
                message, author, date
            ),
        }
    }
}

/// Builder for creating [`UserInfo`] instances.
///
/// # Example
///
/// ```
/// use rustgram_user_info::UserInfo;
///
/// let info = UserInfo::builder()
///     .with_message("Hello, world!".to_string())
///     .with_author("Alice Smith".to_string())
///     .with_date(1234567890)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct UserInfoBuilder {
    message: Option<String>,
    entities: Vec<MessageEntity>,
    author: Option<String>,
    date: Option<i32>,
}

impl Default for UserInfoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UserInfoBuilder {
    /// Creates a new UserInfoBuilder with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            message: None,
            entities: Vec::new(),
            author: None,
            date: None,
        }
    }

    /// Sets the message/bio text.
    #[must_use]
    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// Sets the message entities.
    #[must_use]
    pub fn with_entities(mut self, entities: Vec<MessageEntity>) -> Self {
        self.entities = entities;
        self
    }

    /// Sets the author.
    #[must_use]
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Sets the date.
    #[must_use]
    pub const fn with_date(mut self, date: i32) -> Self {
        self.date = Some(date);
        self
    }

    /// Builds the UserInfo.
    ///
    /// Returns an error if required fields (message, author, date) are not set.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `message` is not set
    /// - `author` is not set
    /// - `date` is not set
    pub fn build(self) -> Result<UserInfo, String> {
        let message = self.message.ok_or("message is required")?;
        let author = self.author.ok_or("author is required")?;
        let date = self.date.ok_or("date is required")?;

        Ok(UserInfo::Info {
            message,
            entities: self.entities,
            author,
            date,
        })
    }

    /// Builds an empty UserInfo.
    ///
    /// This is a convenience method that ignores any previously set values.
    #[must_use]
    pub const fn build_empty(self) -> UserInfo {
        UserInfo::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a minimal valid user info for testing
    fn create_test_user_info(message: &str, author: &str, date: i32) -> UserInfo {
        UserInfo::builder()
            .with_message(message.to_string())
            .with_author(author.to_string())
            .with_date(date)
            .build()
            .unwrap()
    }

    #[test]
    fn test_empty() {
        let info = UserInfo::empty();
        assert!(info.is_empty());
        assert!(!info.is_info());
        assert_eq!(info.message(), None);
        assert_eq!(info.author(), None);
        assert_eq!(info.date(), None);
    }

    #[test]
    fn test_default() {
        let info = UserInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_builder_minimal() {
        let info = create_test_user_info("Hello", "Alice", 1234567890);
        assert!(info.is_info());
        assert!(!info.is_empty());
        assert_eq!(info.message(), Some("Hello"));
        assert_eq!(info.author(), Some("Alice"));
        assert_eq!(info.date(), Some(1234567890));
    }

    #[test]
    fn test_builder_with_entities() {
        let entity1 = MessageEntity::new("bold".to_string(), 0, 5);
        let entity2 = MessageEntity::new("italic".to_string(), 6, 10);
        let info = UserInfo::builder()
            .with_message("Hello world".to_string())
            .with_entities(vec![entity1.clone(), entity2.clone()])
            .with_author("Alice".to_string())
            .with_date(1234567890)
            .build()
            .unwrap();

        assert_eq!(info.message(), Some("Hello world"));
        assert_eq!(info.entities().map(|v| v.len()), Some(2));
        assert_eq!(info.entities().map(|v| v[0].type_()), Some("bold"));
    }

    #[test]
    fn test_builder_missing_message() {
        let result = UserInfo::builder()
            .with_author("Alice".to_string())
            .with_date(1234567890)
            .build();
        assert_eq!(result, Err("message is required".to_string()));
    }

    #[test]
    fn test_builder_missing_author() {
        let result = UserInfo::builder()
            .with_message("Hello".to_string())
            .with_date(1234567890)
            .build();
        assert_eq!(result, Err("author is required".to_string()));
    }

    #[test]
    fn test_builder_missing_date() {
        let result = UserInfo::builder()
            .with_message("Hello".to_string())
            .with_author("Alice".to_string())
            .build();
        assert_eq!(result, Err("date is required".to_string()));
    }

    #[test]
    fn test_build_empty() {
        let info = UserInfo::builder()
            .with_message("Hello".to_string())
            .with_author("Alice".to_string())
            .with_date(1234567890)
            .build_empty();
        assert!(info.is_empty());
    }

    #[test]
    fn test_equality() {
        let info1 = create_test_user_info("Hello", "Alice", 1234567890);
        let info2 = create_test_user_info("Hello", "Alice", 1234567890);
        assert_eq!(info1, info2);

        let info3 = create_test_user_info("Goodbye", "Bob", 9876543210);
        assert_ne!(info1, info3);
    }

    #[test]
    fn test_clone() {
        let info1 = create_test_user_info("Hello", "Alice", 1234567890);
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_display_empty() {
        let info = UserInfo::empty();
        let display = format!("{info}");
        assert_eq!(display, "UserInfo[Empty]");
    }

    #[test]
    fn test_display_info() {
        let info = create_test_user_info("Hello, world!", "Alice", 1234567890);
        let display = format!("{info}");
        assert!(display.contains("Hello, world!"));
        assert!(display.contains("Alice"));
        assert!(display.contains("1234567890"));
    }

    #[test]
    fn test_debug_formatting() {
        let info = create_test_user_info("Hello", "Alice", 1234567890);
        let debug = format!("{info:?}");
        assert!(debug.contains("Hello"));
        assert!(debug.contains("Alice"));
    }

    #[test]
    fn test_message_entity() {
        let entity = MessageEntity::new("bold".to_string(), 0, 5);
        assert_eq!(entity.type_(), "bold");
        assert_eq!(entity.offset(), 0);
        assert_eq!(entity.length(), 5);
    }

    #[test]
    fn test_message_entity_default() {
        let entity = MessageEntity::default();
        assert_eq!(entity.type_(), "");
        assert_eq!(entity.offset(), 0);
        assert_eq!(entity.length(), 0);
    }

    #[test]
    fn test_serialization_empty() {
        let info = UserInfo::empty();
        let json = serde_json::to_string(&info).expect("Failed to serialize");
        let deserialized: UserInfo = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, info);
    }

    #[test]
    fn test_serialization_info() {
        let info = create_test_user_info("Hello", "Alice", 1234567890);
        let json = serde_json::to_string(&info).expect("Failed to serialize");
        assert!(json.contains("Hello"));
        assert!(json.contains("Alice"));

        let deserialized: UserInfo = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, info);
    }

    #[test]
    fn test_serialization_with_entities() {
        let entity = MessageEntity::new("bold".to_string(), 0, 5);
        let info = UserInfo::builder()
            .with_message("Hello".to_string())
            .with_entities(vec![entity])
            .with_author("Alice".to_string())
            .with_date(1234567890)
            .build()
            .unwrap();

        let json = serde_json::to_string(&info).expect("Failed to serialize");
        assert!(json.contains("bold"));

        let deserialized: UserInfo = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, info);
    }

    #[test]
    fn test_builder_chaining() {
        let info = UserInfo::builder()
            .with_message("Hello, world!".to_string())
            .with_author("Alice Smith".to_string())
            .with_date(1234567890)
            .build()
            .unwrap();

        assert_eq!(info.message(), Some("Hello, world!"));
        assert_eq!(info.author(), Some("Alice Smith"));
        assert_eq!(info.date(), Some(1234567890));
    }

    #[test]
    fn test_empty_entities() {
        let info = UserInfo::builder()
            .with_message("Hello".to_string())
            .with_entities(vec![])
            .with_author("Alice".to_string())
            .with_date(1234567890)
            .build()
            .unwrap();

        assert_eq!(info.entities().map(|v| v.len()), Some(0));
    }

    #[test]
    fn test_multiple_entities() {
        let entities = vec![
            MessageEntity::new("bold".to_string(), 0, 5),
            MessageEntity::new("italic".to_string(), 6, 10),
            MessageEntity::new("url".to_string(), 11, 20),
        ];
        let info = UserInfo::builder()
            .with_message("Hello world example".to_string())
            .with_entities(entities)
            .with_author("Alice".to_string())
            .with_date(1234567890)
            .build()
            .unwrap();

        assert_eq!(info.entities().map(|v| v.len()), Some(3));
    }

    #[test]
    fn test_message_ref() {
        let info = create_test_user_info("Hello", "Alice", 1234567890);
        assert_eq!(info.message(), Some("Hello"));
    }

    #[test]
    fn test_author_ref() {
        let info = create_test_user_info("Hello", "Alice", 1234567890);
        assert_eq!(info.author(), Some("Alice"));
    }

    #[test]
    fn test_date_const() {
        let info = create_test_user_info("Hello", "Alice", 1234567890);
        assert_eq!(info.date(), Some(1234567890));
    }

    #[test]
    fn test_is_empty() {
        let empty = UserInfo::empty();
        assert!(empty.is_empty());

        let info = create_test_user_info("Hello", "Alice", 1234567890);
        assert!(!info.is_empty());
    }

    #[test]
    fn test_is_info() {
        let empty = UserInfo::empty();
        assert!(!empty.is_info());

        let info = create_test_user_info("Hello", "Alice", 1234567890);
        assert!(info.is_info());
    }

    #[test]
    fn test_builder_default() {
        let builder = UserInfoBuilder::default();
        assert!(builder.message.is_none());
        assert!(builder.entities.is_empty());
        assert!(builder.author.is_none());
        assert!(builder.date.is_none());
    }

    #[test]
    fn test_with_message() {
        let info = UserInfo::builder()
            .with_message("Test message".to_string())
            .with_author("Alice".to_string())
            .with_date(1234567890)
            .build()
            .unwrap();
        assert_eq!(info.message(), Some("Test message"));
    }

    #[test]
    fn test_with_author() {
        let info = UserInfo::builder()
            .with_message("Hello".to_string())
            .with_author("Bob Smith".to_string())
            .with_date(1234567890)
            .build()
            .unwrap();
        assert_eq!(info.author(), Some("Bob Smith"));
    }

    #[test]
    fn test_with_date() {
        let info = UserInfo::builder()
            .with_message("Hello".to_string())
            .with_author("Alice".to_string())
            .with_date(9876543210)
            .build()
            .unwrap();
        assert_eq!(info.date(), Some(9876543210));
    }

    #[test]
    fn test_empty_message_returns_none() {
        let info = UserInfo::empty();
        assert_eq!(info.message(), None);
    }

    #[test]
    fn test_empty_author_returns_none() {
        let info = UserInfo::empty();
        assert_eq!(info.author(), None);
    }

    #[test]
    fn test_empty_date_returns_none() {
        let info = UserInfo::empty();
        assert_eq!(info.date(), None);
    }

    #[test]
    fn test_empty_entities_returns_none() {
        let info = UserInfo::empty();
        assert_eq!(info.entities(), None);
    }

    #[test]
    fn test_info_returns_some_entities() {
        let entities = vec![MessageEntity::new("bold".to_string(), 0, 5)];
        let info = UserInfo::builder()
            .with_message("Hello".to_string())
            .with_entities(entities)
            .with_author("Alice".to_string())
            .with_date(1234567890)
            .build()
            .unwrap();
        assert!(info.entities().is_some());
        assert_eq!(info.entities().map(|v| v.len()), Some(1));
    }

    #[test]
    fn test_user_info_display_format() {
        let info = create_test_user_info("Test", "Author", 12345);
        let display = format!("{info}");
        assert!(display.contains("UserInfo"));
        assert!(display.contains("Test"));
        assert!(display.contains("Author"));
        assert!(display.contains("12345"));
    }

    #[test]
    fn test_user_info_empty_display_format() {
        let info = UserInfo::empty();
        let display = format!("{info}");
        assert_eq!(display, "UserInfo[Empty]");
    }

    #[test]
    fn test_message_entity_clone() {
        let entity1 = MessageEntity::new("bold".to_string(), 0, 5);
        let entity2 = entity1.clone();
        assert_eq!(entity1, entity2);
    }

    #[test]
    fn test_message_entity_equality() {
        let entity1 = MessageEntity::new("bold".to_string(), 0, 5);
        let entity2 = MessageEntity::new("bold".to_string(), 0, 5);
        assert_eq!(entity1, entity2);

        let entity3 = MessageEntity::new("italic".to_string(), 0, 5);
        assert_ne!(entity1, entity3);
    }

    #[test]
    fn test_serialization_roundtrip_empty() {
        let info = UserInfo::empty();
        let json = serde_json::to_string(&info).expect("serialize");
        let deserialized: UserInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(info, deserialized);
    }

    #[test]
    fn test_serialization_roundtrip_info() {
        let entities = vec![
            MessageEntity::new("bold".to_string(), 0, 5),
            MessageEntity::new("url".to_string(), 6, 15),
        ];
        let info = UserInfo::builder()
            .with_message("Hello world!".to_string())
            .with_entities(entities)
            .with_author("Alice".to_string())
            .with_date(1234567890)
            .build()
            .unwrap();

        let json = serde_json::to_string(&info).expect("serialize");
        let deserialized: UserInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(info, deserialized);
    }

    #[test]
    fn test_builder_new_const() {
        let builder = UserInfoBuilder::new();
        assert!(builder.message.is_none());
        assert!(builder.entities.is_empty());
        assert!(builder.author.is_none());
        assert!(builder.date.is_none());
    }

    #[test]
    fn test_user_info_default_is_empty() {
        let info = UserInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_const_empty() {
        const EMPTY: UserInfo = UserInfo::Empty;
        assert!(EMPTY.is_empty());
    }

    #[test]
    fn test_builder_preserves_entities() {
        let entities = vec![
            MessageEntity::new("bold".to_string(), 0, 5),
            MessageEntity::new("italic".to_string(), 6, 12),
        ];
        let info = UserInfo::builder()
            .with_message("Hello world!".to_string())
            .with_entities(entities.clone())
            .with_author("Alice".to_string())
            .with_date(1234567890)
            .build()
            .unwrap();

        let result_entities = info.entities().unwrap();
        assert_eq!(result_entities.len(), 2);
        assert_eq!(result_entities[0].type_(), "bold");
        assert_eq!(result_entities[1].type_(), "italic");
    }
}
