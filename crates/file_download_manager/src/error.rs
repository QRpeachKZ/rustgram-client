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

//! Error types for file download manager.

use rustgram_file_downloader::Error as DownloaderError;
use rustgram_resource_manager::Error as ResourceManagerError;
use thiserror::Error;

/// Result type for file download manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in the file download manager.
#[derive(Debug, Error)]
pub enum Error {
    /// Error from the underlying file downloader.
    #[error("downloader error: {0}")]
    Downloader(#[from] DownloaderError),

    /// Error from the resource manager.
    #[error("resource manager error: {0}")]
    ResourceManager(#[from] ResourceManagerError),

    /// Download not found.
    #[error("download with id {0} not found")]
    DownloadNotFound(u64),

    /// Download is already paused.
    #[error("download {0} is already paused")]
    AlreadyPaused(u64),

    /// Download is already active.
    #[error("download {0} is already active")]
    AlreadyActive(u64),

    /// Download is already completed.
    #[error("download {0} is already completed")]
    AlreadyCompleted(u64),

    /// Download queue is full.
    #[error("download queue is full (max: {0})")]
    QueueFull(usize),

    /// Invalid download state transition.
    #[error("invalid state transition from {0:?} to {1:?}")]
    InvalidStateTransition(DownloadStateInternal, DownloadStateInternal),

    /// Bandwidth limit exceeded.
    #[error("bandwidth limit exceeded: {0} bytes/sec requested, max is {1} bytes/sec")]
    BandwidthLimitExceeded(u64, u64),

    /// Concurrent download limit exceeded.
    #[error("concurrent download limit exceeded: {0} active, max is {1}")]
    ConcurrentLimitExceeded(usize, usize),

    /// Download was cancelled.
    #[error("download {0} was cancelled")]
    Cancelled(u64),

    /// Invalid priority value.
    #[error("invalid priority value: {0}")]
    InvalidPriority(i8),
}

/// Internal download state for state machine validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadStateInternal {
    /// Download is pending (queued but not started).
    Pending,
    /// Download is active (currently downloading).
    Active,
    /// Download is paused.
    Paused,
    /// Download is completed successfully.
    Completed,
    /// Download failed.
    Failed,
    /// Download was cancelled.
    Cancelled,
}

impl DownloadStateInternal {
    /// Checks if a state transition is valid.
    #[must_use]
    pub const fn can_transition_to(self, new_state: Self) -> bool {
        match (self, new_state) {
            // Pending can go to Active, Paused, or Cancelled
            (Self::Pending, Self::Active) => true,
            (Self::Pending, Self::Paused) => true,
            (Self::Pending, Self::Cancelled) => true,

            // Active can go to Paused, Completed, Failed, or Cancelled
            (Self::Active, Self::Paused) => true,
            (Self::Active, Self::Completed) => true,
            (Self::Active, Self::Failed) => true,
            (Self::Active, Self::Cancelled) => true,

            // Paused can go to Active or Cancelled
            (Self::Paused, Self::Active) => true,
            (Self::Paused, Self::Cancelled) => true,

            // Completed is terminal
            (Self::Completed, _) => false,

            // Failed can go back to Pending or Active (retry)
            (Self::Failed, Self::Pending) => true,
            (Self::Failed, Self::Active) => true,

            // Cancelled is terminal
            (Self::Cancelled, _) => false,

            // Same state is not a transition
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions_valid() {
        assert!(DownloadStateInternal::Pending.can_transition_to(DownloadStateInternal::Active));
        assert!(DownloadStateInternal::Active.can_transition_to(DownloadStateInternal::Paused));
        assert!(DownloadStateInternal::Active.can_transition_to(DownloadStateInternal::Completed));
        assert!(DownloadStateInternal::Paused.can_transition_to(DownloadStateInternal::Active));
        assert!(DownloadStateInternal::Failed.can_transition_to(DownloadStateInternal::Active));
    }

    #[test]
    fn test_state_transitions_invalid() {
        assert!(!DownloadStateInternal::Completed.can_transition_to(DownloadStateInternal::Active));
        assert!(!DownloadStateInternal::Cancelled.can_transition_to(DownloadStateInternal::Active));
        assert!(!DownloadStateInternal::Pending.can_transition_to(DownloadStateInternal::Pending));
    }

    #[test]
    fn test_error_display() {
        let err = Error::DownloadNotFound(123);
        assert_eq!(err.to_string(), "download with id 123 not found");

        let err = Error::QueueFull(10);
        assert_eq!(err.to_string(), "download queue is full (max: 10)");

        let err = Error::BandwidthLimitExceeded(2000, 1000);
        assert_eq!(err.to_string(), "bandwidth limit exceeded: 2000 bytes/sec requested, max is 1000 bytes/sec");
    }
}
