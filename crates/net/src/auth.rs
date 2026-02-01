// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Authentication data management.
//!
//! This module implements TDLib's authentication data sharing system from
//! `td/telegram/net/AuthDataShared.h`.

use parking_lot::Mutex;
use std::fmt;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use crate::dc::{DcError, DcId};

/// Authentication key state.
///
/// Based on TDLib's AuthKeyState from `td/telegram/net/AuthKeyState.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthKeyState {
    /// No auth key
    #[default]
    Empty,

    /// Loading auth key
    Loading,

    /// Auth key is ready
    Ready,
}

/// Authentication key.
///
/// Represents MTProto authentication key.
#[derive(Debug, Clone, PartialEq)]
pub struct AuthKey {
    /// Key ID
    pub id: u64,

    /// Key data (256 bytes for permanent keys)
    pub key: Vec<u8>,

    /// Expiration timestamp (None for permanent keys)
    pub expires_at: Option<std::time::Instant>,

    /// Whether this key was created using PFS
    pub is_pfs: bool,

    /// Whether this is a CDN key
    pub is_cdn: bool,
}

impl AuthKey {
    /// Creates a new auth key.
    pub fn new(id: u64, key: Vec<u8>) -> Self {
        Self {
            id,
            key,
            expires_at: None,
            is_pfs: false,
            is_cdn: false,
        }
    }

    /// Creates a temporary auth key with expiration.
    pub fn temporary(id: u64, key: Vec<u8>, expires_at: std::time::Instant) -> Self {
        Self {
            id,
            key,
            expires_at: Some(expires_at),
            is_pfs: true,
            is_cdn: false,
        }
    }

    /// Returns `true` if the key is expired.
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map(|exp| exp <= Instant::now())
            .unwrap_or(false)
    }

    /// Returns `true` if this is a temporary key.
    pub fn is_temporary(&self) -> bool {
        self.expires_at.is_some()
    }

    /// Returns the auth key data.
    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }

    /// Returns the length of the key.
    pub fn len(&self) -> usize {
        self.key.len()
    }

    /// Returns `true` if the key is empty.
    pub fn is_empty(&self) -> bool {
        self.key.is_empty()
    }
}

/// Server salt for MTProto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerSalt {
    /// Salt value
    pub salt: i64,

    /// Valid since timestamp
    pub valid_since: i64,
}

impl ServerSalt {
    /// Creates a new server salt.
    pub fn new(salt: i64, valid_since: i64) -> Self {
        Self { salt, valid_since }
    }

    /// Returns `true` if the salt is valid at the given timestamp.
    pub fn is_valid(&self, now: i64) -> bool {
        now >= self.valid_since
    }
}

/// Listener for auth key changes.
pub trait AuthKeyListener: Send + Sync {
    /// Called when the auth key is updated.
    fn on_auth_key_update(&self);
}

/// Shared authentication data.
///
/// Based on TDLib's AuthDataShared from `td/telegram/net/AuthDataShared.h`.
pub struct AuthDataShared {
    /// DC ID
    dc_id: DcId,

    /// Current auth key
    auth_key: Arc<Mutex<Option<AuthKey>>>,

    /// Future server salts
    future_salts: Arc<Mutex<Vec<ServerSalt>>>,

    /// Server time difference
    server_time_difference: Arc<Mutex<f64>>,

    /// Auth key state
    auth_key_state: Arc<Mutex<AuthKeyState>>,

    /// Listeners
    listeners: Arc<Mutex<Vec<Box<dyn AuthKeyListener>>>>,

    /// Session ID
    session_id: AtomicU64,

    /// Sequence number counter
    seq_no: AtomicI32,
}

impl fmt::Debug for AuthDataShared {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthDataShared")
            .field("dc_id", &self.dc_id)
            .field("auth_key_state", &self.auth_key_state.lock())
            .field("has_auth_key", &self.auth_key.lock().is_some())
            .field("salt_count", &self.future_salts.lock().len())
            .field("listener_count", &self.listeners.lock().len())
            .finish()
    }
}

impl AuthDataShared {
    /// Creates new shared auth data.
    pub fn new(dc_id: DcId) -> Self {
        // Generate random session ID
        let session_id = rand::random::<u64>();

        Self {
            dc_id,
            auth_key: Arc::new(Mutex::new(None)),
            future_salts: Arc::new(Mutex::new(Vec::new())),
            server_time_difference: Arc::new(Mutex::new(0.0)),
            auth_key_state: Arc::new(Mutex::new(AuthKeyState::Empty)),
            listeners: Arc::new(Mutex::new(Vec::new())),
            session_id: AtomicU64::new(session_id),
            seq_no: AtomicI32::new(0),
        }
    }

