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

//! Password recovery functionality.

use crate::error::{PasswordManagerError, Result};
use crate::state::ResetPasswordResult;
use std::time::{SystemTime, UNIX_EPOCH};

/// Password recovery manager.
///
/// Handles password recovery flow including requesting recovery codes
/// and resetting passwords.
#[derive(Debug, Clone)]
pub struct PasswordRecovery {
    /// Email pattern for recovery
    email_pattern: Option<String>,

    /// Recovery code
    recovery_code: Option<String>,

    /// Code length
    code_length: u32,

    /// Expiration time
    expires_at: Option<i32>,

    /// Recovery state
    state: RecoveryState,
}

/// State of password recovery process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RecoveryState {
    /// No recovery in progress
    #[default]
    Idle,

    /// Recovery code sent
    CodeSent,

    /// Recovery verified
    #[allow(dead_code)]
    Verified,

    /// Recovery completed
    Completed,
}

impl PasswordRecovery {
    /// Create a new password recovery manager
    pub fn new() -> Self {
        Self {
            email_pattern: None,
            recovery_code: None,
            code_length: 6,
            expires_at: None,
            state: RecoveryState::Idle,
        }
    }

    /// Request password recovery
    ///
    /// Initiates password recovery by sending a code to the recovery email.
    ///
    /// # Arguments
    ///
    /// * `email_pattern` - Email address pattern (e.g., "e***@gmail.com")
    ///
    /// # Returns
    ///
    /// Information about the reset including email pattern and expiration
    pub fn request_password_recovery(
        &mut self,
        email_pattern: String,
    ) -> Result<ResetPasswordResult> {
        if self.state != RecoveryState::Idle {
            return Err(PasswordManagerError::InvalidState {
                state: format!("{:?}", self.state),
            });
        }

        if email_pattern.is_empty() {
            return Err(PasswordManagerError::InvalidEmail {
                email: email_pattern,
            });
        }

        let now = Self::current_time();
        let expires_at = now + 3600; // 1 hour

        self.email_pattern = Some(email_pattern.clone());
        self.code_length = 6;
        self.expires_at = Some(expires_at);
        self.state = RecoveryState::CodeSent;

        Ok(ResetPasswordResult::new(
            email_pattern,
            self.code_length,
            expires_at,
        ))
    }

    /// Check recovery code
    ///
    /// Verifies the recovery code is valid.
    ///
    /// # Arguments
    ///
    /// * `code` - Recovery code from email
    pub fn check_recovery_code(&self, code: &str) -> Result<()> {
        if self.state != RecoveryState::CodeSent {
            return Err(PasswordManagerError::InvalidState {
                state: format!("{:?}", self.state),
            });
        }

        if code.is_empty() {
            return Err(PasswordManagerError::InvalidRecoveryCode);
        }

        // Check expiration
        if let Some(expires_at) = self.expires_at {
            let now = Self::current_time();
            if now > expires_at {
                return Err(PasswordManagerError::RecoveryCodeExpired);
            }
        }

        // In real implementation, would validate code against server
        // For now, just check length
        if code.len() != self.code_length as usize {
            return Err(PasswordManagerError::InvalidRecoveryCode);
        }

        Ok(())
    }

    /// Recover password
    ///
    /// Completes password recovery with new password.
    ///
    /// # Arguments
    ///
    /// * `code` - Recovery code from email
    /// * `new_password` - New password to set
    /// * `hint` - Password hint
    pub fn recover_password(&mut self, code: &str, new_password: &str, _hint: &str) -> Result<()> {
        self.check_recovery_code(code)?;

        if new_password.is_empty() {
            return Err(PasswordManagerError::PasswordTooShort { min: 1 });
        }

        // In real implementation, would send new password to server
        self.state = RecoveryState::Completed;
        self.recovery_code = Some(code.to_string());

        Ok(())
    }

    /// Cancel password recovery
    ///
    /// Cancels the password recovery process.
    pub fn cancel(&mut self) -> Result<()> {
        if self.state == RecoveryState::Completed {
            return Err(PasswordManagerError::InvalidState {
                state: "Cannot cancel completed recovery".to_string(),
            });
        }

        self.reset();
        Ok(())
    }

    /// Get recovery state
    #[allow(dead_code)]
    pub const fn state(&self) -> RecoveryState {
        self.state
    }

    /// Get email pattern
    #[allow(dead_code)]
    pub fn email_pattern(&self) -> Option<&str> {
        self.email_pattern.as_deref()
    }

    /// Get expiration time
    #[allow(dead_code)]
    pub const fn expires_at(&self) -> Option<i32> {
        self.expires_at
    }

