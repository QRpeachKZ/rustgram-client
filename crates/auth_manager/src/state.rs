// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Authentication state types
//!
//! This module defines the authentication state enum used by AuthManager.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Authentication state
///
/// Represents the current state of the authentication flow.
/// Based on TDLib's `AuthorizationState` and related types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum State {
    /// No authentication state
    #[default]
    None,

    /// Waiting for phone number
    WaitPhoneNumber,

    /// Waiting for authentication code
    WaitCode,

    /// Waiting for password (2FA)
    WaitPassword,

    /// Waiting for email code
    WaitEmailCode,

    /// Authentication successful
    Ok,

    /// Logging out
    LoggingOut,

    /// Closing connection
    Closing,

    /// Waiting for retry after error
    WaitingRetry {
        /// Number of retry attempts made
        attempts: u32,
        /// Delay before next retry
        delay: Duration,
    },

    /// Network error occurred
    NetworkError(String),
}

impl State {
    /// Check if this is a waiting state (expecting user input)
    pub const fn is_waiting(&self) -> bool {
        matches!(
            self,
            Self::WaitPhoneNumber | Self::WaitCode | Self::WaitPassword | Self::WaitEmailCode
        )
    }

    /// Check if authentication is complete
    pub const fn is_authorized(&self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Check if client is closing
    pub const fn is_closing(&self) -> bool {
        matches!(self, Self::LoggingOut | Self::Closing)
    }

    /// Check if this is an error state
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::WaitingRetry { .. } | Self::NetworkError(_))
    }

    /// Get the state as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::WaitPhoneNumber => "WaitPhoneNumber",
            Self::WaitCode => "WaitCode",
            Self::WaitPassword => "WaitPassword",
            Self::WaitEmailCode => "WaitEmailCode",
            Self::Ok => "Ok",
            Self::LoggingOut => "LoggingOut",
            Self::Closing => "Closing",
            Self::WaitingRetry { .. } => "WaitingRetry",
            Self::NetworkError(_) => "NetworkError",
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_is_waiting() {
        assert!(State::WaitPhoneNumber.is_waiting());
        assert!(State::WaitCode.is_waiting());
        assert!(State::WaitPassword.is_waiting());
        assert!(State::WaitEmailCode.is_waiting());
        assert!(!State::None.is_waiting());
        assert!(!State::Ok.is_waiting());
        assert!(!State::LoggingOut.is_waiting());
        assert!(!State::Closing.is_waiting());
    }

    #[test]
    fn test_state_is_authorized() {
        assert!(State::Ok.is_authorized());
        assert!(!State::None.is_authorized());
        assert!(!State::WaitCode.is_authorized());
    }

    #[test]
    fn test_state_is_closing() {
        assert!(State::LoggingOut.is_closing());
        assert!(State::Closing.is_closing());
        assert!(!State::Ok.is_closing());
        assert!(!State::None.is_closing());
    }

    #[test]
    fn test_state_as_str() {
        assert_eq!(State::None.as_str(), "None");
        assert_eq!(State::WaitPhoneNumber.as_str(), "WaitPhoneNumber");
        assert_eq!(State::WaitCode.as_str(), "WaitCode");
        assert_eq!(State::WaitPassword.as_str(), "WaitPassword");
        assert_eq!(State::WaitEmailCode.as_str(), "WaitEmailCode");
        assert_eq!(State::Ok.as_str(), "Ok");
        assert_eq!(State::LoggingOut.as_str(), "LoggingOut");
        assert_eq!(State::Closing.as_str(), "Closing");
    }

    #[test]
    fn test_state_display() {
        assert_eq!(format!("{}", State::None), "None");
        assert_eq!(format!("{}", State::Ok), "Ok");
        assert_eq!(format!("{}", State::WaitCode), "WaitCode");
    }

    #[test]
    fn test_state_default() {
        assert_eq!(State::default(), State::None);
    }

    #[test]
    fn test_state_is_error() {
        assert!(State::WaitingRetry {
            attempts: 1,
            delay: Duration::from_secs(5)
        }
        .is_error());
        assert!(State::NetworkError("test".to_string()).is_error());
        assert!(!State::Ok.is_error());
        assert!(!State::None.is_error());
    }

    #[test]
    fn test_state_as_str_new_variants() {
        assert_eq!(
            State::WaitingRetry {
                attempts: 1,
                delay: Duration::from_secs(5)
            }
            .as_str(),
            "WaitingRetry"
        );
        assert_eq!(
            State::NetworkError("test".to_string()).as_str(),
            "NetworkError"
        );
    }
}
