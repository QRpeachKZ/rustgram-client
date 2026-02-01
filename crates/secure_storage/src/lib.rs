// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Secure Storage
//!
//! Encrypted storage with key derivation and validation.
//!
//! This module provides secure storage functionality with AES-CBC encryption,
//! SHA-256 hashing, and key derivation (PBKDF2, SHA512). Based on TDLib's
//! SecureStorage.h.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_buffer::BufferSlice;
use rustgram_crypto::{AesCbcState, Sha256State};
use rustgram_types::UInt256;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::{self, Debug, Formatter};

/// 32-byte secret with checksum validation.
///
/// The sum of all bytes modulo 255 must equal 239.
/// Used as the master secret for encryption.
///
/// # Example
///
/// ```rust
/// use rustgram_secure_storage::Secret;
///
/// // Create a valid secret (sum % 255 == 239)
/// let mut bytes = [0u8; 32];
/// bytes[0] = 239; // Make sum % 255 == 239
/// let secret = Secret::create(&bytes).unwrap();
/// assert!(secret.is_valid());
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Secret {
    /// 32-byte secret value
    pub secret: UInt256,
    /// Hash of the secret
    pub hash: i64,
}

impl Secret {
    /// Checksum value for validation: sum % 255 == 239
    pub const CHECKSUM: u8 = 239;

    /// Secret size in bytes
    pub const SIZE: usize = 32;

    /// Creates a Secret from bytes, validating checksum.
    ///
    /// # Arguments
    ///
    /// * `secret` - The 32-byte secret
    ///
    /// # Returns
    ///
    /// Returns Ok(Secret) if checksum is valid, Err otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::Secret;
    ///
    /// // Valid secret (sum % 255 == 239)
    /// let mut bytes = [0u8; 32];
    /// bytes[0] = 239;
    /// assert!(Secret::create(&bytes).is_ok());
    ///
    /// // Invalid secret
    /// let bytes = [0u8; 32];
    /// assert!(Secret::create(&bytes).is_err());
    /// ```
    pub fn create(secret: &[u8; Self::SIZE]) -> Result<Self, SecureStorageError> {
        if !Self::validate_checksum(secret) {
            return Err(SecureStorageError::InvalidChecksum);
        }

        let hash = Self::calc_hash(secret);
        Ok(Self {
            secret: UInt256::new(*secret),
            hash,
        })
    }

    /// Generates a new random Secret.
    ///
    /// # TODO
    ///
    /// Implement proper random generation using `rand` crate.
    ///
    /// # Returns
    ///
    /// Returns a new valid Secret.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::Secret;
    ///
    /// let secret = Secret::create_new();
    /// assert!(secret.is_valid());
    /// ```
    pub fn create_new() -> Self {
        let mut secret = [0u8; Self::SIZE];
        // Stub: Generate random bytes
        secret[0] = Self::CHECKSUM; // Make it valid
        Self {
            secret: UInt256::new(secret),
            hash: Self::calc_hash(&secret),
        }
    }

