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

//! Configuration for file uploads.

use rustgram_file_encryption_key::FileEncryptionKey;
use rustgram_file_id::FileId;
use rustgram_file_type::FileType;
use std::fmt;

/// Configuration for file uploads.
#[derive(Debug, Clone)]
pub struct FileUploaderConfig {
    /// File path
    pub path: String,
    /// File type
    pub file_type: FileType,
    /// File ID
    pub file_id: Option<FileId>,
    /// File size
    pub size: i64,
    /// Expected file size
    pub expected_size: i64,
    /// Whether the size is final
    pub size_is_final: bool,
    /// Encryption key
    pub encryption_key: FileEncryptionKey,
    /// Ready parts (for resuming)
    pub ready_parts: Vec<i32>,
}

impl FileUploaderConfig {
    /// Creates a new file uploader configuration.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path
    /// * `file_type` - The file type
    /// * `file_id` - The optional file ID
    #[must_use]
    pub fn new(path: &str, file_type: FileType, file_id: Option<FileId>) -> Self {
        Self {
            path: path.to_string(),
            file_type,
            file_id,
            size: 0,
            expected_size: 0,
            size_is_final: false,
            encryption_key: FileEncryptionKey::empty(),
            ready_parts: Vec::new(),
        }
    }

    /// Sets the file size.
    ///
    /// # Arguments
    ///
    /// * `size` - The file size
    #[must_use]
    pub const fn with_size(mut self, size: i64) -> Self {
        self.size = size;
        self
    }

    /// Sets the expected file size.
    ///
    /// # Arguments
    ///
    /// * `expected_size` - The expected file size
    #[must_use]
    pub const fn with_expected_size(mut self, expected_size: i64) -> Self {
        self.expected_size = expected_size;
        self
    }

    /// Sets whether the size is final.
    ///
    /// # Arguments
    ///
    /// * `is_final` - Whether the size is final
    #[must_use]
    pub const fn with_size_final(mut self, is_final: bool) -> Self {
        self.size_is_final = is_final;
        self
    }

    /// Sets the encryption key.
    ///
    /// # Arguments
    ///
    /// * `encryption_key` - The encryption key
    #[must_use]
    pub fn with_encryption_key(mut self, encryption_key: FileEncryptionKey) -> Self {
        self.encryption_key = encryption_key;
        self
    }

    /// Sets the ready parts (for resuming).
    ///
    /// # Arguments
    ///
    /// * `ready_parts` - The ready part indices
    #[must_use]
    pub fn with_ready_parts(mut self, ready_parts: Vec<i32>) -> Self {
        self.ready_parts = ready_parts;
        self
    }
}

impl fmt::Display for FileUploaderConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FileUploaderConfig(path={:?}, type={:?}, size={})",
            self.path, self.file_type, self.size
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None);

        assert_eq!(config.path, "/path/to/file.jpg");
        assert_eq!(config.file_type, FileType::Photo);
        assert_eq!(config.size, 0);
    }

    #[test]
    fn test_builder() {
        let file_id = FileId::new(123, 0);
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, Some(file_id))
            .with_size(10_000_000)
            .with_expected_size(10_000_000)
            .with_size_final(true);

        assert_eq!(config.size, 10_000_000);
        assert_eq!(config.expected_size, 10_000_000);
        assert!(config.size_is_final);
        assert_eq!(config.file_id, Some(file_id));
    }

    #[test]
    fn test_display() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None);

        let s = format!("{config}");
        assert!(s.contains("FileUploaderConfig"));
        assert!(s.contains("/path/to/file.jpg"));
    }
}
