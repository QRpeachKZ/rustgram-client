// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Public RSA key sharing for main and CDN DCs.
//!
//! This module implements TDLib's public RSA key management system from:
//! - `td/telegram/net/PublicRsaKeySharedMain.h`
//! - `td/telegram/net/PublicRsaKeySharedCdn.h`
//! - `td/telegram/net/PublicRsaKeyWatchdog.h`

use parking_lot::RwLock;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use thiserror::Error;

use crate::crypto::sha256;
use crate::dc::DcId;

/// Error types for RSA key operations.
#[derive(Debug, Error)]
pub enum RsaKeyError {
    /// No key found for the given fingerprint
    #[error("No RSA key found for fingerprint: {0}")]
    KeyNotFound(i64),

    /// Invalid RSA key format
    #[error("Invalid RSA key format: {0}")]
    InvalidKeyFormat(String),

    /// RSA key operation failed
    #[error("RSA key operation failed: {0}")]
    OperationFailed(String),

    /// Invalid DC ID
    #[error("Invalid DC ID: {0:?}")]
    InvalidDcId(DcId),
}

/// RSA public key with fingerprint.
///
/// Based on TDLib's RsaKey from `td/mtproto/RSA.h`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsaKey {
    /// The RSA public key in PEM format
    pub pem: String,

    /// The fingerprint of the key
    pub fingerprint: i64,

    /// Key size in bits
    pub bits: usize,
}

impl RsaKey {
    /// Creates a new RSA key.
    pub fn new(pem: String, fingerprint: i64, bits: usize) -> Self {
        Self {
            pem,
            fingerprint,
            bits,
        }
    }

    /// Computes fingerprint from a PEM-encoded key.
    ///
    /// Uses SHA-256 hash of the key data (Telegram's method).
    pub fn compute_fingerprint(pem: &str) -> i64 {
        let hash = sha256(pem.as_bytes());

        // Take first 8 bytes and convert to i64 (little-endian)
        let bytes: [u8; 8] = hash[0..8]
            .try_into()
            .unwrap_or([0u8; 8]);

        i64::from_le_bytes(bytes)
    }

    /// Returns the key size.
    pub fn size(&self) -> usize {
        self.bits / 8
    }
}

/// Public RSA key interface trait.
///
/// Based on TDLib's PublicRsaKeyInterface from `td/mtproto/RSA.h`.
pub trait PublicRsaKeyInterface: Send + Sync {
    /// Gets an RSA key matching one of the fingerprints.
    fn get_rsa_key(&self, fingerprints: &[i64]) -> Result<RsaKey, RsaKeyError>;

    /// Drops all cached keys.
    fn drop_keys(&self);
}

/// Shared RSA keys for main DCs.
///
/// Based on TDLib's PublicRsaKeySharedMain from `td/telegram/net/PublicRsaKeySharedMain.h`.
#[derive(Debug, Clone)]
pub struct PublicRsaKeySharedMain {
    keys: Arc<RwLock<Vec<RsaKey>>>,
    is_test: bool,
}

impl PublicRsaKeySharedMain {
    /// Creates a new shared main RSA key holder.
    pub fn new(keys: Vec<RsaKey>, is_test: bool) -> Self {
        Self {
            keys: Arc::new(RwLock::new(keys)),
            is_test,
        }
    }

    /// Creates keys for production environment.
    pub fn create_production() -> Self {
        // In production, these would be loaded from Telegram's known keys
        // For now, use empty list that will be populated dynamically
        Self::new(Vec::new(), false)
    }

    /// Creates keys for test environment.
    pub fn create_test() -> Self {
        Self::new(Vec::new(), true)
    }

    /// Adds an RSA key to the collection.
    pub fn add_key(&self, key: RsaKey) {
        self.keys.write().push(key);
    }

    /// Returns `true` if this is for test environment.
    pub fn is_test(&self) -> bool {
        self.is_test
    }
}

impl Default for PublicRsaKeySharedMain {
    fn default() -> Self {
        Self::create_production()
    }
}

impl PublicRsaKeyInterface for PublicRsaKeySharedMain {
    fn get_rsa_key(&self, fingerprints: &[i64]) -> Result<RsaKey, RsaKeyError> {
        let keys = self.keys.read();

        for &fp in fingerprints {
            if let Some(key) = keys.iter().find(|k| k.fingerprint == fp) {
                return Ok(key.clone());
            }
        }

        Err(RsaKeyError::KeyNotFound(
            *fingerprints.first().unwrap_or(&0),
        ))
    }

