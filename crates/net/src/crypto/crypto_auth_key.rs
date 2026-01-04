// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Cryptographic authentication key handling for MTProto.
//!
//! Based on TDLib's `td/mtproto/AuthKey.h`.
//!
//! This module provides types and functions for handling MTProto cryptographic
//! authentication keys, including key generation, validation, and key ID computation.
//!
//! # Note
//!
//! `CryptoAuthKey` is distinct from `auth::AuthKey`. This type is used at the
//! cryptographic layer for encryption/decryption, while `auth::AuthKey` is used
//! at the application layer for session management.

use std::fmt;
use std::time::Instant;

use rand::Rng;

use crate::crypto::sha1;

/// MTProto cryptographic authentication key.
///
/// Crypto auth keys are 2048-bit (256 byte) keys used for encrypting and decrypting
/// MTProto packets. They can be permanent or temporary (with expiration).
///
/// # References
///
/// - TDLib: `td/mtproto/AuthKey.h`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoAuthKey {
    /// Key data (256 bytes)
    key: [u8; 256],

    /// Auth key ID (computed from SHA1 of key)
    id: u64,

    /// Expiration time (None for permanent keys)
    expires_at: Option<Instant>,

    /// Whether this key was created using PFS (Perfect Forward Secrecy)
    is_pfs: bool,

    /// Whether this is a CDN key (content delivery network)
    is_cdn: bool,
}

impl Default for CryptoAuthKey {
    fn default() -> Self {
        Self {
            key: [0u8; 256],
            id: 0,
            expires_at: None,
            is_pfs: false,
            is_cdn: false,
        }
    }
}

impl CryptoAuthKey {
    /// Size of a crypto auth key in bytes.
    pub const SIZE: usize = 256;

    /// Creates a new crypto auth key from raw bytes.
    ///
    /// # Arguments
    ///
    /// * `key` - 256-byte key data
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_net::crypto::CryptoAuthKey;
    ///
    /// let key_bytes = [42u8; 256];
    /// let auth_key = CryptoAuthKey::new(key_bytes);
    /// assert!(!auth_key.is_empty());
    /// ```
    #[must_use]
    pub fn new(key: [u8; 256]) -> Self {
        let id = Self::compute_id(&key);
        Self {
            key,
            id,
            expires_at: None,
            is_pfs: false,
            is_cdn: false,
        }
    }

    /// Creates a new crypto auth key from a slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the slice is not exactly 256 bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_net::crypto::CryptoAuthKey;
    ///
    /// let key_bytes = vec![42u8; 256];
    /// let auth_key = CryptoAuthKey::from_slice(&key_bytes).unwrap();
    /// assert!(!auth_key.is_empty());
    /// ```
    pub fn from_slice(key: &[u8]) -> Result<Self, CryptoAuthKeyError> {
        if key.len() != Self::SIZE {
            return Err(CryptoAuthKeyError::InvalidLength {
                actual: key.len(),
                expected: Self::SIZE,
            });
        }

        let mut key_array = [0u8; Self::SIZE];
        key_array.copy_from_slice(key);

        Ok(Self::new(key_array))
    }

    /// Creates a temporary crypto auth key with expiration.
    ///
    /// # Arguments
    ///
    /// * `key` - 256-byte key data
    /// * `expires_at` - When the key expires
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustgram_net::crypto::CryptoAuthKey;
    /// use std::time::Instant;
    ///
    /// let key_bytes = [42u8; 256];
    /// let expires = Instant::now() + std::time::Duration::from_secs(3600);
    /// let auth_key = CryptoAuthKey::temporary(key_bytes, expires);
    /// assert!(auth_key.is_temporary());
    /// ```
    #[must_use]
    pub fn temporary(key: [u8; 256], expires_at: Instant) -> Self {
        let id = Self::compute_id(&key);
        Self {
            key,
            id,
            expires_at: Some(expires_at),
            is_pfs: true,
            is_cdn: false,
        }
    }

    /// Creates a CDN crypto auth key.
    ///
    /// CDN keys are used for content delivery network access.
    #[must_use]
    pub fn cdn(key: [u8; 256]) -> Self {
        Self {
            key,
            id: Self::compute_id(&key),
            expires_at: None,
            is_pfs: false,
            is_cdn: true,
        }
    }

