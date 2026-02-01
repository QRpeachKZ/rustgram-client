//! Password authentication (2FA/SRP)
//!
//! This module handles two-factor authentication using SRP (Secure Remote Password).
//! Based on TDLib's `PasswordManager` and `auth_Password` TL types.

use serde::{Deserialize, Serialize};
use std::fmt;

/// SRP (Secure Remote Password) KDF algorithm
///
/// Corresponds to the various `PasswordKdfAlgo` TL constructors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PasswordKdfAlgo {
    /// SHA256 + SHA512 + PBKDF2 + HMAC
    Sha256Sha512Pbkdf2Hmac {
        /// Salt 1
        salt1: Vec<u8>,
        /// Salt 2
        salt2: Vec<u8>,
        /// Number of iterations
        g: i32,
        /// p parameter
        p: Vec<u8>,
    },

    /// Unknown algorithm (for forward compatibility)
    Unknown {
        /// Algorithm name
        name: String,
    },
}

impl fmt::Display for PasswordKdfAlgo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sha256Sha512Pbkdf2Hmac { g, .. } => {
                write!(f, "SHA256+SHA512 PBKDF2 HMAC (g={})", g)
            }
            Self::Unknown { name } => write!(f, "Unknown({})", name),
        }
    }
}

impl PasswordKdfAlgo {
    /// Get the default SHA256+SHA512 PBKDF2 HMAC algorithm
    pub fn sha256_sha512_pbkdf2_hmac(salt1: Vec<u8>, salt2: Vec<u8>, g: i32, p: Vec<u8>) -> Self {
        Self::Sha256Sha512Pbkdf2Hmac { salt1, salt2, g, p }
    }

    /// Check if algorithm is supported
    pub const fn is_supported(&self) -> bool {
        matches!(self, Self::Sha256Sha512Pbkdf2Hmac { .. })
    }

    /// Get algorithm parameters
    #[allow(clippy::type_complexity)]
    pub fn parameters(&self) -> Option<(&Vec<u8>, &Vec<u8>, i32, &Vec<u8>)> {
        match self {
            Self::Sha256Sha512Pbkdf2Hmac { salt1, salt2, g, p } => Some((salt1, salt2, *g, p)),
            Self::Unknown { .. } => None,
        }
    }
}

/// Input check password SRP
///
/// Used for SRP-based password verification.
/// Corresponds to `inputCheckPasswordSRP` TL type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputCheckPasswordSrp {
    /// SRP ID
    pub srp_id: i64,

    /// SRP A parameter
    pub a: Vec<u8>,

    /// SRP M1 parameter
    pub m1: Vec<u8>,
}

impl InputCheckPasswordSrp {
    /// Create a new SRP password check
    pub fn new(srp_id: i64, a: Vec<u8>, m1: Vec<u8>) -> Self {
        Self { srp_id, a, m1 }
    }

    /// Get SRP ID
    pub const fn srp_id(&self) -> i64 {
        self.srp_id
    }

    /// Get SRP A parameter
    pub fn a(&self) -> &[u8] {
        &self.a
    }

    /// Get SRP M1 parameter
    pub fn m1(&self) -> &[u8] {
        &self.m1
    }

    /// Validate SRP parameters
    pub fn is_valid(&self) -> bool {
        !self.a.is_empty() && !self.m1.is_empty() && self.srp_id != 0
    }
}

/// Information about password (2FA) settings
///
/// Corresponds to TDLib's `PasswordInfo` and `auth_Password` TL types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordInfo {
    /// Whether password is enabled
    pub has_password: bool,

    /// Password hint
    pub hint: String,

    /// SRP parameters for verification
    pub srp_g: i32,

    /// SRP p parameter
    pub srp_p: Vec<u8>,

    /// SRP B parameter
    pub srp_b: Vec<u8>,

    /// SRP ID
    pub srp_id: i64,

    /// Current client salt
    pub current_client_salt: Vec<u8>,

    /// Current server salt
    pub current_server_salt: Vec<u8>,

    /// KDF algorithm
    pub algo: PasswordKdfAlgo,

    /// Whether password recovery is available
    pub has_recovery: bool,

    /// Whether secure values (passport) are set
    pub has_secure_values: bool,

    /// Email address pattern for recovery
    pub email_unconfirmed_pattern: Option<String>,
}

impl PasswordInfo {
    /// Create a new password info (no password set)
    pub fn no_password() -> Self {
        Self {
            has_password: false,
            hint: String::new(),
            srp_g: 0,
            srp_p: Vec::new(),
            srp_b: Vec::new(),
            srp_id: 0,
            current_client_salt: Vec::new(),
            current_server_salt: Vec::new(),
            algo: PasswordKdfAlgo::Unknown {
                name: "none".to_string(),
            },
            has_recovery: false,
            has_secure_values: false,
            email_unconfirmed_pattern: None,
        }
    }

    /// Create password info with password
    #[allow(clippy::too_many_arguments)]
    pub fn with_password(
        hint: String,
        srp_g: i32,
        srp_p: Vec<u8>,
        srp_b: Vec<u8>,
        srp_id: i64,
        current_client_salt: Vec<u8>,
        current_server_salt: Vec<u8>,
        algo: PasswordKdfAlgo,
        has_recovery: bool,
        has_secure_values: bool,
        email_unconfirmed_pattern: Option<String>,
    ) -> Self {
        Self {
            has_password: true,
            hint,
            srp_g,
            srp_p,
            srp_b,
            srp_id,
            current_client_salt,
            current_server_salt,
            algo,
            has_recovery,
            has_secure_values,
            email_unconfirmed_pattern,
        }
    }

