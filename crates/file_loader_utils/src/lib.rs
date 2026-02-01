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

//! # File Loader Utils
//!
//! Utility functions for file loading operations.
//!
//! ## TDLib Correspondence
//!
//! This module implements utility functions from TDLib `FileLoaderUtils.*` classes.
//!
//! ## Overview
//!
//! Provides utilities for:
//! - Path generation for files
//! - File size validation
//! - URL encoding/decoding
//! - Local file operations
//! - File location helpers
//!
//! ## Usage
//!
//! ```
//! use rustgram_file_loader_utils::{generate_file_path, validate_file_size};
//! use rustgram_file_id::FileId;
//! use rustgram_file_type::FileType;
//!
//! let file_id = FileId::new(123, 456);
//! let file_type = FileType::Photo;
//! let path = generate_file_path(file_id, file_type).unwrap();
//! assert!(path.contains("photos"));
//!
//! // Validate file size
//! validate_file_size(1024, Some(10_000_000)).unwrap();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use thiserror::Error;

use rustgram_file_id::FileId;
use rustgram_file_type::FileType;

/// Errors that can occur in file loader utils operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    /// Invalid file size provided.
    #[error("invalid file size: {0}")]
    InvalidSize(i64),

    /// Invalid file path provided.
    #[error("invalid file path: {0}")]
    InvalidPath(String),

    /// I/O error occurred.
    #[error("I/O error: {0}")]
    IoError(String),

    /// Invalid URL encoding/decoding.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    /// File not found.
    #[error("file not found: {0}")]
    FileNotFound(String),

    /// Path is not a file.
    #[error("path is not a file: {0}")]
    NotAFile(String),
}

/// Result type for file loader utils operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Default cache directory name.
const CACHE_DIR_NAME: &str = "rustgram";

/// Default files subdirectory name.
const FILES_DIR_NAME: &str = "files";

/// Generates a file path for the given file ID and type.
///
/// The path format is: `{file_type}/{id}_{remote_id}.ext`
///
/// # Arguments
///
/// * `file_id` - The file identifier
/// * `file_type` - The type of file
///
/// # Returns
///
/// A relative file path string.
///
/// # Examples
///
/// ```
/// use rustgram_file_loader_utils::generate_file_path;
/// use rustgram_file_id::FileId;
/// use rustgram_file_type::FileType;
///
/// let file_id = FileId::new(123, 456);
/// let path = generate_file_path(file_id, FileType::Photo).unwrap();
/// assert!(path.contains("photos"));
/// assert!(path.contains("123"));
/// ```
#[must_use]
pub fn generate_file_path(file_id: FileId, file_type: FileType) -> Option<String> {
    if !file_id.is_valid() {
        return None;
    }

    let dir_name = file_type.dir_name();
    let id = file_id.get();
    let remote_id = file_id.get_remote();

    let extension = get_file_extension(file_type);

    match extension {
        Some(ext) => Some(format!("{}/{}_{}.{}", dir_name, id, remote_id, ext)),
        None => Some(format!("{}/{}_{}", dir_name, id, remote_id)),
    }
}

/// Returns the file extension for a given file type.
///
/// # Arguments
///
/// * `file_type` - The type of file
///
/// # Returns
///
/// An optional string containing the file extension (without the dot).
///
/// # Examples
///
/// ```
/// use rustgram_file_loader_utils::get_file_extension;
/// use rustgram_file_type::FileType;
///
/// assert_eq!(get_file_extension(FileType::Photo), Some("jpg"));
/// assert_eq!(get_file_extension(FileType::Video), Some("mp4"));
/// assert_eq!(get_file_extension(FileType::Document), None);
/// ```
#[must_use]
pub const fn get_file_extension(file_type: FileType) -> Option<&'static str> {
    match file_type {
        FileType::Photo | FileType::ProfilePhoto | FileType::Thumbnail => Some("jpg"),
        FileType::VoiceNote => Some("oga"),
        FileType::Video => Some("mp4"),
        FileType::Audio => Some("mp3"),
        FileType::Animation => Some("gif"),
        FileType::VideoNote => Some("mp4"),
        FileType::Sticker => Some("webp"),
        FileType::PhotoStory | FileType::SelfDestructingPhoto => Some("jpg"),
        FileType::VideoStory | FileType::SelfDestructingVideo => Some("mp4"),
        FileType::SelfDestructingVideoNote => Some("mp4"),
        FileType::SelfDestructingVoiceNote => Some("oga"),
        FileType::Background => Some("jpg"),
        FileType::Ringtone => Some("mp3"),
        FileType::Document
        | FileType::Encrypted
        | FileType::Temp
        | FileType::EncryptedThumbnail
        | FileType::SecureDecrypted
        | FileType::SecureEncrypted
        | FileType::DocumentAsFile
        | FileType::CallLog
        | FileType::Wallpaper
        | FileType::Size
        | FileType::None => None,
    }
}

