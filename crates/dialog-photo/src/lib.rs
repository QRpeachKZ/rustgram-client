//! Dialog photo information.
//!
//! This module provides the `DialogPhoto` type, which represents a photo
//! associated with a dialog (chat, channel, or user).
//!
//! # TDLib Alignment
//!
//! - TDLib type: `DialogPhoto` (td/telegram/DialogPhoto.h)
//! - Contains small and big file IDs, minithumbnail, and flags
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_photo::DialogPhoto;
//!
//! // Create a dialog photo with file IDs
//! let photo = DialogPhoto::new(123, 456);
//! assert_eq!(photo.small_file_id(), 123);
//! assert_eq!(photo.big_file_id(), 456);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Dialog photo information.
///
/// Contains file IDs for small and big versions of a dialog photo,
/// along with optional minithumbnail and flags for animation/personal status.
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_photo::DialogPhoto;
///
/// let photo = DialogPhoto::new(123, 456);
/// assert_eq!(photo.small_file_id(), 123);
/// assert_eq!(photo.big_file_id(), 456);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogPhoto {
    /// File ID for the small version of the photo.
    small_file_id: i64,
    /// File ID for the big version of the photo.
    big_file_id: i64,
    /// Minithumbnail data (base64 encoded).
    minithumbnail: Option<String>,
    /// Whether the photo has an animation.
    has_animation: bool,
    /// Whether the photo is personal.
    is_personal: bool,
}

impl DialogPhoto {
    /// Creates a new dialog photo.
    ///
    /// # Arguments
    ///
    /// * `small_file_id` - File ID for the small version
    /// * `big_file_id` - File ID for the big version
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let photo = DialogPhoto::new(123, 456);
    /// assert_eq!(photo.small_file_id(), 123);
    /// ```
    pub fn new(small_file_id: i64, big_file_id: i64) -> Self {
        Self {
            small_file_id,
            big_file_id,
            minithumbnail: None,
            has_animation: false,
            is_personal: false,
        }
    }

    /// Returns the small file ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let photo = DialogPhoto::new(123, 456);
    /// assert_eq!(photo.small_file_id(), 123);
    /// ```
    pub fn small_file_id(&self) -> i64 {
        self.small_file_id
    }

    /// Returns the big file ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let photo = DialogPhoto::new(123, 456);
    /// assert_eq!(photo.big_file_id(), 456);
    /// ```
    pub fn big_file_id(&self) -> i64 {
        self.big_file_id
    }

    /// Returns the minithumbnail data if available.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let mut photo = DialogPhoto::new(123, 456);
    /// photo.set_minithumbnail(Some("thumbnail_data".to_string()));
    /// assert_eq!(photo.minithumbnail(), Some("thumbnail_data"));
    /// ```
    pub fn minithumbnail(&self) -> Option<&str> {
        self.minithumbnail.as_deref()
    }

    /// Checks if the photo has an animation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let mut photo = DialogPhoto::new(123, 456);
    /// photo.set_has_animation(true);
    /// assert!(photo.has_animation());
    /// ```
    pub fn has_animation(&self) -> bool {
        self.has_animation
    }

    /// Checks if the photo is personal.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let mut photo = DialogPhoto::new(123, 456);
    /// photo.set_is_personal(true);
    /// assert!(photo.is_personal());
    /// ```
    pub fn is_personal(&self) -> bool {
        self.is_personal
    }

    /// Sets the minithumbnail data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let mut photo = DialogPhoto::new(123, 456);
    /// photo.set_minithumbnail(Some("data".to_string()));
    /// assert!(photo.minithumbnail().is_some());
    /// ```
    pub fn set_minithumbnail(&mut self, minithumbnail: Option<String>) {
        self.minithumbnail = minithumbnail;
    }

    /// Sets whether the photo has an animation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let mut photo = DialogPhoto::new(123, 456);
    /// photo.set_has_animation(true);
    /// assert!(photo.has_animation());
    /// ```
    pub fn set_has_animation(&mut self, has_animation: bool) {
        self.has_animation = has_animation;
    }

    /// Sets whether the photo is personal.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let mut photo = DialogPhoto::new(123, 456);
    /// photo.set_is_personal(true);
    /// assert!(photo.is_personal());
    /// ```
    pub fn set_is_personal(&mut self, is_personal: bool) {
        self.is_personal = is_personal;
    }

    /// Checks if this photo is empty (no file IDs).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_photo::DialogPhoto;
    ///
    /// let photo = DialogPhoto::new(0, 0);
    /// assert!(photo.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.small_file_id == 0 && self.big_file_id == 0
    }
}

impl Default for DialogPhoto {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl fmt::Display for DialogPhoto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "no photo")
        } else {
            write!(
                f,
                "photo (small={}, big={})",
                self.small_file_id, self.big_file_id
            )
        }
    }
}

