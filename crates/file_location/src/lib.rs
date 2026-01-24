// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # File Location
//!
//! File location types for remote and local file storage.
//!
//! ## TDLib Correspondence
//!
//! This module implements TDLib file location types from `td/telegram/files/FileLocation.h`.
//!
//! ## Structure
//!
//! - **RemoteFileLocation**: Empty, Partial, or Full remote location
//! - **LocalFileLocation**: Empty, Partial, or Full local location
//! - **GenerateFileLocation**: Empty or Full generate location
//!
//! ## Usage
//!
//! ```
//! use rustgram_file_location::{RemoteFileLocation, FullRemoteFileLocation};
//!
//! // Create a full remote file location
//! let remote = FullRemoteFileLocation::common(123, 456);
//! let location = RemoteFileLocation::full(remote);
//! assert!(!location.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::fmt;

// === Invalid file reference marker ===

/// Invalid file reference marker (used by TDLib to detect bad references).
pub const INVALID_FILE_REFERENCE: &str = "#";

// === Empty Remote File Location ===

/// Empty remote file location (file has no remote location yet).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmptyRemoteFileLocation;

// === Partial Remote File Location ===

/// Partial remote file location (file is being downloaded).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PartialRemoteFileLocation {
    /// File ID
    pub file_id: i64,
    /// Total number of parts
    pub part_count: i32,
    /// Size of each part
    pub part_size: i32,
    /// Number of ready parts
    pub ready_part_count: i32,
    /// Whether this is a "big" file
    pub is_big: i32,
    /// Total ready size
    pub ready_size: i64,
}

impl PartialRemoteFileLocation {
    /// Creates a new partial remote file location.
    #[must_use]
    pub const fn new(
        file_id: i64,
        part_count: i32,
        part_size: i32,
        ready_part_count: i32,
        is_big: i32,
        ready_size: i64,
    ) -> Self {
        Self {
            file_id,
            part_count,
            part_size,
            ready_part_count,
            is_big,
            ready_size,
        }
    }

    /// Returns `true` if the download is complete.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        self.ready_part_count >= self.part_count
    }
}

impl fmt::Display for PartialRemoteFileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size_type = if self.is_big != 0 { "Big" } else { "Small" };
        write!(
            f,
            "[{} partial remote: {} parts of size {}, {} ready, total {}]",
            size_type, self.part_count, self.part_size, self.ready_part_count, self.ready_size
        )
    }
}

// === Photo Remote File Location ===

/// Photo remote file location.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhotoRemoteFileLocation {
    /// Photo ID
    pub id: i64,
    /// Access hash
    pub access_hash: i64,
    /// Photo size source
    pub volume_id: i64,
    /// Local ID
    pub local_id: i32,
    /// Secret
    pub secret: i64,
}

impl PhotoRemoteFileLocation {
    /// Creates a new photo remote file location.
    #[must_use]
    pub const fn new(
        id: i64,
        access_hash: i64,
        volume_id: i64,
        local_id: i32,
        secret: i64,
    ) -> Self {
        Self {
            id,
            access_hash,
            volume_id,
            local_id,
            secret,
        }
    }
}

impl fmt::Display for PhotoRemoteFileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Photo ID: {}, access_hash: {}, volume_id: {}, local_id: {}]",
            self.id, self.access_hash, self.volume_id, self.local_id
        )
    }
}

// === Web Remote File Location ===

/// Web remote file location (file from URL).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WebRemoteFileLocation {
    /// URL
    pub url: String,
    /// Access hash
    pub access_hash: i64,
}

impl WebRemoteFileLocation {
    /// Creates a new web remote file location.
    #[must_use]
    pub fn new(url: String, access_hash: i64) -> Self {
        Self { url, access_hash }
    }

    /// Returns the URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }
}

impl fmt::Display for WebRemoteFileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Web URL: {}, access_hash: {}]",
            self.url, self.access_hash
        )
    }
}

// === Common Remote File Location ===

/// Common remote file location (most document types).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommonRemoteFileLocation {
    /// File ID
    pub id: i64,
    /// Access hash
    pub access_hash: i64,
}

impl CommonRemoteFileLocation {
    /// Creates a new common remote file location.
    #[must_use]
    pub const fn new(id: i64, access_hash: i64) -> Self {
        Self { id, access_hash }
    }
}

impl fmt::Display for CommonRemoteFileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ID: {}, access_hash: {}]", self.id, self.access_hash)
    }
}

