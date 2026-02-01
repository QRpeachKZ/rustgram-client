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

//! Password manager state types.

use serde::{Deserialize, Serialize};
use std::fmt;

/// State of the password manager.
///
/// Represents the current state of 2FA password management.
/// Corresponds to TDLib's PasswordManager state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PasswordManagerState {
    /// No password state (idle)
    #[default]
    Idle,

    /// Getting password info
    GettingPasswordInfo,

    /// Setting password
    SettingPassword,

    /// Verifying password
    VerifyingPassword,

    /// Password recovery in progress
    RecoveringPassword,

    /// Setting recovery email
    SettingRecoveryEmail,

    /// Verifying recovery email
    VerifyingRecoveryEmail,

    /// Creating temporary password
    CreatingTempPassword,

    /// Registering passkey
    RegisteringPasskey,
}

impl fmt::Display for PasswordManagerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::GettingPasswordInfo => write!(f, "GettingPasswordInfo"),
            Self::SettingPassword => write!(f, "SettingPassword"),
            Self::VerifyingPassword => write!(f, "VerifyingPassword"),
            Self::RecoveringPassword => write!(f, "RecoveringPassword"),
            Self::SettingRecoveryEmail => write!(f, "SettingRecoveryEmail"),
            Self::VerifyingRecoveryEmail => write!(f, "VerifyingRecoveryEmail"),
            Self::CreatingTempPassword => write!(f, "CreatingTempPassword"),
            Self::RegisteringPasskey => write!(f, "RegisteringPasskey"),
        }
    }
}

impl PasswordManagerState {
    /// Check if state is idle (no operation in progress)
    pub const fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Check if state is busy (operation in progress)
    pub const fn is_busy(&self) -> bool {
        !matches!(self, Self::Idle)
    }

    /// Check if operation is in progress
    pub const fn is_operation_in_progress(&self) -> bool {
        matches!(
            self,
            Self::GettingPasswordInfo
                | Self::SettingPassword
                | Self::VerifyingPassword
                | Self::RecoveringPassword
                | Self::SettingRecoveryEmail
                | Self::VerifyingRecoveryEmail
                | Self::CreatingTempPassword
                | Self::RegisteringPasskey
        )
    }

    /// Check if password operation is in progress
    pub const fn is_password_operation(&self) -> bool {
        matches!(
            self,
            Self::SettingPassword | Self::VerifyingPassword | Self::RecoveringPassword
        )
    }

    /// Check if email operation is in progress
    pub const fn is_email_operation(&self) -> bool {
        matches!(
            self,
            Self::SettingRecoveryEmail | Self::VerifyingRecoveryEmail
        )
    }

    /// Check if temp password operation is in progress
    pub const fn is_temp_password_operation(&self) -> bool {
        matches!(self, Self::CreatingTempPassword)
    }

    /// Check if passkey operation is in progress
    pub const fn is_passkey_operation(&self) -> bool {
        matches!(self, Self::RegisteringPasskey)
    }
}

/// Password information.
///
/// Contains information about the current 2FA password settings.
/// Corresponds to TDLib's passwordInfo type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordInfo {
    /// Whether a password is set
    pub has_password: bool,

    /// Password hint
    pub hint: String,

    /// Whether password recovery is available
    pub has_recovery: bool,

    /// Recovery email pattern (e.g., "e***@gmail.com")
    pub recovery_email_pattern: Option<String>,

    /// Whether the user has a Telegram Passport
    pub has_passport: bool,

    /// SRP parameters for password verification
    pub srp_g: i32,
    /// SRP p parameter
    pub srp_p: Vec<u8>,
    /// SRP b parameter
    pub srp_b: Vec<u8>,
    /// SRP ID
    pub srp_id: i64,

    /// Current salts for SRP
    pub current_algo: Vec<u8>,
    /// Current salt
    pub current_salt: Vec<u8>,
    /// Secure salt
    pub secure_salt: Vec<u8>,
}

impl PasswordInfo {
    /// Create password info for account without password
    pub fn no_password() -> Self {
        Self {
            has_password: false,
            hint: String::new(),
            has_recovery: false,
            recovery_email_pattern: None,
            has_passport: false,
            srp_g: 0,
            srp_p: Vec::new(),
            srp_b: Vec::new(),
            srp_id: 0,
            current_algo: Vec::new(),
            current_salt: Vec::new(),
            secure_salt: Vec::new(),
        }
    }

    /// Create password info for account with password
    #[allow(clippy::too_many_arguments)]
    pub fn with_password(
        hint: String,
        has_recovery: bool,
        recovery_email_pattern: Option<String>,
        has_passport: bool,
        srp_g: i32,
        srp_p: Vec<u8>,
        srp_b: Vec<u8>,
        srp_id: i64,
        current_algo: Vec<u8>,
        current_salt: Vec<u8>,
        secure_salt: Vec<u8>,
    ) -> Self {
        Self {
            has_password: true,
            hint,
            has_recovery,
            recovery_email_pattern,
            has_passport,
            srp_g,
            srp_p,
            srp_b,
            srp_id,
            current_algo,
            current_salt,
            secure_salt,
        }
    }

