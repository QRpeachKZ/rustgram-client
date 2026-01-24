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

//! # Formatted Text
//!
//! Represents formatted text with entities for Telegram messages.
//!
//! This is a simplified version of TDLib's FormattedText that contains
//! the text content and associated formatting entities.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A text entity representing formatting (bold, italic, links, etc.)
///
/// This is a simplified version. The full TDLib implementation includes
/// many more entity types (previews, custom emojis, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Entity type (e.g., "bold", "italic", "url")
    entity_type: String,
    /// Offset in the text where this entity starts
    offset: i32,
    /// Length of the text this entity applies to
    length: i32,
    /// Optional argument (e.g., URL for links, user_id for mentions)
    argument: Option<String>,
}

impl MessageEntity {
    /// Creates a new message entity.
    ///
    /// # Arguments
    ///
    /// * `entity_type` - Type of entity (e.g., "bold", "italic", "url")
    /// * `offset` - Offset in the text where this entity starts
    /// * `length` - Length of the text this entity applies to
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_formatted_text::MessageEntity;
    ///
    /// let entity = MessageEntity::new("bold", 0, 5);
    /// assert_eq!(entity.entity_type(), "bold");
    /// assert_eq!(entity.offset(), 0);
    /// assert_eq!(entity.length(), 5);
    /// ```
    pub fn new(entity_type: &str, offset: i32, length: i32) -> Self {
        Self {
            entity_type: entity_type.to_string(),
            offset,
            length,
            argument: None,
        }
    }

    /// Creates a new message entity with an argument.
    ///
    /// # Arguments
    ///
    /// * `entity_type` - Type of entity (e.g., "textLink")
    /// * `offset` - Offset in the text where this entity starts
    /// * `length` - Length of the text this entity applies to
    /// * `argument` - Optional argument (e.g., URL for text links)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_formatted_text::MessageEntity;
    ///
    /// let entity = MessageEntity::with_argument("textLink", 0, 5, Some("https://example.com"));
    /// assert_eq!(entity.argument(), Some("https://example.com"));
    /// ```
    pub fn with_argument(
        entity_type: &str,
        offset: i32,
        length: i32,
        argument: Option<&str>,
    ) -> Self {
        Self {
            entity_type: entity_type.to_string(),
            offset,
            length,
            argument: argument.map(|s| s.to_string()),
        }
    }

    /// Returns the entity type.
    pub fn entity_type(&self) -> &str {
        &self.entity_type
    }

    /// Returns the offset in the text where this entity starts.
    pub fn offset(&self) -> i32 {
        self.offset
    }

    /// Returns the length of the text this entity applies to.
    pub fn length(&self) -> i32 {
        self.length
    }

    /// Returns the optional argument.
    pub fn argument(&self) -> Option<&str> {
        self.argument.as_deref()
    }

    /// Returns `true` if the entity is valid (offset >= 0, length > 0).
    pub fn is_valid(&self) -> bool {
        self.offset >= 0 && self.length > 0
    }
}

/// Formatted text with entities.
///
/// Represents text with associated formatting entities like bold, italic, links, etc.
///
/// # Example
///
/// ```rust
/// use rustgram_formatted_text::{FormattedText, MessageEntity};
///
/// let text = FormattedText::new("Hello World");
/// assert_eq!(text.text(), "Hello World");
/// assert!(text.entities().is_empty());
///
/// let mut with_entity = FormattedText::new("Hello World");
/// with_entity.add_entity(MessageEntity::new("bold", 0, 5));
/// assert_eq!(with_entity.entities().len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct FormattedText {
    /// The plain text content
    text: String,
    /// Formatting entities applied to the text
    entities: Vec<MessageEntity>,
}

