// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Diffie-Hellman config type for Telegram MTProto client.
//!
//! This module implements the DH config from TDLib.
//!
//! # Example
//!
//! ```rust
//! use rustgram_dh_config::DhConfig;
//!
//! let config = DhConfig::with_params(
//!     101,
//!     "c71caeb9c6b1c9048e6c522f70f13f73980d40238e3e21c14934d037563d930f".to_string(),
//!     2
//! );
//! assert_eq!(config.version(), 101);
//! assert_eq!(config.generator(), 2);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt;

/// Diffie-Hellman configuration.
///
/// Contains parameters for DH key exchange in MTProto.
/// Based on TDLib's DhConfig implementation.
///
/// # Example
///
/// ```rust
/// use rustgram_dh_config::DhConfig;
///
/// let config = DhConfig::with_params(
///     101,
///     "c71caeb9c6b1c9048e6c522f70f13f73980d40238e3e21c14934d037563d930f".to_string(),
///     2
/// );
/// assert_eq!(config.version(), 101);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhConfig {
    /// Version of the DH config
    version: i32,
    /// Prime number as hex string
    prime: String,
    /// Generator value
    generator: i32,
}

impl DhConfig {
    /// Creates a new DH config with the given values.
    ///
    /// # Arguments
    ///
    /// * `version` - Version of the DH config
    /// * `prime` - Prime number as hex string
    /// * `generator` - Generator value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::with_params(101, "prime_hex".to_string(), 2);
    /// assert_eq!(config.version(), 101);
    /// ```
    pub fn with_params(version: i32, prime: String, generator: i32) -> Self {
        Self {
            version,
            prime,
            generator,
        }
    }

    /// Creates an empty DH config.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::empty();
    /// assert_eq!(config.version(), 0);
    /// assert!(config.prime().is_empty());
    /// ```
    pub fn empty() -> Self {
        Self {
            version: 0,
            prime: String::new(),
            generator: 0,
        }
    }

    /// Creates a default DH config with known safe parameters.
    ///
    /// Uses the well-known 2048-bit MODP group from RFC 3526.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::default_safe();
    /// assert!(config.is_valid());
    /// ```
    pub fn default_safe() -> Self {
        Self {
            version: 101,
            prime: "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1\
                   29024E088A67CC74020BBEA63B139B22514A08798E3404DD\
                   EF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245\
                   E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7ED\
                   EE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3D\
                   C2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F\
                   83655D23DCA3AD961C62F356208552BB9ED529077096966D\
                   670C354E4ABC9804F1746C08CA18217C32905E462E36CE3B\
                   E39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9\
                   DE2BCBF6955817183995497CEA956AE515D2261898FA0510\
                   15728E5A8AACAA68FFFFFFFFFFFFFFFF"
                .to_string(),
            generator: 2,
        }
    }

    /// Creates DH config from a mock telegram_api object.
    ///
    /// This is a simplified version for testing. The real implementation would
    /// parse the actual MTProto object.
    ///
    /// # Arguments
    ///
    /// * `version` - Version from telegram_api
    /// * `prime` - Prime as hex string from telegram_api
    /// * `generator` - Generator from telegram_api
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::from_telegram_api(101, "prime_hex".to_string(), 2);
    /// assert_eq!(config.version(), 101);
    /// ```
    pub fn from_telegram_api(version: i32, prime: String, generator: i32) -> Self {
        Self {
            version,
            prime,
            generator,
        }
    }

    /// Returns the version of the DH config.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::with_params(101, "prime".to_string(), 2);
    /// assert_eq!(config.version(), 101);
    /// ```
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Returns the prime as a hex string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::with_params(101, "c71caeb9".to_string(), 2);
    /// assert_eq!(config.prime(), "c71caeb9");
    /// ```
    pub fn prime(&self) -> &str {
        &self.prime
    }

    /// Returns the generator value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::with_params(101, "prime".to_string(), 2);
    /// assert_eq!(config.generator(), 2);
    /// ```
    pub fn generator(&self) -> i32 {
        self.generator
    }

