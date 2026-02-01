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

//! # File Download Manager
//!
//! Manages multiple concurrent file downloads from Telegram servers.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `DownloadManager` class from `td/telegram/DownloadManager.h`.
//!
//! ## Overview
//!
//! The FileDownloadManager handles coordinating multiple file downloads with support for:
//!
//! - **Priority queue**: Downloads are prioritized and queued
//! - **Bandwidth allocation**: Integrates with ResourceManager for bandwidth control
//! - **Progress tracking**: Per-download progress monitoring
//! - **Pause/resume/cancel**: Full download control
//! - **Auto-retry**: Failed downloads can be automatically retried
//!
//! ## Usage
//!
//! ```rust,no_run
//! use rustgram_file_download_manager::{FileDownloadManager, FileDownloadManagerConfig};
//! use rustgram_file_downloader::FileDownloaderConfig;
//! use rustgram_file_location::{FullRemoteFileLocation, LocalFileLocation};
//! use rustgram_resource_manager::ResourcePriority;
//!
//! let config = FileDownloadManagerConfig::new()
//!     .with_max_concurrent_downloads(3)
//!     .with_max_bandwidth(1_000_000); // 1 MB/s
//!
//! let mut manager = FileDownloadManager::new(config);
//!
//! // Add a download
//! let remote = FullRemoteFileLocation::common(123, 456);
//! let download_id = manager.add_download(remote, LocalFileLocation::empty(), 10_000_000, ResourcePriority::Normal)?;
//!
//! // Start downloads
//! manager.process_queue();
//!
//! // Check progress
//! let progress = manager.get_progress(download_id)?;
//! println!("Download progress: {:.1}%", progress);
//! # Ok::<(), rustgram_file_download_manager::Error>(())
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub use callback::{DefaultFileDownloadManagerCallback, FileDownloadManagerCallback};
pub use config::FileDownloadManagerConfig;
pub use download_info::DownloadInfo;
pub use error::{DownloadStateInternal, Error, Result};

mod callback;
mod config;
mod download_info;
mod error;

use rustgram_file_downloader::{FileDownloader, FileDownloaderConfig};
use rustgram_file_id::FileId;
use rustgram_file_location::{FullRemoteFileLocation, LocalFileLocation};
use rustgram_file_source_id::FileSourceId;
use rustgram_resource_manager::{ResourceManager, ResourcePriority};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

/// File download manager for coordinating multiple concurrent downloads.
///
/// Thread-safe through interior mutability.
pub struct FileDownloadManager {
    /// Manager configuration.
    config: FileDownloadManagerConfig,
    /// Download information by download ID.
    downloads: parking_lot::RwLock<HashMap<u64, DownloadInfo>>,
    /// Priority queue of pending downloads.
    queue: parking_lot::RwLock<VecDeque<u64>>,
    /// Currently active downloads.
    active_downloads: parking_lot::RwLock<HashMap<u64, FileDownloader>>,
    /// Completed download IDs (for history management).
    completed_downloads: parking_lot::RwLock<VecDeque<u64>>,
    /// Resource manager for bandwidth control.
    resource_manager: ResourceManager,
    /// Callback for progress notifications.
    callback: Box<dyn FileDownloadManagerCallback>,
    /// Next download ID counter.
    next_download_id: AtomicU64,
}

impl FileDownloadManager {
    /// Creates a new file download manager with default configuration.
    #[must_use]
    pub fn new(config: FileDownloadManagerConfig) -> Self {
        Self::with_callback(config, Box::<DefaultFileDownloadManagerCallback>::default())
    }

    /// Creates a new file download manager with a custom callback.
    ///
    /// # Arguments
    ///
    /// * `config` - Manager configuration
    /// * `callback` - Callback for progress notifications
    pub fn with_callback(
        config: FileDownloadManagerConfig,
        callback: Box<dyn FileDownloadManagerCallback>,
    ) -> Self {
        // Initialize resource manager with bandwidth limits
        let resource_manager = ResourceManager::new()
            .with_max_download_speed(config.max_bandwidth())
            .with_max_concurrent(config.max_concurrent_downloads());

        Self {
            config,
            downloads: parking_lot::RwLock::new(HashMap::new()),
            queue: parking_lot::RwLock::new(VecDeque::new()),
            active_downloads: parking_lot::RwLock::new(HashMap::new()),
            completed_downloads: parking_lot::RwLock::new(VecDeque::new()),
            resource_manager,
            callback,
            next_download_id: AtomicU64::new(1),
        }
    }

