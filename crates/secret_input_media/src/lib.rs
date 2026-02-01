//! # Rustgram Secret Input Media
//!
//! Encrypted media handling for secret chats in Telegram MTProto client.
//!
//! This crate provides types for preparing and handling media content in
//! secret chats, including layer-specific format variations and encryption
//! support.
//!
//! ## Overview
//!
//! - [`SecretInputMedia`] - Encrypted media container for secret chat messages
//! - [`SecretChatLayer`] - Secret chat protocol version/layer enumeration
//! - [`Dimensions`] - Image/file dimensions (width × height)
//! - [`InputEncryptedFile`] - Encrypted file reference (MVP stub)
//! - [`DecryptedMessageMedia`] - Decrypted media content (MVP stub)
//!
//! ## Examples
//!
//! Basic usage:
//!
//! ```rust
//! use rustgram_secret_input_media::{
//!     SecretInputMedia, SecretChatLayer, Dimensions,
//!     InputEncryptedFile, DecryptedMessageMedia
//! };
//!
//! // Create dimensions
//! let dimensions = Dimensions::new(1920, 1080);
//! assert_eq!(dimensions.width(), 1920);
//! assert_eq!(dimensions.height(), 1080);
//! assert!(dimensions.is_valid());
//!
//! // Create an empty media
//! let empty_media = SecretInputMedia::empty();
//! assert!(empty_media.is_empty());
//!
//! // Create media with content
//! let file = InputEncryptedFile::new(12345, 67890);
//! let decrypted = DecryptedMessageMedia::photo(file.clone());
//! let media = SecretInputMedia::new(file, decrypted);
//! assert!(!media.is_empty());
//! ```
//!
//! Working with secret chat layers:
//!
//! ```rust
//! use rustgram_secret_input_media::SecretChatLayer;
//!
//! // Get current layer
//! let current = SecretChatLayer::CURRENT;
//! assert_eq!(current, SecretChatLayer::SpoilerAndCustomEmojiEntities);
//!
//! // Parse layer from i32
//! let layer = SecretChatLayer::from_i32(143);
//! assert_eq!(layer, Some(SecretChatLayer::SupportBigFiles));
//!
//! // Convert back to i32
//! assert_eq!(SecretChatLayer::SupportBigFiles.as_i32(), 143);
//! ```
//!
//! ## TDLib Alignment
//!
//! This implementation aligns with TDLib's secret chat media handling:
//! - Source: `references/td/td/telegram/SecretInputMedia.h`
//! - Source: `references/td/td/telegram/SecretChatLayer.h`
//! - Layer-specific format selection (modern vs legacy)
//! - File size limits for old layers (2GB for layer < 143)
//!
//! ## Thread Safety
//!
//! All types in this crate are `Send` and `Sync` unless otherwise noted.
//! `SecretChatLayer` and `Dimensions` are `Copy`, making them safe to use
//! across threads without any synchronization primitives.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]

use std::fmt;

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-secret_input_media";

/// Maximum file size for legacy secret chat layers (< 143).
///
/// Files larger than this cannot be sent in secret chats with layers below
/// `SupportBigFiles` (143).
pub const MAX_LEGACY_FILE_SIZE: i64 = 2_000_000_000; // 2 GB

/// Image or file dimensions (width × height).
///
/// This simple struct represents the dimensions of media content like images
/// or videos. Both dimensions must be non-zero for the dimensions to be
/// considered valid.
///
/// # Examples
///
/// ```rust
/// use rustgram_secret_input_media::Dimensions;
///
/// let dims = Dimensions::new(1920, 1080);
/// assert_eq!(dims.width(), 1920);
/// assert_eq!(dims.height(), 1080);
/// assert!(dims.is_valid());
///
/// let invalid = Dimensions::new(0, 1080);
/// assert!(!invalid.is_valid());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Dimensions {
    /// Width in pixels
    pub width: u16,
    /// Height in pixels
    pub height: u16,
}

impl Dimensions {
    /// Creates new dimensions with the given width and height.
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::Dimensions;
    ///
    /// let dims = Dimensions::new(1920, 1080);
    /// assert_eq!(dims.width(), 1920);
    /// assert_eq!(dims.height(), 1080);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Returns the width in pixels.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::Dimensions;
    ///
    /// let dims = Dimensions::new(1920, 1080);
    /// assert_eq!(dims.width(), 1920);
    /// ```
    #[inline]
    #[must_use]
    pub const fn width(self) -> u16 {
        self.width
    }

    /// Returns the height in pixels.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::Dimensions;
    ///
    /// let dims = Dimensions::new(1920, 1080);
    /// assert_eq!(dims.height(), 1080);
    /// ```
    #[inline]
    #[must_use]
    pub const fn height(self) -> u16 {
        self.height
    }

    /// Returns `true` if both dimensions are non-zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::Dimensions;
    ///
    /// let valid = Dimensions::new(1920, 1080);
    /// assert!(valid.is_valid());
    ///
    /// let invalid = Dimensions::new(0, 1080);
    /// assert!(!invalid.is_valid());
    ///
    /// let invalid2 = Dimensions::new(1920, 0);
    /// assert!(!invalid2.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.width > 0 && self.height > 0
    }
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}×{}", self.width, self.height)
    }
}