impl FormattedText {
    /// Creates a new formatted text with no entities.
    ///
    /// # Arguments
    ///
    /// * `text` - The plain text content
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Hello, world!");
    /// assert_eq!(text.text(), "Hello, world!");
    /// ```
    #[must_use]
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            entities: Vec::new(),
        }
    }

    /// Creates a new formatted text with entities.
    ///
    /// # Arguments
    ///
    /// * `text` - The plain text content
    /// * `entities` - Vector of formatting entities
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_formatted_text::{FormattedText, MessageEntity};
    ///
    /// let entities = vec![
    ///     MessageEntity::new("bold", 0, 5),
    ///     MessageEntity::new("italic", 6, 5),
    /// ];
    /// let text = FormattedText::with_entities("Hello World", entities);
    /// assert_eq!(text.entities().len(), 2);
    /// ```
    pub fn with_entities(text: &str, entities: Vec<MessageEntity>) -> Self {
        Self {
            text: text.to_string(),
            entities,
        }
    }

    /// Returns the plain text content.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Hello");
    /// assert_eq!(text.text(), "Hello");
    /// ```
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the formatting entities.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Hello");
    /// assert!(text.entities().is_empty());
    /// ```
    pub fn entities(&self) -> &[MessageEntity] {
        &self.entities
    }

    /// Adds a formatting entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to add
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_formatted_text::{FormattedText, MessageEntity};
    ///
    /// let mut text = FormattedText::new("Hello World");
    /// text.add_entity(MessageEntity::new("bold", 0, 5));
    /// assert_eq!(text.entities().len(), 1);
    /// ```
    pub fn add_entity(&mut self, entity: MessageEntity) {
        self.entities.push(entity);
    }

    /// Returns `true` if the text is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("");
    /// assert!(text.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the length of the text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Hello");
    /// assert_eq!(text.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Validates that all entities are within bounds of the text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_formatted_text::{FormattedText, MessageEntity};
    ///
    /// let mut text = FormattedText::new("Hello");
    /// text.add_entity(MessageEntity::new("bold", 0, 5));
    /// assert!(text.validate_entities().is_ok());
    ///
    /// text.add_entity(MessageEntity::new("bold", 0, 10));
    /// assert!(text.validate_entities().is_err());
    /// ```
    pub fn validate_entities(&self) -> Result<(), FormattedTextError> {
        let text_len = self.text.len() as i32;

        for entity in &self.entities {
            if !entity.is_valid() {
                return Err(FormattedTextError::InvalidEntity {
                    offset: entity.offset,
                    length: entity.length,
                });
            }

            if entity.offset + entity.length > text_len {
                return Err(FormattedTextError::EntityOutOfBounds {
                    offset: entity.offset,
                    length: entity.length,
                    text_len,
                });
            }
        }

        Ok(())
    }
}

impl fmt::Display for FormattedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

