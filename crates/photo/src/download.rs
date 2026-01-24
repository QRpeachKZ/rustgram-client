//! Photo download module for Telegram profile photos.
//!
//! This module provides functionality for downloading profile photos from Telegram
//! servers using the `upload.getFile` API method.
//!
//! # Features
//!
//! - Photo download with automatic location resolution
//! - LRU cache with configurable size limit
//! - Support for user profile photos, chat photos, and channel photos
//! - TL serialization/deserialization for upload.getFile API
//!
//! # Architecture
//!
//! The download module is designed to work with the `photo` crate and provides:
//! - [`PhotoDownloader`] - Main downloader with caching
//! - [`InputPhotoFileLocation`] - Photo location specification
//! - [`PhotoData`] - Downloaded photo data container
//! - [`PhotoDownloadError`] - Error types for download operations
//!
//! # Examples
//!
//! ```no_run
//! use rustgram_photo::download::{PhotoDownloader, InputPhotoFileLocation};
//! use rustgram_types::{UserId, AccessHash};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let downloader = PhotoDownloader::new();
//!
//! let location = InputPhotoFileLocation::user_profile(
//!     UserId::new(123).unwrap(),
//!     AccessHash::new(456),
//!     789,
//! );
//!
//! // Download photo (actual network call requires network client)
//! // let data = downloader.download_photo(&location).await?;
//! # Ok(())
//! # }
//! ```

use bytes::BytesMut;
use lru::LruCache;
use rustgram_types::{AccessHash, ChannelId, ChatId, TlDeserialize, TlHelper, TlSerialize, UserId};
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::Arc;
use thiserror::Error;

/// Default photo cache size (100 MB).
pub const DEFAULT_CACHE_SIZE_BYTES: u64 = 100 * 1024 * 1024;

/// Maximum photo size (5 MB).
pub const MAX_PHOTO_SIZE_BYTES: u64 = 5 * 1024 * 1024;

/// TL constructor for `upload.getFile`.
/// Verified from telegram_api.tl
pub const UPLOAD_GET_FILE: u32 = 0xbe5335be;

/// TL constructor for `upload.file`.
pub const UPLOAD_FILE: u32 = 0x96a18d5;

/// TL constructor for `inputFileLocation`.
pub const INPUT_FILE_LOCATION: u32 = 0x1bea9f9a;

/// TL constructor for `inputFileEmpty`.
pub const INPUT_FILE_EMPTY: u32 = 0x96a18d5;

/// Photo download error types.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum PhotoDownloadError {
    /// No network client configured.
    #[error("no network client configured")]
    NoClient,

    /// Photo too large.
    #[error("photo too large: {size} bytes (max: {max} bytes)")]
    PhotoTooLarge { size: u64, max: u64 },

    /// Request timed out.
    #[error("request timed out after {0:?}")]
    Timeout(std::time::Duration),

    /// API error with code and message.
    #[error("API error {code}: {message}")]
    ApiError {
        /// Error code.
        code: i32,
        /// Error message.
        message: String,
    },

    /// Serialization error.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error.
    #[error("deserialization error: {0}")]
    DeserializationError(String),

    /// Cache full.
    #[error("cache full (size: {size} bytes, max: {max} bytes)")]
    CacheFull { size: u64, max: u64 },

    /// Invalid file location.
    #[error("invalid file location: {0}")]
    InvalidFileLocation(String),

    /// Invalid user ID.
    #[error("invalid user ID: {0}")]
    InvalidUserId(i64),

    /// Invalid access hash.
    #[error("invalid access hash")]
    InvalidAccessHash,
}

impl PhotoDownloadError {
    /// Creates an API error from code and message.
    pub fn api_error(code: i32, message: impl Into<String>) -> Self {
        Self::ApiError {
            code,
            message: message.into(),
        }
    }

    /// Creates a serialization error.
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::SerializationError(message.into())
    }

    /// Creates a deserialization error.
    pub fn deserialization(message: impl Into<String>) -> Self {
        Self::DeserializationError(message.into())
    }

    /// Returns `true` if this is a retryable error.
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::Timeout(_) | Self::CacheFull { .. })
    }
}

/// Photo data downloaded from Telegram.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhotoData {
    /// Raw photo data bytes.
    pub data: Vec<u8>,
    /// MIME type of the photo.
    pub mime_type: String,
    /// Photo size in bytes.
    pub size: usize,
}

impl PhotoData {
    /// Creates new photo data.
    #[must_use]
    pub fn new(data: Vec<u8>, mime_type: String) -> Self {
        let size = data.len();
        Self {
            data,
            mime_type,
            size,
        }
    }

    /// Creates new photo data with JPEG MIME type.
    #[must_use]
    pub fn jpeg(data: Vec<u8>) -> Self {
        Self::new(data, "image/jpeg".to_string())
    }

    /// Creates new photo data with PNG MIME type.
    #[must_use]
    pub fn png(data: Vec<u8>) -> Self {
        Self::new(data, "image/png".to_string())
    }

    /// Creates new photo data with GIF MIME type.
    #[must_use]
    pub fn gif(data: Vec<u8>) -> Self {
        Self::new(data, "image/gif".to_string())
    }

    /// Creates new photo data with WEBP MIME type.
    #[must_use]
    pub fn webp(data: Vec<u8>) -> Self {
        Self::new(data, "image/webp".to_string())
    }

