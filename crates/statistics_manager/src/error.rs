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

//! Error types for StatisticsManager.

use std::fmt;

/// Errors that can occur in statistics operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Dialog does not exist or is not a channel.
    InvalidDialog,
    /// Message does not exist.
    InvalidMessage,
    /// Story does not exist.
    InvalidStory,
    /// Statistics are not available for this dialog.
    NotAvailable,
    /// Network error occurred.
    NetworkError(String),
    /// Rate limited by the server.
    RateLimited,
    /// Internal error occurred.
    Internal(String),
    /// Invalid password provided.
    InvalidPassword,
    /// Invalid parameter provided.
    InvalidParameter(String),
    /// Graph data not found.
    GraphNotFound,
    /// Cache error occurred.
    CacheError(String),
}

impl Error {
    /// Returns true if this is a network-related error.
    #[must_use]
    pub const fn is_network_error(&self) -> bool {
        matches!(self, Self::NetworkError(_))
    }

    /// Returns true if this is a rate limit error.
    #[must_use]
    pub const fn is_rate_limited(&self) -> bool {
        matches!(self, Self::RateLimited)
    }

    /// Returns true if this error is retryable.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::NetworkError(_) | Self::RateLimited | Self::Internal(_)
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDialog => write!(f, "Dialog does not exist or is not a channel"),
            Self::InvalidMessage => write!(f, "Message does not exist"),
            Self::InvalidStory => write!(f, "Story does not exist"),
            Self::NotAvailable => write!(f, "Statistics are not available for this dialog"),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::RateLimited => write!(f, "Rate limited by the server"),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
            Self::InvalidPassword => write!(f, "Invalid password provided"),
            Self::InvalidParameter(param) => write!(f, "Invalid parameter: {}", param),
            Self::GraphNotFound => write!(f, "Graph data not found"),
            Self::CacheError(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for statistics operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            Error::InvalidDialog.to_string(),
            "Dialog does not exist or is not a channel"
        );
        assert_eq!(
            Error::NetworkError("timeout".to_string()).to_string(),
            "Network error: timeout"
        );
        assert_eq!(
            Error::InvalidParameter("limit".to_string()).to_string(),
            "Invalid parameter: limit"
        );
    }

    #[test]
    fn test_is_network_error() {
        assert!(Error::NetworkError("test".to_string()).is_network_error());
        assert!(!Error::InvalidDialog.is_network_error());
        assert!(!Error::RateLimited.is_network_error());
    }

    #[test]
    fn test_is_rate_limited() {
        assert!(Error::RateLimited.is_rate_limited());
        assert!(!Error::NetworkError("test".to_string()).is_rate_limited());
        assert!(!Error::InvalidDialog.is_rate_limited());
    }

    #[test]
    fn test_is_retryable() {
        assert!(Error::NetworkError("test".to_string()).is_retryable());
        assert!(Error::RateLimited.is_retryable());
        assert!(Error::Internal("test".to_string()).is_retryable());
        assert!(!Error::InvalidDialog.is_retryable());
        assert!(!Error::InvalidMessage.is_retryable());
        assert!(!Error::NotAvailable.is_retryable());
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(Error::InvalidDialog, Error::InvalidDialog);
        assert_eq!(Error::RateLimited, Error::RateLimited);
        assert_ne!(
            Error::NetworkError("timeout".to_string()),
            Error::NetworkError("error".to_string())
        );
    }
}
