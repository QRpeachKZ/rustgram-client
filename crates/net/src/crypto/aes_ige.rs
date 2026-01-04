// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! AES-IGE (Infinite Garble Extension) encryption/decryption.
//!
//! IGE mode is a variant of CBC mode that provides bidirectional error
//! propagation. It is used in MTProto 2.0 for encrypting packets.
//!
//! # Algorithm
//!
//! IGE mode uses two IV parts:
//! ```text
//! IV = IV1 || IV2  (each 16 bytes for AES-256)
//!
//! Encryption (for i = 0 to n-1):
//!     C[i] = E(K, P[i] ^ C[i-1]) ^ IV[i], where C[-1] = IV1
//!
//! Decryption (for i = 0 to n-1):
//!     P[i] = D(K, C[i] ^ IV[i]) ^ C[i-1], where C[-1] = IV2
//! ```
//!
//! # References
//!
//! - TDLib: `td/utils/crypto.cpp`
//! - MTProto: <https://core.telegram.org/mtproto/description>

use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit};
use aes::Aes256;

/// AES-256-IGE encryption context.
///
/// Provides AES-256 encryption in IGE (Infinite Garble Extension) mode.
///
/// # Examples
///
/// ```no_run
/// use rustgram_net::crypto::AesIge;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let key = [0u8; 32];
/// let mut iv = [0u8; 32];
/// let mut data = vec![0u8; 64];
///
/// AesIge::encrypt(&key, &mut iv, &mut data);
/// # Ok(())
/// # }
/// ```
pub struct AesIge;

/// Block size for AES (16 bytes).
const AES_BLOCK_SIZE: usize = 16;

impl AesIge {
    /// Encrypts data in-place using AES-256-IGE mode.
    ///
    /// # Arguments
    ///
    /// * `key` - 256-bit (32 byte) encryption key
    /// * `iv` - 256-bit (32 byte) initialization vector (IV1 || IV2)
    /// * `data` - Data to encrypt, must be multiple of 16 bytes
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `data.len()` is not a multiple of 16
    /// - `data` is empty
    ///
    /// # Panics
    ///
    /// This function does not panic. All error conditions are handled via `Result`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_net::crypto::AesIge;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let key = [42u8; 32];
    /// let mut iv = [1u8; 32];
    /// let mut plaintext = b"Hello, World!!!!".to_vec();  // 16 bytes (padded)
    ///
    /// AesIge::encrypt(&key, &mut iv, &mut plaintext)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn encrypt(key: &[u8; 32], iv: &mut [u8; 32], data: &mut [u8]) -> Result<(), CryptoError> {
        if data.is_empty() {
            return Err(CryptoError::EmptyData);
        }

        if data.len() % AES_BLOCK_SIZE != 0 {
            return Err(CryptoError::InvalidBlockSize {
                actual: data.len(),
                expected: AES_BLOCK_SIZE,
            });
        }

        // Split IV into IV1 and IV2
        let (iv1, iv2) = iv.split_at_mut(AES_BLOCK_SIZE);

        // Initialize AES cipher
        let mut cipher = Aes256::new(key.into());

        // IGE encryption: C[i] = E(K, P[i] ^ C[i-1]) ^ IV[i]
        // For encryption, C[-1] = IV1, and we use IV2 for XORing
        let mut prev_cipherblock = [0u8; AES_BLOCK_SIZE];
        prev_cipherblock.copy_from_slice(iv1);

        for chunk in data.chunks_exact_mut(AES_BLOCK_SIZE) {
            // XOR plaintext with previous ciphertext block (or IV1 for first block)
            let mut temp = [0u8; AES_BLOCK_SIZE];
            temp.copy_from_slice(chunk);
            xor_inplace(&mut temp, &prev_cipherblock);

            // Encrypt the block
            cipher.encrypt_block_mut((&mut temp).into());

            // XOR with IV2
            xor_inplace(&mut temp, iv2);

            // Store result and update previous cipherblock
            chunk.copy_from_slice(&temp);
            prev_cipherblock.copy_from_slice(chunk);
        }

        // Update IV1 with the last encrypted block for chaining
        iv[..AES_BLOCK_SIZE].copy_from_slice(&prev_cipherblock);

