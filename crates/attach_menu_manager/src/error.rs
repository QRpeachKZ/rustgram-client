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

//! Error types for the attach menu manager.

use std::fmt;

/// Errors that can occur in the attach menu manager.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttachMenuManagerError {
    /// The manager is not active (not authorized or closing).
    NotActive,
    /// The bot cannot be added to the attachment menu.
    BotNotSupported,
    /// User information is not available.
    UserNotAccessible,
    /// Invalid response received.
    InvalidResponse,
    /// Wrong bot received (user ID mismatch).
    WrongBot,
    /// Invalid icon for attach menu bot.
    InvalidIcon,
    /// Serialization error.
    SerializationError,
    /// Cache version mismatch.
    CacheVersionMismatch,
    /// Other error with message.
    Other(String),
}

impl fmt::Display for AttachMenuManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotActive => write!(f, "Can't reload attachment menu bots"),
            Self::BotNotSupported => write!(f, "The bot can't be added to attachment menu"),
            Self::UserNotAccessible => write!(f, "Have no information about user"),
            Self::InvalidResponse => write!(f, "Receive invalid response"),
            Self::WrongBot => write!(f, "Receive wrong bot"),
            Self::InvalidIcon => write!(f, "Have no icon for attach menu bot"),
            Self::SerializationError => write!(f, "Serialization error"),
            Self::CacheVersionMismatch => write!(f, "Cache version mismatch"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AttachMenuManagerError {}

/// Result type for attach menu manager operations.
pub type Result<T> = std::result::Result<T, AttachMenuManagerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", AttachMenuManagerError::NotActive),
            "Can't reload attachment menu bots"
        );
        assert_eq!(
            format!("{}", AttachMenuManagerError::BotNotSupported),
            "The bot can't be added to attachment menu"
        );
        assert_eq!(
            format!("{}", AttachMenuManagerError::UserNotAccessible),
            "Have no information about user"
        );
        assert_eq!(
            format!("{}", AttachMenuManagerError::InvalidResponse),
            "Receive invalid response"
        );
        assert_eq!(
            format!("{}", AttachMenuManagerError::WrongBot),
            "Receive wrong bot"
        );
        assert_eq!(
            format!("{}", AttachMenuManagerError::InvalidIcon),
            "Have no icon for attach menu bot"
        );
        assert_eq!(
            format!("{}", AttachMenuManagerError::SerializationError),
            "Serialization error"
        );
        assert_eq!(
            format!("{}", AttachMenuManagerError::CacheVersionMismatch),
            "Cache version mismatch"
        );
        assert_eq!(
            format!(
                "{}",
                AttachMenuManagerError::Other("custom error".to_string())
            ),
            "custom error"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(
            AttachMenuManagerError::NotActive,
            AttachMenuManagerError::NotActive
        );
        assert_eq!(
            AttachMenuManagerError::BotNotSupported,
            AttachMenuManagerError::BotNotSupported
        );
        assert_ne!(
            AttachMenuManagerError::NotActive,
            AttachMenuManagerError::BotNotSupported
        );
    }

    #[test]
    fn test_error_clone() {
        let error = AttachMenuManagerError::NotActive;
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_result_type() {
        let ok_result: Result<()> = Ok(());
        assert!(ok_result.is_ok());

        let err_result: Result<()> = Err(AttachMenuManagerError::NotActive);
        assert!(err_result.is_err());
    }
}
