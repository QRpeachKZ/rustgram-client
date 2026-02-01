// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! HMAC (Hash-based Message Authentication Code) functions for MTProto.
//!
//! This module provides HMAC-SHA256 and HMAC-SHA512 implementations
//! as specified in MTProto 2.0.
//!
//! # References
//!
//! - RFC 2104: HMAC: Keyed-Hashing for Message Authentication
//! - TDLib: `td/utils/crypto.h`

use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::{Sha256, Sha512};
use thiserror::Error;

/// Error types for HMAC operations.
#[derive(Debug, Error)]
pub enum HmacError {
    /// Invalid key length
    #[error("Invalid key length: {0}")]
    InvalidKeyLength(String),

    /// HMAC operation failed
    #[error("HMAC operation failed: {0}")]
    OperationFailed(String),
}

/// Type alias for HMAC-SHA256.
pub type HmacSha256 = Hmac<Sha256>;

/// Type alias for HMAC-SHA512.
pub type HmacSha512 = Hmac<Sha512>;

/// Computes HMAC-SHA256 of the given data with the provided key.
///
/// # Arguments
///
/// * `key` - The secret key (any length)
/// * `data` - The data to authenticate
///
/// # Returns
///
/// A 32-byte HMAC-SHA256 result
///
/// # Example
///
/// ```ignore
/// use rustgram_net::crypto::hmac_sha256;
///
/// let key = b"secret_key";
/// let data = b"message to authenticate";
/// let result = hmac_sha256(key, data);
/// assert_eq!(result.len(), 32);
/// ```
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    use hmac::Mac;
    let mut mac = HmacSha256::new_from_slice(key)
        .expect("HMAC can accept keys of any size");
    mac.update(data);
    let result = mac.finalize().into_bytes();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    output
}

/// Computes HMAC-SHA512 of the given data with the provided key.
///
/// # Arguments
///
/// * `key` - The secret key (any length)
/// * `data` - The data to authenticate
///
/// # Returns
///
/// A 64-byte HMAC-SHA512 result
///
/// # Example
///
/// ```ignore
/// use rustgram_net::crypto::hmac_sha512;
///
/// let key = b"secret_key";
/// let data = b"message to authenticate";
/// let result = hmac_sha512(key, data);
/// assert_eq!(result.len(), 64);
/// ```
pub fn hmac_sha512(key: &[u8], data: &[u8]) -> [u8; 64] {
    use hmac::Mac;
    let mut mac = HmacSha512::new_from_slice(key)
        .expect("HMAC can accept keys of any size");
    mac.update(data);
    let result = mac.finalize().into_bytes();
    let mut output = [0u8; 64];
    output.copy_from_slice(&result);
    output
}

/// Computes PBKDF2-HMAC-SHA256 derived key.
///
/// This is used for key derivation in MTProto 2.0.
///
/// # Arguments
///
/// * `password` - The password bytes
/// * `salt` - The salt bytes
/// * `iterations` - Number of iterations (recommended: 100000+)
/// * `output` - Output buffer for the derived key
///
/// # Example
///
/// ```ignore
/// use rustgram_net::crypto::pbkdf2_hmac_sha256;
///
/// let password = b"my_password";
/// let salt = b"random_salt";
/// let mut key = [0u8; 32];
/// pbkdf2_hmac_sha256(password, salt, 100_000, &mut key);
/// ```
pub fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], iterations: u32, output: &mut [u8]) {
    let _ = pbkdf2::<HmacSha256>(password, salt, iterations, output);
}

