//! Authentication error types
//!
//! Comprehensive error handling for authentication operations.
//! Based on TDLib's error handling patterns.

use thiserror::Error;

/// Authentication errors
///
/// Covers all error types that can occur during the authentication flow,
/// including network errors, protocol errors, and user input validation.
#[derive(Error, Debug)]
pub enum AuthError {
    /// Invalid phone number format
    #[error("Invalid phone number: {0}")]
    InvalidPhone(String),

    /// Invalid bot token format
    #[error("Invalid bot token: {0}")]
    InvalidBotToken(String),

    /// Network error during authentication
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Server rejected the authentication code
    #[error("Invalid authentication code")]
    InvalidCode,

    /// Code has expired
    #[error("Authentication code expired")]
    CodeExpired,

    /// Rate limited by server
    #[error("Rate limited: retry after {0}s")]
    RateLimited(u32),

    /// Invalid password (2FA)
    #[error("Invalid password")]
    InvalidPassword,

    /// Password is required for this account
    #[error("Password required")]
    PasswordRequired,

    /// DH parameter negotiation failed
    #[error("DH negotiation failed: {0}")]
    DhError(String),

    /// Session expired
    #[error("Session expired")]
    SessionExpired,

    /// Account already exists (during registration)
    #[error("Account already exists")]
    AccountAlreadyExists,

    /// Account does not exist (during login)
    #[error("Account does not exist")]
    AccountNotFound,

    /// Invalid API credentials
    #[error("Invalid API credentials: api_id={0}")]
    InvalidApiCredentials(i32),

    /// Phone number flood waited
    #[error("Phone number flood: try again later")]
    PhoneFlood,

    /// Invalid email address
    #[error("Invalid email address: {0}")]
    InvalidEmail(String),

    /// Email verification failed
    #[error("Email verification failed: {0}")]
    EmailVerificationFailed(String),

    /// QR code authentication failed
    #[error("QR code authentication failed: {0}")]
    QrCodeError(String),

    /// Terms of service not accepted
    #[error("Terms of service must be accepted")]
    TermsNotAccepted,

    /// Internal authentication state error
    #[error("Internal state error: {0}")]
    InternalError(String),

    /// Operation not allowed in current state
    #[error("Operation not allowed in current state: {state}")]
    InvalidState {
        /// Current state
        state: String,
    },

    /// Timeout waiting for response
    #[error("Operation timed out")]
    Timeout,

    /// Request cancelled
    #[error("Request cancelled")]
    Cancelled,

    /// Unknown error from server
    #[error("Server error: code={code}, message={message}")]
    ServerError {
        /// Error code
        code: i32,
        /// Error message
        message: String,
    },
}

/// Result type for authentication operations
pub type AuthResult<T> = Result<T, AuthError>;

impl AuthError {
    /// Check if error is retryable
    pub const fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::NetworkError(_) | Self::RateLimited(_) | Self::Timeout
        )
    }

    /// Check if error is fatal (requires restart)
    pub const fn is_fatal(&self) -> bool {
        matches!(
            self,
            Self::InvalidApiCredentials(_) | Self::InternalError(_)
        )
    }

    /// Check if error is due to invalid user input
    pub const fn is_user_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidPhone(_)
                | Self::InvalidCode
                | Self::InvalidPassword
                | Self::InvalidBotToken(_)
                | Self::InvalidEmail(_)
        )
    }

    /// Create error from server error code and message
    ///
    /// Maps Telegram server errors to [`AuthError`] variants.
    pub fn from_server_error(code: i32, message: String) -> Self {
        match code {
            400 => match message.as_str() {
                "PHONE_NUMBER_INVALID" | "PHONE_CODE_INVALID" => Self::InvalidPhone(message),
                "PHONE_CODE_EXPIRED" => Self::CodeExpired,
                "PHONE_NUMBER_FLOOD" => Self::PhoneFlood,
                "SESSION_PASSWORD_NEEDED" => Self::PasswordRequired,
                "SESSION_PASSWORD_INVALID" => Self::InvalidPassword,
                "PHONE_NUMBER_OCCUPIED" => Self::AccountAlreadyExists,
                "PHONE_NUMBER_UNOCCUPIED" => Self::AccountNotFound,
                "EMAIL_INVALID" => Self::InvalidEmail(message),
                "EMAIL_VERIFY_EXPIRED" => Self::CodeExpired,
                _ => Self::ServerError { code, message },
            },
            401 => Self::SessionExpired,
            429 => {
                // Parse retry_after from message if available
                let retry_after = message
                    .split("FLOOD_WAIT_")
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60);
                Self::RateLimited(retry_after)
            }
            500 => Self::NetworkError(message),
            _ => Self::ServerError { code, message },
        }
    }

    /// Get error code for serialization
    pub const fn error_code(&self) -> i32 {
        match self {
            Self::InvalidPhone(_) => 400,
            Self::InvalidBotToken(_) => 401,
            Self::NetworkError(_) => 500,
            Self::InvalidCode => 400,
            Self::CodeExpired => 400,
            Self::RateLimited(_) => 429,
            Self::InvalidPassword => 400,
            Self::PasswordRequired => 401,
            Self::DhError(_) => 400,
            Self::SessionExpired => 401,
            Self::AccountAlreadyExists => 400,
            Self::AccountNotFound => 400,
            Self::InvalidApiCredentials(_) => 401,
            Self::PhoneFlood => 429,
            Self::InvalidEmail(_) => 400,
            Self::EmailVerificationFailed(_) => 400,
            Self::QrCodeError(_) => 400,
            Self::TermsNotAccepted => 403,
            Self::InternalError(_) => 500,
            Self::InvalidState { .. } => 422,
            Self::Timeout => 408,
            Self::Cancelled => 499,
            Self::ServerError { code, .. } => *code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AuthError::InvalidPhone("+123".to_string());
        assert!(err.to_string().contains("Invalid phone"));

        let err = AuthError::RateLimited(60);
        assert!(err.to_string().contains("60"));
    }

    #[test]
    fn test_error_classification() {
        assert!(AuthError::NetworkError("test".to_string()).is_retryable());
        assert!(AuthError::RateLimited(30).is_retryable());
        assert!(AuthError::Timeout.is_retryable());

        assert!(!AuthError::InvalidCode.is_retryable());

        assert!(AuthError::InvalidApiCredentials(12345).is_fatal());
        assert!(AuthError::InternalError("test".to_string()).is_fatal());

        assert!(AuthError::InvalidPhone("+123".to_string()).is_user_error());
        assert!(AuthError::InvalidPassword.is_user_error());
    }

    #[test]
    fn test_server_error_parsing() {
        let err = AuthError::from_server_error(400, "PHONE_CODE_INVALID".to_string());
        assert!(matches!(err, AuthError::InvalidPhone(_)));

        let err = AuthError::from_server_error(429, "FLOOD_WAIT_60".to_string());
        assert!(matches!(err, AuthError::RateLimited(60)));

        // SESSION_PASSWORD_NEEDED comes with code 400
        let err = AuthError::from_server_error(400, "SESSION_PASSWORD_NEEDED".to_string());
        assert!(matches!(err, AuthError::PasswordRequired));

        // 401 is session expired
        let err = AuthError::from_server_error(401, "SESSION_PASSWORD_NEEDED".to_string());
        assert!(matches!(err, AuthError::SessionExpired));
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(
            AuthError::InvalidPhone("+123".to_string()).error_code(),
            400
        );
        assert_eq!(AuthError::RateLimited(60).error_code(), 429);
        assert_eq!(AuthError::Timeout.error_code(), 408);
    }
}
