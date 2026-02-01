//! # Rustgram BusinessIntro
//!
//! Business intro handling for Telegram MTProto client.
//!
//! This crate provides types for managing business introduction/start page
//! that is shown when users first interact with a business account.
//!
//! ## Overview
//!
//! - [`BusinessIntro`] - Business introduction/start page configuration
//!
//! ## Examples
//!
//! ```
//! use rustgram_business_intro::BusinessIntro;
//!
//! let intro = BusinessIntro::new();
//! assert!(intro.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;

/// Sticker file identifier.
///
/// Represents the file ID of a sticker used in the business intro.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StickerFileId(i64);

impl Default for StickerFileId {
    fn default() -> Self {
        Self(0)
    }
}

impl StickerFileId {
    /// Creates a new sticker file ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::StickerFileId;
    ///
    /// let id = StickerFileId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the raw ID value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::StickerFileId;
    ///
    /// let id = StickerFileId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(&self) -> i64 {
        self.0
    }

    /// Checks if this is a valid file ID (non-zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::StickerFileId;
    ///
    /// assert!(StickerFileId::new(12345).is_valid());
    /// assert!(!StickerFileId::new(0).is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

/// Business introduction/start page configuration.
///
/// Defines the welcome message shown when users first interact with
/// a business account.
///
/// # Examples
///
/// ```
/// use rustgram_business_intro::{BusinessIntro, StickerFileId};
///
/// let intro = BusinessIntro::new();
/// assert!(intro.is_empty());
///
/// let intro = BusinessIntro::with_data(
///     "Welcome!".to_string(),
///     "How can I help you?".to_string(),
///     StickerFileId::new(12345),
/// );
/// assert!(!intro.is_empty());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessIntro {
    /// Title of the business intro
    title: String,
    /// Description/message of the business intro
    description: String,
    /// Optional sticker file ID
    sticker_file_id: StickerFileId,
}

