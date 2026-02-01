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

//! Download information tracking.

use crate::error::{DownloadStateInternal, Result};
use rustgram_file_id::FileId;
use rustgram_file_location::{FullRemoteFileLocation, LocalFileLocation};
use rustgram_file_source_id::FileSourceId;
use rustgram_resource_manager::ResourcePriority;
use std::time::{SystemTime, UNIX_EPOCH};

/// Information about a download in the manager.
#[derive(Debug, Clone)]
pub struct DownloadInfo {
    /// Unique download ID.
    download_id: u64,
    /// File ID being downloaded.
    file_id: FileId,
    /// File source ID.
    file_source_id: FileSourceId,
    /// Remote file location.
    remote: FullRemoteFileLocation,
    /// Local file location.
    local: LocalFileLocation,
    /// File size in bytes.
    size: i64,
    /// Downloaded bytes.
    downloaded_size: i64,
    /// Expected file size (may be updated during download).
    expected_size: i64,
    /// Download priority.
    priority: ResourcePriority,
    /// Current download state.
    state: DownloadStateInternal,
    /// Number of retry attempts.
    retry_count: u32,
    /// Timestamp when download was created.
    created_at: u64,
    /// Timestamp when download was completed.
    completed_at: Option<u64>,
    /// Error message if download failed.
    error_message: Option<String>,
}

