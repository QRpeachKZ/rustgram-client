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

//! # Encrypted File
//!
//! Encrypted file information for secret chats.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `EncryptedFile` struct from `td/telegram/EncryptedFile.h`.
//!
//! ## Structure
//!
//! - `EncryptedFile`: Contains encrypted file metadata (id, access_hash, size, dc_id, key_fingerprint)
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_encrypted_file::EncryptedFile;
//!
//! let file = EncryptedFile::new(123, 456, 789, 1, 100).unwrap();
//! assert_eq!(file.id(), 123);
//! assert_eq!(file.size(), 789);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Encrypted file information for secret chats.
///
/// Corresponds to TDLib `EncryptedFile` struct.
/// Contains metadata about encrypted files used in secret chats.
///
/// ## TDLib Mapping
///
/// - `id_` → `id()`
/// - `access_hash_` → `access_hash()`
/// - `size_` → `size()`
/// - `dc_id_` → `dc_id()`
/// - `key_fingerprint_` → `key_fingerprint()`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedFile {
    /// File identifier
    id: i64,
    /// Access hash for file access validation
    access_hash: i64,
    /// File size in bytes (must be >= 0)
    size: i64,
    /// Data center ID where the file is stored
    dc_id: i32,
    /// Key fingerprint for encryption validation
    key_fingerprint: i32,
}

impl Default for EncryptedFile {
    /// Creates a default empty encrypted file.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_encrypted_file::EncryptedFile;
    ///
    /// let file = EncryptedFile::default();
    /// assert_eq!(file.id(), 0);
    /// assert_eq!(file.size(), 0);
    /// ```
    fn default() -> Self {
        Self {
            id: 0,
            access_hash: 0,
            size: 0,
            dc_id: 0,
            key_fingerprint: 0,
        }
    }
}

impl EncryptedFile {
    /// Creates a new encrypted file.
    ///
    /// # Arguments
    ///
    /// * `id` - File identifier
    /// * `access_hash` - Access hash for file access validation
    /// * `size` - File size in bytes (must be >= 0)
    /// * `dc_id` - Data center ID where the file is stored
    /// * `key_fingerprint` - Key fingerprint for encryption validation
    ///
    /// # Returns
    ///
    /// Returns `Err(EncryptedFileError::InvalidSize)` if size is negative.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_encrypted_file::EncryptedFile;
    ///
    /// let file = EncryptedFile::new(123, 456, 789, 1, 100).unwrap();
    /// assert_eq!(file.id(), 123);
    /// assert_eq!(file.size(), 789);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the size is negative.
    pub fn new(
        id: i64,
        access_hash: i64,
        size: i64,
        dc_id: i32,
        key_fingerprint: i32,
    ) -> Result<Self, EncryptedFileError> {
        if size < 0 {
            return Err(EncryptedFileError::InvalidSize);
        }
        Ok(Self {
            id,
            access_hash,
            size,
            dc_id,
            key_fingerprint,
        })
    }

    /// Creates a new encrypted file without validation.
    ///
    /// # Arguments
    ///
    /// * `id` - File identifier
    /// * `access_hash` - Access hash for file access validation
    /// * `size` - File size in bytes
    /// * `dc_id` - Data center ID where the file is stored
    /// * `key_fingerprint` - Key fingerprint for encryption validation
    ///
    /// # Safety
    ///
    /// This function does not validate that size is non-negative.
    /// Use `new()` for validated construction.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_encrypted_file::EncryptedFile;
    ///
    /// let file = EncryptedFile::new_unchecked(123, 456, 789, 1, 100);
    /// assert_eq!(file.id(), 123);
    /// ```
    #[must_use]
    pub const fn new_unchecked(
        id: i64,
        access_hash: i64,
        size: i64,
        dc_id: i32,
        key_fingerprint: i32,
    ) -> Self {
        Self {
            id,
            access_hash,
            size,
            dc_id,
            key_fingerprint,
        }
    }

    /// Returns the file identifier.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_encrypted_file::EncryptedFile;
    ///
    /// let file = EncryptedFile::new(123, 456, 789, 1, 100)?;
    /// assert_eq!(file.id(), 123);
    /// # Ok::<(), rustgram_encrypted_file::EncryptedFileError>(())
    /// ```
    #[must_use]
    pub const fn id(&self) -> i64 {
        self.id
    }

    /// Returns the access hash.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_encrypted_file::EncryptedFile;
    ///
    /// let file = EncryptedFile::new(123, 456, 789, 1, 100)?;
    /// assert_eq!(file.access_hash(), 456);
    /// # Ok::<(), rustgram_encrypted_file::EncryptedFileError>(())
    /// ```
    #[must_use]
    pub const fn access_hash(&self) -> i64 {
        self.access_hash
    }

    /// Returns the file size in bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_encrypted_file::EncryptedFile;
    ///
    /// let file = EncryptedFile::new(123, 456, 789, 1, 100)?;
    /// assert_eq!(file.size(), 789);
    /// # Ok::<(), rustgram_encrypted_file::EncryptedFileError>(())
    /// ```
    #[must_use]
    pub const fn size(&self) -> i64 {
        self.size
    }

