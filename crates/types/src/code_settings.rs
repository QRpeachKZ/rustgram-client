// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Code settings for Telegram authentication.
//!
//! This module provides the CodeSettings TL type for auth.sendCode requests.

use crate::error::{TypeError, TypeResult};
use crate::tl::{TlHelper, TlSerialize};
use bytes::BytesMut;
use serde::{Deserialize, Serialize};

/// Code settings for authentication.
///
/// Corresponds to the `codeSettings#ad253d78` TL constructor.
/// Used in `auth.sendCode` requests to specify how the authentication code should be sent.
///
/// # TL Schema
///
/// ```text
/// codeSettings#ad253d78 flags:# allow_flashcall:flags.0?true current_number:flags.1?true
/// allow_app_hash:flags.4?true allow_missed_call:flags.5?true allow_firebase:flags.7?true
/// unknown_number:flags.9?true logout_tokens:flags.6?Vector<bytes> token:flags.8?string
/// app_sandbox:flags.8?Bool = CodeSettings;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_types::CodeSettings;
///
/// // Create default settings
/// let settings = CodeSettings::new();
///
/// // Enable flash call
/// let settings = settings.with_flash_call();
///
/// // Enable with current number verification
/// let settings = settings.with_current_number();
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodeSettings {
    /// Flags bitmask
    pub flags: u32,

    /// Allow flash call (flag 0)
    pub allow_flashcall: bool,

    /// Current number verification (flag 1)
    pub current_number: bool,

    /// Allow app hash (flag 4)
    pub allow_app_hash: bool,

    /// Allow missed call (flag 5)
    pub allow_missed_call: bool,

    /// Allow Firebase SMS (flag 7)
    pub allow_firebase: bool,

    /// Unknown number (flag 9)
    pub unknown_number: bool,

    /// Logout tokens (flag 6)
    pub logout_tokens: Option<Vec<Vec<u8>>>,

    /// Firebase token (flag 8)
    pub token: Option<String>,

    /// App sandbox (flag 8)
    pub app_sandbox: Option<bool>,
}

impl CodeSettings {
    /// Creates a new CodeSettings with default values.
    ///
    /// All flags are set to false by default.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::CodeSettings;
    ///
    /// let settings = CodeSettings::new();
    /// assert_eq!(settings.flags, 0);
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables flash call authentication (flag 0).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::CodeSettings;
    ///
    /// let settings = CodeSettings::new().with_flash_call();
    /// assert!(settings.allow_flashcall);
    /// assert!(settings.flags & 0x1 != 0);
    /// ```
    #[must_use]
    pub fn with_flash_call(mut self) -> Self {
        self.allow_flashcall = true;
        self.flags |= 0x1;
        self
    }

    /// Enables current number verification (flag 1).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::CodeSettings;
    ///
    /// let settings = CodeSettings::new().with_current_number();
    /// assert!(settings.current_number);
    /// assert!(settings.flags & 0x2 != 0);
    /// ```
    #[must_use]
    pub fn with_current_number(mut self) -> Self {
        self.current_number = true;
        self.flags |= 0x2;
        self
    }

    /// Enables app hash (flag 4).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::CodeSettings;
    ///
    /// let settings = CodeSettings::new().with_app_hash();
    /// assert!(settings.allow_app_hash);
    /// assert!(settings.flags & 0x10 != 0);
    /// ```
    #[must_use]
    pub fn with_app_hash(mut self) -> Self {
        self.allow_app_hash = true;
        self.flags |= 0x10;
        self
    }

    /// Enables missed call authentication (flag 5).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::CodeSettings;
    ///
    /// let settings = CodeSettings::new().with_missed_call();
    /// assert!(settings.allow_missed_call);
    /// assert!(settings.flags & 0x20 != 0);
    /// ```
    #[must_use]
    pub fn with_missed_call(mut self) -> Self {
        self.allow_missed_call = true;
        self.flags |= 0x20;
        self
    }

    /// Enables Firebase SMS authentication (flag 7).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::CodeSettings;
    ///
    /// let settings = CodeSettings::new().with_firebase();
    /// assert!(settings.allow_firebase);
    /// assert!(settings.flags & 0x80 != 0);
    /// ```
    #[must_use]
    pub fn with_firebase(mut self) -> Self {
        self.allow_firebase = true;
        self.flags |= 0x80;
        self
    }

    /// Enables unknown number mode (flag 9).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::CodeSettings;
    ///
    /// let settings = CodeSettings::new().with_unknown_number();
    /// assert!(settings.unknown_number);
    /// assert!(settings.flags & 0x200 != 0);
    /// ```
    #[must_use]
    pub fn with_unknown_number(mut self) -> Self {
        self.unknown_number = true;
        self.flags |= 0x200;
        self
    }