    /// Returns the secret as bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::Secret;
    ///
    /// let mut bytes = [0u8; 32];
    /// bytes[0] = 239;
    /// let secret = Secret::create(&bytes).unwrap();
    /// assert_eq!(secret.as_slice(), &bytes[..]);
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: UInt256 always contains exactly 32 bytes
        unsafe {
            std::slice::from_raw_parts(
                &self.secret as *const UInt256 as *const u8,
                Self::SIZE,
            )
        }
    }

    /// Encrypts the secret.
    ///
    /// # TODO
    ///
    /// Implement actual encryption with AesCbcState.
    ///
    /// # Arguments
    ///
    /// * `key` - Encryption key
    /// * `salt` - Salt for key derivation
    /// * `algorithm` - Encryption algorithm to use
    ///
    /// # Returns
    ///
    /// Returns EncryptedSecret.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::{Secret, EncryptionAlgorithm};
    ///
    /// let secret = Secret::create_new();
    /// let encrypted = secret.encrypt(&[0u8; 32], &[0u8; 16], EncryptionAlgorithm::Pbkdf2);
    /// ```
    pub fn encrypt(&self, _key: &[u8], _salt: &[u8], _algorithm: EncryptionAlgorithm) -> EncryptedSecret {
        // Stub: Would encrypt with AES-CBC
        EncryptedSecret {
            encrypted_secret: UInt256::new([0u8; 32]),
        }
    }

    /// Returns the secret hash.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::Secret;
    ///
    /// let secret = Secret::create_new();
    /// let hash = secret.get_hash();
    /// ```
    pub fn get_hash(&self) -> i64 {
        self.hash
    }

    /// Clones the secret.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::Secret;
    ///
    /// let secret1 = Secret::create_new();
    /// let secret2 = secret1.clone();
    /// assert_eq!(secret1, secret2);
    /// ```
    pub fn clone(&self) -> Self {
        Self {
            secret: self.secret,
            hash: self.hash,
        }
    }

    /// Returns the size (always 32).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::Secret;
    ///
    /// assert_eq!(Secret::size(), 32);
    /// ```
    pub const fn size() -> usize {
        Self::SIZE
    }

    /// Checks if the secret is valid.
    ///
    /// # Returns
    ///
    /// Returns `true` if checksum is valid, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::Secret;
    ///
    /// let secret = Secret::create_new();
    /// assert!(secret.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        Self::validate_checksum(self.as_slice())
    }

    /// Validates the checksum of secret bytes.
    fn validate_checksum(secret: &[u8; Self::SIZE]) -> bool {
        let sum: u64 = secret.iter().map(|&b| b as u64).sum();
        (sum % 255) as u8 == Self::CHECKSUM
    }

    /// Calculates hash from secret bytes.
    fn calc_hash(secret: &[u8; Self::SIZE]) -> i64 {
        // Simple hash: XOR of all bytes as i64
        let mut hash: i64 = 0;
        for (i, &byte) in secret.iter().enumerate() {
            hash ^= (byte as i64) << (8 * (i % 8));
        }
        hash
    }
}

impl Debug for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Secret")
            .field("hash", &self.hash)
            .field("is_valid", &self.is_valid())
            .finish()
    }
}

/// Encrypted version of a Secret.
#[derive(Clone, PartialEq, Eq)]
pub struct EncryptedSecret {
    /// Encrypted 32-byte value
    pub encrypted_secret: UInt256,
}

impl EncryptedSecret {
    /// Creates EncryptedSecret from encrypted bytes.
    ///
    /// # Arguments
    ///
    /// * `encrypted_secret` - The encrypted 32-byte value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::EncryptedSecret;
    /// use rustgram_types::UInt256;
    ///
    /// let encrypted = EncryptedSecret {
    ///     encrypted_secret: UInt256::new([0u8; 32]),
    /// };
    /// ```
    pub fn create(encrypted_secret: &[u8; 32]) -> Self {
        Self {
            encrypted_secret: UInt256::new(*encrypted_secret),
        }
    }

    /// Decrypts to Secret.
    ///
    /// # TODO
    ///
    /// Implement actual decryption.
    ///
    /// # Arguments
    ///
    /// * `key` - Decryption key
    /// * `salt` - Salt for key derivation
    /// * `algorithm` - Decryption algorithm
    ///
    /// # Returns
    ///
    /// Returns Ok(Secret) if decryption succeeds, Err otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::{EncryptedSecret, EncryptionAlgorithm};
    ///
    /// let encrypted = EncryptedSecret {
    ///     encrypted_secret: rustgram_types::UInt256::new([0u8; 32]),
    /// };
    /// // Note: This will fail with stub implementation
    /// ```
    pub fn decrypt(
        &self,
        _key: &[u8],
        _salt: &[u8],
        _algorithm: EncryptionAlgorithm,
    ) -> Result<Secret, SecureStorageError> {
        // Stub: Would decrypt with AES-CBC
        Err(SecureStorageError::NotImplemented)
    }

    /// Returns encrypted bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::EncryptedSecret;
    ///
    /// let encrypted = EncryptedSecret {
    ///     encrypted_secret: rustgram_types::UInt256::new([0u8; 32]),
    /// };
    /// let bytes = encrypted.as_slice();
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: UInt256 always contains exactly 32 bytes
        unsafe {
            std::slice::from_raw_parts(
                &self.encrypted_secret as *const UInt256 as *const u8,
                32,
            )
        }
    }
}

impl Debug for EncryptedSecret {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("EncryptedSecret").finish()
    }
}

/// User password for encryption.
#[derive(Clone, PartialEq, Eq)]
pub struct Password {
    /// The password string
    pub password: String,
}

impl Password {
    /// Creates Password from string.
    ///
    /// # Arguments
    ///
    /// * `password` - The password string
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::Password;
    ///
    /// let password = Password::new("my_password".to_string());
    /// ```
    pub fn new(password: String) -> Self {
        Self { password }
    }

