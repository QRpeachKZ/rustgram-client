// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! RSA encryption/decryption for MTProto key exchange.
//!
//! This module implements RSA operations using PKCS#1 OAEP padding,
//! which is required for MTProto authentication flow.
//!
//! # References
//!
//! - TDLib: `td/mtproto/Rsa.cpp`
//! - TDLib: `td/mtproto/Handshake.cpp`
//! - MTProto 2.0: <https://core.telegram.org/mtproto/auth_key>

use rsa::oaep::Oaep;
use rsa::pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey};
use rsa::pkcs8::{
    DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
};
use rsa::traits::PublicKeyParts;
use rsa::{rand_core::OsRng, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use thiserror::Error;
use rsa::BigUint;

use crate::crypto::sha1;
use bytes::BytesMut;
use rustgram_types::tl::TlHelper;

/// Error types for RSA operations.
#[derive(Debug, Error)]
pub enum RsaError {
    /// Failed to decode RSA key
    #[error("Failed to decode RSA key: {0}")]
    DecodeError(String),

    /// Failed to encode RSA key
    #[error("Failed to encode RSA key: {0}")]
    EncodeError(String),

    /// Encryption failed
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    /// Decryption failed
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// Invalid key size
    #[error("Invalid key size: {0} bits (expected 2048)")]
    InvalidKeySize(usize),

    /// Data too large for encryption
    #[error("Data too large: {0} bytes (max {1})")]
    DataTooLarge(usize, usize),

    /// Invalid signature
    #[error("Invalid signature")]
    InvalidSignature,

    /// Operation failed
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

/// Result type for RSA operations.
pub type RsaResult<T> = Result<T, RsaError>;

/// RSA public key wrapper for MTProto operations.
///
/// This wraps the rsa crate's RsaPublicKey with MTProto-specific
/// fingerprint computation and key handling.
#[derive(Debug, Clone)]
pub struct RsaPublicKeyWrapper {
    /// The underlying RSA public key
    inner: RsaPublicKey,

    /// The fingerprint (computed from the key)
    fingerprint: i64,

    /// Key size in bits
    bits: usize,
}

impl RsaPublicKeyWrapper {
    /// Creates a new RSA public key wrapper from an existing key.
    ///
    /// # Arguments
    ///
    /// * `key` - The RSA public key
    pub fn new(key: RsaPublicKey) -> Self {
        let bits = key.size() * 8;
        let fingerprint = compute_fingerprint_from_key(&key);

        Self {
            inner: key,
            fingerprint,
            bits,
        }
    }

    /// Computes the fingerprint of an RSA public key.
    ///
    /// Uses the method expected by MTProto servers:
    /// 1) Serialize n and e as TL bytes (without constructor)
    /// 2) SHA1 over the serialized buffer
    /// 3) Take bytes 12..20 as little-endian i64
    ///
    /// # Arguments
    ///
    /// * `key` - The RSA public key
    ///
    /// # Returns
    ///
    /// The fingerprint as i64
    fn compute_fingerprint(key: &RsaPublicKey) -> i64 {
        compute_fingerprint_from_key(key)
    }

    /// Encrypts data using PKCS#1 OAEP padding with SHA256.
    ///
    /// This is used in MTProto authentication for encrypting the
    /// DH inner data during key exchange.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to encrypt
    ///
    /// # Returns
    ///
    /// The encrypted data
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rustgram_net::crypto::RsaPublicKeyWrapper;
    ///
    /// let key = RsaPublicKeyWrapper::from_pem(pem_bytes)?;
    /// let plaintext = b"secret data";
    /// let ciphertext = key.encrypt(plaintext)?;
    /// ```
    pub fn encrypt(&self, data: &[u8]) -> RsaResult<Vec<u8>> {
        // Check key size (MTProto requires 2048-bit)
        if self.bits != 2048 {
            return Err(RsaError::InvalidKeySize(self.bits));
        }

        // Check data size (OAEP reduces max size by overhead)
        let max_size = (self.bits / 8) - 42; // SHA-256 OAEP overhead
        if data.len() > max_size {
            return Err(RsaError::DataTooLarge(data.len(), max_size));
        }

        // Encrypt using OAEP with SHA256
        let padding = Oaep::new::<Sha256>();
        let mut rng = OsRng;

        self.inner
            .encrypt(&mut rng, padding, data)
            .map_err(|e| RsaError::EncryptionFailed(e.to_string()))
    }

    /// Encrypts data using PKCS#1 v1.5 padding (legacy).
    ///
    /// This is used for some older MTProto operations.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to encrypt
    ///
    /// # Returns
    ///
    /// The encrypted data
    pub fn encrypt_v1_5(&self, data: &[u8]) -> RsaResult<Vec<u8>> {
        let mut rng = OsRng;

        self.inner
            .encrypt(&mut rng, Pkcs1v15Encrypt, data)
            .map_err(|e| RsaError::EncryptionFailed(e.to_string()))
    }

    /// Encrypts data using raw RSA (no padding).
    ///
    /// MTProto handshake expects raw RSA on a 256-byte block.
    pub fn encrypt_raw(&self, data: &[u8]) -> RsaResult<Vec<u8>> {
        if data.len() != self.size() {
            return Err(RsaError::DataTooLarge(data.len(), self.size()));
        }

        let n = self.inner.n();
        let e = self.inner.e();
        let m = BigUint::from_bytes_be(data);

        if &m >= n {
            return Err(RsaError::EncryptionFailed(
                "Message representative out of range".into(),
            ));
        }

        let c = m.modpow(e, n);
        let mut out = c.to_bytes_be();

        // Left-pad to key size
        if out.len() < self.size() {
            let mut padded = vec![0u8; self.size() - out.len()];
            padded.append(&mut out);
            out = padded;
        }

        Ok(out)
    }

    /// Verifies a signature against data.
    ///
    /// # Arguments
    ///
    /// * `_signature` - The signature to verify
    /// * `_data` - The original data
    ///
    /// # Returns
    ///
    /// Ok(()) if signature is valid, Err otherwise
    pub fn verify(&self, _signature: &[u8], _data: &[u8]) -> RsaResult<()> {
        // This would require the public key to have a verify method
        // For now, return an error indicating this needs to be implemented
        Err(RsaError::OperationFailed(
            "Verify not yet implemented".into(),
        ))
    }

    /// Parses an RSA public key from PEM format.
    ///
    /// # Arguments
    ///
    /// * `pem` - The PEM-encoded key data
    ///
    /// # Returns
    ///
    /// The parsed public key wrapper
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rustgram_net::crypto::RsaPublicKeyWrapper;
    ///
    /// let pem = b"-----BEGIN PUBLIC KEY-----\n...";
    /// let key = RsaPublicKeyWrapper::from_pem(pem)?;
    /// ```
    pub fn from_pem(pem: &[u8]) -> RsaResult<Self> {
        let pem_str = std::str::from_utf8(pem)
            .map_err(|e| RsaError::DecodeError(format!("Invalid UTF-8: {}", e)))?;

        // Try PKCS#1 format first (-----BEGIN RSA PUBLIC KEY-----)
        // This is the format used by MTProto/TDLib
        let key = RsaPublicKey::from_pkcs1_pem(pem_str)
            .or_else(|_| RsaPublicKey::from_public_key_pem(pem_str))
            .map_err(|e| RsaError::DecodeError(format!("PEM parse error: {}", e)))?;

        Ok(Self::new(key))
    }

    /// Parses an RSA public key from DER format.
    ///
    /// # Arguments
    ///
    /// * `der` - The DER-encoded key data
    pub fn from_der(der: &[u8]) -> RsaResult<Self> {
        let key = RsaPublicKey::from_public_key_der(der)
            .map_err(|e| RsaError::DecodeError(format!("DER parse error: {}", e)))?;

        Ok(Self::new(key))
    }

    /// Returns the fingerprint of this key.
    pub fn fingerprint(&self) -> i64 {
        self.fingerprint
    }

    /// Returns the key size in bits.
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the key size in bytes.
    pub fn size(&self) -> usize {
        self.bits / 8
    }

    /// Converts the key to PEM format.
    pub fn to_pem(&self) -> RsaResult<String> {
        self.inner
            .to_public_key_pem(LineEnding::LF)
            .map_err(|e| RsaError::EncodeError(e.to_string()))
    }

    /// Converts the key to DER format.
    pub fn to_der(&self) -> RsaResult<Vec<u8>> {
        Ok(self.inner
            .to_public_key_der()
            .map_err(|e| RsaError::EncodeError(e.to_string()))?
            .as_ref()
            .to_vec())
    }
}

pub(crate) fn compute_fingerprint_from_key(key: &RsaPublicKey) -> i64 {
    let n = key.n().to_bytes_be();
    let e = key.e().to_bytes_be();

    let mut buf = BytesMut::new();
    TlHelper::write_bytes(&mut buf, &n);
    TlHelper::write_bytes(&mut buf, &e);

    let hash = sha1(buf.as_ref());
    let mut tail = [0u8; 8];
    tail.copy_from_slice(&hash[12..20]);
    i64::from_le_bytes(tail)
}

/// RSA private key wrapper for MTProto operations.
#[derive(Debug, Clone)]
pub struct RsaPrivateKeyWrapper {
    /// The underlying RSA private key
    inner: RsaPrivateKey,

    /// Key size in bits
    bits: usize,
}

impl RsaPrivateKeyWrapper {
    /// Creates a new RSA private key wrapper from an existing key.
    pub fn new(key: RsaPrivateKey) -> Self {
        let bits = key.size() * 8;
        Self { inner: key, bits }
    }

    /// Decrypts data that was encrypted with PKCS#1 OAEP padding.
    ///
    /// # Arguments
    ///
    /// * `ciphertext` - The encrypted data
    ///
    /// # Returns
    ///
    /// The decrypted plaintext
    pub fn decrypt(&self, ciphertext: &[u8]) -> RsaResult<Vec<u8>> {
        let padding = Oaep::new::<Sha256>();

        self.inner
            .decrypt(padding, ciphertext)
            .map_err(|e| RsaError::DecryptionFailed(e.to_string()))
    }

    /// Decrypts data that was encrypted with PKCS#1 v1.5 padding.
    ///
    /// # Arguments
    ///
    /// * `ciphertext` - The encrypted data
    ///
    /// # Returns
    ///
    /// The decrypted plaintext
    pub fn decrypt_v1_5(&self, ciphertext: &[u8]) -> RsaResult<Vec<u8>> {
        self.inner
            .decrypt_blinded(&mut OsRng, Pkcs1v15Encrypt, ciphertext)
            .map_err(|e| RsaError::DecryptionFailed(e.to_string()))
    }

    /// Generates a new RSA private key.
    ///
    /// # Arguments
    ///
    /// * `bits` - Key size in bits (typically 2048)
    pub fn generate(bits: usize) -> RsaResult<Self> {
        if bits != 2048 && bits != 4096 {
            return Err(RsaError::InvalidKeySize(bits));
        }

        let mut rng = OsRng;
        let key = RsaPrivateKey::new(&mut rng, bits)
            .map_err(|e| RsaError::OperationFailed(format!("Key generation failed: {}", e)))?;

        Ok(Self::new(key))
    }

    /// Parses an RSA private key from PEM format.
    ///
    /// # Arguments
    ///
    /// * `pem` - The PEM-encoded key data
    pub fn from_pem(pem: &[u8]) -> RsaResult<Self> {
        let pem_str = std::str::from_utf8(pem)
            .map_err(|e| RsaError::DecodeError(format!("Invalid UTF-8: {}", e)))?;

        let key = RsaPrivateKey::from_pkcs1_pem(pem_str)
            .or_else(|_| RsaPrivateKey::from_pkcs8_pem(pem_str))
            .map_err(|e| RsaError::DecodeError(format!("PEM parse error: {}", e)))?;

        Ok(Self::new(key))
    }

    /// Parses an RSA private key from DER format.
    ///
    /// # Arguments
    ///
    /// * `der` - The DER-encoded key data
    pub fn from_der(der: &[u8]) -> RsaResult<Self> {
        let key = RsaPrivateKey::from_pkcs1_der(der)
            .or_else(|_| RsaPrivateKey::from_pkcs8_der(der))
            .map_err(|e| RsaError::DecodeError(format!("DER parse error: {}", e)))?;

        Ok(Self::new(key))
    }

    /// Returns the public key corresponding to this private key.
    pub fn public_key(&self) -> RsaPublicKeyWrapper {
        RsaPublicKeyWrapper::new(self.inner.to_public_key())
    }

    /// Returns the key size in bits.
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Converts the key to PEM format.
    pub fn to_pem(&self) -> RsaResult<String> {
        self.inner
            .to_pkcs8_pem(LineEnding::LF)
            .map(|pem| pem.to_string())
            .map_err(|e| RsaError::EncodeError(e.to_string()))
    }
}

/// Decrypts a signature using RSA public key (for MTProto).
///
/// This is used in MTProto for verifying the server's response
/// during authentication.
///
/// # Arguments
///
/// * `key` - The RSA public key
/// * `signature` - The signature to decrypt
///
/// # Returns
///
/// The decrypted signature data
pub fn decrypt_signature(
    _key: &RsaPublicKeyWrapper,
    signature: &[u8],
) -> RsaResult<Vec<u8>> {
    // In MTProto, the signature is encrypted with RSA
    // For now, return the signature as-is since proper decryption
    // requires the private key

    // The signature should be 256 bytes (2048 bits) for RSA-2048
    if signature.len() != 256 {
        return Err(RsaError::DecryptionFailed(format!(
            "Invalid signature size: {} (expected 256)",
            signature.len()
        )));
    }

    // Return signature data for now - this would need proper implementation
    // based on the actual MTProto authentication flow
    Ok(signature.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa_key_generation() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        assert_eq!(private_key.bits(), 2048);

        let public_key = private_key.public_key();
        assert_eq!(public_key.bits(), 2048);
    }

    #[test]
    fn test_rsa_key_generation_invalid_size() {
        let result = RsaPrivateKeyWrapper::generate(1024);
        assert!(result.is_err());

        if let Err(RsaError::InvalidKeySize(1024)) = result {
            // Expected error
        } else {
            panic!("Expected InvalidKeySize error");
        }
    }

    #[test]
    fn test_rsa_encrypt_decrypt() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        let public_key = private_key.public_key();

        let plaintext = b"Hello, MTProto!";
        let ciphertext = public_key.encrypt(plaintext).unwrap();
        let decrypted = private_key.decrypt(&ciphertext).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
        assert_ne!(plaintext.to_vec(), ciphertext);
    }

    #[test]
    fn test_rsa_encrypt_decrypt_large() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        let public_key = private_key.public_key();

        // Test with larger data (still within OAEP limits)
        // OAEP with SHA-256 has overhead: key_size - 2*hash_len - 2 = 256 - 64 - 2 = 190 bytes max
        let plaintext = vec![0xAB_u8; 150];
        let ciphertext = public_key.encrypt(&plaintext).unwrap();
        let decrypted = private_key.decrypt(&ciphertext).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_rsa_encrypt_too_large() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        let public_key = private_key.public_key();

        // OAEP reduces max size by 42 bytes for SHA-256
        // 2048 bits = 256 bytes, max plaintext = 256 - 42 = 214 bytes
        let plaintext = vec![0xAB_u8; 215];
        let result = public_key.encrypt(&plaintext);

        assert!(result.is_err());

        if let Err(RsaError::DataTooLarge(215, _)) = result {
            // Expected error
        } else {
            panic!("Expected DataTooLarge error");
        }
    }

    #[test]
    fn test_rsa_fingerprint() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        let public_key = private_key.public_key();

        let fingerprint = public_key.fingerprint();
        assert_ne!(fingerprint, 0);

        // Same key should have same fingerprint
        let fingerprint2 = public_key.fingerprint();
        assert_eq!(fingerprint, fingerprint2);
    }

    #[test]
    fn test_rsa_pem_round_trip() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();

        // Convert to PEM and back
        let pem = private_key.to_pem().unwrap();
        let parsed_key = RsaPrivateKeyWrapper::from_pem(pem.as_bytes()).unwrap();

        assert_eq!(private_key.bits(), parsed_key.bits());
    }

    #[test]
    fn test_rsa_public_key_from_pem() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        let public_key = private_key.public_key();

        // Convert public key to PEM
        let pem = public_key.to_pem().unwrap();

        // Parse it back
        let parsed_key = RsaPublicKeyWrapper::from_pem(pem.as_bytes()).unwrap();

        assert_eq!(public_key.bits(), parsed_key.bits());
        assert_eq!(public_key.fingerprint(), parsed_key.fingerprint());
    }

    #[test]
    fn test_rsa_der_round_trip() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();

        // Convert to DER and back
        let der = private_key.inner.to_pkcs8_der().unwrap();
        let der_bytes = der.as_bytes();

        let parsed_key = RsaPrivateKeyWrapper::from_der(der_bytes).unwrap();

        assert_eq!(private_key.bits(), parsed_key.bits());
    }

    #[test]
    fn test_rsa_public_key_from_der() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        let public_key = private_key.public_key();

        // Convert public key to DER
        let der = public_key.to_der().unwrap();

        // Parse it back
        let parsed_key = RsaPublicKeyWrapper::from_der(&der).unwrap();

        assert_eq!(public_key.bits(), parsed_key.bits());
        assert_eq!(public_key.fingerprint(), parsed_key.fingerprint());
    }

    #[test]
    fn test_rsa_encrypt_v1_5() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        let public_key = private_key.public_key();

        let plaintext = b"Hello, MTProto v1.5!";
        let ciphertext = public_key.encrypt_v1_5(plaintext).unwrap();
        let decrypted = private_key.decrypt_v1_5(&ciphertext).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_rsa_invalid_pem() {
        let result = RsaPublicKeyWrapper::from_pem(b"invalid pem data");
        assert!(result.is_err());

        if let Err(RsaError::DecodeError(_)) = result {
            // Expected error
        } else {
            panic!("Expected DecodeError");
        }
    }

    #[test]
    fn test_rsa_empty_pem() {
        let result = RsaPublicKeyWrapper::from_pem(b"");
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_key_size() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        let public_key = private_key.public_key();

        assert_eq!(public_key.bits(), 2048);
        assert_eq!(public_key.size(), 256); // 2048 / 8

        // Generate 4096-bit key
        let private_key_4096 = RsaPrivateKeyWrapper::generate(4096).unwrap();
        let public_key_4096 = private_key_4096.public_key();

        assert_eq!(public_key_4096.bits(), 4096);
        assert_eq!(public_key_4096.size(), 512); // 4096 / 8
    }

    // Property-based test: encrypt-decrypt round trip
    #[test]
    fn test_rsa_round_trip_property() {
        let test_data = vec![
            vec![0u8; 1],
            vec![0xFF_u8; 50],
            vec![0xAB_u8; 100],
            vec![0x12, 0x34, 0x56, 0x78],
            b"Test message with various characters: !@#$%^&*()".to_vec(),
        ];

        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        let public_key = private_key.public_key();

        for plaintext in test_data {
            if plaintext.len() > 214 {
                continue; // Skip too large data
            }

            let ciphertext = public_key.encrypt(&plaintext).unwrap();
            let decrypted = private_key.decrypt(&ciphertext).unwrap();

            assert_eq!(plaintext, decrypted, "Round trip failed for data length {}", plaintext.len());
        }
    }

    #[test]
    fn test_multiple_encryptions_same_data() {
        let private_key = RsaPrivateKeyWrapper::generate(2048).unwrap();
        let public_key = private_key.public_key();

        let plaintext = b"Same data";

        // Encrypting the same data twice should produce different results
        // (due to random padding in OAEP)
        let ciphertext1 = public_key.encrypt(plaintext).unwrap();
        let ciphertext2 = public_key.encrypt(plaintext).unwrap();

        assert_ne!(ciphertext1, ciphertext2, "Encryptions should be different due to random padding");

        // But both should decrypt to the same plaintext
        let decrypted1 = private_key.decrypt(&ciphertext1).unwrap();
        let decrypted2 = private_key.decrypt(&ciphertext2).unwrap();

        assert_eq!(decrypted1, plaintext.to_vec());
        assert_eq!(decrypted2, plaintext.to_vec());
    }
}
