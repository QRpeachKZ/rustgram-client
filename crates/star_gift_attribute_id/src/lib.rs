// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Star Gift Attribute ID
//!
//! Identifier for star gift attributes in Telegram.
//!
//! Based on TDLib's `StarGiftAttributeId` from `td/telegram/StarGiftAttributeId.h`.
//!
//! # Overview
//!
//! A `StarGiftAttributeId` uniquely identifies a star gift attribute.
//! Attributes can be models, patterns, or backdrops.
//!
//! # Example
//!
//! ```rust
//! use rustgram_star_gift_attribute_id::StarGiftAttributeId;
//!
//! let model = StarGiftAttributeId::model(12345);
//! assert!(model.is_model());
//!
//! let backdrop = StarGiftAttributeId::backdrop(100);
//! assert!(backdrop.is_backdrop());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Star gift attribute type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[repr(i32)]
enum Type {
    /// No attribute type (invalid)
    #[default]
    None = 0,
    /// Model attribute
    Model = 1,
    /// Pattern attribute
    Pattern = 2,
    /// Backdrop attribute
    Backdrop = 3,
}

/// Star gift attribute identifier.
///
/// Represents a unique identifier for a Telegram star gift attribute.
/// Can be one of three types: Model, Pattern, or Backdrop.
///
/// # TDLib Mapping
///
/// - `StarGiftAttributeId::model(sticker_id)` → TDLib: `StarGiftAttributeId::model(int64)`
/// - `StarGiftAttributeId::pattern(sticker_id)` → TDLib: `StarGiftAttributeId::pattern(int64)`
/// - `StarGiftAttributeId::backdrop(backdrop_id)` → TDLib: `StarGiftAttributeId::backdrop(int32)`
///
/// # Example
///
/// ```rust
/// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
///
/// let model = StarGiftAttributeId::model(12345);
/// assert!(model.is_model());
/// assert_eq!(model.sticker_id(), Some(12345));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StarGiftAttributeId {
    type_: Type,
    sticker_id: Option<i64>,
    backdrop_id: Option<i32>,
}

impl StarGiftAttributeId {
    /// Creates a new empty `StarGiftAttributeId`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// let id = StarGiftAttributeId::new();
    /// assert!(!id.is_valid());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a model attribute ID from a sticker ID.
    ///
    /// # Arguments
    ///
    /// * `sticker_id` - The sticker ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// let id = StarGiftAttributeId::model(12345);
    /// assert!(id.is_model());
    /// assert_eq!(id.sticker_id(), Some(12345));
    /// ```
    #[must_use]
    pub fn model(sticker_id: i64) -> Self {
        Self {
            type_: Type::Model,
            sticker_id: Some(sticker_id),
            backdrop_id: None,
        }
    }

    /// Creates a pattern attribute ID from a sticker ID.
    ///
    /// # Arguments
    ///
    /// * `sticker_id` - The sticker ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// let id = StarGiftAttributeId::pattern(54321);
    /// assert!(id.is_pattern());
    /// assert_eq!(id.sticker_id(), Some(54321));
    /// ```
    #[must_use]
    pub fn pattern(sticker_id: i64) -> Self {
        Self {
            type_: Type::Pattern,
            sticker_id: Some(sticker_id),
            backdrop_id: None,
        }
    }

    /// Creates a backdrop attribute ID from a backdrop ID.
    ///
    /// # Arguments
    ///
    /// * `backdrop_id` - The backdrop ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// let id = StarGiftAttributeId::backdrop(100);
    /// assert!(id.is_backdrop());
    /// assert_eq!(id.backdrop_id(), Some(100));
    /// ```
    #[must_use]
    pub fn backdrop(backdrop_id: i32) -> Self {
        Self {
            type_: Type::Backdrop,
            sticker_id: None,
            backdrop_id: Some(backdrop_id),
        }
    }