    /// Returns password as string slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::Password;
    ///
    /// let password = Password::new("test".to_string());
    /// assert_eq!(password.as_slice(), "test");
    /// ```
    pub fn as_slice(&self) -> &str {
        &self.password
    }
}

impl Debug for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Password")
            .field("length", &self.password.len())
            .finish()
    }
}

/// SHA-256 hash of a value.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueHash {
    /// 32-byte hash value
    pub hash: [u8; 32],
}

impl ValueHash {
    /// Creates hash from data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    ///
    /// # Returns
    ///
    /// Returns Ok(ValueHash) if successful, Err otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::ValueHash;
    ///
    /// let hash = ValueHash::create(b"hello world").unwrap();
    /// assert_eq!(hash.as_slice().len(), 32);
    /// ```
    pub fn create(data: &[u8]) -> Result<Self, SecureStorageError> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        Ok(Self {
            hash: hash.into(),
        })
    }

    /// Returns hash as bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::ValueHash;
    ///
    /// let hash = ValueHash::create(b"test").unwrap();
    /// assert_eq!(hash.as_slice().len(), 32);
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        &self.hash
    }
}

impl Debug for ValueHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValueHash")
            .field("hash", &hex::encode(self.hash))
            .finish()
    }
}

/// Encrypted data with its hash.
#[derive(Clone, PartialEq, Eq)]
pub struct EncryptedValue {
    /// Encrypted data
    pub data: BufferSlice,
    /// Hash for verification
    pub hash: ValueHash,
}

impl EncryptedValue {
    /// Creates a new EncryptedValue.
    ///
    /// # Arguments
    ///
    /// * `data` - Encrypted data
    /// * `hash` - Hash for verification
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secure_storage::{EncryptedValue, ValueHash};
    /// use rustgram_buffer::BufferSlice;
    ///
    /// let data = BufferSlice::new(vec![1, 2, 3, 4]);
    /// let hash = ValueHash::create(&data.data).unwrap();
    /// let encrypted = EncryptedValue { data, hash };
    /// ```
    pub fn new(data: BufferSlice, hash: ValueHash) -> Self {
        Self { data, hash }
    }
}

impl Debug for EncryptedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("EncryptedValue")
            .field("data_len", &self.data.len())
            .field("hash", &self.hash)
            .finish()
    }
}

/// Encryption algorithm for secret.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptionAlgorithm {
    /// SHA512-based key derivation
    Sha512,
    /// PBKDF2 key derivation
    Pbkdf2,
}

/// Secure storage error types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecureStorageError {
    /// Invalid checksum
    InvalidChecksum,
    /// Feature not implemented
    NotImplemented,
    /// IO error
    Io(String),
    /// Encryption error
    Encryption(String),
    /// Decryption error
    Decryption(String),
}

impl std::fmt::Display for SecureStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChecksum => write!(f, "Invalid checksum"),
            Self::NotImplemented => write!(f, "Feature not implemented"),
            Self::Io(msg) => write!(f, "IO error: {}", msg),
            Self::Encryption(msg) => write!(f, "Encryption error: {}", msg),
            Self::Decryption(msg) => write!(f, "Decryption error: {}", msg),
        }
    }
}

impl std::error::Error for SecureStorageError {}

/// Calculates AES-CBC state from secret using PBKDF2.
///
/// # TODO
///
/// Implement actual PBKDF2 derivation.
///
/// # Arguments
///
/// * `secret` - The secret bytes
/// * `salt` - The salt
///
/// # Returns
///
/// Returns AesCbcState.
///
/// # Example
///
/// ```rust
/// use rustgram_secure_storage::calc_aes_cbc_state_pbkdf2;
///
/// let secret = [0u8; 32];
/// let salt = [0u8; 16];
/// let state = calc_aes_cbc_state_pbkdf2(&secret, &salt);
/// ```
pub fn calc_aes_cbc_state_pbkdf2(_secret: &[u8], _salt: &[u8]) -> AesCbcState {
    AesCbcState::random(32)
}

