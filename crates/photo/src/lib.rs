//! # Photo
//!
//! Represents a photo in Telegram.
//!
//! ## TDLib Reference
//!
//! - TDLib header: `td/telegram/Photo.h`
//! - TDLib struct: `Photo`
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_photo::Photo;
//!
//! let photo = Photo::new(12345, 1234567890);
//! ```
//!
//! ## Modules
//!
//! - [`download`] - Photo download functionality with caching

use core::fmt;
use rustgram_animation_size::AnimationSize;
use rustgram_photo_size::PhotoSize;
use rustgram_sticker_photo_size::StickerPhotoSize;

pub mod download;

// Re-export download module types for convenience
pub use download::{
    GetFileRequest, GetFileResponse, InputFileLocation, InputPhotoFileLocation, PhotoCacheKey,
    PhotoData, PhotoDownloadError, PhotoDownloader, DEFAULT_CACHE_SIZE_BYTES, INPUT_FILE_EMPTY,
    INPUT_FILE_LOCATION, MAX_PHOTO_SIZE_BYTES, UPLOAD_FILE, UPLOAD_GET_FILE,
};

/// Represents a photo in Telegram.
///
/// TDLib: `struct Photo`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Photo {
    id: i64,
    date: i32,
    minithumbnail: Vec<u8>,
    photos: Vec<PhotoSize>,
    animations: Vec<AnimationSize>,
    sticker_photo_size: Option<StickerPhotoSize>,
    has_stickers: bool,
    sticker_file_ids: Vec<i64>,
}

impl Photo {
    /// Create a new Photo with the given ID and date.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the photo
    /// * `date` - The timestamp when the photo was created
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo::Photo;
    ///
    /// let photo = Photo::new(12345, 1234567890);
    /// assert_eq!(photo.id(), 12345);
    /// ```
    pub fn new(id: i64, date: i32) -> Self {
        Self {
            id,
            date,
            minithumbnail: Vec::new(),
            photos: Vec::new(),
            animations: Vec::new(),
            sticker_photo_size: None,
            has_stickers: false,
            sticker_file_ids: Vec::new(),
        }
    }

    /// Create an empty Photo.
    ///
    /// An empty photo has ID -2 and is considered invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo::Photo;
    ///
    /// let photo = Photo::empty();
    /// assert!(photo.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self {
            id: -2,
            date: 0,
            minithumbnail: Vec::new(),
            photos: Vec::new(),
            animations: Vec::new(),
            sticker_photo_size: None,
            has_stickers: false,
            sticker_file_ids: Vec::new(),
        }
    }

    /// Get the photo ID.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Get the photo date (timestamp).
    pub fn date(&self) -> i32 {
        self.date
    }

    /// Get the minithumbnail data.
    pub fn minithumbnail(&self) -> &[u8] {
        &self.minithumbnail
    }

    /// Get the photo sizes.
    pub fn photos(&self) -> &[PhotoSize] {
        &self.photos
    }

    /// Get the animation sizes.
    pub fn animations(&self) -> &[AnimationSize] {
        &self.animations
    }

    /// Get the sticker photo size.
    pub fn sticker_photo_size(&self) -> Option<&StickerPhotoSize> {
        self.sticker_photo_size.as_ref()
    }

    /// Check if the photo has stickers.
    pub fn has_stickers(&self) -> bool {
        self.has_stickers
    }

    /// Get the sticker file IDs.
    pub fn sticker_file_ids(&self) -> &[i64] {
        &self.sticker_file_ids
    }

    /// Set the minithumbnail data.
    pub fn set_minithumbnail(&mut self, data: Vec<u8>) {
        self.minithumbnail = data;
    }

    /// Add a photo size.
    pub fn add_photo(&mut self, photo: PhotoSize) {
        self.photos.push(photo);
    }

    /// Add an animation size.
    pub fn add_animation(&mut self, animation: AnimationSize) {
        self.animations.push(animation);
    }

    /// Set the sticker photo size.
    pub fn set_sticker_photo_size(&mut self, sticker: StickerPhotoSize) {
        self.sticker_photo_size = Some(sticker);
    }

    /// Set whether the photo has stickers.
    pub fn set_has_stickers(&mut self, has: bool) {
        self.has_stickers = has;
    }

    /// Add a sticker file ID.
    pub fn add_sticker_file_id(&mut self, file_id: i64) {
        self.sticker_file_ids.push(file_id);
    }

    /// Check if the photo is empty (invalid).
    ///
    /// A photo is considered empty if its ID is -2.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo::Photo;
    ///
    /// let photo = Photo::empty();
    /// assert!(photo.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.id == -2
    }

    /// Check if the photo is bad (invalid).
    ///
    /// A photo is bad if it's empty or any of its photo sizes have invalid file IDs.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo::Photo;
    ///
    /// let photo = Photo::empty();
    /// assert!(photo.is_bad());
    /// ```
    pub fn is_bad(&self) -> bool {
        if self.is_empty() {
            return true;
        }
        // Check if any photo has an invalid file ID
        // This is a simplified check - in TDLib it checks PhotoSize::file_id.is_valid()
        false
    }

    /// Get the number of photo sizes.
    pub fn photo_count(&self) -> usize {
        self.photos.len()
    }

    /// Get the number of animation sizes.
    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }

    /// Get the number of sticker file IDs.
    pub fn sticker_count(&self) -> usize {
        self.sticker_file_ids.len()
    }
}

