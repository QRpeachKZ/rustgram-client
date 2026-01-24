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

//! Error types for account management.

use thiserror::Error;

/// Errors that can occur during account management operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum AccountManagerError {
    /// Invalid session identifier.
    #[error("invalid session ID: {0}")]
    InvalidSessionId(i64),

    /// Invalid website identifier.
    #[error("invalid website ID: {0}")]
    InvalidWebsiteId(i64),

    /// Invalid QR code authentication link.
    #[error("invalid QR code authentication link: {0}")]
    InvalidQrCodeLink(String),

    /// Invalid base64 token in QR code.
    #[error("invalid base64 token in QR code")]
    InvalidBase64Token,

    /// Cannot terminate current session.
    #[error("cannot terminate current session")]
    CannotTerminateCurrentSession,

    /// Session not found.
    #[error("session not found: {0}")]
    SessionNotFound(i64),

    /// Website not found.
    #[error("website not found: {0}")]
    WebsiteNotFound(i64),

    /// Invalid TTL value.
    #[error("invalid TTL value: {0}, must be between {1} and {2}")]
    InvalidTtlValue(i32, i32, i32),

    /// Invalid contact token.
    #[error("invalid contact token")]
    InvalidContactToken,

    /// Network error.
    #[error("network error: {0}")]
    NetworkError(String),

    /// Serialization error.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Authorization not confirmed.
    #[error("authorization not confirmed: {0}")]
    AuthorizationNotConfirmed(i64),

    /// Invalid authorization parameters.
    #[error("invalid authorization parameters")]
    InvalidAuthorizationParameters,

    /// Age verification parameters error.
    #[error("age verification error: {0}")]
    AgeVerificationError(String),
}

/// Result type for account management operations.
pub type Result<T> = std::result::Result<T, AccountManagerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AccountManagerError::InvalidSessionId(123);
        assert_eq!(err.to_string(), "invalid session ID: 123");

        let err = AccountManagerError::InvalidQrCodeLink("bad_link".to_string());
        assert_eq!(
            err.to_string(),
            "invalid QR code authentication link: bad_link"
        );

        let err = AccountManagerError::CannotTerminateCurrentSession;
        assert_eq!(err.to_string(), "cannot terminate current session");
    }

    #[test]
    fn test_error_equality() {
        let err1 = AccountManagerError::InvalidSessionId(123);
        let err2 = AccountManagerError::InvalidSessionId(123);
        assert_eq!(err1, err2);

        let err3 = AccountManagerError::InvalidSessionId(456);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_invalid_ttl_value() {
        let err = AccountManagerError::InvalidTtlValue(500, 1, 366);
        assert_eq!(
            err.to_string(),
            "invalid TTL value: 500, must be between 1 and 366"
        );
    }
}
