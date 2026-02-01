// Copyright (c) 2025 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Phone Number Manager
//!
//! Manages phone number changes and verification for Telegram.
//!
//! ## Overview
//!
//! The `PhoneNumberManager` handles phone number operations including:
//! - Setting a new phone number
//! - Verifying phone number changes via authentication codes
//! - Resending authentication codes
//! - Checking verification codes
//!
//! ## Architecture
//!
//! This is a simplified version of TDLib's `PhoneNumberManager` that focuses
//! on the core state machine for phone number verification. The full TDLib
//! implementation includes actor-based concurrency and network queries.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_phone_number_manager::{PhoneNumberManager, State, Type};
//!
//! let manager = PhoneNumberManager::new();
//! assert_eq!(manager.state(), State::Ok);
//! assert_eq!(manager.operation_type(), Type::None);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub use error::{Error, Result};

mod error;

/// Operation type for phone number manager.
///
/// Based on TDLib's `PhoneNumberManager::Type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[repr(i32)]
pub enum Type {
    /// No operation in progress.
    #[default]
    None = 0,
    /// Changing phone number.
    ChangePhone = 1,
    /// Verifying phone number.
    VerifyPhone = 2,
    /// Confirming phone number.
    ConfirmPhone = 3,
}

impl Type {
    /// Returns true if this is `None`.
    #[must_use]
    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns true if an operation is in progress.
    #[must_use]
    pub const fn is_active(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Returns the name of this operation type.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::ChangePhone => "ChangePhone",
            Self::VerifyPhone => "VerifyPhone",
            Self::ConfirmPhone => "ConfirmPhone",
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// State of phone number verification.
///
/// Based on TDLib's `PhoneNumberManager::State`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum State {
    /// Operation completed, no code needed.
    Ok = 0,
    /// Waiting for authentication code.
    WaitCode = 1,
}

impl State {
    /// Returns true if this is `Ok`.
    #[must_use]
    pub const fn is_ok(self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Returns true if waiting for code.
    #[must_use]
    pub const fn is_waiting(self) -> bool {
        matches!(self, Self::WaitCode)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Ok
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "Ok"),
            Self::WaitCode => write!(f, "WaitCode"),
        }
    }
}

/// Phone number authentication settings.
///
/// Based on TDLib's `phoneNumberAuthenticationSettings`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AuthenticationSettings {
    /// Whether to allow flash call authentication.
    allow_flash_call: bool,
    /// Whether to allow SMS authentication.
    allow_sms: bool,
    /// Firebase authentication token.
    firebase_token: Option<String>,
    /// Mobile network code for missing code reports.
    mobile_network_code: Option<String>,
    /// Whether to allow app authentication.
    allow_app_hash: bool,
}

impl AuthenticationSettings {
    /// Creates new authentication settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether flash calls are allowed.
    pub fn with_flash_call(mut self, allow: bool) -> Self {
        self.allow_flash_call = allow;
        self
    }

    /// Sets whether SMS is allowed.
    pub fn with_sms(mut self, allow: bool) -> Self {
        self.allow_sms = allow;
        self
    }

    /// Sets the Firebase authentication token.
    pub fn with_firebase_token(mut self, token: String) -> Self {
        self.firebase_token = Some(token);
        self
    }

    /// Sets the mobile network code.
    pub fn with_mobile_network_code(mut self, code: String) -> Self {
        self.mobile_network_code = Some(code);
        self
    }

    /// Returns if flash calls are allowed.
    #[must_use]
    pub fn allow_flash_call(&self) -> bool {
        self.allow_flash_call
    }

    /// Returns if SMS is allowed.
    #[must_use]
    pub fn allow_sms(&self) -> bool {
        self.allow_sms
    }

    /// Returns the Firebase token.
    #[must_use]
    pub fn firebase_token(&self) -> Option<&str> {
        self.firebase_token.as_deref()
    }

    /// Returns the mobile network code.
    #[must_use]
    pub fn mobile_network_code(&self) -> Option<&str> {
        self.mobile_network_code.as_deref()
    }
}

/// Phone number code type.
///
/// Based on TDLib's `PhoneNumberCodeType`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CodeType {
    /// Verification code call.
    Call,
    /// Flash call code.
    FlashCall,
    /// SMS code.
    Sms,
    /// Fragment SMS code.
    FragmentSms { fragment_id: String },
}

