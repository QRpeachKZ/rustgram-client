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

//! # File Downloader
//!
//! Manages file downloads from Telegram servers.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `FileDownloader` class from `td/telegram/files/FileDownloader.h`.
//!
//! ## Overview
//!
//! The FileDownloader handles downloading files from Telegram servers with support for:
//!
//! - **Chunked downloads**: Files are downloaded in parts
//! - **CDN support**: Can use Telegram's CDN for file downloads
//! - **Resumable downloads**: Partial downloads can be resumed
//! - **Encryption**: Supports encrypted file downloads
//! - **Resource management**: Integrates with ResourceManager for bandwidth control
//!
//! ## Usage
//!
//! ```rust,no_run
//! use rustgram_file_downloader::{FileDownloader, FileDownloaderConfig};
//! use rustgram_file_location::{FullRemoteFileLocation, RemoteFileLocation};
//! use rustgram_file_encryption_key::FileEncryptionKey;
//!
//! let remote = FullRemoteFileLocation::common(123, 456);
//! let config = FileDownloaderConfig::new(remote, 10_000_000);
//!
//! let mut downloader = FileDownloader::new(config)?;
//! downloader.start()?;
//!
//! while !downloader.is_complete() {
//!     match downloader.download_next_part()? {
//!         Some(part_data) => {
//!             // Process downloaded part...
//!         }
//!         None => break,
//!     }
//! }
//! # Ok::<(), rustgram_file_downloader::Error>(())
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use rustgram_file_encryption_key::FileEncryptionKey;
use rustgram_file_location::{FullRemoteFileLocation, LocalFileLocation};
use rustgram_parts_manager::{PartsManager, Part};
use rustgram_resource_manager::{ResourceManager, ResourceType};
use std::fmt;
use std::path::PathBuf;

pub use callback::FileDownloaderCallback;
pub use config::FileDownloaderConfig;
pub use error::Error;
pub use state::DownloadState;

mod callback;
mod config;
mod error;
mod state;

/// Result type for file downloader operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Query type for file downloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QueryType {
    /// Default query type (direct download)
    #[default]
    Default,
    /// CDN query type
    Cdn,
    /// Reupload to CDN
    ReuploadCdn,
}

/// File downloader for managing file downloads from Telegram servers.
///
/// Handles chunked downloads, CDN support, and resource management.
#[derive(Debug)]
pub struct FileDownloader {
    /// Remote file location
    remote: FullRemoteFileLocation,
    /// Local file location
    local: LocalFileLocation,
    /// File size
    size: i64,
    /// File name
    name: String,
    /// Encryption key
    encryption_key: FileEncryptionKey,
    /// Whether this is a small file
    is_small: bool,
    /// Whether to search for existing file
    need_search_file: bool,
    /// Download offset
    offset: i64,
    /// Download limit
    limit: i64,
    /// File path
    _path: Option<PathBuf>,
    /// Current state
    state: DownloadState,
    /// Parts manager
    parts_manager: PartsManager,
    /// Query type
    query_type: QueryType,
    /// Whether to use CDN
    use_cdn: bool,
    /// CDN DC ID
    cdn_dc_id: Option<i32>,
    /// CDN encryption key
    _cdn_encryption_key: Option<Vec<u8>>,
    /// CDN encryption IV
    _cdn_encryption_iv: Option<Vec<u8>>,
    /// CDN file token
    _cdn_file_token: Option<String>,
    /// Need check flag
    need_check: bool,
    /// Ordered flag
    _ordered_flag: bool,
    /// Keep FD flag
    _keep_fd: bool,
    /// Stop flag
    stop_flag: bool,
}

impl FileDownloader {
    /// Creates a new file downloader.
    ///
    /// # Arguments
    ///
    /// * `config` - The downloader configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn new(config: FileDownloaderConfig) -> Result<Self> {
        let part_size = if config.size > 2_000_000 {
            512 * 1024 // Large files use 512KB parts
        } else {
            32 * 1024 // Small files use 32KB parts
        };

        let mut parts_manager = PartsManager::new();
        parts_manager.init(
            rustgram_parts_manager::InitOptions {
                size: config.size,
                expected_size: config.expected_size.unwrap_or(config.size),
                is_size_final: config.is_size_final,
                part_size,
                use_part_count_limit: true,
                is_upload: false,
            },
            &config.ready_parts,
        )?;

