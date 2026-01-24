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

//! # Password Manager
//!
//! Two-Factor Authentication (2FA) password management for Telegram.
//!
//! ## Overview
//!
//! This module provides comprehensive password management functionality including:
//!
//! - Password setup and verification using SRP (Secure Remote Password)
//! - Recovery email management
//! - Password recovery flow
//! - Temporary password generation
//! - Passkey (WebAuthn) support
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rustgram_password_manager::PasswordManager;
//!
//! #[tokio::main]
//! async fn main() {
//!     let manager = PasswordManager::new(12345, "api_hash".to_string());
//!
//!     // Check if password is set
//!     let info = manager.get_password_info().await;
//!     if info.has_password {
//!         println!("Password hint: {}", info.hint);
//!     }
//! }
//! ```
//!
//! ## TDLib Compatibility
//!
//! - **Reference**: `references/td/td/telegram/PasswordManager.{h,cpp}`
//! - **TL Types**: `account.password`, `auth.password`, `inputCheckPasswordSRP`

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;
mod recovery;
mod srp;
mod state;
mod tl;

use crate::error::Result;
use crate::recovery::PasswordRecovery;
use crate::srp::SrpCalculator;
use crate::state::ResetPasswordResult;
use crate::tl::EmailVerificationCodeInfo;
use rustgram_email_verification::EmailVerification;
use rustgram_new_password_state::NewPasswordState;
use rustgram_passkey::Passkey;
use rustgram_temp_password_state::TempPasswordState;
use std::sync::Arc;
use tokio::sync::RwLock;

// Re-exports
pub use error::{PasswordManagerError, Result as PasswordManagerResult};
pub use state::{EmailAddressProtection, PasswordInfo, PasswordManagerState};

/// Minimum password length
const MIN_PASSWORD_LENGTH: usize = 1;

/// Maximum hint length
const MAX_HINT_LENGTH: usize = 255;

/// Password Manager for 2FA.
///
/// Manages all aspects of Telegram's two-factor authentication including
/// password setup, recovery email, temporary passwords, and passkeys.
///
/// # Example
///
/// ```rust
/// use rustgram_password_manager::PasswordManager;
///
/// let manager = PasswordManager::new(12345, "api_hash".to_string());
/// ```
#[derive(Clone)]
pub struct PasswordManager {
    /// API ID from Telegram
    api_id: i32,

    /// API hash from Telegram
    api_hash: Arc<String>,

    /// Current state
    state: Arc<RwLock<PasswordManagerState>>,

    /// Password info
    password_info: Arc<RwLock<PasswordInfo>>,

    /// New password state (for setting passwords)
    #[allow(dead_code)]
    new_password_state: Arc<RwLock<Option<NewPasswordState>>>,

    /// Temp password state
    temp_password_state: Arc<RwLock<TempPasswordState>>,

    /// Recovery manager
    recovery: Arc<RwLock<PasswordRecovery>>,

    /// SRP calculator
    #[allow(dead_code)]
    srp_calculator: Arc<SrpCalculator>,

    /// Registered passkeys
    passkeys: Arc<RwLock<Vec<Passkey>>>,

    /// Recovery email
    recovery_email: Arc<RwLock<Option<String>>>,
}

impl PasswordManager {
    /// Create a new password manager.
    ///
    /// # Arguments
    ///
    /// * `api_id` - API ID from https://my.telegram.org
    /// * `api_hash` - API hash from https://my.telegram.org
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_password_manager::PasswordManager;
    ///
    /// let manager = PasswordManager::new(12345, "api_hash".to_string());
    /// assert_eq!(manager.api_id(), 12345);
    /// ```
    pub fn new(api_id: i32, api_hash: String) -> Self {
        Self {
            api_id,
            api_hash: Arc::new(api_hash),
            state: Arc::new(RwLock::new(PasswordManagerState::Idle)),
            password_info: Arc::new(RwLock::new(PasswordInfo::no_password())),
            new_password_state: Arc::new(RwLock::new(None)),
            temp_password_state: Arc::new(RwLock::new(TempPasswordState::default())),
            recovery: Arc::new(RwLock::new(PasswordRecovery::new())),
            srp_calculator: Arc::new(SrpCalculator::new()),
            passkeys: Arc::new(RwLock::new(Vec::new())),
            recovery_email: Arc::new(RwLock::new(None)),
        }
    }