// === Full Remote File Location ===

/// Full remote file location (can be Photo, Web, or Common).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FullRemoteFileLocation {
    /// Photo location
    Photo(PhotoRemoteFileLocation),
    /// Web location
    Web(WebRemoteFileLocation),
    /// Common location
    Common(CommonRemoteFileLocation),
}

impl FullRemoteFileLocation {
    /// Creates a common remote file location.
    #[must_use]
    pub const fn common(id: i64, access_hash: i64) -> Self {
        // Note: Simplified implementation - TDLib has more complex logic here
        Self::Common(CommonRemoteFileLocation::new(id, access_hash))
    }

    /// Creates a photo remote file location.
    #[must_use]
    pub const fn photo(
        id: i64,
        access_hash: i64,
        volume_id: i64,
        local_id: i32,
        secret: i64,
    ) -> Self {
        Self::Photo(PhotoRemoteFileLocation::new(
            id,
            access_hash,
            volume_id,
            local_id,
            secret,
        ))
    }

    /// Creates a web remote file location.
    #[must_use]
    pub fn web(_file_type: rustgram_file_type::FileType, url: String, access_hash: i64) -> Self {
        Self::Web(WebRemoteFileLocation::new(url, access_hash))
    }

    /// Returns `true` if this is a web location.
    #[must_use]
    pub fn is_web(&self) -> bool {
        matches!(self, Self::Web(_))
    }

    /// Returns `true` if this is a photo location.
    #[must_use]
    pub fn is_photo(&self) -> bool {
        matches!(self, Self::Photo(_))
    }

    /// Returns `true` if this is a common location.
    #[must_use]
    pub fn is_common(&self) -> bool {
        matches!(self, Self::Common(_))
    }

    /// Returns the web location if applicable.
    #[must_use]
    pub fn as_web(&self) -> Option<&WebRemoteFileLocation> {
        match self {
            Self::Web(loc) => Some(loc),
            _ => None,
        }
    }

    /// Returns the photo location if applicable.
    #[must_use]
    pub fn as_photo(&self) -> Option<&PhotoRemoteFileLocation> {
        match self {
            Self::Photo(loc) => Some(loc),
            _ => None,
        }
    }

    /// Returns the common location if applicable.
    #[must_use]
    pub fn as_common(&self) -> Option<&CommonRemoteFileLocation> {
        match self {
            Self::Common(loc) => Some(loc),
            _ => None,
        }
    }
}

impl fmt::Display for FullRemoteFileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Photo(loc) => write!(f, "{loc}"),
            Self::Web(loc) => write!(f, "{loc}"),
            Self::Common(loc) => write!(f, "{loc}"),
        }
    }
}

// === Remote File Location ===

/// Remote file location (can be Empty, Partial, or Full).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteFileLocation {
    /// Empty (no remote location)
    Empty,
    /// Partial (being downloaded)
    Partial(PartialRemoteFileLocation),
    /// Full (complete remote location)
    Full(FullRemoteFileLocation),
}

impl RemoteFileLocation {
    /// Creates an empty remote file location.
    #[must_use]
    pub const fn empty() -> Self {
        Self::Empty
    }

    /// Creates a full remote file location.
    #[must_use]
    pub fn full(location: FullRemoteFileLocation) -> Self {
        Self::Full(location)
    }

    /// Creates a partial remote file location.
    #[must_use]
    pub fn partial(location: PartialRemoteFileLocation) -> Self {
        Self::Partial(location)
    }

    /// Returns the location type.
    #[must_use]
    pub const fn type_(&self) -> LocationType {
        match self {
            Self::Empty => LocationType::Empty,
            Self::Partial(_) => LocationType::Partial,
            Self::Full(_) => LocationType::Full,
        }
    }

    /// Returns `true` if the location is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns the partial location if applicable.
    #[must_use]
    pub fn as_partial(&self) -> Option<&PartialRemoteFileLocation> {
        match self {
            Self::Partial(loc) => Some(loc),
            _ => None,
        }
    }

    /// Returns the full location if applicable.
    #[must_use]
    pub fn as_full(&self) -> Option<&FullRemoteFileLocation> {
        match self {
            Self::Full(loc) => Some(loc),
            _ => None,
        }
    }
}

impl Default for RemoteFileLocation {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for RemoteFileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "[empty remote location]"),
            Self::Partial(loc) => write!(f, "{loc}"),
            Self::Full(loc) => write!(f, "{loc}"),
        }
    }
}