/// Returns the MIME type for a given file type.
///
/// # Arguments
///
/// * `file_type` - The type of file
///
/// # Returns
///
/// An optional string containing the MIME type.
///
/// # Examples
///
/// ```
/// use rustgram_file_loader_utils::get_mime_type;
/// use rustgram_file_type::FileType;
///
/// assert_eq!(get_mime_type(FileType::Photo), Some("image/jpeg"));
/// assert_eq!(get_mime_type(FileType::Video), Some("video/mp4"));
/// ```
#[must_use]
pub const fn get_mime_type(file_type: FileType) -> Option<&'static str> {
    match file_type {
        FileType::Photo | FileType::ProfilePhoto | FileType::Thumbnail => Some("image/jpeg"),
        FileType::VoiceNote => Some("audio/ogg"),
        FileType::Video => Some("video/mp4"),
        FileType::Audio => Some("audio/mpeg"),
        FileType::Animation => Some("image/gif"),
        FileType::VideoNote => Some("video/mp4"),
        FileType::Sticker => Some("image/webp"),
        FileType::PhotoStory | FileType::SelfDestructingPhoto => Some("image/jpeg"),
        FileType::VideoStory | FileType::SelfDestructingVideo => Some("video/mp4"),
        FileType::SelfDestructingVideoNote => Some("video/mp4"),
        FileType::SelfDestructingVoiceNote => Some("audio/ogg"),
        FileType::Background => Some("image/jpeg"),
        FileType::Ringtone => Some("audio/mpeg"),
        FileType::Document
        | FileType::Encrypted
        | FileType::Temp
        | FileType::EncryptedThumbnail
        | FileType::SecureDecrypted
        | FileType::SecureEncrypted
        | FileType::DocumentAsFile
        | FileType::CallLog
        | FileType::Wallpaper
        | FileType::Size
        | FileType::None => None,
    }
}

/// Validates a file size against an optional maximum size.
///
/// # Arguments
///
/// * `size` - The file size in bytes
/// * `max_size` - Optional maximum allowed size in bytes
///
/// # Returns
///
/// Ok(()) if valid, Err(Error::InvalidSize) otherwise.
///
/// # Examples
///
/// ```
/// use rustgram_file_loader_utils::validate_file_size;
///
/// // Valid size
/// assert!(validate_file_size(1024, Some(10_000_000)).is_ok());
///
/// // Size exceeds maximum
/// assert!(validate_file_size(20_000_000, Some(10_000_000)).is_err());
///
/// // Negative size is invalid
/// assert!(validate_file_size(-1, None).is_err());
/// ```
pub fn validate_file_size(size: i64, max_size: Option<i64>) -> Result<()> {
    if size < 0 {
        return Err(Error::InvalidSize(size));
    }

    if let Some(max) = max_size {
        if size > max {
            return Err(Error::InvalidSize(size));
        }
    }

    Ok(())
}

/// Gets the file size at the given path.
///
/// # Arguments
///
/// * `path` - The path to the file
///
/// # Returns
///
/// The file size in bytes, or an error if the file cannot be accessed.
///
/// # Examples
///
/// ```no_run
/// use rustgram_file_loader_utils::get_file_size;
/// use std::path::Path;
///
/// let path = Path::new("/tmp/test.txt");
/// let size = get_file_size(path).unwrap();
/// ```
pub fn get_file_size(path: &Path) -> Result<u64> {
    if !path.exists() {
        return Err(Error::FileNotFound(path.display().to_string()));
    }

    if !path.is_file() {
        return Err(Error::NotAFile(path.display().to_string()));
    }

    std::fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e| Error::IoError(e.to_string()))
}

/// Encodes a string for use in a URL path component.
///
/// Uses percent-encoding to make the string safe for URLs.
///
/// # Arguments
///
/// * `component` - The string to encode
///
/// # Returns
///
/// A percent-encoded string safe for use in URL paths.
///
/// # Examples
///
/// ```
/// use rustgram_file_loader_utils::encode_url_path_component;
///
/// let encoded = encode_url_path_component("hello world");
/// assert!(encoded.contains("hello%20world"));
/// ```
#[must_use]
pub fn encode_url_path_component(component: &str) -> String {
    percent_encoding::utf8_percent_encode(component, percent_encoding::NON_ALPHANUMERIC).to_string()
}

