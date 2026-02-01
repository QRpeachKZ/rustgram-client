// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Photo types for Telegram.
//!
//! This module provides TL deserialization for photo-related types.
//!
//! # TL Schema
//!
//! ```text
//! photoEmpty#2331b22d id:long = Photo;
//! photo#fb197a65 flags:# has_stickers:flags.0?true id:long access_hash:long
//!     file_reference:bytes date:int sizes:Vector<PhotoSize>
//!     video_sizes:flags.1?Vector<VideoSize> dc_id:int = Photo;
//!
//! photoSizeEmpty#e17e23c type:string = PhotoSize;
//! photoSize#75c78e60 type:string w:int h:int size:int = PhotoSize;
//! photoCachedSize#21e1ad6 type:string w:int h:int bytes:bytes = PhotoSize;
//! photoStrippedSize#e0b0bc2e type:string bytes:bytes = PhotoSize;
//! photoSizeProgressive#fa3efb95 type:string w:int h:int sizes:Vector<int> = PhotoSize;
//! photoPathSize#d8214d41 type:string bytes:bytes = PhotoSize;
//! ```

use crate::error::{TlError, VectorError};
use crate::flags::FlagReader;
use rustgram_types::tl::{Bytes, TlDeserialize, TlHelper};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Maximum number of photo sizes allowed in a vector.
const MAX_PHOTO_SIZES: usize = 100;

/// Maximum number of progressive sizes allowed.
const MAX_PROGRESSIVE_SIZES: usize = 50;

/// Photo type.
///
/// Represents a user or chat photo in Telegram.
///
/// # TL Schema
///
/// ```text
/// photoEmpty#2331b22d id:long = Photo;
/// photo#fb197a65 flags:# has_stickers:flags.0?true id:long access_hash:long
///     file_reference:bytes date:int sizes:Vector<PhotoSize>
///     video_sizes:flags.1?Vector<VideoSize> dc_id:int = Photo;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_tl_core::Photo;
///
/// let empty_photo = Photo::Empty { id: 12345 };
/// assert!(empty_photo.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Photo {
    /// Empty photo (placeholder).
    Empty {
        /// Photo ID.
        id: i64,
    },

    /// Full photo with data.
    Photo {
        /// Whether the photo has stickers.
        has_stickers: bool,
        /// Photo ID.
        id: i64,
        /// Access hash for authentication.
        access_hash: i64,
        /// File reference for accessing the photo.
        file_reference: Vec<u8>,
        /// Upload date.
        date: i32,
        /// Available photo sizes.
        sizes: Vec<PhotoSize>,
        /// Video sizes (if available).
        video_sizes: Option<Vec<VideoSize>>,
        /// Datacenter ID where the photo is stored.
        dc_id: i32,
    },
}

impl Photo {
    /// Constructor ID for photoEmpty.
    pub const EMPTY_CONSTRUCTOR: u32 = 0x2331b22d;

    /// Constructor ID for photo.
    pub const PHOTO_CONSTRUCTOR: u32 = 0xfb197a65;

    /// Checks if this is an empty photo.
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty { .. })
    }

    /// Gets the photo ID.
    pub fn id(&self) -> i64 {
        match self {
            Self::Empty { id } | Self::Photo { id, .. } => *id,
        }
    }

    /// Gets the access hash (if available).
    pub fn access_hash(&self) -> Option<i64> {
        match self {
            Self::Photo { access_hash, .. } => Some(*access_hash),
            _ => None,
        }
    }
}

