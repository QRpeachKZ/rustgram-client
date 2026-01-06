//! Email verification
//!
//! This module handles email verification for Telegram authentication.
//! Based on TDLib's `EmailVerification` and `auth_SentCode` email types.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Sent email code
///
/// Information about a sent email verification code.
/// Based on TDLib's `SentEmailCode` type.
///
/// # Example
///
/// ```no_run
/// use rustgram_auth::SentEmailCode;
///
/// let code = SentEmailCode::new("e***@example.com".to_string(), 6);
/// assert_eq!(code.email_pattern(), "e***@example.com");
/// assert_eq!(code.code_length(), 6);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentEmailCode {
    /// Email address pattern (e.g., "e***@example.com")
    pub email_pattern: String,

    /// Length of the verification code
    pub code_length: i32,
}

impl SentEmailCode {
    /// Create a new sent email code
    ///
    /// # Validation
    ///
    /// If `code_length` is negative or >= 100, it will be set to 0.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auth::SentEmailCode;
    ///
    /// let code = SentEmailCode::new("e***@example.com".to_string(), 6);
    /// assert_eq!(code.email_pattern(), "e***@example.com");
    /// assert_eq!(code.code_length(), 6);
    /// ```
    pub fn new(email_pattern: String, code_length: i32) -> Self {
        // Validate code_length: 0 <= length < 100
        let code_length = if !(0..100).contains(&code_length) {
            0
        } else {
            code_length
        };

        Self {
            email_pattern,
            code_length,
        }
    }

    /// Get email pattern
    ///
    /// Returns the email address pattern (e.g., "e***@example.com").
    #[inline]
    pub fn email_pattern(&self) -> &str {
        &self.email_pattern
    }

    /// Get code length
    ///
    /// Returns the length of the verification code.
    pub const fn code_length(&self) -> i32 {
        self.code_length
    }

    /// Check if the email code is empty
    ///
    /// Returns `true` if the email pattern is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_auth::SentEmailCode;
    ///
    /// let code = SentEmailCode::new("e***@example.com".to_string(), 6);
    /// assert!(!code.is_empty());
    ///
    /// let empty = SentEmailCode::new(String::new(), 0);
    /// assert!(empty.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.email_pattern.is_empty()
    }
}

/// Deprecated type alias for backwards compatibility
///
/// # Deprecated
///
/// This type alias is deprecated since version 0.2.0.
/// Use [`SentEmailCode`] instead.
#[deprecated(since = "0.2.0", note = "Use SentEmailCode instead")]
pub type EmailCodeInfo = SentEmailCode;

/// Email verification
///
/// Represents an email verification that can be submitted.
/// Based on TDLib's `EmailVerification` type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailVerification {
    /// Verification via code
    Code {
        /// The verification code
        code: String,
    },

    /// Verification via Apple ID
    AppleId {
        /// Apple ID token
        token: String,
    },

    /// Verification via Google ID
    GoogleId {
        /// Google ID token
        token: String,
    },
}

impl fmt::Display for EmailVerification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Code { .. } => write!(f, "EmailCode"),
            Self::AppleId { .. } => write!(f, "AppleID"),
            Self::GoogleId { .. } => write!(f, "GoogleID"),
        }
    }
}

impl EmailVerification {
    /// Create email verification with code
    pub fn code(code: String) -> Self {
        Self::Code { code }
    }

    /// Create email verification with Apple ID
    pub fn apple_id(token: String) -> Self {
        Self::AppleId { token }
    }

    /// Create email verification with Google ID
    pub fn google_id(token: String) -> Self {
        Self::GoogleId { token }
    }

    /// Check if this is code-based verification
    pub const fn is_code(&self) -> bool {
        matches!(self, Self::Code { .. })
    }

    /// Check if this is Apple ID verification
    pub const fn is_apple_id(&self) -> bool {
        matches!(self, Self::AppleId { .. })
    }

    /// Check if this is Google ID verification
    pub const fn is_google_id(&self) -> bool {
        matches!(self, Self::GoogleId { .. })
    }

    /// Validate the verification data
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Code { code } => !code.is_empty() && code.len() <= 16,
            Self::AppleId { token } => !token.is_empty(),
            Self::GoogleId { token } => !token.is_empty(),
        }
    }

    /// Get the verification value
    pub fn value(&self) -> &str {
        match self {
            Self::Code { code } => code,
            Self::AppleId { token } => token,
            Self::GoogleId { token } => token,
        }
    }
}