        Ok(())
    }

    /// Decrypts data in-place using AES-256-IGE mode.
    ///
    /// # Arguments
    ///
    /// * `key` - 256-bit (32 byte) decryption key
    /// * `iv` - 256-bit (32 byte) initialization vector (IV1 || IV2)
    /// * `data` - Data to decrypt, must be multiple of 16 bytes
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `data.len()` is not a multiple of 16
    /// - `data` is empty
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_net::crypto::AesIge;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let key = [42u8; 32];
    /// let mut iv = [1u8; 32];
    /// let mut ciphertext = vec![0u8; 64];
    ///
    /// AesIge::decrypt(&key, &mut iv, &mut ciphertext)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn decrypt(key: &[u8; 32], iv: &mut [u8; 32], data: &mut [u8]) -> Result<(), CryptoError> {
        if data.is_empty() {
            return Err(CryptoError::EmptyData);
        }

        if data.len() % AES_BLOCK_SIZE != 0 {
            return Err(CryptoError::InvalidBlockSize {
                actual: data.len(),
                expected: AES_BLOCK_SIZE,
            });
        }

        // Split IV into IV1 and IV2
        let (iv1, iv2) = iv.split_at_mut(AES_BLOCK_SIZE);

        // Initialize AES cipher
        let mut cipher = Aes256::new(key.into());

        // IGE decryption: P[i] = D(K, C[i] ^ IV2) ^ C[i-1]
        // For decryption, C[-1] = IV1
        let mut prev_cipherblock = [0u8; AES_BLOCK_SIZE];
        prev_cipherblock.copy_from_slice(iv1);

        for chunk in data.chunks_exact_mut(AES_BLOCK_SIZE) {
            // Save current ciphertext block for next iteration
            let mut current_cipherblock = [0u8; AES_BLOCK_SIZE];
            current_cipherblock.copy_from_slice(chunk);

            // XOR ciphertext with IV2
            let mut temp = [0u8; AES_BLOCK_SIZE];
            temp.copy_from_slice(chunk);
            xor_inplace(&mut temp, iv2);

            // Decrypt the block
            cipher.decrypt_block_mut((&mut temp).into());

            // XOR with previous ciphertext block (or IV1 for first block)
            xor_inplace(&mut temp, &prev_cipherblock);

            // Store result
            chunk.copy_from_slice(&temp);

            // Store current ciphertext for next iteration
            prev_cipherblock = current_cipherblock;
        }

        Ok(())
    }
}

/// XORs `src` into `dst` in-place.
///
/// `dst` will contain `dst ^ src` after this operation.
#[inline]
fn xor_inplace(dst: &mut [u8], src: &[u8]) {
    debug_assert_eq!(dst.len(), src.len());
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d ^= s;
    }
}

/// Convenience function for AES-IGE encryption.
///
/// # Examples
///
/// ```no_run
/// # use rustgram_net::crypto::aes_ige_encrypt;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let key = [0u8; 32];
/// let mut iv = [0u8; 32];
/// let mut data = vec![0u8; 64];
///
/// aes_ige_encrypt(&key, &mut iv, &mut data)?;
/// # Ok(())
/// # }
/// ```
pub fn aes_ige_encrypt(
    key: &[u8; 32],
    iv: &mut [u8; 32],
    data: &mut [u8],
) -> Result<(), CryptoError> {
    AesIge::encrypt(key, iv, data)
}

/// Convenience function for AES-IGE decryption.
///
/// # Examples
///
/// ```no_run
/// # use rustgram_net::crypto::aes_ige_decrypt;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let key = [0u8; 32];
/// let mut iv = [0u8; 32];
/// let mut data = vec![0u8; 64];
///
/// aes_ige_decrypt(&key, &mut iv, &mut data)?;
/// # Ok(())
/// # }
/// ```
pub fn aes_ige_decrypt(
    key: &[u8; 32],
    iv: &mut [u8; 32],
    data: &mut [u8],
) -> Result<(), CryptoError> {
    AesIge::decrypt(key, iv, data)
}

/// Cryptography error type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    /// Data length is not a multiple of the block size.
    InvalidBlockSize {
        /// The actual data length.
        actual: usize,
        /// The expected block size.
        expected: usize,
    },

    /// Data is empty.
    EmptyData,

    /// Invalid key length.
    InvalidKeyLength {
        /// The actual key length.
        actual: usize,
        /// The expected key length.
        expected: usize,
    },
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBlockSize { actual, expected } => write!(
                f,
                "Invalid block size: data length ({}) is not a multiple of {}",
                actual, expected
            ),
            Self::EmptyData => write!(f, "Cannot encrypt/decrypt empty data"),
            Self::InvalidKeyLength { actual, expected } => write!(
                f,
                "Invalid key length: expected {}, got {}",
                expected, actual
            ),
        }
    }
}