impl TlDeserialize for Photo {
    fn deserialize_tl(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            Self::EMPTY_CONSTRUCTOR => {
                let id = TlHelper::read_i64(buf)?;
                Ok(Self::Empty { id })
            }
            Self::PHOTO_CONSTRUCTOR => {
                let flags = TlHelper::read_i32(buf)? as u32;
                let flag_reader = FlagReader::new(flags);

                let has_stickers = flag_reader.read_bool(0);
                let id = TlHelper::read_i64(buf)?;
                let access_hash = TlHelper::read_i64(buf)?;
                let file_reference = TlHelper::read_bytes(buf)?;
                let date = TlHelper::read_i32(buf)?;
                let sizes = deserialize_vector_photo_size(buf)?;
                let video_sizes = if flag_reader.has(1) {
                    Some(deserialize_vector_video_size(buf)?)
                } else {
                    None
                };
                let dc_id = TlHelper::read_i32(buf)?;

                Ok(Self::Photo {
                    has_stickers,
                    id,
                    access_hash,
                    file_reference,
                    date,
                    sizes,
                    video_sizes,
                    dc_id,
                })
            }
            _ => {
                let tl_err = TlError::unknown_constructor(
                    vec![Self::EMPTY_CONSTRUCTOR, Self::PHOTO_CONSTRUCTOR],
                    constructor_id,
                    "Photo",
                );
                Err(rustgram_types::TypeError::from(tl_err))
            }
        }
    }
}

impl fmt::Display for Photo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { id } => write!(f, "PhotoEmpty(id={})", id),
            Self::Photo { id, sizes, .. } => {
                write!(f, "Photo(id={}, {} sizes)", id, sizes.len())
            }
        }
    }
}

/// Photo size variant.
///
/// Represents different ways a photo can be stored and sized.
///
/// # TL Schema
///
/// ```text
/// photoSizeEmpty#e17e23c type:string = PhotoSize;
/// photoSize#75c78e60 type:string w:int h:int size:int = PhotoSize;
/// photoCachedSize#21e1ad6 type:string w:int h:int bytes:bytes = PhotoSize;
/// photoStrippedSize#e0b0bc2e type:string bytes:bytes = PhotoSize;
/// photoSizeProgressive#fa3efb95 type:string w:int h:int sizes:Vector<int> = PhotoSize;
/// photoPathSize#d8214d41 type:string bytes:bytes = PhotoSize;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhotoSize {
    /// Empty photo size (placeholder).
    Empty {
        /// Size type identifier (e.g., "s", "m", "x").
        type_: String,
    },

    /// Regular photo size with dimensions and file size.
    Size {
        /// Size type identifier.
        type_: String,
        /// Width in pixels.
        w: i32,
        /// Height in pixels.
        h: i32,
        /// File size in bytes.
        size: i32,
    },

    /// Cached photo size with embedded data.
    Cached {
        /// Size type identifier.
        type_: String,
        /// Width in pixels.
        w: i32,
        /// Height in pixels.
        h: i32,
        /// Cached image bytes.
        bytes: Vec<u8>,
    },

    /// Stripped thumbnail size (JPEG-compressed).
    Stripped {
        /// Size type identifier.
        type_: String,
        /// Stripped image bytes.
        bytes: Vec<u8>,
    },

    /// Progressive JPEG size with multiple quality levels.
    Progressive {
        /// Size type identifier.
        type_: String,
        /// Width in pixels.
        w: i32,
        /// Height in pixels.
        h: i32,
        /// Progressive sizes (file sizes for each quality level).
        sizes: Vec<i32>,
    },

    /// Path size for stickers.
    Path {
        /// Size type identifier.
        type_: String,
        /// Path image bytes.
        bytes: Vec<u8>,
    },
}

impl PhotoSize {
    /// Constructor ID for photoSizeEmpty.
    pub const EMPTY_CONSTRUCTOR: u32 = 0x0017e23c;

    /// Constructor ID for photoSize.
    pub const SIZE_CONSTRUCTOR: u32 = 0x75c78e60;

    /// Constructor ID for photoCachedSize.
    pub const CACHED_CONSTRUCTOR: u32 = 0x021e1ad6;

    /// Constructor ID for photoStrippedSize.
    pub const STRIPPED_CONSTRUCTOR: u32 = 0xe0b0bc2e;

    /// Constructor ID for photoSizeProgressive.
    pub const PROGRESSIVE_CONSTRUCTOR: u32 = 0xfa3efb95;

    /// Constructor ID for photoPathSize.
    pub const PATH_CONSTRUCTOR: u32 = 0xd8214d41;