impl fmt::Display for CodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Call => write!(f, "Call"),
            Self::FlashCall => write!(f, "FlashCall"),
            Self::Sms => write!(f, "Sms"),
            Self::FragmentSms { fragment_id } => {
                write!(f, "FragmentSms({fragment_id})")
            }
        }
    }
}

/// Authentication code information.
///
/// Based on TDLib's `authenticationCodeInfo`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeInfo {
    /// Phone number that will receive authentication code.
    phone_number: String,
    /// Type of authentication code.
    code_type: CodeType,
    /// Next code type if current fails.
    next_type: Option<CodeType>,
    /// Timeout before next code can be sent.
    timeout: Option<i32>,
}

impl CodeInfo {
    /// Creates new code info.
    #[must_use]
    pub fn new(phone_number: String, code_type: CodeType) -> Self {
        Self {
            phone_number,
            code_type,
            next_type: None,
            timeout: None,
        }
    }

    /// Returns the phone number.
    #[must_use]
    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    /// Returns the code type.
    #[must_use]
    pub fn code_type(&self) -> &CodeType {
        &self.code_type
    }

    /// Returns the next code type.
    #[must_use]
    pub fn next_type(&self) -> Option<&CodeType> {
        self.next_type.as_ref()
    }

    /// Returns the timeout.
    #[must_use]
    pub fn timeout(&self) -> Option<i32> {
        self.timeout
    }
}

/// Phone number manager.
///
/// Manages phone number changes and verification.
///
/// # Example
///
/// ```rust
/// use rustgram_phone_number_manager::PhoneNumberManager;
///
/// # #[tokio::main]
/// # async fn main() {
/// let manager = PhoneNumberManager::new();
/// assert_eq!(manager.state().await, rustgram_phone_number_manager::State::Ok);
/// # }
/// ```
#[derive(Clone)]
pub struct PhoneNumberManager {
    inner: Arc<RwLock<Inner>>,
}

struct Inner {
    state: State,
    operation_type: Type,
    generation: Arc<AtomicI64>,
    phone_number: Option<String>,
    code_info: Option<CodeInfo>,
}

impl fmt::Debug for PhoneNumberManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PhoneNumberManager")
            .field("state", &self.try_state())
            .field("operation_type", &self.try_operation_type())
            .finish()
    }
}