// === Location Type ===

/// Location type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocationType {
    /// Empty location
    Empty,
    /// Partial location
    Partial,
    /// Full location
    Full,
}

// === Empty Local File Location ===

/// Empty local file location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmptyLocalFileLocation;

// === Partial Local File Location ===

/// Partial local file location (file is being downloaded/uploaded).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PartialLocalFileLocation {
    /// File type
    pub file_type: rustgram_file_type::FileType,
    /// Part size
    pub part_size: i64,
    /// File path
    pub path: String,
    /// Initialization vector
    pub iv: String,
    /// Ready bitmask
    pub ready_bitmask: String,
    /// Ready size
    pub ready_size: i64,
}

impl PartialLocalFileLocation {
    /// Creates a new partial local file location.
    #[must_use]
    pub fn new(
        file_type: rustgram_file_type::FileType,
        part_size: i64,
        path: String,
        iv: String,
        ready_bitmask: String,
        ready_size: i64,
    ) -> Self {
        Self {
            file_type,
            part_size,
            path,
            iv,
            ready_bitmask,
            ready_size,
        }
    }
}

impl fmt::Display for PartialLocalFileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[partial local: {}, part_size: {}, path: {}, ready_size: {}]",
            self.file_type, self.part_size, self.path, self.ready_size
        )
    }
}

// === Full Local File Location ===

/// Full local file location.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FullLocalFileLocation {
    /// File type
    pub file_type: rustgram_file_type::FileType,
    /// File path
    pub path: String,
    /// Modification time (nanoseconds)
    pub mtime_nsec: u64,
}

impl FullLocalFileLocation {
    /// Creates a new full local file location.
    #[must_use]
    pub fn new(file_type: rustgram_file_type::FileType, path: String, mtime_nsec: u64) -> Self {
        Self {
            file_type,
            path,
            mtime_nsec,
        }
    }

    /// Returns the file path.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl fmt::Display for FullLocalFileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[full local: {} at {}]", self.file_type, self.path)
    }
}

// === Local File Location ===

/// Local file location (can be Empty, Partial, or Full).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalFileLocation {
    /// Empty
    Empty,
    /// Partial
    Partial(PartialLocalFileLocation),
    /// Full
    Full(FullLocalFileLocation),
}

impl LocalFileLocation {
    /// Creates an empty local file location.
    #[must_use]
    pub const fn empty() -> Self {
        Self::Empty
    }

    /// Creates a full local file location.
    #[must_use]
    pub fn full(location: FullLocalFileLocation) -> Self {
        Self::Full(location)
    }

    /// Creates a partial local file location.
    #[must_use]
    pub fn partial(location: PartialLocalFileLocation) -> Self {
        Self::Partial(location)
    }

    /// Returns `true` if the location is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns the file name if available.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        match self {
            Self::Partial(loc) => Some(&loc.path),
            Self::Full(loc) => Some(&loc.path),
            Self::Empty => None,
        }
    }

    /// Returns the partial location if applicable.
    #[must_use]
    pub fn as_partial(&self) -> Option<&PartialLocalFileLocation> {
        match self {
            Self::Partial(loc) => Some(loc),
            _ => None,
        }
    }

    /// Returns the full location if applicable.
    #[must_use]
    pub fn as_full(&self) -> Option<&FullLocalFileLocation> {
        match self {
            Self::Full(loc) => Some(loc),
            _ => None,
        }
    }
}

impl Default for LocalFileLocation {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for LocalFileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "[empty local location]"),
            Self::Partial(loc) => write!(f, "{loc}"),
            Self::Full(loc) => write!(f, "{loc}"),
        }
    }
}

// === Generate File Location ===

/// Generate file location (for files generated from other files).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenerateFileLocation {
    /// Empty
    Empty,
    /// Full
    Full(FullGenerateFileLocation),
}

impl GenerateFileLocation {
    /// Creates an empty generate file location.
    #[must_use]
    pub const fn empty() -> Self {
        Self::Empty
    }

    /// Returns `true` if the location is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl Default for GenerateFileLocation {
    fn default() -> Self {
        Self::empty()
    }
}

/// Full generate file location.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FullGenerateFileLocation {
    /// File type
    pub file_type: rustgram_file_type::FileType,
    /// Original path
    pub original_path: String,
    /// Conversion string
    pub conversion: String,
}