    /// Returns `true` if this photo data is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the photo data length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

/// Input photo file location for download.
///
/// Represents the location of a photo file on Telegram's servers.
/// This is used to specify which photo to download via `upload.getFile`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputPhotoFileLocation {
    /// User profile photo.
    UserProfile {
        /// User ID
        user_id: UserId,
        /// Access hash for authentication
        access_hash: AccessHash,
        /// Photo ID
        photo_id: i64,
    },
    /// Chat photo.
    ChatPhoto {
        /// Chat ID
        chat_id: ChatId,
        /// Access hash for authentication
        access_hash: AccessHash,
        /// Photo ID
        photo_id: i64,
    },
    /// Channel photo.
    ChannelPhoto {
        /// Channel ID
        channel_id: ChannelId,
        /// Access hash for authentication
        access_hash: AccessHash,
        /// Photo ID
        photo_id: i64,
    },
}

impl InputPhotoFileLocation {
    /// Creates a user profile photo location.
    #[must_use]
    pub fn user_profile(user_id: UserId, access_hash: AccessHash, photo_id: i64) -> Self {
        Self::UserProfile {
            user_id,
            access_hash,
            photo_id,
        }
    }

    /// Creates a chat photo location.
    #[must_use]
    pub fn chat_photo(chat_id: ChatId, access_hash: AccessHash, photo_id: i64) -> Self {
        Self::ChatPhoto {
            chat_id,
            access_hash,
            photo_id,
        }
    }

    /// Creates a channel photo location.
    #[must_use]
    pub fn channel_photo(channel_id: ChannelId, access_hash: AccessHash, photo_id: i64) -> Self {
        Self::ChannelPhoto {
            channel_id,
            access_hash,
            photo_id,
        }
    }

    /// Returns the constructor ID for this location.
    #[must_use]
    pub fn constructor_id(&self) -> u32 {
        // All photo locations use inputPeerPhoto-like constructors
        match self {
            Self::UserProfile { .. } => 0x3d8d4338, // inputPeerProfilePhoto
            Self::ChatPhoto { .. } => 0x3d8d4338,   // inputPeerPhoto
            Self::ChannelPhoto { .. } => 0x3d8d4338, // inputChatPhoto
        }
    }

    /// Serializes this location to TL format.
    pub fn serialize_tl(&self, buf: &mut BytesMut) -> Result<(), rustgram_types::TypeError> {
        // Write constructor ID
        TlHelper::write_constructor_id(buf, self.constructor_id());

        match self {
            Self::UserProfile {
                user_id,
                access_hash,
                photo_id,
            } => {
                TlHelper::write_i64(buf, user_id.get());
                TlHelper::write_i64(buf, access_hash.get());
                TlHelper::write_i64(buf, *photo_id);
            }
            Self::ChatPhoto {
                chat_id,
                access_hash,
                photo_id,
            } => {
                TlHelper::write_i64(buf, chat_id.get());
                TlHelper::write_i64(buf, access_hash.get());
                TlHelper::write_i64(buf, *photo_id);
            }
            Self::ChannelPhoto {
                channel_id,
                access_hash,
                photo_id,
            } => {
                TlHelper::write_i64(buf, channel_id.get());
                TlHelper::write_i64(buf, access_hash.get());
                TlHelper::write_i64(buf, *photo_id);
            }
        }

        Ok(())
    }
}

/// Input file location for upload.getFile API.
///
/// Represents a file location on Telegram's servers for download.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputFileLocation {
    /// Empty location (placeholder).
    Empty,
    /// Local file location.
    Local {
        /// Volume ID
        volume_id: i64,
        /// Local ID within volume
        local_id: i32,
        /// Secret for authentication
        secret: i64,
    },
    /// Encrypted file location.
    Encrypted {
        /// File ID
        id: i64,
        /// Access hash
        access_hash: i64,
    },
    /// Document file location.
    Document {
        /// Document ID
        id: i64,
        /// Access hash
        access_hash: i64,
        /// File reference
        file_reference: Vec<u8>,
    },
}

impl InputFileLocation {
    /// Creates an empty file location.
    #[must_use]
    pub const fn empty() -> Self {
        Self::Empty
    }

    /// Creates a local file location.
    #[must_use]
    pub const fn local(volume_id: i64, local_id: i32, secret: i64) -> Self {
        Self::Local {
            volume_id,
            local_id,
            secret,
        }
    }

    /// Creates an encrypted file location.
    #[must_use]
    pub const fn encrypted(id: i64, access_hash: i64) -> Self {
        Self::Encrypted { id, access_hash }
    }

    /// Creates a document file location.
    #[must_use]
    pub fn document(id: i64, access_hash: i64, file_reference: Vec<u8>) -> Self {
        Self::Document {
            id,
            access_hash,
            file_reference,
        }
    }
}

impl TlSerialize for InputFileLocation {
    fn serialize_tl(&self, buf: &mut BytesMut) -> Result<(), rustgram_types::TypeError> {
        match self {
            Self::Empty => {
                TlHelper::write_constructor_id(buf, INPUT_FILE_EMPTY);
            }
            Self::Local {
                volume_id,
                local_id,
                secret,
            } => {
                TlHelper::write_constructor_id(buf, INPUT_FILE_LOCATION);
                TlHelper::write_i64(buf, *volume_id);
                TlHelper::write_i32(buf, *local_id);
                TlHelper::write_i64(buf, *secret);
            }
            Self::Encrypted { id, access_hash } => {
                TlHelper::write_constructor_id(buf, 0x1816d527); // inputEncryptedFileLocation
                TlHelper::write_i64(buf, *id);
                TlHelper::write_i64(buf, *access_hash);
            }
            Self::Document {
                id,
                access_hash,
                file_reference,
            } => {
                TlHelper::write_constructor_id(buf, 0xbc7fc6cd); // inputDocumentFileLocation
                TlHelper::write_i64(buf, *id);
                TlHelper::write_i64(buf, *access_hash);
                TlHelper::write_bytes(buf, file_reference);
            }
        }

        Ok(())
    }
}

