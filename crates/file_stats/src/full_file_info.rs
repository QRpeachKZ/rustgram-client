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

//! Full file information.
//!
//! Contains `FullFileInfo` which stores complete metadata about a file.

use rustgram_dialog_id::DialogId;
use rustgram_file_type::FileType;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Complete file metadata.
///
/// Stores all information about a file including its type, path,
/// owner dialog, size, and timestamps. Corresponds to TDLib's `FullFileInfo`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FullFileInfo {
    /// The type of file.
    pub file_type: FileType,
    /// File path on disk.
    pub path: String,
    /// Dialog that owns this file.
    pub owner_dialog_id: DialogId,
    /// File size in bytes.
    pub size: i64,
    /// Access time in nanoseconds since Unix epoch.
    pub atime_nsec: u64,
    /// Modification time in nanoseconds since Unix epoch.
    pub mtime_nsec: u64,
}

impl FullFileInfo {
    /// Creates a new `FullFileInfo`.
    ///
    /// # Arguments
    ///
    /// * `file_type` - The type of file
    /// * `path` - File path on disk
    /// * `owner_dialog_id` - Dialog that owns this file
    /// * `size` - File size in bytes
    /// * `atime_nsec` - Access time in nanoseconds
    /// * `mtime_nsec` - Modification time in nanoseconds
    #[must_use]
    pub const fn new(
        file_type: FileType,
        path: String,
        owner_dialog_id: DialogId,
        size: i64,
        atime_nsec: u64,
        mtime_nsec: u64,
    ) -> Self {
        Self {
            file_type,
            path,
            owner_dialog_id,
            size,
            atime_nsec,
            mtime_nsec,
        }
    }

    /// Creates a new `FullFileInfo` with default timestamps.
    #[must_use]
    pub const fn with_defaults(
        file_type: FileType,
        path: String,
        owner_dialog_id: DialogId,
        size: i64,
    ) -> Self {
        Self {
            file_type,
            path,
            owner_dialog_id,
            size,
            atime_nsec: 0,
            mtime_nsec: 0,
        }
    }