    /// Checks if this is a valid attribute ID.
    ///
    /// # Returns
    ///
    /// Returns `true` if the ID is valid (not None), `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// assert!(!StarGiftAttributeId::new().is_valid());
    /// assert!(StarGiftAttributeId::model(123).is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !matches!(self.type_, Type::None)
    }

    /// Checks if this is an empty attribute ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// assert!(StarGiftAttributeId::new().is_empty());
    /// assert!(!StarGiftAttributeId::model(123).is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self.type_, Type::None)
    }

    /// Checks if this is a model attribute.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// assert!(StarGiftAttributeId::model(123).is_model());
    /// assert!(!StarGiftAttributeId::pattern(123).is_model());
    /// ```
    #[must_use]
    pub fn is_model(&self) -> bool {
        matches!(self.type_, Type::Model)
    }

    /// Checks if this is a pattern attribute.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// assert!(StarGiftAttributeId::pattern(123).is_pattern());
    /// assert!(!StarGiftAttributeId::model(123).is_pattern());
    /// ```
    #[must_use]
    pub fn is_pattern(&self) -> bool {
        matches!(self.type_, Type::Pattern)
    }

    /// Checks if this is a backdrop attribute.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// assert!(StarGiftAttributeId::backdrop(100).is_backdrop());
    /// assert!(!StarGiftAttributeId::model(123).is_backdrop());
    /// ```
    #[must_use]
    pub fn is_backdrop(&self) -> bool {
        matches!(self.type_, Type::Backdrop)
    }

    /// Returns the sticker ID if this is a model or pattern attribute.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// let model = StarGiftAttributeId::model(12345);
    /// assert_eq!(model.sticker_id(), Some(12345));
    ///
    /// let backdrop = StarGiftAttributeId::backdrop(100);
    /// assert_eq!(backdrop.sticker_id(), None);
    /// ```
    #[must_use]
    pub fn sticker_id(&self) -> Option<i64> {
        self.sticker_id
    }

    /// Returns the backdrop ID if this is a backdrop attribute.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute_id::StarGiftAttributeId;
    ///
    /// let backdrop = StarGiftAttributeId::backdrop(100);
    /// assert_eq!(backdrop.backdrop_id(), Some(100));
    ///
    /// let model = StarGiftAttributeId::model(12345);
    /// assert_eq!(model.backdrop_id(), None);
    /// ```
    #[must_use]
    pub fn backdrop_id(&self) -> Option<i32> {
        self.backdrop_id
    }
}

impl Hash for StarGiftAttributeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash based on backdrop_id if present, otherwise sticker_id
        if let Some(backdrop_id) = self.backdrop_id {
            backdrop_id.hash(state);
        } else if let Some(sticker_id) = self.sticker_id {
            sticker_id.hash(state);
        } else {
            self.type_.hash(state);
        }
    }
}