    /// Returns the DC ID.
    pub fn dc_id(&self) -> DcId {
        self.dc_id
    }

    /// Returns the current auth key.
    pub fn get_auth_key(&self) -> Option<AuthKey> {
        self.auth_key.lock().clone()
    }

    /// Sets the auth key.
    pub fn set_auth_key(&self, key: AuthKey) {
        *self.auth_key.lock() = Some(key);
        *self.auth_key_state.lock() = AuthKeyState::Ready;

        // Notify listeners
        let listeners = self.listeners.lock();
        for listener in listeners.iter() {
            listener.on_auth_key_update();
        }
    }

    /// Returns the auth key state.
    pub fn auth_key_state(&self) -> AuthKeyState {
        *self.auth_key_state.lock()
    }

    /// Sets the auth key state.
    pub fn set_auth_key_state(&self, state: AuthKeyState) {
        *self.auth_key_state.lock() = state;
    }

    /// Returns future server salts.
    pub fn get_future_salts(&self) -> Vec<ServerSalt> {
        self.future_salts.lock().clone()
    }

    /// Sets future server salts.
    pub fn set_future_salts(&self, salts: Vec<ServerSalt>) {
        *self.future_salts.lock() = salts;
    }

    /// Returns the server time difference.
    pub fn server_time_difference(&self) -> f64 {
        *self.server_time_difference.lock()
    }

    /// Updates the server time difference.
    pub fn update_server_time_difference(&self, diff: f64, _force: bool) {
        *self.server_time_difference.lock() = diff;
    }

    /// Adds a listener for auth key changes.
    pub fn add_listener(&self, listener: Box<dyn AuthKeyListener>) {
        self.listeners.lock().push(listener);
    }

    /// Clears the auth key.
    pub fn clear(&self) {
        *self.auth_key.lock() = None;
        *self.auth_key_state.lock() = AuthKeyState::Empty;
    }

    /// Sets the server salt.
    pub fn set_server_salt(&self, salt: u64) {
        // Convert u64 to ServerSalt with current timestamp
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        let server_salt = ServerSalt::new(salt as i64, now);
        self.future_salts.lock().push(server_salt);
    }

    /// Gets the current server salt.
    pub fn get_server_salt(&self) -> Option<u64> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        self.future_salts
            .lock()
            .iter()
            .find(|s| s.is_valid(now))
            .map(|s| s.salt as u64)
    }

    /// Returns the server salt for packet serialization.
    ///
    /// Returns a default salt if none is set.
    pub fn server_salt(&self) -> u64 {
        self.get_server_salt().unwrap_or(0)
    }

    /// Returns the session ID.
    pub fn session_id(&self) -> u64 {
        self.session_id.load(Ordering::Relaxed)
    }

    /// Generates and returns the next sequence number.
    ///
    /// # Arguments
    ///
    /// * `needs_ack` - Whether this message needs acknowledgment (content-related)
    ///
    /// # Returns
    ///
    /// The next sequence number to use.
    pub fn next_seq_no(&self, needs_ack: bool) -> i32 {
        if needs_ack {
            // Content-related messages increment by 2
            self.seq_no.fetch_add(2, Ordering::Relaxed)
        } else {
            // Service messages use current value
            self.seq_no.load(Ordering::Relaxed)
        }
    }

    /// Gets the auth key data as a reference.
    ///
    /// Returns None if no auth key is set.
    pub fn get_auth_key_data(&self) -> Option<Vec<u8>> {
        self.auth_key.lock().as_ref().map(|k| k.key.clone())
    }
}

impl Clone for AuthDataShared {
    fn clone(&self) -> Self {
        Self {
            dc_id: self.dc_id,
            auth_key: self.auth_key.clone(),
            future_salts: self.future_salts.clone(),
            server_time_difference: self.server_time_difference.clone(),
            auth_key_state: self.auth_key_state.clone(),
            listeners: Arc::new(Mutex::new(Vec::new())),
            session_id: AtomicU64::new(self.session_id.load(Ordering::Relaxed)),
            seq_no: AtomicI32::new(self.seq_no.load(Ordering::Relaxed)),
        }
    }
}

/// Factory for creating AuthDataShared instances.
pub struct AuthDataSharedFactory;

