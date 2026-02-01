//! Encryption and decryption functions for secure storage.

use aes::Aes256;
use cbc::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

use crate::error::{StorageError, StorageResult};
use crate::secure::types::{Secret, ValueHash};

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

/// Nonce size for encryption (16 bytes, 128 bits).
const NONCE_SIZE: usize = 16;

/// Encrypts data using the given secret.
///
/// Returns (encrypted_data, value_hash) where:
/// - encrypted_data includes a random nonce prefix
/// - value_hash is computed from the original data
pub fn encrypt_value(secret: &Secret, data: &[u8]) -> StorageResult<(Vec<u8>, ValueHash)> {
    if data.is_empty() {
        return Err(StorageError::InvalidParameter(
            "Cannot encrypt empty data".to_string(),
        ));
    }

    // Generate random nonce
    let mut nonce = [0u8; NONCE_SIZE];
    getrandom::getrandom(&mut nonce)
        .map_err(|e| StorageError::CryptoError(format!("Failed to generate nonce: {}", e)))?;

    // Derive key from secret (simple version: use secret bytes directly as key)
    let key = secret.as_bytes();

    // Encrypt using AES-256-CBC
    let mut encryptor = Aes256CbcEnc::new_from_slices(key, &nonce)
        .map_err(|e| StorageError::CryptoError(format!("Failed to create encryptor: {}", e)))?;

    // Pad and encrypt data
    let mut encrypted_data = data.to_vec();
    let padded_len = (encrypted_data.len() + 15) / 16 * 16;
    encrypted_data.resize(padded_len, 0);

    let total_len = nonce.len() + padded_len;
    let mut result = nonce.to_vec();
    result.append(&mut encrypted_data);

    encryptor
        .encrypt_padded_mut::<Pkcs7>(&mut result[nonce.len()..], result.len() - nonce.len())
        .map_err(|e| StorageError::CryptoError(format!("Encryption failed: {}", e)))?;

    // Compute hash of original data
    let hash = ValueHash::compute(data);

    Ok((result, hash))
}

/// Decrypts data using the given secret and expected hash.
///
/// # Arguments
/// * `secret` - The secret key for decryption
/// * `expected_hash` - The expected hash of the decrypted data (for validation)
/// * `encrypted_data` - The encrypted data with nonce prefix
///
/// # Errors
/// Returns an error if:
/// - The encrypted data is too short (missing nonce)
/// - Decryption fails
/// - The decrypted data doesn't match the expected hash
pub fn decrypt_value(
    secret: &Secret,
    expected_hash: &ValueHash,
    encrypted_data: &[u8],
) -> StorageResult<Vec<u8>> {
    if encrypted_data.len() < NONCE_SIZE {
        return Err(StorageError::CryptoError(
            "Encrypted data too short (missing nonce)".to_string(),
        ));
    }

    // Extract nonce and ciphertext
    let (nonce, ciphertext) = encrypted_data.split_at(NONCE_SIZE);

    // Derive key from secret
    let key = secret.as_bytes();

    // Decrypt using AES-256-CBC
    let mut decryptor = Aes256CbcDec::new_from_slices(key, nonce)
        .map_err(|e| StorageError::CryptoError(format!("Failed to create decryptor: {}", e)))?;

    let mut decrypted = ciphertext.to_vec();

    let decrypted_len = decryptor
        .decrypt_padded_mut::<Pkcs7>(&mut decrypted)
        .map_err(|e| StorageError::CryptoError(format!("Decryption failed: {}", e)))?
        .len();

    decrypted.truncate(decrypted_len);

    // Verify hash
    let actual_hash = ValueHash::compute(&decrypted);
    if actual_hash.as_bytes() != expected_hash.as_bytes() {
        return Err(StorageError::CryptoError(
            "Decrypted data hash mismatch".to_string(),
        ));
    }

    Ok(decrypted)
}

/// Derives an AES key from a password using PBKDF2.
///
/// # Arguments
/// * `password` - The password bytes
/// * `salt` - The salt for key derivation
/// * `iterations` - Number of PBKDF2 iterations (recommended: 100_000)
///
/// # Returns
/// A 32-byte key suitable for AES-256
pub fn derive_key_pbkdf2(password: &[u8], salt: &[u8], iterations: u32) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut key);
    key
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secure::types::Secret;

    #[test]
    #[cfg(feature = "secure")]
    fn test_encrypt_decrypt_roundtrip() {
        let secret = Secret::generate();
        let data = b"Hello, secure world!";

        let (encrypted, hash) = encrypt_value(&secret, data).unwrap();
        assert_ne!(encrypted, data.to_vec());

        let decrypted = decrypt_value(&secret, &hash, &encrypted).unwrap();
        assert_eq!(decrypted, data.to_vec());
    }

    #[test]
    #[cfg(feature = "secure")]
    fn test_decrypt_wrong_secret() {
        let secret1 = Secret::generate();
        let secret2 = Secret::generate();

        let data = b"Secret message";
        let (encrypted, hash) = encrypt_value(&secret1, data).unwrap();

        let result = decrypt_value(&secret2, &hash, &encrypted);
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "secure")]
    fn test_decrypt_tampered_data() {
        let secret = Secret::generate();
        let data = b"Secret message";

        let (mut encrypted, hash) = encrypt_value(&secret, data).unwrap();

        // Tamper with encrypted data
        encrypted[20] ^= 0xFF;

        let result = decrypt_value(&secret, &hash, &encrypted);
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "secure")]
    fn test_decrypt_empty_data() {
        let secret = Secret::generate();
        let result = encrypt_value(&secret, b"");
        assert!(result.is_err());
    }

    #[test]
    fn test_derive_key_pbkdf2() {
        let password = b"test_password";
        let salt = b"test_salt";
        let iterations = 10_000;

        let key1 = derive_key_pbkdf2(password, salt, iterations);
        let key2 = derive_key_pbkdf2(password, salt, iterations);

        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32);
    }

    #[test]
    fn test_derive_key_different_inputs() {
        let password = b"test_password";
        let salt1 = b"salt1";
        let salt2 = b"salt2";

        let key1 = derive_key_pbkdf2(password, salt1, 10_000);
        let key2 = derive_key_pbkdf2(password, salt2, 10_000);

        assert_ne!(key1, key2);
    }
}