impl fmt::Display for StarGiftAttributeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_ {
            Type::None => write!(f, "unknown attribute"),
            Type::Model => write!(f, "model sticker {}", self.sticker_id.unwrap_or(0)),
            Type::Pattern => write!(f, "pattern sticker {}", self.sticker_id.unwrap_or(0)),
            Type::Backdrop => write!(f, "backdrop {}", self.backdrop_id.unwrap_or(0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = StarGiftAttributeId::new();
        assert!(!id.is_valid());
        assert!(id.is_empty());
    }

    #[test]
    fn test_default() {
        let id = StarGiftAttributeId::default();
        assert!(!id.is_valid());
        assert!(id.is_empty());
    }

    #[test]
    fn test_model() {
        let id = StarGiftAttributeId::model(12345);
        assert!(id.is_valid());
        assert!(id.is_model());
        assert!(!id.is_pattern());
        assert!(!id.is_backdrop());
        assert_eq!(id.sticker_id(), Some(12345));
        assert_eq!(id.backdrop_id(), None);
    }

    #[test]
    fn test_pattern() {
        let id = StarGiftAttributeId::pattern(54321);
        assert!(id.is_valid());
        assert!(!id.is_model());
        assert!(id.is_pattern());
        assert!(!id.is_backdrop());
        assert_eq!(id.sticker_id(), Some(54321));
        assert_eq!(id.backdrop_id(), None);
    }

    #[test]
    fn test_backdrop() {
        let id = StarGiftAttributeId::backdrop(100);
        assert!(id.is_valid());
        assert!(!id.is_model());
        assert!(!id.is_pattern());
        assert!(id.is_backdrop());
        assert_eq!(id.sticker_id(), None);
        assert_eq!(id.backdrop_id(), Some(100));
    }

    #[test]
    fn test_is_valid() {
        assert!(!StarGiftAttributeId::new().is_valid());
        assert!(StarGiftAttributeId::model(123).is_valid());
        assert!(StarGiftAttributeId::pattern(123).is_valid());
        assert!(StarGiftAttributeId::backdrop(100).is_valid());
    }

    #[test]
    fn test_is_empty() {
        assert!(StarGiftAttributeId::new().is_empty());
        assert!(!StarGiftAttributeId::model(123).is_empty());
        assert!(!StarGiftAttributeId::pattern(123).is_empty());
        assert!(!StarGiftAttributeId::backdrop(100).is_empty());
    }

    #[test]
    fn test_equality() {
        let id1 = StarGiftAttributeId::model(12345);
        let id2 = StarGiftAttributeId::model(12345);
        let id3 = StarGiftAttributeId::model(54321);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_equality_different_types() {
        let model = StarGiftAttributeId::model(12345);
        let pattern = StarGiftAttributeId::pattern(12345);
        let backdrop = StarGiftAttributeId::backdrop(12345);

        assert_ne!(model, pattern);
        assert_ne!(model, backdrop);
        assert_ne!(pattern, backdrop);
    }

    #[test]
    fn test_clone() {
        let id1 = StarGiftAttributeId::model(12345);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_display_model() {
        let id = StarGiftAttributeId::model(12345);
        let display = format!("{id}");
        assert!(display.contains("model"));
        assert!(display.contains("12345"));
    }

    #[test]
    fn test_display_pattern() {
        let id = StarGiftAttributeId::pattern(54321);
        let display = format!("{id}");
        assert!(display.contains("pattern"));
        assert!(display.contains("54321"));
    }

    #[test]
    fn test_display_backdrop() {
        let id = StarGiftAttributeId::backdrop(100);
        let display = format!("{id}");
        assert!(display.contains("backdrop"));
        assert!(display.contains("100"));
    }

    #[test]
    fn test_display_empty() {
        let id = StarGiftAttributeId::new();
        let display = format!("{id}");
        assert!(display.contains("unknown"));
    }

    #[test]
    fn test_serialization_model() {
        let id = StarGiftAttributeId::model(12345);
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        let deserialized: StarGiftAttributeId =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_serialization_pattern() {
        let id = StarGiftAttributeId::pattern(54321);
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        let deserialized: StarGiftAttributeId =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_serialization_backdrop() {
        let id = StarGiftAttributeId::backdrop(100);
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        let deserialized: StarGiftAttributeId =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_hash_sticker_id() {
        use std::collections::hash_map::DefaultHasher;

        let id1 = StarGiftAttributeId::model(12345);
        let id2 = StarGiftAttributeId::model(12345);
        let id3 = StarGiftAttributeId::model(54321);

        let mut hasher = DefaultHasher::new();
        id1.hash(&mut hasher);
        let hash1 = hasher.finish();

        hasher = DefaultHasher::new();
        id2.hash(&mut hasher);
        let hash2 = hasher.finish();

        hasher = DefaultHasher::new();
        id3.hash(&mut hasher);
        let hash3 = hasher.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_hash_backdrop_id() {
        use std::collections::hash_map::DefaultHasher;

        let id1 = StarGiftAttributeId::backdrop(100);
        let id2 = StarGiftAttributeId::backdrop(100);
        let id3 = StarGiftAttributeId::backdrop(200);

        let mut hasher = DefaultHasher::new();
        id1.hash(&mut hasher);
        let hash1 = hasher.finish();

        hasher = DefaultHasher::new();
        id2.hash(&mut hasher);
        let hash2 = hasher.finish();

        hasher = DefaultHasher::new();
        id3.hash(&mut hasher);
        let hash3 = hasher.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_zero_values() {
        let model = StarGiftAttributeId::model(0);
        assert!(model.is_valid());
        assert_eq!(model.sticker_id(), Some(0));

        let backdrop = StarGiftAttributeId::backdrop(0);
        assert!(backdrop.is_valid());
        assert_eq!(backdrop.backdrop_id(), Some(0));
    }

    #[test]
    fn test_negative_values() {
        let model = StarGiftAttributeId::model(-1);
        assert!(model.is_valid());
        assert_eq!(model.sticker_id(), Some(-1));

        let backdrop = StarGiftAttributeId::backdrop(-1);
        assert!(backdrop.is_valid());
        assert_eq!(backdrop.backdrop_id(), Some(-1));
    }
}