    /// Returns the auth key ID.
    ///
    /// The ID is computed as the lower 64 bits of SHA1(key).
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Returns the auth key data.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 256] {
        &self.key
    }

    /// Returns a mutable reference to the key data.
    #[must_use]
    pub fn as_bytes_mut(&mut self) -> &mut [u8; 256] {
        &mut self.key
    }

    /// Returns the length of the key (always 256).
    #[must_use]
    pub const fn len(&self) -> usize {
        Self::SIZE
    }

    /// Returns `true` if the key data is all zeros (empty key).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.key.iter().all(|&b| b == 0)
    }

    /// Returns `true` if this is a temporary key (has expiration).
    #[must_use]
    pub const fn is_temporary(&self) -> bool {
        self.expires_at.is_some()
    }

    /// Returns `true` if this is a permanent key.
    #[must_use]
    pub const fn is_permanent(&self) -> bool {
        self.expires_at.is_none()
    }

    /// Returns `true` if the key is expired.
    ///
    /// Always returns `false` for permanent keys.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map(|exp| exp <= Instant::now())
            .unwrap_or(false)
    }

    /// Returns the expiration time, if any.
    #[must_use]
    pub const fn expires_at(&self) -> Option<Instant> {
        self.expires_at
    }

    /// Returns `true` if this key was created using PFS.
    #[must_use]
    pub const fn is_pfs(&self) -> bool {
        self.is_pfs
    }

    /// Returns `true` if this is a CDN key.
    #[must_use]
    pub const fn is_cdn(&self) -> bool {
        self.is_cdn
    }

    /// Computes the auth key ID from raw key bytes.
    ///
    /// The ID is the lower 64 bits of SHA1(key).
    #[must_use]
    fn compute_id(key: &[u8; 256]) -> u64 {
        let hash = sha1(key);
        // SHA1 hash is always 20 bytes, so [12..20] slice is always 8 bytes
        u64::from_le_bytes(
            hash[12..20]
                .try_into()
                .expect("SHA1 hash is always 20 bytes"),
        )
    }

    /// Sets the PFS flag.
    pub fn set_pfs(&mut self, is_pfs: bool) {
        self.is_pfs = is_pfs;
    }

    /// Sets the CDN flag.
    pub fn set_cdn(&mut self, is_cdn: bool) {
        self.is_cdn = is_cdn;
    }

    /// Sets the expiration time.
    pub fn set_expires_at(&mut self, expires_at: Option<Instant>) {
        self.expires_at = expires_at;
    }

    /// Clears the key data (sets all bytes to zero).
    pub fn clear(&mut self) {
        self.key = [0u8; Self::SIZE];
        self.id = 0;
        self.expires_at = None;
    }
}

impl AsRef<[u8; 256]> for CryptoAuthKey {
    fn as_ref(&self) -> &[u8; 256] {
        &self.key
    }
}

impl AsMut<[u8; 256]> for CryptoAuthKey {
    fn as_mut(&mut self) -> &mut [u8; 256] {
        &mut self.key
    }
}

impl fmt::Display for CryptoAuthKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CryptoAuthKey {{ id: {:016x}, temporary: {}, pfs: {}, cdn: {}, expired: {} }}",
            self.id,
            self.is_temporary(),
            self.is_pfs,
            self.is_cdn,
            self.is_expired()
        )
    }
}

/// Crypto auth key error type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoAuthKeyError {
    /// Invalid key length
    InvalidLength {
        /// Actual length
        actual: usize,
        /// Expected length
        expected: usize,
    },

    /// Key is empty
    EmptyKey,

    /// Key is expired
    ExpiredKey,
}

impl std::fmt::Display for CryptoAuthKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength { actual, expected } => write!(
                f,
                "Invalid crypto auth key length: got {} bytes, expected {} bytes",
                actual, expected
            ),
            Self::EmptyKey => write!(f, "Crypto auth key is empty"),
            Self::ExpiredKey => write!(f, "Crypto auth key has expired"),
        }
    }
}

impl std::error::Error for CryptoAuthKeyError {}

/// Type alias for compatibility.
pub type AuthKeyError = CryptoAuthKeyError;

/// Helper trait for working with crypto auth keys.
pub trait CryptoAuthKeyHelper: Send + Sync {
    /// Generates a new random crypto auth key.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_net::crypto::CryptoAuthKeyHelper;
    /// use rustgram_net::crypto::DefaultAuthKeyHelper;
    ///
    /// let key = DefaultAuthKeyHelper::generate();
    /// assert!(!key.is_empty());
    /// ```
    fn generate() -> CryptoAuthKey;

    /// Validates a crypto auth key.
    ///
    /// Returns an error if the key is invalid (empty or expired).
    fn validate(key: &CryptoAuthKey) -> Result<(), CryptoAuthKeyError>;
}

/// Type alias for compatibility.
pub trait AuthKeyHelper: Send + Sync {
    /// Generates a new random crypto auth key.
    fn generate() -> CryptoAuthKey;
    /// Validates a crypto auth key, returning an error if the key is invalid.
    fn validate(key: &CryptoAuthKey) -> Result<(), CryptoAuthKeyError>;
}

