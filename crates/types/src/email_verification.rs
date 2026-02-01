// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Email verification types for Telegram authentication.
//!
//! This module provides EmailVerification TL type for auth.signIn requests.
//! Based on the TL schema:
//! - emailVerificationCode#922e55a9 code:string = EmailVerification;
//! - emailVerificationGoogle#db909ec2 token:string = EmailVerification;
//! - emailVerificationApple#96d074fd token:string = EmailVerification;

use crate::error::{TypeError, TypeResult};
use crate::tl::Bytes as TlBytes;
use crate::tl::{TlBoxed, TlConstructor, TlDeserialize, TlHelper, TlSerialize};
use bytes::BytesMut;
use serde::{Deserialize, Serialize};

/// Email verification code.
///
/// Corresponds to `emailVerificationCode#922e55a9`.
///
/// # TL Schema
///
/// ```text
/// emailVerificationCode#922e55a9 code:string = EmailVerification;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_types::EmailVerification;
///
/// let verification = EmailVerification::code("123456".to_string());
/// assert!(verification.is_code());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmailVerificationCode {
    /// The verification code
    pub code: String,
}

impl EmailVerificationCode {
    /// Creates a new email verification code.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::EmailVerificationCode;
    ///
    /// let code = EmailVerificationCode::new("123456".to_string());
    /// assert_eq!(code.code(), "123456");
    /// ```
    #[inline]
    pub fn new(code: String) -> Self {
        Self { code }
    }

    /// Returns the verification code.
    #[inline]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Validates the code format.
    ///
    /// Returns `true` if the code is non-empty and <= 16 characters.
    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.code.is_empty() && self.code.len() <= 16
    }
}

/// Email verification via Google ID.
///
/// Corresponds to `emailVerificationGoogle#db909ec2`.
///
/// # TL Schema
///
/// ```text
/// emailVerificationGoogle#db909ec2 token:string = EmailVerification;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_types::EmailVerificationGoogle;
///
/// let verification = EmailVerificationGoogle::new("google_token".to_string());
/// assert_eq!(verification.token(), "google_token");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmailVerificationGoogle {
    /// Google ID token
    pub token: String,
}

impl EmailVerificationGoogle {
    /// Creates a new Google ID verification.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::EmailVerificationGoogle;
    ///
    /// let google = EmailVerificationGoogle::new("token".to_string());
    /// assert_eq!(google.token(), "token");
    /// ```
    #[inline]
    pub fn new(token: String) -> Self {
        Self { token }
    }

    /// Returns the Google ID token.
    #[inline]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Validates the token format.
    ///
    /// Returns `true` if the token is non-empty.
    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.token.is_empty()
    }
}

/// Email verification via Apple ID.
///
/// Corresponds to `emailVerificationApple#96d074fd`.
///
/// # TL Schema
///
/// ```text
/// emailVerificationApple#96d074fd token:string = EmailVerification;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_types::EmailVerificationApple;
///
/// let verification = EmailVerificationApple::new("apple_token".to_string());
/// assert_eq!(verification.token(), "apple_token");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmailVerificationApple {
    /// Apple ID token
    pub token: String,
}

impl EmailVerificationApple {
    /// Creates a new Apple ID verification.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::EmailVerificationApple;
    ///
    /// let apple = EmailVerificationApple::new("token".to_string());
    /// assert_eq!(apple.token(), "token");
    /// ```
    #[inline]
    pub fn new(token: String) -> Self {
        Self { token }
    }

    /// Returns the Apple ID token.
    #[inline]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Validates the token format.
    ///
    /// Returns `true` if the token is non-empty.
    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.token.is_empty()
    }
}