/// Secret chat protocol layer version.
///
/// Secret chats support different protocol versions (layers), which determine
/// the available features and message formats. Newer layers are backward
/// compatible with older ones.
///
/// The current version is `SpoilerAndCustomEmojiEntities` (144), which supports
/// all features including big files and custom emoji in spoilers.
///
/// # Examples
///
/// ```rust
/// use rustgram_secret_input_media::SecretChatLayer;
///
/// // Get current layer
/// assert_eq!(SecretChatLayer::CURRENT, SecretChatLayer::SpoilerAndCustomEmojiEntities);
///
/// // Parse from i32
/// let layer = SecretChatLayer::from_i32(143);
/// assert_eq!(layer, Some(SecretChatLayer::SupportBigFiles));
///
/// // Unknown layer returns None
/// assert_eq!(SecretChatLayer::from_i32(999), None);
///
/// // Convert to i32
/// assert_eq!(SecretChatLayer::SupportBigFiles.as_i32(), 143);
///
/// // Mtproto2 is an alias for Default
/// assert_eq!(SecretChatLayer::MTPROTO2, SecretChatLayer::Default);
/// ```
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SecretChatLayer {
    /// Default MTProto 2.0 layer (73)
    Default = 73,
    /// New entities layer (101)
    NewEntities = 101,
    /// Delete messages on chat close layer (123)
    DeleteMessagesOnClose = 123,
    /// Support for big files layer (143)
    ///
    /// This layer introduces support for files larger than 2GB by using
    /// int64 size fields instead of int32.
    SupportBigFiles = 143,
    /// Support for spoilers and custom emoji entities (144)
    ///
    /// This is the current recommended layer for secret chats.
    SpoilerAndCustomEmojiEntities = 144,
}

impl SecretChatLayer {
    /// MTProto 2.0 layer (alias for Default).
    ///
    /// This constant provides the same value as `Default` for compatibility
    /// with TDLib's naming convention where both names refer to layer 73.
    pub const MTPROTO2: Self = Self::Default;

    /// The current recommended secret chat layer.
    ///
    /// This constant should be used when creating new secret chats to ensure
    /// maximum compatibility and feature support.
    pub const CURRENT: Self = Self::SpoilerAndCustomEmojiEntities;

    /// Attempts to convert an i32 to a `SecretChatLayer`.
    ///
    /// Returns `None` if the value doesn't correspond to any known layer.
    ///
    /// # Arguments
    ///
    /// * `value` - Layer number as i32
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::SecretChatLayer;
    ///
    /// assert_eq!(SecretChatLayer::from_i32(73), Some(SecretChatLayer::Default));
    /// assert_eq!(SecretChatLayer::from_i32(143), Some(SecretChatLayer::SupportBigFiles));
    /// assert_eq!(SecretChatLayer::from_i32(999), None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            73 => Some(Self::Default),
            101 => Some(Self::NewEntities),
            123 => Some(Self::DeleteMessagesOnClose),
            143 => Some(Self::SupportBigFiles),
            144 => Some(Self::SpoilerAndCustomEmojiEntities),
            _ => None,
        }
    }

    /// Returns the layer as an i32 value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::SecretChatLayer;
    ///
    /// assert_eq!(SecretChatLayer::Default.as_i32(), 73);
    /// assert_eq!(SecretChatLayer::SupportBigFiles.as_i32(), 143);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Returns `true` if this layer supports big files (int64 size).
    ///
    /// This is `true` for layers >= `SupportBigFiles` (143).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::SecretChatLayer;
    ///
    /// assert!(!SecretChatLayer::NewEntities.supports_big_files());
    /// assert!(SecretChatLayer::SupportBigFiles.supports_big_files());
    /// assert!(SecretChatLayer::CURRENT.supports_big_files());
    /// ```
    #[inline]
    #[must_use]
    pub const fn supports_big_files(self) -> bool {
        self as i32 >= Self::SupportBigFiles as i32
    }
}

/// Encrypted file reference (MVP stub).
///
/// This is a simplified stub type for the MVP version. In a full implementation,
/// this would be a proper TL type with variants like:
/// - `inputEncryptedFileEmpty`
/// - `inputEncryptedFileUploaded`
/// - `inputEncryptedFile`
/// - `inputEncryptedFileBigUploaded`
///
/// # Examples
///
/// ```rust
/// use rustgram_secret_input_media::InputEncryptedFile;
///
/// let file = InputEncryptedFile::new(12345, 67890);
/// assert_eq!(file.id, 12345);
/// assert_eq!(file.access_hash, 67890);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InputEncryptedFile {
    /// File identifier
    pub id: i64,
    /// Access hash for the file
    pub access_hash: i64,
}

