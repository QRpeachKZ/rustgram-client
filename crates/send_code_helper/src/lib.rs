// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Send Code Helper
//!
//! Helper for sending authentication codes in Telegram.
//!
//! Based on TDLib's `SendCodeHelper` from `td/telegram/SendCodeHelper.h`.
//!
//! # Overview
//!
//! A `SendCodeHelper` manages the state of sending authentication codes,
//! tracking phone numbers, code hashes, and authentication code information.
//!
//! # Example
//!
//! ```rust
//! use rustgram_send_code_helper::{SendCodeHelper, AuthenticationCodeInfo};
//!
//! let mut helper = SendCodeHelper::new();
//! helper.set_phone_number("+1234567890");
//! helper.set_phone_code_hash("abc123");
//! assert_eq!(helper.phone_number(), "+1234567890");
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Authentication code type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[repr(i32)]
pub enum AuthenticationCodeType {
    /// No code type
    #[default]
    None = 0,
    /// Message code
    Message = 1,
    /// SMS code
    Sms = 2,
    /// Call code
    Call = 3,
    /// Flash call code
    FlashCall = 4,
    /// Missed call code
    MissedCall = 5,
    /// Fragment code
    Fragment = 6,
    /// Firebase Android SafetyNet
    FirebaseAndroidSafetyNet = 7,
    /// Firebase iOS
    FirebaseIos = 8,
    /// SMS word code
    SmsWord = 9,
    /// SMS phrase code
    SmsPhrase = 10,
    /// Firebase Android Play Integrity
    FirebaseAndroidPlayIntegrity = 11,
}

/// Authentication code information.
///
/// Contains details about an authentication code that was sent.
///
/// # TDLib Mapping
///
/// - `AuthenticationCodeInfo::new()` → TDLib: `AuthenticationCodeInfo()`
///
/// # Example
///
/// ```rust
/// use rustgram_send_code_helper::AuthenticationCodeInfo;
///
/// let info = AuthenticationCodeInfo::new(AuthenticationCodeType::Sms, 6, "***-**");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationCodeInfo {
    /// Code type
    type_: AuthenticationCodeType,
    /// Length of the code
    length: i32,
    /// Pattern of the code (e.g., "***-**")
    pattern: String,
    /// Push timeout in seconds
    push_timeout: i32,
    /// Cloud project number (for Firebase)
    cloud_project_number: i64,
}

impl AuthenticationCodeInfo {
    /// Creates new authentication code information.
    ///
    /// # Arguments
    ///
    /// * `type_` - Type of authentication code
    /// * `length` - Length of the code
    /// * `pattern` - Pattern of the code
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_send_code_helper::{AuthenticationCodeInfo, AuthenticationCodeType};
    ///
    /// let info = AuthenticationCodeInfo::new(AuthenticationCodeType::Sms, 6, "***-**");
    /// ```
    #[must_use]
    pub fn new(type_: AuthenticationCodeType, length: i32, pattern: impl Into<String>) -> Self {
        Self {
            type_,
            length,
            pattern: pattern.into(),
            push_timeout: 0,
            cloud_project_number: 0,
        }
    }

    /// Creates new authentication code information with Firebase details.
    ///
    /// # Arguments
    ///
    /// * `type_` - Type of authentication code
    /// * `length` - Length of the code
    /// * `pattern` - Pattern of the code
    /// * `push_timeout` - Push timeout in seconds
    /// * `cloud_project_number` - Cloud project number
    #[must_use]
    pub fn with_firebase(
        type_: AuthenticationCodeType,
        length: i32,
        pattern: impl Into<String>,
        push_timeout: i32,
        cloud_project_number: i64,
    ) -> Self {
        Self {
            type_,
            length,
            pattern: pattern.into(),
            push_timeout,
            cloud_project_number,
        }
    }

    /// Returns the code type.
    #[must_use]
    pub fn type_(&self) -> AuthenticationCodeType {
        self.type_
    }

    /// Returns the code length.
    #[must_use]
    pub fn length(&self) -> i32 {
        self.length
    }

    /// Returns the code pattern.
    #[must_use]
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Returns the push timeout.
    #[must_use]
    pub fn push_timeout(&self) -> i32 {
        self.push_timeout
    }

    /// Returns the cloud project number.
    #[must_use]
    pub fn cloud_project_number(&self) -> i64 {
        self.cloud_project_number
    }
}

impl Default for AuthenticationCodeInfo {
    fn default() -> Self {
        Self {
            type_: AuthenticationCodeType::None,
            length: 0,
            pattern: String::new(),
            push_timeout: 0,
            cloud_project_number: 0,
        }
    }
}

