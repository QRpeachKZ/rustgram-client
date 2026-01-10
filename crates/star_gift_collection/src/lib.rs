// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Star gift collection types for Telegram MTProto client.
//!
//! This module implements TDLib's StarGiftCollection and StarGiftCollectionId
//! from `td/telegram/StarGiftCollection.h` and `td/telegram/StarGiftCollectionId.h`.
//!
//! # Overview
//!
//! Star gifts are a feature in Telegram that allow users to send virtual gifts
//! using Telegram Stars. Gifts are organized into collections, each with an ID,
//! title, icon, and count of available gifts.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt;
use std::hash::{Hash, Hasher};

/// Identifier for a star gift collection.
///
/// Valid collection IDs are positive integers (i32).
///
/// # Example
///
/// ```
/// use rustgram_star_gift_collection::StarGiftCollectionId;
///
/// let id = StarGiftCollectionId::new(123);
/// assert!(id.is_valid());
/// assert_eq!(id.get(), 123);
///
/// let invalid = StarGiftCollectionId::new(0);
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StarGiftCollectionId(i32);

impl StarGiftCollectionId {
    /// Creates a new StarGiftCollectionId.
    ///
    /// # Arguments
    ///
    /// * `id` - The collection ID value
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_star_gift_collection::StarGiftCollectionId;
    ///
    /// let id = StarGiftCollectionId::new(42);
    /// assert_eq!(id.get(), 42);
    /// ```
    #[inline]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the inner ID value.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_star_gift_collection::StarGiftCollectionId;
    ///
    /// let id = StarGiftCollectionId::new(100);
    /// assert_eq!(id.get(), 100);
    /// ```
    #[inline]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid collection ID.
    ///
    /// Valid collection IDs are positive (greater than 0).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_star_gift_collection::StarGiftCollectionId;
    ///
    /// assert!(StarGiftCollectionId::new(1).is_valid());
    /// assert!(StarGiftCollectionId::new(100).is_valid());
    /// assert!(!StarGiftCollectionId::new(0).is_valid());
    /// assert!(!StarGiftCollectionId::new(-1).is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 > 0
    }
}

impl Hash for StarGiftCollectionId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for StarGiftCollectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gift collection {}", self.0)
    }
}

impl From<i32> for StarGiftCollectionId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<StarGiftCollectionId> for i32 {
    fn from(id: StarGiftCollectionId) -> Self {
        id.0
    }
}

/// A simple file identifier type.
///
/// This is a placeholder for TDLib's FileId type.
/// In a full implementation, this would include more complex file identification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct FileId(i32);

impl FileId {
    /// Creates a new FileId.
    #[inline]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the inner ID value.
    #[inline]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid file ID.
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 > 0
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "file {}", self.0)
    }
}

impl From<i32> for FileId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

/// A star gift collection in Telegram.
///
/// Star gift collections group related gifts together, such as themed collections
/// for holidays, events, or categories.
///
/// # Example
///
/// ```
/// use rustgram_star_gift_collection::{StarGiftCollection, StarGiftCollectionId, FileId};
///
/// let collection = StarGiftCollection::new(
///     StarGiftCollectionId::new(1),
///     "Birthday Gifts".to_string(),
///     FileId::new(12345),
///     10,
///     1234567890
/// );
///
/// assert!(collection.is_valid());
/// assert_eq!(collection.title(), "Birthday Gifts");
/// assert_eq!(collection.gift_count(), 10);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarGiftCollection {
    /// The unique identifier for this collection.
    collection_id: StarGiftCollectionId,
    /// The title/name of the collection.
    title: String,
    /// The file ID for the collection's icon.
    icon_file_id: FileId,
    /// The number of gifts in this collection.
    gift_count: i32,
    /// Hash value for caching/validating the collection data.
    hash: i64,
}

