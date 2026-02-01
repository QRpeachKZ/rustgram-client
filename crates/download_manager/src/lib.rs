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

//! # Download Manager
//!
//! Manager for tracking file downloads.
//!
//! ## TDLib Alignment
//!
//! Simplified version of TDLib's `DownloadManager` that tracks
//! download progress and state for files.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_download_manager::{DownloadManager, DownloadState, DownloadError};
//! use rustgram_file_id::FileId;
//!
//! let mut manager = DownloadManager::new();
//!
//! let file_id = FileId::new(123, 0);
//! manager.start_download(file_id, 1000).unwrap();
//!
//! manager.update_progress(file_id, 500).unwrap();
//! assert_eq!(manager.get_progress(file_id).unwrap(), 500);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_file_id::FileId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub mod error;
pub use error::{DownloadError, Result};

/// Download state for a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadState {
    /// Download is active.
    Active,
    /// Download is paused.
    Paused,
    /// Download is completed.
    Completed,
}

/// Download information for a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadInfo {
    /// Total size in bytes.
    total_size: i64,
    /// Downloaded bytes.
    downloaded: i64,
    /// Current state.
    state: DownloadState,
}

impl DownloadInfo {
    /// Creates a new download info.
    pub fn new(total_size: i64) -> Self {
        Self {
            total_size,
            downloaded: 0,
            state: DownloadState::Active,
        }
    }

    /// Gets the total size.
    pub fn total_size(&self) -> i64 {
        self.total_size
    }

    /// Gets the downloaded bytes.
    pub fn downloaded(&self) -> i64 {
        self.downloaded
    }

    /// Gets the download state.
    pub fn state(&self) -> DownloadState {
        self.state
    }

    /// Checks if download is complete.
    pub fn is_complete(&self) -> bool {
        self.state == DownloadState::Completed
    }

    /// Gets the progress as a percentage (0-100).
    pub fn progress_percent(&self) -> i32 {
        if self.total_size <= 0 {
            return 0;
        }
        let percent = (self.downloaded * 100 / self.total_size) as i32;
        percent.clamp(0, 100)
    }
}

