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

//! # New Password State
//!
//! Password state for Telegram 2FA (Two-Factor Authentication).
//!
//! Based on TDLib's NewPasswordState implementation.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Minimum size for secure salt (in bytes).
const MIN_SECURE_SALT_SIZE: usize = 8;

/// Minimum size for regular salt (in bytes).
const MIN_SALT_SIZE: usize = 8;

/// New password state for 2FA.
///
/// Contains the cryptographic parameters needed for setting up
/// Two-Factor Authentication (2FA) password in Telegram.
///
/// # Example
///
/// ```rust
/// use rustgram_new_password_state::NewPasswordState;
///
/// let state = NewPasswordState::new(
///     vec![1u8; 16],
///     vec![2u8; 16],
///     vec![3u8; 16],
///     vec![4u8; 16],
///     2,
/// );
/// assert!(state.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct NewPasswordState {
    /// Client salt for password derivation
    client_salt: Vec<u8>,
    /// Server salt for password derivation
    server_salt: Vec<u8>,
    /// SRP prime modulus (p parameter)
    srp_p: Vec<u8>,
    /// Secure salt for additional encryption
    secure_salt: Vec<u8>,
    /// SRP generator (g parameter)
    srp_g: i32,
}

impl NewPasswordState {
    /// Creates a new password state.
    ///
    /// # Arguments
    ///
    /// * `client_salt` - Client salt bytes
    /// * `server_salt` - Server salt bytes
    /// * `srp_p` - SRP prime modulus bytes
    /// * `secure_salt` - Secure salt bytes
    /// * `srp_g` - SRP generator value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_new_password_state::NewPasswordState;
    ///
    /// let state = NewPasswordState::new(
    ///     vec![1u8; 16],
    ///     vec![2u8; 16],
    ///     vec![3u8; 16],
    ///     vec![4u8; 16],
    ///     2,
    /// );
    /// assert!(state.is_valid());
    /// ```
    pub fn new(
        client_salt: Vec<u8>,
        server_salt: Vec<u8>,
        srp_p: Vec<u8>,
        secure_salt: Vec<u8>,
        srp_g: i32,
    ) -> Self {
        Self {
            client_salt,
            server_salt,
            srp_p,
            secure_salt,
            srp_g,
        }
    }

    /// Returns the client salt.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_new_password_state::NewPasswordState;
    ///
    /// let state = NewPasswordState::new(
    ///     vec![1u8; 16],
    ///     vec![2u8; 16],
    ///     vec![3u8; 16],
    ///     vec![4u8; 16],
    ///     2,
    /// );
    /// assert_eq!(state.client_salt(), &[1u8; 16]);
    /// ```
    pub fn client_salt(&self) -> &[u8] {
        &self.client_salt
    }

    /// Returns the server salt.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_new_password_state::NewPasswordState;
    ///
    /// let state = NewPasswordState::new(
    ///     vec![1u8; 16],
    ///     vec![2u8; 16],
    ///     vec![3u8; 16],
    ///     vec![4u8; 16],
    ///     2,
    /// );
    /// assert_eq!(state.server_salt(), &[2u8; 16]);
    /// ```
    pub fn server_salt(&self) -> &[u8] {
        &self.server_salt
    }

    /// Returns the SRP prime modulus (p parameter).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_new_password_state::NewPasswordState;
    ///
    /// let state = NewPasswordState::new(
    ///     vec![1u8; 16],
    ///     vec![2u8; 16],
    ///     vec![3u8; 16],
    ///     vec![4u8; 16],
    ///     2,
    /// );
    /// assert_eq!(state.srp_p(), &[3u8; 16]);
    /// ```
    pub fn srp_p(&self) -> &[u8] {
        &self.srp_p
    }

    /// Returns the secure salt.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_new_password_state::NewPasswordState;
    ///
    /// let state = NewPasswordState::new(
    ///     vec![1u8; 16],
    ///     vec![2u8; 16],
    ///     vec![3u8; 16],
    ///     vec![4u8; 16],
    ///     2,
    /// );
    /// assert_eq!(state.secure_salt(), &[4u8; 16]);
    /// ```
    pub fn secure_salt(&self) -> &[u8] {
        &self.secure_salt
    }

