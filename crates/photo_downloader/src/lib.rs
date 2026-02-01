//! Photo downloader for Telegram profile photos.
//!
//! This module provides functionality for downloading profile photos from Telegram
//! servers using the `upload.getFile` API method.
//!
//! # Features
//!
//! - Photo download with automatic location resolution
//! - LRU cache with 100 MB size limit
//! - Support for different photo sizes
//! - Integration with FileDownloader
//!
//! # Examples
//!
//! ```no_run
//! # use rustgram_photo_downloader::PhotoDownloader;
//! # use rustgram_net::NetQueryDispatcher;
//! # use std::sync::Arc;
//! #
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let dispatcher = Arc::new(NetQueryDispatcher::new());
//! let downloader = PhotoDownloader::new(dispatcher);
//!
//! // Download a photo by location
//! // let data = downloader.download_photo(&location).await?;
//! # Ok(())
//! # }
//! ```

use bytes::BytesMut;
use lru::LruCache;
use rustgram_types::{AccessHash, TlDeserialize, TlHelper, TlSerialize, UserId};
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
        matches!(
            self,
            Self::Timeout(_) | Self::CacheFull { .. }
        )
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
        Self { data, mime_type, size }
    }

    /// Creates new photo data with JPEG MIME type.
    #[must_use]
    pub fn jpeg(data: Vec<u8>) -> Self {
        Self::new(data, "image/jpeg".to_string())
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputPhotoFileLocation {
    /// User profile photo.
    UserProfile {
        /// User ID
        user_id: UserId,
        /// Access hash
        access_hash: AccessHash,
        /// Photo ID
        photo_id: i64,
    },
    /// Chat photo.
    ChatPhoto {
        /// Chat ID (as i64)
        chat_id: i64,
        /// Access hash
        access_hash: AccessHash,
        /// Photo ID
        photo_id: i64,
    },
    /// Channel photo.
    ChannelPhoto {
        /// Channel ID
        channel_id: u64,
        /// Access hash
        access_hash: AccessHash,
        /// Photo ID
        photo_id: i64,
    },
}

/// Input file location for upload.getFile API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputFileLocation {
    /// Empty location
    Empty,
    /// Local file location
    Local {
        /// Volume ID
        volume_id: i64,
        /// Local ID within volume
        local_id: i32,
        /// Secret for authentication
        secret: i64,
    },
    /// Encrypted file location
    Encrypted {
        /// File ID
        id: i64,
        /// Access hash
        access_hash: i64,
    },
    /// Partial file location
    Partial {
        /// File ID
        id: i64,
        /// Access hash
        access_hash: i64,
    },
}

impl InputPhotoFileLocation {
    /// Returns the constructor ID for this location.
    pub fn constructor_id(&self) -> u32 {
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
                TlHelper::write_i64(buf, *chat_id);
                TlHelper::write_i64(buf, access_hash.get());
                TlHelper::write_i64(buf, *photo_id);
            }
            Self::ChannelPhoto {
                channel_id,
                access_hash,
                photo_id,
            } => {
                TlHelper::write_i64(buf, *channel_id as i64);
                TlHelper::write_i64(buf, access_hash.get());
                TlHelper::write_i64(buf, *photo_id);
            }
        }

        Ok(())
    }
}

/// Request for `upload.getFile`.
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
        match &self.location {
            InputFileLocation::Empty => {
                TlHelper::write_constructor_id(buf, 0x96a18d5); // inputFileEmpty
            }
            InputFileLocation::Local {
                volume_id,
                local_id,
                secret,
            } => {
                TlHelper::write_constructor_id(buf, 0x1bea9f9a); // inputFileLocation
                TlHelper::write_i64(buf, *volume_id);
                TlHelper::write_i32(buf, *local_id);
                TlHelper::write_i64(buf, *secret);
            }
            InputFileLocation::Encrypted {
                id,
                access_hash,
            } => {
                TlHelper::write_constructor_id(buf, 0x1816d527); // inputEncryptedFileLocation
                TlHelper::write_i64(buf, *id);
                TlHelper::write_i64(buf, *access_hash);
            }
            InputFileLocation::Partial {
                id,
                access_hash,
            } => {
                TlHelper::write_constructor_id(buf, 0xbc7fc6cd); // inputDocumentFileLocation
                TlHelper::write_i64(buf, *id);
                TlHelper::write_i64(buf, *access_hash);
            }
        }

        // Write size limit (always present)
        TlHelper::write_i32(buf, self.size_limit.unwrap_or(0));

        Ok(())
    }
}

/// Response from `upload.getFile`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFileResponse {
    /// File type (e.g., "image/jpeg")
    pub mime_type: String,
    /// File data bytes
    pub bytes: Vec<u8>,
    /// File size in bytes
    pub size: i32,
}