/// Errors that can occur when working with formatted text.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FormattedTextError {
    /// An entity has invalid parameters (negative offset or non-positive length)
    #[error("Invalid entity: offset={offset}, length={length}")]
    InvalidEntity { offset: i32, length: i32 },

    /// An entity extends beyond the bounds of the text
    #[error("Entity out of bounds: offset={offset}, length={length}, text_len={text_len}")]
    EntityOutOfBounds {
        offset: i32,
        length: i32,
        text_len: i32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatted_text_new() {
        let text = FormattedText::new("Hello, world!");
        assert_eq!(text.text(), "Hello, world!");
        assert!(text.entities().is_empty());
    }

    #[test]
    fn test_formatted_text_default() {
        let text = FormattedText::default();
        assert_eq!(text.text(), "");
        assert!(text.is_empty());
    }

    #[test]
    fn test_formatted_text_with_entities() {
        let entities = vec![
            MessageEntity::new("bold", 0, 5),
            MessageEntity::new("italic", 6, 5),
        ];
        let text = FormattedText::with_entities("Hello World", entities);
        assert_eq!(text.text(), "Hello World");
        assert_eq!(text.entities().len(), 2);
    }

    #[test]
    fn test_formatted_text_add_entity() {
        let mut text = FormattedText::new("Hello World");
        assert_eq!(text.entities().len(), 0);

        text.add_entity(MessageEntity::new("bold", 0, 5));
        assert_eq!(text.entities().len(), 1);

        text.add_entity(MessageEntity::new("italic", 6, 5));
        assert_eq!(text.entities().len(), 2);
    }

    #[test]
    fn test_formatted_text_is_empty() {
        let text = FormattedText::new("");
        assert!(text.is_empty());

        let text = FormattedText::new("Hello");
        assert!(!text.is_empty());
    }

    #[test]
    fn test_formatted_text_len() {
        let text = FormattedText::new("Hello");
        assert_eq!(text.len(), 5);

        let text = FormattedText::new("");
        assert_eq!(text.len(), 0);
    }

    #[test]
    fn test_message_entity_new() {
        let entity = MessageEntity::new("bold", 0, 5);
        assert_eq!(entity.entity_type(), "bold");
        assert_eq!(entity.offset(), 0);
        assert_eq!(entity.length(), 5);
        assert_eq!(entity.argument(), None);
    }

    #[test]
    fn test_message_entity_with_argument() {
        let entity = MessageEntity::with_argument("textLink", 0, 5, Some("https://example.com"));
        assert_eq!(entity.entity_type(), "textLink");
        assert_eq!(entity.argument(), Some("https://example.com"));
    }

    #[test]
    fn test_message_entity_is_valid() {
        let valid = MessageEntity::new("bold", 0, 5);
        assert!(valid.is_valid());

        let invalid_offset = MessageEntity::new("bold", -1, 5);
        assert!(!invalid_offset.is_valid());

        let invalid_length = MessageEntity::new("bold", 0, 0);
        assert!(!invalid_length.is_valid());

        let negative_length = MessageEntity::new("bold", 0, -1);
        assert!(!negative_length.is_valid());
    }

    #[test]
    fn test_validate_entities_valid() {
        let mut text = FormattedText::new("Hello World");
        text.add_entity(MessageEntity::new("bold", 0, 5));
        text.add_entity(MessageEntity::new("italic", 6, 5));
        assert!(text.validate_entities().is_ok());
    }

    #[test]
    fn test_validate_entities_invalid_entity() {
        let mut text = FormattedText::new("Hello");
        text.add_entity(MessageEntity::new("bold", -1, 5));
        assert!(text.validate_entities().is_err());
    }

    #[test]
    fn test_validate_entities_out_of_bounds() {
        let mut text = FormattedText::new("Hello");
        text.add_entity(MessageEntity::new("bold", 0, 10));
        let result = text.validate_entities();
        assert!(result.is_err());
        match result {
            Err(FormattedTextError::EntityOutOfBounds {
                offset,
                length,
                text_len,
            }) => {
                assert_eq!(offset, 0);
                assert_eq!(length, 10);
                assert_eq!(text_len, 5);
            }
            _ => panic!("Expected EntityOutOfBounds error"),
        }
    }

    #[test]
    fn test_equality() {
        let text1 = FormattedText::new("Hello");
        let text2 = FormattedText::new("Hello");
        assert_eq!(text1, text2);

        let text3 = FormattedText::new("World");
        assert_ne!(text1, text3);
    }

    #[test]
    fn test_entity_equality() {
        let entity1 = MessageEntity::new("bold", 0, 5);
        let entity2 = MessageEntity::new("bold", 0, 5);
        assert_eq!(entity1, entity2);

        let entity3 = MessageEntity::new("italic", 0, 5);
        assert_ne!(entity1, entity3);
    }

    #[test]
    fn test_display() {
        let text = FormattedText::new("Hello World");
        assert_eq!(format!("{}", text), "Hello World");
    }

    #[test]
    fn test_clone() {
        let text1 = FormattedText::new("Hello");
        let text2 = text1.clone();
        assert_eq!(text1, text2);
    }

    #[test]
    fn test_serialization() {
        let text = FormattedText::new("Hello");
        let json = serde_json::to_string(&text).unwrap();
        let parsed: FormattedText = serde_json::from_str(&json).unwrap();
        assert_eq!(text, parsed);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let entity1 = MessageEntity::new("bold", 0, 5);
        let entity2 = MessageEntity::new("bold", 0, 5);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        entity1.hash(&mut hasher1);
        entity2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_common_entity_types() {
        let types = [
            "bold",
            "italic",
            "underline",
            "strikethrough",
            "code",
            "pre",
        ];

        for entity_type in types {
            let entity = MessageEntity::new(entity_type, 0, 5);
            assert_eq!(entity.entity_type(), entity_type);
            assert!(entity.is_valid());
        }
    }

    #[test]
    fn test_entity_with_none_argument() {
        let entity = MessageEntity::with_argument("textLink", 0, 5, None);
        assert_eq!(entity.argument(), None);
    }

    #[test]
    fn test_entity_with_empty_argument() {
        let entity = MessageEntity::with_argument("textLink", 0, 5, Some(""));
        assert_eq!(entity.argument(), Some(""));
    }

    #[test]
    fn test_validate_multiple_entities() {
        let mut text = FormattedText::new("Hello World Test");
        text.add_entity(MessageEntity::new("bold", 0, 5));
        text.add_entity(MessageEntity::new("italic", 6, 5));
        text.add_entity(MessageEntity::new("underline", 12, 4));
        assert!(text.validate_entities().is_ok());
    }

    #[test]
    fn test_entity_at_boundary() {
        let mut text = FormattedText::new("Hello");
        text.add_entity(MessageEntity::new("bold", 0, 5));
        assert!(text.validate_entities().is_ok());
    }

    #[test]
    fn test_empty_text_with_entity() {
        let mut text = FormattedText::new("");
        text.add_entity(MessageEntity::new("bold", 0, 0));
        assert!(!text.validate_entities().is_ok());
    }

    #[test]
    fn test_entity_zero_length_at_valid_position() {
        let mut text = FormattedText::new("Hello");
        text.add_entity(MessageEntity::new("bold", 2, 0));
        assert!(!text.validate_entities().is_ok());
    }
}