    /// Adds a new download to the queue.
    ///
    /// # Arguments
    ///
    /// * `remote` - Remote file location
    /// * `local` - Local file location
    /// * `size` - File size in bytes
    /// * `priority` - Download priority
    ///
    /// # Errors
    ///
    /// Returns an error if the queue is full.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rustgram_file_download_manager::FileDownloadManager;
    /// use rustgram_file_location::{FullRemoteFileLocation, LocalFileLocation};
    /// use rustgram_resource_manager::ResourcePriority;
    ///
    /// let manager = FileDownloadManager::new(Default::default());
    /// let remote = FullRemoteFileLocation::common(123, 456);
    ///
    /// let download_id = manager.add_download(
    ///     remote,
    ///     LocalFileLocation::empty(),
    ///     10_000_000,
    ///     ResourcePriority::Normal
    /// )?;
    /// # Ok::<(), rustgram_file_download_manager::Error>(())
    /// ```
    pub fn add_download(
        &self,
        remote: FullRemoteFileLocation,
        local: LocalFileLocation,
        size: i64,
        priority: ResourcePriority,
    ) -> Result<u64> {
        // Check queue size
        let queue = self.queue.read();
        if queue.len() >= self.config.queue_size() {
            return Err(Error::QueueFull(self.config.queue_size()));
        }
        drop(queue);

        let download_id = self.next_download_id.fetch_add(1, Ordering::SeqCst);
        let file_id = FileId::new(download_id as i32, 0);
        let file_source_id = FileSourceId::new(download_id as i32);

        let info = DownloadInfo::new(
            download_id,
            file_id,
            file_source_id,
            remote,
            local,
            size,
            priority,
        );

        {
            let mut downloads = self.downloads.write();
            downloads.insert(download_id, info.clone());
        }

        {
            let mut queue = self.queue.write();
            // Insert in priority order (higher priority first)
            let insert_pos = queue
                .iter()
                .position(|&id| {
                    let downloads = self.downloads.read();
                    downloads
                        .get(&id)
                        .is_some_and(|info| info.priority() < priority)
                })
                .unwrap_or(queue.len());
            queue.insert(insert_pos, download_id);
        }

        self.callback.on_download_added(download_id, file_id);
        Ok(download_id)
    }

    /// Pauses a download.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The download ID to pause
    ///
    /// # Errors
    ///
    /// Returns an error if the download is not found or already paused.
    pub fn pause_download(&self, download_id: u64) -> Result<()> {
        let mut downloads = self.downloads.write();
        let info = downloads
            .get_mut(&download_id)
            .ok_or(Error::DownloadNotFound(download_id))?;

        if info.is_paused() {
            return Err(Error::AlreadyPaused(download_id));
        }

        if info.is_completed() {
            return Err(Error::AlreadyCompleted(download_id));
        }

        info.transition_to(crate::error::DownloadStateInternal::Paused)?;

        // If active, remove from active downloads
        if info.is_active() {
            let mut active = self.active_downloads.write();
            active.remove(&download_id);
        }

        self.callback.on_download_paused(download_id, info.file_id());
        Ok(())
    }

    /// Resumes a paused download.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The download ID to resume
    ///
    /// # Errors
    ///
    /// Returns an error if the download is not found or not paused.
    pub fn resume_download(&self, download_id: u64) -> Result<()> {
        let mut downloads = self.downloads.write();
        let info = downloads
            .get_mut(&download_id)
            .ok_or(Error::DownloadNotFound(download_id))?;

        if !info.is_paused() {
            return Err(Error::InvalidStateTransition(
                info.state(),
                crate::error::DownloadStateInternal::Active,
            ));
        }

        info.transition_to(crate::error::DownloadStateInternal::Active)?;

        // Re-add to queue for processing
        let mut queue = self.queue.write();
        if !queue.contains(&download_id) {
            queue.push_back(download_id);
        }

        self.callback.on_download_resumed(download_id, info.file_id());
        Ok(())
    }

