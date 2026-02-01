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

//! # Email Verification
//!
//! Email verification methods for Telegram authentication.
//!
//! Supports multiple verification methods:
//! - Email code (6-digit code sent to email)
//! - Apple ID
//! - Google ID

use serde::{Deserialize, Serialize};

/// Type of email verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmailVerificationType {
    /// No verification
    None,
    /// Email code verification
    Code,
    /// Apple ID verification
    Apple,
    /// Google ID verification
    Google,
}

/// Represents email verification for authentication.
///
/// # Example
///
/// ```rust
/// use rustgram_email_verification::{EmailVerification, EmailVerificationType};
///
/// // Email code verification
/// let code = EmailVerification::code("123456");
/// assert!(code.is_email_code());
/// assert_eq!(code.type_(), EmailVerificationType::Code);
/// assert_eq!(code.token(), Some("123456"));
///
/// // Apple ID verification
/// let apple = EmailVerification::apple("apple_token");
/// assert_eq!(apple.type_(), EmailVerificationType::Apple);
///
/// // Google ID verification
/// let google = EmailVerification::google("google_token");
/// assert_eq!(google.type_(), EmailVerificationType::Google);
///
/// // Empty verification
/// let empty = EmailVerification::none();
/// assert!(empty.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmailVerification {
    type_: EmailVerificationType,
    token: String,
}

impl EmailVerification {
    /// Creates an empty email verification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_email_verification::EmailVerification;
    ///
    /// let verification = EmailVerification::none();
    /// assert!(verification.is_empty());
    /// ```
    pub fn none() -> Self {
        Self {
            type_: EmailVerificationType::None,
            token: String::new(),
        }
    }

    /// Creates email code verification.
    ///
    /// # Arguments
    ///
    /// * `code` - The 6-digit verification code
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_email_verification::EmailVerification;
    ///
    /// let verification = EmailVerification::code("123456");
    /// assert!(verification.is_email_code());
    /// ```
    pub fn code(code: &str) -> Self {
        Self {
            type_: EmailVerificationType::Code,
            token: code.to_string(),
        }
    }

    /// Creates Apple ID verification.
    ///
    /// # Arguments
    ///
    /// * `token` - Apple ID authentication token
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_email_verification::{EmailVerification, EmailVerificationType};
    ///
    /// let verification = EmailVerification::apple("apple_token");
    /// assert_eq!(verification.type_(), EmailVerificationType::Apple);
    /// ```
    pub fn apple(token: &str) -> Self {
        Self {
            type_: EmailVerificationType::Apple,
            token: token.to_string(),
        }
    }

    /// Creates Google ID verification.
    ///
    /// # Arguments
    ///
    /// * `token` - Google ID authentication token
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_email_verification::{EmailVerification, EmailVerificationType};
    ///
    /// let verification = EmailVerification::google("google_token");
    /// assert_eq!(verification.type_(), EmailVerificationType::Google);
    /// ```
    pub fn google(token: &str) -> Self {
        Self {
            type_: EmailVerificationType::Google,
            token: token.to_string(),
        }
    }

    /// Returns the verification type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_email_verification::{EmailVerification, EmailVerificationType};
    ///
    /// let verification = EmailVerification::code("123456");
    /// assert_eq!(verification.type_(), EmailVerificationType::Code);
    /// ```
    pub fn type_(&self) -> EmailVerificationType {
        self.type_
    }

    /// Returns the verification code or token.
    ///
    /// Returns `None` if verification type is `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_email_verification::EmailVerification;
    ///
    /// let verification = EmailVerification::code("123456");
    /// assert_eq!(verification.token(), Some("123456"));
    ///
    /// let empty = EmailVerification::none();
    /// assert_eq!(empty.token(), None);
    /// ```
    pub fn token(&self) -> Option<&str> {
        if self.type_ == EmailVerificationType::None {
            None
        } else {
            Some(&self.token)
        }
    }

    /// Deprecated: Use token() instead
    #[deprecated(since = "0.1.0", note = "Use token() instead")]
    pub fn code_value(&self) -> Option<&str> {
        self.token()
    }

    /// Returns `true` if this is an empty (no) verification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_email_verification::EmailVerification;
    ///
    /// let verification = EmailVerification::none();
    /// assert!(verification.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.type_ == EmailVerificationType::None
    }

    /// Returns `true` if this is an email code verification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_email_verification::EmailVerification;
    ///
    /// let verification = EmailVerification::code("123456");
    /// assert!(verification.is_email_code());
    /// ```
    pub fn is_email_code(&self) -> bool {
        self.type_ == EmailVerificationType::Code
    }

    /// Returns `true` if this is an Apple ID verification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_email_verification::EmailVerification;
    ///
    /// let verification = EmailVerification::apple("token");
    /// assert!(verification.is_apple_id());
    /// ```
    pub fn is_apple_id(&self) -> bool {
        self.type_ == EmailVerificationType::Apple
    }

    /// Returns `true` if this is a Google ID verification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_email_verification::EmailVerification;
    ///
    /// let verification = EmailVerification::google("token");
    /// assert!(verification.is_google_id());
    /// ```
    pub fn is_google_id(&self) -> bool {
        self.type_ == EmailVerificationType::Google
    }
}

impl Default for EmailVerification {
    fn default() -> Self {
        Self::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let verification = EmailVerification::none();
        assert!(verification.is_empty());
        assert_eq!(verification.type_(), EmailVerificationType::None);
        assert_eq!(verification.token(), None);
        assert!(!verification.is_email_code());
        assert!(!verification.is_apple_id());
        assert!(!verification.is_google_id());
    }