impl InputEncryptedFile {
    /// Creates a new encrypted file reference.
    ///
    /// # Arguments
    ///
    /// * `id` - File identifier
    /// * `access_hash` - Access hash for the file
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::InputEncryptedFile;
    ///
    /// let file = InputEncryptedFile::new(12345, 67890);
    /// assert_eq!(file.id, 12345);
    /// assert_eq!(file.access_hash, 67890);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(id: i64, access_hash: i64) -> Self {
        Self { id, access_hash }
    }
}

/// Decrypted message media content (MVP stub).
///
/// This is a simplified stub type for the MVP version. In a full implementation,
/// this would be a proper TL type with variants matching the Secret API schema:
/// - `decryptedMessageMediaEmpty`
/// - `decryptedMessageMediaPhoto`
/// - `decryptedMessageMediaDocument` (modern, layer 143+)
/// - `decryptedMessageMediaDocument46` (legacy, layer < 143)
/// - `decryptedMessageMediaVideo`
/// - etc.
///
/// # Examples
///
/// ```rust
/// use rustgram_secret_input_media::{InputEncryptedFile, DecryptedMessageMedia};
///
/// let file = InputEncryptedFile::new(12345, 67890);
///
/// // Create photo media
/// let photo = DecryptedMessageMedia::photo(file.clone());
/// assert!(matches!(photo, DecryptedMessageMedia::Photo { .. }));
///
/// // Create document media
/// let doc = DecryptedMessageMedia::document(file.clone(), "image/jpeg".to_string());
/// assert!(matches!(doc, DecryptedMessageMedia::Document { .. }));
///
/// // Create empty media
/// let empty = DecryptedMessageMedia::empty();
/// assert!(matches!(empty, DecryptedMessageMedia::Empty));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DecryptedMessageMedia {
    /// Empty media placeholder
    Empty,
    /// Photo media
    Photo {
        /// Encrypted file reference
        file: InputEncryptedFile,
    },
    /// Document media (generic file)
    Document {
        /// Encrypted file reference
        file: InputEncryptedFile,
        /// MIME type of the document
        mime_type: String,
    },
}

impl DecryptedMessageMedia {
    /// Creates an empty decrypted media.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::DecryptedMessageMedia;
    ///
    /// let empty = DecryptedMessageMedia::empty();
    /// assert!(matches!(empty, DecryptedMessageMedia::Empty));
    /// ```
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Creates a photo decrypted media.
    ///
    /// # Arguments
    ///
    /// * `file` - Encrypted file reference
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::{InputEncryptedFile, DecryptedMessageMedia};
    ///
    /// let file = InputEncryptedFile::new(12345, 67890);
    /// let photo = DecryptedMessageMedia::photo(file);
    /// assert!(matches!(photo, DecryptedMessageMedia::Photo { .. }));
    /// ```
    #[inline]
    #[must_use]
    pub fn photo(file: InputEncryptedFile) -> Self {
        Self::Photo { file }
    }

    /// Creates a document decrypted media.
    ///
    /// # Arguments
    ///
    /// * `file` - Encrypted file reference
    /// * `mime_type` - MIME type of the document
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::{InputEncryptedFile, DecryptedMessageMedia};
    ///
    /// let file = InputEncryptedFile::new(12345, 67890);
    /// let doc = DecryptedMessageMedia::document(file, "application/pdf".to_string());
    /// assert!(matches!(doc, DecryptedMessageMedia::Document { .. }));
    /// ```
    #[inline]
    #[must_use]
    pub fn document(file: InputEncryptedFile, mime_type: String) -> Self {
        Self::Document { file, mime_type }
    }

    /// Returns `true` if this is empty media.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::DecryptedMessageMedia;
    ///
    /// assert!(DecryptedMessageMedia::empty().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

/// Encrypted media container for secret chat messages.
///
/// This type holds both the encrypted file reference and the decrypted media
/// content for use in secret chats. It handles layer-specific format selection
/// and ensures compatibility with different secret chat protocol versions.
///
/// # Examples
///
/// ```rust
/// use rustgram_secret_input_media::{
///     SecretInputMedia, InputEncryptedFile, DecryptedMessageMedia
/// };
///
/// // Create empty media
/// let empty = SecretInputMedia::empty();
/// assert!(empty.is_empty());
/// assert!(empty.input_file().is_none());
/// assert!(empty.decrypted_media().is_none());
///
/// // Create media with content
/// let file = InputEncryptedFile::new(12345, 67890);
/// let decrypted = DecryptedMessageMedia::photo(file.clone());
/// let media = SecretInputMedia::new(file, decrypted);
/// assert!(!media.is_empty());
/// assert!(media.input_file().is_some());
/// assert!(media.decrypted_media().is_some());
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SecretInputMedia {
    /// Encrypted file reference (if present)
    input_file: Option<InputEncryptedFile>,
    /// Decrypted media content (if present)
    decrypted_media: Option<DecryptedMessageMedia>,
}

impl SecretInputMedia {
    /// Creates a new secret input media with the given components.
    ///
    /// # Arguments
    ///
    /// * `input_file` - Encrypted file reference
    /// * `decrypted_media` - Decrypted media content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::{
    ///     SecretInputMedia, InputEncryptedFile, DecryptedMessageMedia
    /// };
    ///
    /// let file = InputEncryptedFile::new(12345, 67890);
    /// let decrypted = DecryptedMessageMedia::photo(file.clone());
    /// let media = SecretInputMedia::new(file, decrypted);
    /// assert!(!media.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn new(input_file: InputEncryptedFile, decrypted_media: DecryptedMessageMedia) -> Self {
        Self {
            input_file: Some(input_file),
            decrypted_media: Some(decrypted_media),
        }
    }