/// Decodes a percent-encoded URL path component.
///
/// # Arguments
///
/// * `component` - The percent-encoded string to decode
///
/// # Returns
///
/// The decoded string, or an error if the encoding is invalid.
///
/// # Examples
///
/// ```
/// use rustgram_file_loader_utils::decode_url_path_component;
///
/// let decoded = decode_url_path_component("hello%20world").unwrap();
/// assert_eq!(decoded, "hello world");
/// ```
pub fn decode_url_path_component(component: &str) -> Result<String> {
    percent_encoding::percent_decode(component.as_bytes())
        .decode_utf8()
        .map(|cow| cow.into_owned())
        .map_err(|e| Error::InvalidUrl(e.to_string()))
}

/// Ensures a directory exists, creating it if necessary.
///
/// # Arguments
///
/// * `path` - The path to the directory
///
/// # Returns
///
/// Ok(()) if the directory exists or was created successfully.
///
/// # Examples
///
/// ```no_run
/// use rustgram_file_loader_utils::ensure_directory_exists;
/// use std::path::Path;
///
/// let path = Path::new("/tmp/test_dir");
/// ensure_directory_exists(path).unwrap();
/// ```
pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    if path.exists() {
        if path.is_dir() {
            return Ok(());
        }
        return Err(Error::InvalidPath(format!(
            "path exists but is not a directory: {}",
            path.display()
        )));
    }

    std::fs::create_dir_all(path).map_err(|e| Error::IoError(e.to_string()))
}

/// Creates a temporary file with the given prefix.
///
/// # Arguments
///
/// * `prefix` - The prefix for the temporary file name
///
/// # Returns
///
/// A tuple of the file handle and its path.
///
/// # Examples
///
/// ```no_run
/// use rustgram_file_loader_utils::create_temp_file;
///
/// let (file, path) = create_temp_file("test").unwrap();
/// // Use the file...
/// ```
pub fn create_temp_file(prefix: &str) -> Result<(File, PathBuf)> {
    let temp =
        tempfile::NamedTempFile::with_prefix(prefix).map_err(|e| Error::IoError(e.to_string()))?;

    // Persist the temp file to prevent deletion
    let (file, path) = temp.keep().map_err(|e| Error::IoError(e.to_string()))?;

    Ok((file, path))
}

/// Writes data to a file atomically.
///
/// Creates a temporary file, writes the data, then atomically renames
/// it to the target path.
///
/// # Arguments
///
/// * `path` - The target file path
/// * `data` - The data to write
///
/// # Returns
///
/// Ok(()) if the write succeeded.
///
/// # Examples
///
/// ```no_run
/// use rustgram_file_loader_utils::atomic_write;
/// use std::path::Path;
///
/// let path = Path::new("/tmp/test.txt");
/// atomic_write(path, b"Hello, world!").unwrap();
/// ```
pub fn atomic_write(path: &Path, data: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| Error::InvalidPath("no parent directory".to_string()))?;

    ensure_directory_exists(parent)?;

    // Create temp file in the same directory as target for atomic rename
    let temp_path = parent.join(format!(".atomic_write_{}", std::process::id()));

    {
        let temp_file = File::create(&temp_path).map_err(|e| Error::IoError(e.to_string()))?;

        temp_file
            .set_permissions(std::fs::Permissions::from_mode(0o644))
            .map_err(|e| Error::IoError(e.to_string()))?;

        let mut writer = std::io::BufWriter::new(temp_file);
        writer
            .write_all(data)
            .map_err(|e| Error::IoError(e.to_string()))?;
        writer.flush().map_err(|e| Error::IoError(e.to_string()))?;
    }

    std::fs::rename(&temp_path, path).map_err(|e| Error::IoError(e.to_string()))?;

    Ok(())
}

/// Gets the local file path for a given file ID.
///
/// Returns a path in the system's cache directory.
///
/// # Arguments
///
/// * `file_id` - The file identifier
///
/// # Returns
///
/// A PathBuf pointing to the local file location.
///
/// # Examples
///
/// ```
/// use rustgram_file_loader_utils::get_local_path;
/// use rustgram_file_id::FileId;
///
/// let file_id = FileId::new(123, 456);
/// let path = get_local_path(file_id);
/// // Path will be in ~/.cache/rustgram/files/
/// ```
#[must_use]
pub fn get_local_path(file_id: FileId) -> PathBuf {
    let cache_dir = get_cache_dir();
    cache_dir.join(format!("{}_{}", file_id.get(), file_id.get_remote()))
}

