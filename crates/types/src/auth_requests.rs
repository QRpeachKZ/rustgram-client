// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Authentication request types for Telegram MTProto.
//!
//! This module provides TL request types for authentication operations.
//!
//! # TL Schema
//!
//! ```text
//! auth.sendCode#a677244f phone_number:string api_id:int api_hash:string settings:CodeSettings = auth.SentCode;
//! auth.signIn#8d52a951 flags:# phone_number:string phone_code_hash:string phone_code:flags.0?string email_verification:flags.1?EmailVerification = auth.Authorization;
//! auth.logOut#3e72ba19 = auth.LoggedOut;
//! ```

use crate::email_verification::EmailVerification;
use crate::error::{TypeError, TypeResult};
use crate::tl::{TlHelper, TlSerialize};
use bytes::BytesMut;
use serde::{Deserialize, Serialize};

use super::code_settings::CodeSettings;

/// Send code request.
///
/// Corresponds to `auth.sendCode#a677244f`.
///
/// # TL Schema
///
/// ```text
/// auth.sendCode#a677244f phone_number:string api_id:int api_hash:string settings:CodeSettings = auth.SentCode;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_types::{SendCodeRequest, CodeSettings};
///
/// let request = SendCodeRequest::new(
///     "+1234567890".to_string(),
///     12345,
///     "api_hash".to_string(),
///     CodeSettings::new(),
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendCodeRequest {
    /// Phone number in international format
    pub phone_number: String,

    /// API ID from Telegram
    pub api_id: i32,

    /// API hash from Telegram
    pub api_hash: String,

    /// Code settings
    pub settings: CodeSettings,
}

impl SendCodeRequest {
    /// Creates a new SendCodeRequest.
    ///
    /// # Arguments
    ///
    /// * `phone_number` - Phone number in international format (e.g., "+1234567890")
    /// * `api_id` - API ID from Telegram
    /// * `api_hash` - API hash from Telegram
    /// * `settings` - Code settings
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::{SendCodeRequest, CodeSettings};
    ///
    /// let request = SendCodeRequest::new(
    ///     "+1234567890".to_string(),
    ///     12345,
    ///     "api_hash".to_string(),
    ///     CodeSettings::new(),
    /// );
    /// ```
    pub fn new(
        phone_number: String,
        api_id: i32,
        api_hash: String,
        settings: CodeSettings,
    ) -> Self {
        Self {
            phone_number,
            api_id,
            api_hash,
            settings,
        }
    }

    /// Returns the phone number.
    #[inline]
    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    /// Returns the API ID.
    #[inline]
    pub const fn api_id(&self) -> i32 {
        self.api_id
    }

    /// Returns the API hash.
    #[inline]
    pub fn api_hash(&self) -> &str {
        &self.api_hash
    }

    /// Returns the code settings.
    #[inline]
    pub fn settings(&self) -> &CodeSettings {
        &self.settings
    }

    /// Validates the request.
    ///
    /// Returns `true` if the request is valid:
    /// - Phone number is not empty
    /// - API hash is not empty
    pub fn is_valid(&self) -> bool {
        !self.phone_number.is_empty() && !self.api_hash.is_empty()
    }
}

impl TlSerialize for SendCodeRequest {
    /// Serializes SendCodeRequest to TL format.
    ///
    /// # TL Format
    ///
    /// ```text
    /// phone_number:string
    /// api_id:int
    /// api_hash:string
    /// settings:CodeSettings
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `TypeError` if serialization fails.
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        TlHelper::write_string(buf, &self.phone_number);
        TlHelper::write_i32(buf, self.api_id);
        TlHelper::write_string(buf, &self.api_hash);
        self.settings.serialize_tl(buf)?;
        Ok(())
    }
}

/// Sign in request.
///
/// Corresponds to `auth.signIn#8d52a951`.
///
/// # TL Schema
///
/// ```text
/// auth.signIn#8d52a951 flags:# phone_number:string phone_code_hash:string
/// phone_code:flags.0?string email_verification:flags.1?EmailVerification = auth.Authorization;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_types::SignInRequest;
///
/// let request = SignInRequest::with_code(
///     "+1234567890".to_string(),
///     "phone_code_hash".to_string(),
///     "12345".to_string(),
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignInRequest {
    /// Flags bitmask
    pub flags: u32,

    /// Phone number
    pub phone_number: String,

    /// Phone code hash from sendCode response
    pub phone_code_hash: String,

    /// Phone code (flag 0)
    pub phone_code: Option<String>,

    /// Email verification (flag 1)
    pub email_verification: Option<EmailVerification>,
}

