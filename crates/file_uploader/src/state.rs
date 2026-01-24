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

//! Upload state for file uploads.

use serde::{Deserialize, Serialize};
use std::fmt;

/// State of a file upload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum UploadState {
    /// Upload has not started
    #[default]
    Idle,
    /// Upload is active
    Active,
    /// Upload is paused
    Paused,
    /// Upload completed successfully
    Complete,
    /// Upload failed
    Failed,
    /// Upload was stopped
    Stopped,
}

impl UploadState {
    /// Returns `true` if the upload is idle.
    #[must_use]
    pub const fn is_idle(self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Returns `true` if the upload is active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns `true` if the upload is paused.
    #[must_use]
    pub const fn is_paused(self) -> bool {
        matches!(self, Self::Paused)
    }

    /// Returns `true` if the upload is complete.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns `true` if the upload failed.
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns `true` if the upload is stopped.
    #[must_use]
    pub const fn is_stopped(self) -> bool {
        matches!(self, Self::Stopped)
    }

    /// Returns `true` if the upload is in a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Complete | Self::Failed | Self::Stopped)
    }
}

impl fmt::Display for UploadState {
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
        let state = UploadState::default();
        assert_eq!(state, UploadState::Idle);
    }

    #[test]
    fn test_is_idle() {
        assert!(UploadState::Idle.is_idle());
        assert!(!UploadState::Active.is_idle());
    }

    #[test]
    fn test_is_active() {
        assert!(UploadState::Active.is_active());
        assert!(!UploadState::Idle.is_active());
    }

    #[test]
    fn test_is_paused() {
        assert!(UploadState::Paused.is_paused());
        assert!(!UploadState::Active.is_paused());
    }

    #[test]
    fn test_is_complete() {
        assert!(UploadState::Complete.is_complete());
        assert!(!UploadState::Active.is_complete());
    }

    #[test]
    fn test_is_failed() {
        assert!(UploadState::Failed.is_failed());
        assert!(!UploadState::Active.is_failed());
    }

    #[test]
    fn test_is_stopped() {
        assert!(UploadState::Stopped.is_stopped());
        assert!(!UploadState::Active.is_stopped());
    }

    #[test]
    fn test_is_terminal() {
        assert!(UploadState::Complete.is_terminal());
        assert!(UploadState::Failed.is_terminal());
        assert!(UploadState::Stopped.is_terminal());
        assert!(!UploadState::Active.is_terminal());
        assert!(!UploadState::Idle.is_terminal());
        assert!(!UploadState::Paused.is_terminal());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", UploadState::Idle), "Idle");
        assert_eq!(format!("{}", UploadState::Active), "Active");
        assert_eq!(format!("{}", UploadState::Paused), "Paused");
        assert_eq!(format!("{}", UploadState::Complete), "Complete");
        assert_eq!(format!("{}", UploadState::Failed), "Failed");
        assert_eq!(format!("{}", UploadState::Stopped), "Stopped");
    }

    #[test]
    fn test_serialization() {
        let state = UploadState::Active;
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("Active"));

        let deserialized: UploadState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, UploadState::Active);
    }
}
