// Copyright 2024 rustgram-client contributors
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

//! Error types for password manager.

use thiserror::Error;

/// Result type for password manager operations.
pub type Result<T> = std::result::Result<T, PasswordManagerError>;

/// Errors that can occur in password manager operations.
///
/// These errors correspond to various failure conditions in TDLib's
/// password management functionality.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PasswordManagerError {
    /// Invalid password provided
    #[error("Invalid password")]
    InvalidPassword,

    /// Password is too short (minimum {min} characters)
    #[error("Password is too short (minimum {min} characters)")]
    PasswordTooShort {
        /// Minimum required length
        min: usize,
    },

    /// Password hint is too long (maximum {max} characters)
    #[error("Password hint is too long (maximum {max} characters)")]
    HintTooLong {
        /// Maximum allowed length
        max: usize,
    },

    /// Invalid email address format
    #[error("Invalid email address: {email}")]
    InvalidEmail {
        /// The invalid email address
        email: String,
    },

    /// Invalid email verification code
    #[error("Invalid email verification code")]
    InvalidEmailCode,

    /// Email code has expired
    #[error("Email code has expired")]
    EmailCodeExpired,

    /// Invalid recovery code
    #[error("Invalid recovery code")]
    InvalidRecoveryCode,

    /// Recovery code has expired
    #[error("Recovery code has expired")]
    RecoveryCodeExpired,

    /// Password recovery not available
    #[error("Password recovery is not available")]
    RecoveryNotAvailable,

    /// Invalid state for this operation
    #[error("Invalid state for operation: {state}")]
    InvalidState {
        /// Current state
        state: String,
    },

    /// Temporary password not available
    #[error("Temporary password not available")]
    TempPasswordNotAvailable,

    /// Temporary password has expired
    #[error("Temporary password has expired")]
    TempPasswordExpired,

    /// Temporary password creation failed
    #[error("Failed to create temporary password: {reason}")]
    TempPasswordCreationFailed {
        /// Failure reason
        reason: String,
    },

    /// Passkey operation failed
    #[error("Passkey operation failed: {operation}")]
    PasskeyOperationFailed {
        /// Operation that failed
        operation: String,
    },

    /// Passkey not found
    #[error("Passkey not found: {id}")]
    PasskeyNotFound {
        /// Passkey ID
        id: String,
    },

    /// SRP computation failed
    #[error("SRP computation failed")]
    SrpComputationFailed,

    /// Invalid SRP parameters
    #[error("Invalid SRP parameters")]
    InvalidSrpParameters,

    /// Network error
    #[error("Network error: {message}")]
    Network {
        /// Error message
        message: String,
    },

    /// Internal error
    #[error("Internal error: {message}")]
    Internal {
        /// Error message
        message: String,
    },

    /// Operation cancelled
    #[error("Operation cancelled")]
    Cancelled,

    /// Rate limited (too many attempts)
    #[error("Rate limited: retry after {seconds} seconds")]
    RateLimited {
        /// Seconds to wait before retry
        seconds: u32,
    },
}

impl PasswordManagerError {
    /// Check if error is retryable
    pub const fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Network { .. } | Self::RateLimited { .. } | Self::Cancelled
        )
    }

    /// Check if error is temporary
    pub const fn is_temporary(&self) -> bool {
        matches!(
            self,
            Self::Network { .. }
                | Self::RateLimited { .. }
                | Self::EmailCodeExpired
                | Self::RecoveryCodeExpired
                | Self::TempPasswordExpired
        )
    }

    /// Check if error is due to invalid input
    pub const fn is_invalid_input(&self) -> bool {
        matches!(
            self,
            Self::InvalidPassword
                | Self::PasswordTooShort { .. }
                | Self::HintTooLong { .. }
                | Self::InvalidEmail { .. }
                | Self::InvalidEmailCode
                | Self::InvalidRecoveryCode
                | Self::InvalidSrpParameters
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = PasswordManagerError::InvalidPassword;
        assert_eq!(err.to_string(), "Invalid password");

        let err = PasswordManagerError::PasswordTooShort { min: 8 };
        assert!(err.to_string().contains("8"));
    }

    #[test]
    fn test_error_retryable() {
        assert!(PasswordManagerError::Network {
            message: "test".to_string()
        }
        .is_retryable());

        assert!(PasswordManagerError::RateLimited { seconds: 60 }.is_retryable());

        assert!(!PasswordManagerError::InvalidPassword.is_retryable());
    }

    #[test]
    fn test_error_temporary() {
        assert!(PasswordManagerError::Network {
            message: "test".to_string()
        }
        .is_temporary());

        assert!(PasswordManagerError::EmailCodeExpired.is_temporary());

        assert!(!PasswordManagerError::InvalidPassword.is_temporary());
    }

    #[test]
    fn test_error_invalid_input() {
        assert!(PasswordManagerError::InvalidPassword.is_invalid_input());

        assert!(PasswordManagerError::PasswordTooShort { min: 8 }.is_invalid_input());

        assert!(PasswordManagerError::InvalidEmail {
            email: "test".to_string()
        }
        .is_invalid_input());

        assert!(!PasswordManagerError::Network {
            message: "test".to_string()
        }
        .is_invalid_input());
    }

    #[test]
    fn test_error_equality() {
        let err1 = PasswordManagerError::InvalidPassword;
        let err2 = PasswordManagerError::InvalidPassword;
        assert_eq!(err1, err2);

        let err3 = PasswordManagerError::InvalidEmailCode;
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_error_clone() {
        let err1 = PasswordManagerError::RateLimited { seconds: 30 };
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }
}
