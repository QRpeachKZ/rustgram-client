// Copyright 2025 rustgram-client contributors
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

//! # File Encryption Key
//!
//! Encryption key for encrypted files in TDLib.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `FileEncryptionKey` struct from `td/telegram/files/FileEncryptionKey.h`.
//!
//! ## Structure
//!
//! - `FileEncryptionKey`: Container for encryption key and IV
//! - `EncryptionKeyType`: Type of encryption (None, Secret, Secure)
//!
//! ## Usage
//!
//! ```
//! use rustgram_file_encryption_key::{FileEncryptionKey, EncryptionKeyType};
//!
//! // Create a new encryption key
//! let key = FileEncryptionKey::new(vec![0u8; 32], vec![0u8; 16], EncryptionKeyType::Secret);
//! assert!(key.is_secret());
//! assert!(!key.is_empty());
//! assert_eq!(key.size(), 48); // 32 bytes key + 16 bytes IV
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Key size for encryption (32 bytes = 256 bits)
pub const KEY_SIZE: usize = 32;

/// IV size for encryption (16 bytes = 128 bits)
pub const IV_SIZE: usize = 16;

/// Combined key + IV size
pub const KEY_IV_SIZE: usize = KEY_SIZE + IV_SIZE;

/// Encryption key type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum EncryptionKeyType {
    /// No encryption
    #[default]
    None,
    /// Secret chat encryption
    Secret,
    /// Secure storage encryption
    Secure,
}

impl fmt::Display for EncryptionKeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            EncryptionKeyType::None => "None",
            EncryptionKeyType::Secret => "Secret",
            EncryptionKeyType::Secure => "Secure",
        };
        write!(f, "{name}")
    }
}

/// File encryption key.
///
/// Contains the encryption key and initialization vector (IV) for encrypted files.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileEncryptionKey {
    /// Combined key and IV data
    key_iv: Vec<u8>,
    /// Type of encryption
    type_: EncryptionKeyType,
}

impl FileEncryptionKey {
    /// Creates a new file encryption key.
    ///
    /// # Arguments
    ///
    /// * `key` - Encryption key (32 bytes for AES-256)
    /// * `iv` - Initialization vector (16 bytes for AES)
    /// * `type_` - Type of encryption
    #[must_use]
    pub fn new(key: Vec<u8>, iv: Vec<u8>, type_: EncryptionKeyType) -> Self {
        let mut key_iv = key;
        key_iv.extend_from_slice(&iv);
        Self { key_iv, type_ }
    }

    /// Creates an empty encryption key.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            key_iv: Vec::new(),
            type_: EncryptionKeyType::None,
        }
    }

    /// Returns `true` if this is a secret chat encryption key.
    #[must_use]
    pub const fn is_secret(&self) -> bool {
        matches!(self.type_, EncryptionKeyType::Secret)
    }

    /// Returns `true` if this is a secure storage encryption key.
    #[must_use]
    pub const fn is_secure(&self) -> bool {
        matches!(self.type_, EncryptionKeyType::Secure)
    }

    /// Returns `true` if the key is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.key_iv.is_empty()
    }

    /// Returns the size of the key + IV data.
    #[must_use]
    pub fn size(&self) -> usize {
        self.key_iv.len()
    }

    /// Returns the encryption type.
    #[must_use]
    pub const fn type_(&self) -> EncryptionKeyType {
        self.type_
    }

    /// Returns a reference to the key data (first 32 bytes).
    #[must_use]
    pub fn key(&self) -> Option<&[u8]> {
        if self.key_iv.len() >= KEY_SIZE {
            Some(&self.key_iv[..KEY_SIZE])
        } else {
            None
        }
    }

    /// Returns a reference to the IV data (bytes 32-48).
    #[must_use]
    pub fn iv(&self) -> Option<&[u8]> {
        if self.key_iv.len() >= KEY_IV_SIZE {
            Some(&self.key_iv[KEY_SIZE..KEY_IV_SIZE])
        } else {
            None
        }
    }

    /// Returns a reference to the combined key + IV data.
    #[must_use]
    pub fn key_iv(&self) -> &[u8] {
        &self.key_iv
    }

    /// Calculates the fingerprint of the key.
    ///
    /// The fingerprint is a 32-bit integer derived from the key data.
    #[must_use]
    pub fn calc_fingerprint(&self) -> i32 {
        if self.key_iv.len() < 4 {
            return 0;
        }
        // Simple XOR-based fingerprint (TDLib uses a more complex calculation)
        let mut fingerprint: i32 = 0;
        for (i, &byte) in self.key_iv.iter().enumerate().take(16) {
            fingerprint ^= (byte as i32) << ((i % 4) * 8);
        }
        fingerprint
    }

    /// Creates a mutable reference to the IV for modification.
    ///
    /// # Safety
    ///
    /// This method is intended for internal use when updating the IV during encryption.
    #[must_use]
    pub fn mutable_iv(&mut self) -> &mut [u8] {
        if self.key_iv.len() < KEY_IV_SIZE {
            // Extend if needed
            self.key_iv.resize(KEY_IV_SIZE, 0);
        }
        &mut self.key_iv[KEY_SIZE..]
    }
}