    /// Cancels a download.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The download ID to cancel
    ///
    /// # Errors
    ///
    /// Returns an error if the download is not found.
    pub fn cancel_download(&self, download_id: u64) -> Result<()> {
        let mut downloads = self.downloads.write();
        let info = downloads
            .get_mut(&download_id)
            .ok_or(Error::DownloadNotFound(download_id))?;

        if info.is_completed() {
            return Err(Error::AlreadyCompleted(download_id));
        }

        info.transition_to(crate::error::DownloadStateInternal::Cancelled)?;

        // Remove from active downloads and queue
        let mut active = self.active_downloads.write();
        active.remove(&download_id);

        let mut queue = self.queue.write();
        queue.retain(|&id| id != download_id);

        self.callback.on_download_cancelled(download_id, info.file_id());
        Ok(())
    }

    /// Removes a download from the manager.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The download ID to remove
    ///
    /// # Errors
    ///
    /// Returns an error if the download is still active.
    pub fn remove_download(&self, download_id: u64) -> Result<()> {
        {
            let downloads = self.downloads.read();
            let info = downloads
                .get(&download_id)
                .ok_or(Error::DownloadNotFound(download_id))?;

            if info.is_active() || info.is_pending() {
                return Err(Error::Cancelled(download_id));
            }
        }

        {
            let mut downloads = self.downloads.write();
            let info = downloads.remove(&download_id);
            if let Some(info) = info {
                self.callback.on_download_removed(download_id, info.file_id());
            }
        }

        // Remove from completed history
        let mut completed = self.completed_downloads.write();
        completed.retain(|&id| id != download_id);

        Ok(())
    }

    /// Gets the progress of a download.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The download ID
    ///
    /// # Errors
    ///
    /// Returns an error if the download is not found.
    #[must_use = "returns download progress (0.0-1.0) that should be used"]
    pub fn get_progress(&self, download_id: u64) -> Result<f64> {
        let downloads = self.downloads.read();
        let info = downloads
            .get(&download_id)
            .ok_or(Error::DownloadNotFound(download_id))?;
        Ok(info.progress())
    }

    /// Gets download information.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The download ID
    ///
    /// # Errors
    ///
    /// Returns an error if the download is not found.
    #[must_use = "returns download info that should be used"]
    pub fn get_download_info(&self, download_id: u64) -> Result<DownloadInfo> {
        let downloads = self.downloads.read();
        downloads
            .get(&download_id)
            .cloned()
            .ok_or(Error::DownloadNotFound(download_id))
    }

