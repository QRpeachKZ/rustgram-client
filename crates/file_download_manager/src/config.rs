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

//! Configuration for file download manager.

use rustgram_resource_manager::ResourcePriority;
use std::time::Duration;

/// Configuration for the file download manager.
///
/// # Examples
///
/// ```
/// use rustgram_file_download_manager::FileDownloadManagerConfig;
/// use rustgram_resource_manager::ResourcePriority;
///
/// let config = FileDownloadManagerConfig::new()
///     .with_max_concurrent_downloads(4)
///     .with_max_bandwidth(1_000_000) // 1 MB/s
///     .with_default_priority(ResourcePriority::Normal)
///     .with_queue_size(100);
/// ```
#[derive(Debug, Clone)]
pub struct FileDownloadManagerConfig {
    /// Maximum number of concurrent downloads.
    max_concurrent_downloads: usize,
    /// Maximum bandwidth for downloads in bytes per second (0 = unlimited).
    max_bandwidth: u64,
    /// Default priority for new downloads.
    default_priority: ResourcePriority,
    /// Maximum size of the download queue.
    queue_size: usize,
    /// Whether to automatically retry failed downloads.
    auto_retry: bool,
    /// Maximum number of retry attempts.
    max_retries: u32,
    /// Delay between retry attempts.
    retry_delay: Duration,
    /// Whether to remove completed downloads from the queue automatically.
    auto_remove_completed: bool,
    /// Maximum number of completed downloads to keep in history.
    max_completed_history: usize,
}

impl Default for FileDownloadManagerConfig {
    fn default() -> Self {
        Self {
            max_concurrent_downloads: 3,
            max_bandwidth: 0, // Unlimited
            default_priority: ResourcePriority::Normal,
            queue_size: 100,
            auto_retry: true,
            max_retries: 3,
            retry_delay: Duration::from_secs(5),
            auto_remove_completed: false,
            max_completed_history: 200,
        }
    }
}