impl Default for FileEncryptionKey {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for FileEncryptionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[FileEncryptionKey type: {}, size: {}, fingerprint: {}]",
            self.type_,
            self.size(),
            self.calc_fingerprint()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rstest::rstest;

    // === Basic trait tests ===

    #[test]
    fn test_clone() {
        let key = FileEncryptionKey::new(vec![1u8; 32], vec![2u8; 16], EncryptionKeyType::Secret);
        let cloned = key.clone();
        assert_eq!(key, cloned);
    }

    #[test]
    fn test_default() {
        let key = FileEncryptionKey::default();
        assert!(key.is_empty());
        assert_eq!(key.type_(), EncryptionKeyType::None);
    }

    // === Constructor tests ===

    #[test]
    fn test_new() {
        let key = FileEncryptionKey::new(vec![1u8; 32], vec![2u8; 16], EncryptionKeyType::Secret);
        assert!(!key.is_empty());
        assert!(key.is_secret());
        assert!(!key.is_secure());
        assert_eq!(key.size(), 48);
    }

    #[test]
    fn test_empty() {
        let key = FileEncryptionKey::empty();
        assert!(key.is_empty());
        assert!(!key.is_secret());
        assert!(!key.is_secure());
        assert_eq!(key.size(), 0);
    }

    // === Type tests ===

    #[rstest]
    #[case(EncryptionKeyType::None, false, false)]
    #[case(EncryptionKeyType::Secret, true, false)]
    #[case(EncryptionKeyType::Secure, false, true)]
    fn test_type_checks(
        #[case] type_: EncryptionKeyType,
        #[case] is_secret: bool,
        #[case] is_secure: bool,
    ) {
        let key = FileEncryptionKey::new(vec![1u8; 32], vec![2u8; 16], type_);
        assert_eq!(key.is_secret(), is_secret);
        assert_eq!(key.is_secure(), is_secure);
        assert_eq!(key.type_(), type_);
    }

    // === Key/IV access tests ===

    #[test]
    fn test_key_iv_access() {
        let key_data = vec![1u8; 32];
        let iv_data = vec![2u8; 16];
        let key =
            FileEncryptionKey::new(key_data.clone(), iv_data.clone(), EncryptionKeyType::Secret);

        assert_eq!(key.key(), Some(&key_data[..]));
        assert_eq!(key.iv(), Some(&iv_data[..]));
        assert_eq!(key.key_iv().len(), 48);
    }

    #[test]
    fn test_key_iv_access_empty() {
        let key = FileEncryptionKey::empty();
        assert_eq!(key.key(), None);
        assert_eq!(key.iv(), None);
        assert_eq!(key.key_iv().len(), 0);
    }

    #[test]
    fn test_key_iv_access_partial() {
        let key = FileEncryptionKey::new(vec![1u8; 10], vec![], EncryptionKeyType::Secret);
        assert_eq!(key.key(), None); // Less than 32 bytes
        assert_eq!(key.iv(), None);
    }

    // === Fingerprint tests ===

    #[test]
    fn test_calc_fingerprint() {
        let key = FileEncryptionKey::new(vec![1u8; 32], vec![2u8; 16], EncryptionKeyType::Secret);
        let fingerprint = key.calc_fingerprint();
        // Fingerprint should be deterministic
        assert_eq!(key.calc_fingerprint(), fingerprint);
    }

    #[test]
    fn test_calc_fingerprint_empty() {
        let key = FileEncryptionKey::empty();
        assert_eq!(key.calc_fingerprint(), 0);
    }