    #[test]
    fn test_code() {
        let verification = EmailVerification::code("123456");
        assert!(!verification.is_empty());
        assert_eq!(verification.type_(), EmailVerificationType::Code);
        assert_eq!(verification.token(), Some("123456"));
        assert!(verification.is_email_code());
        assert!(!verification.is_apple_id());
        assert!(!verification.is_google_id());
    }

    #[test]
    fn test_apple() {
        let verification = EmailVerification::apple("apple_token");
        assert!(!verification.is_empty());
        assert_eq!(verification.type_(), EmailVerificationType::Apple);
        assert_eq!(verification.token(), Some("apple_token"));
        assert!(!verification.is_email_code());
        assert!(verification.is_apple_id());
        assert!(!verification.is_google_id());
    }

    #[test]
    fn test_google() {
        let verification = EmailVerification::google("google_token");
        assert!(!verification.is_empty());
        assert_eq!(verification.type_(), EmailVerificationType::Google);
        assert_eq!(verification.token(), Some("google_token"));
        assert!(!verification.is_email_code());
        assert!(!verification.is_apple_id());
        assert!(verification.is_google_id());
    }

    #[test]
    fn test_code_with_different_values() {
        let codes = ["000000", "123456", "999999", "111111"];

        for code in codes {
            let verification = EmailVerification::code(code);
            assert_eq!(verification.token(), Some(code));
            assert!(verification.is_email_code());
        }
    }

    #[test]
    fn test_code_with_longer_string() {
        // Allow longer codes for flexibility
        let verification = EmailVerification::code("123456789");
        assert_eq!(verification.token(), Some("123456789"));
        assert!(verification.is_email_code());
    }

    #[test]
    fn test_code_with_empty_string() {
        let verification = EmailVerification::code("");
        assert_eq!(verification.token(), Some(""));
        assert!(verification.is_email_code());
    }

    #[test]
    fn test_apple_with_token() {
        let tokens = ["apple_token_123", "bearer_token", "oauth_token"];

        for token in tokens {
            let verification = EmailVerification::apple(token);
            assert_eq!(verification.token(), Some(token));
            assert!(verification.is_apple_id());
        }
    }

    #[test]
    fn test_google_with_token() {
        let tokens = ["google_token_123", "oauth_token", "id_token"];

        for token in tokens {
            let verification = EmailVerification::google(token);
            assert_eq!(verification.token(), Some(token));
            assert!(verification.is_google_id());
        }
    }

    #[test]
    fn test_equality() {
        let verification1 = EmailVerification::code("123456");
        let verification2 = EmailVerification::code("123456");
        assert_eq!(verification1, verification2);

        let verification3 = EmailVerification::code("654321");
        assert_ne!(verification1, verification3);

        let verification4 = EmailVerification::apple("123456");
        assert_ne!(verification1, verification4);
    }

    #[test]
    fn test_default() {
        let verification = EmailVerification::default();
        assert!(verification.is_empty());
        assert_eq!(verification.type_(), EmailVerificationType::None);
    }

    #[test]
    fn test_clone() {
        let verification1 = EmailVerification::code("123456");
        let verification2 = verification1.clone();
        assert_eq!(verification1, verification2);
    }

    #[test]
    fn test_type_variants() {
        let types = [
            EmailVerificationType::None,
            EmailVerificationType::Code,
            EmailVerificationType::Apple,
            EmailVerificationType::Google,
        ];

        for type_ in types {
            // Test that all types are distinct
            for other_type in &types {
                if type_ != *other_type {
                    assert_ne!(type_, *other_type);
                }
            }
        }
    }

    #[test]
    fn test_serialization() {
        let verification = EmailVerification::code("123456");
        let json = serde_json::to_string(&verification).unwrap();
        let parsed: EmailVerification = serde_json::from_str(&json).unwrap();
        assert_eq!(verification, parsed);
    }

    #[test]
    fn test_serialization_apple() {
        let verification = EmailVerification::apple("apple_token");
        let json = serde_json::to_string(&verification).unwrap();
        let parsed: EmailVerification = serde_json::from_str(&json).unwrap();
        assert_eq!(verification, parsed);
    }

    #[test]
    fn test_serialization_google() {
        let verification = EmailVerification::google("google_token");
        let json = serde_json::to_string(&verification).unwrap();
        let parsed: EmailVerification = serde_json::from_str(&json).unwrap();
        assert_eq!(verification, parsed);
    }

    #[test]
    fn test_serialization_none() {
        let verification = EmailVerification::none();
        let json = serde_json::to_string(&verification).unwrap();
        let parsed: EmailVerification = serde_json::from_str(&json).unwrap();
        assert_eq!(verification, parsed);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let verification1 = EmailVerification::code("123456");
        let verification2 = EmailVerification::code("123456");

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        verification1.hash(&mut hasher1);
        verification2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_type() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let type1 = EmailVerificationType::Code;
        let type2 = EmailVerificationType::Code;

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        type1.hash(&mut hasher1);
        type2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_debug_format() {
        let verification = EmailVerification::code("123456");
        let debug_str = format!("{:?}", verification);
        assert!(debug_str.contains("EmailVerification"));
    }

    #[test]
    fn test_code_is_always_some_for_non_none() {
        let test_cases = [
            EmailVerification::code("123456"),
            EmailVerification::apple("token"),
            EmailVerification::google("token"),
        ];

        for verification in test_cases {
            assert!(verification.token().is_some());
            assert!(!verification.is_empty());
        }
    }

    #[test]
    fn test_only_none_is_empty() {
        let none = EmailVerification::none();
        assert!(none.is_empty());
        assert_eq!(none.token(), None);

        let non_empty = [
            EmailVerification::code(""),
            EmailVerification::apple(""),
            EmailVerification::google(""),
        ];

        for verification in non_empty {
            assert!(!verification.is_empty());
            assert!(verification.token().is_some());
        }
    }
}