impl SignInRequest {
    /// Creates a new SignInRequest with phone code.
    ///
    /// # Arguments
    ///
    /// * `phone_number` - Phone number
    /// * `phone_code_hash` - Phone code hash from sendCode response
    /// * `phone_code` - Authentication code from SMS/email
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::SignInRequest;
    ///
    /// let request = SignInRequest::with_code(
    ///     "+1234567890".to_string(),
    ///     "hash".to_string(),
    ///     "12345".to_string(),
    /// );
    /// assert!(request.has_phone_code());
    /// ```
    pub fn with_code(phone_number: String, phone_code_hash: String, phone_code: String) -> Self {
        Self {
            flags: 0x1,
            phone_number,
            phone_code_hash,
            phone_code: Some(phone_code),
            email_verification: None,
        }
    }

    /// Creates a new SignInRequest with email verification.
    ///
    /// # Arguments
    ///
    /// * `phone_number` - Phone number
    /// * `phone_code_hash` - Phone code hash from sendCode response
    /// * `email_verification` - Email verification
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::{SignInRequest, EmailVerification};
    ///
    /// let request = SignInRequest::with_email(
    ///     "+1234567890".to_string(),
    ///     "hash".to_string(),
    ///     EmailVerification::code("email_code".to_string()),
    /// );
    /// assert!(request.has_email_verification());
    /// ```
    pub fn with_email(
        phone_number: String,
        phone_code_hash: String,
        email_verification: EmailVerification,
    ) -> Self {
        Self {
            flags: 0x2,
            phone_number,
            phone_code_hash,
            phone_code: None,
            email_verification: Some(email_verification),
        }
    }

    /// Creates a new SignInRequest with both phone code and email verification.
    ///
    /// # Arguments
    ///
    /// * `phone_number` - Phone number
    /// * `phone_code_hash` - Phone code hash from sendCode response
    /// * `phone_code` - Authentication code from SMS
    /// * `email_verification` - Email verification
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::{SignInRequest, EmailVerification};
    ///
    /// let request = SignInRequest::with_code_and_email(
    ///     "+1234567890".to_string(),
    ///     "hash".to_string(),
    ///     "12345".to_string(),
    ///     EmailVerification::code("email_code".to_string()),
    /// );
    /// assert!(request.has_phone_code());
    /// assert!(request.has_email_verification());
    /// ```
    pub fn with_code_and_email(
        phone_number: String,
        phone_code_hash: String,
        phone_code: String,
        email_verification: EmailVerification,
    ) -> Self {
        Self {
            flags: 0x3,
            phone_number,
            phone_code_hash,
            phone_code: Some(phone_code),
            email_verification: Some(email_verification),
        }
    }

    /// Returns the phone number.
    #[inline]
    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    /// Returns the phone code hash.
    #[inline]
    pub fn phone_code_hash(&self) -> &str {
        &self.phone_code_hash
    }

    /// Returns the phone code.
    #[inline]
    pub fn phone_code(&self) -> Option<&str> {
        self.phone_code.as_deref()
    }

    /// Returns the email verification.
    #[inline]
    pub fn email_verification(&self) -> Option<&EmailVerification> {
        self.email_verification.as_ref()
    }

    /// Checks if this request has a phone code.
    #[inline]
    pub const fn has_phone_code(&self) -> bool {
        self.flags & 0x1 != 0
    }

    /// Checks if this request has email verification.
    #[inline]
    pub const fn has_email_verification(&self) -> bool {
        self.flags & 0x2 != 0
    }