    /// Creates an empty secret input media.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::SecretInputMedia;
    ///
    /// let media = SecretInputMedia::empty();
    /// assert!(media.is_empty());
    /// assert!(media.input_file().is_none());
    /// assert!(media.decrypted_media().is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self {
            input_file: None,
            decrypted_media: None,
        }
    }

    /// Returns `true` if this media is empty (no decrypted content).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::{
    ///     SecretInputMedia, InputEncryptedFile, DecryptedMessageMedia
    /// };
    ///
    /// let empty = SecretInputMedia::empty();
    /// assert!(empty.is_empty());
    ///
    /// let file = InputEncryptedFile::new(12345, 67890);
    /// let decrypted = DecryptedMessageMedia::photo(file.clone());
    /// let media = SecretInputMedia::new(file, decrypted);
    /// assert!(!media.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.decrypted_media.is_none()
    }

    /// Returns a reference to the encrypted file, if present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::{
    ///     SecretInputMedia, InputEncryptedFile, DecryptedMessageMedia
    /// };
    ///
    /// let media = SecretInputMedia::empty();
    /// assert!(media.input_file().is_none());
    ///
    /// let file = InputEncryptedFile::new(12345, 67890);
    /// let decrypted = DecryptedMessageMedia::photo(file.clone());
    /// let media = SecretInputMedia::new(file, decrypted);
    /// assert!(media.input_file().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub const fn input_file(&self) -> Option<&InputEncryptedFile> {
        self.input_file.as_ref()
    }

    /// Returns a reference to the decrypted media, if present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_secret_input_media::{
    ///     SecretInputMedia, InputEncryptedFile, DecryptedMessageMedia
    /// };
    ///
    /// let media = SecretInputMedia::empty();
    /// assert!(media.decrypted_media().is_none());
    ///
    /// let file = InputEncryptedFile::new(12345, 67890);
    /// let decrypted = DecryptedMessageMedia::photo(file.clone());
    /// let media = SecretInputMedia::new(file, decrypted);
    /// assert!(media.decrypted_media().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub const fn decrypted_media(&self) -> Option<&DecryptedMessageMedia> {
        self.decrypted_media.as_ref()
    }
}

impl Default for SecretInputMedia {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Dimensions tests =====

    #[test]
    fn test_dimensions_new_valid() {
        let dims = Dimensions::new(1920, 1080);
        assert_eq!(dims.width(), 1920);
        assert_eq!(dims.height(), 1080);
        assert!(dims.is_valid());
    }

    #[test]
    fn test_dimensions_new_zero_width() {
        let dims = Dimensions::new(0, 1080);
        assert_eq!(dims.width(), 0);
        assert_eq!(dims.height(), 1080);
        assert!(!dims.is_valid());
    }

    #[test]
    fn test_dimensions_new_zero_height() {
        let dims = Dimensions::new(1920, 0);
        assert_eq!(dims.width(), 1920);
        assert_eq!(dims.height(), 0);
        assert!(!dims.is_valid());
    }

    #[test]
    fn test_dimensions_width_accessor() {
        let dims = Dimensions::new(1280, 720);
        assert_eq!(dims.width(), 1280);
    }

    #[test]
    fn test_dimensions_height_accessor() {
        let dims = Dimensions::new(1280, 720);
        assert_eq!(dims.height(), 720);
    }

    #[test]
    fn test_dimensions_is_valid_true() {
        let dims = Dimensions::new(800, 600);
        assert!(dims.is_valid());
    }

    #[test]
    fn test_dimensions_is_valid_false_zero_width() {
        let dims = Dimensions::new(0, 600);
        assert!(!dims.is_valid());
    }

    #[test]
    fn test_dimensions_is_valid_false_zero_height() {
        let dims = Dimensions::new(800, 0);
        assert!(!dims.is_valid());
    }

    #[test]
    fn test_dimensions_display_format() {
        let dims = Dimensions::new(1920, 1080);
        assert_eq!(format!("{}", dims), "1920×1080");
    }

    #[test]
    fn test_dimensions_equality_equal() {
        let dims1 = Dimensions::new(1920, 1080);
        let dims2 = Dimensions::new(1920, 1080);
        assert_eq!(dims1, dims2);
    }

    #[test]
    fn test_dimensions_equality_different_width() {
        let dims1 = Dimensions::new(1920, 1080);
        let dims2 = Dimensions::new(1280, 1080);
        assert_ne!(dims1, dims2);
    }

    #[test]
    fn test_dimensions_equality_different_height() {
        let dims1 = Dimensions::new(1920, 1080);
        let dims2 = Dimensions::new(1920, 720);
        assert_ne!(dims1, dims2);
    }