    /// Gets the size type identifier.
    pub fn type_(&self) -> &str {
        match self {
            Self::Empty { type_, .. }
            | Self::Size { type_, .. }
            | Self::Cached { type_, .. }
            | Self::Stripped { type_, .. }
            | Self::Progressive { type_, .. }
            | Self::Path { type_, .. } => type_,
        }
    }
}

impl TlDeserialize for PhotoSize {
    fn deserialize_tl(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            Self::EMPTY_CONSTRUCTOR => {
                let type_ = TlHelper::read_string(buf)?;
                Ok(Self::Empty { type_ })
            }
            Self::SIZE_CONSTRUCTOR => {
                let type_ = TlHelper::read_string(buf)?;
                let w = TlHelper::read_i32(buf)?;
                let h = TlHelper::read_i32(buf)?;
                let size = TlHelper::read_i32(buf)?;
                Ok(Self::Size { type_, w, h, size })
            }
            Self::CACHED_CONSTRUCTOR => {
                let type_ = TlHelper::read_string(buf)?;
                let w = TlHelper::read_i32(buf)?;
                let h = TlHelper::read_i32(buf)?;
                let bytes = TlHelper::read_bytes(buf)?;
                Ok(Self::Cached { type_, w, h, bytes })
            }
            Self::STRIPPED_CONSTRUCTOR => {
                let type_ = TlHelper::read_string(buf)?;
                let bytes = TlHelper::read_bytes(buf)?;
                Ok(Self::Stripped { type_, bytes })
            }
            Self::PROGRESSIVE_CONSTRUCTOR => {
                let type_ = TlHelper::read_string(buf)?;
                let w = TlHelper::read_i32(buf)?;
                let h = TlHelper::read_i32(buf)?;
                let sizes = deserialize_vector_i32(buf)?;
                Ok(Self::Progressive { type_, w, h, sizes })
            }
            Self::PATH_CONSTRUCTOR => {
                let type_ = TlHelper::read_string(buf)?;
                let bytes = TlHelper::read_bytes(buf)?;
                Ok(Self::Path { type_, bytes })
            }
            _ => {
                let tl_err = TlError::unknown_constructor(
                    vec![
                        Self::EMPTY_CONSTRUCTOR,
                        Self::SIZE_CONSTRUCTOR,
                        Self::CACHED_CONSTRUCTOR,
                        Self::STRIPPED_CONSTRUCTOR,
                        Self::PROGRESSIVE_CONSTRUCTOR,
                        Self::PATH_CONSTRUCTOR,
                    ],
                    constructor_id,
                    "PhotoSize",
                );
                Err(rustgram_types::TypeError::from(tl_err))
            }
        }
    }
}

/// Video size (placeholder for full implementation).
///
/// Note: This is a simplified placeholder. The full implementation would
/// include all VideoSize variants from the TL schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoSize {
    /// Size type identifier.
    #[serde(skip)]
    pub type_: String,
    /// Video dimensions placeholder.
    #[serde(skip)]
    pub data: Vec<u8>,
}

/// Deserializes a vector of PhotoSize values.
fn deserialize_vector_photo_size(buf: &mut Bytes) -> rustgram_types::TypeResult<Vec<PhotoSize>> {
    // Read vector prefix (0x1cb5c415 for vector#1cb5c415)
    let prefix = TlHelper::read_constructor_id(buf)?;
    if prefix != 0x1cb5c415 {
        let tl_err = TlError::VectorError(VectorError::invalid_prefix(prefix));
        return Err(rustgram_types::TypeError::from(tl_err));
    }

    let count = TlHelper::read_i32(buf)? as usize;
    if count > MAX_PHOTO_SIZES {
        let tl_err = TlError::VectorError(VectorError::too_large(count, MAX_PHOTO_SIZES));
        return Err(rustgram_types::TypeError::from(tl_err));
    }

    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(PhotoSize::deserialize_tl(buf)?);
    }

    Ok(result)
}