/// Helper for sending authentication codes.
///
/// Manages the state of sending authentication codes, including
/// phone numbers, code hashes, and code information.
///
/// # TDLib Mapping
///
/// - `SendCodeHelper::new()` → TDLib: `SendCodeHelper()`
/// - `set_phone_number()` → TDLib: Sets `phone_number_`
/// - `set_phone_code_hash()` → TDLib: Sets `phone_code_hash_`
///
/// # Example
///
/// ```rust
/// use rustgram_send_code_helper::SendCodeHelper;
///
/// let mut helper = SendCodeHelper::new();
/// helper.set_phone_number("+1234567890");
/// helper.set_phone_code_hash("abc123");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendCodeHelper {
    /// Phone number
    phone_number: String,
    /// Phone code hash
    phone_code_hash: String,
    /// Information about the sent code
    sent_code_info: AuthenticationCodeInfo,
    /// Information about the next code
    next_code_info: AuthenticationCodeInfo,
    /// Timestamp when next code is available
    next_code_timestamp: i64,
}

impl SendCodeHelper {
    /// Creates a new send code helper.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_send_code_helper::SendCodeHelper;
    ///
    /// let helper = SendCodeHelper::new();
    /// assert_eq!(helper.phone_number(), "");
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            phone_number: String::new(),
            phone_code_hash: String::new(),
            sent_code_info: AuthenticationCodeInfo::default(),
            next_code_info: AuthenticationCodeInfo::default(),
            next_code_timestamp: 0,
        }
    }

    /// Sets the phone number.
    ///
    /// # Arguments
    ///
    /// * `phone_number` - Phone number string
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_send_code_helper::SendCodeHelper;
    ///
    /// let mut helper = SendCodeHelper::new();
    /// helper.set_phone_number("+1234567890");
    /// ```
    pub fn set_phone_number(&mut self, phone_number: impl Into<String>) {
        self.phone_number = phone_number.into();
    }

    /// Sets the phone code hash.
    ///
    /// # Arguments
    ///
    /// * `phone_code_hash` - Phone code hash
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_send_code_helper::SendCodeHelper;
    ///
    /// let mut helper = SendCodeHelper::new();
    /// helper.set_phone_code_hash("abc123");
    /// ```
    pub fn set_phone_code_hash(&mut self, phone_code_hash: impl Into<String>) {
        self.phone_code_hash = phone_code_hash.into();
    }

    /// Sets the sent code information.
    ///
    /// # Arguments
    ///
    /// * `info` - Authentication code information
    pub fn set_sent_code_info(&mut self, info: AuthenticationCodeInfo) {
        self.sent_code_info = info;
    }

    /// Sets the next code information.
    ///
    /// # Arguments
    ///
    /// * `info` - Authentication code information
    pub fn set_next_code_info(&mut self, info: AuthenticationCodeInfo) {
        self.next_code_info = info;
    }

    /// Sets the next code timestamp.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Unix timestamp
    pub fn set_next_code_timestamp(&mut self, timestamp: i64) {
        self.next_code_timestamp = timestamp;
    }

    /// Returns the phone number.
    #[must_use]
    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    /// Returns the phone code hash.
    #[must_use]
    pub fn phone_code_hash(&self) -> &str {
        &self.phone_code_hash
    }

    /// Returns the sent code information.
    #[must_use]
    pub fn sent_code_info(&self) -> &AuthenticationCodeInfo {
        &self.sent_code_info
    }

    /// Returns the next code information.
    #[must_use]
    pub fn next_code_info(&self) -> &AuthenticationCodeInfo {
        &self.next_code_info
    }

    /// Returns the next code timestamp.
    #[must_use]
    pub fn next_code_timestamp(&self) -> i64 {
        self.next_code_timestamp
    }

    /// Checks if the phone number is set.
    #[must_use]
    pub fn has_phone_number(&self) -> bool {
        !self.phone_number.is_empty()
    }

    /// Checks if the phone code hash is set.
    #[must_use]
    pub fn has_phone_code_hash(&self) -> bool {
        !self.phone_code_hash.is_empty()
    }
}

