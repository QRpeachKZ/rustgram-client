//! QR code authentication
//!
//! This module handles QR code-based authentication for Telegram.
//! Based on TDLib's QR code authentication flow.

use serde::{Deserialize, Serialize};
use std::fmt;

/// QR code login session
///
/// Represents an active QR code authentication session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeLogin {
    /// Login token (embedded in QR code)
    pub login_token: Vec<u8>,

    /// List of user IDs that should not import the login
    pub other_user_ids: Vec<i64>,

    /// DC ID for the login
    pub dc_id: i32,

    /// When the token was created
    pub created_at: i64,

    /// Expiration time of the token
    pub expires_at: i64,

    /// Number of times token was exported
    pub exports: u32,
}

impl QrCodeLogin {
    /// Create a new QR code login session
    pub fn new(login_token: Vec<u8>, dc_id: i32, expires_in_seconds: i64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        Self {
            login_token,
            other_user_ids: Vec::new(),
            dc_id,
            created_at: now,
            expires_at: now + expires_in_seconds,
            exports: 0,
        }
    }

    /// Check if the QR code has expired
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        now > self.expires_at
    }

    /// Get remaining time until expiration (seconds)
    pub fn remaining_time(&self) -> i64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        (self.expires_at - now).max(0)
    }

    /// Get login token
    pub fn login_token(&self) -> &[u8] {
        &self.login_token
    }

    /// Get DC ID
    pub const fn dc_id(&self) -> i32 {
        self.dc_id
    }

    /// Add user ID to exclusion list
    pub fn add_other_user_id(mut self, user_id: i64) -> Self {
        self.other_user_ids.push(user_id);
        self
    }

    /// Increment export count
    pub fn increment_exports(&mut self) {
        self.exports += 1;
    }

    /// Get export count
    pub const fn exports(&self) -> u32 {
        self.exports
    }

    /// Generate QR code URL (tg://login?token=...)
    pub fn to_url(&self) -> String {
        // Encode token as base64
        use base64::prelude::*;
        let token_encoded = BASE64_STANDARD.encode(&self.login_token);
        format!("tg://login?token={}", token_encoded)
    }
}

/// QR code status
///
/// Current status of QR code authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QrCodeStatus {
    /// Waiting for QR code scan
    Waiting,

    /// QR code was scanned
    Scanned,

    /// QR code was rejected
    Rejected {
        /// Reason for rejection
        reason: String,
    },

    /// QR code authentication succeeded
    Success,

    /// QR code expired
    Expired,

    /// Unknown status
    Unknown,
}

impl fmt::Display for QrCodeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Waiting => write!(f, "Waiting"),
            Self::Scanned => write!(f, "Scanned"),
            Self::Rejected { reason } => write!(f, "Rejected: {}", reason),
            Self::Success => write!(f, "Success"),
            Self::Expired => write!(f, "Expired"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl QrCodeStatus {
    /// Check if QR code is still active
    pub const fn is_active(&self) -> bool {
        matches!(self, Self::Waiting | Self::Scanned)
    }

    /// Check if QR code was rejected
    pub const fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected { .. })
    }

    /// Check if QR code authentication succeeded
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    /// Check if QR code expired
    pub const fn is_expired(&self) -> bool {
        matches!(self, Self::Expired)
    }

    /// Create rejected status
    pub fn rejected(reason: String) -> Self {
        Self::Rejected { reason }
    }
}

/// Import QR code login token
///
/// Used when scanning another device's QR code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportQrCodeToken {
    /// Token to import
    pub token: Vec<u8>,

    /// DC ID of the login
    pub dc_id: i32,
}

impl ImportQrCodeToken {
    /// Create import token from URL
    ///
    /// Parses a tg://login URL and extracts the token.
    pub fn from_url(url: &str) -> Option<Self> {
        // Extract token from tg://login?token=BASE64
        use base64::prelude::*;
        let token_part = url.strip_prefix("tg://login?token=")?;
        let token = BASE64_STANDARD.decode(token_part).ok()?;

        // Extract DC ID from token (last 4 bytes)
        if token.len() < 4 {
            return None;
        }

        let dc_id_bytes = &token[token.len() - 4..];
        let dc_id = u32::from_le_bytes([
            dc_id_bytes[0],
            dc_id_bytes[1],
            dc_id_bytes[2],
            dc_id_bytes[3],
        ]) as i32;

        Some(Self { token, dc_id })
    }

    /// Create import token from raw data
    pub fn new(token: Vec<u8>, dc_id: i32) -> Self {
        Self { token, dc_id }
    }

    /// Get token
    pub fn token(&self) -> &[u8] {
        &self.token
    }

    /// Get DC ID
    pub const fn dc_id(&self) -> i32 {
        self.dc_id
    }

    /// Validate token format
    pub fn is_valid(&self) -> bool {
        !self.token.is_empty() && self.dc_id > 0
    }
}

/// QR code login result
///
/// Result of completing QR code authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeLoginResult {
    /// User ID that authenticated
    pub user_id: i64,

    /// First name
    pub first_name: String,

    /// Last name
    pub last_name: String,

    /// Authentication key
    pub auth_key: Vec<u8>,
}

impl QrCodeLoginResult {
    /// Create new login result
    pub fn new(user_id: i64, first_name: String, last_name: String, auth_key: Vec<u8>) -> Self {
        Self {
            user_id,
            first_name,
            last_name,
            auth_key,
        }
    }

    /// Get user ID
    pub const fn user_id(&self) -> i64 {
        self.user_id
    }

    /// Get full name
    pub fn full_name(&self) -> String {
        if self.last_name.is_empty() {
            self.first_name.clone()
        } else {
            format!("{} {}", self.first_name, self.last_name)
        }
    }

    /// Get auth key
    pub fn auth_key(&self) -> &[u8] {
        &self.auth_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_code_login_expiration() {
        let login = QrCodeLogin::new(vec![1, 2, 3, 4], 2, 300);
        assert!(!login.is_expired());
        assert!(login.remaining_time() >= 0);
    }

    #[test]
    fn test_qr_code_login_url() {
        let login = QrCodeLogin::new(vec![1, 2, 3, 4], 2, 300);
        let url = login.to_url();
        assert!(url.starts_with("tg://login?token="));
    }

    #[test]
    fn test_qr_code_status() {
        assert!(QrCodeStatus::Waiting.is_active());
        assert!(QrCodeStatus::Scanned.is_active());
        assert!(!QrCodeStatus::Success.is_active());
        assert!(QrCodeStatus::Success.is_success());
        assert!(QrCodeStatus::Rejected {
            reason: "test".to_string()
        }
        .is_rejected());
    }

    #[test]
    fn test_import_qr_code_token() {
        // Test with a valid token (base64 encoded)
        use base64::prelude::*;
        // DC ID 2 in little-endian: [2, 0, 0, 0]
        let token_bytes = vec![1, 2, 3, 4, 2, 0, 0, 0];
        let token_b64 = BASE64_STANDARD.encode(&token_bytes);
        let url = format!("tg://login?token={}", token_b64);

        let import = ImportQrCodeToken::from_url(&url).unwrap();
        assert!(import.is_valid());
        assert_eq!(import.dc_id(), 2);
    }

    #[test]
    fn test_qr_code_login_result() {
        let result = QrCodeLoginResult::new(
            123456,
            "John".to_string(),
            "Doe".to_string(),
            vec![1, 2, 3, 4],
        );

        assert_eq!(result.user_id(), 123456);
        assert_eq!(result.full_name(), "John Doe");
    }
}