    /// Returns the prime as bytes if valid hex.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::with_params(101, "c71c".to_string(), 2);
    /// let bytes = config.prime_as_bytes();
    /// assert!(bytes.is_some());
    /// ```
    pub fn prime_as_bytes(&self) -> Option<Vec<u8>> {
        hex::decode(&self.prime).ok()
    }

    /// Checks if the DH config is valid.
    ///
    /// A valid config has:
    /// - Positive version
    /// - Non-empty prime
    /// - Generator in range [2, 65537]
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::with_params(101, "c71caeb9".to_string(), 2);
    /// assert!(config.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.version > 0 && !self.prime.is_empty() && self.generator >= 2 && self.generator <= 65537
    }

    /// Checks if the prime is valid hexadecimal.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::with_params(101, "c71caeb9".to_string(), 2);
    /// assert!(config.is_valid_hex());
    /// ```
    pub fn is_valid_hex(&self) -> bool {
        self.prime_as_bytes().is_some()
    }

    /// Returns the bit length of the prime.
    ///
    /// Returns None if the prime is not valid hex.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::with_params(101, "ff".to_string(), 2);
    /// assert_eq!(config.prime_bit_length(), Some(8));
    /// ```
    pub fn prime_bit_length(&self) -> Option<usize> {
        let bytes = self.prime_as_bytes()?;
        Some(bytes.len() * 8)
    }

    /// Serializes the DH config to a byte vector.
    ///
    /// Format: version(4 bytes) | generator(4 bytes) | prime_len(4 bytes) | prime_bytes
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::with_params(101, "c71c".to_string(), 2);
    /// let bytes = config.serialize();
    /// assert!(!bytes.is_empty());
    /// ```
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Version (4 bytes, little endian)
        result.extend_from_slice(&self.version.to_le_bytes());

        // Generator (4 bytes, little endian)
        result.extend_from_slice(&self.generator.to_le_bytes());

        // Prime bytes if valid hex
        if let Some(prime_bytes) = self.prime_as_bytes() {
            let prime_len = prime_bytes.len() as u32;
            result.extend_from_slice(&prime_len.to_le_bytes());
            result.extend_from_slice(&prime_bytes);
        } else {
            // Prime length 0
            result.extend_from_slice(&0u32.to_le_bytes());
        }

        result
    }

    /// Deserializes a DH config from a byte vector.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dh_config::DhConfig;
    ///
    /// let config = DhConfig::with_params(101, "c71c".to_string(), 2);
    /// let bytes = config.serialize();
    /// let restored = DhConfig::deserialize(&bytes);
    /// assert!(restored.is_some());
    /// ```
    pub fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < 8 {
            return None;
        }

        let version = i32::from_le_bytes(data[0..4].try_into().ok()?);
        let generator = i32::from_le_bytes(data[4..8].try_into().ok()?);

        let prime = if data.len() > 8 {
            let prime_len = u32::from_le_bytes(data[8..12].try_into().ok()?) as usize;
            if data.len() < 12 + prime_len {
                return None;
            }
            let prime_bytes = &data[12..12 + prime_len];
            hex::encode(prime_bytes)
        } else {
            String::new()
        };

        Some(Self {
            version,
            prime,
            generator,
        })
    }
}