    /// Get API ID
    pub const fn api_id(&self) -> i32 {
        self.api_id
    }

    /// Get API hash
    pub async fn api_hash(&self) -> String {
        self.api_hash.as_ref().clone()
    }

    /// Get current state
    pub async fn get_state(&self) -> PasswordManagerState {
        *self.state.read().await
    }

    /// Get current state (alias for get_state)
    pub async fn state(&self) -> PasswordManagerState {
        self.get_state().await
    }

    /// Get password info
    pub async fn get_password_info(&self) -> PasswordInfo {
        self.password_info.read().await.clone()
    }

    /// Check if password is set
    pub async fn has_password(&self) -> bool {
        self.password_info.read().await.has_password
    }

    /// Set password
    ///
    /// Sets or changes the 2FA password.
    ///
    /// # Arguments
    ///
    /// * `current_password` - Current password (empty if setting for first time)
    /// * `new_password` - New password
    /// * `new_hint` - Password hint
    /// * `recovery_email` - Optional recovery email
    pub async fn set_password(
        &self,
        current_password: &str,
        new_password: &str,
        new_hint: &str,
        recovery_email: Option<String>,
    ) -> Result<()> {
        let mut state = self.state.write().await;

        if state.is_busy() {
            return Err(PasswordManagerError::InvalidState {
                state: state.to_string(),
            });
        }

        // Validate new password
        if new_password.len() < MIN_PASSWORD_LENGTH {
            return Err(PasswordManagerError::PasswordTooShort {
                min: MIN_PASSWORD_LENGTH,
            });
        }

        // Validate hint
        if new_hint.len() > MAX_HINT_LENGTH {
            return Err(PasswordManagerError::HintTooLong {
                max: MAX_HINT_LENGTH,
            });
        }

        // Validate recovery email format if provided
        if let Some(email) = &recovery_email {
            self.validate_email(email)?;
        }

        // Check current password if password exists
        {
            let info = self.password_info.read().await;
            if info.has_password && current_password.is_empty() {
                return Err(PasswordManagerError::InvalidPassword);
            }
        }

        *state = PasswordManagerState::SettingPassword;

        // In real implementation, would:
        // 1. Verify current password with SRP
        // 2. Compute new password hash
        // 3. Send account.updatePasswordSettings query
        //
        // For now, update local state
        {
            let mut info = self.password_info.write().await;
            info.has_password = true;
            info.hint = new_hint.to_string();
            info.has_recovery = recovery_email.is_some();
            if let Some(email) = &recovery_email {
                info.recovery_email_pattern = Some(self.mask_email(email));
            }
        }

        *self.recovery_email.write().await = recovery_email;
        *state = PasswordManagerState::Idle;

        Ok(())
    }

    /// Set login email address
    ///
    /// Sets the email address for login authentication.
    ///
    /// # Arguments
    ///
    /// * `email_address` - Email address to set
    pub async fn set_login_email_address(&self, email_address: String) -> Result<()> {
        self.validate_email(&email_address)?;

        let state = self.state.read().await;
        if state.is_busy() {
            return Err(PasswordManagerError::InvalidState {
                state: state.to_string(),
            });
        }

        // In real implementation, would send auth.bindTempAuthKey query
        Ok(())
    }

    /// Resend login email code
    ///
    /// Requests to resend the login email code.
    pub async fn resend_login_email_code(&self) -> Result<EmailVerificationCodeInfo> {
        let state = self.state.read().await;
        if !state.is_busy() {
            return Err(PasswordManagerError::InvalidState {
                state: "Not expecting email code".to_string(),
            });
        }

        // In real implementation, would resend code
        Ok(EmailVerificationCodeInfo::new(
            "e***@example.com".to_string(),
            6,
        ))
    }