impl Default for SendCodeHelper {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SendCodeHelper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SendCodeHelper(phone={}",
            if self.phone_number.is_empty() {
                "none"
            } else {
                &self.phone_number
            }
        )?;
        if !self.phone_code_hash.is_empty() {
            write!(f, " hash=***")?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // AuthenticationCodeInfo tests
    #[test]
    fn test_auth_code_info_new() {
        let info = AuthenticationCodeInfo::new(AuthenticationCodeType::Sms, 6, "***-**");
        assert_eq!(info.type_(), AuthenticationCodeType::Sms);
        assert_eq!(info.length(), 6);
        assert_eq!(info.pattern(), "***-**");
    }

    #[test]
    fn test_auth_code_info_default() {
        let info = AuthenticationCodeInfo::default();
        assert_eq!(info.type_(), AuthenticationCodeType::None);
        assert_eq!(info.length(), 0);
        assert_eq!(info.pattern(), "");
    }

    #[test]
    fn test_auth_code_info_with_firebase() {
        let info = AuthenticationCodeInfo::with_firebase(
            AuthenticationCodeType::FirebaseIos,
            6,
            "***-**",
            30,
            123456789,
        );
        assert_eq!(info.push_timeout(), 30);
        assert_eq!(info.cloud_project_number(), 123456789);
    }

    // SendCodeHelper tests
    #[test]
    fn test_send_code_helper_new() {
        let helper = SendCodeHelper::new();
        assert_eq!(helper.phone_number(), "");
        assert_eq!(helper.phone_code_hash(), "");
    }

    #[test]
    fn test_send_code_helper_default() {
        let helper = SendCodeHelper::default();
        assert!(!helper.has_phone_number());
        assert!(!helper.has_phone_code_hash());
    }

    #[test]
    fn test_set_phone_number() {
        let mut helper = SendCodeHelper::new();
        helper.set_phone_number("+1234567890");
        assert_eq!(helper.phone_number(), "+1234567890");
        assert!(helper.has_phone_number());
    }

    #[test]
    fn test_set_phone_code_hash() {
        let mut helper = SendCodeHelper::new();
        helper.set_phone_code_hash("abc123");
        assert_eq!(helper.phone_code_hash(), "abc123");
        assert!(helper.has_phone_code_hash());
    }

    #[test]
    fn test_set_sent_code_info() {
        let mut helper = SendCodeHelper::new();
        let info = AuthenticationCodeInfo::new(AuthenticationCodeType::Sms, 6, "***-**");
        helper.set_sent_code_info(info.clone());
        assert_eq!(helper.sent_code_info().type_(), AuthenticationCodeType::Sms);
    }

    #[test]
    fn test_set_next_code_info() {
        let mut helper = SendCodeHelper::new();
        let info = AuthenticationCodeInfo::new(AuthenticationCodeType::Call, 5, "*****");
        helper.set_next_code_info(info.clone());
        assert_eq!(
            helper.next_code_info().type_(),
            AuthenticationCodeType::Call
        );
    }

    #[test]
    fn test_set_next_code_timestamp() {
        let mut helper = SendCodeHelper::new();
        helper.set_next_code_timestamp(1234567890);
        assert_eq!(helper.next_code_timestamp(), 1234567890);
    }

    #[test]
    fn test_has_phone_number() {
        let mut helper = SendCodeHelper::new();
        assert!(!helper.has_phone_number());
        helper.set_phone_number("+1234567890");
        assert!(helper.has_phone_number());
    }

    #[test]
    fn test_has_phone_code_hash() {
        let mut helper = SendCodeHelper::new();
        assert!(!helper.has_phone_code_hash());
        helper.set_phone_code_hash("abc123");
        assert!(helper.has_phone_code_hash());
    }

    #[test]
    fn test_equality() {
        let mut helper1 = SendCodeHelper::new();
        helper1.set_phone_number("+1234567890");

        let mut helper2 = SendCodeHelper::new();
        helper2.set_phone_number("+1234567890");

        assert_eq!(helper1, helper2);
    }

    #[test]
    fn test_clone() {
        let mut helper1 = SendCodeHelper::new();
        helper1.set_phone_number("+1234567890");
        helper1.set_phone_code_hash("abc123");

        let helper2 = helper1.clone();
        assert_eq!(helper2.phone_number(), "+1234567890");
        assert_eq!(helper2.phone_code_hash(), "abc123");
    }

    #[test]
    fn test_display() {
        let mut helper = SendCodeHelper::new();
        helper.set_phone_number("+1234567890");
        let display = format!("{helper}");
        assert!(display.contains("+1234567890"));
    }

    #[test]
    fn test_display_with_hash() {
        let mut helper = SendCodeHelper::new();
        helper.set_phone_number("+1234567890");
        helper.set_phone_code_hash("abc123");
        let display = format!("{helper}");
        assert!(display.contains("hash"));
    }

    #[test]
    fn test_serialization() {
        let mut helper = SendCodeHelper::new();
        helper.set_phone_number("+1234567890");
        helper.set_phone_code_hash("abc123");

        let json = serde_json::to_string(&helper).expect("Failed to serialize");
        let deserialized: SendCodeHelper =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, helper);
    }

    #[test]
    fn test_auth_code_types() {
        let types = vec![
            AuthenticationCodeType::None,
            AuthenticationCodeType::Message,
            AuthenticationCodeType::Sms,
            AuthenticationCodeType::Call,
            AuthenticationCodeType::FlashCall,
            AuthenticationCodeType::MissedCall,
            AuthenticationCodeType::Fragment,
            AuthenticationCodeType::FirebaseAndroidSafetyNet,
            AuthenticationCodeType::FirebaseIos,
            AuthenticationCodeType::SmsWord,
            AuthenticationCodeType::SmsPhrase,
            AuthenticationCodeType::FirebaseAndroidPlayIntegrity,
        ];

        for code_type in types {
            let info = AuthenticationCodeInfo::new(code_type, 6, "***-**");
            assert_eq!(info.type_(), code_type);
        }
    }
}