    /// Check if SRP parameters are valid for verification
    pub fn has_valid_srp(&self) -> bool {
        self.srp_id != 0
            && !self.srp_p.is_empty()
            && !self.srp_b.is_empty()
            && self.srp_g > 0
            && !self.current_salt.is_empty()
    }

    /// Check if recovery is available
    pub const fn has_recovery(&self) -> bool {
        self.has_recovery
    }

    /// Get password hint
    pub fn hint(&self) -> &str {
        &self.hint
    }

    /// Get recovery email pattern
    pub fn recovery_email_pattern(&self) -> Option<&str> {
        self.recovery_email_pattern.as_deref()
    }
}

impl Default for PasswordInfo {
    fn default() -> Self {
        Self::no_password()
    }
}

/// Reset password result.
///
/// Result of password reset operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResetPasswordResult {
    /// Email pattern for recovery code
    pub email_pattern: String,

    /// Reset code length
    pub code_length: u32,

    /// Expiration time (Unix timestamp)
    pub expires_at: i32,
}

impl ResetPasswordResult {
    /// Create a new reset password result
    pub fn new(email_pattern: String, code_length: u32, expires_at: i32) -> Self {
        Self {
            email_pattern,
            code_length,
            expires_at,
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

    /// Get expiration time
    pub const fn expires_at(&self) -> i32 {
        self.expires_at
    }
}

/// Email address protection settings.
///
/// Controls how email address is protected in account settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum EmailAddressProtection {
    /// Email is not protected (visible to everyone)
    #[default]
    None,

    /// Email is protected by contacts only
    Contacts,

    /// Email is protected by password
    Password,

    /// Email is completely hidden
    Hidden,
}

impl fmt::Display for EmailAddressProtection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Contacts => write!(f, "Contacts"),
            Self::Password => write!(f, "Password"),
            Self::Hidden => write!(f, "Hidden"),
        }
    }
}

impl EmailAddressProtection {
    /// Check if email is visible
    pub const fn is_visible(&self) -> bool {
        matches!(self, Self::None | Self::Contacts)
    }

    /// Check if email requires password to view
    pub const fn requires_password(&self) -> bool {
        matches!(self, Self::Password)
    }

    /// Check if email is hidden
    pub const fn is_hidden(&self) -> bool {
        matches!(self, Self::Hidden)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_manager_state_display() {
        assert_eq!(PasswordManagerState::Idle.to_string(), "Idle");
        assert_eq!(
            PasswordManagerState::SettingPassword.to_string(),
            "SettingPassword"
        );
    }

    #[test]
    fn test_password_manager_state_checks() {
        assert!(PasswordManagerState::Idle.is_idle());
        assert!(!PasswordManagerState::Idle.is_busy());
        assert!(!PasswordManagerState::SettingPassword.is_idle());
        assert!(PasswordManagerState::SettingPassword.is_busy());
        assert!(PasswordManagerState::SettingPassword.is_password_operation());
        assert!(PasswordManagerState::SettingRecoveryEmail.is_email_operation());
    }

    #[test]
    fn test_password_info_no_password() {
        let info = PasswordInfo::no_password();
        assert!(!info.has_password);
        assert!(!info.has_recovery());
        assert!(!info.has_valid_srp());
        assert_eq!(info.hint(), "");
    }

    #[test]
    fn test_password_info_with_password() {
        let info = PasswordInfo::with_password(
            "my hint".to_string(),
            true,
            Some("e***@test.com".to_string()),
            false,
            2048,
            vec![1, 2, 3],
            vec![4, 5, 6],
            12345,
            vec![7, 8],
            vec![9, 10],
            vec![11, 12],
        );
        assert!(info.has_password);
        assert!(info.has_recovery());
        assert!(info.has_valid_srp());
        assert_eq!(info.hint(), "my hint");
        assert_eq!(info.recovery_email_pattern(), Some("e***@test.com"));
    }

    #[test]
    fn test_reset_password_result() {
        let result = ResetPasswordResult::new("e***@test.com".to_string(), 6, 1735795200);
        assert_eq!(result.email_pattern(), "e***@test.com");
        assert_eq!(result.code_length(), 6);
        assert_eq!(result.expires_at(), 1735795200);
    }

    #[test]
    fn test_email_address_protection() {
        assert!(EmailAddressProtection::None.is_visible());
        assert!(!EmailAddressProtection::None.requires_password());
        assert!(!EmailAddressProtection::None.is_hidden());

        assert!(EmailAddressProtection::Contacts.is_visible());
        assert!(!EmailAddressProtection::Contacts.requires_password());

        assert!(!EmailAddressProtection::Password.is_visible());
        assert!(EmailAddressProtection::Password.requires_password());

        assert!(!EmailAddressProtection::Hidden.is_visible());
        assert!(!EmailAddressProtection::Hidden.requires_password());
        assert!(EmailAddressProtection::Hidden.is_hidden());
    }

    #[test]
    fn test_password_info_default() {
        let info = PasswordInfo::default();
        assert_eq!(info, PasswordInfo::no_password());
    }

    #[test]
    fn test_password_manager_state_default() {
        let state = PasswordManagerState::default();
        assert_eq!(state, PasswordManagerState::Idle);
    }
}