    /// Returns the data center ID.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_encrypted_file::EncryptedFile;
    ///
    /// let file = EncryptedFile::new(123, 456, 789, 1, 100)?;
    /// assert_eq!(file.dc_id(), 1);
    /// # Ok::<(), rustgram_encrypted_file::EncryptedFileError>(())
    /// ```
    #[must_use]
    pub const fn dc_id(&self) -> i32 {
        self.dc_id
    }

    /// Returns the key fingerprint.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_encrypted_file::EncryptedFile;
    ///
    /// let file = EncryptedFile::new(123, 456, 789, 1, 100)?;
    /// assert_eq!(file.key_fingerprint(), 100);
    /// # Ok::<(), rustgram_encrypted_file::EncryptedFileError>(())
    /// ```
    #[must_use]
    pub const fn key_fingerprint(&self) -> i32 {
        self.key_fingerprint
    }

    /// Checks if this file uses 64-bit size serialization.
    ///
    /// Returns `true` if the size is >= 2^31, which requires
    /// 64-bit storage format.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_encrypted_file::EncryptedFile;
    ///
    /// let small_file = EncryptedFile::new(1, 1, 1000, 1, 1)?;
    /// assert!(!small_file.has_64bit_size());
    ///
    /// let large_file = EncryptedFile::new(2, 2, 0x100000000, 1, 1)?;
    /// assert!(large_file.has_64bit_size());
    /// # Ok::<(), rustgram_encrypted_file::EncryptedFileError>(())
    /// ```
    #[must_use]
    pub fn has_64bit_size(&self) -> bool {
        self.size >= (1i64 << 31)
    }

    /// Checks if the file is empty (all fields are zero).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_encrypted_file::EncryptedFile;
    ///
    /// let empty = EncryptedFile::default();
    /// assert!(empty.is_empty());
    ///
    /// let file = EncryptedFile::new(1, 1, 1, 1, 1)?;
    /// assert!(!file.is_empty());
    /// # Ok::<(), rustgram_encrypted_file::EncryptedFileError>(())
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id == 0
            && self.access_hash == 0
            && self.size == 0
            && self.dc_id == 0
            && self.key_fingerprint == 0
    }
}

impl fmt::Display for EncryptedFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EncryptedFile[id={}, access_hash={}, size={}, dc_id={}, key_fingerprint={}]",
            self.id, self.access_hash, self.size, self.dc_id, self.key_fingerprint
        )
    }
}

/// Errors that can occur when working with encrypted files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptedFileError {
    /// Invalid file size (negative value)
    InvalidSize,
}

impl fmt::Display for EncryptedFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSize => write!(f, "Invalid file size: size must be non-negative"),
        }
    }
}