/// Default implementation of `CryptoAuthKeyHelper`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultAuthKeyHelper;

impl CryptoAuthKeyHelper for DefaultAuthKeyHelper {
    fn generate() -> CryptoAuthKey {
        let mut key = [0u8; CryptoAuthKey::SIZE];
        rand::thread_rng().fill(&mut key[..]);
        CryptoAuthKey::new(key)
    }

    fn validate(key: &CryptoAuthKey) -> Result<(), CryptoAuthKeyError> {
        if key.is_empty() {
            return Err(CryptoAuthKeyError::EmptyKey);
        }

        if key.is_expired() {
            return Err(CryptoAuthKeyError::ExpiredKey);
        }

        Ok(())
    }
}

impl AuthKeyHelper for DefaultAuthKeyHelper {
    fn generate() -> CryptoAuthKey {
        let mut key = [0u8; CryptoAuthKey::SIZE];
        rand::thread_rng().fill(&mut key[..]);
        CryptoAuthKey::new(key)
    }

    fn validate(key: &CryptoAuthKey) -> Result<(), CryptoAuthKeyError> {
        if key.is_empty() {
            return Err(CryptoAuthKeyError::EmptyKey);
        }

        if key.is_expired() {
            return Err(CryptoAuthKeyError::ExpiredKey);
        }

        Ok(())
    }
}

/// Computes the auth key ID from raw key bytes.
///
/// This is a convenience function that computes the key ID
/// (lower 64 bits of SHA1 hash).
///
/// # Examples
///
/// ```
/// use rustgram_net::crypto::{compute_auth_key_id, ComputeAuthKeyId};
///
/// let key = [42u8; 256];
/// let id = compute_auth_key_id(&key);
/// assert_ne!(id, 0);
///
/// // Or use the trait method
/// let id2 = key.compute_auth_key_id();
/// assert_eq!(id, id2);
/// ```
#[must_use]
pub fn compute_auth_key_id(key: &[u8; 256]) -> u64 {
    CryptoAuthKey::compute_id(key)
}

/// Trait for computing auth key ID.
pub trait ComputeAuthKeyId {
    /// Computes the auth key ID from this key.
    ///
    /// The ID is the lower 64 bits of SHA1 hash of the key.
    fn compute_auth_key_id(&self) -> u64;
}