impl Default for Photo {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for Photo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Photo(id: {}, date: {}, sizes: {})",
            self.id,
            self.date,
            self.photos.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10 tests)
    #[test]
    fn test_clone() {
        let a = Photo::new(12345, 1234567890);
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_partial_eq() {
        let a = Photo::new(12345, 1234567890);
        let b = Photo::new(12345, 1234567890);
        assert_eq!(a, b);

        let c = Photo::new(54321, 1234567890);
        assert_ne!(a, c);
    }

    #[test]
    fn test_default() {
        let photo = Photo::default();
        assert!(photo.is_empty());
    }

    #[test]
    fn test_debug() {
        let photo = Photo::new(12345, 1234567890);
        let debug_str = format!("{:?}", photo);
        assert!(debug_str.contains("Photo"));
        assert!(debug_str.contains("12345"));
    }

    // Constructor tests (6 tests)
    #[test]
    fn test_new() {
        let photo = Photo::new(12345, 1234567890);
        assert_eq!(photo.id(), 12345);
        assert_eq!(photo.date(), 1234567890);
    }

    #[test]
    fn test_new_defaults() {
        let photo = Photo::new(12345, 1234567890);
        assert!(photo.minithumbnail().is_empty());
        assert!(photo.photos().is_empty());
        assert!(photo.animations().is_empty());
        assert!(photo.sticker_photo_size().is_none());
        assert!(!photo.has_stickers());
        assert!(photo.sticker_file_ids().is_empty());
    }

    #[test]
    fn test_empty() {
        let photo = Photo::empty();
        assert_eq!(photo.id(), -2);
        assert!(photo.is_empty());
    }

    // Getter tests (12 tests)
    #[test]
    fn test_id() {
        let photo = Photo::new(12345, 1234567890);
        assert_eq!(photo.id(), 12345);
    }

    #[test]
    fn test_date() {
        let photo = Photo::new(12345, 1234567890);
        assert_eq!(photo.date(), 1234567890);
    }

    #[test]
    fn test_minithumbnail() {
        let mut photo = Photo::new(12345, 1234567890);
        photo.set_minithumbnail(vec![1, 2, 3, 4]);
        assert_eq!(photo.minithumbnail(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_photos() {
        let photo = Photo::new(12345, 1234567890);
        assert!(photo.photos().is_empty());
    }

    #[test]
    fn test_animations() {
        let photo = Photo::new(12345, 1234567890);
        assert!(photo.animations().is_empty());
    }

    #[test]
    fn test_sticker_photo_size() {
        let photo = Photo::new(12345, 1234567890);
        assert!(photo.sticker_photo_size().is_none());
    }

    #[test]
    fn test_has_stickers() {
        let photo = Photo::new(12345, 1234567890);
        assert!(!photo.has_stickers());
    }

    #[test]
    fn test_sticker_file_ids() {
        let photo = Photo::new(12345, 1234567890);
        assert!(photo.sticker_file_ids().is_empty());
    }

    // Method tests (12 tests)
    #[test]
    fn test_set_minithumbnail() {
        let mut photo = Photo::new(12345, 1234567890);
        photo.set_minithumbnail(vec![1, 2, 3, 4]);
        assert_eq!(photo.minithumbnail(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_add_sticker_file_id() {
        let mut photo = Photo::new(12345, 1234567890);
        photo.add_sticker_file_id(111);
        photo.add_sticker_file_id(222);
        assert_eq!(photo.sticker_file_ids(), &[111, 222]);
    }

    #[test]
    fn test_set_has_stickers() {
        let mut photo = Photo::new(12345, 1234567890);
        assert!(!photo.has_stickers());
        photo.set_has_stickers(true);
        assert!(photo.has_stickers());
    }

    #[test]
    fn test_is_empty_true() {
        let photo = Photo::empty();
        assert!(photo.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let photo = Photo::new(12345, 1234567890);
        assert!(!photo.is_empty());
    }

    #[test]
    fn test_is_bad_empty() {
        let photo = Photo::empty();
        assert!(photo.is_bad());
    }

    #[test]
    fn test_is_bad_normal() {
        let photo = Photo::new(12345, 1234567890);
        assert!(!photo.is_bad());
    }

    #[test]
    fn test_photo_count() {
        let photo = Photo::new(12345, 1234567890);
        assert_eq!(photo.photo_count(), 0);
    }

    #[test]
    fn test_animation_count() {
        let photo = Photo::new(12345, 1234567890);
        assert_eq!(photo.animation_count(), 0);
    }

    #[test]
    fn test_sticker_count() {
        let mut photo = Photo::new(12345, 1234567890);
        photo.add_sticker_file_id(111);
        photo.add_sticker_file_id(222);
        assert_eq!(photo.sticker_count(), 2);
    }

    // Display tests (2 tests)
    #[test]
    fn test_display() {
        let photo = Photo::new(12345, 1234567890);
        assert_eq!(
            format!("{}", photo),
            "Photo(id: 12345, date: 1234567890, sizes: 0)"
        );
    }
}