    /// Check if account has password
    pub const fn has_password(&self) -> bool {
        self.has_password
    }

    /// Get password hint
    pub fn hint(&self) -> &str {
        &self.hint
    }

    /// Check if password recovery is available
    pub const fn has_recovery(&self) -> bool {
        self.has_recovery
    }

    /// Get email recovery pattern
    pub fn recovery_email(&self) -> Option<&str> {
        self.email_unconfirmed_pattern.as_deref()
    }

    /// Check if SRP parameters are valid
    pub fn has_valid_srp(&self) -> bool {
        self.srp_id != 0 && !self.srp_p.is_empty() && !self.srp_b.is_empty() && self.srp_g > 0
    }

    /// Get KDF algorithm
    pub fn algo(&self) -> &PasswordKdfAlgo {
        &self.algo
    }
}

impl Default for PasswordInfo {
    fn default() -> Self {
        Self::no_password()
    }
}

/// New password settings
///
/// Used when setting a new password.
/// Corresponds to `auth_passwordInputSettings` TL type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPasswordSettings {
    /// New password hint
    pub new_hint: String,

    /// New password hash
    pub new_password_hash: Vec<u8>,

    /// New salt for password hashing
    pub new_salt: Vec<u8>,

    /// KDF algorithm for new password
    pub new_algo: PasswordKdfAlgo,

    /// Email for recovery
    pub email: Option<String>,
}

impl NewPasswordSettings {
    /// Create new password settings
    pub fn new(
        hint: String,
        new_password_hash: Vec<u8>,
        new_salt: Vec<u8>,
        new_algo: PasswordKdfAlgo,
    ) -> Self {
        Self {
            new_hint: hint,
            new_password_hash,
            new_salt,
            new_algo,
            email: None,
        }
    }

    /// Add recovery email
    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// Get hint
    pub fn hint(&self) -> &str {
        &self.new_hint
    }

    /// Check if has recovery email
    pub const fn has_email(&self) -> bool {
        self.email.is_some()
    }

    /// Get recovery email
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}

/// Password recovery code info
///
/// Information about password recovery code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordRecoveryCode {
    /// Email pattern
    pub email_pattern: String,

    /// Code length
    pub code_length: u32,

    /// When the code was sent
    pub sent_at: i64,

    /// Expiration time
    pub expires_at: i64,
}

impl PasswordRecoveryCode {
    /// Create new password recovery code info
    pub fn new(email_pattern: String, code_length: u32, expires_in_seconds: i64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        Self {
            email_pattern,
            code_length,
            sent_at: now,
            expires_at: now + expires_in_seconds,
        }
    }

    /// Check if code has expired
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        now > self.expires_at
    }

    /// Get email pattern
    pub fn email_pattern(&self) -> &str {
        &self.email_pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdf_algo_display() {
        let algo = PasswordKdfAlgo::Sha256Sha512Pbkdf2Hmac {
            salt1: vec![1, 2, 3],
            salt2: vec![4, 5, 6],
            g: 2048,
            p: vec![7, 8, 9],
        };
        assert!(algo.to_string().contains("2048"));
    }

    #[test]
    fn test_kdf_algo_supported() {
        let algo = PasswordKdfAlgo::Sha256Sha512Pbkdf2Hmac {
            salt1: vec![1, 2, 3],
            salt2: vec![4, 5, 6],
            g: 2048,
            p: vec![7, 8, 9],
        };
        assert!(algo.is_supported());

        let unknown = PasswordKdfAlgo::Unknown {
            name: "unknown".to_string(),
        };
        assert!(!unknown.is_supported());
    }

    #[test]
    fn test_password_info_no_password() {
        let info = PasswordInfo::no_password();
        assert!(!info.has_password());
        assert!(!info.has_recovery());
        assert!(!info.has_valid_srp());
    }

    #[test]
    fn test_password_info_with_password() {
        let algo = PasswordKdfAlgo::Sha256Sha512Pbkdf2Hmac {
            salt1: vec![1, 2, 3],
            salt2: vec![4, 5, 6],
            g: 2048,
            p: vec![7, 8, 9],
        };

        let info = PasswordInfo::with_password(
            "my hint".to_string(),
            2048,
            vec![1, 2, 3],
            vec![4, 5, 6],
            12345,
            vec![7, 8],
            vec![9, 10],
            algo,
            true,
            false,
            Some("e***@test.com".to_string()),
        );

        assert!(info.has_password());
        assert!(info.has_recovery());
        assert!(info.has_valid_srp());
        assert_eq!(info.hint(), "my hint");
        assert_eq!(info.recovery_email(), Some("e***@test.com"));
    }

    #[test]
    fn test_new_password_settings() {
        let algo = PasswordKdfAlgo::Sha256Sha512Pbkdf2Hmac {
            salt1: vec![1, 2, 3],
            salt2: vec![4, 5, 6],
            g: 2048,
            p: vec![7, 8, 9],
        };

        let settings =
            NewPasswordSettings::new("new hint".to_string(), vec![1, 2, 3], vec![4, 5, 6], algo)
                .with_email("test@example.com".to_string());

        assert!(settings.has_email());
        assert_eq!(settings.hint(), "new hint");
        assert_eq!(settings.email(), Some("test@example.com"));
    }

    #[test]
    fn test_input_check_password_srp() {
        let check = InputCheckPasswordSrp::new(12345, vec![1, 2, 3], vec![4, 5, 6]);
        assert!(check.is_valid());
        assert_eq!(check.srp_id(), 12345);
    }

    #[test]
    fn test_input_check_password_srp_invalid() {
        let check = InputCheckPasswordSrp::new(0, vec![], vec![]);
        assert!(!check.is_valid());
    }
}