    /// Returns the SRP generator (g parameter).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_new_password_state::NewPasswordState;
    ///
    /// let state = NewPasswordState::new(
    ///     vec![1u8; 16],
    ///     vec![2u8; 16],
    ///     vec![3u8; 16],
    ///     vec![4u8; 16],
    ///     2,
    /// );
    /// assert_eq!(state.srp_g(), 2);
    /// ```
    pub fn srp_g(&self) -> i32 {
        self.srp_g
    }

    /// Sets the client salt.
    ///
    /// # Arguments
    ///
    /// * `salt` - New client salt bytes
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_new_password_state::NewPasswordState;
    ///
    /// let mut state = NewPasswordState::new(
    ///     vec![1u8; 16],
    ///     vec![2u8; 16],
    ///     vec![3u8; 16],
    ///     vec![4u8; 16],
    ///     2,
    /// );
    /// state.set_client_salt(vec![5u8; 16]);
    /// assert_eq!(state.client_salt(), &[5u8; 16]);
    /// ```
    pub fn set_client_salt(&mut self, salt: Vec<u8>) {
        self.client_salt = salt;
    }

    /// Sets the server salt.
    ///
    /// # Arguments
    ///
    /// * `salt` - New server salt bytes
    pub fn set_server_salt(&mut self, salt: Vec<u8>) {
        self.server_salt = salt;
    }

    /// Sets the SRP prime modulus.
    ///
    /// # Arguments
    ///
    /// * `p` - New SRP prime modulus bytes
    pub fn set_srp_p(&mut self, p: Vec<u8>) {
        self.srp_p = p;
    }

    /// Sets the secure salt.
    ///
    /// # Arguments
    ///
    /// * `salt` - New secure salt bytes
    pub fn set_secure_salt(&mut self, salt: Vec<u8>) {
        self.secure_salt = salt;
    }

    /// Sets the SRP generator.
    ///
    /// # Arguments
    ///
    /// * `g` - New SRP generator value
    pub fn set_srp_g(&mut self, g: i32) {
        self.srp_g = g;
    }

    /// Validates this password state.
    ///
    /// Returns `Ok(())` if the state is valid, or an error describing the issue.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The secure salt is too small (< 8 bytes)
    /// - The client salt is too small (< 8 bytes)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_new_password_state::NewPasswordState;
    ///
    /// let state = NewPasswordState::new(
    ///     vec![1u8; 16],
    ///     vec![2u8; 16],
    ///     vec![3u8; 16],
    ///     vec![4u8; 16],
    ///     2,
    /// );
    /// assert!(state.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), NewPasswordStateError> {
        if self.secure_salt.len() < MIN_SECURE_SALT_SIZE {
            return Err(NewPasswordStateError::SecureSaltTooSmall {
                actual: self.secure_salt.len(),
                required: MIN_SECURE_SALT_SIZE,
            });
        }

        if self.client_salt.len() < MIN_SALT_SIZE {
            return Err(NewPasswordStateError::SaltTooSmall {
                actual: self.client_salt.len(),
                required: MIN_SALT_SIZE,
            });
        }

        Ok(())
    }

    /// Returns `true` if this password state is valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_new_password_state::NewPasswordState;
    ///
    /// let state = NewPasswordState::new(
    ///     vec![1u8; 16],
    ///     vec![2u8; 16],
    ///     vec![3u8; 16],
    ///     vec![4u8; 16],
    ///     2,
    /// );
    /// assert!(state.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Returns the length of the client salt.
    pub fn client_salt_len(&self) -> usize {
        self.client_salt.len()
    }

    /// Returns the length of the server salt.
    pub fn server_salt_len(&self) -> usize {
        self.server_salt.len()
    }

    /// Returns the length of the SRP prime modulus.
    pub fn srp_p_len(&self) -> usize {
        self.srp_p.len()
    }

    /// Returns the length of the secure salt.
    pub fn secure_salt_len(&self) -> usize {
        self.secure_salt.len()
    }
}

