//! Secure storage types.

use crate::error::{StorageError, StorageResult};
use bytes::Bytes;

/// A 32-byte secret key for encryption.
///
/// Following TDLib convention, the sum of all bytes modulo 255 must equal 239.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Secret(Bytes);

impl Secret {
    /// Size of a secret in bytes.
    pub const SIZE: usize = 32;

    /// The validation value: sum of bytes % 255 must equal this.
    pub const VALIDATION_SUM: u8 = 239;

    /// Creates a new secret from raw bytes.
    ///
    /// # Errors
    /// Returns an error if the bytes don't validate according to TDLib convention.
    pub fn new(bytes: Bytes) -> StorageResult<Self> {
        if bytes.len() != Self::SIZE {
            return Err(StorageError::InvalidParameter(format!(
                "Secret must be exactly {} bytes, got {}",
                Self::SIZE,
                bytes.len()
            )));
        }

        let sum: u32 = bytes.iter().map(|&b| b as u32).sum();
        if (sum % 255) as u8 != Self::VALIDATION_SUM {
            return Err(StorageError::InvalidParameter(
                "Secret bytes do not validate (sum % 255 != 239)".to_string(),
            ));
        }

        Ok(Self(bytes))
    }

    /// Generates a new random secret.
    #[cfg(feature = "secure")]
    pub fn generate() -> Self {
        use rand::Rng;

        let mut bytes = vec![0u8; Self::SIZE - 1];
        rand::thread_rng().fill(&mut bytes[..]);

        // Calculate the last byte to make the sum % 255 == 239
        let sum: u32 = bytes.iter().map(|&b| b as u32).sum();
        let last_byte = (Self::VALIDATION_SUM as u32 + 255 * 3 - (sum % 255)) as u8 % 255;
        bytes.push(last_byte);

        Self(Bytes::from(bytes))
    }

    /// Returns a reference to the secret bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consumes the secret and returns the underlying bytes.
    pub fn into_bytes(self) -> Bytes {
        self.0
    }
}

/// A SHA-256 hash value (32 bytes).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueHash(Bytes);

impl ValueHash {
    /// Size of a hash in bytes.
    pub const SIZE: usize = 32;

    /// Creates a new value hash from bytes.
    pub fn new(bytes: Bytes) -> StorageResult<Self> {
        if bytes.len() != Self::SIZE {
            return Err(StorageError::InvalidParameter(format!(
                "Hash must be exactly {} bytes, got {}",
                Self::SIZE,
                bytes.len()
            )));
        }

        Ok(Self(bytes))
    }

    /// Computes SHA-256 hash of the given data.
    pub fn compute(data: &[u8]) -> Self {
        use sha2::{Digest, Sha256};

        let hash = Sha256::digest(data);
        Self(Bytes::from(hash.to_vec()))
    }

    /// Returns a reference to the hash bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "secure")]
    fn test_secret_validation() {
        // Valid secret (manually calculated to sum % 255 == 239)
        let mut bytes = vec![0u8; 32];
        bytes[0] = 239; // First byte is 239, rest are 0
        let secret = Secret::new(Bytes::from(bytes));
        assert!(secret.is_ok());
    }

    #[test]
    #[cfg(feature = "secure")]
    fn test_secret_validation_fail() {
        // Invalid secret (all zeros, sum % 255 == 0)
        let bytes = vec![0u8; 32];
        let secret = Secret::new(Bytes::from(bytes));
        assert!(secret.is_err());
    }

    #[test]
    #[cfg(feature = "secure")]
    fn test_secret_generate() {
        let secret = Secret::generate();
        assert_eq!(secret.as_bytes().len(), Secret::SIZE);

        // Verify it validates
        assert!(Secret::new(secret.into_bytes()).is_ok());
    }

    #[test]
    fn test_value_hash_compute() {
        let data = b"test data";
        let hash = ValueHash::compute(data);
        assert_eq!(hash.as_bytes().len(), ValueHash::SIZE);
    }

    #[test]
    fn test_value_hash_size_validation() {
        let result = ValueHash::new(Bytes::from(vec![0u8; 16]));
        assert!(result.is_err());
    }
}
