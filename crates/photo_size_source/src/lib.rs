//! # Photo Size Source
//!
//! Source identifier for photo sizes.
//!
//! ## TDLib Reference
//!
//! - TDLib header: `td/telegram/PhotoSizeSource.h`
//! - TDLib struct: `PhotoSizeSource`
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_photo_size_source::PhotoSizeSource;
//! use rustgram_file_type::FileType;
//! use rustgram_photo_size_type::PhotoSizeType;
//!
//! let source = PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s'));
//! ```

use core::fmt;
use rustgram_dialog_id::DialogId;
use rustgram_file_type::FileType;
use rustgram_photo_size_type::PhotoSizeType;

/// Source identifier for photo sizes.
///
/// This enum represents different sources for photo sizes in Telegram.
///
/// TDLib: `struct PhotoSizeSource`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhotoSizeSource {
    /// Legacy photos with secret
    Legacy {
        /// The secret for legacy photos
        secret: i64,
    },
    /// Thumbnail for photos, documents, encrypted thumbnails
    Thumbnail {
        /// File type of the thumbnail
        file_type: FileType,
        /// Thumbnail type character
        thumbnail_type: PhotoSizeType,
    },
    /// Dialog photo (small)
    DialogPhotoSmall {
        /// Dialog ID
        dialog_id: DialogId,
        /// Dialog access hash
        dialog_access_hash: i64,
    },
    /// Dialog photo (big)
    DialogPhotoBig {
        /// Dialog ID
        dialog_id: DialogId,
        /// Dialog access hash
        dialog_access_hash: i64,
    },
    /// Sticker set thumbnail
    StickerSetThumbnail {
        /// Sticker set ID
        sticker_set_id: i64,
        /// Sticker set access hash
        sticker_set_access_hash: i64,
    },
    /// Full legacy photos with volume_id, local_id, secret
    FullLegacy {
        /// Volume ID
        volume_id: i64,
        /// Local ID
        local_id: i32,
        /// Secret
        secret: i64,
    },
    /// Legacy dialog photo (small)
    DialogPhotoSmallLegacy {
        /// Dialog ID
        dialog_id: DialogId,
        /// Dialog access hash
        dialog_access_hash: i64,
        /// Volume ID
        volume_id: i64,
        /// Local ID
        local_id: i32,
    },
    /// Legacy dialog photo (big)
    DialogPhotoBigLegacy {
        /// Dialog ID
        dialog_id: DialogId,
        /// Dialog access hash
        dialog_access_hash: i64,
        /// Volume ID
        volume_id: i64,
        /// Local ID
        local_id: i32,
    },
    /// Legacy sticker set thumbnail
    StickerSetThumbnailLegacy {
        /// Sticker set ID
        sticker_set_id: i64,
        /// Sticker set access hash
        sticker_set_access_hash: i64,
        /// Volume ID
        volume_id: i64,
        /// Local ID
        local_id: i32,
    },
    /// Sticker set thumbnail identified by version
    StickerSetThumbnailVersion {
        /// Sticker set ID
        sticker_set_id: i64,
        /// Sticker set access hash
        sticker_set_access_hash: i64,
        /// Version
        version: i32,
    },
}

impl PhotoSizeSource {
    /// Create a thumbnail source.
    ///
    /// # Arguments
    ///
    /// * `file_type` - The file type of the thumbnail
    /// * `thumbnail_type` - The thumbnail type character
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_source::PhotoSizeSource;
    /// use rustgram_file_type::FileType;
    /// use rustgram_photo_size_type::PhotoSizeType;
    ///
    /// let source = PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s'));
    /// ```
    pub fn thumbnail(file_type: FileType, thumbnail_type: PhotoSizeType) -> Self {
        PhotoSizeSource::Thumbnail {
            file_type,
            thumbnail_type,
        }
    }