/// Request for `upload.getFile`.
///
/// This is the TL request structure for downloading files from Telegram.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFileRequest {
    /// File location to download
    pub location: InputFileLocation,
    /// File size limit (optional)
    pub size_limit: Option<i32>,
}

impl GetFileRequest {
    /// TL constructor ID for upload.getFile.
    pub const CONSTRUCTOR_ID: u32 = UPLOAD_GET_FILE;

    /// Creates a new get file request.
    #[must_use]
    pub fn new(location: InputFileLocation) -> Self {
        Self {
            location,
            size_limit: Some(MAX_PHOTO_SIZE_BYTES as i32),
        }
    }

    /// Creates a new request with custom size limit.
    #[must_use]
    pub fn with_size_limit(location: InputFileLocation, limit: i32) -> Self {
        Self {
            location,
            size_limit: Some(limit),
        }
    }

    /// Creates a new request without size limit.
    #[must_use]
    pub fn without_size_limit(location: InputFileLocation) -> Self {
        Self {
            location,
            size_limit: None,
        }
    }

    /// Returns the constructor ID for this request.
    #[must_use]
    pub const fn constructor_id(&self) -> u32 {
        Self::CONSTRUCTOR_ID
    }
}

impl TlSerialize for GetFileRequest {
    fn serialize_tl(&self, buf: &mut BytesMut) -> Result<(), rustgram_types::TypeError> {
        // Write constructor ID
        TlHelper::write_constructor_id(buf, Self::CONSTRUCTOR_ID);

        // Serialize the InputFileLocation
        self.location.serialize_tl(buf)?;

        // Write size limit
        TlHelper::write_i32(buf, self.size_limit.unwrap_or(0));

        Ok(())
    }
}

/// Response from `upload.getFile`.
///
/// Contains the downloaded file data and metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFileResponse {
    /// File type (e.g., "image/jpeg")
    pub mime_type: String,
    /// File data bytes
    pub bytes: Vec<u8>,
    /// File size in bytes
    pub size: i32,
}

impl GetFileResponse {
    /// Creates a new file response.
    #[must_use]
    pub fn new(mime_type: String, bytes: Vec<u8>, size: i32) -> Self {
        Self {
            mime_type,
            bytes,
            size,
        }
    }

    /// Returns `true` if the response is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns the number of bytes in the response.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl TlDeserialize for GetFileResponse {
    fn deserialize_tl(
        buf: &mut rustgram_types::tl::Bytes,
    ) -> Result<Self, rustgram_types::TypeError> {
        // Read constructor ID
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            // upload.file#96a18d5
            UPLOAD_FILE => {
                let mime_type = TlHelper::read_string(buf)?;
                let bytes = TlHelper::read_bytes(buf)?;
                let size = TlHelper::read_i32(buf)?;

                Ok(Self {
                    mime_type,
                    bytes,
                    size,
                })
            }
            // upload.fileCdnRedirect#ea52fe5a
            0xea52fe5a => {
                // CDN redirect - return minimal response
                Ok(Self {
                    mime_type: String::new(),
                    bytes: Vec::new(),
                    size: 0,
                })
            }
            _ => Err(rustgram_types::TypeError::DeserializationError(format!(
                "Unknown constructor ID for GetFileResponse: 0x{:08x}",
                constructor_id
            ))),
        }
    }
}

/// Photo downloader with caching support.
///
/// Provides photo download functionality with LRU caching and size limits.
///
/// # Example
///
/// ```no_run
/// use rustgram_photo::download::PhotoDownloader;
///
/// let downloader = PhotoDownloader::new();
/// assert_eq!(downloader.max_cache_size(), 100 * 1024 * 1024);
/// ```
#[derive(Clone)]
pub struct PhotoDownloader {
    /// LRU cache for downloaded photos
    cache: Arc<parking_lot::Mutex<LruCache<PhotoCacheKey, PhotoData>>>,
    /// Current cache size in bytes
    cache_size: Arc<parking_lot::Mutex<u64>>,
    /// Maximum cache size in bytes
    max_cache_size: u64,
}

impl fmt::Debug for PhotoDownloader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PhotoDownloader")
            .field("max_cache_size", &self.max_cache_size)
            .field("cache_size", &self.cache_size)
            .finish()
    }
}

impl Default for PhotoDownloader {
    fn default() -> Self {
        Self::new()
    }
}

impl PhotoDownloader {
    /// Creates a new photo downloader with default settings.
    ///
    /// Uses a cache capacity of 1000 entries and 100 MB size limit.
    #[must_use]
    pub fn new() -> Self {
        let capacity = NonZeroUsize::new(1000).expect("1000 > 0");

        Self {
            cache: Arc::new(parking_lot::Mutex::new(LruCache::new(capacity))),
            cache_size: Arc::new(parking_lot::Mutex::new(0)),
            max_cache_size: DEFAULT_CACHE_SIZE_BYTES,
        }
    }

    /// Creates a new photo downloader with custom cache size.
    ///
    /// # Arguments
    ///
    /// * `max_cache_size` - Maximum cache size in bytes
    #[must_use]
    pub fn with_cache_size(max_cache_size: u64) -> Self {
        let capacity = NonZeroUsize::new(1000).expect("1000 > 0");

        Self {
            cache: Arc::new(parking_lot::Mutex::new(LruCache::new(capacity))),
            cache_size: Arc::new(parking_lot::Mutex::new(0)),
            max_cache_size,
        }
    }

    /// Creates a new photo downloader with custom capacity and size.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of cache entries
    /// * `max_cache_size` - Maximum cache size in bytes
    #[must_use]
    pub fn with_capacity(capacity: usize, max_cache_size: u64) -> Self {
        let nz_capacity =
            NonZeroUsize::new(capacity).unwrap_or_else(|| NonZeroUsize::new(1).expect("1 > 0"));

        Self {
            cache: Arc::new(parking_lot::Mutex::new(LruCache::new(nz_capacity))),
            cache_size: Arc::new(parking_lot::Mutex::new(0)),
            max_cache_size,
        }
    }