impl StarGiftCollection {
    /// Creates a new StarGiftCollection.
    ///
    /// # Arguments
    ///
    /// * `collection_id` - The unique identifier for the collection
    /// * `title` - The title/name of the collection
    /// * `icon_file_id` - The file ID for the collection's icon
    /// * `gift_count` - The number of gifts in this collection
    /// * `hash` - Hash value for caching/validating the collection data
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_star_gift_collection::{StarGiftCollection, StarGiftCollectionId, FileId};
    ///
    /// let collection = StarGiftCollection::new(
    ///     StarGiftCollectionId::new(5),
    ///     "Holiday Special".to_string(),
    ///     FileId::new(999),
    ///     25,
    ///     9876543210
    /// );
    /// ```
    pub fn new(
        collection_id: StarGiftCollectionId,
        title: String,
        icon_file_id: FileId,
        gift_count: i32,
        hash: i64,
    ) -> Self {
        Self {
            collection_id,
            title,
            icon_file_id,
            gift_count,
            hash,
        }
    }

    /// Returns the collection ID.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_star_gift_collection::{StarGiftCollection, StarGiftCollectionId, FileId};
    ///
    /// let collection = StarGiftCollection::new(
    ///     StarGiftCollectionId::new(42),
    ///     "Test".to_string(),
    ///     FileId::new(1),
    ///     5,
    ///     0
    /// );
    /// assert_eq!(collection.collection_id().get(), 42);
    /// ```
    pub fn collection_id(&self) -> StarGiftCollectionId {
        self.collection_id
    }

    /// Returns the title of the collection.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_star_gift_collection::{StarGiftCollection, StarGiftCollectionId, FileId};
    ///
    /// let collection = StarGiftCollection::new(
    ///     StarGiftCollectionId::new(1),
    ///     "Summer Collection".to_string(),
    ///     FileId::new(1),
    ///     0,
    ///     0
    /// );
    /// assert_eq!(collection.title(), "Summer Collection");
    /// ```
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the icon file ID.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_star_gift_collection::{StarGiftCollection, StarGiftCollectionId, FileId};
    ///
    /// let collection = StarGiftCollection::new(
    ///     StarGiftCollectionId::new(1),
    ///     "Test".to_string(),
    ///     FileId::new(555),
    ///     0,
    ///     0
    /// );
    /// assert_eq!(collection.icon_file_id().get(), 555);
    /// ```
    pub fn icon_file_id(&self) -> FileId {
        self.icon_file_id
    }

    /// Returns the number of gifts in this collection.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_star_gift_collection::{StarGiftCollection, StarGiftCollectionId, FileId};
    ///
    /// let collection = StarGiftCollection::new(
    ///     StarGiftCollectionId::new(1),
    ///     "Test".to_string(),
    ///     FileId::new(1),
    ///     100,
    ///     0
    /// );
    /// assert_eq!(collection.gift_count(), 100);
    /// ```
    pub fn gift_count(&self) -> i32 {
        self.gift_count
    }

    /// Returns the hash value for this collection.
    ///
    /// The hash is used for caching and validation purposes.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_star_gift_collection::{StarGiftCollection, StarGiftCollectionId, FileId};
    ///
    /// let collection = StarGiftCollection::new(
    ///     StarGiftCollectionId::new(1),
    ///     "Test".to_string(),
    ///     FileId::new(1),
    ///     0,
    ///     12345
    /// );
    /// assert_eq!(collection.hash(), 12345);
    /// ```
    pub fn hash(&self) -> i64 {
        self.hash
    }