    /// Create a dialog photo source.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `dialog_access_hash` - The dialog access hash
    /// * `is_big` - Whether this is the big variant
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_source::PhotoSizeSource;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(123);
    /// let source = PhotoSizeSource::dialog_photo(dialog_id, 456, false);
    /// ```
    pub fn dialog_photo(dialog_id: DialogId, dialog_access_hash: i64, is_big: bool) -> Self {
        if is_big {
            PhotoSizeSource::DialogPhotoBig {
                dialog_id,
                dialog_access_hash,
            }
        } else {
            PhotoSizeSource::DialogPhotoSmall {
                dialog_id,
                dialog_access_hash,
            }
        }
    }

    /// Create a full legacy source.
    ///
    /// # Arguments
    ///
    /// * `volume_id` - The volume ID
    /// * `local_id` - The local ID
    /// * `secret` - The secret
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_source::PhotoSizeSource;
    ///
    /// let source = PhotoSizeSource::full_legacy(12345, 678, 999);
    /// ```
    pub fn full_legacy(volume_id: i64, local_id: i32, secret: i64) -> Self {
        PhotoSizeSource::FullLegacy {
            volume_id,
            local_id,
            secret,
        }
    }

    /// Create a legacy dialog photo source.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `dialog_access_hash` - The dialog access hash
    /// * `is_big` - Whether this is the big variant
    /// * `volume_id` - The volume ID
    /// * `local_id` - The local ID
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_source::PhotoSizeSource;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(123);
    /// let source = PhotoSizeSource::dialog_photo_legacy(dialog_id, 456, false, 789, 10);
    /// ```
    pub fn dialog_photo_legacy(
        dialog_id: DialogId,
        dialog_access_hash: i64,
        is_big: bool,
        volume_id: i64,
        local_id: i32,
    ) -> Self {
        if is_big {
            PhotoSizeSource::DialogPhotoBigLegacy {
                dialog_id,
                dialog_access_hash,
                volume_id,
                local_id,
            }
        } else {
            PhotoSizeSource::DialogPhotoSmallLegacy {
                dialog_id,
                dialog_access_hash,
                volume_id,
                local_id,
            }
        }
    }

    /// Create a legacy sticker set thumbnail source.
    ///
    /// # Arguments
    ///
    /// * `sticker_set_id` - The sticker set ID
    /// * `sticker_set_access_hash` - The sticker set access hash
    /// * `volume_id` - The volume ID
    /// * `local_id` - The local ID
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_source::PhotoSizeSource;
    ///
    /// let source = PhotoSizeSource::sticker_set_thumbnail_legacy(12345, 678, 789, 10);
    /// ```
    pub fn sticker_set_thumbnail_legacy(
        sticker_set_id: i64,
        sticker_set_access_hash: i64,
        volume_id: i64,
        local_id: i32,
    ) -> Self {
        PhotoSizeSource::StickerSetThumbnailLegacy {
            sticker_set_id,
            sticker_set_access_hash,
            volume_id,
            local_id,
        }
    }

    /// Create a sticker set thumbnail source with version.
    ///
    /// # Arguments
    ///
    /// * `sticker_set_id` - The sticker set ID
    /// * `sticker_set_access_hash` - The sticker set access hash
    /// * `version` - The version
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_source::PhotoSizeSource;
    ///
    /// let source = PhotoSizeSource::sticker_set_thumbnail(12345, 678, 1);
    /// ```
    pub fn sticker_set_thumbnail(
        sticker_set_id: i64,
        sticker_set_access_hash: i64,
        version: i32,
    ) -> Self {
        PhotoSizeSource::StickerSetThumbnailVersion {
            sticker_set_id,
            sticker_set_access_hash,
            version,
        }
    }

    /// Get the file type for this source.
    ///
    /// Returns the file type if applicable, None otherwise.
    pub fn file_type(&self) -> Option<FileType> {
        match self {
            PhotoSizeSource::Thumbnail { file_type, .. } => Some(*file_type),
            _ => None,
        }
    }

    /// Check if this is a legacy source.
    pub fn is_legacy(&self) -> bool {
        matches!(
            self,
            PhotoSizeSource::Legacy { .. }
                | PhotoSizeSource::FullLegacy { .. }
                | PhotoSizeSource::DialogPhotoSmallLegacy { .. }
                | PhotoSizeSource::DialogPhotoBigLegacy { .. }
                | PhotoSizeSource::StickerSetThumbnailLegacy { .. }
        )
    }