/// Email address reset result
///
/// Result of attempting to reset email address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmailResetResult {
    /// Reset was successful
    Success,

    /// Reset is not available yet (try again later)
    NotAvailable,

    /// Reset failed
    Failed {
        /// Error reason
        reason: String,
    },
}

impl EmailResetResult {
    /// Check if reset was successful
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    /// Check if reset is temporarily unavailable
    pub const fn is_not_available(&self) -> bool {
        matches!(self, Self::NotAvailable)
    }
}

/// Email verification settings
///
/// Settings for email-based authentication.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailSettings {
    /// Allow Apple ID authentication
    pub allow_apple_id: bool,

    /// Allow Google ID authentication
    pub allow_google_id: bool,

    /// Email address (if set)
    pub email_address: Option<String>,
}

impl EmailSettings {
    /// Create default email settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable Apple ID authentication
    pub fn with_apple_id(mut self) -> Self {
        self.allow_apple_id = true;
        self
    }

    /// Enable Google ID authentication
    pub fn with_google_id(mut self) -> Self {
        self.allow_google_id = true;
        self
    }

    /// Set email address
    pub fn with_email(mut self, email: String) -> Self {
        self.email_address = Some(email);
        self
    }

    /// Check if any ID provider is allowed
    pub const fn has_id_provider(&self) -> bool {
        self.allow_apple_id || self.allow_google_id
    }

    /// Get email address
    pub fn email_address(&self) -> Option<&str> {
        self.email_address.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sent_email_code() {
        let code = SentEmailCode::new("e***@test.com".to_string(), 6);

        assert_eq!(code.email_pattern(), "e***@test.com");
        assert_eq!(code.code_length(), 6);
        assert!(!code.is_empty());
    }

    #[test]
    fn test_sent_email_code_validation() {
        // Test negative code_length
        let code = SentEmailCode::new("e***@test.com".to_string(), -1);
        assert_eq!(code.code_length(), 0);

        // Test code_length >= 100
        let code = SentEmailCode::new("e***@test.com".to_string(), 100);
        assert_eq!(code.code_length(), 0);

        // Test valid code_length
        let code = SentEmailCode::new("e***@test.com".to_string(), 99);
        assert_eq!(code.code_length(), 99);
    }

    #[test]
    fn test_sent_email_code_empty() {
        let code = SentEmailCode::new(String::new(), 0);
        assert!(code.is_empty());
        assert_eq!(code.email_pattern(), "");
    }

    #[test]
    fn test_email_verification_code() {
        let verification = EmailVerification::code("123456".to_string());

        assert!(verification.is_code());
        assert!(!verification.is_apple_id());
        assert!(!verification.is_google_id());
        assert!(verification.is_valid());
        assert_eq!(verification.value(), "123456");
    }

    #[test]
    fn test_email_verification_apple_id() {
        let verification = EmailVerification::apple_id("apple_token".to_string());

        assert!(!verification.is_code());
        assert!(verification.is_apple_id());
        assert!(!verification.is_google_id());
        assert!(verification.is_valid());
    }

    #[test]
    fn test_email_verification_invalid() {
        let verification = EmailVerification::code("".to_string());
        assert!(!verification.is_valid());
    }

    #[test]
    fn test_email_settings() {
        let settings = EmailSettings::new()
            .with_apple_id()
            .with_google_id()
            .with_email("test@example.com".to_string());

        assert!(settings.allow_apple_id);
        assert!(settings.allow_google_id);
        assert!(settings.has_id_provider());
        assert_eq!(settings.email_address(), Some("test@example.com"));
    }

    #[test]
    fn test_email_reset_result() {
        assert!(EmailResetResult::Success.is_success());
        assert!(EmailResetResult::NotAvailable.is_not_available());
        assert!(!EmailResetResult::Failed {
            reason: "test".to_string()
        }
        .is_success());
    }

    #[test]
    fn test_email_code_info_deprecated() {
        // Test that EmailCodeInfo still works as a type alias
        #[allow(deprecated)]
        let info = EmailCodeInfo::new("e***@test.com".to_string(), 6);
        assert_eq!(info.email_pattern(), "e***@test.com");
        assert_eq!(info.code_length(), 6);
    }
}
