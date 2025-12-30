// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! SHA-1 and SHA-256 hash functions.
//!
//! Based on TDLib's `td/utils/crypto.h`.

use sha1::Sha1;
use sha2::{Digest, Sha256};

/// Computes SHA-1 hash of the input data.
///
/// # Arguments
///
/// * `data` - Input data to hash
///
/// # Returns
///
/// 20-byte (160 bit) SHA-1 hash
///
/// # Examples
///
/// ```
/// use rustgram_net::crypto::sha1;
///
/// let hash = sha1(b"hello");
/// assert_eq!(hash.len(), 20);
/// ```
#[must_use]
pub fn sha1(data: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    *result.as_ref()
}

/// Computes SHA-256 hash of the input data.
///
/// # Arguments
///
/// * `data` - Input data to hash
///
/// # Returns
///
/// 32-byte (256 bit) SHA-256 hash
///
/// # Examples
///
/// ```
/// use rustgram_net::crypto::sha256;
///
/// let hash = sha256(b"hello");
/// assert_eq!(hash.len(), 32);
/// ```
#[must_use]
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    *result.as_ref()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test vectors from NIST
    #[test]
    fn test_sha1_empty() {
        let hash = sha1(b"");
        assert_eq!(
            hash,
            [
                0xDA, 0x39, 0xA3, 0xEE, 0x5E, 0x6B, 0x4B, 0x0D, 0x32, 0x55, 0xBF, 0xEF, 0x95, 0x60,
                0x18, 0x90, 0xAF, 0xD8, 0x07, 0x09
            ]
        );
    }

    #[test]
    fn test_sha1_abc() {
        let hash = sha1(b"abc");
        assert_eq!(
            hash,
            [
                0xA9, 0x99, 0x3E, 0x36, 0x47, 0x06, 0x81, 0x6A, 0xBA, 0x3E, 0x25, 0x71, 0x78, 0x50,
                0xC2, 0x6C, 0x9C, 0xD0, 0xD8, 0x9D
            ]
        );
    }

    #[test]
    fn test_sha1_longer() {
        let hash = sha1(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
        assert_eq!(
            hash,
            [
                0x84, 0x98, 0x3E, 0x44, 0x1C, 0x3B, 0xD2, 0x6E, 0xBA, 0xAE, 0x4A, 0xA1, 0xF9, 0x51,
                0x29, 0xE5, 0xE5, 0x46, 0x70, 0xF1
            ]
        );
    }

    #[test]
    fn test_sha256_empty() {
        let hash = sha256(b"");
        assert_eq!(
            hash,
            [
                0xE3, 0xB0, 0xC4, 0x42, 0x98, 0xFC, 0x1C, 0x14, 0x9A, 0xFB, 0xF4, 0xC8, 0x99, 0x6F,
                0xB9, 0x24, 0x27, 0xAE, 0x41, 0xE4, 0x64, 0x9B, 0x93, 0x4C, 0xA4, 0x95, 0x99, 0x1B,
                0x78, 0x52, 0xB8, 0x55
            ]
        );
    }

    #[test]
    fn test_sha256_abc() {
        let hash = sha256(b"abc");
        assert_eq!(
            hash,
            [
                0xBA, 0x78, 0x16, 0xBF, 0x8F, 0x01, 0xCF, 0xEA, 0x41, 0x41, 0x40, 0xDE, 0x5D, 0xAE,
                0x22, 0x23, 0xB0, 0x03, 0x61, 0xA3, 0x96, 0x17, 0x7A, 0x9C, 0xB4, 0x10, 0xFF, 0x61,
                0xF2, 0x00, 0x15, 0xAD
            ]
        );
    }

    #[test]
    fn test_sha256_longer() {
        let hash = sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
        assert_eq!(
            hash,
            [
                0x24, 0x8D, 0x6A, 0x61, 0xD2, 0x06, 0x38, 0xB8, 0xE5, 0xC0, 0x26, 0x93, 0x0C, 0x3E,
                0x60, 0x39, 0xA3, 0x3C, 0xE4, 0x59, 0x64, 0xFF, 0x21, 0x67, 0xF6, 0xEC, 0xED, 0xD4,
                0x19, 0xDB, 0x06, 0xC1
            ]
        );
    }

    #[test]
    fn test_sha1_different_inputs() {
        let hash1 = sha1(b"hello");
        let hash2 = sha1(b"world");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_sha256_different_inputs() {
        let hash1 = sha256(b"hello");
        let hash2 = sha256(b"world");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_sha1_avalanche_effect() {
        let hash1 = sha1(b"hello world");
        let hash2 = sha1(b"hello world!");
        // Changing one bit should dramatically change the hash
        let diff_count = hash1
            .iter()
            .zip(hash2.iter())
            .filter(|(a, b)| a != b)
            .count();
        // Most bytes should be different (avalanche effect)
        assert!(diff_count > 10);
    }

    #[test]
    fn test_sha256_avalanche_effect() {
        let hash1 = sha256(b"hello world");
        let hash2 = sha256(b"hello world!");
        // Changing one bit should dramatically change the hash
        let diff_count = hash1
            .iter()
            .zip(hash2.iter())
            .filter(|(a, b)| a != b)
            .count();
        // Most bytes should be different (avalanche effect)
        assert!(diff_count > 10);
    }

    // Property-based tests
    #[cfg(feature = "proptest")]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn prop_sha1_length(data in prop::collection::vec(0u8..256, 0..1000)) {
                let hash = sha1(&data);
                prop_assert_eq!(hash.len(), 20);
            }

            #[test]
            fn prop_sha256_length(data in prop::collection::vec(0u8..256, 0..1000)) {
                let hash = sha256(&data);
                prop_assert_eq!(hash.len(), 32);
            }

            #[test]
            fn prop_sha1_deterministic(data in prop::collection::vec(0u8..256, 0..100)) {
                let hash1 = sha1(&data);
                let hash2 = sha1(&data);
                prop_assert_eq!(hash1, hash2);
            }

            #[test]
            fn prop_sha256_deterministic(data in prop::collection::vec(0u8..256, 0..100)) {
                let hash1 = sha256(&data);
                let hash2 = sha256(&data);
                prop_assert_eq!(hash1, hash2);
            }
        }
    }
}