/// Gets the cache file path for a given file ID.
///
/// Returns a path in the system's cache directory under the files subdirectory.
///
/// # Arguments
///
/// * `file_id` - The file identifier
///
/// # Returns
///
/// A PathBuf pointing to the cache file location.
///
/// # Examples
///
/// ```
/// use rustgram_file_loader_utils::get_cache_path;
/// use rustgram_file_id::FileId;
///
/// let file_id = FileId::new(123, 456);
/// let path = get_cache_path(file_id);
/// // Path will be in ~/.cache/rustgram/files/
/// ```
#[must_use]
pub fn get_cache_path(file_id: FileId) -> PathBuf {
    let cache_dir = get_cache_dir();
    cache_dir
        .join(FILES_DIR_NAME)
        .join(format!("{}_{}", file_id.get(), file_id.get_remote()))
}

/// Gets the base cache directory for rustgram files.
///
/// # Returns
///
/// A PathBuf pointing to the cache directory.
#[must_use]
fn get_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join(CACHE_DIR_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // === generate_file_path tests ===

    #[rstest]
    #[case(FileType::Photo, "photos")]
    #[case(FileType::Video, "videos")]
    #[case(FileType::Document, "documents")]
    #[case(FileType::Audio, "music")]
    fn test_generate_file_path_dir(#[case] file_type: FileType, #[case] expected_dir: &str) {
        let file_id = FileId::new(123, 456);
        let path = generate_file_path(file_id, file_type);
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(path.contains(expected_dir));
        assert!(path.contains("123"));
        assert!(path.contains("456"));
    }

    #[test]
    fn test_generate_file_path_invalid_id() {
        let file_id = FileId::empty();
        let path = generate_file_path(file_id, FileType::Photo);
        assert!(path.is_none());
    }

    #[test]
    fn test_generate_file_path_with_extension() {
        let file_id = FileId::new(123, 456);
        let path = generate_file_path(file_id, FileType::Photo);
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(path.contains(".jpg"));
    }

    #[test]
    fn test_generate_file_path_without_extension() {
        let file_id = FileId::new(123, 456);
        let path = generate_file_path(file_id, FileType::Document);
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(!path.contains('.'));
    }

    // === get_file_extension tests ===

    #[rstest]
    #[case(FileType::Photo, "jpg")]
    #[case(FileType::ProfilePhoto, "jpg")]
    #[case(FileType::Thumbnail, "jpg")]
    #[case(FileType::Video, "mp4")]
    #[case(FileType::VoiceNote, "oga")]
    #[case(FileType::Audio, "mp3")]
    #[case(FileType::Animation, "gif")]
    #[case(FileType::VideoNote, "mp4")]
    #[case(FileType::Sticker, "webp")]
    fn test_get_file_extension_some(#[case] file_type: FileType, #[case] expected: &str) {
        assert_eq!(get_file_extension(file_type), Some(expected));
    }

    #[rstest]
    #[case(FileType::Document)]
    #[case(FileType::Encrypted)]
    #[case(FileType::Temp)]
    fn test_get_file_extension_none(#[case] file_type: FileType) {
        assert_eq!(get_file_extension(file_type), None);
    }

    // === get_mime_type tests ===

    #[rstest]
    #[case(FileType::Photo, "image/jpeg")]
    #[case(FileType::Video, "video/mp4")]
    #[case(FileType::VoiceNote, "audio/ogg")]
    #[case(FileType::Audio, "audio/mpeg")]
    fn test_get_mime_type_some(#[case] file_type: FileType, #[case] expected: &str) {
        assert_eq!(get_mime_type(file_type), Some(expected));
    }

    #[rstest]
    #[case(FileType::Document)]
    #[case(FileType::Encrypted)]
    fn test_get_mime_type_none(#[case] file_type: FileType) {
        assert_eq!(get_mime_type(file_type), None);
    }

    // === validate_file_size tests ===

    #[test]
    fn test_validate_file_size_valid() {
        assert!(validate_file_size(1024, Some(10_000_000)).is_ok());
        assert!(validate_file_size(10_000_000, Some(10_000_000)).is_ok());
        assert!(validate_file_size(0, Some(10_000_000)).is_ok());
    }

    #[test]
    fn test_validate_file_size_no_max() {
        assert!(validate_file_size(1024, None).is_ok());
        assert!(validate_file_size(10_000_000, None).is_ok());
        assert!(validate_file_size(0, None).is_ok());
    }

    #[test]
    fn test_validate_file_size_exceeds_max() {
        let result = validate_file_size(20_000_000, Some(10_000_000));
        assert!(result.is_err());
        assert_eq!(result, Err(Error::InvalidSize(20_000_000)));
    }

    #[test]
    fn test_validate_file_size_negative() {
        let result = validate_file_size(-1, Some(10_000_000));
        assert!(result.is_err());
        assert_eq!(result, Err(Error::InvalidSize(-1)));
    }

    #[rstest]
    #[case(-1)]
    #[case(-100)]
    #[case(i64::MIN)]
    fn test_validate_file_size_negative_variants(#[case] size: i64) {
        let result = validate_file_size(size, None);
        assert!(result.is_err());
        assert_eq!(result, Err(Error::InvalidSize(size)));
    }

    // === get_file_size tests ===

    #[test]
    fn test_get_file_size_not_found() {
        let path = Path::new("/nonexistent/path/file.txt");
        let result = get_file_size(path);
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::FileNotFound(_))));
    }

    #[test]
    fn test_get_file_size_not_a_file() {
        let path = Path::new("/tmp");
        let result = get_file_size(path);
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::NotAFile(_))));
    }

    #[test]
    fn test_get_file_size_success() {
        let (mut file, path) = create_temp_file("test_size").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        file.flush().unwrap();

        let size = get_file_size(&path).unwrap();
        assert_eq!(size, 13);
    }

    // === encode_url_path_component tests ===

    #[test]
    fn test_encode_url_path_component_simple() {
        let encoded = encode_url_path_component("hello");
        assert_eq!(encoded, "hello");
    }

    #[test]
    fn test_encode_url_path_component_space() {
        let encoded = encode_url_path_component("hello world");
        assert_eq!(encoded, "hello%20world");
    }

    #[test]
    fn test_encode_url_path_component_special() {
        let encoded = encode_url_path_component("hello/world?query=test");
        assert!(encoded.contains("hello%2F"));
        assert!(encoded.contains("world%3F"));
    }

    #[test]
    fn test_encode_url_path_component_unicode() {
        let encoded = encode_url_path_component("hello 世界");
        assert!(encoded.contains("%"));
        assert!(encoded.contains("hello"));
    }

    // === decode_url_path_component tests ===

    #[test]
    fn test_decode_url_path_component_simple() {
        let decoded = decode_url_path_component("hello").unwrap();
        assert_eq!(decoded, "hello");
    }

    #[test]
    fn test_decode_url_path_component_space() {
        let decoded = decode_url_path_component("hello%20world").unwrap();
        assert_eq!(decoded, "hello world");
    }

    #[test]
    fn test_decode_url_path_component_special() {
        let decoded = decode_url_path_component("hello%2Fworld%3Fquery%3Dtest").unwrap();
        assert_eq!(decoded, "hello/world?query=test");
    }

    #[test]
    fn test_decode_url_path_component_invalid() {
        let result = decode_url_path_component("hello%FF");
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidUrl(_))));
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let original = "hello/world?query=test & more";
        let encoded = encode_url_path_component(original);
        let decoded = decode_url_path_component(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    // === ensure_directory_exists tests ===

    #[test]
    fn test_ensure_directory_exists_new() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let test_path = temp_dir.path().join("new_dir");
        assert!(!test_path.exists());

        let result = ensure_directory_exists(&test_path);
        assert!(result.is_ok());
        assert!(test_path.exists());
        assert!(test_path.is_dir());
    }

    #[test]
    fn test_ensure_directory_exists_existing() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let test_path = temp_dir.path();
        assert!(test_path.exists());

        let result = ensure_directory_exists(test_path);
        assert!(result.is_ok());
        assert!(test_path.exists());
    }

    #[test]
    fn test_ensure_directory_exists_nested() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let test_path = temp_dir.path().join("parent/child/grandchild");
        assert!(!test_path.exists());

        let result = ensure_directory_exists(&test_path);
        assert!(result.is_ok());
        assert!(test_path.exists());
        assert!(test_path.is_dir());
    }

    #[test]
    fn test_ensure_directory_exists_file_collision() {
        let (mut file, path) = create_temp_file("test_dir").unwrap();
        file.write_all(b"test").unwrap();
        file.flush().unwrap();

        let result = ensure_directory_exists(&path);
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidPath(_))));
    }

    // === create_temp_file tests ===

    #[test]
    fn test_create_temp_file_success() {
        let (file, path) = create_temp_file("test").unwrap();
        assert!(path.exists());
        assert!(path.is_file());
        drop(file);
        // File persists after dropping (using keep())
        assert!(path.exists());
        // Clean up
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_create_temp_file_writable() {
        let (mut file, path) = create_temp_file("test_write").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        file.flush().unwrap();

        let size = get_file_size(&path).unwrap();
        assert_eq!(size, 13);
    }

    // === atomic_write tests ===

    #[test]
    fn test_atomic_write_new_file() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test.txt");

        assert!(!test_path.exists());
        let result = atomic_write(&test_path, b"Hello, world!");
        assert!(result.is_ok());
        assert!(test_path.exists());

        let content = std::fs::read(&test_path).unwrap();
        assert_eq!(content, b"Hello, world!");
    }

    #[test]
    fn test_atomic_write_overwrite() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test.txt");

        atomic_write(&test_path, b"Original").unwrap();
        atomic_write(&test_path, b"Updated").unwrap();

        let content = std::fs::read(&test_path).unwrap();
        assert_eq!(content, b"Updated");
    }

    #[test]
    fn test_atomic_write_creates_parent_dirs() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let test_path = temp_dir.path().join("parent/child/test.txt");

        assert!(!test_path.exists());
        let result = atomic_write(&test_path, b"Nested file");
        assert!(result.is_ok());
        assert!(test_path.exists());

        let content = std::fs::read(&test_path).unwrap();
        assert_eq!(content, b"Nested file");
    }

    #[test]
    fn test_atomic_write_empty_data() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let test_path = temp_dir.path().join("empty.txt");

        atomic_write(&test_path, b"").unwrap();

        let content = std::fs::read(&test_path).unwrap();
        assert_eq!(content, b"");
        assert_eq!(get_file_size(&test_path).unwrap(), 0);
    }

    #[test]
    fn test_atomic_write_large_data() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let test_path = temp_dir.path().join("large.bin");

        let large_data = vec![0u8; 1024 * 1024];
        atomic_write(&test_path, &large_data).unwrap();

        let content = std::fs::read(&test_path).unwrap();
        assert_eq!(content.len(), 1024 * 1024);
    }

    // === get_local_path tests ===

    #[test]
    fn test_get_local_path_structure() {
        let file_id = FileId::new(123, 456);
        let path = get_local_path(file_id);

        assert!(path.to_string_lossy().contains("rustgram"));
        assert!(path.to_string_lossy().contains("123"));
        assert!(path.to_string_lossy().contains("456"));
    }

    #[test]
    fn test_get_local_path_unique_ids() {
        let file_id1 = FileId::new(123, 456);
        let file_id2 = FileId::new(789, 012);

        let path1 = get_local_path(file_id1);
        let path2 = get_local_path(file_id2);

        assert_ne!(path1, path2);
    }

    // === get_cache_path tests ===

    #[test]
    fn test_get_cache_path_structure() {
        let file_id = FileId::new(123, 456);
        let path = get_cache_path(file_id);

        assert!(path.to_string_lossy().contains("rustgram"));
        assert!(path.to_string_lossy().contains("files"));
        assert!(path.to_string_lossy().contains("123"));
        assert!(path.to_string_lossy().contains("456"));
    }

    #[test]
    fn test_get_cache_path_vs_local_path() {
        let file_id = FileId::new(123, 456);
        let cache_path = get_cache_path(file_id);
        let local_path = get_local_path(file_id);

        assert_ne!(cache_path, local_path);
        assert!(cache_path.to_string_lossy().contains("files"));
    }

    // === Error tests ===

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", Error::InvalidSize(100)),
            "invalid file size: 100"
        );
        assert_eq!(
            format!("{}", Error::InvalidPath("test".to_string())),
            "invalid file path: test"
        );
        assert_eq!(
            format!("{}", Error::IoError("error".to_string())),
            "I/O error: error"
        );
        assert_eq!(
            format!("{}", Error::InvalidUrl("url".to_string())),
            "invalid URL: url"
        );
    }

    #[test]
    fn test_error_eq() {
        assert_eq!(Error::InvalidSize(100), Error::InvalidSize(100));
        assert_ne!(Error::InvalidSize(100), Error::InvalidSize(200));
    }
}