    #[test]
    fn test_dimensions_copy_semantics() {
        let dims1 = Dimensions::new(1920, 1080);
        let dims2 = dims1; // Copy, not move
        assert_eq!(dims1, dims2);
        assert_eq!(dims1.width(), 1920);
    }

    // ===== SecretChatLayer tests =====

    #[test]
    fn test_secret_chat_layer_from_i32_73() {
        let layer = SecretChatLayer::from_i32(73);
        assert_eq!(layer, Some(SecretChatLayer::Default));
    }

    #[test]
    fn test_secret_chat_layer_from_i32_101() {
        let layer = SecretChatLayer::from_i32(101);
        assert_eq!(layer, Some(SecretChatLayer::NewEntities));
    }

    #[test]
    fn test_secret_chat_layer_from_i32_123() {
        let layer = SecretChatLayer::from_i32(123);
        assert_eq!(layer, Some(SecretChatLayer::DeleteMessagesOnClose));
    }

    #[test]
    fn test_secret_chat_layer_from_i32_143() {
        let layer = SecretChatLayer::from_i32(143);
        assert_eq!(layer, Some(SecretChatLayer::SupportBigFiles));
    }

    #[test]
    fn test_secret_chat_layer_from_i32_144() {
        let layer = SecretChatLayer::from_i32(144);
        assert_eq!(layer, Some(SecretChatLayer::SpoilerAndCustomEmojiEntities));
    }

    #[test]
    fn test_secret_chat_layer_from_i32_unknown() {
        let layer = SecretChatLayer::from_i32(999);
        assert_eq!(layer, None);
    }

    #[test]
    fn test_secret_chat_layer_from_i32_negative() {
        let layer = SecretChatLayer::from_i32(-1);
        assert_eq!(layer, None);
    }

    #[test]
    fn test_secret_chat_layer_as_i32_default() {
        assert_eq!(SecretChatLayer::Default.as_i32(), 73);
    }

    #[test]
    fn test_secret_chat_layer_as_i32_new_entities() {
        assert_eq!(SecretChatLayer::NewEntities.as_i32(), 101);
    }

    #[test]
    fn test_secret_chat_layer_as_i32_support_big_files() {
        assert_eq!(SecretChatLayer::SupportBigFiles.as_i32(), 143);
    }

    #[test]
    fn test_secret_chat_layer_current() {
        assert_eq!(
            SecretChatLayer::CURRENT,
            SecretChatLayer::SpoilerAndCustomEmojiEntities
        );
    }

    #[test]
    fn test_secret_chat_layer_supports_big_files_default() {
        assert!(!SecretChatLayer::Default.supports_big_files());
    }

    #[test]
    fn test_secret_chat_layer_supports_big_files_new_entities() {
        assert!(!SecretChatLayer::NewEntities.supports_big_files());
    }

    #[test]
    fn test_secret_chat_layer_supports_big_files_support_big_files() {
        assert!(SecretChatLayer::SupportBigFiles.supports_big_files());
    }

    #[test]
    fn test_secret_chat_layer_supports_big_files_current() {
        assert!(SecretChatLayer::CURRENT.supports_big_files());
    }

    #[test]
    fn test_secret_chat_layer_equality_same() {
        assert_eq!(SecretChatLayer::Default, SecretChatLayer::MTPROTO2);
    }

    #[test]
    fn test_secret_chat_layer_equality_different() {
        assert_ne!(SecretChatLayer::Default, SecretChatLayer::SupportBigFiles);
    }

    // ===== InputEncryptedFile tests =====

    #[test]
    fn test_input_encrypted_file_new() {
        let file = InputEncryptedFile::new(12345, 67890);
        assert_eq!(file.id, 12345);
        assert_eq!(file.access_hash, 67890);
    }

    #[test]
    fn test_input_encrypted_file_new_negative_id() {
        let file = InputEncryptedFile::new(-12345, 67890);
        assert_eq!(file.id, -12345);
        assert_eq!(file.access_hash, 67890);
    }

    #[test]
    fn test_input_encrypted_file_equality_equal() {
        let file1 = InputEncryptedFile::new(12345, 67890);
        let file2 = InputEncryptedFile::new(12345, 67890);
        assert_eq!(file1, file2);
    }

    #[test]
    fn test_input_encrypted_file_equality_different_id() {
        let file1 = InputEncryptedFile::new(12345, 67890);
        let file2 = InputEncryptedFile::new(54321, 67890);
        assert_ne!(file1, file2);
    }

    #[test]
    fn test_input_encrypted_file_equality_different_hash() {
        let file1 = InputEncryptedFile::new(12345, 67890);
        let file2 = InputEncryptedFile::new(12345, 9876);
        assert_ne!(file1, file2);
    }

    // ===== DecryptedMessageMedia tests =====

    #[test]
    fn test_decrypted_media_empty() {
        let media = DecryptedMessageMedia::empty();
        assert!(matches!(media, DecryptedMessageMedia::Empty));
        assert!(media.is_empty());
    }