    /// Checks if this collection is valid.
    ///
    /// A valid collection has a valid collection ID and a non-empty title.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_star_gift_collection::{StarGiftCollection, StarGiftCollectionId, FileId};
    ///
    /// let valid = StarGiftCollection::new(
    ///     StarGiftCollectionId::new(1),
    ///     "Valid".to_string(),
    ///     FileId::new(1),
    ///     0,
    ///     0
    /// );
    /// assert!(valid.is_valid());
    ///
    /// let invalid = StarGiftCollection::new(
    ///     StarGiftCollectionId::new(0),
    ///     "".to_string(),
    ///     FileId::new(1),
    ///     0,
    ///     0
    /// );
    /// assert!(!invalid.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.collection_id.is_valid() && !self.title.is_empty()
    }

    /// Sets a new title for the collection.
    ///
    /// # Arguments
    ///
    /// * `title` - The new title
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Sets a new icon file ID for the collection.
    ///
    /// # Arguments
    ///
    /// * `icon_file_id` - The new icon file ID
    pub fn with_icon_file_id(mut self, icon_file_id: FileId) -> Self {
        self.icon_file_id = icon_file_id;
        self
    }

    /// Sets a new gift count for the collection.
    ///
    /// # Arguments
    ///
    /// * `gift_count` - The new gift count
    pub fn with_gift_count(mut self, gift_count: i32) -> Self {
        self.gift_count = gift_count;
        self
    }

    /// Sets a new hash value for the collection.
    ///
    /// # Arguments
    ///
    /// * `hash` - The new hash value
    pub fn with_hash(mut self, hash: i64) -> Self {
        self.hash = hash;
        self
    }
}

impl fmt::Display for StarGiftCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StarGiftCollection(id={}, title={}, gifts={})",
            self.collection_id, self.title, self.gift_count
        )
    }
}