impl DownloadInfo {
    /// Creates a new download info.
    ///
    /// # Arguments
    ///
    /// * `download_id` - Unique download identifier
    /// * `file_id` - File ID
    /// * `file_source_id` - File source ID
    /// * `remote` - Remote file location
    /// * `local` - Local file location
    /// * `size` - File size
    /// * `priority` - Download priority
    pub fn new(
        download_id: u64,
        file_id: FileId,
        file_source_id: FileSourceId,
        remote: FullRemoteFileLocation,
        local: LocalFileLocation,
        size: i64,
        priority: ResourcePriority,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            download_id,
            file_id,
            file_source_id,
            remote,
            local,
            size,
            downloaded_size: 0,
            expected_size: size,
            priority,
            state: DownloadStateInternal::Pending,
            retry_count: 0,
            created_at: now,
            completed_at: None,
            error_message: None,
        }
    }

    /// Returns the download ID.
    #[must_use]
    pub const fn download_id(&self) -> u64 {
        self.download_id
    }

    /// Returns the file ID.
    #[must_use]
    pub const fn file_id(&self) -> FileId {
        self.file_id
    }

    /// Returns the file source ID.
    #[must_use]
    pub const fn file_source_id(&self) -> FileSourceId {
        self.file_source_id
    }

    /// Returns the remote file location.
    #[must_use]
    pub const fn remote(&self) -> &FullRemoteFileLocation {
        &self.remote
    }

    /// Returns the local file location.
    #[must_use]
    pub const fn local(&self) -> &LocalFileLocation {
        &self.local
    }

    /// Returns the file size.
    #[must_use]
    pub const fn size(&self) -> i64 {
        self.size
    }

    /// Returns the downloaded size.
    #[must_use]
    pub const fn downloaded_size(&self) -> i64 {
        self.downloaded_size
    }

    /// Returns the expected size.
    #[must_use]
    pub const fn expected_size(&self) -> i64 {
        self.expected_size
    }

    /// Returns the download priority.
    #[must_use]
    pub const fn priority(&self) -> ResourcePriority {
        self.priority
    }

    /// Returns the current state.
    #[must_use]
    pub const fn state(&self) -> DownloadStateInternal {
        self.state
    }

    /// Returns the retry count.
    #[must_use]
    pub const fn retry_count(&self) -> u32 {
        self.retry_count
    }

    /// Returns the creation timestamp.
    #[must_use]
    pub const fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Returns the completion timestamp if completed.
    #[must_use]
    pub const fn completed_at(&self) -> Option<u64> {
        self.completed_at
    }

    /// Returns the error message if failed.
    #[must_use]
    pub const fn error_message(&self) -> Option<&String> {
        self.error_message.as_ref()
    }

    /// Returns `true` if the download is pending.
    #[must_use]
    pub const fn is_pending(&self) -> bool {
        matches!(self.state, DownloadStateInternal::Pending)
    }

    /// Returns `true` if the download is active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self.state, DownloadStateInternal::Active)
    }

    /// Returns `true` if the download is paused.
    #[must_use]
    pub const fn is_paused(&self) -> bool {
        matches!(self.state, DownloadStateInternal::Paused)
    }

    /// Returns `true` if the download is completed.
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        matches!(self.state, DownloadStateInternal::Completed)
    }

    /// Returns `true` if the download is failed.
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        matches!(self.state, DownloadStateInternal::Failed)
    }

    /// Returns `true` if the download is cancelled.
    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        matches!(self.state, DownloadStateInternal::Cancelled)
    }

    /// Returns the download progress as a percentage (0.0 to 100.0).
    #[must_use]
    pub fn progress(&self) -> f64 {
        if self.expected_size <= 0 {
            0.0
        } else {
            (self.downloaded_size as f64 / self.expected_size as f64) * 100.0
        }
    }

    /// Returns `true` if the download can be retried.
    #[must_use]
    pub fn can_retry(&self, max_retries: u32) -> bool {
        self.is_failed() && self.retry_count < max_retries
    }

    /// Updates the downloaded size.
    ///
    /// # Arguments
    ///
    /// * `size` - New downloaded size
    pub fn update_downloaded_size(&mut self, size: i64) {
        self.downloaded_size = size.min(self.expected_size);
    }

    /// Updates the expected size.
    ///
    /// # Arguments
    ///
    /// * `size` - New expected size
    pub fn update_expected_size(&mut self, size: i64) {
        if size > 0 {
            self.expected_size = size;
        }
    }

    /// Transitions to a new state.
    ///
    /// # Arguments
    ///
    /// * `new_state` - The new state
    ///
    /// # Errors
    ///
    /// Returns an error if the transition is invalid.
    pub fn transition_to(&mut self, new_state: DownloadStateInternal) -> Result<()> {
        if !self.state.can_transition_to(new_state) {
            return Err(crate::error::Error::InvalidStateTransition(
                self.state,
                new_state,
            ));
        }

        self.state = new_state;

        if matches!(
            new_state,
            DownloadStateInternal::Completed
                | DownloadStateInternal::Failed
                | DownloadStateInternal::Cancelled
        ) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            self.completed_at = Some(now);
        }

        Ok(())
    }

    /// Increments the retry count.
    pub fn increment_retry(&mut self) {
        self.retry_count = self.retry_count.saturating_add(1);
    }

    /// Sets the error message.
    ///
    /// # Arguments
    ///
    /// * `message` - Error message
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    /// Clears the error message.
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_download_info() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let local = LocalFileLocation::empty();
        let file_id = FileId::new(1, 0);
        let file_source_id = FileSourceId::new(1);

        let info = DownloadInfo::new(
            100,
            file_id,
            file_source_id,
            remote,
            local,
            10_000_000,
            ResourcePriority::Normal,
        );

        assert_eq!(info.download_id(), 100);
        assert!(info.is_pending());
        assert_eq!(info.downloaded_size(), 0);
        assert_eq!(info.progress(), 0.0);
    }

    #[test]
    fn test_state_transitions() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let local = LocalFileLocation::empty();
        let file_id = FileId::new(1, 0);
        let file_source_id = FileSourceId::new(1);

        let mut info = DownloadInfo::new(
            100,
            file_id,
            file_source_id,
            remote.clone(),
            local.clone(),
            10_000_000,
            ResourcePriority::Normal,
        );

        // Pending -> Active
        assert!(info.transition_to(DownloadStateInternal::Active).is_ok());
        assert!(info.is_active());

        // Active -> Paused
        assert!(info.transition_to(DownloadStateInternal::Paused).is_ok());
        assert!(info.is_paused());

        // Paused -> Active
        assert!(info.transition_to(DownloadStateInternal::Active).is_ok());
        assert!(info.is_active());

        // Active -> Completed
        assert!(info.transition_to(DownloadStateInternal::Completed).is_ok());
        assert!(info.is_completed());
        assert!(info.completed_at().is_some());
    }

    #[test]
    fn test_invalid_state_transitions() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let local = LocalFileLocation::empty();
        let file_id = FileId::new(1, 0);
        let file_source_id = FileSourceId::new(1);

        let mut info = DownloadInfo::new(
            100,
            file_id,
            file_source_id,
            remote,
            local,
            10_000_000,
            ResourcePriority::Normal,
        );

        info.state = DownloadStateInternal::Completed;
        assert!(info
            .transition_to(DownloadStateInternal::Active)
            .is_err());
    }

    #[test]
    fn test_update_downloaded_size() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let local = LocalFileLocation::empty();
        let file_id = FileId::new(1, 0);
        let file_source_id = FileSourceId::new(1);

        let mut info = DownloadInfo::new(
            100,
            file_id,
            file_source_id,
            remote,
            local,
            10_000_000,
            ResourcePriority::Normal,
        );

        info.update_downloaded_size(5_000_000);
        assert_eq!(info.downloaded_size(), 5_000_000);
        assert!((info.progress() - 50.0).abs() < 0.01);

        // Try to set beyond expected size
        info.update_downloaded_size(15_000_000);
        assert_eq!(info.downloaded_size(), 10_000_000);
    }

    #[test]
    fn test_update_expected_size() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let local = LocalFileLocation::empty();
        let file_id = FileId::new(1, 0);
        let file_source_id = FileSourceId::new(1);

        let mut info = DownloadInfo::new(
            100,
            file_id,
            file_source_id,
            remote,
            local,
            10_000_000,
            ResourcePriority::Normal,
        );

        info.update_expected_size(20_000_000);
        assert_eq!(info.expected_size(), 20_000_000);

        // Don't allow zero or negative
        info.update_expected_size(-1);
        assert_eq!(info.expected_size(), 20_000_000);
    }

    #[test]
    fn test_can_retry() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let local = LocalFileLocation::empty();
        let file_id = FileId::new(1, 0);
        let file_source_id = FileSourceId::new(1);

        let mut info = DownloadInfo::new(
            100,
            file_id,
            file_source_id,
            remote,
            local,
            10_000_000,
            ResourcePriority::Normal,
        );

        info.state = DownloadStateInternal::Failed;
        assert!(info.can_retry(3));

        info.increment_retry();
        assert!(info.can_retry(3));

        info.increment_retry();
        info.increment_retry();
        assert!(!info.can_retry(3));
    }

    #[test]
    fn test_error_handling() {
        let remote = FullRemoteFileLocation::common(123, 456);
        let local = LocalFileLocation::empty();
        let file_id = FileId::new(1, 0);
        let file_source_id = FileSourceId::new(1);

        let mut info = DownloadInfo::new(
            100,
            file_id,
            file_source_id,
            remote,
            local,
            10_000_000,
            ResourcePriority::Normal,
        );

        info.set_error("Network error".to_string());
        assert_eq!(info.error_message(), Some(&"Network error".to_string()));

        info.clear_error();
        assert!(info.error_message().is_none());
    }
}
