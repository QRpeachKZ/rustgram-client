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

//! TL type stubs for password manager.
//!
//! This module contains stub implementations for TL types that are
//! referenced by TDLib but not yet implemented in Rustgram.
//!
//! These stubs provide minimal functionality for compilation and
//! should be replaced with full implementations when the TL layer
//! is complete.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Stub for InputCheckPasswordSrp TL type.
///
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InputCheckPasswordSrp {
    /// SRP ID
    pub srp_id: i64,

    /// SRP A parameter
    pub a: Vec<u8>,

    /// SRP M1 parameter
    pub m1: Vec<u8>,
}

impl InputCheckPasswordSrp {
    /// Create a new InputCheckPasswordSrp
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
}

/// Stub for auth_passwordInputSettings TL type.
///
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordInputSettings {
    /// New password hash
    pub new_password_hash: Vec<u8>,

    /// New password hint
    pub new_hint: String,

    /// New salt
    pub new_salt: Vec<u8>,

    /// Email for recovery
    pub email: Option<String>,
}

impl PasswordInputSettings {
    /// Create new password input settings
    pub fn new(
        new_password_hash: Vec<u8>,
        new_hint: String,
        new_salt: Vec<u8>,
        email: Option<String>,
    ) -> Self {
        Self {
            new_password_hash,
            new_hint,
            new_salt,
            email,
        }
    }

    /// Get password hint
    pub fn hint(&self) -> &str {
        &self.new_hint
    }

    /// Get email
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}

/// Stub for passwordEmailVerificationCode TL type.
///
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailVerificationCodeInfo {
    /// Email pattern
    pub email_pattern: String,

    /// Code length
    pub code_length: u32,
}

impl EmailVerificationCodeInfo {
    /// Create new email verification code info
    pub fn new(email_pattern: String, code_length: u32) -> Self {
        Self {
            email_pattern,
            code_length,
        }
    }

    /// Get email pattern
    pub fn email_pattern(&self) -> &str {
        &self.email_pattern
    }

    /// Get code length
    pub const fn code_length(&self) -> u32 {
        self.code_length
    }
}

/// Stub for passkeyLoginRegistration TL type.
///
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyRegistrationOptions {
    /// Challenge data
    pub challenge: Vec<u8>,

    /// Relying party ID
    pub rp_id: String,

    /// User ID
    pub user_id: i64,

    /// User name
    pub user_name: String,
}

impl PasskeyRegistrationOptions {
    /// Create new passkey registration options
    pub fn new(challenge: Vec<u8>, rp_id: String, user_id: i64, user_name: String) -> Self {
        Self {
            challenge,
            rp_id,
            user_id,
            user_name,
        }
    }

    /// Get challenge
    pub fn challenge(&self) -> &[u8] {
        &self.challenge
    }

    /// Get RP ID
    pub fn rp_id(&self) -> &str {
        &self.rp_id
    }
}

/// Stub for account.takePasswordInfo TL type.
///
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TakePasswordInfoFlags {
    /// Take password info
    Take = 0,
}

/// Stub for account.getPasswordSettings TL type.
///
/// TODO: Replace with full TL implementation when available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordSettings {
    /// Email for recovery
    pub email: Option<String>,
}

impl PasswordSettings {
    /// Create new password settings
    pub fn new(email: Option<String>) -> Self {
        Self { email }
    }

    /// Get email
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_check_password_srp() {
        let srp = InputCheckPasswordSrp::new(12345, vec![1, 2, 3], vec![4, 5, 6]);
        assert_eq!(srp.srp_id(), 12345);
        assert_eq!(srp.a(), &[1, 2, 3]);
        assert_eq!(srp.m1(), &[4, 5, 6]);
    }

    #[test]
    fn test_password_input_settings() {
        let settings = PasswordInputSettings::new(
            vec![1, 2, 3],
            "my hint".to_string(),
            vec![4, 5, 6],
            Some("test@example.com".to_string()),
        );
        assert_eq!(settings.hint(), "my hint");
        assert_eq!(settings.email(), Some("test@example.com"));
    }

    #[test]
    fn test_password_input_settings_no_email() {
        let settings =
            PasswordInputSettings::new(vec![1, 2, 3], "my hint".to_string(), vec![4, 5, 6], None);
        assert_eq!(settings.hint(), "my hint");
        assert!(settings.email().is_none());
    }

    #[test]
    fn test_email_verification_code_info() {
        let info = EmailVerificationCodeInfo::new("e***@test.com".to_string(), 6);
        assert_eq!(info.email_pattern(), "e***@test.com");
        assert_eq!(info.code_length(), 6);
    }

    #[test]
    fn test_passkey_registration_options() {
        let options = PasskeyRegistrationOptions::new(
            vec![1, 2, 3],
            "example.com".to_string(),
            12345,
            "user".to_string(),
        );
        assert_eq!(options.challenge(), &[1, 2, 3]);
        assert_eq!(options.rp_id(), "example.com");
    }

    #[test]
    fn test_password_settings() {
        let settings = PasswordSettings::new(Some("test@example.com".to_string()));
        assert_eq!(settings.email(), Some("test@example.com"));
    }

    #[test]
    fn test_password_settings_no_email() {
        let settings = PasswordSettings::new(None);
        assert!(settings.email().is_none());
    }

    #[test]
    fn test_take_password_info_flags() {
        let flags = TakePasswordInfoFlags::Take;
        let _ = flags;
    }
}