impl Default for PhoneNumberManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PhoneNumberManager {
    /// Creates a new phone number manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Inner {
                state: State::Ok,
                operation_type: Type::None,
                generation: Arc::new(AtomicI64::new(0)),
                phone_number: None,
                code_info: None,
            })),
        }
    }

    /// Returns the current state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_phone_number_manager::PhoneNumberManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = PhoneNumberManager::new();
    /// assert_eq!(manager.state().await, rustgram_phone_number_manager::State::Ok);
    /// # }
    /// ```
    pub async fn state(&self) -> State {
        self.inner.read().await.state
    }

    /// Returns the current state (synchronous, may return None if lock fails).
    #[must_use]
    pub fn try_state(&self) -> Option<State> {
        self.inner.try_read().ok().map(|inner| inner.state)
    }

    /// Returns the operation type.
    pub async fn operation_type(&self) -> Type {
        self.inner.read().await.operation_type
    }

    /// Returns the operation type (synchronous).
    #[must_use]
    pub fn try_operation_type(&self) -> Option<Type> {
        self.inner.try_read().ok().map(|inner| inner.operation_type)
    }

    /// Returns the current generation.
    #[must_use]
    pub fn generation(&self) -> i64 {
        // We can access the atomic directly without lock
        Arc::clone(
            &self
                .inner
                .try_read()
                .ok()
                .map(|inner| Arc::clone(&inner.generation))
                .unwrap_or_else(|| Arc::new(AtomicI64::new(0))),
        )
        .load(Ordering::SeqCst)
    }

    /// Returns the phone number being set.
    pub async fn phone_number(&self) -> Option<String> {
        self.inner.read().await.phone_number.clone()
    }

    /// Sets a new phone number.
    ///
    /// # Arguments
    ///
    /// * `phone_number` - The new phone number
    /// * `settings` - Authentication settings
    /// * `code_type` - Type of code to send
    ///
    /// # Errors
    ///
    /// Returns an error if an operation is already in progress.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_phone_number_manager::{
    ///     PhoneNumberManager, AuthenticationSettings, CodeType
    /// };
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = PhoneNumberManager::new();
    /// let settings = AuthenticationSettings::new().with_sms(true);
    ///
    /// let result = manager.set_phone_number(
    ///     "+1234567890".to_string(),
    ///     settings,
    ///     CodeType::Sms
    /// ).await;
    ///
    /// assert!(result.is_ok());
    /// # }
    /// ```
    pub async fn set_phone_number(
        &self,
        phone_number: String,
        settings: AuthenticationSettings,
        code_type: CodeType,
    ) -> Result<CodeInfo> {
        let mut inner = self.inner.write().await;

        if inner.operation_type.is_active() {
            warn!("Operation already in progress: {:?}", inner.operation_type);
            return Err(Error::OperationInProgress {
                operation: inner.operation_type,
            });
        }

        info!("Setting phone number: {}", phone_number);
        debug!(
            "Authentication settings: flash_call={}, sms={}",
            settings.allow_flash_call, settings.allow_sms
        );

        inner.operation_type = Type::ChangePhone;
        inner.state = State::WaitCode;
        inner.phone_number = Some(phone_number.clone());
        self.inc_generation(&inner);

        let code_info = CodeInfo {
            phone_number: phone_number.clone(),
            code_type,
            next_type: None,
            timeout: Some(60),
        };
        inner.code_info = Some(code_info.clone());

        Ok(code_info)
    }

    /// Sends Firebase SMS authentication.
    ///
    /// # Arguments
    ///
    /// * `token` - Firebase authentication token
    ///
    /// # Errors
    ///
    /// Returns an error if no operation is in progress.
    pub async fn send_firebase_sms(&self, token: String) -> Result<()> {
        let inner = self.inner.read().await;

        if !inner.operation_type.is_active() {
            return Err(Error::NoOperationInProgress);
        }

        info!("Sending Firebase SMS with token");
        debug!("Token length: {}", token.len());

        // In the full implementation, this would send the Firebase request
        Ok(())
    }

    /// Reports a missing authentication code.
    ///
    /// # Arguments
    ///
    /// * `mobile_network_code` - Mobile network code
    ///
    /// # Errors
    ///
    /// Returns an error if no operation is in progress.
    pub async fn report_missing_code(&self, mobile_network_code: String) -> Result<()> {
        let inner = self.inner.read().await;

        if !inner.operation_type.is_active() {
            return Err(Error::NoOperationInProgress);
        }

        info!(
            "Reporting missing code for network: {}",
            mobile_network_code
        );

        // In the full implementation, this would send the report to Telegram
        Ok(())
    }

    /// Resends the authentication code.
    ///
    /// # Arguments
    ///
    /// * `reason` - Reason for resending
    ///
    /// # Errors
    ///
    /// Returns an error if no operation is in progress.
    pub async fn resend_code(&self, reason: CodeType) -> Result<CodeInfo> {
        let mut inner = self.inner.write().await;

        if !inner.operation_type.is_active() {
            return Err(Error::NoOperationInProgress);
        }

        info!("Resending code, reason: {:?}", reason);
        self.inc_generation(&inner);

        let code_info = CodeInfo {
            phone_number: inner.phone_number.clone().unwrap_or_default(),
            code_type: reason,
            next_type: None,
            timeout: Some(60),
        };
        inner.code_info = Some(code_info.clone());

        Ok(code_info)
    }

    /// Checks the authentication code.
    ///
    /// # Arguments
    ///
    /// * `code` - The authentication code to check
    ///
    /// # Errors
    ///
    /// Returns an error if no operation is in progress or the code is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_phone_number_manager::PhoneNumberManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = PhoneNumberManager::new();
    ///
    /// // After setting a phone number and receiving a code...
    /// let result = manager.check_code("12345".to_string()).await;
    /// # }
    /// ```
    pub async fn check_code(&self, code: String) -> Result<()> {
        let mut inner = self.inner.write().await;

        if !inner.operation_type.is_active() {
            return Err(Error::NoOperationInProgress);
        }

        if inner.state != State::WaitCode {
            return Err(Error::InvalidState {
                expected: State::WaitCode,
                got: inner.state,
            });
        }

        info!("Checking authentication code");
        debug!("Code length: {}", code.len());

        // In the full implementation, this would verify the code with Telegram
        // For now, we'll simulate success if code is not empty
        if code.is_empty() {
            return Err(Error::InvalidCode);
        }

        inner.state = State::Ok;
        inner.operation_type = Type::None;
        inner.phone_number = None;
        inner.code_info = None;

        info!("Phone number verified successfully");
        Ok(())
    }

    /// Increments the generation counter.
    fn inc_generation(&self, inner: &Inner) {
        inner.generation.fetch_add(1, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Type Tests ===

    #[test]
    fn test_type_default() {
        assert_eq!(Type::default(), Type::None);
    }

    #[test]
    fn test_type_is_none() {
        assert!(Type::None.is_none());
        assert!(!Type::ChangePhone.is_none());
        assert!(!Type::VerifyPhone.is_none());
        assert!(!Type::ConfirmPhone.is_none());
    }

    #[test]
    fn test_type_is_active() {
        assert!(!Type::None.is_active());
        assert!(Type::ChangePhone.is_active());
        assert!(Type::VerifyPhone.is_active());
        assert!(Type::ConfirmPhone.is_active());
    }

    #[test]
    fn test_type_name() {
        assert_eq!(Type::None.name(), "None");
        assert_eq!(Type::ChangePhone.name(), "ChangePhone");
        assert_eq!(Type::VerifyPhone.name(), "VerifyPhone");
        assert_eq!(Type::ConfirmPhone.name(), "ConfirmPhone");
    }

    #[test]
    fn test_type_display() {
        assert_eq!(format!("{}", Type::None), "None");
        assert_eq!(format!("{}", Type::ChangePhone), "ChangePhone");
    }

    #[test]
    fn test_type_repr() {
        assert_eq!(Type::None as i32, 0);
        assert_eq!(Type::ChangePhone as i32, 1);
        assert_eq!(Type::VerifyPhone as i32, 2);
        assert_eq!(Type::ConfirmPhone as i32, 3);
    }

    // === State Tests ===

    #[test]
    fn test_state_default() {
        assert_eq!(State::default(), State::Ok);
    }

    #[test]
    fn test_state_is_ok() {
        assert!(State::Ok.is_ok());
        assert!(!State::WaitCode.is_ok());
    }

    #[test]
    fn test_state_is_waiting() {
        assert!(!State::Ok.is_waiting());
        assert!(State::WaitCode.is_waiting());
    }

    #[test]
    fn test_state_display() {
        assert_eq!(format!("{}", State::Ok), "Ok");
        assert_eq!(format!("{}", State::WaitCode), "WaitCode");
    }

    #[test]
    fn test_state_repr() {
        assert_eq!(State::Ok as i32, 0);
        assert_eq!(State::WaitCode as i32, 1);
    }

    // === AuthenticationSettings Tests ===

    #[test]
    fn test_settings_default() {
        let settings = AuthenticationSettings::default();
        assert!(!settings.allow_flash_call());
        assert!(!settings.allow_sms());
        assert!(settings.firebase_token().is_none());
        assert!(settings.mobile_network_code().is_none());
    }

    #[test]
    fn test_settings_with_flash_call() {
        let settings = AuthenticationSettings::new().with_flash_call(true);
        assert!(settings.allow_flash_call());
        assert!(!settings.allow_sms());
    }

    #[test]
    fn test_settings_with_sms() {
        let settings = AuthenticationSettings::new().with_sms(true);
        assert!(!settings.allow_flash_call());
        assert!(settings.allow_sms());
    }

    #[test]
    fn test_settings_with_firebase_token() {
        let settings = AuthenticationSettings::new().with_firebase_token("token123".to_string());
        assert_eq!(settings.firebase_token(), Some("token123"));
    }

    #[test]
    fn test_settings_with_mobile_network_code() {
        let settings = AuthenticationSettings::new().with_mobile_network_code("310260".to_string());
        assert_eq!(settings.mobile_network_code(), Some("310260"));
    }

    #[test]
    fn test_settings_chain() {
        let settings = AuthenticationSettings::new()
            .with_flash_call(true)
            .with_sms(true)
            .with_firebase_token("token".to_string())
            .with_mobile_network_code("310260".to_string());

        assert!(settings.allow_flash_call());
        assert!(settings.allow_sms());
        assert_eq!(settings.firebase_token(), Some("token"));
        assert_eq!(settings.mobile_network_code(), Some("310260"));
    }

    // === CodeType Tests ===

    #[test]
    fn test_code_type_display() {
        assert_eq!(format!("{}", CodeType::Call), "Call");
        assert_eq!(format!("{}", CodeType::FlashCall), "FlashCall");
        assert_eq!(format!("{}", CodeType::Sms), "Sms");
        assert!(format!(
            "{}",
            CodeType::FragmentSms {
                fragment_id: "abc".to_string()
            }
        )
        .contains("FragmentSms"));
    }

    // === CodeInfo Tests ===

    #[test]
    fn test_code_info_new() {
        let info = CodeInfo::new("+1234567890".to_string(), CodeType::Sms);
        assert_eq!(info.phone_number(), "+1234567890");
        assert!(matches!(info.code_type(), CodeType::Sms));
        assert!(info.next_type().is_none());
        assert!(info.timeout().is_none());
    }

    #[test]
    fn test_code_info_phone_number() {
        let info = CodeInfo::new("test".to_string(), CodeType::Call);
        assert_eq!(info.phone_number(), "test");
    }

    #[test]
    fn test_code_info_code_type() {
        let info = CodeInfo::new("test".to_string(), CodeType::FlashCall);
        assert!(matches!(info.code_type(), CodeType::FlashCall));
    }

    // === PhoneNumberManager Tests ===

    #[tokio::test]
    async fn test_manager_new() {
        let manager = PhoneNumberManager::new();
        assert_eq!(manager.state().await, State::Ok);
        assert_eq!(manager.operation_type().await, Type::None);
        assert!(manager.phone_number().await.is_none());
    }

    #[tokio::test]
    async fn test_manager_default() {
        let manager = PhoneNumberManager::default();
        assert_eq!(manager.state().await, State::Ok);
    }

    #[tokio::test]
    async fn test_manager_clone() {
        let manager1 = PhoneNumberManager::new();
        let manager2 = manager1.clone();

        assert_eq!(manager1.state().await, State::Ok);
        assert_eq!(manager2.state().await, State::Ok);
    }

    #[tokio::test]
    async fn test_set_phone_number() {
        let manager = PhoneNumberManager::new();
        let settings = AuthenticationSettings::new().with_sms(true);

        let result = manager
            .set_phone_number("+1234567890".to_string(), settings, CodeType::Sms)
            .await;

        assert!(result.is_ok());
        let code_info = result.unwrap();
        assert_eq!(code_info.phone_number(), "+1234567890");
        assert!(matches!(code_info.code_type(), CodeType::Sms));

        assert_eq!(manager.state().await, State::WaitCode);
        assert_eq!(manager.operation_type().await, Type::ChangePhone);
    }

    #[tokio::test]
    async fn test_set_phone_number_twice_fails() {
        let manager = PhoneNumberManager::new();
        let settings = AuthenticationSettings::new();

        let result1 = manager
            .set_phone_number("+1234567890".to_string(), settings.clone(), CodeType::Sms)
            .await;
        assert!(result1.is_ok());

        let result2 = manager
            .set_phone_number("+9876543210".to_string(), settings, CodeType::Sms)
            .await;
        assert!(result2.is_err());

        match result2 {
            Err(Error::OperationInProgress { operation }) => {
                assert_eq!(operation, Type::ChangePhone);
            }
            _ => panic!("Expected OperationInProgress error"),
        }
    }

    #[tokio::test]
    async fn test_check_code_success() {
        let manager = PhoneNumberManager::new();
        let settings = AuthenticationSettings::new();

        manager
            .set_phone_number("+1234567890".to_string(), settings, CodeType::Sms)
            .await
            .unwrap();

        let result = manager.check_code("12345".to_string()).await;
        assert!(result.is_ok());

        assert_eq!(manager.state().await, State::Ok);
        assert_eq!(manager.operation_type().await, Type::None);
    }

    #[tokio::test]
    async fn test_check_code_empty_fails() {
        let manager = PhoneNumberManager::new();
        let settings = AuthenticationSettings::new();

        manager
            .set_phone_number("+1234567890".to_string(), settings, CodeType::Sms)
            .await
            .unwrap();

        let result = manager.check_code("".to_string()).await;
        assert!(result.is_err());

        match result {
            Err(Error::InvalidCode) => {}
            _ => panic!("Expected InvalidCode error"),
        }
    }

    #[tokio::test]
    async fn test_check_code_without_operation_fails() {
        let manager = PhoneNumberManager::new();

        let result = manager.check_code("12345".to_string()).await;
        assert!(result.is_err());

        match result {
            Err(Error::NoOperationInProgress) => {}
            _ => panic!("Expected NoOperationInProgress error"),
        }
    }

    #[tokio::test]
    async fn test_resend_code() {
        let manager = PhoneNumberManager::new();
        let settings = AuthenticationSettings::new();

        manager
            .set_phone_number("+1234567890".to_string(), settings, CodeType::Sms)
            .await
            .unwrap();

        let result = manager.resend_code(CodeType::Call).await;
        assert!(result.is_ok());

        let code_info = result.unwrap();
        assert!(matches!(code_info.code_type(), CodeType::Call));
    }

    #[tokio::test]
    async fn test_resend_code_without_operation_fails() {
        let manager = PhoneNumberManager::new();

        let result = manager.resend_code(CodeType::Sms).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_firebase_sms() {
        let manager = PhoneNumberManager::new();
        let settings = AuthenticationSettings::new();

        manager
            .set_phone_number("+1234567890".to_string(), settings, CodeType::Sms)
            .await
            .unwrap();

        let result = manager
            .send_firebase_sms("firebase_token".to_string())
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_firebase_sms_without_operation_fails() {
        let manager = PhoneNumberManager::new();

        let result = manager.send_firebase_sms("token".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_report_missing_code() {
        let manager = PhoneNumberManager::new();
        let settings = AuthenticationSettings::new();

        manager
            .set_phone_number("+1234567890".to_string(), settings, CodeType::Sms)
            .await
            .unwrap();

        let result = manager.report_missing_code("310260".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_report_missing_code_without_operation_fails() {
        let manager = PhoneNumberManager::new();

        let result = manager.report_missing_code("310260".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generation_increments() {
        let manager = PhoneNumberManager::new();
        let gen1 = manager.generation();

        let settings = AuthenticationSettings::new();
        manager
            .set_phone_number("+1234567890".to_string(), settings, CodeType::Sms)
            .await
            .unwrap();

        let gen2 = manager.generation();
        assert!(gen2 > gen1);
    }

    #[tokio::test]
    async fn test_phone_number_retrieved() {
        let manager = PhoneNumberManager::new();
        let settings = AuthenticationSettings::new();

        manager
            .set_phone_number("+1234567890".to_string(), settings, CodeType::Sms)
            .await
            .unwrap();

        assert_eq!(
            manager.phone_number().await,
            Some("+1234567890".to_string())
        );
    }

    // === Integration Tests ===

    #[tokio::test]
    async fn test_full_verification_flow() {
        let manager = PhoneNumberManager::new();

        // Initial state
        assert_eq!(manager.state().await, State::Ok);
        assert_eq!(manager.operation_type().await, Type::None);

        // Set phone number
        let settings = AuthenticationSettings::new()
            .with_flash_call(false)
            .with_sms(true);

        let code_info = manager
            .set_phone_number("+1234567890".to_string(), settings, CodeType::Sms)
            .await
            .unwrap();

        assert_eq!(code_info.phone_number(), "+1234567890");
        assert_eq!(manager.state().await, State::WaitCode);
        assert_eq!(manager.operation_type().await, Type::ChangePhone);

        // Resend code
        let code_info2 = manager.resend_code(CodeType::Call).await.unwrap();
        assert!(matches!(code_info2.code_type(), CodeType::Call));

        // Check code
        let result = manager.check_code("12345".to_string()).await;
        assert!(result.is_ok());

        // Final state
        assert_eq!(manager.state().await, State::Ok);
        assert_eq!(manager.operation_type().await, Type::None);
    }
}