    /// Downloads a photo from Telegram.
    ///
    /// # Arguments
    ///
    /// * `location` - Photo location to download
    ///
    /// # Returns
    ///
    /// Downloaded photo data
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Network request fails
    /// - Photo is too large
    /// - Cache is full
    ///
    /// # Note
    ///
    /// This is a placeholder implementation. Actual download requires
    /// a network client integration.
    pub async fn download_photo(
        &self,
        location: &InputPhotoFileLocation,
    ) -> Result<PhotoData, PhotoDownloadError> {
        // Check cache first
        let cache_key = PhotoCacheKey::from_location(location);

        {
            let cache = self.cache.lock();
            if let Some(data) = cache.peek(&cache_key) {
                return Ok(data.clone());
            }
        }

        // Create file location from photo location
        let file_location = self.file_location_from_photo(location)?;

        // Check size limit
        if let InputFileLocation::Local { .. } = file_location {
            // Size would be checked during download
        }

        // TODO: Actually download the photo via network
        // For now, return placeholder error
        Err(PhotoDownloadError::NoClient)
    }

    /// Converts InputPhotoFileLocation to InputFileLocation.
    fn file_location_from_photo(
        &self,
        photo_location: &InputPhotoFileLocation,
    ) -> Result<InputFileLocation, PhotoDownloadError> {
        // Convert the photo location to the appropriate InputFileLocation
        // This is a simplified implementation for the photo crate
        match photo_location {
            InputPhotoFileLocation::UserProfile { photo_id, .. } => Ok(InputFileLocation::Local {
                volume_id: 0,
                local_id: *photo_id as i32,
                secret: 0,
            }),
            InputPhotoFileLocation::ChatPhoto { photo_id, .. } => Ok(InputFileLocation::Local {
                volume_id: 0,
                local_id: *photo_id as i32,
                secret: 0,
            }),
            InputPhotoFileLocation::ChannelPhoto { photo_id, .. } => Ok(InputFileLocation::Local {
                volume_id: 0,
                local_id: *photo_id as i32,
                secret: 0,
            }),
        }
    }

    /// Adds photo data to cache.
    ///
    /// # Arguments
    ///
    /// * `key` - Cache key
    /// * `data` - Photo data to cache
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful
    ///
    /// # Errors
    ///
    /// Returns error if cache is full or data is too large
    pub fn add_to_cache(
        &self,
        key: PhotoCacheKey,
        data: PhotoData,
    ) -> Result<(), PhotoDownloadError> {
        let mut cache = self.cache.lock();
        let mut cache_size = self.cache_size.lock();

        let data_size = data.size as u64;

        // Check if single item exceeds cache size
        if data_size > self.max_cache_size {
            return Err(PhotoDownloadError::CacheFull {
                size: data_size,
                max: self.max_cache_size,
            });
        }

        // Evict entries until we have space
        while *cache_size + data_size > self.max_cache_size {
            if let Some((_, evicted)) = cache.pop_lru() {
                *cache_size = cache_size.saturating_sub(evicted.size as u64);
            } else {
                // Cache is empty but still too large
                return Err(PhotoDownloadError::CacheFull {
                    size: data_size,
                    max: self.max_cache_size,
                });
            }
        }

        cache.put(key, data);
        *cache_size += data_size;

        Ok(())
    }

    /// Clears the photo cache.
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock();
        let mut cache_size = self.cache_size.lock();

        cache.clear();
        *cache_size = 0;
    }

    /// Returns the current cache size in bytes.
    #[must_use]
    pub fn cache_size(&self) -> u64 {
        *self.cache_size.lock()
    }

    /// Returns the maximum cache size in bytes.
    #[must_use]
    pub const fn max_cache_size(&self) -> u64 {
        self.max_cache_size
    }

    /// Returns the cache utilization as a percentage (0.0 to 100.0).
    #[must_use]
    pub fn cache_utilization(&self) -> f64 {
        let size = self.cache_size();
        if self.max_cache_size == 0 {
            0.0
        } else {
            (size as f64 / self.max_cache_size as f64) * 100.0
        }
    }

    /// Returns the number of entries in the cache.
    #[must_use]
    pub fn cache_len(&self) -> usize {
        self.cache.lock().len()
    }

    /// Checks if the cache is empty.
    #[must_use]
    pub fn cache_is_empty(&self) -> bool {
        self.cache.lock().is_empty()
    }
}

/// Cache key for photos.
///
/// Used to uniquely identify cached photos by their location.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhotoCacheKey {
    /// Type identifier (user/chat/channel)
    pub type_id: u8,
    /// Entity ID (user/chat/channel ID)
    pub entity_id: u64,
    /// Photo ID
    pub photo_id: i64,
}

impl PhotoCacheKey {
    /// Creates a cache key from a photo location.
    #[must_use]
    pub fn from_location(location: &InputPhotoFileLocation) -> Self {
        match location {
            InputPhotoFileLocation::UserProfile {
                user_id, photo_id, ..
            } => Self {
                type_id: 1,
                entity_id: user_id.get() as u64,
                photo_id: *photo_id,
            },
            InputPhotoFileLocation::ChatPhoto {
                chat_id, photo_id, ..
            } => Self {
                type_id: 2,
                entity_id: chat_id.get() as u64,
                photo_id: *photo_id,
            },
            InputPhotoFileLocation::ChannelPhoto {
                channel_id,
                photo_id,
                ..
            } => Self {
                type_id: 3,
                entity_id: channel_id.get() as u64,
                photo_id: *photo_id,
            },
        }
    }