    /// Validates the request.
    ///
    /// Returns `true` if the request is valid:
    /// - Phone number is not empty
    /// - Phone code hash is not empty
    /// - If flag 0 is set, phone_code must be Some
    /// - If flag 1 is set, email_verification must be Some
    pub fn is_valid(&self) -> bool {
        if self.phone_number.is_empty() || self.phone_code_hash.is_empty() {
            return false;
        }

        if self.flags & 0x1 != 0 && self.phone_code.is_none() {
            return false;
        }

        if self.flags & 0x2 != 0 && self.email_verification.is_none() {
            return false;
        }

        true
    }
}

impl TlSerialize for SignInRequest {
    /// Serializes SignInRequest to TL format.
    ///
    /// # TL Format
    ///
    /// ```text
    /// flags:#
    /// phone_number:string
    /// phone_code_hash:string
    /// phone_code:flags.0?string
    /// email_verification:flags.1?EmailVerification
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `TypeError` if:
    /// - Flag 0 is set but phone_code is None
    /// - Flag 1 is set but email_verification is None
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        TlHelper::write_i32(buf, self.flags as i32);
        TlHelper::write_string(buf, &self.phone_number);
        TlHelper::write_string(buf, &self.phone_code_hash);

        if self.flags & 0x1 != 0 {
            let phone_code = self.phone_code.as_ref().ok_or_else(|| {
                TypeError::SerializationError("phone_code flag set but no value".into())
            })?;
            TlHelper::write_string(buf, phone_code);
        }

        if self.flags & 0x2 != 0 {
            let email_verification = self.email_verification.as_ref().ok_or_else(|| {
                TypeError::SerializationError("email_verification flag set but no value".into())
            })?;
            email_verification.serialize_tl(buf)?;
        }

        Ok(())
    }
}

/// Log out request.
///
/// Corresponds to `auth.logOut#3e72ba19`.
///
/// # TL Schema
///
/// ```text
/// auth.logOut#3e72ba19 = auth.LoggedOut;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_types::LogOutRequest;
///
/// let request = LogOutRequest::new();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogOutRequest;