    #[test]
    fn test_decrypted_media_photo() {
        let file = InputEncryptedFile::new(12345, 67890);
        let media = DecryptedMessageMedia::photo(file);
        assert!(matches!(media, DecryptedMessageMedia::Photo { .. }));
        assert!(!media.is_empty());
    }

    #[test]
    fn test_decrypted_media_document() {
        let file = InputEncryptedFile::new(12345, 67890);
        let media = DecryptedMessageMedia::document(file, "image/jpeg".to_string());
        assert!(matches!(media, DecryptedMessageMedia::Document { .. }));
        assert!(!media.is_empty());
    }

    #[test]
    fn test_decrypted_media_photo_accessor() {
        let file = InputEncryptedFile::new(12345, 67890);
        let media = DecryptedMessageMedia::photo(file.clone());

        match media {
            DecryptedMessageMedia::Photo { file: f } => {
                assert_eq!(f.id, 12345);
            }
            _ => panic!("Expected Photo variant"),
        }
    }

    #[test]
    fn test_decrypted_media_document_accessor() {
        let file = InputEncryptedFile::new(12345, 67890);
        let mime_type = "application/pdf".to_string();
        let media = DecryptedMessageMedia::document(file.clone(), mime_type.clone());

        match media {
            DecryptedMessageMedia::Document {
                file: f,
                mime_type: mt,
            } => {
                assert_eq!(f.id, 12345);
                assert_eq!(mt, "application/pdf");
            }
            _ => panic!("Expected Document variant"),
        }
    }

    #[test]
    fn test_decrypted_media_equality_empty() {
        let media1 = DecryptedMessageMedia::empty();
        let media2 = DecryptedMessageMedia::empty();
        assert_eq!(media1, media2);
    }

    #[test]
    fn test_decrypted_media_equality_photo_same() {
        let file = InputEncryptedFile::new(12345, 67890);
        let media1 = DecryptedMessageMedia::photo(file.clone());
        let media2 = DecryptedMessageMedia::photo(file);
        assert_eq!(media1, media2);
    }

    #[test]
    fn test_decrypted_media_equality_photo_different() {
        let file1 = InputEncryptedFile::new(12345, 67890);
        let file2 = InputEncryptedFile::new(54321, 67890);
        let media1 = DecryptedMessageMedia::photo(file1);
        let media2 = DecryptedMessageMedia::photo(file2);
        assert_ne!(media1, media2);
    }

    #[test]
    fn test_decrypted_media_equality_different_variants() {
        let file = InputEncryptedFile::new(12345, 67890);
        let photo = DecryptedMessageMedia::photo(file.clone());
        let doc = DecryptedMessageMedia::document(file, "image/jpeg".to_string());
        let empty = DecryptedMessageMedia::empty();

        assert_ne!(photo, doc);
        assert_ne!(photo, empty);
        assert_ne!(doc, empty);
    }

    // ===== SecretInputMedia tests =====

    #[test]
    fn test_secret_input_media_empty() {
        let media = SecretInputMedia::empty();
        assert!(media.is_empty());
        assert!(media.input_file().is_none());
        assert!(media.decrypted_media().is_none());
    }

    #[test]
    fn test_secret_input_media_is_empty_true() {
        let media = SecretInputMedia::empty();
        assert!(media.is_empty());
    }

    #[test]
    fn test_secret_input_media_is_empty_false() {
        let file = InputEncryptedFile::new(12345, 67890);
        let decrypted = DecryptedMessageMedia::photo(file.clone());
        let media = SecretInputMedia::new(file, decrypted);
        assert!(!media.is_empty());
    }

    #[test]
    fn test_secret_input_media_new_with_data() {
        let file = InputEncryptedFile::new(12345, 67890);
        let decrypted = DecryptedMessageMedia::photo(file.clone());
        let media = SecretInputMedia::new(file, decrypted);

        assert!(!media.is_empty());
        assert!(media.input_file().is_some());
        assert!(media.decrypted_media().is_some());
    }

    #[test]
    fn test_secret_input_media_input_file_accessor() {
        let file = InputEncryptedFile::new(12345, 67890);
        let decrypted = DecryptedMessageMedia::photo(file.clone());
        let media = SecretInputMedia::new(file, decrypted);

        let input_file = media.input_file();
        assert!(input_file.is_some());
        assert_eq!(input_file.unwrap().id, 12345);
    }

    #[test]
    fn test_secret_input_media_decrypted_media_accessor() {
        let file = InputEncryptedFile::new(12345, 67890);
        let decrypted = DecryptedMessageMedia::photo(file.clone());
        let media = SecretInputMedia::new(file, decrypted);

        let decrypted_media = media.decrypted_media();
        assert!(decrypted_media.is_some());
        assert!(matches!(
            decrypted_media.unwrap(),
            DecryptedMessageMedia::Photo { .. }
        ));
    }

    #[test]
    fn test_secret_input_media_input_file_none_for_empty() {
        let media = SecretInputMedia::empty();
        assert!(media.input_file().is_none());
    }

    #[test]
    fn test_secret_input_media_decrypted_media_none_for_empty() {
        let media = SecretInputMedia::empty();
        assert!(media.decrypted_media().is_none());
    }