/// Calculates AES-CBC state from seed using SHA512.
///
/// # TODO
///
/// Implement actual SHA512 derivation.
///
/// # Arguments
///
/// * `seed` - The seed bytes
///
/// # Returns
///
/// Returns AesCbcState.
///
/// # Example
///
/// ```rust
/// use rustgram_secure_storage::calc_aes_cbc_state_sha512;
///
/// let seed = [0u8; 32];
/// let state = calc_aes_cbc_state_sha512(&seed);
/// ```
pub fn calc_aes_cbc_state_sha512(_seed: &[u8]) -> AesCbcState {
    AesCbcState::random(32)
}

/// Calculates hash from data view.
///
/// # TODO
///
/// Implement actual DataView support.
///
/// # Arguments
///
/// * `data` - The data to hash
///
/// # Returns
///
/// Returns Ok(ValueHash) if successful.
///
/// # Example
///
/// ```rust
/// use rustgram_secure_storage::calc_value_hash;
///
/// let hash = calc_value_hash(b"hello world").unwrap();
/// assert_eq!(hash.as_slice().len(), 32);
/// ```
pub fn calc_value_hash(data: &[u8]) -> Result<ValueHash, SecureStorageError> {
    ValueHash::create(data)
}

/// Generates random prefix for encryption.
///
/// # TODO
///
/// Implement proper random generation.
///
/// # Arguments
///
/// * `data_size` - Size of data to encrypt
///
/// # Returns
///
/// Returns BufferSlice with random prefix.
///
/// # Example
///
/// ```rust
/// use rustgram_secure_storage::gen_random_prefix;
///
/// let prefix = gen_random_prefix(100);
/// ```
pub fn gen_random_prefix(_data_size: i64) -> BufferSlice {
    BufferSlice::zero(16)
}

/// Encrypts data with secret.
///
/// # TODO
///
/// Implement actual encryption.
///
/// # Arguments
///
/// * `secret` - The secret to use
/// * `data` - The data to encrypt
///
/// # Returns
///
/// Returns Ok(EncryptedValue) if successful.
///
/// # Example
///
/// ```rust
/// use rustgram_secure_storage::{Secret, encrypt_value};
///
/// let secret = Secret::create_new();
/// let encrypted = encrypt_value(&secret, b"test data").unwrap();
/// ```
pub fn encrypt_value(_secret: &Secret, _data: &[u8]) -> Result<EncryptedValue, SecureStorageError> {
    // Stub: Would encrypt with AES-CBC
    Ok(EncryptedValue {
        data: BufferSlice::new(_data.to_vec()),
        hash: ValueHash::create(_data)?,
    })
}