impl ComputeAuthKeyId for [u8; 256] {
    fn compute_auth_key_id(&self) -> u64 {
        compute_auth_key_id(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_crypto_auth_key_default() {
        let key = CryptoAuthKey::default();
        assert!(key.is_empty());
        assert_eq!(key.id(), 0);
        assert!(key.is_permanent());
        assert!(!key.is_expired());
    }

    #[test]
    fn test_crypto_auth_key_new() {
        let key_bytes = [42u8; 256];
        let key = CryptoAuthKey::new(key_bytes);
        assert!(!key.is_empty());
        assert_ne!(key.id(), 0);
        assert_eq!(key.as_bytes(), &key_bytes);
    }

    #[test]
    fn test_crypto_auth_key_from_slice() {
        let key_bytes = vec![42u8; 256];
        let key = match CryptoAuthKey::from_slice(&key_bytes) {
            Ok(k) => k,
            Err(_) => panic!("Expected Ok key"),
        };
        assert!(!key.is_empty());

        // Wrong size
        let short_key = vec![1u8; 100];
        assert!(CryptoAuthKey::from_slice(&short_key).is_err());
    }

    #[test]
    fn test_crypto_auth_key_temporary() {
        let key_bytes = [42u8; 256];
        let expires = Instant::now() + Duration::from_secs(60);
        let key = CryptoAuthKey::temporary(key_bytes, expires);

        assert!(key.is_temporary());
        assert!(key.is_pfs());
        assert!(!key.is_expired());
        assert_eq!(key.expires_at(), Some(expires));
    }

    #[test]
    fn test_crypto_auth_key_temporary_expired() {
        let key_bytes = [42u8; 256];
        let expires = Instant::now() - Duration::from_secs(1); // Already expired
        let key = CryptoAuthKey::temporary(key_bytes, expires);

        assert!(key.is_expired());
    }

    #[test]
    fn test_crypto_auth_key_cdn() {
        let key_bytes = [42u8; 256];
        let key = CryptoAuthKey::cdn(key_bytes);

        assert!(key.is_cdn());
        assert!(key.is_permanent());
    }

    #[test]
    fn test_crypto_auth_key_len() {
        let key = CryptoAuthKey::new([1u8; 256]);
        assert_eq!(key.len(), 256);
    }

    #[test]
    fn test_crypto_auth_key_as_bytes_mut() {
        let mut key = CryptoAuthKey::new([42u8; 256]);
        key.as_bytes_mut()[0] = 99;
        assert_eq!(key.as_bytes()[0], 99);
    }

    #[test]
    fn test_crypto_auth_key_setters() {
        let mut key = CryptoAuthKey::new([42u8; 256]);

        key.set_pfs(true);
        assert!(key.is_pfs());

        key.set_cdn(true);
        assert!(key.is_cdn());

        let expires = Instant::now() + Duration::from_secs(100);
        key.set_expires_at(Some(expires));
        assert_eq!(key.expires_at(), Some(expires));
    }

    #[test]
    fn test_crypto_auth_key_clear() {
        let mut key = CryptoAuthKey::new([42u8; 256]);
        assert!(!key.is_empty());

        key.clear();
        assert!(key.is_empty());
        assert_eq!(key.id(), 0);
    }

    #[test]
    fn test_crypto_auth_key_id_deterministic() {
        let key_bytes = [123u8; 256];
        let key1 = CryptoAuthKey::new(key_bytes);
        let key2 = CryptoAuthKey::new(key_bytes);

        assert_eq!(key1.id(), key2.id());
    }

    #[test]
    fn test_crypto_auth_key_id_unique() {
        let key1 = CryptoAuthKey::new([1u8; 256]);
        let key2 = CryptoAuthKey::new([2u8; 256]);

        assert_ne!(key1.id(), key2.id());
    }

    #[test]
    fn test_crypto_auth_key_display() {
        let key = CryptoAuthKey::new([42u8; 256]);
        let s = format!("{key}");
        assert!(s.contains("CryptoAuthKey"));
        assert!(s.contains("temporary: false"));
    }

    #[test]
    fn test_crypto_auth_key_error_display() {
        let err = CryptoAuthKeyError::InvalidLength {
            actual: 100,
            expected: 256,
        };
        let s = format!("{err}");
        assert!(s.contains("100"));
        assert!(s.contains("256"));

        let err = CryptoAuthKeyError::EmptyKey;
        let s = format!("{err}");
        assert!(s.contains("empty"));

        let err = CryptoAuthKeyError::ExpiredKey;
        let s = format!("{err}");
        assert!(s.contains("expired"));
    }

    #[test]
    fn test_default_auth_key_helper_generate() {
        let key = <DefaultAuthKeyHelper as CryptoAuthKeyHelper>::generate();
        assert!(!key.is_empty());
        assert!(!key.is_temporary());
    }

    #[test]
    fn test_default_auth_key_helper_validate() {
        let key = <DefaultAuthKeyHelper as CryptoAuthKeyHelper>::generate();
        assert!(<DefaultAuthKeyHelper as CryptoAuthKeyHelper>::validate(&key).is_ok());

        let empty_key = CryptoAuthKey::default();
        assert!(matches!(
            <DefaultAuthKeyHelper as CryptoAuthKeyHelper>::validate(&empty_key),
            Err(CryptoAuthKeyError::EmptyKey)
        ));

        let expired_key =
            CryptoAuthKey::temporary([1u8; 256], Instant::now() - Duration::from_secs(1));
        assert!(matches!(
            <DefaultAuthKeyHelper as CryptoAuthKeyHelper>::validate(&expired_key),
            Err(CryptoAuthKeyError::ExpiredKey)
        ));
    }

    #[test]
    fn test_compute_auth_key_id() {
        let key = [42u8; 256];
        let id = compute_auth_key_id(&key);
        assert_ne!(id, 0);

        let id2 = compute_auth_key_id(&key);
        assert_eq!(id, id2);
    }

    #[test]
    fn test_crypto_auth_key_as_ref() {
        let key = CryptoAuthKey::new([123u8; 256]);
        let bytes: &[u8; 256] = key.as_ref();
        assert_eq!(bytes[0], 123);
    }

    #[test]
    fn test_crypto_auth_key_as_mut() {
        let mut key = CryptoAuthKey::new([0u8; 256]);
        let bytes: &mut [u8; 256] = key.as_mut();
        bytes[0] = 255;
        assert_eq!(key.as_bytes()[0], 255);
    }

    #[test]
    fn test_auth_key_error_alias() {
        // Verify the type alias works
        let err: AuthKeyError = CryptoAuthKeyError::EmptyKey;
        assert!(matches!(err, CryptoAuthKeyError::EmptyKey));
    }

    #[test]
    fn test_auth_key_helper_compatibility() {
        // Verify the trait alias works
        let key = <DefaultAuthKeyHelper as CryptoAuthKeyHelper>::generate();
        assert!(<DefaultAuthKeyHelper as AuthKeyHelper>::validate(&key).is_ok());
    }
}