    /// Sets logout tokens (flag 6).
    ///
    /// # Arguments
    ///
    /// * `tokens` - Vector of logout tokens
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::CodeSettings;
    ///
    /// let tokens = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let settings = CodeSettings::new().with_logout_tokens(tokens);
    /// assert!(settings.logout_tokens.is_some());
    /// assert!(settings.flags & 0x40 != 0);
    /// ```
    #[must_use]
    pub fn with_logout_tokens(mut self, tokens: Vec<Vec<u8>>) -> Self {
        self.logout_tokens = Some(tokens);
        self.flags |= 0x40;
        self
    }

    /// Sets Firebase token and app sandbox (flag 8).
    ///
    /// # Arguments
    ///
    /// * `token` - Firebase token
    /// * `app_sandbox` - Whether app is in sandbox mode
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_types::CodeSettings;
    ///
    /// let settings = CodeSettings::new().with_token("firebase_token".to_string(), true);
    /// assert_eq!(settings.token.as_deref(), Some("firebase_token"));
    /// assert_eq!(settings.app_sandbox, Some(true));
    /// assert!(settings.flags & 0x100 != 0);
    /// ```
    #[must_use]
    pub fn with_token(mut self, token: String, app_sandbox: bool) -> Self {
        self.token = Some(token);
        self.app_sandbox = Some(app_sandbox);
        self.flags |= 0x100;
        self
    }

    /// Checks if flash call is enabled.
    #[inline]
    pub const fn is_flash_call_allowed(&self) -> bool {
        self.allow_flashcall
    }

    /// Checks if current number verification is enabled.
    #[inline]
    pub const fn is_current_number(&self) -> bool {
        self.current_number
    }

    /// Checks if app hash is enabled.
    #[inline]
    pub const fn is_app_hash_allowed(&self) -> bool {
        self.allow_app_hash
    }

    /// Checks if missed call is enabled.
    #[inline]
    pub const fn is_missed_call_allowed(&self) -> bool {
        self.allow_missed_call
    }

    /// Checks if Firebase is enabled.
    #[inline]
    pub const fn is_firebase_allowed(&self) -> bool {
        self.allow_firebase
    }

    /// Checks if unknown number mode is enabled.
    #[inline]
    pub const fn is_unknown_number(&self) -> bool {
        self.unknown_number
    }
}