    /// Returns the number of active downloads.
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.active_downloads.read().len()
    }

    /// Returns the number of pending downloads in the queue.
    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.queue.read().len()
    }

    /// Returns the total number of downloads.
    #[must_use]
    pub fn total_count(&self) -> usize {
        self.downloads.read().len()
    }

    /// Processes the download queue and starts pending downloads.
    ///
    /// This method should be called periodically or when new downloads are added.
    pub fn process_queue(&self) {
        let max_concurrent = self.config.max_concurrent_downloads();
        let active_count = self.active_count();

        if active_count >= max_concurrent {
            return;
        }

        let available_slots = max_concurrent - active_count;
        let mut queue = self.queue.write();
        let mut downloads = self.downloads.write();
        let mut active = self.active_downloads.write();

        let mut started = 0;
        while started < available_slots && !queue.is_empty() {
            if let Some(download_id) = queue.pop_front() {
                if let Some(info) = downloads.get_mut(&download_id) {
                    if info.is_paused() {
                        continue;
                    }

                    if let Err(_e) =
                        info.transition_to(crate::error::DownloadStateInternal::Active)
                    {
                        continue;
                    }

                    // Create downloader configuration
                    let config = FileDownloaderConfig::new(info.remote().clone(), info.size())
                        .with_local(info.local().clone());

                    if let Ok(downloader) = FileDownloader::new(config) {
                        self.callback
                            .on_download_started(download_id, info.file_id());
                        active.insert(download_id, downloader);
                        started += 1;
                    }
                }
            }
        }
    }

    /// Updates download progress for a download.
    ///
    /// # Arguments
    ///
    /// * `download_id` - The download ID
    /// * `downloaded_size` - New downloaded size
    ///
    /// # Errors
    ///
    /// Returns an error if the download is not found.
    pub fn update_progress(&self, download_id: u64, downloaded_size: i64) -> Result<()> {
        let mut downloads = self.downloads.write();
        let info = downloads
            .get_mut(&download_id)
            .ok_or(Error::DownloadNotFound(download_id))?;

        info.update_downloaded_size(downloaded_size);

        if info.progress() >= 100.0 {
            // Ensure we're in Active state before transitioning to Completed
            if info.is_pending() || info.is_paused() {
                let _ = info.transition_to(crate::error::DownloadStateInternal::Active);
            }

            info.transition_to(crate::error::DownloadStateInternal::Completed)?;
            self.callback
                .on_download_completed(download_id, info.file_id());

            // Add to completed history
            let mut completed = self.completed_downloads.write();
            completed.push_back(download_id);

            // Remove from active
            let mut active = self.active_downloads.write();
            active.remove(&download_id);

            // Trim completed history if needed
            if self.config.auto_remove_completed()
                && completed.len() > self.config.max_completed_history()
            {
                let excess = completed.len() - self.config.max_completed_history();
                for _ in 0..excess {
                    if let Some(removed_id) = completed.pop_front() {
                        let mut downloads = self.downloads.write();
                        downloads.remove(&removed_id);
                    }
                }
            }
        } else {
            self.callback.on_download_progress(
                download_id,
                info.file_id(),
                info.downloaded_size(),
                info.expected_size(),
                info.progress(),
            );
        }

        Ok(())
    }

    /// Pauses all active downloads and removes pending ones from queue.
    pub fn pause_all(&self) {
        let downloads = self.downloads.read();
        for (download_id, info) in downloads.iter() {
            if info.is_active() || info.is_pending() {
                let _ = self.pause_download(*download_id);
            }
        }
    }

    /// Resumes all paused downloads.
    pub fn resume_all(&self) {
        let downloads = self.downloads.read();
        for (download_id, info) in downloads.iter() {
            if info.is_paused() {
                let _ = self.resume_download(*download_id);
            }
        }
    }

    /// Cancels all downloads.
    pub fn cancel_all(&self) {
        let downloads = self.downloads.read();
        for download_id in downloads.keys() {
            let _ = self.cancel_download(*download_id);
        }
    }

    /// Clears all completed downloads from history.
    pub fn clear_completed(&self) {
        let mut downloads = self.downloads.write();
        let mut completed = self.completed_downloads.write();
        let mut to_remove = Vec::new();

        for download_id in completed.iter() {
            if let Some(info) = downloads.get(download_id) {
                if info.is_completed() {
                    to_remove.push(*download_id);
                }
            }
        }

        for download_id in to_remove {
            downloads.remove(&download_id);
            completed.retain(|&id| id != download_id);
        }
    }

    /// Returns the resource manager for bandwidth control.
    #[must_use]
    pub const fn resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }
}

impl fmt::Debug for FileDownloadManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileDownloadManager")
            .field("config", &self.config)
            .field("active_count", &self.active_count())
            .field("pending_count", &self.pending_count())
            .field("total_count", &self.total_count())
            .finish()
    }
}