/// Computes PBKDF2-HMAC-SHA512 derived key.
///
/// # Arguments
///
/// * `password` - The password bytes
/// * `salt` - The salt bytes
/// * `iterations` - Number of iterations
/// * `output` - Output buffer for the derived key
pub fn pbkdf2_hmac_sha512(password: &[u8], salt: &[u8], iterations: u32, output: &mut [u8]) {
    let _ = pbkdf2::<HmacSha512>(password, salt, iterations, output);
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFC 4231 test vectors for HMAC-SHA256
    #[test]
    fn test_hmac_sha256_rfc_vectors() {
        // Test case 1
        let key = [0x0b_u8; 20];
        let data = b"Hi There";
        let expected = [
            0xb0, 0x34, 0x4c, 0x61, 0xd8, 0xdb, 0x38, 0x53,
            0x5c, 0xa8, 0xaf, 0xce, 0xaf, 0x0b, 0xf1, 0x2b,
            0x88, 0x1d, 0xc2, 0x00, 0xc9, 0x83, 0x3d, 0xa7,
            0x26, 0xe9, 0x37, 0x6c, 0x2e, 0x32, 0xcf, 0xf7,
        ];
        let result = hmac_sha256(&key, data);
        assert_eq!(result, expected);

        // Test case 2
        let key = b"Jefe";
        let data = b"what do ya want for nothing?";
        let expected = [
            0x5b, 0xdc, 0xc1, 0x46, 0xbf, 0x60, 0x75, 0x4e,
            0x6a, 0x04, 0x24, 0x26, 0x08, 0x95, 0x75, 0xc7,
            0x5a, 0x00, 0x3f, 0x08, 0x9d, 0x27, 0x39, 0x83,
            0x9d, 0xec, 0x58, 0xb9, 0x64, 0xec, 0x38, 0x43,
        ];
        let result = hmac_sha256(key, data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hmac_sha512_basic() {
        let key = b"test_key";
        let data = b"test_data";

        let result1 = hmac_sha512(key, data);
        assert_eq!(result1.len(), 64);

        // Same inputs should produce same output
        let result2 = hmac_sha512(key, data);
        assert_eq!(result1, result2);

        // Different data should produce different output
        let result3 = hmac_sha512(key, b"different_data");
        assert_ne!(result1, result3);
    }

    #[test]
    fn test_pbkdf2_hmac_sha256() {
        // RFC 7914 test vector (for PBKDF2-HMAC-SHA256)
        let password = b"password";
        let salt = b"salt";
        let iterations = 1;
        let mut output = [0u8; 32];

        pbkdf2_hmac_sha256(password, salt, iterations, &mut output);

        // Just verify the output is deterministic
        let mut output2 = [0u8; 32];
        pbkdf2_hmac_sha256(password, salt, iterations, &mut output2);
        assert_eq!(output, output2);

        // More iterations should produce different output
        let mut output3 = [0u8; 32];
        pbkdf2_hmac_sha256(password, salt, 2, &mut output3);
        assert_ne!(output, output3);
    }

    #[test]
    fn test_pbkdf2_hmac_sha256_more_iterations() {
        let password = b"password";
        let salt = b"salt";
        let iterations = 100;
        let mut output = [0u8; 32];

        pbkdf2_hmac_sha256(password, salt, iterations, &mut output);

        // Verify output is non-zero
        assert_ne!(output, [0u8; 32]);
    }

    #[test]
    fn test_hmac_different_keys() {
        let data = b"same_data";

        let result1 = hmac_sha256(b"key1", data);
        let result2 = hmac_sha256(b"key2", data);

        assert_ne!(result1, result2);
    }

    #[test]
    fn test_hmac_empty_key() {
        let key = b"";
        let data = b"test_data";

        // HMAC should work with empty key
        let result = hmac_sha256(key, data);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_hmac_empty_data() {
        let key = b"test_key";
        let data = b"";

        // HMAC should work with empty data
        let result = hmac_sha256(key, data);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_hmac_long_key() {
        // Key longer than SHA256 block size (64 bytes)
        let key = vec![0xAB_u8; 100];
        let data = b"test_data";

        let result = hmac_sha256(&key, data);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_hmac_long_data() {
        let key = b"test_key";
        let data = vec![0xAB_u8; 10000];

        let result = hmac_sha256(key, &data);
        assert_eq!(result.len(), 32);
    }
}