impl FileDownloadManagerConfig {
    /// Creates a new default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_concurrent_downloads: 3,
            max_bandwidth: 0,
            default_priority: ResourcePriority::Normal,
            queue_size: 100,
            auto_retry: true,
            max_retries: 3,
            retry_delay: Duration::from_secs(5),
            auto_remove_completed: false,
            max_completed_history: 200,
        }
    }

    /// Sets the maximum number of concurrent downloads.
    ///
    /// # Arguments
    ///
    /// * `max` - Maximum concurrent downloads (must be > 0)
    #[must_use]
    pub const fn with_max_concurrent_downloads(mut self, max: usize) -> Self {
        self.max_concurrent_downloads = if max == 0 { 1 } else { max };
        self
    }

    /// Sets the maximum bandwidth for downloads.
    ///
    /// # Arguments
    ///
    /// * `bytes_per_sec` - Maximum bandwidth in bytes per second (0 = unlimited)
    #[must_use]
    pub const fn with_max_bandwidth(mut self, bytes_per_sec: u64) -> Self {
        self.max_bandwidth = bytes_per_sec;
        self
    }

    /// Sets the default priority for new downloads.
    ///
    /// # Arguments
    ///
    /// * `priority` - Default priority level
    #[must_use]
    pub const fn with_default_priority(mut self, priority: ResourcePriority) -> Self {
        self.default_priority = priority;
        self
    }

    /// Sets the maximum size of the download queue.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum queue size (must be > 0)
    #[must_use]
    pub const fn with_queue_size(mut self, size: usize) -> Self {
        self.queue_size = if size == 0 { 1 } else { size };
        self
    }

    /// Sets whether to automatically retry failed downloads.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether auto-retry is enabled
    #[must_use]
    pub const fn with_auto_retry(mut self, enabled: bool) -> Self {
        self.auto_retry = enabled;
        self
    }

    /// Sets the maximum number of retry attempts.
    ///
    /// # Arguments
    ///
    /// * `max` - Maximum retry attempts
    #[must_use]
    pub const fn with_max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    /// Sets the delay between retry attempts.
    ///
    /// # Arguments
    ///
    /// * `delay` - Delay between retries
    #[must_use]
    pub const fn with_retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = delay;
        self
    }

    /// Sets whether to automatically remove completed downloads.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether auto-remove is enabled
    #[must_use]
    pub const fn with_auto_remove_completed(mut self, enabled: bool) -> Self {
        self.auto_remove_completed = enabled;
        self
    }

    /// Sets the maximum number of completed downloads to keep in history.
    ///
    /// # Arguments
    ///
    /// * `max` - Maximum completed history size
    #[must_use]
    pub const fn with_max_completed_history(mut self, max: usize) -> Self {
        self.max_completed_history = max;
        self
    }

    /// Returns the maximum number of concurrent downloads.
    #[must_use]
    pub const fn max_concurrent_downloads(&self) -> usize {
        self.max_concurrent_downloads
    }

    /// Returns the maximum bandwidth in bytes per second.
    #[must_use]
    pub const fn max_bandwidth(&self) -> u64 {
        self.max_bandwidth
    }

    /// Returns the default priority for new downloads.
    #[must_use]
    pub const fn default_priority(&self) -> ResourcePriority {
        self.default_priority
    }

    /// Returns the maximum queue size.
    #[must_use]
    pub const fn queue_size(&self) -> usize {
        self.queue_size
    }

    /// Returns whether auto-retry is enabled.
    #[must_use]
    pub const fn auto_retry(&self) -> bool {
        self.auto_retry
    }

    /// Returns the maximum number of retry attempts.
    #[must_use]
    pub const fn max_retries(&self) -> u32 {
        self.max_retries
    }

    /// Returns the delay between retry attempts.
    #[must_use]
    pub const fn retry_delay(&self) -> Duration {
        self.retry_delay
    }

    /// Returns whether auto-remove completed is enabled.
    #[must_use]
    pub const fn auto_remove_completed(&self) -> bool {
        self.auto_remove_completed
    }

    /// Returns the maximum completed history size.
    #[must_use]
    pub const fn max_completed_history(&self) -> usize {
        self.max_completed_history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = FileDownloadManagerConfig::default();
        assert_eq!(config.max_concurrent_downloads(), 3);
        assert_eq!(config.max_bandwidth(), 0);
        assert_eq!(config.default_priority(), ResourcePriority::Normal);
        assert_eq!(config.queue_size(), 100);
        assert!(config.auto_retry());
        assert_eq!(config.max_retries(), 3);
    }

    #[test]
    fn test_new_config() {
        let config = FileDownloadManagerConfig::new();
        assert_eq!(config.max_concurrent_downloads(), 3);
        assert_eq!(config.max_bandwidth(), 0);
    }

    #[test]
    fn test_with_max_concurrent_downloads() {
        let config = FileDownloadManagerConfig::new().with_max_concurrent_downloads(5);
        assert_eq!(config.max_concurrent_downloads(), 5);

        let config = FileDownloadManagerConfig::new().with_max_concurrent_downloads(0);
        assert_eq!(config.max_concurrent_downloads(), 1);
    }

    #[test]
    fn test_with_max_bandwidth() {
        let config = FileDownloadManagerConfig::new().with_max_bandwidth(1_000_000);
        assert_eq!(config.max_bandwidth(), 1_000_000);
    }

    #[test]
    fn test_with_queue_size() {
        let config = FileDownloadManagerConfig::new().with_queue_size(50);
        assert_eq!(config.queue_size(), 50);

        let config = FileDownloadManagerConfig::new().with_queue_size(0);
        assert_eq!(config.queue_size(), 1);
    }

    #[test]
    fn test_builder_pattern() {
        let config = FileDownloadManagerConfig::new()
            .with_max_concurrent_downloads(4)
            .with_max_bandwidth(2_000_000)
            .with_queue_size(200);

        assert_eq!(config.max_concurrent_downloads(), 4);
        assert_eq!(config.max_bandwidth(), 2_000_000);
        assert_eq!(config.queue_size(), 200);
    }
}