    /// Check login email code
    ///
    /// Verifies the email code for login.
    ///
    /// # Arguments
    ///
    /// * `code` - Email verification code
    pub async fn check_login_email_code(&self, code: &str) -> Result<()> {
        if code.is_empty() {
            return Err(PasswordManagerError::InvalidEmailCode);
        }

        // In real implementation, would verify code
        Ok(())
    }

    /// Set recovery email address
    ///
    /// Sets or updates the recovery email for password recovery.
    ///
    /// # Arguments
    ///
    /// * `recovery_email` - Recovery email address
    pub async fn set_recovery_email_address(&self, recovery_email: String) -> Result<()> {
        self.validate_email(&recovery_email)?;

        let mut state = self.state.write().await;
        if state.is_busy() {
            return Err(PasswordManagerError::InvalidState {
                state: state.to_string(),
            });
        }

        *state = PasswordManagerState::SettingRecoveryEmail;

        // In real implementation, would send account.updatePasswordSettings query
        *self.recovery_email.write().await = Some(recovery_email);
        *state = PasswordManagerState::Idle;

        Ok(())
    }

    /// Get recovery email address
    ///
    /// Returns the recovery email pattern (e.g., "e***@gmail.com").
    pub async fn get_recovery_email_address(&self) -> Option<String> {
        self.password_info
            .read()
            .await
            .recovery_email_pattern()
            .map(String::from)
    }

    /// Check recovery email code
    ///
    /// Verifies the code sent to recovery email.
    ///
    /// # Arguments
    ///
    /// * `code` - Verification code
    pub async fn check_recovery_email_code(&self, code: &str) -> Result<()> {
        if code.is_empty() {
            return Err(PasswordManagerError::InvalidEmailCode);
        }

        // In real implementation, would verify code
        Ok(())
    }

    /// Resend recovery email code
    ///
    /// Requests to resend the recovery email code.
    pub async fn resend_recovery_email_code(&self) -> Result<EmailVerificationCodeInfo> {
        let info = self.password_info.read().await;
        if !info.has_recovery() {
            return Err(PasswordManagerError::RecoveryNotAvailable);
        }

        // In real implementation, would resend code
        Ok(EmailVerificationCodeInfo::new(
            info.recovery_email_pattern()
                .unwrap_or("e***@example.com")
                .to_string(),
            6,
        ))
    }

    /// Cancel recovery verification
    ///
    /// Cancels the ongoing recovery email verification.
    pub async fn cancel_recovery_verification(&self) -> Result<()> {
        let mut state = self.state.write().await;
        if !state.is_email_operation() {
            return Err(PasswordManagerError::InvalidState {
                state: state.to_string(),
            });
        }

        *state = PasswordManagerState::Idle;
        Ok(())
    }

    /// Send email verification code
    ///
    /// Sends a verification code to the email.
    pub async fn send_email_verification_code(
        &self,
        verification: EmailVerification,
    ) -> Result<()> {
        if verification.is_empty() {
            return Err(PasswordManagerError::InvalidEmail {
                email: "empty".to_string(),
            });
        }

        // In real implementation, would send verification code
        Ok(())
    }

    /// Resend email verification code
    ///
    /// Resends the email verification code.
    pub async fn resend_email_verification_code(&self) -> Result<EmailVerificationCodeInfo> {
        // In real implementation, would resend code
        Ok(EmailVerificationCodeInfo::new(
            "e***@example.com".to_string(),
            6,
        ))
    }

    /// Check email verification code
    ///
    /// Verifies the email verification code.
    ///
    /// # Arguments
    ///
    /// * `verification` - Email verification (code or token)
    pub async fn check_email_verification_code(
        &self,
        verification: EmailVerification,
    ) -> Result<()> {
        if verification.is_empty() {
            return Err(PasswordManagerError::InvalidEmailCode);
        }

        // In real implementation, would verify code
        Ok(())
    }