    /// Check if recovery is in progress
    #[allow(dead_code)]
    pub const fn is_in_progress(&self) -> bool {
        matches!(self.state, RecoveryState::CodeSent)
    }

    /// Check if recovery is expired
    #[allow(dead_code)]
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = Self::current_time();
            now > expires_at
        } else {
            false
        }
    }

    /// Reset recovery state
    fn reset(&mut self) {
        self.email_pattern = None;
        self.recovery_code = None;
        self.expires_at = None;
        self.state = RecoveryState::Idle;
    }

    /// Get current Unix timestamp
    fn current_time() -> i32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0)
    }
}

impl Default for PasswordRecovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_recovery_new() {
        let recovery = PasswordRecovery::new();
        assert_eq!(recovery.state(), RecoveryState::Idle);
        assert!(!recovery.is_in_progress());
        assert!(recovery.email_pattern().is_none());
    }

    #[test]
    fn test_request_password_recovery() {
        let mut recovery = PasswordRecovery::new();
        let result = recovery
            .request_password_recovery("e***@test.com".to_string())
            .unwrap();

        assert_eq!(result.email_pattern(), "e***@test.com");
        assert_eq!(result.code_length(), 6);
        assert!(result.expires_at() > 0);
        assert_eq!(recovery.state(), RecoveryState::CodeSent);
        assert!(recovery.is_in_progress());
    }

    #[test]
    fn test_request_recovery_invalid_email() {
        let mut recovery = PasswordRecovery::new();
        let result = recovery.request_password_recovery("".to_string());
        assert!(matches!(
            result,
            Err(PasswordManagerError::InvalidEmail { .. })
        ));
    }

    #[test]
    fn test_check_recovery_code_valid() {
        let mut recovery = PasswordRecovery::new();
        recovery
            .request_password_recovery("e***@test.com".to_string())
            .unwrap();

        let result = recovery.check_recovery_code("123456");
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_recovery_code_empty() {
        let mut recovery = PasswordRecovery::new();
        recovery
            .request_password_recovery("e***@test.com".to_string())
            .unwrap();

        let result = recovery.check_recovery_code("");
        assert!(matches!(
            result,
            Err(PasswordManagerError::InvalidRecoveryCode)
        ));
    }

    #[test]
    fn test_check_recovery_code_wrong_length() {
        let mut recovery = PasswordRecovery::new();
        recovery
            .request_password_recovery("e***@test.com".to_string())
            .unwrap();

        let result = recovery.check_recovery_code("12345");
        assert!(matches!(
            result,
            Err(PasswordManagerError::InvalidRecoveryCode)
        ));
    }

    #[test]
    fn test_recover_password() {
        let mut recovery = PasswordRecovery::new();
        recovery
            .request_password_recovery("e***@test.com".to_string())
            .unwrap();

        let result = recovery.recover_password("123456", "newpass", "hint");
        assert!(result.is_ok());
        assert_eq!(recovery.state(), RecoveryState::Completed);
        assert!(!recovery.is_in_progress());
    }

    #[test]
    fn test_recover_password_invalid_code() {
        let mut recovery = PasswordRecovery::new();
        recovery
            .request_password_recovery("e***@test.com".to_string())
            .unwrap();

        let result = recovery.recover_password("", "newpass", "hint");
        assert!(matches!(
            result,
            Err(PasswordManagerError::InvalidRecoveryCode)
        ));
    }

    #[test]
    fn test_cancel_recovery() {
        let mut recovery = PasswordRecovery::new();
        recovery
            .request_password_recovery("e***@test.com".to_string())
            .unwrap();

        let result = recovery.cancel();
        assert!(result.is_ok());
        assert_eq!(recovery.state(), RecoveryState::Idle);
        assert!(recovery.email_pattern().is_none());
    }

    #[test]
    fn test_cancel_completed_recovery() {
        let mut recovery = PasswordRecovery::new();
        recovery
            .request_password_recovery("e***@test.com".to_string())
            .unwrap();
        recovery
            .recover_password("123456", "newpass", "hint")
            .unwrap();

        let result = recovery.cancel();
        assert!(result.is_err());
    }

    #[test]
    fn test_recovery_default() {
        let recovery = PasswordRecovery::default();
        assert_eq!(recovery.state(), RecoveryState::Idle);
    }

    #[test]
    fn test_recovery_clone() {
        let mut recovery1 = PasswordRecovery::new();
        recovery1
            .request_password_recovery("e***@test.com".to_string())
            .unwrap();

        let recovery2 = recovery1.clone();
        assert_eq!(recovery1.state(), recovery2.state());
        assert_eq!(recovery1.email_pattern(), recovery2.email_pattern());
    }
}