    /// Check if this is a dialog photo source.
    pub fn is_dialog_photo(&self) -> bool {
        matches!(
            self,
            PhotoSizeSource::DialogPhotoSmall { .. }
                | PhotoSizeSource::DialogPhotoBig { .. }
                | PhotoSizeSource::DialogPhotoSmallLegacy { .. }
                | PhotoSizeSource::DialogPhotoBigLegacy { .. }
        )
    }

    /// Check if this is a sticker set thumbnail source.
    pub fn is_sticker_set_thumbnail(&self) -> bool {
        matches!(
            self,
            PhotoSizeSource::StickerSetThumbnail { .. }
                | PhotoSizeSource::StickerSetThumbnailLegacy { .. }
                | PhotoSizeSource::StickerSetThumbnailVersion { .. }
        )
    }
}

impl fmt::Display for PhotoSizeSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhotoSizeSource::Legacy { secret } => {
                write!(f, "Legacy(secret: {})", secret)
            }
            PhotoSizeSource::Thumbnail {
                file_type,
                thumbnail_type,
            } => write!(f, "Thumbnail({:?}, {})", file_type, thumbnail_type),
            PhotoSizeSource::DialogPhotoSmall {
                dialog_id,
                dialog_access_hash,
            } => write!(
                f,
                "DialogPhotoSmall(dialog_id: {}, hash: {})",
                dialog_id.get(),
                dialog_access_hash
            ),
            PhotoSizeSource::DialogPhotoBig {
                dialog_id,
                dialog_access_hash,
            } => write!(
                f,
                "DialogPhotoBig(dialog_id: {}, hash: {})",
                dialog_id.get(),
                dialog_access_hash
            ),
            PhotoSizeSource::StickerSetThumbnail {
                sticker_set_id,
                sticker_set_access_hash,
            } => write!(
                f,
                "StickerSetThumbnail(id: {}, hash: {})",
                sticker_set_id, sticker_set_access_hash
            ),
            PhotoSizeSource::FullLegacy {
                volume_id,
                local_id,
                secret,
            } => write!(
                f,
                "FullLegacy(volume: {}, local: {}, secret: {})",
                volume_id, local_id, secret
            ),
            PhotoSizeSource::DialogPhotoSmallLegacy {
                dialog_id,
                dialog_access_hash,
                volume_id,
                local_id,
            } => write!(
                f,
                "DialogPhotoSmallLegacy(dialog_id: {}, hash: {}, volume: {}, local: {})",
                dialog_id.get(),
                dialog_access_hash,
                volume_id,
                local_id
            ),
            PhotoSizeSource::DialogPhotoBigLegacy {
                dialog_id,
                dialog_access_hash,
                volume_id,
                local_id,
            } => write!(
                f,
                "DialogPhotoBigLegacy(dialog_id: {}, hash: {}, volume: {}, local: {})",
                dialog_id.get(),
                dialog_access_hash,
                volume_id,
                local_id
            ),
            PhotoSizeSource::StickerSetThumbnailLegacy {
                sticker_set_id,
                sticker_set_access_hash,
                volume_id,
                local_id,
            } => write!(
                f,
                "StickerSetThumbnailLegacy(id: {}, hash: {}, volume: {}, local: {})",
                sticker_set_id, sticker_set_access_hash, volume_id, local_id
            ),
            PhotoSizeSource::StickerSetThumbnailVersion {
                sticker_set_id,
                sticker_set_access_hash,
                version,
            } => write!(
                f,
                "StickerSetThumbnailVersion(id: {}, hash: {}, version: {})",
                sticker_set_id, sticker_set_access_hash, version
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dialog_id(id: i64) -> DialogId {
        DialogId::new(id)
    }

    // Basic trait tests (8 tests)
    #[test]
    fn test_clone() {
        let a = PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s'));
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_partial_eq() {
        let a = PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s'));
        let b = PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s'));
        assert_eq!(a, b);

        let c = PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('m'));
        assert_ne!(a, c);
    }

    #[test]
    fn test_debug() {
        let source = PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s'));
        let debug_str = format!("{:?}", source);
        assert!(debug_str.contains("Thumbnail"));
    }

    // Constructor tests (12 tests)
    #[test]
    fn test_thumbnail() {
        let source = PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s'));
        assert!(matches!(source, PhotoSizeSource::Thumbnail { .. }));
    }

    #[test]
    fn test_dialog_photo_small() {
        let source = PhotoSizeSource::dialog_photo(make_dialog_id(123), 456, false);
        assert!(matches!(source, PhotoSizeSource::DialogPhotoSmall { .. }));
    }

    #[test]
    fn test_dialog_photo_big() {
        let source = PhotoSizeSource::dialog_photo(make_dialog_id(123), 456, true);
        assert!(matches!(source, PhotoSizeSource::DialogPhotoBig { .. }));
    }

    #[test]
    fn test_full_legacy() {
        let source = PhotoSizeSource::full_legacy(12345, 678, 999);
        assert!(matches!(source, PhotoSizeSource::FullLegacy { .. }));
    }

    #[test]
    fn test_dialog_photo_legacy_small() {
        let source = PhotoSizeSource::dialog_photo_legacy(make_dialog_id(123), 456, false, 789, 10);
        assert!(matches!(
            source,
            PhotoSizeSource::DialogPhotoSmallLegacy { .. }
        ));
    }

    #[test]
    fn test_dialog_photo_legacy_big() {
        let source = PhotoSizeSource::dialog_photo_legacy(make_dialog_id(123), 456, true, 789, 10);
        assert!(matches!(
            source,
            PhotoSizeSource::DialogPhotoBigLegacy { .. }
        ));
    }

    #[test]
    fn test_sticker_set_thumbnail_legacy() {
        let source = PhotoSizeSource::sticker_set_thumbnail_legacy(12345, 678, 789, 10);
        assert!(matches!(
            source,
            PhotoSizeSource::StickerSetThumbnailLegacy { .. }
        ));
    }

    #[test]
    fn test_sticker_set_thumbnail() {
        let source = PhotoSizeSource::sticker_set_thumbnail(12345, 678, 1);
        assert!(matches!(
            source,
            PhotoSizeSource::StickerSetThumbnailVersion { .. }
        ));
    }

    // Method tests (6 tests)
    #[test]
    fn test_file_type() {
        let source = PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s'));
        assert_eq!(source.file_type(), Some(FileType::Photo));
    }

    #[test]
    fn test_file_type_none() {
        let source = PhotoSizeSource::Legacy { secret: 123 };
        assert_eq!(source.file_type(), None);
    }

    #[test]
    fn test_is_legacy() {
        assert!(PhotoSizeSource::Legacy { secret: 123 }.is_legacy());
        assert!(PhotoSizeSource::full_legacy(1, 2, 3).is_legacy());
        assert!(!PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s')).is_legacy());
    }

    #[test]
    fn test_is_dialog_photo() {
        assert!(PhotoSizeSource::dialog_photo(make_dialog_id(123), 456, false).is_dialog_photo());
        assert!(PhotoSizeSource::dialog_photo(make_dialog_id(123), 456, true).is_dialog_photo());
        assert!(
            !PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s')).is_dialog_photo()
        );
    }

    #[test]
    fn test_is_sticker_set_thumbnail() {
        assert!(PhotoSizeSource::sticker_set_thumbnail(123, 456, 1).is_sticker_set_thumbnail());
        assert!(
            PhotoSizeSource::sticker_set_thumbnail_legacy(123, 456, 789, 10)
                .is_sticker_set_thumbnail()
        );
        assert!(
            !PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s'))
                .is_sticker_set_thumbnail()
        );
    }

    // Display tests (5 tests)
    #[test]
    fn test_display_thumbnail() {
        let source = PhotoSizeSource::thumbnail(FileType::Photo, PhotoSizeType::new('s'));
        let display_str = format!("{}", source);
        assert!(display_str.contains("Thumbnail"));
    }
}