    /// Creates a new cache key.
    #[must_use]
    pub const fn new(type_id: u8, entity_id: u64, photo_id: i64) -> Self {
        Self {
            type_id,
            entity_id,
            photo_id,
        }
    }

    /// Creates a cache key for a user profile photo.
    #[must_use]
    pub fn user_profile(user_id: UserId, photo_id: i64) -> Self {
        Self {
            type_id: 1,
            entity_id: user_id.get() as u64,
            photo_id,
        }
    }

    /// Creates a cache key for a chat photo.
    #[must_use]
    pub fn chat_photo(chat_id: ChatId, photo_id: i64) -> Self {
        Self {
            type_id: 2,
            entity_id: chat_id.get() as u64,
            photo_id,
        }
    }

    /// Creates a cache key for a channel photo.
    #[must_use]
    pub fn channel_photo(channel_id: ChannelId, photo_id: i64) -> Self {
        Self {
            type_id: 3,
            entity_id: channel_id.get() as u64,
            photo_id,
        }
    }
}

/// Version information for the download module.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-photo";

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Constants tests
    // =========================================================================

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_CACHE_SIZE_BYTES, 100 * 1024 * 1024);
        assert_eq!(MAX_PHOTO_SIZE_BYTES, 5 * 1024 * 1024);
        assert_eq!(UPLOAD_GET_FILE, 0xbe5335be);
        assert_eq!(UPLOAD_FILE, 0x96a18d5);
        assert_eq!(INPUT_FILE_LOCATION, 0x1bea9f9a);
        assert_eq!(INPUT_FILE_EMPTY, 0x96a18d5);
    }

    // =========================================================================
    // PhotoDownloadError tests
    // =========================================================================

    #[test]
    fn test_error_api_error() {
        let err = PhotoDownloadError::api_error(404, "Not found");
        assert!(matches!(
            err,
            PhotoDownloadError::ApiError { code: 404, .. }
        ));
        assert_eq!(err.to_string(), "API error 404: Not found");
    }

    #[test]
    fn test_error_serialization() {
        let err = PhotoDownloadError::serialization("Invalid format");
        assert!(matches!(err, PhotoDownloadError::SerializationError(_)));
        assert!(err.to_string().contains("Invalid format"));
    }

    #[test]
    fn test_error_retryable() {
        assert!(PhotoDownloadError::Timeout(std::time::Duration::from_secs(1)).is_retryable());
        assert!(PhotoDownloadError::CacheFull { size: 100, max: 50 }.is_retryable());
        assert!(!PhotoDownloadError::api_error(404, "Not found").is_retryable());
        assert!(!PhotoDownloadError::NoClient.is_retryable());
    }

    #[test]
    fn test_error_clone() {
        let err1 = PhotoDownloadError::api_error(500, "Error");
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_error_partial_eq() {
        let err1 = PhotoDownloadError::NoClient;
        let err2 = PhotoDownloadError::NoClient;
        assert_eq!(err1, err2);

        let err3 = PhotoDownloadError::api_error(500, "Error");
        assert_ne!(err1, err3);
    }

    // =========================================================================
    // PhotoData tests
    // =========================================================================

    #[test]
    fn test_photo_data_new() {
        let data = vec![1, 2, 3, 4];
        let mime = "image/jpeg".to_string();
        let photo = PhotoData::new(data.clone(), mime.clone());

        assert_eq!(photo.data, data);
        assert_eq!(photo.mime_type, mime);
        assert_eq!(photo.size, 4);
    }

    #[test]
    fn test_photo_data_jpeg() {
        let data = vec![1, 2, 3];
        let photo = PhotoData::jpeg(data);

        assert_eq!(photo.mime_type, "image/jpeg");
        assert_eq!(photo.size, 3);
    }

    #[test]
    fn test_photo_data_png() {
        let data = vec![1, 2, 3];
        let photo = PhotoData::png(data);

        assert_eq!(photo.mime_type, "image/png");
        assert_eq!(photo.size, 3);
    }

    #[test]
    fn test_photo_data_gif() {
        let data = vec![1, 2, 3];
        let photo = PhotoData::gif(data);

        assert_eq!(photo.mime_type, "image/gif");
        assert_eq!(photo.size, 3);
    }

    #[test]
    fn test_photo_data_webp() {
        let data = vec![1, 2, 3];
        let photo = PhotoData::webp(data);

        assert_eq!(photo.mime_type, "image/webp");
        assert_eq!(photo.size, 3);
    }

    #[test]
    fn test_photo_data_empty() {
        let photo = PhotoData::jpeg(vec![]);

        assert!(photo.is_empty());
        assert_eq!(photo.len(), 0);
    }

    #[test]
    fn test_photo_data_clone() {
        let photo1 = PhotoData::jpeg(vec![1, 2, 3]);
        let photo2 = photo1.clone();

        assert_eq!(photo1, photo2);
    }

    #[test]
    fn test_photo_data_equality() {
        let data = vec![1, 2, 3];
        let photo1 = PhotoData::new(data.clone(), "image/jpeg".to_string());
        let photo2 = PhotoData::new(data, "image/jpeg".to_string());

        assert_eq!(photo1, photo2);
    }

    #[test]
    fn test_photo_data_partial_eq() {
        let photo1 = PhotoData::jpeg(vec![1, 2, 3]);
        let photo2 = PhotoData::jpeg(vec![1, 2, 3]);
        assert_eq!(photo1, photo2);

        let photo3 = PhotoData::png(vec![1, 2, 3]);
        assert_ne!(photo1, photo3);
    }

    // =========================================================================
    // InputPhotoFileLocation tests
    // =========================================================================

    #[test]
    fn test_input_photo_user_profile() {
        let user_id = UserId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::user_profile(user_id, access_hash, 789);

        assert_eq!(location.constructor_id(), 0x3d8d4338);
    }

    #[test]
    fn test_input_photo_chat_photo() {
        let chat_id = ChatId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::chat_photo(chat_id, access_hash, 789);

        assert_eq!(location.constructor_id(), 0x3d8d4338);
    }

    #[test]
    fn test_input_photo_channel_photo() {
        let channel_id = ChannelId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::channel_photo(channel_id, access_hash, 789);

        assert_eq!(location.constructor_id(), 0x3d8d4338);
    }

    #[test]
    fn test_input_photo_serialize() {
        let user_id = UserId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::user_profile(user_id, access_hash, 789);

        let mut buf = BytesMut::new();
        let result = location.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert!(buf.len() >= 28); // constructor (4) + user_id (8) + access_hash (8) + photo_id (8)
    }

    #[test]
    fn test_input_photo_clone() {
        let user_id = UserId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::user_profile(user_id, access_hash, 789);

        let cloned = location.clone();
        assert_eq!(location, cloned);
    }

    #[test]
    fn test_input_photo_partial_eq() {
        let user_id = UserId::new(123).unwrap();
        let access_hash = AccessHash::new(456);

        let loc1 = InputPhotoFileLocation::user_profile(user_id, access_hash, 789);
        let loc2 = InputPhotoFileLocation::user_profile(user_id, access_hash, 789);
        assert_eq!(loc1, loc2);

        let loc3 = InputPhotoFileLocation::user_profile(user_id, access_hash, 999);
        assert_ne!(loc1, loc3);
    }

    // =========================================================================
    // InputFileLocation tests
    // =========================================================================

    #[test]
    fn test_input_file_empty() {
        let location = InputFileLocation::empty();
        assert_eq!(location, InputFileLocation::Empty);
    }

    #[test]
    fn test_input_file_local() {
        let location = InputFileLocation::local(1, 2, 3);

        assert!(matches!(
            location,
            InputFileLocation::Local {
                volume_id: 1,
                local_id: 2,
                secret: 3
            }
        ));
    }

    #[test]
    fn test_input_file_encrypted() {
        let location = InputFileLocation::encrypted(123, 456);

        assert!(matches!(
            location,
            InputFileLocation::Encrypted {
                id: 123,
                access_hash: 456
            }
        ));
    }

    #[test]
    fn test_input_file_document() {
        let location = InputFileLocation::document(123, 456, vec![1, 2, 3]);

        assert!(matches!(
            location,
            InputFileLocation::Document {
                id: 123,
                access_hash: 456,
                ..
            }
        ));
    }

    #[test]
    fn test_input_file_serialize_empty() {
        let location = InputFileLocation::Empty;

        let mut buf = BytesMut::new();
        let result = location.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert_eq!(buf.len(), 4); // constructor ID only
    }

    #[test]
    fn test_input_file_serialize_local() {
        let location = InputFileLocation::local(1, 2, 3);

        let mut buf = BytesMut::new();
        let result = location.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert_eq!(buf.len(), 24); // constructor (4) + volume_id (8) + local_id (4) + secret (8)
    }

    #[test]
    fn test_input_file_clone() {
        let location = InputFileLocation::local(1, 2, 3);
        let cloned = location.clone();

        assert_eq!(location, cloned);
    }

    // =========================================================================
    // GetFileRequest tests
    // =========================================================================

    #[test]
    fn test_get_file_request_new() {
        let location = InputFileLocation::Empty;
        let request = GetFileRequest::new(location);

        assert_eq!(request.location, InputFileLocation::Empty);
        assert_eq!(request.size_limit, Some(MAX_PHOTO_SIZE_BYTES as i32));
    }

    #[test]
    fn test_get_file_request_with_size_limit() {
        let location = InputFileLocation::Empty;
        let request = GetFileRequest::with_size_limit(location, 1024);

        assert_eq!(request.location, InputFileLocation::Empty);
        assert_eq!(request.size_limit, Some(1024));
    }

    #[test]
    fn test_get_file_request_without_size_limit() {
        let location = InputFileLocation::Empty;
        let request = GetFileRequest::without_size_limit(location);

        assert_eq!(request.location, InputFileLocation::Empty);
        assert_eq!(request.size_limit, None);
    }

    #[test]
    fn test_get_file_request_constructor_id() {
        assert_eq!(GetFileRequest::CONSTRUCTOR_ID, UPLOAD_GET_FILE);

        let location = InputFileLocation::Empty;
        let request = GetFileRequest::new(location);
        assert_eq!(request.constructor_id(), UPLOAD_GET_FILE);
    }

    #[test]
    fn test_get_file_request_serialize() {
        let location = InputFileLocation::local(1, 2, 3);
        let request = GetFileRequest::new(location);

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert!(buf.len() >= 28); // constructor (4) + location + limit
    }

    #[test]
    fn test_get_file_request_clone() {
        let location = InputFileLocation::Empty;
        let request1 = GetFileRequest::new(location.clone());
        let request2 = request1.clone();

        assert_eq!(request1, request2);
    }

    #[test]
    fn test_get_file_request_equality() {
        let location = InputFileLocation::Empty;
        let request1 = GetFileRequest::new(location.clone());
        let request2 = GetFileRequest::new(location);

        assert_eq!(request1, request2);
    }

    // =========================================================================
    // GetFileResponse tests
    // =========================================================================

    #[test]
    fn test_get_file_response_new() {
        let response = GetFileResponse::new("image/jpeg".to_string(), vec![1, 2, 3], 3);

        assert_eq!(response.mime_type, "image/jpeg");
        assert_eq!(response.bytes, vec![1, 2, 3]);
        assert_eq!(response.size, 3);
    }

    #[test]
    fn test_get_file_response_empty() {
        let response = GetFileResponse::new(String::new(), vec![], 0);

        assert!(response.is_empty());
        assert_eq!(response.len(), 0);
    }

    #[test]
    fn test_get_file_response_len() {
        let response = GetFileResponse::new("image/jpeg".to_string(), vec![1, 2, 3, 4], 4);

        assert_eq!(response.len(), 4);
    }

    #[test]
    fn test_get_file_response_clone() {
        let response1 = GetFileResponse::new("image/jpeg".to_string(), vec![1, 2, 3], 3);
        let response2 = response1.clone();

        assert_eq!(response1, response2);
    }

    #[test]
    fn test_get_file_response_equality() {
        let response1 = GetFileResponse::new("image/jpeg".to_string(), vec![1, 2, 3], 3);
        let response2 = GetFileResponse::new("image/jpeg".to_string(), vec![1, 2, 3], 3);

        assert_eq!(response1, response2);
    }

    // =========================================================================
    // PhotoCacheKey tests
    // =========================================================================

    #[test]
    fn test_cache_key_new() {
        let key = PhotoCacheKey::new(1, 123, 456);

        assert_eq!(key.type_id, 1);
        assert_eq!(key.entity_id, 123);
        assert_eq!(key.photo_id, 456);
    }

    #[test]
    fn test_cache_key_from_user_profile() {
        let user_id = UserId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::user_profile(user_id, access_hash, 789);

        let key = PhotoCacheKey::from_location(&location);

        assert_eq!(key.type_id, 1);
        assert_eq!(key.entity_id, 123);
        assert_eq!(key.photo_id, 789);
    }

    #[test]
    fn test_cache_key_user_profile() {
        let user_id = UserId::new(123).unwrap();
        let key = PhotoCacheKey::user_profile(user_id, 789);

        assert_eq!(key.type_id, 1);
        assert_eq!(key.entity_id, 123);
        assert_eq!(key.photo_id, 789);
    }

    #[test]
    fn test_cache_key_from_chat_photo() {
        let chat_id = ChatId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::chat_photo(chat_id, access_hash, 789);

        let key = PhotoCacheKey::from_location(&location);

        assert_eq!(key.type_id, 2);
        assert_eq!(key.entity_id, 123);
        assert_eq!(key.photo_id, 789);
    }

    #[test]
    fn test_cache_key_chat_photo() {
        let chat_id = ChatId::new(123).unwrap();
        let key = PhotoCacheKey::chat_photo(chat_id, 789);

        assert_eq!(key.type_id, 2);
        assert_eq!(key.entity_id, 123);
        assert_eq!(key.photo_id, 789);
    }

    #[test]
    fn test_cache_key_from_channel_photo() {
        let channel_id = ChannelId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::channel_photo(channel_id, access_hash, 789);

        let key = PhotoCacheKey::from_location(&location);

        assert_eq!(key.type_id, 3);
        assert_eq!(key.entity_id, 123);
        assert_eq!(key.photo_id, 789);
    }

    #[test]
    fn test_cache_key_channel_photo() {
        let channel_id = ChannelId::new(123).unwrap();
        let key = PhotoCacheKey::channel_photo(channel_id, 789);

        assert_eq!(key.type_id, 3);
        assert_eq!(key.entity_id, 123);
        assert_eq!(key.photo_id, 789);
    }

    #[test]
    fn test_cache_key_clone() {
        let key1 = PhotoCacheKey::new(1, 123, 456);
        let key2 = key1.clone();

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_cache_key_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let key1 = PhotoCacheKey::new(1, 123, 456);
        let key2 = PhotoCacheKey::new(1, 123, 456);

        // Same keys should have the same hash
        let mut hasher1 = DefaultHasher::new();
        key1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        key2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    // =========================================================================
    // PhotoDownloader tests
    // =========================================================================

    #[test]
    fn test_photo_downloader_new() {
        let downloader = PhotoDownloader::new();

        assert_eq!(downloader.max_cache_size(), DEFAULT_CACHE_SIZE_BYTES);
        assert_eq!(downloader.cache_size(), 0);
    }

    #[test]
    fn test_photo_downloader_default() {
        let downloader = PhotoDownloader::default();

        assert_eq!(downloader.max_cache_size(), DEFAULT_CACHE_SIZE_BYTES);
        assert_eq!(downloader.cache_size(), 0);
    }

    #[test]
    fn test_photo_downloader_with_cache_size() {
        let downloader = PhotoDownloader::with_cache_size(50 * 1024 * 1024);

        assert_eq!(downloader.max_cache_size(), 50 * 1024 * 1024);
        assert_eq!(downloader.cache_size(), 0);
    }

    #[test]
    fn test_photo_downloader_with_capacity() {
        let downloader = PhotoDownloader::with_capacity(500, 10 * 1024 * 1024);

        assert_eq!(downloader.max_cache_size(), 10 * 1024 * 1024);
        assert_eq!(downloader.cache_size(), 0);
    }

    #[test]
    fn test_photo_downloader_cache_utilization() {
        let downloader = PhotoDownloader::new();

        assert_eq!(downloader.cache_utilization(), 0.0);
    }

    #[test]
    fn test_photo_downloader_add_to_cache() {
        let downloader = PhotoDownloader::new();
        let key = PhotoCacheKey::new(1, 123, 456);
        let data = PhotoData::jpeg(vec![1, 2, 3]);

        let result = downloader.add_to_cache(key, data.clone());

        assert!(result.is_ok());
        assert_eq!(downloader.cache_size(), 3);
    }

    #[test]
    fn test_photo_downloader_add_to_cache_eviction() {
        let downloader = PhotoDownloader::with_cache_size(10);
        let key1 = PhotoCacheKey::new(1, 1, 1);
        let key2 = PhotoCacheKey::new(1, 2, 2);
        let data1 = PhotoData::jpeg(vec![1, 2, 3, 4, 5]);
        let data2 = PhotoData::jpeg(vec![6, 7, 8, 9, 10, 11]);

        // Add first photo
        assert!(downloader.add_to_cache(key1, data1).is_ok());
        assert_eq!(downloader.cache_size(), 5);

        // Add second photo - should evict first
        assert!(downloader.add_to_cache(key2, data2).is_ok());
        assert_eq!(downloader.cache_size(), 6);
    }

    #[test]
    fn test_photo_downloader_clear_cache() {
        let downloader = PhotoDownloader::new();
        let key = PhotoCacheKey::new(1, 123, 456);
        let data = PhotoData::jpeg(vec![1, 2, 3]);

        downloader.add_to_cache(key, data).unwrap();
        assert_eq!(downloader.cache_size(), 3);

        downloader.clear_cache();
        assert_eq!(downloader.cache_size(), 0);
    }

    #[test]
    fn test_photo_downloader_cache_len() {
        let downloader = PhotoDownloader::new();
        assert_eq!(downloader.cache_len(), 0);

        let key = PhotoCacheKey::new(1, 123, 456);
        let data = PhotoData::jpeg(vec![1, 2, 3]);

        downloader.add_to_cache(key, data).unwrap();
        assert_eq!(downloader.cache_len(), 1);
    }

    #[test]
    fn test_photo_downloader_cache_is_empty() {
        let downloader = PhotoDownloader::new();
        assert!(downloader.cache_is_empty());

        let key = PhotoCacheKey::new(1, 123, 456);
        let data = PhotoData::jpeg(vec![1, 2, 3]);

        downloader.add_to_cache(key, data).unwrap();
        assert!(!downloader.cache_is_empty());
    }

    #[test]
    fn test_photo_downloader_clone() {
        let downloader1 = PhotoDownloader::new();
        let downloader2 = downloader1.clone();

        // Both should share the same cache
        let key = PhotoCacheKey::new(1, 123, 456);
        let data = PhotoData::jpeg(vec![1, 2, 3]);

        downloader1.add_to_cache(key, data).unwrap();
        assert_eq!(downloader2.cache_size(), 3);
    }

    // =========================================================================
    // Integration tests
    // =========================================================================

    #[test]
    fn test_full_cache_workflow() {
        let downloader = PhotoDownloader::with_cache_size(100);

        let mut total_size: u64 = 0;

        // Add several photos
        for i in 1..=5 {
            let key = PhotoCacheKey::new(1, i as u64, (i * 100) as i64);
            let size = 20;
            let data = PhotoData::jpeg(vec![0; size]);

            downloader.add_to_cache(key, data).unwrap();
            total_size += size as u64;
        }

        assert_eq!(downloader.cache_size(), total_size);

        // Check utilization
        let utilization = downloader.cache_utilization();
        assert!(utilization > 0.0 && utilization <= 100.0);

        // Clear and verify
        downloader.clear_cache();
        assert_eq!(downloader.cache_size(), 0);
        assert_eq!(downloader.cache_utilization(), 0.0);
    }

    #[test]
    fn test_photo_location_variants() {
        let user_id = UserId::new(123).unwrap();
        let chat_id = ChatId::new(123).unwrap();
        let channel_id = ChannelId::new(123).unwrap();
        let access_hash = AccessHash::new(456);

        let user_location = InputPhotoFileLocation::user_profile(user_id, access_hash, 789);

        let chat_location = InputPhotoFileLocation::chat_photo(chat_id, access_hash, 789);

        let channel_location = InputPhotoFileLocation::channel_photo(channel_id, access_hash, 789);

        // All have the same constructor ID
        assert_eq!(user_location.constructor_id(), 0x3d8d4338);
        assert_eq!(chat_location.constructor_id(), 0x3d8d4338);
        assert_eq!(channel_location.constructor_id(), 0x3d8d4338);

        // But different cache keys
        let user_key = PhotoCacheKey::from_location(&user_location);
        let chat_key = PhotoCacheKey::from_location(&chat_location);
        let channel_key = PhotoCacheKey::from_location(&channel_location);

        assert_eq!(user_key.type_id, 1);
        assert_eq!(chat_key.type_id, 2);
        assert_eq!(channel_key.type_id, 3);
    }

    #[test]
    fn test_large_photo_error() {
        let size = 10 * 1024 * 1024; // 10 MB
        let max = 5 * 1024 * 1024; // 5 MB

        let err = PhotoDownloadError::PhotoTooLarge { size, max };

        assert!(err.to_string().contains("too large"));
        assert!(err.to_string().contains(&size.to_string()));
        assert!(err.to_string().contains(&max.to_string()));
    }

    #[test]
    fn test_cache_full_error() {
        let size = 200 * 1024 * 1024; // 200 MB
        let max = 100 * 1024 * 1024; // 100 MB

        let err = PhotoDownloadError::CacheFull { size, max };

        assert!(err.to_string().contains("cache full"));
        assert!(err.to_string().contains(&size.to_string()));
        assert!(err.to_string().contains(&max.to_string()));
    }

    #[test]
    fn test_photo_data_mime_types() {
        let data = vec![1, 2, 3];

        let jpeg = PhotoData::jpeg(data.clone());
        assert_eq!(jpeg.mime_type, "image/jpeg");

        let png = PhotoData::png(data.clone());
        assert_eq!(png.mime_type, "image/png");

        let gif = PhotoData::gif(data.clone());
        assert_eq!(gif.mime_type, "image/gif");

        let webp = PhotoData::webp(data);
        assert_eq!(webp.mime_type, "image/webp");
    }

    #[test]
    fn test_get_file_request_constants() {
        assert_eq!(GetFileRequest::CONSTRUCTOR_ID, UPLOAD_GET_FILE);
    }
}