impl LogOutRequest {
    /// Creates a new LogOutRequest.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::LogOutRequest;
    ///
    /// let request = LogOutRequest::new();
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for LogOutRequest {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TlSerialize for LogOutRequest {
    /// Serializes LogOutRequest to TL format.
    ///
    /// This is an empty request (no fields).
    ///
    /// # Errors
    ///
    /// Never returns an error.
    fn serialize_tl(&self, _buf: &mut BytesMut) -> TypeResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_code_request_new() {
        let settings = CodeSettings::new();
        let request = SendCodeRequest::new(
            "+1234567890".to_string(),
            12345,
            "api_hash".to_string(),
            settings.clone(),
        );

        assert_eq!(request.phone_number(), "+1234567890");
        assert_eq!(request.api_id(), 12345);
        assert_eq!(request.api_hash(), "api_hash");
        assert!(request.is_valid());
    }

    #[test]
    fn test_send_code_request_invalid() {
        let request =
            SendCodeRequest::new(String::new(), 12345, String::new(), CodeSettings::new());

        assert!(!request.is_valid());
    }

    #[test]
    fn test_send_code_request_serialize() {
        let request = SendCodeRequest::new(
            "+1234567890".to_string(),
            12345,
            "api_hash".to_string(),
            CodeSettings::new(),
        );

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert!(buf.len() > 0);
    }

    #[test]
    fn test_sign_in_request_with_code() {
        let request = SignInRequest::with_code(
            "+1234567890".to_string(),
            "hash".to_string(),
            "12345".to_string(),
        );

        assert_eq!(request.phone_number(), "+1234567890");
        assert_eq!(request.phone_code_hash(), "hash");
        assert_eq!(request.phone_code(), Some("12345"));
        assert!(request.has_phone_code());
        assert!(!request.has_email_verification());
        assert!(request.is_valid());
    }

    #[test]
    fn test_sign_in_request_with_email() {
        let email = EmailVerification::code("email_code".to_string());
        let request =
            SignInRequest::with_email("+1234567890".to_string(), "hash".to_string(), email);

        assert_eq!(request.phone_number(), "+1234567890");
        assert_eq!(request.phone_code_hash(), "hash");
        assert!(request.phone_code().is_none());
        assert!(!request.has_phone_code());
        assert!(request.has_email_verification());
        assert!(request.is_valid());
    }

    #[test]
    fn test_sign_in_request_with_code_and_email() {
        let email = EmailVerification::code("email_code".to_string());
        let request = SignInRequest::with_code_and_email(
            "+1234567890".to_string(),
            "hash".to_string(),
            "12345".to_string(),
            email,
        );

        assert!(request.has_phone_code());
        assert!(request.has_email_verification());
        assert_eq!(request.flags, 0x3);
        assert!(request.is_valid());
    }

    #[test]
    fn test_sign_in_request_invalid_empty_phone() {
        let request =
            SignInRequest::with_code(String::new(), "hash".to_string(), "12345".to_string());

        assert!(!request.is_valid());
    }

    #[test]
    fn test_sign_in_request_invalid_empty_hash() {
        let request = SignInRequest::with_code(
            "+1234567890".to_string(),
            String::new(),
            "12345".to_string(),
        );

        assert!(!request.is_valid());
    }

    #[test]
    fn test_sign_in_request_serialize_with_code() {
        let request = SignInRequest::with_code(
            "+1234567890".to_string(),
            "hash".to_string(),
            "12345".to_string(),
        );

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert!(buf.len() > 0);
    }

    #[test]
    fn test_sign_in_request_serialize_with_email() {
        let email = EmailVerification::code("email_code".to_string());
        let request =
            SignInRequest::with_email("+1234567890".to_string(), "hash".to_string(), email);

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert!(buf.len() > 0);
    }

    #[test]
    fn test_sign_in_request_serialize_error_no_code() {
        let mut request = SignInRequest::with_code(
            "+1234567890".to_string(),
            "hash".to_string(),
            "12345".to_string(),
        );
        // Set flag but remove code
        request.phone_code = None;

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_sign_in_request_serialize_error_no_email() {
        let mut request = SignInRequest::with_email(
            "+1234567890".to_string(),
            "hash".to_string(),
            EmailVerification::code("email_code".to_string()),
        );
        // Set flag but remove email
        request.email_verification = None;

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_out_request_new() {
        let request = LogOutRequest::new();
        let request2 = LogOutRequest::default();
        assert_eq!(request, request2);
    }

    #[test]
    fn test_log_out_request_serialize() {
        let request = LogOutRequest::new();
        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);
        assert!(result.is_ok());
        // Empty request should produce 0 bytes
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_send_code_request_clone() {
        let request1 = SendCodeRequest::new(
            "+1234567890".to_string(),
            12345,
            "api_hash".to_string(),
            CodeSettings::new().with_flash_call(),
        );
        let request2 = request1.clone();
        assert_eq!(request1, request2);
    }

    #[test]
    fn test_sign_in_request_clone() {
        let request1 = SignInRequest::with_code(
            "+1234567890".to_string(),
            "hash".to_string(),
            "12345".to_string(),
        );
        let request2 = request1.clone();
        assert_eq!(request1, request2);
    }

    #[test]
    fn test_sign_in_request_equality() {
        let request1 = SignInRequest::with_code(
            "+1234567890".to_string(),
            "hash".to_string(),
            "12345".to_string(),
        );
        let request2 = SignInRequest::with_code(
            "+1234567890".to_string(),
            "hash".to_string(),
            "12345".to_string(),
        );
        assert_eq!(request1, request2);

        let request3 = SignInRequest::with_code(
            "+1234567890".to_string(),
            "hash".to_string(),
            "54321".to_string(),
        );
        assert_ne!(request1, request3);
    }

    #[test]
    fn test_sign_in_request_with_google_email() {
        let email = EmailVerification::google("google_token".to_string());
        let request =
            SignInRequest::with_email("+1234567890".to_string(), "hash".to_string(), email);

        assert!(request.has_email_verification());
        assert!(request.email_verification().unwrap().is_google());
        assert!(request.is_valid());
    }

    #[test]
    fn test_sign_in_request_with_apple_email() {
        let email = EmailVerification::apple("apple_token".to_string());
        let request =
            SignInRequest::with_email("+1234567890".to_string(), "hash".to_string(), email);

        assert!(request.has_email_verification());
        assert!(request.email_verification().unwrap().is_apple());
        assert!(request.is_valid());
    }
}