    fn drop_keys(&self) {
        self.keys.write().clear();
    }
}

/// Listener for CDN RSA key changes.
pub trait RsaKeyListener: Send + Sync {
    /// Called when RSA keys are updated.
    fn notify(&self) -> bool;
}

/// Shared RSA keys for CDN DCs.
///
/// Based on TDLib's PublicRsaKeySharedCdn from `td/telegram/net/PublicRsaKeySharedCdn.h`.
pub struct PublicRsaKeySharedCdn {
    dc_id: DcId,
    keys: Arc<RwLock<Vec<RsaKey>>>,
    listeners: Arc<RwLock<Vec<Box<dyn RsaKeyListener>>>>,
}

impl fmt::Debug for PublicRsaKeySharedCdn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PublicRsaKeySharedCdn")
            .field("dc_id", &self.dc_id)
            .field("key_count", &self.keys.read().len())
            .field("listener_count", &self.listeners.read().len())
            .finish()
    }
}

impl PublicRsaKeySharedCdn {
    /// Creates a new shared CDN RSA key holder.
    pub fn new(dc_id: DcId) -> Result<Self, RsaKeyError> {
        if !dc_id.is_exact() {
            return Err(RsaKeyError::InvalidDcId(dc_id));
        }

        Ok(Self {
            dc_id,
            keys: Arc::new(RwLock::new(Vec::new())),
            listeners: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Returns the DC ID.
    pub fn dc_id(&self) -> DcId {
        self.dc_id
    }

    /// Adds an RSA key to this CDN.
    pub fn add_rsa(&self, key: RsaKey) {
        {
            let mut keys = self.keys.write();
            keys.push(key);
        }

        self.notify();
    }

    /// Returns `true` if this CDN has keys.
    pub fn has_keys(&self) -> bool {
        !self.keys.read().is_empty()
    }

    /// Adds a listener for key changes.
    pub fn add_listener(&self, listener: Box<dyn RsaKeyListener>) {
        self.listeners.write().push(listener);
    }

    /// Notifies all listeners.
    fn notify(&self) {
        let listeners = self.listeners.read();
        for listener in listeners.iter() {
            listener.notify();
        }
    }

    /// Gets an RSA key without locking (internal use).
    fn get_rsa_key_unsafe(&self, fingerprint: i64) -> Option<RsaKey> {
        let keys = self.keys.read();
        keys.iter()
            .find(|k| k.fingerprint == fingerprint)
            .cloned()
    }
}

impl PublicRsaKeyInterface for PublicRsaKeySharedCdn {
    fn get_rsa_key(&self, fingerprints: &[i64]) -> Result<RsaKey, RsaKeyError> {
        let keys = self.keys.read();

        for &fp in fingerprints {
            if let Some(key) = keys.iter().find(|k| k.fingerprint == fp) {
                return Ok(key.clone());
            }
        }

        Err(RsaKeyError::KeyNotFound(
            *fingerprints.first().unwrap_or(&0),
        ))
    }

    fn drop_keys(&self) {
        self.keys.write().clear();
    }
}

/// Watchdog for monitoring RSA key usage.
///
/// Based on TDLib's PublicRsaKeyWatchdog from `td/telegram/net/PublicRsaKeyWatchdog.h`.
#[derive(Debug, Default)]
pub struct PublicRsaKeyWatchdog {
    /// Tracks which DCs have active keys
    dc_keys: Arc<RwLock<HashMap<i32, bool>>>,
}

impl PublicRsaKeyWatchdog {
    /// Creates a new watchdog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a DC as having an active key.
    pub fn register_dc(&self, dc_id: DcId) {
        if dc_id.is_exact() {
            let mut keys = self.dc_keys.write();
            keys.insert(dc_id.get_raw_id(), true);
        }
    }

    /// Unregisters a DC.
    pub fn unregister_dc(&self, dc_id: DcId) {
        if dc_id.is_exact() {
            let mut keys = self.dc_keys.write();
            keys.remove(&dc_id.get_raw_id());
        }
    }

    /// Returns `true` if a DC has an active key.
    pub fn has_key_for_dc(&self, dc_id: DcId) -> bool {
        if !dc_id.is_exact() {
            return false;
        }

        let keys = self.dc_keys.read();
        keys.get(&dc_id.get_raw_id()).copied().unwrap_or(false)
    }

    /// Returns all DC IDs with active keys.
    pub fn active_dcs(&self) -> Vec<i32> {
        let keys = self.dc_keys.read();
        keys.keys().copied().collect()
    }
}

/// Combined RSA key manager for all DC types.
///
/// Provides a unified interface for accessing both main and CDN RSA keys.
#[derive(Debug)]
pub struct RsaKeyManager {
    main_keys: Arc<PublicRsaKeySharedMain>,
    cdn_keys: Arc<RwLock<HashMap<i32, Arc<PublicRsaKeySharedCdn>>>>,
    watchdog: Arc<PublicRsaKeyWatchdog>,
}

impl RsaKeyManager {
    /// Creates a new RSA key manager.
    pub fn new(is_test: bool) -> Self {
        Self {
            main_keys: Arc::new(if is_test {
                PublicRsaKeySharedMain::create_test()
            } else {
                PublicRsaKeySharedMain::create_production()
            }),
            cdn_keys: Arc::new(RwLock::new(HashMap::new())),
            watchdog: Arc::new(PublicRsaKeyWatchdog::new()),
        }
    }

    /// Returns the main DC keys.
    pub fn main_keys(&self) -> Arc<PublicRsaKeySharedMain> {
        Arc::clone(&self.main_keys)
    }

    /// Gets or creates CDN keys for a DC.
    pub fn get_cdn_keys(&self, dc_id: DcId) -> Result<Arc<PublicRsaKeySharedCdn>, RsaKeyError> {
        if !dc_id.is_exact() {
            return Err(RsaKeyError::InvalidDcId(dc_id));
        }

        let mut keys = self.cdn_keys.write();

        if let Some(cdn_keys) = keys.get(&dc_id.get_raw_id()) {
            Ok(Arc::clone(cdn_keys))
        } else {
            let cdn_keys = Arc::new(PublicRsaKeySharedCdn::new(dc_id)?);
            keys.insert(dc_id.get_raw_id(), Arc::clone(&cdn_keys));
            Ok(cdn_keys)
        }
    }

    /// Returns the watchdog.
    pub fn watchdog(&self) -> Arc<PublicRsaKeyWatchdog> {
        Arc::clone(&self.watchdog)
    }

    /// Gets an RSA key for a DC (main or CDN).
    pub fn get_rsa_key(
        &self,
        dc_id: DcId,
        fingerprints: &[i64],
    ) -> Result<RsaKey, RsaKeyError> {
        if dc_id.is_internal() {
            // Use main keys
            self.main_keys.get_rsa_key(fingerprints)
        } else {
            // Use CDN keys
            let cdn_keys = self.get_cdn_keys(dc_id)?;
            cdn_keys.get_rsa_key(fingerprints)
        }
    }

    /// Adds a main RSA key.
    pub fn add_main_key(&self, key: RsaKey) {
        self.main_keys.add_key(key);
    }

    /// Adds a CDN RSA key.
    pub fn add_cdn_key(&self, dc_id: DcId, key: RsaKey) -> Result<(), RsaKeyError> {
        let cdn_keys = self.get_cdn_keys(dc_id)?;
        cdn_keys.add_rsa(key);
        self.watchdog.register_dc(dc_id);
        Ok(())
    }

    /// Drops all keys.
    pub fn drop_all_keys(&self) {
        self.main_keys.drop_keys();
        self.cdn_keys.write().clear();

        // Clear watchdog tracking
        let dcs = self.watchdog.active_dcs();
        for dc_id_value in dcs {
            self.watchdog.unregister_dc(DcId::internal(dc_id_value));
            self.watchdog.unregister_dc(DcId::external(dc_id_value));
        }
    }
}

impl Default for RsaKeyManager {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener {
        notified: Arc<RwLock<bool>>,
    }

    impl TestListener {
        fn new() -> Self {
            Self {
                notified: Arc::new(RwLock::new(false)),
            }
        }

        fn was_notified(&self) -> bool {
            *self.notified.read()
        }
    }

    impl Clone for TestListener {
        fn clone(&self) -> Self {
            Self {
                notified: Arc::clone(&self.notified),
            }
        }
    }

    impl RsaKeyListener for TestListener {
        fn notify(&self) -> bool {
            *self.notified.write() = true;
            true
        }
    }

    #[test]
    fn test_rsa_key_fingerprint() {
        let pem = "-----BEGIN PUBLIC KEY-----\ntest key data\n-----END PUBLIC KEY-----";
        let fp = RsaKey::compute_fingerprint(pem);

        // Same input should give same fingerprint
        let fp2 = RsaKey::compute_fingerprint(pem);
        assert_eq!(fp, fp2);

        // Different input should give different fingerprint
        let fp3 = RsaKey::compute_fingerprint("different key");
        assert_ne!(fp, fp3);
    }

    #[test]
    fn test_rsa_key() {
        let key = RsaKey::new("test.pem".to_string(), 12345, 2048);

        assert_eq!(key.fingerprint, 12345);
        assert_eq!(key.bits, 2048);
        assert_eq!(key.size(), 256);
    }

    #[test]
    fn test_main_shared_keys() {
        let main = PublicRsaKeySharedMain::new(vec![], false);

        let key1 = RsaKey::new("key1.pem".to_string(), 111, 2048);
        let key2 = RsaKey::new("key2.pem".to_string(), 222, 2048);

        main.add_key(key1.clone());
        main.add_key(key2.clone());

        // Should find existing key
        let found = main.get_rsa_key(&[111]).unwrap();
        assert_eq!(found.fingerprint, 111);

        // Should find first matching
        let found = main.get_rsa_key(&[999, 222]).unwrap();
        assert_eq!(found.fingerprint, 222);

        // Should fail if none match
        let result = main.get_rsa_key(&[999]);
        assert!(result.is_err());

        // Drop keys
        main.drop_keys();
        let result = main.get_rsa_key(&[111]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cdn_shared_keys() {
        let dc_id = DcId::external(2);
        let cdn = PublicRsaKeySharedCdn::new(dc_id).unwrap();

        assert_eq!(cdn.dc_id(), dc_id);
        assert!(!cdn.has_keys());

        let key = RsaKey::new("cdn_key.pem".to_string(), 333, 2048);
        cdn.add_rsa(key.clone());

        assert!(cdn.has_keys());

        let found = cdn.get_rsa_key(&[333]).unwrap();
        assert_eq!(found.fingerprint, 333);

        // Test listener
        let listener = TestListener::new();
        cdn.add_listener(Box::new(listener.clone()));

        let key2 = RsaKey::new("cdn_key2.pem".to_string(), 444, 2048);
        cdn.add_rsa(key2);

        assert!(listener.was_notified());
    }

    #[test]
    fn test_cdn_shared_keys_invalid_dc() {
        let result = PublicRsaKeySharedCdn::new(DcId::invalid());
        assert!(result.is_err());
    }

    #[test]
    fn test_watchdog() {
        let watchdog = PublicRsaKeyWatchdog::new();

        let dc1 = DcId::internal(1);
        let dc2 = DcId::internal(2);

        assert!(!watchdog.has_key_for_dc(dc1));
        assert!(!watchdog.has_key_for_dc(dc2));

        watchdog.register_dc(dc1);
        assert!(watchdog.has_key_for_dc(dc1));
        assert!(!watchdog.has_key_for_dc(dc2));

        watchdog.register_dc(dc2);
        assert!(watchdog.has_key_for_dc(dc2));

        let dcs = watchdog.active_dcs();
        assert_eq!(dcs.len(), 2);
        assert!(dcs.contains(&1));
        assert!(dcs.contains(&2));

        watchdog.unregister_dc(dc1);
        assert!(!watchdog.has_key_for_dc(dc1));
        assert!(watchdog.has_key_for_dc(dc2));
    }

    #[test]
    fn test_rsa_key_manager() {
        let manager = RsaKeyManager::new(false);

        // Add main key
        let main_key = RsaKey::new("main.pem".to_string(), 100, 2048);
        manager.add_main_key(main_key.clone());

        // Get main key
        let found = manager.get_rsa_key(DcId::internal(1), &[100]).unwrap();
        assert_eq!(found.fingerprint, 100);

        // Add CDN key
        let dc_cdn = DcId::external(3);
        let cdn_key = RsaKey::new("cdn.pem".to_string(), 200, 2048);
        manager.add_cdn_key(dc_cdn, cdn_key).unwrap();

        // Get CDN key
        let found = manager.get_rsa_key(dc_cdn, &[200]).unwrap();
        assert_eq!(found.fingerprint, 200);

        // Verify watchdog was updated
        assert!(manager.watchdog().has_key_for_dc(dc_cdn));

        // Drop all keys
        manager.drop_all_keys();
        assert!(!manager.watchdog().has_key_for_dc(dc_cdn));
    }

    #[test]
    fn test_rsa_key_manager_invalid_dc() {
        let manager = RsaKeyManager::new(false);

        let result = manager.get_cdn_keys(DcId::invalid());
        assert!(result.is_err());

        let result = manager.add_cdn_key(DcId::invalid(), RsaKey::new("x".to_string(), 0, 0));
        assert!(result.is_err());
    }
}