impl Default for FileDownloadManager {
    fn default() -> Self {
        Self::new(FileDownloadManagerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_resource_manager::ResourcePriority;

    #[test]
    fn test_new_manager() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        assert_eq!(manager.active_count(), 0);
        assert_eq!(manager.pending_count(), 0);
        assert_eq!(manager.total_count(), 0);
    }

    #[test]
    fn test_default_manager() {
        let manager = FileDownloadManager::default();
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_add_download() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        let result = manager.add_download(
            remote,
            LocalFileLocation::empty(),
            10_000_000,
            ResourcePriority::Normal,
        );

        assert!(result.is_ok());
        let download_id = result.unwrap();
        assert_eq!(download_id, 1);
        assert_eq!(manager.pending_count(), 1);
        assert_eq!(manager.total_count(), 1);
    }

    #[test]
    fn test_get_download_info() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        let download_id = manager
            .add_download(
                remote,
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();

        let info = manager.get_download_info(download_id);
        assert!(info.is_ok());
        let info = info.unwrap();
        assert_eq!(info.download_id(), download_id);
        assert!(info.is_pending());
    }

    #[test]
    fn test_get_progress() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        let download_id = manager
            .add_download(
                remote,
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();

        let progress = manager.get_progress(download_id);
        assert!(progress.is_ok());
        assert_eq!(progress.unwrap(), 0.0);
    }

    #[test]
    fn test_pause_download() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        let download_id = manager
            .add_download(
                remote,
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();

        let result = manager.pause_download(download_id);
        assert!(result.is_ok());

        let info = manager.get_download_info(download_id).unwrap();
        assert!(info.is_paused());
    }

    #[test]
    fn test_pause_already_paused() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        let download_id = manager
            .add_download(
                remote,
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();

        manager.pause_download(download_id).unwrap();
        let result = manager.pause_download(download_id);
        assert!(matches!(result, Err(Error::AlreadyPaused(_))));
    }

    #[test]
    fn test_resume_download() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        let download_id = manager
            .add_download(
                remote,
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();

        manager.pause_download(download_id).unwrap();
        let result = manager.resume_download(download_id);
        assert!(result.is_ok());

        let info = manager.get_download_info(download_id).unwrap();
        assert!(info.is_active());
    }

    #[test]
    fn test_cancel_download() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        let download_id = manager
            .add_download(
                remote,
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();

        let result = manager.cancel_download(download_id);
        assert!(result.is_ok());

        let info = manager.get_download_info(download_id).unwrap();
        assert!(info.is_cancelled());
    }

    #[test]
    fn test_cancel_not_found() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let result = manager.cancel_download(999);
        assert!(matches!(result, Err(Error::DownloadNotFound(999))));
    }

    #[test]
    fn test_update_progress() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        let download_id = manager
            .add_download(
                remote,
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();

        let result = manager.update_progress(download_id, 5_000_000);
        assert!(result.is_ok());

        let progress = manager.get_progress(download_id).unwrap();
        assert!((progress - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_update_progress_complete() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        let download_id = manager
            .add_download(
                remote,
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();

        let result = manager.update_progress(download_id, 10_000_000);
        assert!(result.is_ok());

        let info = manager.get_download_info(download_id).unwrap();
        assert!(info.is_completed());
    }

    #[test]
    fn test_pause_all() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        manager
            .add_download(
                remote.clone(),
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();
        manager
            .add_download(remote, LocalFileLocation::empty(), 5_000_000, ResourcePriority::Normal)
            .unwrap();

        manager.pause_all();

        assert_eq!(manager.pending_count(), 0);
    }

    #[test]
    fn test_clear_completed() {
        let manager = FileDownloadManager::new(FileDownloadManagerConfig::new());
        let remote = FullRemoteFileLocation::common(123, 456);

        let download_id = manager
            .add_download(
                remote,
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();

        manager.update_progress(download_id, 10_000_000).unwrap();
        manager.clear_completed();

        let result = manager.get_download_info(download_id);
        assert!(matches!(result, Err(Error::DownloadNotFound(_))));
    }

    #[test]
    fn test_priority_queue() {
        let config = FileDownloadManagerConfig::new().with_max_concurrent_downloads(3);
        let manager = FileDownloadManager::new(config);
        let remote = FullRemoteFileLocation::common(123, 456);

        // Add downloads with different priorities
        manager
            .add_download(
                remote.clone(),
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Low,
            )
            .unwrap();
        manager
            .add_download(
                remote.clone(),
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::High,
            )
            .unwrap();
        manager
            .add_download(
                remote.clone(),
                LocalFileLocation::empty(),
                10_000_000,
                ResourcePriority::Normal,
            )
            .unwrap();

        // Queue should be ordered by priority
        assert_eq!(manager.pending_count(), 3);
    }
}