impl Default for DhConfig {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for DhConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DhConfig[version={}, generator={}, prime_len={}",
            self.version,
            self.generator,
            self.prime.len()
        )?;
        if let Some(bits) = self.prime_bit_length() {
            write!(f, ", bits={}]", bits)
        } else {
            write!(f, ", bits=invalid]")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_params() {
        let config = DhConfig::with_params(101, "prime_hex".to_string(), 2);
        assert_eq!(config.version(), 101);
        assert_eq!(config.prime(), "prime_hex");
        assert_eq!(config.generator(), 2);
    }

    #[test]
    fn test_empty() {
        let config = DhConfig::empty();
        assert_eq!(config.version(), 0);
        assert!(config.prime().is_empty());
        assert_eq!(config.generator(), 0);
    }

    #[test]
    fn test_default_safe() {
        let config = DhConfig::default_safe();
        assert_eq!(config.version(), 101);
        assert!(!config.prime().is_empty());
        assert_eq!(config.generator(), 2);
        assert!(config.is_valid());
    }

    #[test]
    fn test_from_telegram_api() {
        let config = DhConfig::from_telegram_api(100, "test_prime".to_string(), 3);
        assert_eq!(config.version(), 100);
        assert_eq!(config.prime(), "test_prime");
        assert_eq!(config.generator(), 3);
    }

    #[test]
    fn test_version() {
        let config = DhConfig::with_params(101, "prime".to_string(), 2);
        assert_eq!(config.version(), 101);
    }

    #[test]
    fn test_prime() {
        let config = DhConfig::with_params(101, "c71caeb9".to_string(), 2);
        assert_eq!(config.prime(), "c71caeb9");
    }

    #[test]
    fn test_generator() {
        let config = DhConfig::with_params(101, "prime".to_string(), 5);
        assert_eq!(config.generator(), 5);
    }

    #[test]
    fn test_prime_as_bytes_valid() {
        let config = DhConfig::with_params(101, "c71c".to_string(), 2);
        let bytes = config.prime_as_bytes();
        assert!(bytes.is_some());
        assert_eq!(bytes.unwrap(), vec![0xc7, 0x1c]);
    }

    #[test]
    fn test_prime_as_bytes_invalid() {
        let config = DhConfig::with_params(101, "xyz".to_string(), 2);
        let bytes = config.prime_as_bytes();
        assert!(bytes.is_none());
    }

    #[test]
    fn test_is_valid() {
        let config = DhConfig::with_params(101, "c71caeb9".to_string(), 2);
        assert!(config.is_valid());
    }

    #[test]
    fn test_is_valid_empty_prime() {
        let config = DhConfig::with_params(101, String::new(), 2);
        assert!(!config.is_valid());
    }

    #[test]
    fn test_is_valid_zero_version() {
        let config = DhConfig::with_params(0, "c71caeb9".to_string(), 2);
        assert!(!config.is_valid());
    }

    #[test]
    fn test_is_valid_negative_version() {
        let config = DhConfig::with_params(-1, "c71caeb9".to_string(), 2);
        assert!(!config.is_valid());
    }

    #[test]
    fn test_is_valid_generator_too_small() {
        let config = DhConfig::with_params(101, "c71caeb9".to_string(), 1);
        assert!(!config.is_valid());
    }

    #[test]
    fn test_is_valid_generator_too_large() {
        let config = DhConfig::with_params(101, "c71caeb9".to_string(), 65538);
        assert!(!config.is_valid());
    }

    #[test]
    fn test_is_valid_hex() {
        let config = DhConfig::with_params(101, "c71caeb9".to_string(), 2);
        assert!(config.is_valid_hex());
    }

    #[test]
    fn test_is_valid_hex_false() {
        let config = DhConfig::with_params(101, "xyz".to_string(), 2);
        assert!(!config.is_valid_hex());
    }

    #[test]
    fn test_prime_bit_length() {
        let config = DhConfig::with_params(101, "ff".to_string(), 2);
        assert_eq!(config.prime_bit_length(), Some(8));
    }

    #[test]
    fn test_prime_bit_length_invalid() {
        let config = DhConfig::with_params(101, "xyz".to_string(), 2);
        assert_eq!(config.prime_bit_length(), None);
    }

    #[test]
    fn test_prime_bit_length_2048() {
        let config = DhConfig::default_safe();
        assert_eq!(config.prime_bit_length(), Some(2048));
    }

    #[test]
    fn test_serialize() {
        let config = DhConfig::with_params(101, "c71c".to_string(), 2);
        let bytes = config.serialize();
        assert!(!bytes.is_empty());
        assert_eq!(bytes[0..4], 101i32.to_le_bytes());
        assert_eq!(bytes[4..8], 2i32.to_le_bytes());
    }

    #[test]
    fn test_deserialize_valid() {
        let config = DhConfig::with_params(101, "c71c".to_string(), 2);
        let bytes = config.serialize();
        let restored = DhConfig::deserialize(&bytes);
        assert!(restored.is_some());
        let restored = restored.unwrap();
        assert_eq!(restored.version(), 101);
        assert_eq!(restored.prime(), "c71c");
        assert_eq!(restored.generator(), 2);
    }

    #[test]
    fn test_deserialize_invalid_too_short() {
        let bytes = vec![1, 2, 3];
        let result = DhConfig::deserialize(&bytes);
        assert!(result.is_none());
    }

    #[test]
    fn test_deserialize_invalid_partial_prime() {
        let mut bytes = vec![0u8; 16];
        // Set prime length to 100 but only provide 4 bytes
        bytes[8..12].copy_from_slice(&100u32.to_le_bytes());
        let result = DhConfig::deserialize(&bytes);
        assert!(result.is_none());
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let config = DhConfig::with_params(100, "abcd1234".to_string(), 5);
        let bytes = config.serialize();
        let restored = DhConfig::deserialize(&bytes).unwrap();
        assert_eq!(config, restored);
    }

    #[test]
    fn test_default() {
        let config = DhConfig::default();
        assert_eq!(config.version(), 0);
        assert!(config.prime().is_empty());
        assert_eq!(config.generator(), 0);
    }

    #[test]
    fn test_equality() {
        let config1 = DhConfig::with_params(101, "prime".to_string(), 2);
        let config2 = DhConfig::with_params(101, "prime".to_string(), 2);
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_inequality_version() {
        let config1 = DhConfig::with_params(101, "prime".to_string(), 2);
        let config2 = DhConfig::with_params(102, "prime".to_string(), 2);
        assert_ne!(config1, config2);
    }

    #[test]
    fn test_inequality_prime() {
        let config1 = DhConfig::with_params(101, "prime1".to_string(), 2);
        let config2 = DhConfig::with_params(101, "prime2".to_string(), 2);
        assert_ne!(config1, config2);
    }

    #[test]
    fn test_inequality_generator() {
        let config1 = DhConfig::with_params(101, "prime".to_string(), 2);
        let config2 = DhConfig::with_params(101, "prime".to_string(), 3);
        assert_ne!(config1, config2);
    }

    #[test]
    fn test_clone() {
        let config1 = DhConfig::with_params(101, "prime".to_string(), 2);
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_display_valid() {
        let config = DhConfig::with_params(101, "ff".to_string(), 2);
        let display = format!("{}", config);
        assert!(display.contains("101"));
        assert!(display.contains("2"));
        assert!(display.contains("bits=8"));
    }

    #[test]
    fn test_display_invalid_hex() {
        let config = DhConfig::with_params(101, "xyz".to_string(), 2);
        let display = format!("{}", config);
        assert!(display.contains("bits=invalid"));
    }

    #[test]
    fn test_debug() {
        let config = DhConfig::with_params(101, "prime".to_string(), 2);
        let debug = format!("{:?}", config);
        assert!(debug.contains("DhConfig"));
    }

    #[test]
    fn test_min_generator() {
        let config = DhConfig::with_params(101, "prime".to_string(), 2);
        assert!(config.is_valid());
    }

    #[test]
    fn test_max_generator() {
        let config = DhConfig::with_params(101, "prime".to_string(), 65537);
        assert!(config.is_valid());
    }

    #[test]
    fn test_empty_prime_serialization() {
        let config = DhConfig::with_params(101, String::new(), 2);
        let bytes = config.serialize();
        // Version + generator + prime_len(0) = 12 bytes
        assert_eq!(bytes.len(), 12);
        let restored = DhConfig::deserialize(&bytes).unwrap();
        assert!(restored.prime().is_empty());
    }

    #[test]
    fn test_long_prime() {
        let long_prime = "abcd".repeat(512);
        let config = DhConfig::with_params(101, long_prime.clone(), 2);
        assert_eq!(config.prime().len(), 2048);
    }

    #[test]
    fn test_common_generator_values() {
        for generator in [2, 3, 5, 65537] {
            let config = DhConfig::with_params(101, "prime".to_string(), generator);
            assert!(config.is_valid());
        }
    }

    #[test]
    fn test_version_progression() {
        let config1 = DhConfig::with_params(100, "prime".to_string(), 2);
        let config2 = DhConfig::with_params(101, "prime".to_string(), 2);
        let config3 = DhConfig::with_params(102, "prime".to_string(), 2);

        assert!(config1.version() < config2.version());
        assert!(config2.version() < config3.version());
    }
}
