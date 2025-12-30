//! Authentication error types

use thiserror::Error;

/// Authentication errors
#[derive(Error, Debug)]
pub enum AuthError {
    /// Invalid phone number format
    #[error("Invalid phone number: {0}")]
    InvalidPhone(String),

    /// Network error during authentication
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Server rejected the authentication code
    #[error("Invalid authentication code")]
    InvalidCode,

    /// DH parameter negotiation failed
    #[error("DH negotiation failed: {0}")]
    DhError(String),

    /// Rate limited by server
    #[error("Rate limited: retry after {0}s")]
    RateLimited(u32),

    /// Internal authentication state error
    #[error("Internal state error: {0}")]
    InternalError(String),
}

/// Result type for authentication operations
pub type AuthResult<T> = Result<T, AuthError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AuthError::InvalidPhone("+123".to_string());
        assert!(err.to_string().contains("Invalid phone"));
    }
}