impl Serialize for DialogPhoto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (
            self.small_file_id,
            self.big_file_id,
            &self.minithumbnail,
            self.has_animation,
            self.is_personal,
        )
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DialogPhoto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (small_file_id, big_file_id, minithumbnail, has_animation, is_personal) =
            <(i64, i64, Option<String>, bool, bool)>::deserialize(deserializer)?;
        let mut photo = Self::new(small_file_id, big_file_id);
        photo.minithumbnail = minithumbnail;
        photo.has_animation = has_animation;
        photo.is_personal = is_personal;
        Ok(photo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10)
    #[test]
    fn test_debug() {
        let photo = DialogPhoto::new(123, 456);
        assert!(format!("{:?}", photo).contains("DialogPhoto"));
    }

    #[test]
    fn test_clone() {
        let photo = DialogPhoto::new(123, 456);
        let cloned = photo.clone();
        assert_eq!(photo, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let p1 = DialogPhoto::new(123, 456);
        let p2 = DialogPhoto::new(123, 456);
        let p3 = DialogPhoto::new(789, 1011);
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_default() {
        let photo = DialogPhoto::default();
        assert!(photo.is_empty());
    }

    #[test]
    fn test_display_with_ids() {
        let photo = DialogPhoto::new(123, 456);
        let display = format!("{}", photo);
        assert!(display.contains("123"));
        assert!(display.contains("456"));
    }

    #[test]
    fn test_display_empty() {
        let photo = DialogPhoto::new(0, 0);
        assert_eq!(format!("{}", photo), "no photo");
    }

    // Constructor tests (1 * 2 = 2)
    #[test]
    fn test_new() {
        let photo = DialogPhoto::new(123, 456);
        assert_eq!(photo.small_file_id(), 123);
        assert_eq!(photo.big_file_id(), 456);
        assert!(!photo.has_animation());
        assert!(!photo.is_personal());
        assert_eq!(photo.minithumbnail(), None);
    }

    // Getter tests (5 * 2 = 10)
    #[test]
    fn test_small_file_id() {
        let photo = DialogPhoto::new(123, 456);
        assert_eq!(photo.small_file_id(), 123);
    }

    #[test]
    fn test_big_file_id() {
        let photo = DialogPhoto::new(123, 456);
        assert_eq!(photo.big_file_id(), 456);
    }

    #[test]
    fn test_minithumbnail_none() {
        let photo = DialogPhoto::new(123, 456);
        assert_eq!(photo.minithumbnail(), None);
    }

    #[test]
    fn test_minithumbnail_some() {
        let mut photo = DialogPhoto::new(123, 456);
        photo.set_minithumbnail(Some("data".to_string()));
        assert_eq!(photo.minithumbnail(), Some("data"));
    }

    #[test]
    fn test_has_animation_false() {
        let photo = DialogPhoto::new(123, 456);
        assert!(!photo.has_animation());
    }

    #[test]
    fn test_has_animation_true() {
        let mut photo = DialogPhoto::new(123, 456);
        photo.set_has_animation(true);
        assert!(photo.has_animation());
    }

    #[test]
    fn test_is_personal_false() {
        let photo = DialogPhoto::new(123, 456);
        assert!(!photo.is_personal());
    }

    #[test]
    fn test_is_personal_true() {
        let mut photo = DialogPhoto::new(123, 456);
        photo.set_is_personal(true);
        assert!(photo.is_personal());
    }

    // Setter tests (3 * 2 = 6)
    #[test]
    fn test_set_minithumbnail() {
        let mut photo = DialogPhoto::new(123, 456);
        photo.set_minithumbnail(Some("thumb".to_string()));
        assert_eq!(photo.minithumbnail(), Some("thumb"));
    }

    #[test]
    fn test_set_minithumbnail_clear() {
        let mut photo = DialogPhoto::new(123, 456);
        photo.set_minithumbnail(Some("thumb".to_string()));
        photo.set_minithumbnail(None);
        assert_eq!(photo.minithumbnail(), None);
    }

    #[test]
    fn test_set_has_animation() {
        let mut photo = DialogPhoto::new(123, 456);
        photo.set_has_animation(true);
        assert!(photo.has_animation());
    }

    #[test]
    fn test_set_is_personal() {
        let mut photo = DialogPhoto::new(123, 456);
        photo.set_is_personal(true);
        assert!(photo.is_personal());
    }

    // Method tests (1 * 3 = 3)
    #[test]
    fn test_is_empty_true() {
        let photo = DialogPhoto::new(0, 0);
        assert!(photo.is_empty());
    }

    #[test]
    fn test_is_empty_false_with_small() {
        let photo = DialogPhoto::new(123, 0);
        assert!(!photo.is_empty());
    }

    #[test]
    fn test_is_empty_false_with_big() {
        let photo = DialogPhoto::new(0, 456);
        assert!(!photo.is_empty());
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize() {
        let mut photo = DialogPhoto::new(123, 456);
        photo.set_minithumbnail(Some("thumb".to_string()));
        photo.set_has_animation(true);
        photo.set_is_personal(false);

        let json = serde_json::to_string(&photo).unwrap();
        let deserialized: DialogPhoto = serde_json::from_str(&json).unwrap();
        assert_eq!(photo, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_empty() {
        let photo = DialogPhoto::new(0, 0);
        let json = serde_json::to_string(&photo).unwrap();
        let deserialized: DialogPhoto = serde_json::from_str(&json).unwrap();
        assert_eq!(photo, deserialized);
    }

    // Edge cases (2)
    #[test]
    fn test_negative_file_ids() {
        let photo = DialogPhoto::new(-1, -2);
        assert_eq!(photo.small_file_id(), -1);
        assert_eq!(photo.big_file_id(), -2);
        assert!(!photo.is_empty());
    }

    #[test]
    fn test_large_file_ids() {
        let photo = DialogPhoto::new(i64::MAX, i64::MAX - 1);
        assert_eq!(photo.small_file_id(), i64::MAX);
        assert_eq!(photo.big_file_id(), i64::MAX - 1);
    }

    // Doc test example (1)
    #[test]
    fn test_doc_example() {
        let photo = DialogPhoto::new(123, 456);
        assert_eq!(photo.small_file_id(), 123);
        assert_eq!(photo.big_file_id(), 456);
    }
}