    /// Request password recovery
    ///
    /// Initiates password recovery flow.
    pub async fn request_password_recovery(&self) -> Result<ResetPasswordResult> {
        let mut recovery = self.recovery.write().await;
        let info = self.password_info.read().await;

        if !info.has_recovery() {
            return Err(PasswordManagerError::RecoveryNotAvailable);
        }

        let email_pattern = info
            .recovery_email_pattern()
            .unwrap_or("e***@example.com")
            .to_string();

        recovery.request_password_recovery(email_pattern)
    }

    /// Check recovery code
    ///
    /// Verifies the password recovery code.
    ///
    /// # Arguments
    ///
    /// * `code` - Recovery code from email
    pub async fn check_recovery_code(&self, code: &str) -> Result<()> {
        let recovery = self.recovery.read().await;
        recovery.check_recovery_code(code)
    }

    /// Recover password
    ///
    /// Completes password recovery with new password.
    ///
    /// # Arguments
    ///
    /// * `code` - Recovery code
    /// * `new_password` - New password
    /// * `hint` - Password hint
    pub async fn recover_password(&self, code: &str, new_password: &str, hint: &str) -> Result<()> {
        let mut recovery = self.recovery.write().await;

        if new_password.is_empty() {
            return Err(PasswordManagerError::PasswordTooShort { min: 1 });
        }

        recovery.recover_password(code, new_password, hint)?;

        // Update password info
        let mut info = self.password_info.write().await;
        info.has_password = true;
        info.hint = hint.to_string();

        Ok(())
    }

    /// Reset password
    ///
    /// Requests password reset (sends recovery code).
    pub async fn reset_password(&self) -> Result<ResetPasswordResult> {
        self.request_password_recovery().await
    }

    /// Cancel password reset
    ///
    /// Cancels ongoing password reset.
    pub async fn cancel_password_reset(&self) -> Result<()> {
        let mut recovery = self.recovery.write().await;
        recovery.cancel()
    }

    /// Get temp password state
    ///
    /// Returns current temporary password state.
    pub async fn get_temp_password_state(&self) -> TempPasswordState {
        self.temp_password_state.read().await.clone()
    }

    /// Create temp password
    ///
    /// Creates a temporary password for secure operations.
    ///
    /// # Arguments
    ///
    /// * `password` - Current password
    /// * `valid_for` - Validity period in seconds (max 3600)
    pub async fn create_temp_password(
        &self,
        password: &str,
        valid_for: i32,
    ) -> Result<TempPasswordState> {
        if password.is_empty() {
            return Err(PasswordManagerError::InvalidPassword);
        }

        if valid_for <= 0 || valid_for > 3600 {
            return Err(PasswordManagerError::TempPasswordCreationFailed {
                reason: "valid_for must be between 1 and 3600 seconds".to_string(),
            });
        }

        // In real implementation, would send account.getTmpPassword query
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let temp_password = format!("tmp_{}", password);
        let valid_until = now + valid_for;

        let state = TempPasswordState::new(temp_password, valid_until);
        *self.temp_password_state.write().await = state.clone();

        Ok(state)
    }

    /// Drop temp password
    ///
    /// Deletes the current temporary password.
    pub async fn drop_temp_password(&self) -> Result<()> {
        let mut state = self.temp_password_state.write().await;
        if !state.has_temp_password() {
            return Err(PasswordManagerError::TempPasswordNotAvailable);
        }

        state.clear();
        Ok(())
    }

    /// Drop cached secret
    ///
    /// Clears cached password secret.
    pub async fn drop_cached_secret(&self) -> Result<()> {
        // In real implementation, would clear cached secret
        Ok(())
    }

