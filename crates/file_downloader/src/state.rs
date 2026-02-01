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

//! Download state for file downloads.

use serde::{Deserialize, Serialize};
use std::fmt;

/// State of a file download.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DownloadState {
    /// Download has not started
    #[default]
    Idle,
    /// Download is active
    Active,
    /// Download is paused
    Paused,
    /// Download completed successfully
    Complete,
    /// Download failed
    Failed,
    /// Download was stopped
    Stopped,
}

impl DownloadState {
    /// Returns `true` if the download is idle.
    #[must_use]
    pub const fn is_idle(self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Returns `true` if the download is active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns `true` if the download is paused.
    #[must_use]
    pub const fn is_paused(self) -> bool {
        matches!(self, Self::Paused)
    }

    /// Returns `true` if the download is complete.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns `true` if the download failed.
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns `true` if the download is stopped.
    #[must_use]
    pub const fn is_stopped(self) -> bool {
        matches!(self, Self::Stopped)
    }

    /// Returns `true` if the download is in a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Complete | Self::Failed | Self::Stopped)
    }
}

impl fmt::Display for DownloadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Idle => "Idle",
            Self::Active => "Active",
            Self::Paused => "Paused",
            Self::Complete => "Complete",
            Self::Failed => "Failed",
            Self::Stopped => "Stopped",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let state = DownloadState::default();
        assert_eq!(state, DownloadState::Idle);
    }

    #[test]
    fn test_is_idle() {
        assert!(DownloadState::Idle.is_idle());
        assert!(!DownloadState::Active.is_idle());
    }

    #[test]
    fn test_is_active() {
        assert!(DownloadState::Active.is_active());
        assert!(!DownloadState::Idle.is_active());
    }

    #[test]
    fn test_is_paused() {
        assert!(DownloadState::Paused.is_paused());
        assert!(!DownloadState::Active.is_paused());
    }

    #[test]
    fn test_is_complete() {
        assert!(DownloadState::Complete.is_complete());
        assert!(!DownloadState::Active.is_complete());
    }

    #[test]
    fn test_is_failed() {
        assert!(DownloadState::Failed.is_failed());
        assert!(!DownloadState::Active.is_failed());
    }

    #[test]
    fn test_is_stopped() {
        assert!(DownloadState::Stopped.is_stopped());
        assert!(!DownloadState::Active.is_stopped());
    }

    #[test]
    fn test_is_terminal() {
        assert!(DownloadState::Complete.is_terminal());
        assert!(DownloadState::Failed.is_terminal());
        assert!(DownloadState::Stopped.is_terminal());
        assert!(!DownloadState::Active.is_terminal());
        assert!(!DownloadState::Idle.is_terminal());
        assert!(!DownloadState::Paused.is_terminal());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DownloadState::Idle), "Idle");
        assert_eq!(format!("{}", DownloadState::Active), "Active");
        assert_eq!(format!("{}", DownloadState::Paused), "Paused");
        assert_eq!(format!("{}", DownloadState::Complete), "Complete");
        assert_eq!(format!("{}", DownloadState::Failed), "Failed");
        assert_eq!(format!("{}", DownloadState::Stopped), "Stopped");
    }

    #[test]
    fn test_serialization() {
        let state = DownloadState::Active;
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("Active"));

        let deserialized: DownloadState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, DownloadState::Active);
    }
}