    #[test]
    fn test_calc_fingerprint_different_keys() {
        let key1 = FileEncryptionKey::new(
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32,
            ],
            vec![2u8; 16],
            EncryptionKeyType::Secret,
        );
        let key2 = FileEncryptionKey::new(
            vec![
                5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
            ],
            vec![4u8; 16],
            EncryptionKeyType::Secret,
        );
        assert_ne!(key1.calc_fingerprint(), key2.calc_fingerprint());
    }

    // === Mutable IV tests ===

    #[test]
    fn test_mutable_iv() {
        let mut key =
            FileEncryptionKey::new(vec![1u8; 32], vec![2u8; 16], EncryptionKeyType::Secret);
        let iv = key.mutable_iv();
        assert_eq!(iv.len(), 16);
        iv[0] = 99;
        assert_eq!(key.iv().unwrap()[0], 99);
    }

    #[test]
    fn test_mutable_iv_expands() {
        let mut key = FileEncryptionKey::new(vec![1u8; 32], vec![], EncryptionKeyType::Secret);
        let iv = key.mutable_iv();
        assert_eq!(iv.len(), 16);
        assert_eq!(key.size(), 48);
    }

    // === Display tests ===

    #[rstest]
    #[case(EncryptionKeyType::None, "None")]
    #[case(EncryptionKeyType::Secret, "Secret")]
    #[case(EncryptionKeyType::Secure, "Secure")]
    fn test_display_type(#[case] type_: EncryptionKeyType, #[case] expected: &str) {
        assert_eq!(format!("{type_}"), expected);
    }

    #[test]
    fn test_display_key() {
        let key = FileEncryptionKey::new(vec![1u8; 32], vec![2u8; 16], EncryptionKeyType::Secret);
        let s = format!("{key}");
        assert!(s.contains("Secret"));
        assert!(s.contains("48"));
    }

    // === Equality tests ===

    #[test]
    fn test_equality() {
        let key1 = FileEncryptionKey::new(vec![1u8; 32], vec![2u8; 16], EncryptionKeyType::Secret);
        let key2 = FileEncryptionKey::new(vec![1u8; 32], vec![2u8; 16], EncryptionKeyType::Secret);
        let key3 = FileEncryptionKey::new(vec![3u8; 32], vec![4u8; 16], EncryptionKeyType::Secret);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize() {
        let key = FileEncryptionKey::new(vec![1u8; 32], vec![2u8; 16], EncryptionKeyType::Secret);
        let json = serde_json::to_string(&key).unwrap();
        // Should contain the key_iv data
        assert!(json.contains("key_iv"));
    }

    #[test]
    fn test_deserialize() {
        let key = FileEncryptionKey::new(vec![1u8; 32], vec![2u8; 16], EncryptionKeyType::Secret);
        let json = serde_json::to_string(&key).unwrap();
        let deserialized: FileEncryptionKey = serde_json::from_str(&json).unwrap();
        assert_eq!(key, deserialized);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let original =
            FileEncryptionKey::new(vec![5u8; 32], vec![6u8; 16], EncryptionKeyType::Secure);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: FileEncryptionKey = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // === Property-based tests ===

    proptest! {
        #[test]
        fn test_key_size_is_48(key in proptest::collection::vec(0u8..=0, 32), iv in proptest::collection::vec(0u8..=0, 16)) {
            let enc_key = FileEncryptionKey::new(key, iv, EncryptionKeyType::Secret);
            prop_assert_eq!(enc_key.size(), 48);
        }

        #[test]
        fn test_fingerprint_deterministic(key in proptest::collection::vec(any::<u8>(), 32), iv in proptest::collection::vec(any::<u8>(), 16)) {
            let enc_key = FileEncryptionKey::new(key, iv, EncryptionKeyType::Secret);
            let fp1 = enc_key.calc_fingerprint();
            let fp2 = enc_key.calc_fingerprint();
            prop_assert_eq!(fp1, fp2);
        }

        #[test]
        fn test_empty_key_has_size_0(key in proptest::collection::vec(any::<u8>(), 0)) {
            let enc_key = FileEncryptionKey::new(key, vec![], EncryptionKeyType::None);
            prop_assert!(enc_key.is_empty());
            prop_assert_eq!(enc_key.size(), 0);
        }

        #[test]
        fn test_roundtrip_serialization(
            key in proptest::collection::vec(any::<u8>(), 32),
            iv in proptest::collection::vec(any::<u8>(), 16),
            type_ in prop_oneof![Just(EncryptionKeyType::None), Just(EncryptionKeyType::Secret), Just(EncryptionKeyType::Secure)]
        ) {
            let original = FileEncryptionKey::new(key, iv, type_);
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: FileEncryptionKey = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(original, deserialized);
        }
    }
}
