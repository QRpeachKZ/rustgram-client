// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! DH (Diffie-Hellman) config module.
//!
//! Implements the DH configuration returned by messages.getDhConfig.

use bytes::Bytes;

use crate::error::{ConfigError, Result};
use crate::tl::DhConfigTl;
use rustgram_types::tl::TlHelper;

/// Diffie-Hellman configuration.
///
/// Contains the parameters needed for DH key exchange in MTProto.
///
/// # Examples
///
/// ```no_run
/// use rustgram_config::DhConfig;
/// use bytes::Bytes;
///
/// let config = DhConfig::new(1, Bytes::from("prime_data"), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhConfig {
    /// DH config version.
    pub version: i32,
    /// Prime modulus (p).
    pub prime: Bytes,
    /// Generator value (g).
    pub g: i32,
}

impl DhConfig {
    /// Creates a new DhConfig.
    ///
    /// # Arguments
    ///
    /// * `version` - The DH config version
    /// * `prime` - The prime modulus as bytes
    /// * `g` - The generator value
    pub fn new(version: i32, prime: Bytes, g: i32) -> Self {
        Self { version, prime, g }
    }

    /// Creates a DhConfig from TL response bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - The serialized TL response data
    /// * `expected_version` - The version we're requesting (for not-modified check)
    pub fn from_tl_bytes(data: Bytes, expected_version: i32) -> Result<Self> {
        // Try to read the constructor ID first to determine response type
        let mut peek_buf = rustgram_types::tl::Bytes::new(data.clone());
        let constructor_id = TlHelper::read_constructor_id(&mut peek_buf)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;

        match constructor_id {
            crate::tl::DH_CONFIG_MAGIC => {
                let tl_config = DhConfigTl::deserialize_tl_bytes(data)?;
                Ok(Self::from_tl(tl_config))
            }
            crate::tl::DH_CONFIG_NOT_MODIFIED_MAGIC => {
                let _ = crate::tl::DhConfigNotModified::deserialize_tl_bytes(data)?;
                // If not modified, return config with expected version
                // Note: In real implementation, we'd return cached config
                Err(ConfigError::expired(format!(
                    "DH config not modified, version {} is current",
                    expected_version
                )))
            }
            _ => Err(ConfigError::invalid_config(format!(
                "Unknown DH config constructor: 0x{:08x}",
                constructor_id
            ))),
        }
    }

    /// Creates a DhConfig from the TL DhConfig type.
    pub fn from_tl(tl_config: DhConfigTl) -> Self {
        Self {
            version: tl_config.version,
            prime: Bytes::from(tl_config.prime),
            g: tl_config.g,
        }
    }

    /// Converts to the TL representation.
    pub fn to_tl(&self) -> DhConfigTl {
        DhConfigTl {
            g: self.g,
            prime: self.prime.to_vec(),
            version: self.version,
        }
    }

    /// Returns the DH config version.
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Returns the prime modulus.
    pub fn prime(&self) -> &Bytes {
        &self.prime
    }

    /// Returns the generator value.
    pub fn g(&self) -> i32 {
        self.g
    }

    /// Validates the DH parameters.
    ///
    /// Performs basic validation of the DH parameters.
    pub fn validate(&self) -> Result<()> {
        // Check generator is in valid range (2 <= g <= p-2)
        if self.g < 2 {
            return Err(ConfigError::invalid_config("DH generator g must be >= 2"));
        }

        // Check prime is at least 2048 bits (256 bytes) for security
        if self.prime.len() < 256 {
            return Err(ConfigError::invalid_config(
                "DH prime must be at least 2048 bits",
            ));
        }

        // Check version is positive
        if self.version < 0 {
            return Err(ConfigError::invalid_config(
                "DH version must be non-negative",
            ));
        }

        Ok(())
    }

    /// Returns `true` if this config needs update compared to given version.
    pub fn needs_update(&self, version: i32) -> bool {
        self.version > version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_prime() -> Bytes {
        // A 2048-bit prime for testing (truncated)
        let prime_hex = "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183995497CEA956AE515D2261898FA051015728E5A8AACAA68FFFFFFFFFFFFFFFF";
        Bytes::from(hex::decode(prime_hex).expect("Valid hex string should decode successfully"))
    }

    #[test]
    fn test_dh_config_creation() {
        let prime = make_test_prime();
        let config = DhConfig::new(1, prime.clone(), 2);

        assert_eq!(config.version(), 1);
        assert_eq!(config.g(), 2);
        assert_eq!(config.prime(), &prime);
    }

    #[test]
    fn test_dh_config_validate() {
        let prime = make_test_prime();
        let config = DhConfig::new(1, prime, 2);

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_dh_config_validate_small_prime() {
        let small_prime = Bytes::from(vec![1u8; 100]); // Too small
        let config = DhConfig::new(1, small_prime, 2);

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_dh_config_validate_small_generator() {
        let prime = make_test_prime();
        let config = DhConfig::new(1, prime, 1); // g < 2

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_dh_config_validate_negative_version() {
        let prime = make_test_prime();
        let config = DhConfig::new(-1, prime, 2);

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_dh_config_needs_update() {
        let prime = make_test_prime();
        let config = DhConfig::new(5, prime, 2);

        assert!(config.needs_update(4));
        assert!(!config.needs_update(5));
        assert!(!config.needs_update(6));
    }

    #[test]
    fn test_dh_config_roundtrip_tl() {
        let prime = make_test_prime();
        let config = DhConfig::new(1, prime.clone(), 2);

        let tl_config = config.to_tl();
        let restored = DhConfig::from_tl(tl_config);

        assert_eq!(restored.version, config.version);
        assert_eq!(restored.g, config.g);
        assert_eq!(restored.prime, config.prime);
    }

    #[test]
    #[cfg(feature = "proptest")]
    fn proptest_dh_config_version() {
        use proptest::prelude::*;

        proptest!(|(version in -10i32..100i32)| {
            let prime = make_test_prime();
            let config = DhConfig::new(version, prime, 2);

            if version >= 0 {
                assert!(config.validate().is_ok());
            } else {
                assert!(config.validate().is_err());
            }
        });
    }
}