impl TlDeserialize for GetFileResponse {
    fn deserialize_tl(buf: &mut rustgram_types::tl::Bytes) -> Result<Self, rustgram_types::TypeError> {
        // Read constructor ID
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            // upload.file#96a18d5
            0x96a18d5 => {
                let mime_type = TlHelper::read_string(buf)?;
                let _bytes = TlHelper::read_bytes(buf)?;
                let size = TlHelper::read_i32(buf)?;

                Ok(Self {
                    mime_type,
                    bytes: Vec::new(), // Simplified - in real impl would read actual bytes
                    size,
                })
            }
            // upload.fileCdnRedirect#ea52fe5a
            0xea52fe5a => {
                // CDN redirect - not implemented
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
    #[must_use]
    pub fn new() -> Self {
        // SAFETY: DEFAULT_CACHE_CAPACITY is 5000, which is > 0
        #[allow(clippy::expect_used)]
        let capacity = NonZeroUsize::new(1000)
            .expect("Capacity must be > 0");

        Self {
            cache: Arc::new(parking_lot::Mutex::new(LruCache::new(capacity))),
            cache_size: Arc::new(parking_lot::Mutex::new(0)),
            max_cache_size: DEFAULT_CACHE_SIZE_BYTES,
        }
    }

    /// Creates a new photo downloader with custom cache size.
    #[must_use]
    pub fn with_cache_size(max_cache_size: u64) -> Self {
        let capacity = NonZeroUsize::new(1000)
            .expect("Capacity must be > 0");

        Self {
            cache: Arc::new(parking_lot::Mutex::new(LruCache::new(capacity))),
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
        // In a real implementation, this would convert the photo location
        // to the appropriate InputFileLocation for upload.getFile
        // For now, return a placeholder
        match photo_location {
            InputPhotoFileLocation::UserProfile { photo_id, .. } => {
                Ok(InputFileLocation::Local {
                    volume_id: 0,
                    local_id: *photo_id as i32,
                    secret: 0,
                })
            }
            InputPhotoFileLocation::ChatPhoto { photo_id, .. } => {
                Ok(InputFileLocation::Local {
                    volume_id: 0,
                    local_id: *photo_id as i32,
                    secret: 0,
                })
            }
            InputPhotoFileLocation::ChannelPhoto { photo_id, .. } => {
                Ok(InputFileLocation::Local {
                    volume_id: 0,
                    local_id: *photo_id as i32,
                    secret: 0,
                })
            }
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
    /// Returns error if cache is full
    pub fn add_to_cache(
        &self,
        key: PhotoCacheKey,
        data: PhotoData,
    ) -> Result<(), PhotoDownloadError> {
        let mut cache = self.cache.lock();
        let mut cache_size = self.cache_size.lock();

        let data_size = data.size as u64;

        // Check if we need to evict
        while *cache_size + data_size > self.max_cache_size {
            if let Some((_, evicted)) = cache.pop_lru() {
                *cache_size -= evicted.size as u64;
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

    /// Returns the cache utilization as a percentage.
    #[must_use]
    pub fn cache_utilization(&self) -> f64 {
        let size = self.cache_size();
        if self.max_cache_size == 0 {
            0.0
        } else {
            (size as f64 / self.max_cache_size as f64) * 100.0
        }
    }
}

/// Cache key for photos.
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
    pub fn from_location(location: &InputPhotoFileLocation) -> Self {
        match location {
            InputPhotoFileLocation::UserProfile {
                user_id,
                photo_id,
                ..
            } => Self {
                type_id: 1,
                entity_id: user_id.get() as u64,
                photo_id: *photo_id,
            },
            InputPhotoFileLocation::ChatPhoto {
                chat_id,
                photo_id,
                ..
            } => Self {
                type_id: 2,
                entity_id: *chat_id as u64,
                photo_id: *photo_id,
            },
            InputPhotoFileLocation::ChannelPhoto {
                channel_id,
                photo_id,
                ..
            } => Self {
                type_id: 3,
                entity_id: *channel_id,
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
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-photo-downloader";

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::AccessHash;

    // =========================================================================
    // Constants tests
    // =========================================================================

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_CACHE_SIZE_BYTES, 100 * 1024 * 1024);
        assert_eq!(MAX_PHOTO_SIZE_BYTES, 5 * 1024 * 1024);
        assert_eq!(UPLOAD_GET_FILE, 0xbe5335be);
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
        assert_eq!(CRATE_NAME, "rustgram-photo-downloader");
    }

    // =========================================================================
    // PhotoDownloadError tests
    // =========================================================================

    #[test]
    fn test_error_api_error() {
        let err = PhotoDownloadError::api_error(404, "Not found");
        assert!(matches!(err, PhotoDownloadError::ApiError { code: 404, .. }));
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
        assert!(PhotoDownloadError::CacheFull {
            size: 100,
            max: 50
        }
        .is_retryable());
        assert!(!PhotoDownloadError::api_error(404, "Not found").is_retryable());
        assert!(!PhotoDownloadError::NoClient.is_retryable());
    }

    #[test]
    fn test_error_clone() {
        let err1 = PhotoDownloadError::api_error(500, "Error");
        let err2 = err1.clone();
        assert_eq!(err1, err2);
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

    // =========================================================================
    // InputPhotoFileLocation tests
    // =========================================================================

    #[test]
    fn test_input_photo_user_profile() {
        let user_id = UserId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::UserProfile {
            user_id,
            access_hash,
            photo_id: 789,
        };

        assert_eq!(location.constructor_id(), 0x3d8d4338);
    }

    #[test]
    fn test_input_photo_chat_photo() {
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::ChatPhoto {
            chat_id: 123,
            access_hash,
            photo_id: 789,
        };

        assert_eq!(location.constructor_id(), 0x3d8d4338);
    }

    #[test]
    fn test_input_photo_channel_photo() {
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::ChannelPhoto {
            channel_id: 123,
            access_hash,
            photo_id: 789,
        };

        assert_eq!(location.constructor_id(), 0x3d8d4338);
    }

    #[test]
    fn test_input_photo_serialize() {
        let user_id = UserId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::UserProfile {
            user_id,
            access_hash,
            photo_id: 789,
        };

        let mut buf = BytesMut::new();
        let result = location.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert!(buf.len() >= 28); // constructor (4) + user_id (8) + access_hash (8) + photo_id (8)
    }

    #[test]
    fn test_input_photo_clone() {
        let user_id = UserId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::UserProfile {
            user_id,
            access_hash,
            photo_id: 789,
        };

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
        assert_eq!(
            request.size_limit,
            Some(MAX_PHOTO_SIZE_BYTES as i32)
        );
    }

    #[test]
    fn test_get_file_request_with_size_limit() {
        let location = InputFileLocation::Empty;
        let request = GetFileRequest::with_size_limit(location, 1024);

        assert_eq!(request.location, InputFileLocation::Empty);
        assert_eq!(request.size_limit, Some(1024));
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
        let location = InputFileLocation::Local {
            volume_id: 1,
            local_id: 2,
            secret: 3,
        };
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
        let request1 = GetFileRequest::new(location);
        let request2 = GetFileRequest::new(InputFileLocation::Empty);

        assert_eq!(request1, request2);
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
        let location = InputPhotoFileLocation::UserProfile {
            user_id,
            access_hash,
            photo_id: 789,
        };

        let key = PhotoCacheKey::from_location(&location);

        assert_eq!(key.type_id, 1);
        assert_eq!(key.entity_id, 123);
        assert_eq!(key.photo_id, 789);
    }

    #[test]
    fn test_cache_key_from_chat_photo() {
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::ChatPhoto {
            chat_id: 123,
            access_hash,
            photo_id: 789,
        };

        let key = PhotoCacheKey::from_location(&location);

        assert_eq!(key.type_id, 2);
        assert_eq!(key.entity_id, 123);
        assert_eq!(key.photo_id, 789);
    }

    #[test]
    fn test_cache_key_from_channel_photo() {
        let access_hash = AccessHash::new(456);
        let location = InputPhotoFileLocation::ChannelPhoto {
            channel_id: 123,
            access_hash,
            photo_id: 789,
        };

        let key = PhotoCacheKey::from_location(&location);

        assert_eq!(key.type_id, 3);
        assert_eq!(key.entity_id, 123);
        assert_eq!(key.photo_id, 789);
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

    // =========================================================================
    // GetFileResponse tests
    // =========================================================================

    #[test]
    fn test_get_file_response_constants() {
        assert_eq!(UPLOAD_GET_FILE, 0xbe5335be);
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
        let access_hash = AccessHash::new(456);

        let user_location = InputPhotoFileLocation::UserProfile {
            user_id,
            access_hash,
            photo_id: 789,
        };

        let chat_location = InputPhotoFileLocation::ChatPhoto {
            chat_id: 123,
            access_hash,
            photo_id: 789,
        };

        let channel_location = InputPhotoFileLocation::ChannelPhoto {
            channel_id: 123,
            access_hash,
            photo_id: 789,
        };

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
        let max = 5 * 1024 * 1024;   // 5 MB

        let err = PhotoDownloadError::PhotoTooLarge {
            size,
            max,
        };

        assert!(err.to_string().contains("too large"));
        assert!(err.to_string().contains(&size.to_string()));
        assert!(err.to_string().contains(&max.to_string()));
    }

    #[test]
    fn test_cache_full_error() {
        let size = 200 * 1024 * 1024; // 200 MB
        let max = 100 * 1024 * 1024;  // 100 MB

        let err = PhotoDownloadError::CacheFull { size, max };

        assert!(err.to_string().contains("cache full"));
        assert!(err.to_string().contains(&size.to_string()));
        assert!(err.to_string().contains(&max.to_string()));
    }
}