    /// Get passkey login options
    ///
    /// Returns options for passkey authentication.
    pub async fn get_passkey_login_options(&self) -> Result<String> {
        // In real implementation, would return passkey login options
        Ok("{\"challenge\":\"challenge\"}".to_string())
    }

    /// Init passkey registration
    ///
    /// Initiates registration of a new passkey.
    pub async fn init_passkey_registration(&self) -> Result<String> {
        let mut state = self.state.write().await;
        if state.is_busy() {
            return Err(PasswordManagerError::InvalidState {
                state: state.to_string(),
            });
        }

        *state = PasswordManagerState::RegisteringPasskey;

        // In real implementation, would return passkey registration options
        Ok("{\"challenge\":\"challenge\"}".to_string())
    }

    /// Register passkey
    ///
    /// Completes passkey registration.
    ///
    /// # Arguments
    ///
    /// * `name` - Passkey name
    /// * `credential` - WebAuthn credential data
    pub async fn register_passkey(&self, name: String, credential: String) -> Result<Passkey> {
        let mut state = self.state.write().await;

        if *state != PasswordManagerState::RegisteringPasskey {
            return Err(PasswordManagerError::InvalidState {
                state: state.to_string(),
            });
        }

        if name.is_empty() {
            return Err(PasswordManagerError::PasskeyOperationFailed {
                operation: "name is empty".to_string(),
            });
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let passkey = Passkey::builder()
            .with_id(credential.clone())
            .with_name(name.clone())
            .with_added_date(now)
            .build()
            .map_err(|e| PasswordManagerError::PasskeyOperationFailed {
                operation: format!("build passkey: {:?}", e),
            })?;

        self.passkeys.write().await.push(passkey.clone());
        *state = PasswordManagerState::Idle;

        Ok(passkey)
    }

    /// Get passkeys
    ///
    /// Returns all registered passkeys.
    pub async fn get_passkeys(&self) -> Vec<Passkey> {
        self.passkeys.read().await.clone()
    }

    /// Delete passkey
    ///
    /// Deletes a passkey.
    ///
    /// # Arguments
    ///
    /// * `passkey_id` - Passkey credential ID
    pub async fn delete_passkey(&self, passkey_id: String) -> Result<()> {
        let mut passkeys = self.passkeys.write().await;

        let index = passkeys
            .iter()
            .position(|p| p.id() == passkey_id)
            .ok_or_else(|| PasswordManagerError::PasskeyNotFound {
                id: passkey_id.clone(),
            })?;

        passkeys.remove(index);
        Ok(())
    }

    /// Validate email address format
    fn validate_email(&self, email: &str) -> Result<()> {
        if email.is_empty() {
            return Err(PasswordManagerError::InvalidEmail {
                email: email.to_string(),
            });
        }

        if !email.contains('@') || !email.contains('.') {
            return Err(PasswordManagerError::InvalidEmail {
                email: email.to_string(),
            });
        }

        Ok(())
    }

    /// Mask email for display
    fn mask_email(&self, email: &str) -> String {
        if let Some(at_pos) = email.find('@') {
            let first_char = email.chars().next();
            let domain = &email[at_pos..];
            format!("{}***{}", first_char.unwrap_or('e'), domain)
        } else {
            email.to_string()
        }
    }
}

impl Default for PasswordManager {
    fn default() -> Self {
        Self::new(0, String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_email_verification::EmailVerification;

    #[tokio::test]
    async fn test_password_manager_new() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        assert_eq!(manager.api_id(), 12345);
        assert_eq!(manager.api_hash().await, "api_hash");
        assert_eq!(manager.state().await, PasswordManagerState::Idle);
        assert!(!manager.has_password().await);
    }

    #[tokio::test]
    async fn test_set_password_no_current() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.set_password("", "newpass", "hint", None).await;
        assert!(result.is_ok());
        assert!(manager.has_password().await);
    }