    /// Returns the file path as a reference.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the file name from the path.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        Path::new(&self.path)
            .file_name()
            .and_then(|name| name.to_str())
    }

    /// Returns the file extension from the path.
    #[must_use]
    pub fn extension(&self) -> Option<&str> {
        Path::new(&self.path)
            .extension()
            .and_then(|ext| ext.to_str())
    }

    /// Returns the access time in nanoseconds.
    #[must_use]
    pub const fn atime_nsec(&self) -> u64 {
        self.atime_nsec
    }

    /// Returns the modification time in nanoseconds.
    #[must_use]
    pub const fn mtime_nsec(&self) -> u64 {
        self.mtime_nsec
    }

    /// Returns the access time in seconds.
    #[must_use]
    pub const fn atime_sec(&self) -> u64 {
        self.atime_nsec / 1_000_000_000
    }

    /// Returns the modification time in seconds.
    #[must_use]
    pub const fn mtime_sec(&self) -> u64 {
        self.mtime_nsec / 1_000_000_000
    }

    /// Returns `true` if the file has valid timestamps.
    #[must_use]
    pub const fn has_valid_timestamps(&self) -> bool {
        self.atime_nsec > 0 || self.mtime_nsec > 0
    }

    /// Returns `true` if the file is empty (size is 0).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns `true` if the file size is greater than zero.
    #[must_use]
    pub const fn has_content(&self) -> bool {
        self.size > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_info() -> FullFileInfo {
        FullFileInfo::new(
            FileType::Photo,
            "/path/to/photo.jpg".to_string(),
            DialogId::new(12345),
            1024000,
            1_700_000_000_000_000_000,
            1_700_000_001_000_000_000,
        )
    }

    // === Construction tests ===

    #[test]
    fn test_new() {
        let info = FullFileInfo::new(
            FileType::Video,
            "/path/to/video.mp4".to_string(),
            DialogId::new(67890),
            2048000,
            100,
            200,
        );
        assert_eq!(info.file_type, FileType::Video);
        assert_eq!(info.path, "/path/to/video.mp4");
        assert_eq!(info.owner_dialog_id.get(), 67890);
        assert_eq!(info.size, 2048000);
        assert_eq!(info.atime_nsec, 100);
        assert_eq!(info.mtime_nsec, 200);
    }

    #[test]
    fn test_default() {
        let info = FullFileInfo::default();
        assert_eq!(info.file_type, FileType::None);
        assert!(info.path.is_empty());
        assert_eq!(info.owner_dialog_id.get(), 0);
        assert_eq!(info.size, 0);
        assert_eq!(info.atime_nsec, 0);
        assert_eq!(info.mtime_nsec, 0);
    }

    #[test]
    fn test_with_defaults() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            "/path/to/doc.pdf".to_string(),
            DialogId::new(11111),
            512000,
        );
        assert_eq!(info.file_type, FileType::Document);
        assert_eq!(info.path, "/path/to/doc.pdf");
        assert_eq!(info.owner_dialog_id.get(), 11111);
        assert_eq!(info.size, 512000);
        assert_eq!(info.atime_nsec, 0);
        assert_eq!(info.mtime_nsec, 0);
    }

    // === Path methods ===

    #[test]
    fn test_path() {
        let info = make_test_info();
        assert_eq!(info.path(), "/path/to/photo.jpg");
    }

    #[test]
    fn test_file_name() {
        let info = make_test_info();
        assert_eq!(info.file_name(), Some("photo.jpg"));
    }

    #[test]
    fn test_file_name_no_extension() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            "/path/to/no_ext_file".to_string(),
            DialogId::new(1),
            100,
        );
        assert_eq!(info.file_name(), Some("no_ext_file"));
    }

    #[test]
    fn test_file_name_empty_path() {
        let info = FullFileInfo::default();
        assert_eq!(info.file_name(), None);
    }

    #[test]
    fn test_extension() {
        let info = make_test_info();
        assert_eq!(info.extension(), Some("jpg"));
    }

    #[test]
    fn test_extension_none() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            "/path/to/no_ext".to_string(),
            DialogId::new(1),
            100,
        );
        assert_eq!(info.extension(), None);
    }

    #[test]
    fn test_extension_empty_path() {
        let info = FullFileInfo::default();
        assert_eq!(info.extension(), None);
    }

    #[test]
    fn test_extension_multiple_dots() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            "/path/to/archive.tar.gz".to_string(),
            DialogId::new(1),
            100,
        );
        assert_eq!(info.extension(), Some("gz"));
    }

    // === Timestamp methods ===

    #[test]
    fn test_atime_nsec() {
        let info = make_test_info();
        assert_eq!(info.atime_nsec(), 1_700_000_000_000_000_000);
    }

    #[test]
    fn test_mtime_nsec() {
        let info = make_test_info();
        assert_eq!(info.mtime_nsec(), 1_700_000_001_000_000_000);
    }

    #[test]
    fn test_atime_sec() {
        let info = make_test_info();
        assert_eq!(info.atime_sec(), 1_700_000_000);
    }

    #[test]
    fn test_mtime_sec() {
        let info = make_test_info();
        assert_eq!(info.mtime_sec(), 1_700_000_001);
    }

    #[test]
    fn test_has_valid_timestamps_true() {
        let info = make_test_info();
        assert!(info.has_valid_timestamps());
    }

    #[test]
    fn test_has_valid_timestamps_atime_only() {
        let mut info = make_test_info();
        info.mtime_nsec = 0;
        assert!(info.has_valid_timestamps());
    }

    #[test]
    fn test_has_valid_timestamps_mtime_only() {
        let mut info = make_test_info();
        info.atime_nsec = 0;
        assert!(info.has_valid_timestamps());
    }

    #[test]
    fn test_has_valid_timestamps_false() {
        let info = FullFileInfo::default();
        assert!(!info.has_valid_timestamps());
    }

    // === Size methods ===

    #[test]
    fn test_is_empty_true() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            "/path/to/empty".to_string(),
            DialogId::new(1),
            0,
        );
        assert!(info.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let info = make_test_info();
        assert!(!info.is_empty());
    }

    #[test]
    fn test_has_content_true() {
        let info = make_test_info();
        assert!(info.has_content());
    }

    #[test]
    fn test_has_content_false() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            "/path/to/empty".to_string(),
            DialogId::new(1),
            0,
        );
        assert!(!info.has_content());
    }

    // === Equality tests ===

    #[test]
    fn test_equality_true() {
        let info1 = make_test_info();
        let info2 = make_test_info();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_equality_false_file_type() {
        let mut info2 = make_test_info();
        info2.file_type = FileType::Video;
        assert_ne!(make_test_info(), info2);
    }

    #[test]
    fn test_equality_false_path() {
        let mut info2 = make_test_info();
        info2.path = "/other/path".to_string();
        assert_ne!(make_test_info(), info2);
    }

    #[test]
    fn test_equality_false_owner() {
        let mut info2 = make_test_info();
        info2.owner_dialog_id = DialogId::new(99999);
        assert_ne!(make_test_info(), info2);
    }

    #[test]
    fn test_equality_false_size() {
        let mut info2 = make_test_info();
        info2.size = 9999;
        assert_ne!(make_test_info(), info2);
    }

    #[test]
    fn test_equality_false_atime() {
        let mut info2 = make_test_info();
        info2.atime_nsec = 999;
        assert_ne!(make_test_info(), info2);
    }

    #[test]
    fn test_equality_false_mtime() {
        let mut info2 = make_test_info();
        info2.mtime_nsec = 888;
        assert_ne!(make_test_info(), info2);
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize() {
        let info = make_test_info();
        let json = serde_json::to_string(&info).expect("Failed to serialize");
        assert!(json.contains("/path/to/photo.jpg"));
        assert!(json.contains("1024000"));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "file_type":"Photo",
            "path":"/path/to/photo.jpg",
            "owner_dialog_id":12345,
            "size":1024000,
            "atime_nsec":1700000000000000000,
            "mtime_nsec":1700000001000000000
        }"#;
        let info: FullFileInfo = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(info.file_type, FileType::Photo);
        assert_eq!(info.path, "/path/to/photo.jpg");
        assert_eq!(info.owner_dialog_id.get(), 12345);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let original = make_test_info();
        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: FullFileInfo =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }

    // === Clone tests ===

    #[test]
    fn test_clone() {
        let info1 = make_test_info();
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    // === Debug tests ===

    #[test]
    fn test_debug_format() {
        let info = make_test_info();
        let debug = format!("{info:?}");
        assert!(debug.contains("Photo"));
        assert!(debug.contains("/path/to/photo.jpg"));
    }

    // === Windows path handling tests ===

    #[test]
    fn test_windows_path_file_name() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            r"C:\Users\test\document.pdf".to_string(),
            DialogId::new(1),
            100,
        );
        // Windows paths use backslashes, Path::file_name still works
        let name = info.file_name();
        assert!(name.is_some());
    }

    #[test]
    fn test_windows_path_extension() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            r"C:\Users\test\document.pdf".to_string(),
            DialogId::new(1),
            100,
        );
        let ext = info.extension();
        assert_eq!(ext, Some("pdf"));
    }

    // === Edge cases ===

    #[test]
    fn test_path_root_only() {
        let info =
            FullFileInfo::with_defaults(FileType::Document, "/".to_string(), DialogId::new(1), 100);
        assert_eq!(info.file_name(), None);
        assert_eq!(info.extension(), None);
    }

    #[test]
    fn test_path_directory_only() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            "/path/to/directory/".to_string(),
            DialogId::new(1),
            100,
        );
        // Paths ending in / still have the directory name as the final component
        assert_eq!(info.file_name(), Some("directory"));
        assert_eq!(info.extension(), None);
    }

    #[test]
    fn test_path_hidden_file() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            "/path/to/.hidden".to_string(),
            DialogId::new(1),
            100,
        );
        assert_eq!(info.file_name(), Some(".hidden"));
        assert_eq!(info.extension(), None);
    }

    #[test]
    fn test_path_dotfile_with_extension() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            "/path/to/.hidden.txt".to_string(),
            DialogId::new(1),
            100,
        );
        assert_eq!(info.file_name(), Some(".hidden.txt"));
        assert_eq!(info.extension(), Some("txt"));
    }

    // === Large file tests ===

    #[test]
    fn test_large_file_size() {
        let info = FullFileInfo::new(
            FileType::Video,
            "/path/to/large.mp4".to_string(),
            DialogId::new(1),
            i64::MAX,
            0,
            0,
        );
        assert_eq!(info.size, i64::MAX);
        assert!(info.has_content());
    }

    #[test]
    fn test_negative_size_means_no_content() {
        let info = FullFileInfo::with_defaults(
            FileType::Document,
            "/path/to/doc".to_string(),
            DialogId::new(1),
            -100,
        );
        // has_content() returns size > 0, so negative means no content
        assert!(!info.has_content());
        assert_eq!(info.size, -100);
    }
}
