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

//! # File Uploader
//!
//! Manages file uploads to Telegram servers.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `FileUploader` class from `td/telegram/files/FileUploader.h`.
//!
//! ## Overview
//!
//! The FileUploader handles uploading files to Telegram servers with support for:
//!
//! - **Chunked uploads**: Files are uploaded in parts
//! - **Encryption**: Supports encrypted file uploads
//! - **Resumable uploads**: Partial uploads can be resumed
//! - **Resource management**: Integrates with ResourceManager for bandwidth control
//!
//! ## Usage
//!
//! ```rust,no_run
//! use rustgram_file_uploader::{FileUploader, FileUploaderConfig};
//! use rustgram_file_type::FileType;
//! use rustgram_file_id::FileId;
//!
//! let config = FileUploaderConfig::new(
//!     "/path/to/file.jpg",
//!     FileType::Photo,
//!     FileId::new(123, 0)
//! );
//!
//! let mut uploader = FileUploader::new(config)?;
//! uploader.start()?;
//!
//! while !uploader.is_complete() {
//!     match uploader.get_next_part()? {
//!         Some(part) => {
//!             // Upload the part...
//!             uploader.on_part_ok(part.id, part.size)?;
//!         }
//!         None => break,
//!     }
//! }
//! # Ok::<(), rustgram_file_uploader::Error>(())
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use rustgram_file_encryption_key::FileEncryptionKey;
use rustgram_file_id::FileId;
use rustgram_file_upload_id::FileUploadId;
use rustgram_file_type::FileType;
use rustgram_parts_manager::{PartsManager, Part};
use rustgram_resource_manager::{ResourceManager, ResourceType};
use std::fmt;
use std::path::PathBuf;

pub use callback::FileUploaderCallback;
pub use config::FileUploaderConfig;
pub use error::Error;
pub use state::UploadState;

mod callback;
mod config;
mod error;
mod state;

/// Result type for file uploader operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Priority for file uploads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UploadPriority {
    /// Low priority
    Low = 0,
    /// Normal priority (default)
    #[default]
    Normal = 1,
    /// High priority
    High = 2,
}

impl UploadPriority {
    /// Returns the numeric value of this priority.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }
}

/// File uploader for managing file uploads to Telegram servers.
///
/// Handles chunked uploads, encryption, and resource management.
#[derive(Debug)]
pub struct FileUploader {
    /// File path
    path: PathBuf,
    /// File type
    file_type: FileType,
    /// File ID
    file_id: FileId,
    /// Upload ID
    upload_id: FileUploadId,
    /// File size
    size: i64,
    /// Expected file size (passed to parts manager during init)
    _expected_size: i64,
    /// Encryption key
    encryption_key: FileEncryptionKey,
    /// Whether this is a small file
    is_small: bool,
    /// Upload offset
    offset: i64,
    /// Upload limit
    limit: i64,
    /// Current state
    state: UploadState,
    /// Parts manager
    parts_manager: PartsManager,
    /// Upload priority
    priority: UploadPriority,
    /// Need check flag
    need_check: bool,
    /// Stop flag
    stop_flag: bool,
    /// Whether to use CDN
    use_cdn: bool,
}

impl FileUploader {
    /// Creates a new file uploader.
    ///
    /// # Arguments
    ///
    /// * `config` - The uploader configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn new(config: FileUploaderConfig) -> Result<Self> {
        let part_size = if config.size > 2_000_000 {
            512 * 1024 // Large files use 512KB parts
        } else {
            32 * 1024 // Small files use 32KB parts
        };

        let mut parts_manager = PartsManager::new();
        parts_manager.init(
            rustgram_parts_manager::InitOptions {
                size: config.size,
                expected_size: config.expected_size,
                is_size_final: config.size_is_final,
                part_size,
                use_part_count_limit: true,
                is_upload: true,
            },
            &config.ready_parts,
        )?;

        let file_id = config.file_id.unwrap_or_else(|| FileId::new(0, 0));
        let upload_id = FileUploadId::new(file_id, 0);

