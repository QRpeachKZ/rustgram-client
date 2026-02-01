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

//! # File Upload ID
//!
//! Identifier for tracking file uploads in TDLib.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `FileUploadId` class from `td/telegram/files/FileUploadId.h`.
//!
//! ## Usage
//!
//! ```
//! use rustgram_file_upload_id::FileUploadId;
//! use rustgram_file_id::FileId;
//!
//! let file_id = FileId::new(42, 0);
//! let upload_id = FileUploadId::new(file_id, 123);
//! assert!(upload_id.is_valid());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use rustgram_file_id::FileId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// File upload identifier.
///
/// Combines a file ID with an internal upload ID for tracking uploads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct FileUploadId {
    /// The file ID being uploaded
    file_id: FileId,
    /// The internal upload ID
    internal_upload_id: i64,
}

impl FileUploadId {
    /// Creates a new file upload ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID being uploaded
    /// * `internal_upload_id` - The internal upload ID
    #[must_use]
    pub const fn new(file_id: FileId, internal_upload_id: i64) -> Self {
        Self {
            file_id,
            internal_upload_id,
        }
    }

    /// Returns `true` if this upload ID is valid (has a valid file ID).
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.file_id.is_valid()
    }

    /// Returns the file ID.
    #[must_use]
    pub const fn file_id(&self) -> FileId {
        self.file_id
    }

    /// Returns the internal upload ID.
    #[must_use]
    pub const fn internal_upload_id(&self) -> i64 {
        self.internal_upload_id
    }
}

impl fmt::Display for FileUploadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "file {}+{}", self.file_id, self.internal_upload_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // === Constructor tests ===

    #[test]
    fn test_new() {
        let file_id = FileId::new(42, 0);
        let upload_id = FileUploadId::new(file_id, 123);
        assert_eq!(upload_id.file_id(), file_id);
        assert_eq!(upload_id.internal_upload_id(), 123);
    }

    #[test]
    fn test_default() {
        let upload_id = FileUploadId::default();
        assert!(!upload_id.is_valid());
    }

    // === Validity tests ===

    #[test]
    fn test_is_valid() {
        let file_id = FileId::new(42, 0);
        let upload_id = FileUploadId::new(file_id, 123);
        assert!(upload_id.is_valid());
    }

    #[test]
    fn test_is_valid_invalid_file_id() {
        let file_id = FileId::empty();
        let upload_id = FileUploadId::new(file_id, 123);
        assert!(!upload_id.is_valid());
    }

    // === Getter tests ===

    #[test]
    fn test_file_id() {
        let file_id = FileId::new(42, 0);
        let upload_id = FileUploadId::new(file_id, 123);
        assert_eq!(upload_id.file_id(), file_id);
    }

    #[test]
    fn test_internal_upload_id() {
        let file_id = FileId::new(42, 0);
        let upload_id = FileUploadId::new(file_id, 123);
        assert_eq!(upload_id.internal_upload_id(), 123);
    }

    // === Comparison tests ===

    #[test]
    fn test_equality() {
        let file_id = FileId::new(42, 0);
        let upload_id1 = FileUploadId::new(file_id, 123);
        let upload_id2 = FileUploadId::new(file_id, 123);
        let upload_id3 = FileUploadId::new(file_id, 456);

        assert_eq!(upload_id1, upload_id2);
        assert_ne!(upload_id1, upload_id3);
    }

    #[test]
    fn test_equality_different_file_id() {
        let file_id1 = FileId::new(42, 0);
        let file_id2 = FileId::new(43, 0);
        let upload_id1 = FileUploadId::new(file_id1, 123);
        let upload_id2 = FileUploadId::new(file_id2, 123);

        assert_ne!(upload_id1, upload_id2);
    }

    // === Display tests ===

    #[test]
    fn test_display() {
        let file_id = FileId::new(42, 0);
        let upload_id = FileUploadId::new(file_id, 123);
        let s = format!("{upload_id}");
        assert!(s.contains("42"));
        assert!(s.contains("123"));
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize() {
        let file_id = FileId::new(42, 0);
        let upload_id = FileUploadId::new(file_id, 123);
        let json = serde_json::to_string(&upload_id).unwrap();
        assert!(json.contains("file_id"));
        assert!(json.contains("internal_upload_id"));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"file_id":{"id":42,"remote_id":0},"internal_upload_id":123}"#;
        let upload_id: FileUploadId = serde_json::from_str(json).unwrap();
        assert_eq!(upload_id.internal_upload_id(), 123);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let file_id = FileId::new(42, 0);
        let original = FileUploadId::new(file_id, 123);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: FileUploadId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // === Hash tests ===

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let file_id = FileId::new(42, 0);
        let upload_id1 = FileUploadId::new(file_id, 123);
        let upload_id2 = FileUploadId::new(file_id, 123);
        let upload_id3 = FileUploadId::new(file_id, 456);

        let mut set = HashSet::new();
        set.insert(upload_id1);
        set.insert(upload_id2);
        set.insert(upload_id3);

        assert_eq!(set.len(), 2); // upload_id1 and upload_id2 are the same
    }
}