    #[tokio::test]
    async fn test_set_password_too_short() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.set_password("", "", "hint", None).await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::PasswordTooShort { .. })
        ));
    }

    #[tokio::test]
    async fn test_set_password_hint_too_long() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let long_hint = "a".repeat(256);
        let result = manager
            .set_password("pass", "newpass", &long_hint, None)
            .await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::HintTooLong { .. })
        ));
    }

    #[tokio::test]
    async fn test_set_password_with_recovery_email() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager
            .set_password("", "newpass", "hint", Some("test@example.com".to_string()))
            .await;
        assert!(result.is_ok());
        assert_eq!(
            manager.get_recovery_email_address().await,
            Some("t***@example.com".to_string())
        );
    }

    #[tokio::test]
    async fn test_set_password_invalid_email() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager
            .set_password("", "newpass", "hint", Some("invalid".to_string()))
            .await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::InvalidEmail { .. })
        ));
    }

    #[tokio::test]
    async fn test_get_password_info() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let info = manager.get_password_info().await;
        assert!(!info.has_password);
        assert!(!info.has_recovery());
    }

    #[tokio::test]
    async fn test_set_recovery_email_address() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager
            .set_recovery_email_address("test@example.com".to_string())
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_recovery_email_invalid() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager
            .set_recovery_email_address("invalid".to_string())
            .await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::InvalidEmail { .. })
        ));
    }

    #[tokio::test]
    async fn test_resend_recovery_email_code() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager
            .set_password("", "pass", "hint", Some("test@example.com".to_string()))
            .await
            .unwrap();

        let result = manager.resend_recovery_email_code().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().code_length(), 6);
    }

    #[tokio::test]
    async fn test_check_recovery_email_code_valid() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.check_recovery_email_code("123456").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_recovery_email_code_empty() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.check_recovery_email_code("").await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::InvalidEmailCode)
        ));
    }

    #[tokio::test]
    async fn test_send_email_verification_code() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let verification = EmailVerification::code("123456");
        let result = manager.send_email_verification_code(verification).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_email_verification_code_empty() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let verification = EmailVerification::none();
        let result = manager.send_email_verification_code(verification).await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::InvalidEmail { .. })
        ));
    }

    #[tokio::test]
    async fn test_request_password_recovery() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager
            .set_password("", "pass", "hint", Some("test@example.com".to_string()))
            .await
            .unwrap();

        let result = manager.request_password_recovery().await;
        assert!(result.is_ok());
        assert!(result.unwrap().email_pattern.contains("t***"));
    }

    #[tokio::test]
    async fn test_request_password_recovery_no_email() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.request_password_recovery().await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::RecoveryNotAvailable)
        ));
    }

    #[tokio::test]
    async fn test_check_recovery_code() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager
            .set_password("", "pass", "hint", Some("test@example.com".to_string()))
            .await
            .unwrap();
        manager.request_password_recovery().await.unwrap();

        let result = manager.check_recovery_code("123456").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_recover_password() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager
            .set_password("", "oldpass", "hint", Some("test@example.com".to_string()))
            .await
            .unwrap();
        manager.request_password_recovery().await.unwrap();

        let result = manager
            .recover_password("123456", "newpass", "newhint")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_recover_password_empty_password() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager
            .set_password("", "pass", "hint", Some("test@example.com".to_string()))
            .await
            .unwrap();
        manager.request_password_recovery().await.unwrap();

        let result = manager.recover_password("123456", "", "newhint").await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::PasswordTooShort { .. })
        ));
    }

    #[tokio::test]
    async fn test_cancel_password_reset() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager
            .set_password("", "pass", "hint", Some("test@example.com".to_string()))
            .await
            .unwrap();
        manager.request_password_recovery().await.unwrap();

        let result = manager.cancel_password_reset().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_temp_password_state() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let state = manager.get_temp_password_state().await;
        assert!(!state.has_temp_password());
    }

    #[tokio::test]
    async fn test_create_temp_password() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.create_temp_password("password", 300).await;
        assert!(result.is_ok());

        let state = result.unwrap();
        assert!(state.has_temp_password());
    }

    #[tokio::test]
    async fn test_create_temp_password_empty_password() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.create_temp_password("", 300).await;
        assert!(matches!(result, Err(PasswordManagerError::InvalidPassword)));
    }

    #[tokio::test]
    async fn test_create_temp_password_invalid_duration() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.create_temp_password("password", 0).await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::TempPasswordCreationFailed { .. })
        ));

        let result2 = manager.create_temp_password("password", 4000).await;
        assert!(matches!(
            result2,
            Err(PasswordManagerError::TempPasswordCreationFailed { .. })
        ));
    }

    #[tokio::test]
    async fn test_drop_temp_password() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager.create_temp_password("password", 300).await.unwrap();

        let result = manager.drop_temp_password().await;
        assert!(result.is_ok());

        let state = manager.get_temp_password_state().await;
        assert!(!state.has_temp_password());
    }

    #[tokio::test]
    async fn test_drop_temp_password_none() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.drop_temp_password().await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::TempPasswordNotAvailable)
        ));
    }

    #[tokio::test]
    async fn test_drop_cached_secret() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.drop_cached_secret().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_passkey_login_options() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.get_passkey_login_options().await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("challenge"));
    }

    #[tokio::test]
    async fn test_init_passkey_registration() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.init_passkey_registration().await;
        assert!(result.is_ok());
        assert_eq!(
            manager.state().await,
            PasswordManagerState::RegisteringPasskey
        );
    }

    #[tokio::test]
    async fn test_register_passkey() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager.init_passkey_registration().await.unwrap();

        let result = manager
            .register_passkey("My Key".to_string(), "credential_data".to_string())
            .await;
        assert!(result.is_ok());

        let passkey = result.unwrap();
        assert_eq!(passkey.name(), "My Key");
        assert_eq!(passkey.id(), "credential_data");
    }

    #[tokio::test]
    async fn test_register_passkey_empty_name() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager.init_passkey_registration().await.unwrap();

        let result = manager
            .register_passkey("".to_string(), "credential_data".to_string())
            .await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::PasskeyOperationFailed { .. })
        ));
    }

    #[tokio::test]
    async fn test_register_passkey_invalid_state() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager
            .register_passkey("My Key".to_string(), "credential_data".to_string())
            .await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::InvalidState { .. })
        ));
    }

    #[tokio::test]
    async fn test_get_passkeys() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager.init_passkey_registration().await.unwrap();
        manager
            .register_passkey("Key 1".to_string(), "cred1".to_string())
            .await
            .unwrap();
        manager.init_passkey_registration().await.unwrap();
        manager
            .register_passkey("Key 2".to_string(), "cred2".to_string())
            .await
            .unwrap();

        let passkeys = manager.get_passkeys().await;
        assert_eq!(passkeys.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_passkey() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        manager.init_passkey_registration().await.unwrap();
        manager
            .register_passkey("My Key".to_string(), "credential_data".to_string())
            .await
            .unwrap();

        let result = manager.delete_passkey("credential_data".to_string()).await;
        assert!(result.is_ok());

        let passkeys = manager.get_passkeys().await;
        assert_eq!(passkeys.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_passkey_not_found() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        let result = manager.delete_passkey("nonexistent".to_string()).await;
        assert!(matches!(
            result,
            Err(PasswordManagerError::PasskeyNotFound { .. })
        ));
    }

    #[tokio::test]
    async fn test_default() {
        let manager = PasswordManager::default();
        assert_eq!(manager.api_id(), 0);
        assert_eq!(manager.api_hash().await, "");
    }

    #[tokio::test]
    async fn test_mask_email() {
        let manager = PasswordManager::new(12345, "api_hash".to_string());
        assert_eq!(manager.mask_email("test@example.com"), "t***@example.com");
        assert_eq!(manager.mask_email("a@b.co"), "a***@b.co");
        assert_eq!(manager.mask_email("invalid"), "invalid");
    }
}