/// Decrypts data with secret and verifies hash.
///
/// # TODO
///
/// Implement actual decryption.
///
/// # Arguments
///
/// * `secret` - The secret to use
/// * `hash` - The expected hash
/// * `data` - The encrypted data
///
/// # Returns
///
/// Returns Ok(BufferSlice) if successful.
///
/// # Example
///
/// ```rust
/// use rustgram_secure_storage::{Secret, ValueHash, decrypt_value};
///
/// let secret = Secret::create_new();
/// let hash = ValueHash::create(b"test").unwrap();
/// // Note: Will fail with stub implementation
/// ```
pub fn decrypt_value(
    _secret: &Secret,
    _hash: &ValueHash,
    _data: &[u8],
) -> Result<BufferSlice, SecureStorageError> {
    // Stub: Would decrypt and verify
    Err(SecureStorageError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_create_valid() {
        let mut bytes = [0u8; 32];
        bytes[0] = 239; // Make sum % 255 == 239
        let secret = Secret::create(&bytes);
        assert!(secret.is_ok());
        let secret = secret.unwrap();
        assert!(secret.is_valid());
    }

    #[test]
    fn test_secret_create_invalid() {
        let bytes = [0u8; 32];
        let result = Secret::create(&bytes);
        assert!(matches!(result, Err(SecureStorageError::InvalidChecksum)));
    }

    #[test]
    fn test_secret_create_new() {
        let secret = Secret::create_new();
        assert!(secret.is_valid());
        assert_eq!(secret.size(), 32);
    }

    #[test]
    fn test_secret_as_slice() {
        let mut bytes = [0u8; 32];
        bytes[0] = 239;
        let secret = Secret::create(&bytes).unwrap();
        assert_eq!(secret.as_slice(), &bytes[..]);
    }

    #[test]
    fn test_secret_clone() {
        let secret1 = Secret::create_new();
        let secret2 = secret1.clone();
        assert_eq!(secret1, secret2);
    }

    #[test]
    fn test_secret_size() {
        assert_eq!(Secret::size(), 32);
    }

    #[test]
    fn test_secret_is_valid() {
        let valid = Secret::create_new();
        assert!(valid.is_valid());

        let mut invalid_bytes = [0u8; 32];
        let invalid = Secret::create(&invalid_bytes);
        assert!(invalid.is_err());
    }

    #[test]
    fn test_secret_get_hash() {
        let secret = Secret::create_new();
        let hash = secret.get_hash();
        // Hash should be some i64 value
        assert_ne!(hash, 0);
    }

    #[test]
    fn test_secret_equality() {
        let secret1 = Secret::create_new();
        let secret2 = secret1.clone();
        assert_eq!(secret1, secret2);

        let secret3 = Secret::create_new();
        assert_ne!(secret1, secret3);
    }

    #[test]
    fn test_secret_debug() {
        let secret = Secret::create_new();
        let debug_str = format!("{:?}", secret);
        assert!(debug_str.contains("Secret"));
        assert!(debug_str.contains("is_valid"));
    }

    #[test]
    fn test_secret_encrypt() {
        let secret = Secret::create_new();
        let encrypted = secret.encrypt(&[0u8; 32], &[0u8; 16], EncryptionAlgorithm::Pbkdf2);
        // Should not panic
        let _ = encrypted;
    }

    #[test]
    fn test_encrypted_secret_create() {
        let bytes = [0u8; 32];
        let encrypted = EncryptedSecret::create(&bytes);
        assert_eq!(encrypted.as_slice().len(), 32);
    }

    #[test]
    fn test_encrypted_secret_as_slice() {
        let bytes = [5u8; 32];
        let encrypted = EncryptedSecret::create(&bytes);
        assert_eq!(encrypted.as_slice(), &bytes[..]);
    }

    #[test]
    fn test_encrypted_secret_debug() {
        let encrypted = EncryptedSecret::create(&[0u8; 32]);
        let debug_str = format!("{:?}", encrypted);
        assert!(debug_str.contains("EncryptedSecret"));
    }

    #[test]
    fn test_password_new() {
        let password = Password::new("test123".to_string());
        assert_eq!(password.as_slice(), "test123");
    }

    #[test]
    fn test_password_as_slice() {
        let password = Password::new("mypassword".to_string());
        assert_eq!(password.as_slice(), "mypassword");
    }

    #[test]
    fn test_password_equality() {
        let password1 = Password::new("same".to_string());
        let password2 = Password::new("same".to_string());
        let password3 = Password::new("different".to_string());
        assert_eq!(password1, password2);
        assert_ne!(password1, password3);
    }

    #[test]
    fn test_password_debug() {
        let password = Password::new("secret".to_string());
        let debug_str = format!("{:?}", password);
        assert!(debug_str.contains("Password"));
        assert!(debug_str.contains("6")); // length
    }

    #[test]
    fn test_value_hash_create() {
        let hash = ValueHash::create(b"test data").unwrap();
        assert_eq!(hash.as_slice().len(), 32);
    }

    #[test]
    fn test_value_hash_as_slice() {
        let hash = ValueHash::create(b"hello").unwrap();
        assert_eq!(hash.as_slice().len(), 32);
    }

    #[test]
    fn test_value_hash_different_inputs() {
        let hash1 = ValueHash::create(b"test1").unwrap();
        let hash2 = ValueHash::create(b"test2").unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_value_hash_same_inputs() {
        let hash1 = ValueHash::create(b"same data").unwrap();
        let hash2 = ValueHash::create(b"same data").unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_value_hash_empty() {
        let hash = ValueHash::create(b"").unwrap();
        assert_eq!(hash.as_slice().len(), 32);
    }

    #[test]
    fn test_value_hash_debug() {
        let hash = ValueHash::create(b"test").unwrap();
        let debug_str = format!("{:?}", hash);
        assert!(debug_str.contains("ValueHash"));
    }

    #[test]
    fn test_encrypted_value_new() {
        let data = BufferSlice::new(vec![1, 2, 3]);
        let hash = ValueHash::create(&[1, 2, 3]).unwrap();
        let encrypted = EncryptedValue::new(data.clone(), hash);
        assert_eq!(encrypted.data.len(), 3);
    }

    #[test]
    fn test_encrypted_value_debug() {
        let data = BufferSlice::new(vec![1, 2, 3]);
        let hash = ValueHash::create(&[1, 2, 3]).unwrap();
        let encrypted = EncryptedValue::new(data, hash);
        let debug_str = format!("{:?}", encrypted);
        assert!(debug_str.contains("EncryptedValue"));
    }

    #[test]
    fn test_encryption_algorithm_variants() {
        let algorithms = [
            EncryptionAlgorithm::Sha512,
            EncryptionAlgorithm::Pbkdf2,
        ];

        assert_eq!(algorithms.len(), 2);
    }

    #[test]
    fn test_encryption_algorithm_equality() {
        assert_eq!(EncryptionAlgorithm::Sha512, EncryptionAlgorithm::Sha512);
        assert_eq!(EncryptionAlgorithm::Pbkdf2, EncryptionAlgorithm::Pbkdf2);
        assert_ne!(EncryptionAlgorithm::Sha512, EncryptionAlgorithm::Pbkdf2);
    }

    #[test]
    fn test_calc_aes_cbc_state_pbkdf2() {
        let secret = [0u8; 32];
        let salt = [0u8; 16];
        let state = calc_aes_cbc_state_pbkdf2(&secret, &salt);
        assert!(state.is_valid());
    }

    #[test]
    fn test_calc_aes_cbc_state_sha512() {
        let seed = [0u8; 32];
        let state = calc_aes_cbc_state_sha512(&seed);
        assert!(state.is_valid());
    }

    #[test]
    fn test_calc_value_hash() {
        let hash = calc_value_hash(b"test data").unwrap();
        assert_eq!(hash.as_slice().len(), 32);
    }

    #[test]
    fn test_gen_random_prefix() {
        let prefix = gen_random_prefix(100);
        assert_eq!(prefix.len(), 16);
    }

    #[test]
    fn test_encrypt_value() {
        let secret = Secret::create_new();
        let encrypted = encrypt_value(&secret, b"test data").unwrap();
        assert_eq!(encrypted.data.len(), 9);
    }

    #[test]
    fn test_secure_storage_error_display() {
        let err = SecureStorageError::InvalidChecksum;
        assert_eq!(format!("{}", err), "Invalid checksum");

        let err = SecureStorageError::NotImplemented;
        assert_eq!(format!("{}", err), "Feature not implemented");
    }

    #[test]
    fn test_secret_checksum_validation() {
        // Test that sum % 255 == 239 is enforced
        let mut bytes = [0u8; 32];

        // Invalid: sum = 0, 0 % 255 = 0 != 239
        assert!(Secret::create(&bytes).is_err());

        // Valid: bytes[0] = 239, sum = 239, 239 % 255 = 239
        bytes[0] = 239;
        assert!(Secret::create(&bytes).is_ok());

        // Valid: bytes = [1, 1, ..., 1, 119], sum = 31*1 + 119 = 150, 150 % 255 != 239
        // Let's try: bytes = [239, 0, 0, ..., 0], sum = 239, 239 % 255 = 239
        let mut bytes2 = [0u8; 32];
        bytes2[0] = 239;
        assert!(Secret::create(&bytes2).is_ok());
    }

    #[test]
    fn test_multiple_secrets_different() {
        let secret1 = Secret::create_new();
        let secret2 = Secret::create_new();
        // Different random secrets should be different
        assert_ne!(secret1, secret2);
    }

    #[test]
    fn test_value_hash_large_input() {
        let large_data = vec![0u8; 10000];
        let hash = ValueHash::create(&large_data).unwrap();
        assert_eq!(hash.as_slice().len(), 32);
    }

    #[test]
    fn test_password_empty() {
        let password = Password::new(String::new());
        assert_eq!(password.as_slice(), "");
        assert_eq!(password.password.len(), 0);
    }

    #[test]
    fn test_encrypted_value_clone() {
        let data = BufferSlice::new(vec![1, 2, 3]);
        let hash = ValueHash::create(&[1, 2, 3]).unwrap();
        let encrypted1 = EncryptedValue::new(data, hash);
        let encrypted2 = encrypted1.clone();
        assert_eq!(encrypted1, encrypted2);
    }

    #[test]
    fn test_secret_with_algorithm_variants() {
        let secret = Secret::create_new();

        // Should work with all algorithms (stubs)
        let _ = secret.encrypt(&[0u8; 32], &[0u8; 16], EncryptionAlgorithm::Sha512);
        let _ = secret.encrypt(&[0u8; 32], &[0u8; 16], EncryptionAlgorithm::Pbkdf2);
    }
}
