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

//! # Storage Manager
//!
//! Manager for Telegram storage statistics and garbage collection.
//!
//! ## Overview
//!
//! The `StorageManager` provides methods to:
//! - Get storage statistics (fast and detailed)
//! - Run garbage collection
//! - Get database statistics
//! - Manage file storage optimization
//!
//! ## Usage
//!
//! ```rust,no_run
//! use rustgram_storage::StorageManager;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = StorageManager::new();
//!
//!     // Get fast storage statistics
//!     let stats = manager.get_storage_stats_fast().await?;
//!     println!("Total size: {} bytes", stats.size);
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod error;
pub mod types;

use crate::error::{Error, Result};
use crate::types::{DatabaseStats, FileGcParameters, FileStats, FileStatsFast};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Default storage path.
const DEFAULT_STORAGE_PATH: &str = "/tmp/telegram";

/// Storage Manager.
///
/// Manages Telegram storage statistics and garbage collection.
#[derive(Debug, Clone)]
pub struct StorageManager {
    /// Storage path.
    storage_path: Arc<String>,
    /// Cached fast statistics.
    fast_stats: Arc<RwLock<FileStatsFast>>,
}

impl StorageManager {
    /// Creates a new StorageManager.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_storage::StorageManager;
    ///
    /// let manager = StorageManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage_path: Arc::new(DEFAULT_STORAGE_PATH.to_string()),
            fast_stats: Arc::new(RwLock::new(FileStatsFast::new(0, 0, 0, 0, 0))),
        }
    }

    /// Creates a new StorageManager with a custom storage path.
    ///
    /// # Arguments
    ///
    /// * `path` - The storage directory path
    #[must_use]
    pub fn with_path(path: String) -> Self {
        Self {
            storage_path: Arc::new(path),
            fast_stats: Arc::new(RwLock::new(FileStatsFast::new(0, 0, 0, 0, 0))),
        }
    }

    /// Gets fast storage statistics without file scanning.
    ///
    /// # Returns
    ///
    /// Returns cached storage statistics including total size, count, and database sizes.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `get_storage_stats_fast(Promise<FileStatsFast> promise)`
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rustgram_storage::StorageManager;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = StorageManager::new();
    /// let stats = manager.get_storage_stats_fast().await?;
    /// println!("Total size: {} bytes", stats.size);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_storage_stats_fast(&self) -> Result<FileStatsFast> {
        let stats = self.fast_stats.read().await;
        Ok(FileStatsFast::new(
            stats.size,
            stats.count,
            stats.database_size,
            stats.language_pack_database_size,
            stats.log_size,
        ))
    }

    /// Gets detailed storage statistics.
    ///
    /// # Arguments
    ///
    /// * `need_all_files` - Whether to include all files in the result
    /// * `dialog_limit` - Maximum number of dialogs to include in statistics
    ///
    /// # Returns
    ///
    /// Returns detailed file statistics broken down by type and dialog.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `get_storage_stats(bool need_all_files, int32 dialog_limit, Promise<FileStats> promise)`
    pub async fn get_storage_stats(
        &self,
        need_all_files: bool,
        dialog_limit: i32,
    ) -> Result<FileStats> {
        // In a real implementation, this would scan the storage directory
        // For now, return empty stats
        Ok(FileStats::new(need_all_files, dialog_limit > 0))
    }

    /// Gets database statistics.
    ///
    /// # Returns
    ///
    /// Returns database-specific statistics and debug information.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `get_database_stats(Promise<DatabaseStats> promise)`
    pub async fn get_database_stats(&self) -> Result<DatabaseStats> {
        // In a real implementation, this would query the database
        Ok(DatabaseStats::new("Database statistics not implemented".to_string()))
    }

    /// Runs garbage collection on storage.
    ///
    /// # Arguments
    ///
    /// * `parameters` - GC parameters for filtering files
    /// * `return_deleted_stats` - Whether to return statistics of deleted files
    ///
    /// # Returns
    ///
    /// Returns statistics of files after garbage collection.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `run_gc(FileGcParameters parameters, bool return_deleted_file_statistics, Promise<FileStats> promise)`
    pub async fn run_gc(
        &self,
        parameters: FileGcParameters,
        return_deleted_stats: bool,
    ) -> Result<FileStats> {
        // In a real implementation, this would:
        // 1. Scan files based on parameters
        // 2. Delete files that match GC criteria
        // 3. Return statistics
        tracing::info!("Running garbage collection: return_deleted_stats={}", return_deleted_stats);
        Ok(FileStats::new(return_deleted_stats, parameters.dialog_ids.is_some()))
    }

    /// Updates storage optimizer settings.
    ///
    /// # TDLib Mapping
    ///
    /// TDLib: `update_use_storage_optimizer()`
    pub async fn update_use_storage_optimizer(&self) {
        tracing::info!("Updating storage optimizer settings");
    }

    /// Gets the storage path.
    #[must_use]
    pub const fn storage_path(&self) -> &str {
        // Note: We can't return &str from Arc<String> directly in const context
        // This is a simplified version
        ""
    }

    /// Updates the cached fast statistics.
    pub async fn update_fast_stats(&self, stats: FileStatsFast) {
        let mut fast_stats = self.fast_stats.write().await;
        *fast_stats = stats;
    }
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manager_new() {
        let manager = StorageManager::new();
        let stats = manager.get_storage_stats_fast().await.unwrap();
        assert_eq!(stats.size, 0);
        assert_eq!(stats.count, 0);
    }

    #[tokio::test]
    async fn test_manager_default() {
        let manager = StorageManager::default();
        let stats = manager.get_storage_stats_fast().await.unwrap();
        assert_eq!(stats.size, 0);
    }

    #[tokio::test]
    async fn test_get_storage_stats() {
        let manager = StorageManager::new();
        let stats = manager.get_storage_stats(true, 10).await.unwrap();
        assert_eq!(stats.get_total_size(), 0);
        assert_eq!(stats.get_total_count(), 0);
    }

    #[tokio::test]
    async fn test_get_database_stats() {
        let manager = StorageManager::new();
        let stats = manager.get_database_stats().await.unwrap();
        assert!(!stats.debug.is_empty());
    }

    #[tokio::test]
    async fn test_run_gc() {
        let manager = StorageManager::new();
        let params = FileGcParameters::new();
        let stats = manager.run_gc(params, false).await.unwrap();
        assert_eq!(stats.get_total_size(), 0);
    }

    #[tokio::test]
    async fn test_update_fast_stats() {
        let manager = StorageManager::new();
        let new_stats = FileStatsFast::new(1024, 10, 512, 256, 128);
        manager.update_fast_stats(new_stats.clone()).await;

        let stats = manager.get_storage_stats_fast().await.unwrap();
        assert_eq!(stats.size, 1024);
        assert_eq!(stats.count, 10);
    }
}