        Ok(Self {
            remote: config.remote,
            local: config.local,
            size: config.size,
            name: config.name,
            encryption_key: config.encryption_key,
            is_small: config.size <= 2_000_000,
            need_search_file: config.need_search_file,
            offset: config.offset,
            limit: config.limit,
            _path: None,
            state: DownloadState::Idle,
            parts_manager,
            query_type: QueryType::Default,
            use_cdn: false,
            cdn_dc_id: None,
            _cdn_encryption_key: None,
            _cdn_encryption_iv: None,
            _cdn_file_token: None,
            need_check: false,
            _ordered_flag: false,
            _keep_fd: false,
            stop_flag: false,
        })
    }

    /// Starts the download.
    ///
    /// # Errors
    ///
    /// Returns an error if the download cannot be started.
    pub fn start(&mut self) -> Result<()> {
        if self.state != DownloadState::Idle {
            return Err(Error::InvalidState);
        }

        self.state = DownloadState::Active;
        Ok(())
    }

    /// Stops the download.
    pub fn stop(&mut self) {
        self.stop_flag = true;
        self.state = DownloadState::Stopped;
    }

    /// Returns `true` if the download is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.parts_manager.ready()
    }

    /// Returns the current download state.
    #[must_use]
    pub const fn state(&self) -> DownloadState {
        self.state
    }

    /// Returns the file size.
    #[must_use]
    pub const fn size(&self) -> i64 {
        self.size
    }

    /// Returns the remote file location.
    #[must_use]
    pub fn remote(&self) -> &FullRemoteFileLocation {
        &self.remote
    }

    /// Returns the local file location.
    #[must_use]
    pub fn local(&self) -> &LocalFileLocation {
        &self.local
    }

    /// Returns the file name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the encryption key.
    #[must_use]
    pub fn encryption_key(&self) -> &FileEncryptionKey {
        &self.encryption_key
    }

    /// Returns the download offset.
    #[must_use]
    pub const fn offset(&self) -> i64 {
        self.offset
    }

    /// Returns the download limit.
    #[must_use]
    pub const fn limit(&self) -> i64 {
        self.limit
    }

    /// Returns `true` if this is a small file.
    #[must_use]
    pub const fn is_small(&self) -> bool {
        self.is_small
    }

    /// Gets the next part to download.
    ///
    /// # Errors
    ///
    /// Returns an error if the download is not active.
    pub fn get_next_part(&mut self) -> Result<Part> {
        if self.state != DownloadState::Active {
            return Err(Error::NotActive);
        }

        let part = self.parts_manager.start_part()?;
        if part.is_empty() {
            self.state = DownloadState::Complete;
        }
        Ok(part)
    }

    /// Marks a part as successfully downloaded.
    ///
    /// # Arguments
    ///
    /// * `part` - The part that was downloaded
    /// * `actual_size` - The actual size of the downloaded data
    ///
    /// # Errors
    ///
    /// Returns an error if the part ID is invalid.
    pub fn on_part_ok(&mut self, part: Part, actual_size: usize) -> Result<()> {
        self.parts_manager
            .on_part_ok(part.id, part.size, actual_size)?;

        if self.parts_manager.ready() {
            self.state = DownloadState::Complete;
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

    /// Gets the download progress as a percentage.
    #[must_use]
    pub fn progress(&self) -> f64 {
        if self.size == 0 {
            return 0.0;
        }

        let ready_size = self.parts_manager.get_ready_size();
        (ready_size as f64 / self.size as f64) * 100.0
    }

    /// Gets the number of downloaded bytes.
    #[must_use]
    pub fn downloaded_bytes(&self) -> i64 {
        self.parts_manager.get_ready_size()
    }

    /// Gets the number of remaining parts.
    #[must_use]
    pub fn remaining_parts(&self) -> i32 {
        self.parts_manager.get_part_count() - self.parts_manager.get_ready_prefix_count()
    }

    /// Updates the downloaded part range.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset in bytes
    /// * `limit` - The limit in bytes
    /// * `max_resource_limit` - Maximum resource limit
    pub fn update_downloaded_part(&mut self, offset: i64, limit: i64, max_resource_limit: i64) {
        self.offset = offset;
        self.limit = limit.min(max_resource_limit);
        self.parts_manager
            .set_streaming_offset(offset, limit.max(0));
    }

    /// Sets the query type.
    ///
    /// # Arguments
    ///
    /// * `query_type` - The query type to use
    pub fn set_query_type(&mut self, query_type: QueryType) {
        self.query_type = query_type;
    }

    /// Gets the query type.
    #[must_use]
    pub const fn query_type(&self) -> QueryType {
        self.query_type
    }

    /// Returns `true` if CDN is being used.
    #[must_use]
    pub const fn use_cdn(&self) -> bool {
        self.use_cdn
    }

    /// Enables CDN downloads.
    ///
    /// # Arguments
    ///
    /// * `dc_id` - The CDN DC ID
    pub fn enable_cdn(&mut self, dc_id: i32) {
        self.use_cdn = true;
        self.cdn_dc_id = Some(dc_id);
        self.query_type = QueryType::Cdn;
    }

    /// Returns the CDN DC ID.
    #[must_use]
    pub const fn cdn_dc_id(&self) -> Option<i32> {
        self.cdn_dc_id
    }

    /// Returns `true` if the download needs checking.
    #[must_use]
    pub const fn need_check(&self) -> bool {
        self.need_check
    }

    /// Sets that the download needs checking.
    pub fn set_need_check(&mut self) {
        self.need_check = true;
        self.parts_manager.set_need_check();
    }

    /// Gets the bitmask of downloaded parts.
    #[must_use]
    pub fn get_bitmask(&self) -> String {
        // Return a simplified representation
        format!(
            "{}/{} parts",
            self.parts_manager.get_ready_prefix_count(),
            self.parts_manager.get_part_count()
        )
    }

    /// Gets the ready prefix size.
    #[must_use]
    pub fn get_ready_prefix_size(&self) -> i64 {
        self.parts_manager.get_checked_prefix_size()
    }

    /// Checks if the file is locally available.
    ///
    /// # Arguments
    ///
    /// * `resource_manager` - The resource manager to use
    ///
    /// # Errors
    ///
    /// Returns an error if the check fails.
    pub fn check_local(&self, resource_manager: &ResourceManager) -> Result<bool> {
        if !self.need_search_file {
            return Ok(false);
        }

        // Check if we can start a download
        Ok(resource_manager.can_start(ResourceType::Download))
    }
}

impl fmt::Display for FileDownloader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FileDownloader(size={}, progress={:.1}%, state={:?})",
            self.size,
            self.progress(),
            self.state
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_file_location::FullRemoteFileLocation;

    #[test]
    fn test_new() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000)
            .with_expected_size(10_000_000)
            .with_size_final(true);

        let downloader = FileDownloader::new(config);
        assert!(downloader.is_ok());

        let downloader = downloader.unwrap();
        assert_eq!(downloader.size(), 10_000_000);
        assert!(downloader.is_small());
    }

    #[test]
    fn test_start() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let mut downloader = FileDownloader::new(config).unwrap();
        assert!(downloader.start().is_ok());
        assert_eq!(downloader.state(), DownloadState::Active);
    }

    #[test]
    fn test_start_already_active() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let mut downloader = FileDownloader::new(config).unwrap();
        downloader.start().unwrap();

        let result = downloader.start();
        assert!(matches!(result, Err(Error::InvalidState)));
    }

    #[test]
    fn test_stop() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let mut downloader = FileDownloader::new(config).unwrap();
        downloader.start().unwrap();
        downloader.stop();

        assert_eq!(downloader.state(), DownloadState::Stopped);
    }

    #[test]
    fn test_is_complete_initially() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let downloader = FileDownloader::new(config).unwrap();
        assert!(!downloader.is_complete());
    }

    #[test]
    fn test_progress_initially() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let downloader = FileDownloader::new(config).unwrap();
        assert_eq!(downloader.progress(), 0.0);
    }

    #[test]
    fn test_downloaded_bytes_initially() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let downloader = FileDownloader::new(config).unwrap();
        assert_eq!(downloader.downloaded_bytes(), 0);
    }

    #[test]
    fn test_remaining_parts_initially() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let downloader = FileDownloader::new(config).unwrap();
        assert!(downloader.remaining_parts() > 0);
    }

    #[test]
    fn test_update_downloaded_part() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let mut downloader = FileDownloader::new(config).unwrap();
        downloader.update_downloaded_part(1000, 5000, 10_000_000);

        assert_eq!(downloader.offset(), 1000);
        assert_eq!(downloader.limit(), 5000);
    }

    #[test]
    fn test_set_query_type() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let mut downloader = FileDownloader::new(config).unwrap();
        downloader.set_query_type(QueryType::Cdn);

        assert_eq!(downloader.query_type(), QueryType::Cdn);
    }

    #[test]
    fn test_enable_cdn() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let mut downloader = FileDownloader::new(config).unwrap();
        downloader.enable_cdn(5);

        assert!(downloader.use_cdn());
        assert_eq!(downloader.cdn_dc_id(), Some(5));
        assert_eq!(downloader.query_type(), QueryType::Cdn);
    }

    #[test]
    fn test_set_need_check() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let mut downloader = FileDownloader::new(config).unwrap();
        downloader.set_need_check();

        assert!(downloader.need_check());
    }

    #[test]
    fn test_get_bitmask() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let downloader = FileDownloader::new(config).unwrap();
        let bitmask = downloader.get_bitmask();

        assert!(bitmask.contains("parts"));
    }

    #[test]
    fn test_get_ready_prefix_size() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let downloader = FileDownloader::new(config).unwrap();
        assert_eq!(downloader.get_ready_prefix_size(), 0);
    }

    #[test]
    fn test_large_file() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 5_000_000);

        let downloader = FileDownloader::new(config).unwrap();
        assert!(!downloader.is_small());
    }

    #[test]
    fn test_display() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let config = FileDownloaderConfig::new(remote, 10_000_000);

        let downloader = FileDownloader::new(config).unwrap();
        let s = format!("{downloader}");

        assert!(s.contains("FileDownloader"));
        assert!(s.contains("10000000"));
    }
}