impl AuthDataSharedFactory {
    /// Creates shared auth data for a DC.
    pub fn create(dc_id: DcId) -> Result<AuthDataShared, DcError> {
        if !dc_id.is_exact() {
            return Err(DcError::InvalidId(dc_id.get_value()));
        }

        Ok(AuthDataShared::new(dc_id))
    }

    /// Gets auth key for a DC (from storage or creates new).
    pub fn get_auth_key_for_dc(_dc_id: DcId) -> Option<AuthKey> {
        // In a real implementation, this would load from storage
        // For now, return None
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener {
        called: Arc<Mutex<bool>>,
    }

    impl TestListener {
        fn new() -> Self {
            Self {
                called: Arc::new(Mutex::new(false)),
            }
        }

        fn was_called(&self) -> bool {
            *self.called.lock()
        }
    }

    impl Clone for TestListener {
        fn clone(&self) -> Self {
            Self {
                called: Arc::clone(&self.called),
            }
        }
    }

    impl AuthKeyListener for TestListener {
        fn on_auth_key_update(&self) {
            *self.called.lock() = true;
        }
    }

    #[test]
    fn test_auth_key() {
        let key = AuthKey::new(123, vec![1, 2, 3, 4]);
        assert_eq!(key.id, 123);
        assert_eq!(key.len(), 4);
        assert!(!key.is_temporary());
        assert!(!key.is_expired());
    }

    #[test]
    fn test_temporary_auth_key() {
        let expires = std::time::Instant::now() + std::time::Duration::from_secs(60);
        let key = AuthKey::temporary(123, vec![1, 2, 3, 4], expires);
        assert!(key.is_temporary());
        assert!(!key.is_expired());
    }

    #[test]
    fn test_server_salt() {
        let salt = ServerSalt::new(12345, 1000);
        assert_eq!(salt.salt, 12345);
        assert!(salt.is_valid(1000));
        assert!(salt.is_valid(2000));
        assert!(!salt.is_valid(999));
    }

    #[test]
    fn test_auth_data_shared() {
        let auth_data = AuthDataShared::new(DcId::internal(2));
        assert_eq!(auth_data.dc_id(), DcId::internal(2));
        assert_eq!(auth_data.auth_key_state(), AuthKeyState::Empty);

        let key = AuthKey::new(123, vec![1, 2, 3, 4]);
        auth_data.set_auth_key(key.clone());

        assert_eq!(auth_data.auth_key_state(), AuthKeyState::Ready);
        assert_eq!(auth_data.get_auth_key(), Some(key));
    }

    #[test]
    fn test_auth_data_shared_listener() {
        let auth_data = AuthDataShared::new(DcId::internal(2));
        let listener = TestListener::new();

        auth_data.add_listener(Box::new(listener.clone()));

        let key = AuthKey::new(123, vec![1, 2, 3, 4]);
        auth_data.set_auth_key(key);

        assert!(listener.was_called());
    }

    #[test]
    fn test_auth_data_shared_salts() {
        let auth_data = AuthDataShared::new(DcId::internal(2));

        let salts = vec![ServerSalt::new(1, 1000), ServerSalt::new(2, 2000)];

        auth_data.set_future_salts(salts.clone());

        assert_eq!(auth_data.get_future_salts(), salts);
    }

    #[test]
    fn test_auth_data_shared_time() {
        let auth_data = AuthDataShared::new(DcId::internal(2));

        assert_eq!(auth_data.server_time_difference(), 0.0);

        auth_data.update_server_time_difference(10.5, false);

        assert_eq!(auth_data.server_time_difference(), 10.5);
    }

    #[test]
    fn test_auth_data_shared_clear() {
        let auth_data = AuthDataShared::new(DcId::internal(2));

        let key = AuthKey::new(123, vec![1, 2, 3, 4]);
        auth_data.set_auth_key(key);

        assert_eq!(auth_data.auth_key_state(), AuthKeyState::Ready);

        auth_data.clear();

        assert_eq!(auth_data.auth_key_state(), AuthKeyState::Empty);
        assert!(auth_data.get_auth_key().is_none());
    }

    #[test]
    fn test_factory_create() {
        let auth_data = match AuthDataSharedFactory::create(DcId::internal(2)) {
            Ok(data) => data,
            Err(_) => panic!("Expected Ok auth_data"),
        };
        assert_eq!(auth_data.dc_id(), DcId::internal(2));

        let result = AuthDataSharedFactory::create(DcId::invalid());
        assert!(result.is_err());
    }
}