impl fmt::Display for NewPasswordState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NewPasswordState {{ client_salt: {} bytes, server_salt: {} bytes, srp_p: {} bytes, secure_salt: {} bytes, srp_g: {} }}",
            self.client_salt.len(),
            self.server_salt.len(),
            self.srp_p.len(),
            self.secure_salt.len(),
            self.srp_g
        )
    }
}

/// Errors that can occur when validating a new password state.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NewPasswordStateError {
    /// Secure salt is too small
    #[error("secure salt length too small: {actual} < {required}")]
    SecureSaltTooSmall {
        /// Actual salt length
        actual: usize,
        /// Required minimum salt length
        required: usize,
    },

    /// Client salt is too small
    #[error("salt length too small: {actual} < {required}")]
    SaltTooSmall {
        /// Actual salt length
        actual: usize,
        /// Required minimum salt length
        required: usize,
    },
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn test_new_password_state_new() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );

        assert_eq!(state.client_salt(), &[1u8; 16]);
        assert_eq!(state.server_salt(), &[2u8; 16]);
        assert_eq!(state.srp_p(), &[3u8; 16]);
        assert_eq!(state.secure_salt(), &[4u8; 16]);
        assert_eq!(state.srp_g(), 2);
    }

    #[test]
    fn test_new_password_state_is_valid() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        assert!(state.is_valid());
    }

    #[test]
    fn test_new_password_state_secure_salt_too_small() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 4], // Too small
            2,
        );
        assert!(!state.is_valid());

        let result = state.validate();
        assert!(result.is_err());
        match result {
            Err(NewPasswordStateError::SecureSaltTooSmall { actual, required }) => {
                assert_eq!(actual, 4);
                assert_eq!(required, MIN_SECURE_SALT_SIZE);
            }
            _ => panic!("Expected SecureSaltTooSmall error"),
        }
    }

    #[test]
    fn test_new_password_state_client_salt_too_small() {
        let state = NewPasswordState::new(
            vec![1u8; 4], // Too small
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        assert!(!state.is_valid());

        let result = state.validate();
        assert!(result.is_err());
        match result {
            Err(NewPasswordStateError::SaltTooSmall { actual, required }) => {
                assert_eq!(actual, 4);
                assert_eq!(required, MIN_SALT_SIZE);
            }
            _ => panic!("Expected SaltTooSmall error"),
        }
    }

    #[test]
    fn test_new_password_state_min_sizes() {
        let state = NewPasswordState::new(
            vec![1u8; MIN_SALT_SIZE],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; MIN_SECURE_SALT_SIZE],
            2,
        );
        assert!(state.is_valid());
    }

    #[test]
    fn test_new_password_state_empty_srp_p() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![], // Empty SRP P
            vec![4u8; 16],
            2,
        );
        // Empty SRP P is technically valid according to our validation
        // (though it wouldn't work in practice)
        assert!(state.validate().is_ok());
    }

    #[test]
    fn test_new_password_state_empty_server_salt() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![], // Empty server salt
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        // Empty server salt is technically valid according to our validation
        assert!(state.validate().is_ok());
    }

    #[test]
    fn test_set_client_salt() {
        let mut state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        state.set_client_salt(vec![5u8; 16]);
        assert_eq!(state.client_salt(), &[5u8; 16]);
    }

    #[test]
    fn test_set_server_salt() {
        let mut state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        state.set_server_salt(vec![5u8; 16]);
        assert_eq!(state.server_salt(), &[5u8; 16]);
    }

    #[test]
    fn test_set_srp_p() {
        let mut state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        state.set_srp_p(vec![5u8; 16]);
        assert_eq!(state.srp_p(), &[5u8; 16]);
    }

    #[test]
    fn test_set_secure_salt() {
        let mut state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        state.set_secure_salt(vec![5u8; 16]);
        assert_eq!(state.secure_salt(), &[5u8; 16]);
    }

    #[test]
    fn test_set_srp_g() {
        let mut state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        state.set_srp_g(5);
        assert_eq!(state.srp_g(), 5);
    }

    #[test]
    fn test_equality() {
        let state1 = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        let state2 = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        assert_eq!(state1, state2);

        let state3 = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![5u8; 16], // Different
            2,
        );
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_default() {
        let state = NewPasswordState::default();
        assert!(!state.is_valid()); // Empty salts are invalid
    }

    #[test]
    fn test_clone() {
        let state1 = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        let state2 = state1.clone();
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let state1 = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        let state2 = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        state1.hash(&mut hasher1);
        state2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_serialization() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );

        let json = serde_json::to_string(&state).unwrap();
        let parsed: NewPasswordState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, parsed);
    }

    #[test]
    fn test_display() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            2,
        );
        let display = format!("{}", state);
        assert!(display.contains("16"));
        assert!(display.contains("2"));
    }

    #[test]
    fn test_length_methods() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 24],
            vec![3u8; 32],
            vec![4u8; 40],
            2,
        );

        assert_eq!(state.client_salt_len(), 16);
        assert_eq!(state.server_salt_len(), 24);
        assert_eq!(state.srp_p_len(), 32);
        assert_eq!(state.secure_salt_len(), 40);
    }

    #[test]
    fn test_negative_srp_g() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            -1, // Negative g (though not typical)
        );
        assert!(state.is_valid()); // Validation doesn't check g value
        assert_eq!(state.srp_g(), -1);
    }

    #[test]
    fn test_large_srp_g() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            i32::MAX,
        );
        assert!(state.is_valid());
        assert_eq!(state.srp_g(), i32::MAX);
    }

    #[test]
    fn test_zero_srp_g() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; 16],
            0,
        );
        assert!(state.is_valid());
        assert_eq!(state.srp_g(), 0);
    }

    #[test]
    fn test_error_display() {
        let error = NewPasswordStateError::SecureSaltTooSmall {
            actual: 4,
            required: 8,
        };
        let display = format!("{}", error);
        assert!(display.contains("4"));
        assert!(display.contains("8"));
    }

    #[test]
    fn test_salt_error_too_small() {
        let error = NewPasswordStateError::SaltTooSmall {
            actual: 4,
            required: 8,
        };
        let display = format!("{}", error);
        assert!(display.contains("4"));
        assert!(display.contains("8"));
    }

    #[test]
    fn test_both_errors_equality() {
        let error1 = NewPasswordStateError::SecureSaltTooSmall {
            actual: 4,
            required: 8,
        };
        let error2 = NewPasswordStateError::SecureSaltTooSmall {
            actual: 4,
            required: 8,
        };
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_salt_size_exactly_minimum() {
        let state = NewPasswordState::new(
            vec![1u8; MIN_SALT_SIZE],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; MIN_SECURE_SALT_SIZE],
            2,
        );
        assert!(state.is_valid());
    }

    #[test]
    fn test_salt_size_one_below_minimum() {
        let state = NewPasswordState::new(
            vec![1u8; MIN_SALT_SIZE - 1],
            vec![2u8; 16],
            vec![3u8; 16],
            vec![4u8; MIN_SECURE_SALT_SIZE],
            2,
        );
        assert!(!state.is_valid());
    }

    #[test]
    fn test_large_salt_sizes() {
        let state = NewPasswordState::new(
            vec![1u8; 128],
            vec![2u8; 128],
            vec![3u8; 256],
            vec![4u8; 128],
            2,
        );
        assert!(state.is_valid());
    }

    #[test]
    fn test_all_field_getters() {
        let state = NewPasswordState::new(
            vec![1u8; 16],
            vec![2u8; 24],
            vec![3u8; 32],
            vec![4u8; 40],
            5,
        );
        assert_eq!(state.client_salt_len(), 16);
        assert_eq!(state.server_salt_len(), 24);
        assert_eq!(state.srp_p_len(), 32);
        assert_eq!(state.secure_salt_len(), 40);
        assert_eq!(state.srp_g(), 5);
    }
}