impl TlSerialize for CodeSettings {
    /// Serializes CodeSettings to TL format.
    ///
    /// # TL Format
    ///
    /// ```text
    /// flags:# (i32)
    /// logout_tokens:flags.6?Vector<bytes>
    /// token:flags.8?string
    /// app_sandbox:flags.8?Bool
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `TypeError` if:
    /// - Flag 6 is set but logout_tokens is None
    /// - Flag 8 is set but token or app_sandbox is None
    fn serialize_tl(&self, buf: &mut BytesMut) -> TypeResult<()> {
        // Write flags
        TlHelper::write_i32(buf, self.flags as i32);

        // Handle logout_tokens (flag 6: 0x40)
        if self.flags & 0x40 != 0 {
            let tokens = self.logout_tokens.as_ref().ok_or_else(|| {
                TypeError::SerializationError("logout_tokens flag set but no value".into())
            })?;
            TlHelper::write_i32(buf, tokens.len() as i32);
            for token in tokens {
                TlHelper::write_bytes(buf, token);
            }
        }

        // Handle token and app_sandbox (flag 8: 0x100)
        if self.flags & 0x100 != 0 {
            let token = self.token.as_ref().ok_or_else(|| {
                TypeError::SerializationError("token flag set but no value".into())
            })?;
            TlHelper::write_string(buf, token);

            let sandbox = self.app_sandbox.ok_or_else(|| {
                TypeError::SerializationError("app_sandbox flag set but no value".into())
            })?;

            // Write Bool as constructor ID (TL boolTrue: 0x997275b5, boolFalse: 0xbc799737)
            if sandbox {
                TlHelper::write_constructor_id(buf, 0x997275b5); // true
            } else {
                TlHelper::write_constructor_id(buf, 0xbc799737); // false
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_settings_new() {
        let settings = CodeSettings::new();
        assert_eq!(settings.flags, 0);
        assert!(!settings.allow_flashcall);
        assert!(!settings.current_number);
        assert!(!settings.allow_app_hash);
        assert!(!settings.allow_missed_call);
        assert!(!settings.allow_firebase);
        assert!(!settings.unknown_number);
    }

    #[test]
    fn test_code_settings_with_flash_call() {
        let settings = CodeSettings::new().with_flash_call();
        assert!(settings.allow_flashcall);
        assert!(settings.flags & 0x1 != 0);
        assert!(settings.is_flash_call_allowed());
    }

    #[test]
    fn test_code_settings_with_current_number() {
        let settings = CodeSettings::new().with_current_number();
        assert!(settings.current_number);
        assert!(settings.flags & 0x2 != 0);
        assert!(settings.is_current_number());
    }

    #[test]
    fn test_code_settings_with_app_hash() {
        let settings = CodeSettings::new().with_app_hash();
        assert!(settings.allow_app_hash);
        assert!(settings.flags & 0x10 != 0);
        assert!(settings.is_app_hash_allowed());
    }

    #[test]
    fn test_code_settings_with_missed_call() {
        let settings = CodeSettings::new().with_missed_call();
        assert!(settings.allow_missed_call);
        assert!(settings.flags & 0x20 != 0);
        assert!(settings.is_missed_call_allowed());
    }

    #[test]
    fn test_code_settings_with_firebase() {
        let settings = CodeSettings::new().with_firebase();
        assert!(settings.allow_firebase);
        assert!(settings.flags & 0x80 != 0);
        assert!(settings.is_firebase_allowed());
    }

    #[test]
    fn test_code_settings_with_unknown_number() {
        let settings = CodeSettings::new().with_unknown_number();
        assert!(settings.unknown_number);
        assert!(settings.flags & 0x200 != 0);
        assert!(settings.is_unknown_number());
    }

    #[test]
    fn test_code_settings_with_logout_tokens() {
        let tokens = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let settings = CodeSettings::new().with_logout_tokens(tokens);
        assert!(settings.logout_tokens.is_some());
        assert!(settings.flags & 0x40 != 0);
        assert_eq!(settings.logout_tokens.unwrap().len(), 2);
    }

    #[test]
    fn test_code_settings_with_token() {
        let settings = CodeSettings::new().with_token("test_token".to_string(), true);
        assert_eq!(settings.token.as_deref(), Some("test_token"));
        assert_eq!(settings.app_sandbox, Some(true));
        assert!(settings.flags & 0x100 != 0);
    }

    #[test]
    fn test_code_settings_serialize_default() {
        let settings = CodeSettings::new();
        let mut buf = BytesMut::new();
        let result = settings.serialize_tl(&mut buf);
        assert!(result.is_ok());
        // Should only contain flags (4 bytes)
        assert_eq!(buf.len(), 4);
    }

    #[test]
    fn test_code_settings_serialize_with_flags() {
        let settings = CodeSettings::new()
            .with_flash_call()
            .with_current_number()
            .with_app_hash();

        let mut buf = BytesMut::new();
        let result = settings.serialize_tl(&mut buf);
        assert!(result.is_ok());

        // Flags: 0x1 | 0x2 | 0x10 = 0x13
        assert_eq!(buf[0], 0x13);
        assert_eq!(buf.len(), 4); // Only flags, no optional fields
    }

    #[test]
    fn test_code_settings_serialize_with_logout_tokens() {
        let tokens = vec![vec![1, 2, 3]];
        let settings = CodeSettings::new().with_logout_tokens(tokens);

        let mut buf = BytesMut::new();
        let result = settings.serialize_tl(&mut buf);
        assert!(result.is_ok());

        // Should contain: flags (4) + count (4) + token data (1 + padding + 3 = 4)
        assert!(buf.len() >= 12);
    }

    #[test]
    fn test_code_settings_serialize_with_token() {
        let settings = CodeSettings::new().with_token("test".to_string(), false);

        let mut buf = BytesMut::new();
        let result = settings.serialize_tl(&mut buf);
        assert!(result.is_ok());

        // Should contain: flags (4) + string length + string + bool (4)
        assert!(buf.len() >= 12);
    }

    #[test]
    fn test_code_settings_serialize_error_no_logout_tokens() {
        let mut settings = CodeSettings::new();
        settings.flags |= 0x40; // Set flag 6 but don't provide tokens

        let mut buf = BytesMut::new();
        let result = settings.serialize_tl(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_code_settings_serialize_error_no_token() {
        let mut settings = CodeSettings::new();
        settings.flags |= 0x100; // Set flag 8 but don't provide token

        let mut buf = BytesMut::new();
        let result = settings.serialize_tl(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_code_settings_clone() {
        let settings1 = CodeSettings::new()
            .with_flash_call()
            .with_token("token".to_string(), true);
        let settings2 = settings1.clone();
        assert_eq!(settings1.flags, settings2.flags);
        assert_eq!(settings1.token, settings2.token);
    }

    #[test]
    fn test_code_settings_equality() {
        let settings1 = CodeSettings::new().with_flash_call();
        let settings2 = CodeSettings::new().with_flash_call();
        assert_eq!(settings1, settings2);

        let settings3 = CodeSettings::new().with_missed_call();
        assert_ne!(settings1, settings3);
    }

    #[test]
    fn test_code_settings_multiple_flags() {
        let settings = CodeSettings::new()
            .with_flash_call()
            .with_current_number()
            .with_app_hash()
            .with_missed_call()
            .with_firebase()
            .with_unknown_number();

        // All flags should be set: 0x1 | 0x2 | 0x10 | 0x20 | 0x80 | 0x200 = 0x2B3
        assert_eq!(settings.flags, 0x2B3);
        assert!(settings.is_flash_call_allowed());
        assert!(settings.is_current_number());
        assert!(settings.is_app_hash_allowed());
        assert!(settings.is_missed_call_allowed());
        assert!(settings.is_firebase_allowed());
        assert!(settings.is_unknown_number());
    }
}