/// Manager for file downloads.
///
/// Tracks download progress and state for multiple files.
#[derive(Debug, Clone)]
pub struct DownloadManager {
    /// Map of file_id -> DownloadInfo
    downloads: HashMap<FileId, DownloadInfo>,
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DownloadManager {
    /// Creates a new download manager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    ///
    /// let manager = DownloadManager::new();
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            downloads: HashMap::new(),
        }
    }

    /// Starts a new download.
    ///
    /// # Errors
    ///
    /// Returns `DownloadError::AlreadyExists` if a download for this file already exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// assert!(manager.is_downloading(file_id));
    /// ```
    pub fn start_download(&mut self, file_id: FileId, total_size: i64) -> Result<()> {
        if self.downloads.contains_key(&file_id) {
            return Err(DownloadError::AlreadyExists { file_id });
        }
        let info = DownloadInfo::new(total_size);
        self.downloads.insert(file_id, info);
        Ok(())
    }

    /// Updates download progress.
    ///
    /// # Errors
    ///
    /// Returns `DownloadError::NotFound` if no download exists for this file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// manager.update_progress(file_id, 500).unwrap();
    ///
    /// assert_eq!(manager.get_progress(file_id).unwrap(), 500);
    /// ```
    pub fn update_progress(&mut self, file_id: FileId, downloaded: i64) -> Result<()> {
        let info = self
            .downloads
            .get_mut(&file_id)
            .ok_or(DownloadError::NotFound { file_id })?;
        info.downloaded = downloaded.clamp(0, info.total_size);
        Ok(())
    }

    /// Pauses or resumes a download.
    ///
    /// # Errors
    ///
    /// Returns `DownloadError::NotFound` if no download exists for this file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// manager.toggle_pause(file_id).unwrap();
    ///
    /// assert!(manager.is_paused(file_id));
    /// ```
    pub fn toggle_pause(&mut self, file_id: FileId) -> Result<()> {
        let info = self
            .downloads
            .get_mut(&file_id)
            .ok_or(DownloadError::NotFound { file_id })?;

        match info.state {
            DownloadState::Active => info.state = DownloadState::Paused,
            DownloadState::Paused => info.state = DownloadState::Active,
            DownloadState::Completed => {}
        }
        Ok(())
    }

    /// Marks a download as complete.
    ///
    /// # Errors
    ///
    /// Returns `DownloadError::NotFound` if no download exists for this file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// manager.complete_download(file_id).unwrap();
    ///
    /// assert!(manager.is_complete(file_id));
    /// ```
    pub fn complete_download(&mut self, file_id: FileId) -> Result<()> {
        let info = self
            .downloads
            .get_mut(&file_id)
            .ok_or(DownloadError::NotFound { file_id })?;
        info.state = DownloadState::Completed;
        info.downloaded = info.total_size;
        Ok(())
    }

    /// Removes a download from tracking.
    ///
    /// # Errors
    ///
    /// Returns `DownloadError::NotFound` if no download exists for this file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// manager.remove_download(file_id).unwrap();
    ///
    /// assert!(!manager.is_downloading(file_id));
    /// ```
    pub fn remove_download(&mut self, file_id: FileId) -> Result<()> {
        self.downloads
            .remove(&file_id)
            .ok_or(DownloadError::NotFound { file_id })?;
        Ok(())
    }

    /// Gets download info for a file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// let info = manager.get_info(file_id).unwrap();
    ///
    /// assert_eq!(info.total_size(), 1000);
    /// ```
    pub fn get_info(&self, file_id: FileId) -> Option<&DownloadInfo> {
        self.downloads.get(&file_id)
    }

    /// Gets the downloaded bytes for a file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// manager.update_progress(file_id, 500).unwrap();
    ///
    /// assert_eq!(manager.get_progress(file_id).unwrap(), 500);
    /// ```
    pub fn get_progress(&self, file_id: FileId) -> Option<i64> {
        self.get_info(file_id).map(|info| info.downloaded())
    }

    /// Checks if a download is active.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// assert!(manager.is_downloading(file_id));
    /// ```
    pub fn is_downloading(&self, file_id: FileId) -> bool {
        self.get_info(file_id)
            .map(|info| info.state() == DownloadState::Active)
            .unwrap_or(false)
    }

    /// Checks if a download is paused.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// manager.toggle_pause(file_id).unwrap();
    ///
    /// assert!(manager.is_paused(file_id));
    /// ```
    pub fn is_paused(&self, file_id: FileId) -> bool {
        self.get_info(file_id)
            .map(|info| info.state() == DownloadState::Paused)
            .unwrap_or(false)
    }

    /// Checks if a download is complete.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// manager.complete_download(file_id).unwrap();
    ///
    /// assert!(manager.is_complete(file_id));
    /// ```
    pub fn is_complete(&self, file_id: FileId) -> bool {
        self.get_info(file_id)
            .map(|info| info.is_complete())
            .unwrap_or(false)
    }

    /// Gets the total number of tracked downloads.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// assert_eq!(manager.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.downloads.len()
    }

    /// Clears all downloads.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_download_manager::DownloadManager;
    /// use rustgram_file_id::FileId;
    ///
    /// let mut manager = DownloadManager::new();
    /// let file_id = FileId::new(123, 0);
    ///
    /// manager.start_download(file_id, 1000).unwrap();
    /// manager.clear();
    ///
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.downloads.clear();
    }
}

impl Serialize for DownloadManager {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.downloads.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DownloadManager {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let downloads = HashMap::deserialize(deserializer)?;
        Ok(Self { downloads })
    }
}