impl std::error::Error for EncryptedFileError {}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests
    #[test]
    fn test_default() {
        let file = EncryptedFile::default();
        assert_eq!(file.id(), 0);
        assert_eq!(file.access_hash(), 0);
        assert_eq!(file.size(), 0);
        assert_eq!(file.dc_id(), 0);
        assert_eq!(file.key_fingerprint(), 0);
    }

    #[test]
    fn test_clone() {
        let file = EncryptedFile::new(123, 456, 789, 1, 100).unwrap();
        let cloned = file.clone();
        assert_eq!(file, cloned);
    }

    #[test]
    fn test_equality() {
        let file1 = EncryptedFile::new(123, 456, 789, 1, 100).unwrap();
        let file2 = EncryptedFile::new(123, 456, 789, 1, 100).unwrap();
        assert_eq!(file1, file2);
    }

    #[test]
    fn test_inequality() {
        let file1 = EncryptedFile::new(123, 456, 789, 1, 100).unwrap();
        let file2 = EncryptedFile::new(124, 456, 789, 1, 100).unwrap();
        assert_ne!(file1, file2);
    }

    // Constructor tests
    #[test]
    fn test_new_valid() {
        let file = EncryptedFile::new(123, 456, 789, 1, 100);
        assert!(file.is_ok());
        let file = file.unwrap();
        assert_eq!(file.id(), 123);
        assert_eq!(file.access_hash(), 456);
        assert_eq!(file.size(), 789);
        assert_eq!(file.dc_id(), 1);
        assert_eq!(file.key_fingerprint(), 100);
    }

    #[test]
    fn test_new_invalid_size() {
        let file = EncryptedFile::new(123, 456, -1, 1, 100);
        assert!(file.is_err());
        assert_eq!(file.unwrap_err(), EncryptedFileError::InvalidSize);
    }

    #[test]
    fn test_new_zero_size() {
        let file = EncryptedFile::new(123, 456, 0, 1, 100);
        assert!(file.is_ok());
        assert_eq!(file.unwrap().size(), 0);
    }

    #[test]
    fn test_new_unchecked() {
        let file = EncryptedFile::new_unchecked(123, 456, 789, 1, 100);
        assert_eq!(file.id(), 123);
        assert_eq!(file.access_hash(), 456);
        assert_eq!(file.size(), 789);
        assert_eq!(file.dc_id(), 1);
        assert_eq!(file.key_fingerprint(), 100);
    }

    // Getter tests
    #[test]
    fn test_getters() {
        let file = EncryptedFile::new(123456, 789012, 999999, 2, 200).unwrap();
        assert_eq!(file.id(), 123456);
        assert_eq!(file.access_hash(), 789012);
        assert_eq!(file.size(), 999999);
        assert_eq!(file.dc_id(), 2);
        assert_eq!(file.key_fingerprint(), 200);
    }

    // 64-bit size tests
    #[test]
    fn test_has_64bit_size_small() {
        let file = EncryptedFile::new(1, 1, 1000, 1, 1).unwrap();
        assert!(!file.has_64bit_size());
    }

    #[test]
    fn test_has_64bit_size_boundary() {
        let file = EncryptedFile::new(1, 1, (1i64 << 31) - 1, 1, 1).unwrap();
        assert!(!file.has_64bit_size());

        let file = EncryptedFile::new(1, 1, 1i64 << 31, 1, 1).unwrap();
        assert!(file.has_64bit_size());
    }

    #[test]
    fn test_has_64bit_size_large() {
        let file = EncryptedFile::new(1, 1, 0x100000000, 1, 1).unwrap();
        assert!(file.has_64bit_size());
    }

    // isEmpty tests
    #[test]
    fn test_is_empty_true() {
        let file = EncryptedFile::default();
        assert!(file.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let file = EncryptedFile::new(1, 0, 0, 0, 0).unwrap();
        assert!(!file.is_empty());

        let file = EncryptedFile::new(0, 1, 0, 0, 0).unwrap();
        assert!(!file.is_empty());

        let file = EncryptedFile::new(0, 0, 1, 0, 0).unwrap();
        assert!(!file.is_empty());

        let file = EncryptedFile::new(0, 0, 0, 1, 0).unwrap();
        assert!(!file.is_empty());

        let file = EncryptedFile::new(0, 0, 0, 0, 1).unwrap();
        assert!(!file.is_empty());
    }

    // Display tests
    #[test]
    fn test_display() {
        let file = EncryptedFile::new(123, 456, 789, 1, 100).unwrap();
        let display = format!("{}", file);
        assert!(display.contains("123"));
        assert!(display.contains("456"));
        assert!(display.contains("789"));
        assert!(display.contains("1"));
        assert!(display.contains("100"));
    }

    // Error tests
    #[test]
    fn test_error_display() {
        let error = EncryptedFileError::InvalidSize;
        let display = format!("{}", error);
        assert!(display.contains("Invalid"));
    }

    // Serialization tests
    #[test]
    fn test_serialize_deserialize() {
        let file = EncryptedFile::new(123, 456, 789, 1, 100).unwrap();
        let json = serde_json::to_string(&file).unwrap();
        let deserialized: EncryptedFile = serde_json::from_str(&json).unwrap();
        assert_eq!(file, deserialized);
    }

    #[test]
    fn test_serialize_default() {
        let file = EncryptedFile::default();
        let json = serde_json::to_string(&file).unwrap();
        let deserialized: EncryptedFile = serde_json::from_str(&json).unwrap();
        assert_eq!(file, deserialized);
    }

    // Edge cases
    #[test]
    fn test_negative_id() {
        let file = EncryptedFile::new(-1, 456, 789, 1, 100);
        assert!(file.is_ok());
        assert_eq!(file.unwrap().id(), -1);
    }

    #[test]
    fn test_negative_access_hash() {
        let file = EncryptedFile::new(123, -1, 789, 1, 100);
        assert!(file.is_ok());
        assert_eq!(file.unwrap().access_hash(), -1);
    }

    #[test]
    fn test_negative_dc_id() {
        let file = EncryptedFile::new(123, 456, 789, -1, 100);
        assert!(file.is_ok());
        assert_eq!(file.unwrap().dc_id(), -1);
    }

    #[test]
    fn test_negative_key_fingerprint() {
        let file = EncryptedFile::new(123, 456, 789, 1, -1);
        assert!(file.is_ok());
        assert_eq!(file.unwrap().key_fingerprint(), -1);
    }

    #[test]
    fn test_max_values() {
        let file = EncryptedFile::new(i64::MAX, i64::MAX, i64::MAX, i32::MAX, i32::MAX);
        assert!(file.is_ok());
        let file = file.unwrap();
        assert_eq!(file.id(), i64::MAX);
        assert_eq!(file.access_hash(), i64::MAX);
        assert_eq!(file.size(), i64::MAX);
        assert_eq!(file.dc_id(), i32::MAX);
        assert_eq!(file.key_fingerprint(), i32::MAX);
    }

    #[test]
    fn test_min_values() {
        let file = EncryptedFile::new(i64::MIN, i64::MIN, 0, i32::MIN, i32::MIN);
        assert!(file.is_ok());
        let file = file.unwrap();
        assert_eq!(file.id(), i64::MIN);
        assert_eq!(file.access_hash(), i64::MIN);
        assert_eq!(file.size(), 0);
        assert_eq!(file.dc_id(), i32::MIN);
        assert_eq!(file.key_fingerprint(), i32::MIN);
    }
}