impl Hash for StarGiftCollection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.collection_id.hash(state);
        self.hash.hash(state);
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-star-gift-collection";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-star-gift-collection");
    }

    // StarGiftCollectionId tests
    #[test]
    fn test_collection_id_new() {
        let id = StarGiftCollectionId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_collection_id_is_valid() {
        assert!(StarGiftCollectionId::new(1).is_valid());
        assert!(StarGiftCollectionId::new(100).is_valid());
        assert!(StarGiftCollectionId::new(i32::MAX).is_valid());
        assert!(!StarGiftCollectionId::new(0).is_valid());
        assert!(!StarGiftCollectionId::new(-1).is_valid());
        assert!(!StarGiftCollectionId::new(-100).is_valid());
    }

    #[test]
    fn test_collection_id_default() {
        let id = StarGiftCollectionId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_collection_id_from_i32() {
        let id = StarGiftCollectionId::from(42);
        assert_eq!(id.get(), 42);
    }

    #[test]
    fn test_collection_id_into_i32() {
        let id = StarGiftCollectionId::new(99);
        let value: i32 = id.into();
        assert_eq!(value, 99);
    }

    #[test]
    fn test_collection_id_display() {
        let id = StarGiftCollectionId::new(5);
        assert_eq!(format!("{id}"), "gift collection 5");
    }

    #[test]
    fn test_collection_id_ord() {
        let id1 = StarGiftCollectionId::new(1);
        let id2 = StarGiftCollectionId::new(2);
        assert!(id1 < id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_collection_id_hash() {
        use std::collections::hash_map::DefaultHasher;
        let id1 = StarGiftCollectionId::new(42);
        let id2 = StarGiftCollectionId::new(42);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        id1.hash(&mut hasher1);
        id2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    // FileId tests
    #[test]
    fn test_file_id_new() {
        let file_id = FileId::new(456);
        assert_eq!(file_id.get(), 456);
    }

    #[test]
    fn test_file_id_is_valid() {
        assert!(FileId::new(1).is_valid());
        assert!(!FileId::new(0).is_valid());
        assert!(!FileId::new(-1).is_valid());
    }

    // StarGiftCollection tests
    #[test]
    fn test_collection_new() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test Collection".to_string(),
            FileId::new(123),
            10,
            1234567890,
        );

        assert_eq!(collection.collection_id().get(), 1);
        assert_eq!(collection.title(), "Test Collection");
        assert_eq!(collection.icon_file_id().get(), 123);
        assert_eq!(collection.gift_count(), 10);
        assert_eq!(collection.hash(), 1234567890);
    }

    #[test]
    fn test_collection_is_valid() {
        let valid = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Valid".to_string(),
            FileId::new(1),
            0,
            0,
        );
        assert!(valid.is_valid());

        let invalid_id = StarGiftCollection::new(
            StarGiftCollectionId::new(0),
            "Title".to_string(),
            FileId::new(1),
            0,
            0,
        );
        assert!(!invalid_id.is_valid());

        let invalid_title = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "".to_string(),
            FileId::new(1),
            0,
            0,
        );
        assert!(!invalid_title.is_valid());
    }

    #[test]
    fn test_collection_with_title() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Old Title".to_string(),
            FileId::new(1),
            0,
            0,
        )
        .with_title("New Title".to_string());

        assert_eq!(collection.title(), "New Title");
    }

    #[test]
    fn test_collection_with_icon_file_id() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test".to_string(),
            FileId::new(1),
            0,
            0,
        )
        .with_icon_file_id(FileId::new(999));

        assert_eq!(collection.icon_file_id().get(), 999);
    }

    #[test]
    fn test_collection_with_gift_count() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test".to_string(),
            FileId::new(1),
            5,
            0,
        )
        .with_gift_count(50);

        assert_eq!(collection.gift_count(), 50);
    }

    #[test]
    fn test_collection_with_hash() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test".to_string(),
            FileId::new(1),
            0,
            100,
        )
        .with_hash(200);

        assert_eq!(collection.hash(), 200);
    }

    #[test]
    fn test_collection_display() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(5),
            "Holiday Gifts".to_string(),
            FileId::new(1),
            25,
            0,
        );
        let display = format!("{collection}");
        assert!(display.contains("5"));
        assert!(display.contains("Holiday Gifts"));
        assert!(display.contains("25"));
    }

    #[test]
    fn test_collection_eq() {
        let collection1 = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Same".to_string(),
            FileId::new(1),
            10,
            100,
        );
        let collection2 = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Same".to_string(),
            FileId::new(1),
            10,
            100,
        );
        assert_eq!(collection1, collection2);
    }

    #[test]
    fn test_collection_clone() {
        let collection1 = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test".to_string(),
            FileId::new(1),
            10,
            100,
        );
        let collection2 = collection1.clone();
        assert_eq!(collection1, collection2);
    }

    #[test]
    fn test_collection_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;
        let collection1 = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test".to_string(),
            FileId::new(1),
            10,
            12345,
        );
        let collection2 = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test".to_string(),
            FileId::new(1),
            10,
            12345,
        );

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        Hash::hash(&collection1, &mut hasher1);
        Hash::hash(&collection2, &mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_collection_builder_chain() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Initial".to_string(),
            FileId::new(1),
            0,
            0,
        )
        .with_title("Updated".to_string())
        .with_gift_count(100)
        .with_hash(999);

        assert_eq!(collection.title(), "Updated");
        assert_eq!(collection.gift_count(), 100);
        assert_eq!(collection.hash(), 999);
    }

    #[test]
    fn test_empty_gift_count() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Empty".to_string(),
            FileId::new(1),
            0,
            0,
        );
        assert_eq!(collection.gift_count(), 0);
        assert!(collection.is_valid()); // Still valid with 0 gifts
    }

    #[test]
    fn test_negative_gift_count() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test".to_string(),
            FileId::new(1),
            -5,
            0,
        );
        assert_eq!(collection.gift_count(), -5);
    }

    #[test]
    fn test_zero_hash() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test".to_string(),
            FileId::new(1),
            0,
            0,
        );
        assert_eq!(collection.hash(), 0);
    }

    #[test]
    fn test_negative_hash() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test".to_string(),
            FileId::new(1),
            0,
            -123,
        );
        assert_eq!(collection.hash(), -123);
    }

    #[test]
    fn test_collection_with_invalid_file_id() {
        let collection = StarGiftCollection::new(
            StarGiftCollectionId::new(1),
            "Test".to_string(),
            FileId::new(0),
            0,
            0,
        );
        assert!(!collection.icon_file_id().is_valid());
        // Collection is still valid even with invalid file ID
        assert!(collection.is_valid());
    }
}