/// Deserializes a vector of VideoSize values (placeholder).
fn deserialize_vector_video_size(buf: &mut Bytes) -> rustgram_types::TypeResult<Vec<VideoSize>> {
    // Read vector prefix
    let prefix = TlHelper::read_constructor_id(buf)?;
    if prefix != 0x1cb5c415 {
        let tl_err = TlError::VectorError(VectorError::invalid_prefix(prefix));
        return Err(rustgram_types::TypeError::from(tl_err));
    }

    let count = TlHelper::read_i32(buf)? as usize;
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        // Placeholder: skip unknown data
        let _type = TlHelper::read_string(buf)?;
        let _data = TlHelper::read_bytes(buf)?;
        result.push(VideoSize {
            type_: _type,
            data: _data,
        });
    }

    Ok(result)
}

/// Deserializes a vector of i32 values.
fn deserialize_vector_i32(buf: &mut Bytes) -> rustgram_types::TypeResult<Vec<i32>> {
    // Read vector prefix
    let prefix = TlHelper::read_constructor_id(buf)?;
    if prefix != 0x1cb5c415 {
        let tl_err = TlError::VectorError(VectorError::invalid_prefix(prefix));
        return Err(rustgram_types::TypeError::from(tl_err));
    }

    let count = TlHelper::read_i32(buf)? as usize;
    if count > MAX_PROGRESSIVE_SIZES {
        let tl_err = TlError::VectorError(VectorError::too_large(count, MAX_PROGRESSIVE_SIZES));
        return Err(rustgram_types::TypeError::from(tl_err));
    }

    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(TlHelper::read_i32(buf)?);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_buffer(data: &[u8]) -> Bytes {
        Bytes::new(bytes::Bytes::copy_from_slice(data))
    }

    #[test]
    fn test_photo_empty_deserialize() {
        // photoEmpty#2331b22d id:long = Photo;
        let mut data = vec![0x2d, 0xb2, 0x31, 0x23]; // constructor (little-endian)
        data.extend_from_slice(&126i64.to_le_bytes()); // id

        let mut buf = create_buffer(&data);
        let result = Photo::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result, Photo::Empty { id: 126 });
        assert!(result.is_empty());
    }

    #[test]
    fn test_photo_empty_constructor_id() {
        assert_eq!(Photo::EMPTY_CONSTRUCTOR, 0x2331b22d);
        assert_eq!(Photo::PHOTO_CONSTRUCTOR, 0xfb197a65);
    }

    #[test]
    fn test_photo_empty_id() {
        let photo = Photo::Empty { id: 12345 };
        assert_eq!(photo.id(), 12345);
        assert!(photo.access_hash().is_none());
    }

    #[test]
    fn test_photo_size_empty_deserialize() {
        // photoSizeEmpty#e17e23c type:string = PhotoSize;
        // Constructor in little-endian: 0x3c, 0xe2, 0x17, 0x00
        let mut data = vec![0x3c, 0xe2, 0x17, 0x00]; // constructor
                                                     // String "s": TL read format is [length, data, padding]
                                                     // For length=1: data='s', padding=(4 - ((1+1) % 4)) % 4 = 2
        data.extend_from_slice(&[1u8]); // length
        data.extend_from_slice(b"s"); // data
        data.extend_from_slice(&[0u8, 0]); // padding

        let mut buf = create_buffer(&data);
        let result = PhotoSize::deserialize_tl(&mut buf).unwrap();

        assert_eq!(
            result,
            PhotoSize::Empty {
                type_: "s".to_string()
            }
        );
        assert_eq!(result.type_(), "s");
    }

    #[test]
    fn test_photo_size_constructors() {
        assert_eq!(PhotoSize::EMPTY_CONSTRUCTOR, 0x0017e23c);
        assert_eq!(PhotoSize::SIZE_CONSTRUCTOR, 0x75c78e60);
        assert_eq!(PhotoSize::CACHED_CONSTRUCTOR, 0x021e1ad6);
        assert_eq!(PhotoSize::STRIPPED_CONSTRUCTOR, 0xe0b0bc2e);
        assert_eq!(PhotoSize::PROGRESSIVE_CONSTRUCTOR, 0xfa3efb95);
        assert_eq!(PhotoSize::PATH_CONSTRUCTOR, 0xd8214d41);
    }

    #[test]
    fn test_vector_i32_deserialize() {
        // Vector with 3 integers: [1, 2, 3]
        let mut data = vec![0x15, 0xc4, 0xb5, 0x1c]; // vector constructor
        data.extend_from_slice(&3i32.to_le_bytes()); // count
        data.extend_from_slice(&1i32.to_le_bytes());
        data.extend_from_slice(&2i32.to_le_bytes());
        data.extend_from_slice(&3i32.to_le_bytes());

        let mut buf = create_buffer(&data);
        let result = deserialize_vector_i32(&mut buf).unwrap();

        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_vector_too_large() {
        let mut data = vec![0x15, 0xc4, 0xb5, 0x1c]; // vector constructor
        data.extend_from_slice(&101i32.to_le_bytes()); // count exceeds MAX

        let mut buf = create_buffer(&data);
        let result = deserialize_vector_i32(&mut buf);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Too large") || err_msg.contains("exceeds"));
    }

    #[test]
    fn test_vector_invalid_prefix() {
        let mut data = vec![0xFF, 0xFF, 0xFF, 0xFF]; // invalid prefix
        data.extend_from_slice(&1i32.to_le_bytes());

        let mut buf = create_buffer(&data);
        let result = deserialize_vector_i32(&mut buf);

        assert!(result.is_err());
        // Just verify we get an error, the exact message format may vary
        assert!(result.is_err());
    }

    #[test]
    fn test_photo_display() {
        let empty = Photo::Empty { id: 123 };
        assert_eq!(format!("{}", empty), "PhotoEmpty(id=123)");

        let photo = Photo::Photo {
            has_stickers: false,
            id: 456,
            access_hash: 789,
            file_reference: vec![],
            date: 0,
            sizes: vec![],
            video_sizes: None,
            dc_id: 0,
        };
        assert_eq!(format!("{}", photo), "Photo(id=456, 0 sizes)");
    }

    #[test]
    fn test_photo_size_type_getter() {
        let size = PhotoSize::Size {
            type_: "m".to_string(),
            w: 100,
            h: 100,
            size: 5000,
        };
        assert_eq!(size.type_(), "m");

        let cached = PhotoSize::Cached {
            type_: "x".to_string(),
            w: 800,
            h: 600,
            bytes: vec![1, 2, 3],
        };
        assert_eq!(cached.type_(), "x");
    }

    #[test]
    fn test_photo_unknown_constructor() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0]; // invalid constructor
        let mut buf = create_buffer(&data);
        let result = Photo::deserialize_tl(&mut buf);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Unknown constructor") || err_msg.contains("0xffffffff"));
    }

    #[test]
    fn test_photo_size_unknown_constructor() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0]; // invalid constructor
        let mut buf = create_buffer(&data);
        let result = PhotoSize::deserialize_tl(&mut buf);

        assert!(result.is_err());
    }

    // Additional tests to increase coverage

    #[test]
    fn test_photo_access_hash() {
        let empty = Photo::Empty { id: 123 };
        assert!(empty.access_hash().is_none());

        let photo = Photo::Photo {
            has_stickers: false,
            id: 456,
            access_hash: 789,
            file_reference: vec![],
            date: 0,
            sizes: vec![],
            video_sizes: None,
            dc_id: 0,
        };
        assert_eq!(photo.access_hash(), Some(789));
    }

    #[test]
    fn test_photo_id() {
        let empty = Photo::Empty { id: 111 };
        assert_eq!(empty.id(), 111);

        let photo = Photo::Photo {
            has_stickers: false,
            id: 222,
            access_hash: 0,
            file_reference: vec![],
            date: 0,
            sizes: vec![],
            video_sizes: None,
            dc_id: 0,
        };
        assert_eq!(photo.id(), 222);
    }

    #[test]
    fn test_photo_size_type_getter_all_variants() {
        let empty = PhotoSize::Empty {
            type_: "empty".to_string()
        };
        assert_eq!(empty.type_(), "empty");

        let size = PhotoSize::Size {
            type_: "size".to_string(),
            w: 100,
            h: 100,
            size: 5000
        };
        assert_eq!(size.type_(), "size");

        let cached = PhotoSize::Cached {
            type_: "cached".to_string(),
            w: 200,
            h: 200,
            bytes: vec![]
        };
        assert_eq!(cached.type_(), "cached");

        let stripped = PhotoSize::Stripped {
            type_: "stripped".to_string(),
            bytes: vec![]
        };
        assert_eq!(stripped.type_(), "stripped");

        let progressive = PhotoSize::Progressive {
            type_: "progressive".to_string(),
            w: 300,
            h: 300,
            sizes: vec![]
        };
        assert_eq!(progressive.type_(), "progressive");

        let path = PhotoSize::Path {
            type_: "path".to_string(),
            bytes: vec![]
        };
        assert_eq!(path.type_(), "path");
    }

    #[test]
    fn test_photo_equality() {
        let photo1 = Photo::Empty { id: 123 };
        let photo2 = Photo::Empty { id: 123 };
        assert_eq!(photo1, photo2);

        let photo3 = Photo::Empty { id: 456 };
        assert_ne!(photo1, photo3);

        let photo4 = Photo::Photo {
            has_stickers: false,
            id: 123,
            access_hash: 0,
            file_reference: vec![],
            date: 0,
            sizes: vec![],
            video_sizes: None,
            dc_id: 0,
        };
        assert_ne!(photo1, photo4);
    }

    #[test]
    fn test_photo_size_equality() {
        let size1 = PhotoSize::Size {
            type_: "m".to_string(),
            w: 100,
            h: 100,
            size: 5000
        };
        let size2 = PhotoSize::Size {
            type_: "m".to_string(),
            w: 100,
            h: 100,
            size: 5000
        };
        assert_eq!(size1, size2);

        let size3 = PhotoSize::Size {
            type_: "x".to_string(),
            w: 100,
            h: 100,
            size: 5000
        };
        assert_ne!(size1, size3);
    }

    #[test]
    fn test_photo_clone() {
        let photo1 = Photo::Photo {
            has_stickers: true,
            id: 123,
            access_hash: 456,
            file_reference: vec![1, 2, 3],
            date: 789,
            sizes: vec![],
            video_sizes: Some(vec![]),
            dc_id: 2,
        };
        let photo2 = photo1.clone();
        assert_eq!(photo1, photo2);
    }

    #[test]
    fn test_photo_size_clone() {
        let size1 = PhotoSize::Cached {
            type_: "test".to_string(),
            w: 100,
            h: 100,
            bytes: vec![1, 2, 3]
        };
        let size2 = size1.clone();
        assert_eq!(size1, size2);
    }

    #[test]
    fn test_vector_i32_empty() {
        let mut data = vec![0x15, 0xc4, 0xb5, 0x1c]; // vector constructor
        data.extend_from_slice(&0i32.to_le_bytes()); // count = 0

        let mut buf = create_buffer(&data);
        let result = deserialize_vector_i32(&mut buf).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn test_vector_i32_single_element() {
        let mut data = vec![0x15, 0xc4, 0xb5, 0x1c]; // vector constructor
        data.extend_from_slice(&1i32.to_le_bytes()); // count
        data.extend_from_slice(&42i32.to_le_bytes());

        let mut buf = create_buffer(&data);
        let result = deserialize_vector_i32(&mut buf).unwrap();

        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_vector_i32_boundary_size() {
        let mut data = vec![0x15, 0xc4, 0xb5, 0x1c]; // vector constructor
        data.extend_from_slice(&50i32.to_le_bytes()); // count (at MAX_PROGRESSIVE_SIZES boundary)

        for i in 0i32..50 {
            data.extend_from_slice(&i.to_le_bytes());
        }

        let mut buf = create_buffer(&data);
        let result = deserialize_vector_i32(&mut buf).unwrap();

        assert_eq!(result.len(), 50);
    }

    #[test]
    fn test_video_size_placeholder() {
        // Test the VideoSize placeholder struct
        let video_size = VideoSize {
            type_: "test".to_string(),
            data: vec![1, 2, 3]
        };

        assert_eq!(video_size.type_, "test");
        assert_eq!(video_size.data, vec![1, 2, 3]);
    }
}