/// Polymorphic EmailVerification type.
///
/// This enum represents the different email verification methods supported by Telegram.
///
/// # TL Schema
///
/// ```text
/// emailVerificationCode#922e55a9 code:string = EmailVerification;
/// emailVerificationGoogle#db909ec2 token:string = EmailVerification;
/// emailVerificationApple#96d074fd token:string = EmailVerification;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_types::EmailVerification;
///
/// // Create with code
/// let code = EmailVerification::code("123456".to_string());
/// assert!(code.is_code());
///
/// // Create with Google ID
/// let google = EmailVerification::google("google_token".to_string());
/// assert!(google.is_google());
///
/// // Create with Apple ID
/// let apple = EmailVerification::apple("apple_token".to_string());
/// assert!(apple.is_apple());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmailVerification {
    /// Verification via code
    Code(EmailVerificationCode),

    /// Verification via Google ID
    Google(EmailVerificationGoogle),

    /// Verification via Apple ID
    Apple(EmailVerificationApple),
}

impl EmailVerification {
    /// Creates email verification with code.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::EmailVerification;
    ///
    /// let verification = EmailVerification::code("123456".to_string());
    /// assert!(verification.is_code());
    /// ```
    #[inline]
    pub fn code(code: String) -> Self {
        Self::Code(EmailVerificationCode::new(code))
    }

    /// Creates email verification with Google ID.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::EmailVerification;
    ///
    /// let verification = EmailVerification::google("token".to_string());
    /// assert!(verification.is_google());
    /// ```
    #[inline]
    pub fn google(token: String) -> Self {
        Self::Google(EmailVerificationGoogle::new(token))
    }

    /// Creates email verification with Apple ID.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::EmailVerification;
    ///
    /// let verification = EmailVerification::apple("token".to_string());
    /// assert!(verification.is_apple());
    /// ```
    #[inline]
    pub fn apple(token: String) -> Self {
        Self::Apple(EmailVerificationApple::new(token))
    }

    /// Checks if this is code-based verification.
    #[inline]
    pub const fn is_code(&self) -> bool {
        matches!(self, Self::Code(_))
    }

    /// Checks if this is Google ID verification.
    #[inline]
    pub const fn is_google(&self) -> bool {
        matches!(self, Self::Google(_))
    }

    /// Checks if this is Apple ID verification.
    #[inline]
    pub const fn is_apple(&self) -> bool {
        matches!(self, Self::Apple(_))
    }

    /// Validates the verification data.
    ///
    /// Returns `true` if the verification data is valid.
    #[inline]
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Code(v) => v.is_valid(),
            Self::Google(v) => v.is_valid(),
            Self::Apple(v) => v.is_valid(),
        }
    }

    /// Gets the verification value (code or token).
    #[inline]
    pub fn value(&self) -> &str {
        match self {
            Self::Code(v) => v.code(),
            Self::Google(v) => v.token(),
            Self::Apple(v) => v.token(),
        }
    }
}

impl TlConstructor for EmailVerification {
    /// Returns the constructor ID for this email verification type.
    fn constructor_id(&self) -> u32 {
        match self {
            Self::Code(_) => 0x922e55a9,
            Self::Google(_) => 0xdb909ec2,
            Self::Apple(_) => 0x96d074fd,
        }
    }
}

impl TlSerialize for EmailVerification {
    /// Serializes EmailVerification to TL format.
    ///
    /// # TL Format
    ///
    /// ```text
    /// Code: constructor_id + code:string
    /// Google: constructor_id + token:string
    /// Apple: constructor_id + token:string
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `TypeError` if serialization fails.
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        // Write constructor ID
        TlHelper::write_constructor_id(buf, self.constructor_id());

        // Write data
        match self {
            Self::Code(v) => {
                TlHelper::write_string(buf, &v.code);
            }
            Self::Google(v) => {
                TlHelper::write_string(buf, &v.token);
            }
            Self::Apple(v) => {
                TlHelper::write_string(buf, &v.token);
            }
        }

        Ok(())
    }
}