    #[test]
    fn test_secret_input_media_equality_equal() {
        let file = InputEncryptedFile::new(12345, 67890);
        let decrypted = DecryptedMessageMedia::photo(file.clone());
        let media1 = SecretInputMedia::new(file.clone(), decrypted.clone());
        let media2 = SecretInputMedia::new(file, decrypted);
        assert_eq!(media1, media2);
    }

    #[test]
    fn test_secret_input_media_equality_different_file() {
        let file1 = InputEncryptedFile::new(12345, 67890);
        let file2 = InputEncryptedFile::new(54321, 67890);
        let decrypted1 = DecryptedMessageMedia::photo(file1.clone());
        let decrypted2 = DecryptedMessageMedia::photo(file2.clone());
        let media1 = SecretInputMedia::new(file1, decrypted1);
        let media2 = SecretInputMedia::new(file2, decrypted2);
        assert_ne!(media1, media2);
    }

    #[test]
    fn test_secret_input_media_equality_different_decrypted() {
        let file = InputEncryptedFile::new(12345, 67890);
        let decrypted1 = DecryptedMessageMedia::photo(file.clone());
        let decrypted2 = DecryptedMessageMedia::document(file.clone(), "image/jpeg".to_string());
        let media1 = SecretInputMedia::new(file.clone(), decrypted1);
        let media2 = SecretInputMedia::new(file, decrypted2);
        assert_ne!(media1, media2);
    }

    #[test]
    fn test_secret_input_media_equality_empty_vs_populated() {
        let file = InputEncryptedFile::new(12345, 67890);
        let decrypted = DecryptedMessageMedia::photo(file.clone());
        let populated = SecretInputMedia::new(file, decrypted);
        let empty = SecretInputMedia::empty();
        assert_ne!(populated, empty);
    }

    #[test]
    fn test_secret_input_media_default() {
        let media = SecretInputMedia::default();
        assert!(media.is_empty());
        assert_eq!(media, SecretInputMedia::empty());
    }

    // ===== Constants tests =====

    #[test]
    fn test_constants() {
        assert_eq!(MAX_LEGACY_FILE_SIZE, 2_000_000_000);
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-secret_input_media");
    }

    // ===== Integration tests =====

    #[test]
    fn test_secret_chat_layer_round_trip() {
        let original = SecretChatLayer::SupportBigFiles;
        let value = original.as_i32();
        let parsed = SecretChatLayer::from_i32(value);
        assert_eq!(parsed, Some(original));
    }