impl Default for BusinessIntro {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessIntro {
    /// Creates a new empty business intro.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::BusinessIntro;
    ///
    /// let intro = BusinessIntro::new();
    /// assert!(intro.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            sticker_file_id: StickerFileId::default(),
        }
    }

    /// Creates a business intro with the given data.
    ///
    /// # Arguments
    ///
    /// * `title` - Title of the intro
    /// * `description` - Description/message of the intro
    /// * `sticker_file_id` - Optional sticker file ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::{BusinessIntro, StickerFileId};
    ///
    /// let intro = BusinessIntro::with_data(
    ///     "Welcome!".to_string(),
    ///     "How can I help you?".to_string(),
    ///     StickerFileId::new(12345),
    /// );
    /// assert!(!intro.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn with_data(title: String, description: String, sticker_file_id: StickerFileId) -> Self {
        Self {
            title,
            description,
            sticker_file_id,
        }
    }

    /// Checks if the intro is empty (no content).
    ///
    /// # Returns
    ///
    /// `true` if title, description, and sticker are all empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::BusinessIntro;
    ///
    /// assert!(BusinessIntro::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.title.is_empty() && self.description.is_empty() && !self.sticker_file_id.is_valid()
    }

    /// Returns the title.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::BusinessIntro;
    ///
    /// let intro = BusinessIntro::with_data("Hello".to_string(), String::new(), Default::default());
    /// assert_eq!(intro.title(), "Hello");
    /// ```
    #[inline]
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the description.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::BusinessIntro;
    ///
    /// let intro = BusinessIntro::with_data(String::new(), "Welcome!".to_string(), Default::default());
    /// assert_eq!(intro.description(), "Welcome!");
    /// ```
    #[inline]
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the sticker file ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::{BusinessIntro, StickerFileId};
    ///
    /// let id = StickerFileId::new(12345);
    /// let intro = BusinessIntro::with_data(String::new(), String::new(), id);
    /// assert_eq!(intro.sticker_file_id().get(), 12345);
    /// ```
    #[inline]
    #[must_use]
    pub const fn sticker_file_id(&self) -> StickerFileId {
        self.sticker_file_id
    }

    /// Checks if a sticker is set.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::{BusinessIntro, StickerFileId};
    ///
    /// let intro = BusinessIntro::new();
    /// assert!(!intro.has_sticker());
    ///
    /// let intro = BusinessIntro::with_data(String::new(), String::new(), StickerFileId::new(12345));
    /// assert!(intro.has_sticker());
    /// ```
    #[inline]
    #[must_use]
    pub fn has_sticker(&self) -> bool {
        self.sticker_file_id.is_valid()
    }

    /// Sets the title.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::BusinessIntro;
    ///
    /// let mut intro = BusinessIntro::new();
    /// intro.set_title("Welcome".to_string());
    /// assert_eq!(intro.title(), "Welcome");
    /// ```
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Sets the description.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::BusinessIntro;
    ///
    /// let mut intro = BusinessIntro::new();
    /// intro.set_description("Hello there!".to_string());
    /// assert_eq!(intro.description(), "Hello there!");
    /// ```
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Sets the sticker file ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::{BusinessIntro, StickerFileId};
    ///
    /// let mut intro = BusinessIntro::new();
    /// intro.set_sticker_file_id(StickerFileId::new(12345));
    /// assert!(intro.has_sticker());
    /// ```
    pub fn set_sticker_file_id(&mut self, sticker_file_id: StickerFileId) {
        self.sticker_file_id = sticker_file_id;
    }

    /// Clears all content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_intro::{BusinessIntro, StickerFileId};
    ///
    /// let mut intro = BusinessIntro::with_data(
    ///     "Title".to_string(),
    ///     "Description".to_string(),
    ///     StickerFileId::new(12345),
    /// );
    /// assert!(!intro.is_empty());
    ///
    /// intro.clear();
    /// assert!(intro.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl fmt::Display for BusinessIntro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessIntro {{ title: {:?}, description: {:?}, has_sticker: {} }}",
            self.title,
            self.description,
            self.has_sticker()
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-business-intro";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== StickerFileId Tests ==========

    #[test]
    fn test_sticker_file_id_new() {
        let id = StickerFileId::new(12345);
        assert_eq!(id.get(), 12345);
    }

    #[test]
    fn test_sticker_file_id_is_valid() {
        assert!(StickerFileId::new(12345).is_valid());
        assert!(StickerFileId::new(-1).is_valid());
        assert!(!StickerFileId::new(0).is_valid());
    }

    #[test]
    fn test_sticker_file_id_default() {
        let id = StickerFileId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    // ========== BusinessIntro Constructor Tests ==========

    #[test]
    fn test_new_creates_empty() {
        let intro = BusinessIntro::new();
        assert!(intro.is_empty());
        assert_eq!(intro.title(), "");
        assert_eq!(intro.description(), "");
        assert!(!intro.has_sticker());
    }

    #[test]
    fn test_default_creates_empty() {
        let intro = BusinessIntro::default();
        assert!(intro.is_empty());
    }

    #[test]
    fn test_with_data_sets_values() {
        let intro = BusinessIntro::with_data(
            "Welcome".to_string(),
            "How can I help?".to_string(),
            StickerFileId::new(12345),
        );

        assert_eq!(intro.title(), "Welcome");
        assert_eq!(intro.description(), "How can I help?");
        assert_eq!(intro.sticker_file_id().get(), 12345);
    }

    // ========== is_empty Tests ==========

    #[test]
    fn test_is_empty_when_all_empty() {
        let intro = BusinessIntro::new();
        assert!(intro.is_empty());
    }

    #[test]
    fn test_is_empty_with_title() {
        let intro =
            BusinessIntro::with_data("Title".to_string(), String::new(), Default::default());
        assert!(!intro.is_empty());
    }

    #[test]
    fn test_is_empty_with_description() {
        let intro =
            BusinessIntro::with_data(String::new(), "Description".to_string(), Default::default());
        assert!(!intro.is_empty());
    }

    #[test]
    fn test_is_empty_with_sticker() {
        let intro =
            BusinessIntro::with_data(String::new(), String::new(), StickerFileId::new(12345));
        assert!(!intro.is_empty());
    }

    // ========== Accessor Tests ==========

    #[test]
    fn test_title_accessor() {
        let intro =
            BusinessIntro::with_data("Hello".to_string(), String::new(), Default::default());
        assert_eq!(intro.title(), "Hello");
    }

    #[test]
    fn test_description_accessor() {
        let intro =
            BusinessIntro::with_data(String::new(), "World".to_string(), Default::default());
        assert_eq!(intro.description(), "World");
    }

    #[test]
    fn test_sticker_file_id_accessor() {
        let intro =
            BusinessIntro::with_data(String::new(), String::new(), StickerFileId::new(99999));
        assert_eq!(intro.sticker_file_id().get(), 99999);
    }

    #[test]
    fn test_has_sticker_when_valid() {
        let intro =
            BusinessIntro::with_data(String::new(), String::new(), StickerFileId::new(12345));
        assert!(intro.has_sticker());
    }

    #[test]
    fn test_has_sticker_when_invalid() {
        let intro = BusinessIntro::new();
        assert!(!intro.has_sticker());
    }

    // ========== Mutator Tests ==========

    #[test]
    fn test_set_title() {
        let mut intro = BusinessIntro::new();
        intro.set_title("New Title".to_string());
        assert_eq!(intro.title(), "New Title");
    }

    #[test]
    fn test_set_description() {
        let mut intro = BusinessIntro::new();
        intro.set_description("New Description".to_string());
        assert_eq!(intro.description(), "New Description");
    }

    #[test]
    fn test_set_sticker_file_id() {
        let mut intro = BusinessIntro::new();
        assert!(!intro.has_sticker());

        intro.set_sticker_file_id(StickerFileId::new(54321));
        assert!(intro.has_sticker());
        assert_eq!(intro.sticker_file_id().get(), 54321);
    }

    #[test]
    fn test_clear() {
        let mut intro = BusinessIntro::with_data(
            "Title".to_string(),
            "Description".to_string(),
            StickerFileId::new(12345),
        );
        assert!(!intro.is_empty());

        intro.clear();
        assert!(intro.is_empty());
        assert_eq!(intro.title(), "");
        assert_eq!(intro.description(), "");
        assert!(!intro.has_sticker());
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_same_values() {
        let intro1 = BusinessIntro::with_data(
            "Title".to_string(),
            "Description".to_string(),
            StickerFileId::new(12345),
        );
        let intro2 = BusinessIntro::with_data(
            "Title".to_string(),
            "Description".to_string(),
            StickerFileId::new(12345),
        );
        assert_eq!(intro1, intro2);
    }

    #[test]
    fn test_inequality_different_title() {
        let intro1 =
            BusinessIntro::with_data("Title1".to_string(), "Desc".to_string(), Default::default());
        let intro2 =
            BusinessIntro::with_data("Title2".to_string(), "Desc".to_string(), Default::default());
        assert_ne!(intro1, intro2);
    }

    #[test]
    fn test_inequality_different_description() {
        let intro1 =
            BusinessIntro::with_data("Title".to_string(), "Desc1".to_string(), Default::default());
        let intro2 =
            BusinessIntro::with_data("Title".to_string(), "Desc2".to_string(), Default::default());
        assert_ne!(intro1, intro2);
    }

    #[test]
    fn test_inequality_different_sticker() {
        let intro1 =
            BusinessIntro::with_data(String::new(), String::new(), StickerFileId::new(11111));
        let intro2 =
            BusinessIntro::with_data(String::new(), String::new(), StickerFileId::new(22222));
        assert_ne!(intro1, intro2);
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone() {
        let intro1 = BusinessIntro::with_data(
            "Title".to_string(),
            "Description".to_string(),
            StickerFileId::new(12345),
        );
        let intro2 = intro1.clone();
        assert_eq!(intro1, intro2);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let intro =
            BusinessIntro::with_data("Hello".to_string(), "World".to_string(), Default::default());
        let s = format!("{}", intro);
        assert!(s.contains("Hello"));
        assert!(s.contains("World"));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-business-intro");
    }
}