        Ok(Self {
            path: PathBuf::from(config.path),
            file_type: config.file_type,
            file_id,
            upload_id,
            size: config.size,
            _expected_size: config.expected_size,
            encryption_key: config.encryption_key,
            is_small: config.size <= 2_000_000,
            offset: 0,
            limit: 0,
            state: UploadState::Idle,
            parts_manager,
            priority: UploadPriority::default(),
            need_check: false,
            stop_flag: false,
            use_cdn: false,
        })
    }

    /// Starts the upload.
    ///
    /// # Errors
    ///
    /// Returns an error if the upload cannot be started.
    pub fn start(&mut self) -> Result<()> {
        if self.state != UploadState::Idle {
            return Err(Error::InvalidState);
        }

        self.state = UploadState::Active;
        Ok(())
    }

    /// Stops the upload.
    pub fn stop(&mut self) {
        self.stop_flag = true;
        self.state = UploadState::Stopped;
    }

    /// Returns `true` if the upload is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.parts_manager.ready()
    }

    /// Returns the current upload state.
    #[must_use]
    pub const fn state(&self) -> UploadState {
        self.state
    }

    /// Returns the file size.
    #[must_use]
    pub const fn size(&self) -> i64 {
        self.size
    }

    /// Returns the file path.
    #[must_use]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns the file type.
    #[must_use]
    pub const fn file_type(&self) -> FileType {
        self.file_type
    }

    /// Returns the file ID.
    #[must_use]
    pub const fn file_id(&self) -> FileId {
        self.file_id
    }

    /// Returns the upload ID.
    #[must_use]
    pub const fn upload_id(&self) -> FileUploadId {
        self.upload_id
    }

    /// Returns the encryption key.
    #[must_use]
    pub fn encryption_key(&self) -> &FileEncryptionKey {
        &self.encryption_key
    }

    /// Returns the upload priority.
    #[must_use]
    pub const fn priority(&self) -> UploadPriority {
        self.priority
    }

    /// Sets the upload priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - The upload priority
    pub fn set_priority(&mut self, priority: UploadPriority) {
        self.priority = priority;
    }

    /// Returns `true` if this is a small file.
    #[must_use]
    pub const fn is_small(&self) -> bool {
        self.is_small
    }

    /// Gets the next part to upload.
    ///
    /// # Errors
    ///
    /// Returns an error if the upload is not active.
    pub fn get_next_part(&mut self) -> Result<Part> {
        if self.state != UploadState::Active {
            return Err(Error::NotActive);
        }

        let part = self.parts_manager.start_part()?;
        if part.is_empty() {
            self.state = UploadState::Complete;
        }
        Ok(part)
    }

    /// Marks a part as successfully uploaded.
    ///
    /// # Arguments
    ///
    /// * `part_id` - The part ID that was uploaded
    /// * `part_size` - The size of the part
    /// * `actual_size` - The actual size of the uploaded data
    ///
    /// # Errors
    ///
    /// Returns an error if the part ID is invalid.
    pub fn on_part_ok(&mut self, part_id: i32, part_size: usize, actual_size: usize) -> Result<()> {
        self.parts_manager.on_part_ok(part_id, part_size, actual_size)?;

        if self.parts_manager.ready() {
            self.state = UploadState::Complete;
        }

        Ok(())
    }

    /// Marks a part as failed.
    ///
    /// # Arguments
    ///
    /// * `part_id` - The part ID that failed
    pub fn on_part_failed(&mut self, part_id: i32) {
        self.parts_manager.on_part_failed(part_id);
    }

    /// Gets the upload progress as a percentage.
    #[must_use]
    pub fn progress(&self) -> f64 {
        if self.size == 0 {
            return 0.0;
        }

        let ready_size = self.parts_manager.get_ready_size();
        (ready_size as f64 / self.size as f64) * 100.0
    }

    /// Gets the number of uploaded bytes.
    #[must_use]
    pub fn uploaded_bytes(&self) -> i64 {
        self.parts_manager.get_ready_size()
    }

    /// Gets the number of remaining parts.
    #[must_use]
    pub fn remaining_parts(&self) -> i32 {
        self.parts_manager.get_part_count() - self.parts_manager.get_ready_prefix_count()
    }

    /// Updates the uploaded part range.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset in bytes
    /// * `limit` - The limit in bytes
    pub fn update_uploaded_part(&mut self, offset: i64, limit: i64) {
        self.offset = offset;
        self.limit = limit;
        self.parts_manager.set_streaming_offset(offset, limit.max(0));
    }

    /// Returns `true` if the upload needs checking.
    #[must_use]
    pub const fn need_check(&self) -> bool {
        self.need_check
    }

    /// Sets that the upload needs checking.
    pub fn set_need_check(&mut self) {
        self.need_check = true;
        self.parts_manager.set_need_check();
    }

    /// Gets the bitmask of uploaded parts.
    #[must_use]
    pub fn get_bitmask(&self) -> String {
        format!(
            "{}/{} parts",
            self.parts_manager.get_ready_prefix_count(),
            self.parts_manager.get_part_count()
        )
    }

    /// Checks if resources are available for upload.
    ///
    /// # Arguments
    ///
    /// * `resource_manager` - The resource manager to use
    ///
    /// # Errors
    ///
    /// Returns an error if the check fails.
    pub fn check_resource(&self, resource_manager: &ResourceManager) -> Result<bool> {
        Ok(resource_manager.can_start(ResourceType::Upload))
    }

    /// Enables CDN uploads.
    pub fn enable_cdn(&mut self) {
        self.use_cdn = true;
    }

    /// Returns `true` if CDN is being used.
    #[must_use]
    pub const fn use_cdn(&self) -> bool {
        self.use_cdn
    }

    /// Gets the upload offset.
    #[must_use]
    pub const fn offset(&self) -> i64 {
        self.offset
    }

    /// Gets the upload limit.
    #[must_use]
    pub const fn limit(&self) -> i64 {
        self.limit
    }
}