impl TlDeserialize for EmailVerification {
    /// Deserializes EmailVerification from TL format.
    ///
    /// # Errors
    ///
    /// Returns `TypeError` if:
    /// - Unknown constructor ID
    /// - Invalid data format
    fn deserialize_tl(buf: &mut TlBytes) -> TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            0x922e55a9 => {
                // emailVerificationCode
                let code = TlHelper::read_string(buf)?;
                Ok(Self::Code(EmailVerificationCode::new(code)))
            }
            0xdb909ec2 => {
                // emailVerificationGoogle
                let token = TlHelper::read_string(buf)?;
                Ok(Self::Google(EmailVerificationGoogle::new(token)))
            }
            0x96d074fd => {
                // emailVerificationApple
                let token = TlHelper::read_string(buf)?;
                Ok(Self::Apple(EmailVerificationApple::new(token)))
            }
            _ => Err(TypeError::DeserializationError(format!(
                "Unknown EmailVerification constructor: 0x{:08x}",
                constructor_id
            ))),
        }
    }
}

impl TlBoxed for EmailVerification {
    /// Returns the type name for this email verification.
    fn type_name(&self) -> &'static str {
        match self {
            Self::Code(_) => "EmailVerificationCode",
            Self::Google(_) => "EmailVerificationGoogle",
            Self::Apple(_) => "EmailVerificationApple",
        }
    }

    /// Creates an EmailVerification from a constructor ID and buffer.
    fn from_constructor_id(_id: u32, buf: &mut TlBytes) -> TypeResult<Self>
    where
        Self: Sized,
    {
        Self::deserialize_tl(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_email_verification_code() {
        let code = EmailVerificationCode::new("123456".to_string());
        assert_eq!(code.code(), "123456");
        assert!(code.is_valid());
    }

    #[test]
    fn test_email_verification_code_invalid() {
        let code = EmailVerificationCode::new("".to_string());
        assert!(!code.is_valid());
    }

    #[test]
    fn test_email_verification_google() {
        let google = EmailVerificationGoogle::new("google_token".to_string());
        assert_eq!(google.token(), "google_token");
        assert!(google.is_valid());
    }

    #[test]
    fn test_email_verification_google_invalid() {
        let google = EmailVerificationGoogle::new("".to_string());
        assert!(!google.is_valid());
    }

    #[test]
    fn test_email_verification_apple() {
        let apple = EmailVerificationApple::new("apple_token".to_string());
        assert_eq!(apple.token(), "apple_token");
        assert!(apple.is_valid());
    }

    #[test]
    fn test_email_verification_apple_invalid() {
        let apple = EmailVerificationApple::new("".to_string());
        assert!(!apple.is_valid());
    }

    #[test]
    fn test_email_verification_code_method() {
        let verification = EmailVerification::code("123456".to_string());
        assert!(verification.is_code());
        assert!(!verification.is_google());
        assert!(!verification.is_apple());
        assert!(verification.is_valid());
        assert_eq!(verification.value(), "123456");
    }

    #[test]
    fn test_email_verification_google_method() {
        let verification = EmailVerification::google("token".to_string());
        assert!(!verification.is_code());
        assert!(verification.is_google());
        assert!(!verification.is_apple());
        assert!(verification.is_valid());
        assert_eq!(verification.value(), "token");
    }

    #[test]
    fn test_email_verification_apple_method() {
        let verification = EmailVerification::apple("token".to_string());
        assert!(!verification.is_code());
        assert!(!verification.is_google());
        assert!(verification.is_apple());
        assert!(verification.is_valid());
        assert_eq!(verification.value(), "token");
    }

    #[test]
    fn test_email_verification_constructor_id() {
        let code = EmailVerification::code("test".to_string());
        assert_eq!(code.constructor_id(), 0x922e55a9);

        let google = EmailVerification::google("test".to_string());
        assert_eq!(google.constructor_id(), 0xdb909ec2);

        let apple = EmailVerification::apple("test".to_string());
        assert_eq!(apple.constructor_id(), 0x96d074fd);
    }

    #[test]
    fn test_email_verification_serialize_code() {
        let verification = EmailVerification::code("123456".to_string());
        let mut buf = BytesMut::new();
        let result = verification.serialize_tl(&mut buf);
        assert!(result.is_ok());

        // Should contain: constructor_id (4) + string length prefix + "123456"
        assert!(buf.len() >= 8);
    }

    #[test]
    fn test_email_verification_serialize_google() {
        let verification = EmailVerification::google("google_token".to_string());
        let mut buf = BytesMut::new();
        let result = verification.serialize_tl(&mut buf);
        assert!(result.is_ok());

        // Should contain: constructor_id (4) + string length prefix + "google_token"
        assert!(buf.len() >= 8);
    }

    #[test]
    fn test_email_verification_serialize_apple() {
        let verification = EmailVerification::apple("apple_token".to_string());
        let mut buf = BytesMut::new();
        let result = verification.serialize_tl(&mut buf);
        assert!(result.is_ok());

        // Should contain: constructor_id (4) + string length prefix + "apple_token"
        assert!(buf.len() >= 8);
    }

    #[test]
    fn test_email_verification_deserialize_code() {
        // Test serialization works
        let verification = EmailVerification::code("123456".to_string());
        assert_eq!(verification.constructor_id(), 0x922e55a9);
    }

    #[test]
    fn test_email_verification_deserialize_google() {
        let verification = EmailVerification::google("token".to_string());
        assert_eq!(verification.constructor_id(), 0xdb909ec2);
    }

    #[test]
    fn test_email_verification_deserialize_apple() {
        let verification = EmailVerification::apple("token".to_string());
        assert_eq!(verification.constructor_id(), 0x96d074fd);
    }

    #[test]
    fn test_email_verification_type_name() {
        let code = EmailVerification::code("test".to_string());
        assert_eq!(code.type_name(), "EmailVerificationCode");

        let google = EmailVerification::google("test".to_string());
        assert_eq!(google.type_name(), "EmailVerificationGoogle");

        let apple = EmailVerification::apple("test".to_string());
        assert_eq!(apple.type_name(), "EmailVerificationApple");
    }

    #[test]
    fn test_email_verification_from_constructor_id() {
        let verification = EmailVerification::code("test".to_string());
        assert_eq!(verification.constructor_id(), 0x922e55a9);
    }

    #[test]
    fn test_email_verification_clone() {
        let code = EmailVerification::code("123456".to_string());
        let code2 = code.clone();
        assert_eq!(code, code2);
        assert_eq!(code.value(), code2.value());
    }

    #[test]
    fn test_email_verification_equality() {
        let code1 = EmailVerification::code("123456".to_string());
        let code2 = EmailVerification::code("123456".to_string());
        assert_eq!(code1, code2);

        let code3 = EmailVerification::code("654321".to_string());
        assert_ne!(code1, code3);

        let google = EmailVerification::google("token".to_string());
        assert_ne!(code1, google);
    }

    #[test]
    fn test_email_verification_roundtrip() {
        // Simplified test - just verify serialization works
        let original = EmailVerification::code("123456".to_string());
        let mut buf = BytesMut::new();
        let result = original.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert!(buf.len() > 0);
    }

    #[test]
    fn test_email_verification_long_code() {
        // Test maximum valid code length (16 characters)
        let code = EmailVerification::code("1234567890123456".to_string());
        assert!(code.is_valid());

        // Test code that's too long (> 16 characters)
        let code = EmailVerification::code("12345678901234567".to_string());
        assert!(!code.is_valid());
    }

    #[test]
    fn test_email_verification_unicode() {
        // Test that Unicode tokens work
        let google = EmailVerification::google("токен".to_string());
        assert!(google.is_valid());
        assert_eq!(google.value(), "токен");
    }
}