impl FullGenerateFileLocation {
    /// Creates a new full generate file location.
    #[must_use]
    pub fn new(
        file_type: rustgram_file_type::FileType,
        original_path: String,
        conversion: String,
    ) -> Self {
        Self {
            file_type,
            original_path,
            conversion,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_file_type::FileType;

    // === EmptyRemoteFileLocation tests ===

    #[test]
    fn test_empty_remote_file_location() {
        let _empty = EmptyRemoteFileLocation;
        // Just test that it exists and can be created
    }

    // === PartialRemoteFileLocation tests ===

    #[test]
    fn test_partial_remote_file_location_new() {
        let partial = PartialRemoteFileLocation::new(123, 10, 512, 5, 0, 2560);
        assert_eq!(partial.file_id, 123);
        assert_eq!(partial.part_count, 10);
        assert_eq!(partial.part_size, 512);
        assert_eq!(partial.ready_part_count, 5);
        assert!(!partial.is_ready());

        let complete = PartialRemoteFileLocation::new(123, 10, 512, 10, 0, 5120);
        assert!(complete.is_ready());
    }

    #[test]
    fn test_partial_remote_display() {
        let partial = PartialRemoteFileLocation::new(123, 10, 512, 5, 0, 2560);
        let s = format!("{partial}");
        assert!(s.contains("partial remote"));
        assert!(s.contains("10"));
    }

    // === PhotoRemoteFileLocation tests ===

    #[test]
    fn test_photo_remote_file_location_new() {
        let photo = PhotoRemoteFileLocation::new(123, 456, 789, 10, 999);
        assert_eq!(photo.id, 123);
        assert_eq!(photo.access_hash, 456);
        assert_eq!(photo.volume_id, 789);
        assert_eq!(photo.local_id, 10);
        assert_eq!(photo.secret, 999);
    }

    // === WebRemoteFileLocation tests ===

    #[test]
    fn test_web_remote_file_location_new() {
        let web = WebRemoteFileLocation::new(String::from("https://example.com/file"), 12345);
        assert_eq!(web.url, "https://example.com/file");
        assert_eq!(web.access_hash, 12345);
        assert_eq!(web.url(), "https://example.com/file");
    }

    // === CommonRemoteFileLocation tests ===

    #[test]
    fn test_common_remote_file_location_new() {
        let common = CommonRemoteFileLocation::new(123, 456);
        assert_eq!(common.id, 123);
        assert_eq!(common.access_hash, 456);
    }

    // === FullRemoteFileLocation tests ===

    #[test]
    fn test_full_remote_location_common() {
        let full = FullRemoteFileLocation::common(123, 456);
        assert!(full.is_common());
        assert!(!full.is_photo());
        assert!(!full.is_web());
        assert!(full.as_common().is_some());
    }

    #[test]
    fn test_full_remote_location_photo() {
        let full = FullRemoteFileLocation::photo(123, 456, 789, 10, 999);
        assert!(full.is_photo());
        assert!(!full.is_common());
        assert!(!full.is_web());
        assert!(full.as_photo().is_some());
    }

    #[test]
    fn test_full_remote_location_web() {
        let full = FullRemoteFileLocation::web(
            FileType::Video,
            String::from("https://example.com/file"),
            12345,
        );
        assert!(full.is_web());
        assert!(!full.is_common());
        assert!(!full.is_photo());
        assert!(full.as_web().is_some());
    }

    // === RemoteFileLocation tests ===

    #[test]
    fn test_remote_location_empty() {
        let remote = RemoteFileLocation::empty();
        assert!(remote.is_empty());
        assert_eq!(remote.type_(), LocationType::Empty);
    }

    #[test]
    fn test_remote_location_full() {
        let full = FullRemoteFileLocation::common(123, 456);
        let remote = RemoteFileLocation::full(full);
        assert!(!remote.is_empty());
        assert_eq!(remote.type_(), LocationType::Full);
        assert!(remote.as_full().is_some());
    }

    #[test]
    fn test_remote_location_partial() {
        let partial = PartialRemoteFileLocation::new(123, 10, 512, 5, 0, 2560);
        let remote = RemoteFileLocation::partial(partial);
        assert!(!remote.is_empty());
        assert_eq!(remote.type_(), LocationType::Partial);
        assert!(remote.as_partial().is_some());
    }

    #[test]
    fn test_remote_location_default() {
        let remote = RemoteFileLocation::default();
        assert!(remote.is_empty());
    }

    // === PartialLocalFileLocation tests ===

    #[test]
    fn test_partial_local_file_location_new() {
        let partial = PartialLocalFileLocation::new(
            FileType::Document,
            512,
            String::from("/path/to/file"),
            String::from("iv"),
            String::from("bitmask"),
            2560,
        );
        assert_eq!(partial.file_type, FileType::Document);
        assert_eq!(partial.part_size, 512);
        assert_eq!(partial.path, "/path/to/file");
    }

    // === FullLocalFileLocation tests ===

    #[test]
    fn test_full_local_file_location_new() {
        let full =
            FullLocalFileLocation::new(FileType::Photo, String::from("/path/to/photo.jpg"), 12345);
        assert_eq!(full.file_type, FileType::Photo);
        assert_eq!(full.path, "/path/to/photo.jpg");
        assert_eq!(full.mtime_nsec, 12345);
        assert_eq!(full.path(), "/path/to/photo.jpg");
    }

    // === LocalFileLocation tests ===

    #[test]
    fn test_local_location_empty() {
        let local = LocalFileLocation::empty();
        assert!(local.is_empty());
        assert!(local.file_name().is_none());
    }

    #[test]
    fn test_local_location_full() {
        let full =
            FullLocalFileLocation::new(FileType::Photo, String::from("/path/to/photo.jpg"), 12345);
        let local = LocalFileLocation::full(full);
        assert!(!local.is_empty());
        assert_eq!(local.file_name(), Some("/path/to/photo.jpg"));
        assert!(local.as_full().is_some());
    }

    #[test]
    fn test_local_location_partial() {
        let partial = PartialLocalFileLocation::new(
            FileType::Document,
            512,
            String::from("/path/to/file"),
            String::from("iv"),
            String::from("bitmask"),
            2560,
        );
        let local = LocalFileLocation::partial(partial);
        assert!(!local.is_empty());
        assert_eq!(local.file_name(), Some("/path/to/file"));
        assert!(local.as_partial().is_some());
    }

    #[test]
    fn test_local_location_default() {
        let local = LocalFileLocation::default();
        assert!(local.is_empty());
    }

    // === GenerateFileLocation tests ===

    #[test]
    fn test_generate_location_empty() {
        let gen = GenerateFileLocation::empty();
        assert!(gen.is_empty());
    }

    #[test]
    fn test_full_generate_file_location_new() {
        let full = FullGenerateFileLocation::new(
            FileType::Animation,
            String::from("/path/to/input"),
            String::from("convert_to_mp4"),
        );
        assert_eq!(full.file_type, FileType::Animation);
        assert_eq!(full.original_path, "/path/to/input");
        assert_eq!(full.conversion, "convert_to_mp4");
    }

    // === Display tests ===

    #[test]
    fn test_display_remote_empty() {
        let remote = RemoteFileLocation::empty();
        assert_eq!(format!("{remote}"), "[empty remote location]");
    }

    #[test]
    fn test_display_local_empty() {
        let local = LocalFileLocation::empty();
        assert_eq!(format!("{local}"), "[empty local location]");
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize_common_remote_location() {
        let loc = CommonRemoteFileLocation::new(123, 456);
        let json = serde_json::to_string(&loc).unwrap();
        assert!(json.contains("123"));
        assert!(json.contains("456"));
    }

    #[test]
    fn test_deserialize_common_remote_location() {
        let json = r#"{"id":123,"access_hash":456}"#;
        let loc: CommonRemoteFileLocation = serde_json::from_str(json).unwrap();
        assert_eq!(loc.id, 123);
        assert_eq!(loc.access_hash, 456);
    }

    #[test]
    fn test_serialize_full_local_location() {
        let loc =
            FullLocalFileLocation::new(FileType::Photo, String::from("/path/to/photo.jpg"), 12345);
        let json = serde_json::to_string(&loc).unwrap();
        assert!(json.contains("/path/to/photo.jpg"));
    }

    #[test]
    fn test_serialize_roundtrip_remote() {
        let original = RemoteFileLocation::full(FullRemoteFileLocation::common(123, 456));
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: RemoteFileLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_serialize_roundtrip_local() {
        let original = LocalFileLocation::full(FullLocalFileLocation::new(
            FileType::Photo,
            String::from("/path/to/photo.jpg"),
            12345,
        ));
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LocalFileLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