impl std::error::Error for CryptoError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_inplace() {
        let mut a = [0x12, 0x34, 0x56, 0x78];
        let b = [0xFF, 0x00, 0xFF, 0x00];
        xor_inplace(&mut a, &b);
        assert_eq!(a, [0xED, 0x34, 0xA9, 0x78]);
    }

    #[test]
    fn test_aes_ige_empty_data() {
        let key = [0u8; 32];
        let mut iv = [0u8; 32];
        let mut data = vec![];

        let result = AesIge::encrypt(&key, &mut iv, &mut data);
        assert_eq!(result, Err(CryptoError::EmptyData));

        let result = AesIge::decrypt(&key, &mut iv, &mut data);
        assert_eq!(result, Err(CryptoError::EmptyData));
    }

    #[test]
    fn test_aes_ige_invalid_block_size() {
        let key = [0u8; 32];
        let mut iv = [0u8; 32];
        let mut data = vec![0u8; 15]; // Not a multiple of 16

        let result = AesIge::encrypt(&key, &mut iv, &mut data);
        assert!(matches!(result, Err(CryptoError::InvalidBlockSize { .. })));

        let result = AesIge::decrypt(&key, &mut iv, &mut data);
        assert!(matches!(result, Err(CryptoError::InvalidBlockSize { .. })));
    }

    #[test]
    fn test_aes_ige_roundtrip() {
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];
        let iv_orig = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD,
            0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB,
            0xCC, 0xDD, 0xEE, 0xFF,
        ];

        let original = b"Hello, World!!!!This is a test.";
        let mut original_padded = original.to_vec();

        // Pad to block size
        while original_padded.len() % AES_BLOCK_SIZE != 0 {
            original_padded.push(0);
        }

        let mut data = original_padded.clone();

        // Encrypt with original IV
        let mut iv_enc = iv_orig;
        assert!(AesIge::encrypt(&key, &mut iv_enc, &mut data).is_ok());

        // Data should have changed
        assert_ne!(&data, &original_padded);

        // Decrypt with original IV (not the modified one)
        let mut iv_dec = iv_orig;
        assert!(AesIge::decrypt(&key, &mut iv_dec, &mut data).is_ok());

        // Should match original (with padding)
        assert_eq!(&data, &original_padded);
    }

    #[test]
    fn test_aes_ige_single_block() {
        let key = [0x42u8; 32];
        let iv_orig = [0x13u8; 32];
        let original: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut data = original;

        // For encryption, use the original IV
        let mut iv_enc = iv_orig;
        assert!(AesIge::encrypt(&key, &mut iv_enc, &mut data).is_ok());
        assert_ne!(data, original);

        // For decryption, we need to use the ORIGINAL IV, not the modified one
        // In MTProto, the IV is reset for each message direction
        let mut iv_dec = iv_orig;
        assert!(AesIge::decrypt(&key, &mut iv_dec, &mut data).is_ok());
        assert_eq!(data, original);
    }

    #[test]
    fn test_aes_ige_multiple_blocks() {
        let key = [1u8; 32];
        let iv_orig = [2u8; 32];
        let original = vec![3u8; 64]; // 4 blocks
        let mut data = original.clone();

        // Encrypt with original IV
        let mut iv_enc = iv_orig;
        assert!(AesIge::encrypt(&key, &mut iv_enc, &mut data).is_ok());
        assert_ne!(data, original);

        // Decrypt with original IV
        let mut iv_dec = iv_orig;
        assert!(AesIge::decrypt(&key, &mut iv_dec, &mut data).is_ok());
        assert_eq!(data, original);
    }

    #[test]
    fn test_aes_ige_different_inputs_different_outputs() {
        let key = [0u8; 32];
        let mut iv1 = [0u8; 32];
        let mut iv2 = [0u8; 32];

        let mut data1 = vec![1u8; 32];
        let mut data2 = vec![2u8; 32];

        assert!(AesIge::encrypt(&key, &mut iv1, &mut data1).is_ok());
        assert!(AesIge::encrypt(&key, &mut iv2, &mut data2).is_ok());

        assert_ne!(data1, data2);
    }

    #[test]
    fn test_crypto_error_display() {
        let err = CryptoError::InvalidBlockSize {
            actual: 15,
            expected: 16,
        };
        let s = format!("{err}");
        assert!(s.contains("Invalid block size"));
        assert!(s.contains("15"));
        assert!(s.contains("16"));

        let err = CryptoError::EmptyData;
        let s = format!("{err}");
        assert!(s.contains("empty"));
    }

    // Property-based tests
    #[cfg(feature = "proptest")]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn prop_aes_ige_roundtrip(
                key in prop::array::uniform32(0u8..),
                iv in prop::array::uniform32(0u8..),
                data in prop::collection::vec(0u8..=255, 16)
            ) {
                // Only test multiples of block size
                if data.len() % 16 == 0 && !data.is_empty() {
                    let mut iv_enc = iv;
                    let mut iv_dec = iv;
                    let mut encrypted = data.clone();
                    let mut decrypted = data.clone();

                    prop_assert!(AesIge::encrypt(&key, &mut iv_enc, &mut encrypted).is_ok());
                    prop_assert!(AesIge::decrypt(&key, &mut iv_dec, &mut decrypted).is_ok());

                    prop_assert_eq!(&data[..], &encrypted[..]);
                    prop_assert_eq!(&data[..], &decrypted[..]);
                }
            }

            #[test]
            fn prop_encrypt_changes_data(
                key in prop::array::uniform32(0u8..),
                iv in prop::array::uniform32(0u8..),
            ) {
                let mut iv = iv;
                let mut data = vec![42u8; 32];

                let before = data.clone();
                prop_assert!(AesIge::encrypt(&key, &mut iv, &mut data).is_ok());

                // With random key/IV, data should almost certainly change
                // (except for the 1/2^256 chance of no change)
                prop_assert_ne!(data, before);
            }
        }
    }
}