    #[test]
    fn test_all_secret_chat_layers_round_trip() {
        let layers = [
            SecretChatLayer::Default,
            SecretChatLayer::MTPROTO2,
            SecretChatLayer::NewEntities,
            SecretChatLayer::DeleteMessagesOnClose,
            SecretChatLayer::SupportBigFiles,
            SecretChatLayer::SpoilerAndCustomEmojiEntities,
        ];

        for layer in layers {
            let value = layer.as_i32();
            let parsed = SecretChatLayer::from_i32(value);
            assert_eq!(parsed, Some(layer));
        }
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Property: Dimensions::is_valid returns true iff both width > 0 and height > 0
    proptest! {
        #[test]
        fn prop_dimensions_is_valid(width in any::<u16>(), height in any::<u16>()) {
            let dims = Dimensions::new(width, height);
            assert_eq!(dims.is_valid(), width > 0 && height > 0);
        }
    }

    // Property: Dimensions width/height match input
    proptest! {
        #[test]
        fn prop_dimensions_accessors(width in any::<u16>(), height in any::<u16>()) {
            let dims = Dimensions::new(width, height);
            assert_eq!(dims.width(), width);
            assert_eq!(dims.height(), height);
        }
    }

    // Property: SecretChatLayer::from_i32 is inverse of as_i32
    proptest! {
        #[test]
        fn prop_secret_chat_layer_round_trip(value in 73i32..=144) {
            if let Some(layer) = SecretChatLayer::from_i32(value) {
                let round_trip = layer.as_i32();
                assert_eq!(round_trip, value);
            }
        }
    }

    // Property: SecretChatLayer::from_i32 returns None for unknown values
    proptest! {
        #[test]
        fn prop_secret_chat_layer_from_i32_unknown(value in -1000i32..72) {
            assert_eq!(SecretChatLayer::from_i32(value), None);
        }
    }

    // Property: SecretChatLayer::from_i32 returns None for unknown values (high range)
    proptest! {
        #[test]
        fn prop_secret_chat_layer_from_i32_unknown_high(value in 145i32..1000) {
            assert_eq!(SecretChatLayer::from_i32(value), None);
        }
    }

    // Property: InputEncryptedFile equality is reflexive
    proptest! {
        #[test]
        fn prop_input_encrypted_file_reflexive(id in any::<i64>(), hash in any::<i64>()) {
            let file = InputEncryptedFile::new(id, hash);
            assert_eq!(file, file);
        }
    }

    // Property: InputEncryptedFile clone is equal
    proptest! {
        #[test]
        fn prop_input_encrypted_file_clone_eq(id in any::<i64>(), hash in any::<i64>()) {
            let file1 = InputEncryptedFile::new(id, hash);
            let file2 = file1.clone();
            assert_eq!(file1, file2);
        }
    }

    // Property: DecryptedMessageMedia::is_empty true only for Empty variant
    #[test]
    fn prop_decrypted_media_is_empty_empty() {
        let media = DecryptedMessageMedia::empty();
        assert!(media.is_empty());
    }

    // Property: DecryptedMessageMedia::photo is never empty
    proptest! {
        #[test]
        fn prop_decrypted_media_photo_not_empty(id in any::<i64>(), hash in any::<i64>()) {
            let file = InputEncryptedFile::new(id, hash);
            let media = DecryptedMessageMedia::photo(file);
            assert!(!media.is_empty());
        }
    }

    // Property: DecryptedMessageMedia::document is never empty
    proptest! {
        #[test]
        fn prop_decrypted_media_document_not_empty(
            id in any::<i64>(),
            hash in any::<i64>(),
            mime_type in ".*"
        ) {
            let file = InputEncryptedFile::new(id, hash);
            let media = DecryptedMessageMedia::document(file, mime_type);
            assert!(!media.is_empty());
        }
    }

    // Property: SecretInputMedia::empty is always empty
    #[test]
    fn prop_secret_input_media_empty_is_empty() {
        let media = SecretInputMedia::empty();
        assert!(media.is_empty());
    }

    // Property: SecretInputMedia::new is never empty
    proptest! {
        #[test]
        fn prop_secret_input_media_new_not_empty(id in any::<i64>(), hash in any::<i64>()) {
            let file = InputEncryptedFile::new(id, hash);
            let decrypted = DecryptedMessageMedia::photo(file.clone());
            let media = SecretInputMedia::new(file, decrypted);
            assert!(!media.is_empty());
        }
    }

    // Property: SecretInputMedia::new returns Some for accessors
    proptest! {
        #[test]
        fn prop_secret_input_media_new_accessors_some(id in any::<i64>(), hash in any::<i64>()) {
            let file = InputEncryptedFile::new(id, hash);
            let decrypted = DecryptedMessageMedia::photo(file.clone());
            let media = SecretInputMedia::new(file, decrypted);
            assert!(media.input_file().is_some());
            assert!(media.decrypted_media().is_some());
        }
    }

    // Property: SecretInputMedia::empty returns None for accessors
    #[test]
    fn prop_secret_input_media_empty_accessors_none() {
        let media = SecretInputMedia::empty();
        assert!(media.input_file().is_none());
        assert!(media.decrypted_media().is_none());
    }

    // Property: SecretInputMedia equality is reflexive
    proptest! {
        #[test]
        fn prop_secret_input_media_reflexive(id in any::<i64>(), hash in any::<i64>()) {
            let file = InputEncryptedFile::new(id, hash);
            let decrypted = DecryptedMessageMedia::photo(file.clone());
            let media = SecretInputMedia::new(file, decrypted);
            assert_eq!(media, media);
        }
    }

    // Property: SecretInputMedia clone is equal
    proptest! {
        #[test]
        fn prop_secret_input_media_clone_eq(id in any::<i64>(), hash in any::<i64>()) {
            let file = InputEncryptedFile::new(id, hash);
            let decrypted = DecryptedMessageMedia::photo(file.clone());
            let media1 = SecretInputMedia::new(file, decrypted);
            let media2 = media1.clone();
            assert_eq!(media1, media2);
        }
    }

    // Property: Dimensions equality is reflexive
    proptest! {
        #[test]
        fn prop_dimensions_reflexive(width in any::<u16>(), height in any::<u16>()) {
            let dims = Dimensions::new(width, height);
            assert_eq!(dims, dims);
        }
    }

    // Property: Dimensions clone is equal
    proptest! {
        #[test]
        fn prop_dimensions_clone_eq(width in any::<u16>(), height in any::<u16>()) {
            let dims1 = Dimensions::new(width, height);
            let dims2 = dims1.clone();
            assert_eq!(dims1, dims2);
        }
    }

    // Property: SecretChatLayer equality is reflexive
    proptest! {
        #[test]
        fn prop_secret_chat_layer_reflexive(layer_value in 73i32..=144) {
            if let Some(layer) = SecretChatLayer::from_i32(layer_value) {
                assert_eq!(layer, layer);
            }
        }
    }

    // Property: supports_big_files is true for layers >= 143
    proptest! {
        #[test]
        fn prop_supports_big_files_threshold(layer_value in 73i32..=144) {
            if let Some(layer) = SecretChatLayer::from_i32(layer_value) {
                assert_eq!(layer.supports_big_files(), layer_value >= 143);
            }
        }
    }

    // Property: Display format contains dimensions values
    proptest! {
        #[test]
        fn prop_dimensions_display_contains_values(width in 1u16.., height in 1u16..) {
            let dims = Dimensions::new(width, height);
            let s = format!("{}", dims);
            assert!(s.contains(&width.to_string()));
            assert!(s.contains(&height.to_string()));
        }
    }
}