impl fmt::Display for DownloadManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DownloadManager(count={})", self.count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Creation tests (2)
    #[test]
    fn test_new() {
        let manager = DownloadManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_default() {
        let manager = DownloadManager::default();
        assert_eq!(manager.count(), 0);
    }

    // Start download tests (3)
    #[test]
    fn test_start_download() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        assert!(manager.is_downloading(file_id));
    }

    #[test]
    fn test_start_download_duplicate() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        let result = manager.start_download(file_id, 2000);
        assert!(matches!(result, Err(DownloadError::AlreadyExists { .. })));
    }

    #[test]
    fn test_start_multiple_downloads() {
        let mut manager = DownloadManager::new();

        for i in 1..=5 {
            let file_id = FileId::new(i, 0);
            manager.start_download(file_id, 1000).unwrap();
        }

        assert_eq!(manager.count(), 5);
    }

    // Update progress tests (3)
    #[test]
    fn test_update_progress() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        manager.update_progress(file_id, 500).unwrap();

        assert_eq!(manager.get_progress(file_id).unwrap(), 500);
    }

    #[test]
    fn test_update_progress_not_found() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        let result = manager.update_progress(file_id, 500);
        assert!(matches!(result, Err(DownloadError::NotFound { .. })));
    }

    #[test]
    fn test_update_progress_clamp() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        manager.update_progress(file_id, 2000).unwrap();

        assert_eq!(manager.get_progress(file_id).unwrap(), 1000);
    }

    // Toggle pause tests (3)
    #[test]
    fn test_toggle_pause_to_paused() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        manager.toggle_pause(file_id).unwrap();

        assert!(manager.is_paused(file_id));
    }

    #[test]
    fn test_toggle_pause_to_active() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        manager.toggle_pause(file_id).unwrap();
        manager.toggle_pause(file_id).unwrap();

        assert!(manager.is_downloading(file_id));
    }

    #[test]
    fn test_toggle_pause_not_found() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        let result = manager.toggle_pause(file_id);
        assert!(matches!(result, Err(DownloadError::NotFound { .. })));
    }

    // Complete download tests (3)
    #[test]
    fn test_complete_download() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        manager.complete_download(file_id).unwrap();

        assert!(manager.is_complete(file_id));
        assert_eq!(manager.get_progress(file_id).unwrap(), 1000);
    }

    #[test]
    fn test_complete_download_not_found() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        let result = manager.complete_download(file_id);
        assert!(matches!(result, Err(DownloadError::NotFound { .. })));
    }

    #[test]
    fn test_complete_download_sets_progress() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        manager.update_progress(file_id, 500).unwrap();
        manager.complete_download(file_id).unwrap();

        assert_eq!(manager.get_progress(file_id).unwrap(), 1000);
    }

    // Remove download tests (2)
    #[test]
    fn test_remove_download() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        manager.remove_download(file_id).unwrap();

        assert!(!manager.is_downloading(file_id));
    }

    #[test]
    fn test_remove_download_not_found() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        let result = manager.remove_download(file_id);
        assert!(matches!(result, Err(DownloadError::NotFound { .. })));
    }

    // Get info tests (2)
    #[test]
    fn test_get_info() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        let info = manager.get_info(file_id).unwrap();

        assert_eq!(info.total_size(), 1000);
        assert_eq!(info.downloaded(), 0);
    }

    #[test]
    fn test_get_info_not_found() {
        let manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        assert!(manager.get_info(file_id).is_none());
    }

    // State check tests (3)
    #[test]
    fn test_is_downloading() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        assert!(manager.is_downloading(file_id));
    }

    #[test]
    fn test_is_paused() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        manager.toggle_pause(file_id).unwrap();

        assert!(manager.is_paused(file_id));
    }

    #[test]
    fn test_is_complete() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        manager.complete_download(file_id).unwrap();

        assert!(manager.is_complete(file_id));
    }

    // Count tests (2)
    #[test]
    fn test_count_empty() {
        let manager = DownloadManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_count() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        assert_eq!(manager.count(), 1);
    }

    // Clear tests (1)
    #[test]
    fn test_clear() {
        let mut manager = DownloadManager::new();

        for i in 1..=5 {
            let file_id = FileId::new(i, 0);
            manager.start_download(file_id, 1000).unwrap();
        }

        manager.clear();
        assert_eq!(manager.count(), 0);
    }

    // Display tests (1)
    #[test]
    fn test_display() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        let display = format!("{}", manager);

        assert!(display.contains("DownloadManager"));
        assert!(display.contains("count=1"));
    }

    // Clone tests (1)
    #[test]
    fn test_clone() {
        let mut manager = DownloadManager::new();
        let file_id = FileId::new(123, 0);

        manager.start_download(file_id, 1000).unwrap();
        let cloned = manager.clone();

        assert!(cloned.is_downloading(file_id));
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize() {
        let mut manager = DownloadManager::new();

        for i in 1..=3 {
            let file_id = FileId::new(i, 0);
            manager.start_download(file_id, 1000).unwrap();
        }

        let json = serde_json::to_string(&manager).unwrap();
        let deserialized: DownloadManager = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.count(), 3);
    }

    #[test]
    fn test_serialize_empty() {
        let manager = DownloadManager::new();

        let json = serde_json::to_string(&manager).unwrap();
        let deserialized: DownloadManager = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.count(), 0);
    }

    // DownloadInfo tests (3)
    #[test]
    fn test_download_info_progress_percent() {
        let info = DownloadInfo::new(1000);
        assert_eq!(info.progress_percent(), 0);
    }

    #[test]
    fn test_download_info_is_complete() {
        let mut info = DownloadInfo::new(1000);
        assert!(!info.is_complete());

        info.state = DownloadState::Completed;
        assert!(info.is_complete());
    }

    #[test]
    fn test_download_info_getters() {
        let info = DownloadInfo::new(1000);
        assert_eq!(info.total_size(), 1000);
        assert_eq!(info.downloaded(), 0);
        assert_eq!(info.state(), DownloadState::Active);
    }
}
