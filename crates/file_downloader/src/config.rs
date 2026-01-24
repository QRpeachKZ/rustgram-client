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

//! Configuration for file downloads.

use rustgram_file_encryption_key::FileEncryptionKey;
use rustgram_file_location::{FullRemoteFileLocation, LocalFileLocation};
use std::fmt;

/// Configuration for file downloads.
#[derive(Debug, Clone)]
pub struct FileDownloaderConfig {
    /// Remote file location
    pub remote: FullRemoteFileLocation,
    /// Local file location
    pub local: LocalFileLocation,
    /// File size
    pub size: i64,
    /// Expected file size
    pub expected_size: Option<i64>,
    /// Whether the size is final
    pub is_size_final: bool,
    /// File name
    pub name: String,
    /// Encryption key
    pub encryption_key: FileEncryptionKey,
    /// Whether to search for existing file
    pub need_search_file: bool,
    /// Download offset
    pub offset: i64,
    /// Download limit
    pub limit: i64,
    /// Ready parts (for resuming)
    pub ready_parts: Vec<i32>,
}

impl FileDownloaderConfig {
    /// Creates a new file downloader configuration.
    ///
    /// # Arguments
    ///
    /// * `remote` - The remote file location
    /// * `size` - The file size
    #[must_use]
    pub fn new(remote: FullRemoteFileLocation, size: i64) -> Self {
        Self {
            remote,
            local: LocalFileLocation::empty(),
            size,
            expected_size: None,
            is_size_final: false,
            name: String::new(),
            encryption_key: FileEncryptionKey::empty(),
            need_search_file: false,
            offset: 0,
            limit: 0,
            ready_parts: Vec::new(),
        }
    }

    /// Sets the local file location.
    ///
    /// # Arguments
    ///
    /// * `local` - The local file location
    #[must_use]
    pub fn with_local(mut self, local: LocalFileLocation) -> Self {
        self.local = local;
        self
    }

    /// Sets the expected file size.
    ///
    /// # Arguments
    ///
    /// * `expected_size` - The expected file size
    #[must_use]
    pub const fn with_expected_size(mut self, expected_size: i64) -> Self {
        self.expected_size = Some(expected_size);
        self
    }

    /// Sets whether the size is final.
    ///
    /// # Arguments
    ///
    /// * `is_final` - Whether the size is final
    #[must_use]
    pub const fn with_size_final(mut self, is_final: bool) -> Self {
        self.is_size_final = is_final;
        self
    }

    /// Sets the file name.
    ///
    /// # Arguments
    ///
    /// * `name` - The file name
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
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

    /// Sets whether to search for an existing file.
    ///
    /// # Arguments
    ///
    /// * `need_search` - Whether to search for the file
    #[must_use]
    pub const fn with_need_search_file(mut self, need_search: bool) -> Self {
        self.need_search_file = need_search;
        self
    }

    /// Sets the download offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The download offset
    #[must_use]
    pub const fn with_offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the download limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - The download limit
    #[must_use]
    pub const fn with_limit(mut self, limit: i64) -> Self {
        self.limit = limit;
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

impl fmt::Display for FileDownloaderConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FileDownloaderConfig(size={}, offset={}, limit={})",
            self.size, self.offset, self.limit
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        assert_eq!(config.size, 10_000_000);
        assert_eq!(config.offset, 0);
        assert_eq!(config.limit, 0);
    }

    #[test]
    fn test_builder() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000)
            .with_expected_size(10_000_000)
            .with_size_final(true)
            .with_name(String::from("test.jpg"))
            .with_offset(1000)
            .with_limit(5000);

        assert_eq!(config.expected_size, Some(10_000_000));
        assert!(config.is_size_final);
        assert_eq!(config.name, "test.jpg");
        assert_eq!(config.offset, 1000);
        assert_eq!(config.limit, 5000);
    }

    #[test]
    fn test_display() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let s = format!("{config}");
        assert!(s.contains("FileDownloaderConfig"));
        assert!(s.contains("10000000"));
    }
}