impl fmt::Display for FileUploader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FileUploader(path={:?}, size={}, progress={:.1}%, state={:?})",
            self.path,
            self.size,
            self.progress(),
            self.state
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000)
            .with_expected_size(10_000_000)
            .with_size_final(true);

        let uploader = FileUploader::new(config);
        assert!(uploader.is_ok());

        let uploader = uploader.unwrap();
        assert_eq!(uploader.size(), 10_000_000);
        assert!(uploader.is_small());
        assert_eq!(uploader.file_type(), FileType::Photo);
    }

    #[test]
    fn test_start() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let mut uploader = FileUploader::new(config).unwrap();
        assert!(uploader.start().is_ok());
        assert_eq!(uploader.state(), UploadState::Active);
    }

    #[test]
    fn test_start_already_active() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let mut uploader = FileUploader::new(config).unwrap();
        uploader.start().unwrap();

        let result = uploader.start();
        assert!(matches!(result, Err(Error::InvalidState)));
    }

    #[test]
    fn test_stop() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let mut uploader = FileUploader::new(config).unwrap();
        uploader.start().unwrap();
        uploader.stop();

        assert_eq!(uploader.state(), UploadState::Stopped);
    }

    #[test]
    fn test_is_complete_initially() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let uploader = FileUploader::new(config).unwrap();
        assert!(!uploader.is_complete());
    }

    #[test]
    fn test_progress_initially() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let uploader = FileUploader::new(config).unwrap();
        assert_eq!(uploader.progress(), 0.0);
    }

    #[test]
    fn test_uploaded_bytes_initially() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let uploader = FileUploader::new(config).unwrap();
        assert_eq!(uploader.uploaded_bytes(), 0);
    }

    #[test]
    fn test_remaining_parts_initially() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let uploader = FileUploader::new(config).unwrap();
        assert!(uploader.remaining_parts() > 0);
    }

    #[test]
    fn test_set_priority() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let mut uploader = FileUploader::new(config).unwrap();
        uploader.set_priority(UploadPriority::High);

        assert_eq!(uploader.priority(), UploadPriority::High);
    }

    #[test]
    fn test_enable_cdn() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let mut uploader = FileUploader::new(config).unwrap();
        uploader.enable_cdn();

        assert!(uploader.use_cdn());
    }

    #[test]
    fn test_set_need_check() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let mut uploader = FileUploader::new(config).unwrap();
        uploader.set_need_check();

        assert!(uploader.need_check());
    }

    #[test]
    fn test_get_bitmask() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let uploader = FileUploader::new(config).unwrap();
        let bitmask = uploader.get_bitmask();

        assert!(bitmask.contains("parts"));
    }

    #[test]
    fn test_large_file() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(5_000_000);

        let uploader = FileUploader::new(config).unwrap();
        assert!(!uploader.is_small());
    }

    #[test]
    fn test_display() {
        let config = FileUploaderConfig::new("/path/to/file.jpg", FileType::Photo, None)
            .with_size(10_000_000);

        let uploader = FileUploader::new(config).unwrap();
        let s = format!("{uploader}");

        assert!(s.contains("FileUploader"));
        assert!(s.contains("/path/to/file.jpg"));
        assert!(s.contains("10000000"));
    }

    #[test]
    fn test_upload_priority_value() {
        assert_eq!(UploadPriority::Low.value(), 0);
        assert_eq!(UploadPriority::Normal.value(), 1);
        assert_eq!(UploadPriority::High.value(), 2);
    }
}
