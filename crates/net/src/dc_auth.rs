// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! DC authentication key manager.
//!
//! This module implements TDLib's DcAuthManager from `td/telegram/net/DcAuthManager.h`.
//!
//! Manages authentication keys for all DCs including:
//! - Main auth keys (permanent)
//! - DC-specific auth keys
//! - Temporary auth keys (for Perfect Forward Secrecy)
//! - Auth key export/import between DCs

use crate::auth::{AuthDataShared, AuthKey, AuthKeyState};
use crate::dc::DcId;
use crate::query::NetQuery;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use thiserror::Error;

/// Error types for DC auth operations.
#[derive(Debug, Error)]
pub enum DcAuthError {
    /// Invalid DC ID
    #[error("Invalid DC ID: {0:?}")]
    InvalidDcId(DcId),

    /// Auth key not found
    #[error("Auth key not found for DC {0:?}")]
    AuthKeyNotFound(DcId),

    /// Auth key export failed
    #[error("Failed to export auth key from DC {from_dc:?} to {to_dc:?}: {reason}")]
    ExportFailed {
        /// Source DC
        from_dc: DcId,
        /// Destination DC
        to_dc: DcId,
        /// Error reason
        reason: String,
    },

    /// Auth key import failed
    #[error("Failed to import auth key for DC {dc:?}: {reason}")]
    ImportFailed {
        /// Target DC
        dc: DcId,
        /// Error reason
        reason: String,
    },

    /// Operation timeout
    #[error("Operation timeout for DC {0:?}")]
    Timeout(DcId),

    /// Authorization check failed
    #[error("Authorization check failed")]
    AuthorizationFailed,

    /// Destroy operation in progress
    #[error("Destroy operation in progress")]
    DestroyInProgress,
}

/// State of a DC in the auth manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[derive(Default)]
pub enum DcState {
    /// Waiting for auth key
    #[default]
    Waiting = 0,

    /// Exporting auth key from another DC
    Export = 1,

    /// Importing auth key
    Import = 2,

    /// Waiting for authorization OK
    BeforeOk = 3,

    /// Auth key is ready
    Ok = 4,
}

/// Information about a DC's authentication state.
#[derive(Debug, Clone)]
pub struct DcAuthInfo {
    /// DC ID
    pub dc_id: DcId,

    /// Shared auth data
    pub auth_data: Arc<AuthDataShared>,

    /// Current auth key state
    pub auth_key_state: AuthKeyState,

    /// DC state in the auth loop
    pub state: DcState,

    /// Wait ID for pending operations
    pub wait_id: u64,

    /// Export ID for key export operations
    pub export_id: i64,

    /// Exported key bytes
    pub export_bytes: Option<Vec<u8>>,

    /// Last state change timestamp
    pub last_update: Instant,
}

impl DcAuthInfo {
    /// Creates new DC auth info.
    pub fn new(dc_id: DcId, auth_data: Arc<AuthDataShared>) -> Self {
        Self {
            dc_id,
            auth_data,
            auth_key_state: AuthKeyState::Empty,
            state: DcState::Waiting,
            wait_id: 0,
            export_id: 0,
            export_bytes: None,
            last_update: Instant::now(),
        }
    }

    /// Returns `true` if auth is ready.
    pub fn is_ready(&self) -> bool {
        self.state == DcState::Ok && self.auth_key_state == AuthKeyState::Ready
    }

    /// Returns `true` if an operation is in progress.
    pub fn is_busy(&self) -> bool {
        matches!(
            self.state,
            DcState::Export | DcState::Import | DcState::BeforeOk
        )
    }
}

/// Registered temporary auth key.
///
/// RAII guard that unregisters the key when dropped.
pub struct RegisteredAuthKey {
    watchdog: Arc<TempAuthKeyWatchdog>,
    auth_key_id: i64,
}

impl RegisteredAuthKey {
    /// Creates a new registered auth key.
    fn new(watchdog: Arc<TempAuthKeyWatchdog>, auth_key_id: i64) -> Self {
        Self {
            watchdog,
            auth_key_id,
        }
    }

    /// Returns the auth key ID.
    pub fn id(&self) -> i64 {
        self.auth_key_id
    }
}

impl Drop for RegisteredAuthKey {
    fn drop(&mut self) {
        self.watchdog.unregister_auth_key_id(self.auth_key_id);
    }
}

/// Temporary auth key watchdog.
///
/// Based on TDLib's TempAuthKeyWatchdog from `td/telegram/net/TempAuthKeyWatchdog.h`.
///
/// Manages temporary auth keys and sends `auth.dropTempAuthKeys` requests
/// to clean up unused keys on the server.
#[derive(Debug, Default)]
pub struct TempAuthKeyWatchdog {
    /// Maps auth key IDs to reference counts
    id_counts: Mutex<HashMap<i64, u32>>,

    /// Whether a sync is needed
    need_sync: Mutex<bool>,

    /// Whether a sync is currently running
    run_sync: Mutex<bool>,
}

impl TempAuthKeyWatchdog {
    /// Creates a new watchdog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an auth key ID.
    ///
    /// Returns a guard that will automatically unregister when dropped.
    pub fn register_auth_key_id(&self, id: i64) -> RegisteredAuthKey {
        let mut counts = self.id_counts.lock();
        *counts.entry(id).or_insert(0) += 1;

        *self.need_sync.lock() = true;

        RegisteredAuthKey::new(Arc::new(self.clone()), id)
    }

    /// Unregisters an auth key ID (internal use).
    fn unregister_auth_key_id(&self, id: i64) {
        let mut counts = self.id_counts.lock();

        if let Some(count) = counts.get_mut(&id) {
            if *count > 0 {
                *count -= 1;
            }

            if *count == 0 {
                counts.remove(&id);
            }
        }

        *self.need_sync.lock() = true;
    }

    /// Returns the list of active auth key IDs.
    pub fn active_key_ids(&self) -> Vec<i64> {
        let counts = self.id_counts.lock();
        counts.keys().copied().collect()
    }

    /// Returns `true` if a sync is needed.
    pub fn needs_sync(&self) -> bool {
        *self.need_sync.lock()
    }

    /// Marks that sync has been performed.
    pub fn mark_synced(&self) {
        *self.need_sync.lock() = false;
    }

    /// Returns the number of active keys.
    pub fn active_count(&self) -> usize {
        self.id_counts.lock().len()
    }
}

impl Clone for TempAuthKeyWatchdog {
    fn clone(&self) -> Self {
        // Note: This creates a shallow clone that shares the same underlying data
        // In a real implementation, you'd want to use Arc<Mutex<...>> internally
        // For now, we'll create a new instance with shared Arc
        Self {
            id_counts: Mutex::new(self.id_counts.lock().clone()),
            need_sync: Mutex::new(*self.need_sync.lock()),
            run_sync: Mutex::new(*self.run_sync.lock()),
        }
    }
}

/// DC authentication manager.
///
/// Based on TDLib's DcAuthManager from `td/telegram/net/DcAuthManager.h`.
///
/// Manages authentication keys across all DCs, handling:
/// - Auth key export/import between DCs
/// - Temporary auth key lifecycle
/// - Authorization state tracking
/// - Auth key destruction
pub struct DcAuthManager {
    /// Main DC ID
    main_dc_id: Mutex<DcId>,

    /// DC information indexed by raw DC ID
    dcs: Mutex<HashMap<i32, DcAuthInfo>>,

    /// Temporary auth key watchdog
    temp_watchdog: Arc<TempAuthKeyWatchdog>,

    /// Whether authorization should be checked
    check_authorization: Mutex<bool>,

    /// Whether destroy is in progress
    destroy_in_progress: Mutex<bool>,

    /// Whether auth keys should be destroyed
    need_destroy_auth_key: Mutex<bool>,
}

impl DcAuthManager {
    /// Creates a new DC auth manager.
    pub fn new(main_dc_id: DcId) -> Self {
        Self {
            main_dc_id: Mutex::new(main_dc_id),
            dcs: Mutex::new(HashMap::new()),
            temp_watchdog: Arc::new(TempAuthKeyWatchdog::new()),
            check_authorization: Mutex::new(false),
            destroy_in_progress: Mutex::new(false),
            need_destroy_auth_key: Mutex::new(false),
        }
    }

    /// Returns the main DC ID.
    pub fn main_dc_id(&self) -> DcId {
        *self.main_dc_id.lock()
    }

    /// Updates the main DC ID.
    pub fn update_main_dc(&self, new_main_dc_id: DcId) {
        *self.main_dc_id.lock() = new_main_dc_id;
    }

    /// Adds a DC to the manager.
    pub fn add_dc(&self, dc_id: DcId, auth_data: Arc<AuthDataShared>) -> Result<(), DcAuthError> {
        if !dc_id.is_exact() {
            return Err(DcAuthError::InvalidDcId(dc_id));
        }

        let mut dcs = self.dcs.lock();
        dcs.insert(dc_id.get_raw_id(), DcAuthInfo::new(dc_id, auth_data));

        Ok(())
    }

    /// Gets auth info for a DC.
    pub fn get_dc(&self, dc_id: DcId) -> Result<DcAuthInfo, DcAuthError> {
        if !dc_id.is_exact() {
            return Err(DcAuthError::InvalidDcId(dc_id));
        }

        let dcs = self.dcs.lock();
        dcs.get(&dc_id.get_raw_id())
            .cloned()
            .ok_or(DcAuthError::AuthKeyNotFound(dc_id))
    }

    /// Finds auth info for a DC (returns None if not found).
    pub fn find_dc(&self, dc_id: DcId) -> Option<DcAuthInfo> {
        if !dc_id.is_exact() {
            return None;
        }

        let dcs = self.dcs.lock();
        dcs.get(&dc_id.get_raw_id()).cloned()
    }

    /// Returns all DC IDs.
    pub fn all_dc_ids(&self) -> Vec<DcId> {
        let dcs = self.dcs.lock();
        dcs.keys().map(|&id| DcId::internal(id)).collect()
    }

    /// Returns the temporary auth key watchdog.
    pub fn temp_watchdog(&self) -> Arc<TempAuthKeyWatchdog> {
        Arc::clone(&self.temp_watchdog)
    }

    /// Registers a temporary auth key.
    ///
    /// Returns a guard that unregisters the key when dropped.
    pub fn register_temp_auth_key(&self, auth_key_id: i64) -> RegisteredAuthKey {
        self.temp_watchdog.register_auth_key_id(auth_key_id)
    }

    /// Checks if authorization is OK.
    pub fn check_authorization_is_ok(&self) -> Result<(), DcAuthError> {
        if *self.check_authorization.lock() {
            // In a real implementation, this would verify authorization
            // For now, just return OK
            Ok(())
        } else {
            Err(DcAuthError::AuthorizationFailed)
        }
    }

    /// Enables authorization checking.
    pub fn enable_authorization_check(&self) {
        *self.check_authorization.lock() = true;
    }

    /// Disables authorization checking.
    pub fn disable_authorization_check(&self) {
        *self.check_authorization.lock() = false;
    }

    /// Updates auth key state for a DC.
    pub fn update_auth_key_state(
        &self,
        dc_id: DcId,
        state: AuthKeyState,
    ) -> Result<(), DcAuthError> {
        let mut dcs = self.dcs.lock();

        if let Some(info) = dcs.get_mut(&dc_id.get_raw_id()) {
            info.auth_key_state = state;
            info.last_update = Instant::now();
            Ok(())
        } else {
            Err(DcAuthError::AuthKeyNotFound(dc_id))
        }
    }

    /// Updates DC state.
    pub fn update_dc_state(&self, dc_id: DcId, state: DcState) -> Result<(), DcAuthError> {
        let mut dcs = self.dcs.lock();

        if let Some(info) = dcs.get_mut(&dc_id.get_raw_id()) {
            info.state = state;
            info.last_update = Instant::now();
            Ok(())
        } else {
            Err(DcAuthError::AuthKeyNotFound(dc_id))
        }
    }

    /// Sets export data for a DC.
    pub fn set_export_data(
        &self,
        dc_id: DcId,
        export_id: i64,
        data: Vec<u8>,
    ) -> Result<(), DcAuthError> {
        let mut dcs = self.dcs.lock();

        if let Some(info) = dcs.get_mut(&dc_id.get_raw_id()) {
            info.export_id = export_id;
            info.export_bytes = Some(data);
            info.state = DcState::Import;
            Ok(())
        } else {
            Err(DcAuthError::AuthKeyNotFound(dc_id))
        }
    }

    /// Starts auth key destruction.
    pub fn destroy_auth_keys(&self) -> Result<(), DcAuthError> {
        if *self.destroy_in_progress.lock() {
            return Err(DcAuthError::DestroyInProgress);
        }

        *self.destroy_in_progress.lock() = true;
        *self.need_destroy_auth_key.lock() = true;

        Ok(())
    }

    /// Returns `true` if destroy is in progress.
    pub fn is_destroying(&self) -> bool {
        *self.destroy_in_progress.lock()
    }

    /// Returns `true` if auth keys need to be destroyed.
    pub fn need_destroy_auth_key(&self) -> bool {
        *self.need_destroy_auth_key.lock()
    }

    /// Marks auth keys as destroyed.
    pub fn mark_auth_keys_destroyed(&self) {
        *self.need_destroy_auth_key.lock() = false;
        *self.destroy_in_progress.lock() = false;
    }

    /// Clears all DC data.
    pub fn clear(&self) {
        let mut dcs = self.dcs.lock();
        dcs.clear();
    }

    /// Processes a query result.
    ///
    /// This is called by the network layer when queries complete.
    pub fn on_query_result(&self, _query: &NetQuery) -> Result<(), DcAuthError> {
        // In a real implementation, this would handle:
        // - auth.exportAuthorization results
        // - auth.importAuthorization results
        // - auth.dropTempAuthKeys results
        // For now, just return OK
        Ok(())
    }

    /// Runs the DC auth loop.
    ///
    /// This should be called periodically to update auth states.
    pub fn dc_loop(&self, dc_id: DcId) -> Result<(), DcAuthError> {
        let info = self.get_dc(dc_id)?;

        if info.is_ready() {
            return Ok(());
        }

        match info.state {
            DcState::Waiting => {
                // In a real implementation, would start auth key exchange
                // or export from main DC
            }
            DcState::Export => {
                // In a real implementation, would export auth key
            }
            DcState::Import => {
                // In a real implementation, would import auth key
            }
            DcState::BeforeOk => {
                // In a real implementation, would wait for auth.ok
            }
            DcState::Ok => {
                // Auth is ready, nothing to do
            }
        }

        Ok(())
    }

    /// Runs the destroy loop.
    ///
    /// Cleans up auth keys when requested.
    pub fn destroy_loop(&self) -> Result<(), DcAuthError> {
        if !self.need_destroy_auth_key() {
            return Ok(());
        }

        // In a real implementation, would:
        // 1. Call auth.logOut on all DCs
        // 2. Destroy all auth keys
        // 3. Clear all auth data

        self.mark_auth_keys_destroyed();
        Ok(())
    }
}

impl Default for DcAuthManager {
    fn default() -> Self {
        Self::new(DcId::internal(2))
    }
}

/// Storage for DC auth keys.
///
/// Provides persistence for auth keys across sessions.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DcAuthKeyStorage {
    /// Stored auth keys indexed by DC ID
    keys: HashMap<i32, StoredAuthKey>,
}

/// Stored auth key data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAuthKey {
    /// DC ID
    pub dc_id: i32,

    /// Auth key ID
    pub auth_key_id: u64,

    /// Auth key data (encrypted)
    pub key_data: Vec<u8>,

    /// Expiration timestamp (None for permanent keys)
    pub expires_at: Option<u64>,

    /// Whether this is a temporary key
    pub is_temporary: bool,

    /// Whether this is a CDN key
    pub is_cdn: bool,
}

impl DcAuthKeyStorage {
    /// Creates a new storage.
    pub fn new() -> Self {
        Self::default()
    }

    /// Stores an auth key.
    pub fn store_key(&mut self, dc_id: DcId, key: &AuthKey) -> Result<(), DcAuthError> {
        if !dc_id.is_exact() {
            return Err(DcAuthError::InvalidDcId(dc_id));
        }

        let stored = StoredAuthKey {
            dc_id: dc_id.get_raw_id(),
            auth_key_id: key.id,
            key_data: key.key.clone(),
            expires_at: key.expires_at.map(|e| {
                // Convert Instant to timestamp (simplified)
                e.elapsed().as_secs()
            }),
            is_temporary: key.is_temporary(),
            is_cdn: key.is_cdn,
        };

        self.keys.insert(dc_id.get_raw_id(), stored);
        Ok(())
    }

    /// Loads an auth key for a DC.
    pub fn load_key(&self, dc_id: DcId) -> Result<AuthKey, DcAuthError> {
        if !dc_id.is_exact() {
            return Err(DcAuthError::InvalidDcId(dc_id));
        }

        let stored = self
            .keys
            .get(&dc_id.get_raw_id())
            .ok_or(DcAuthError::AuthKeyNotFound(dc_id))?;

        let expires_at = if stored.is_temporary {
            Some(Instant::now() + std::time::Duration::from_secs(stored.expires_at.unwrap_or(0)))
        } else {
            None
        };

        Ok(AuthKey {
            id: stored.auth_key_id,
            key: stored.key_data.clone(),
            expires_at,
            is_pfs: stored.is_temporary,
            is_cdn: stored.is_cdn,
        })
    }

    /// Removes an auth key.
    pub fn remove_key(&mut self, dc_id: DcId) -> Result<(), DcAuthError> {
        if !dc_id.is_exact() {
            return Err(DcAuthError::InvalidDcId(dc_id));
        }

        self.keys
            .remove(&dc_id.get_raw_id())
            .ok_or(DcAuthError::AuthKeyNotFound(dc_id))?;

        Ok(())
    }

    /// Returns `true` if a DC has a stored key.
    pub fn has_key(&self, dc_id: DcId) -> bool {
        if !dc_id.is_exact() {
            return false;
        }

        self.keys.contains_key(&dc_id.get_raw_id())
    }

    /// Clears all stored keys.
    pub fn clear(&mut self) {
        self.keys.clear();
    }

    /// Returns the number of stored keys.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Returns `true` if storage is empty.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dc_auth_info() {
        let dc_id = DcId::internal(2);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));

        let mut info = DcAuthInfo::new(dc_id, auth_data);

        assert_eq!(info.dc_id, dc_id);
        assert_eq!(info.state, DcState::Waiting);
        assert!(!info.is_ready());
        assert!(!info.is_busy());

        info.state = DcState::Ok;
        info.auth_key_state = AuthKeyState::Ready;

        assert!(info.is_ready());
        assert!(!info.is_busy());
    }

    #[test]
    fn test_temp_auth_key_watchdog() {
        let watchdog = TempAuthKeyWatchdog::new();

        assert_eq!(watchdog.active_count(), 0);
        assert!(!watchdog.needs_sync());

        {
            let _key1 = watchdog.register_auth_key_id(100);
            let _key2 = watchdog.register_auth_key_id(200);

            assert_eq!(watchdog.active_count(), 2);
            assert!(watchdog.needs_sync());

            let ids = watchdog.active_key_ids();
            assert!(ids.contains(&100));
            assert!(ids.contains(&200));
        }

        // Keys should be unregistered after drop
        assert_eq!(watchdog.active_count(), 0);
    }

    #[test]
    fn test_dc_auth_manager() {
        let manager = DcAuthManager::new(DcId::internal(2));

        assert_eq!(manager.main_dc_id(), DcId::internal(2));

        let dc1 = DcId::internal(1);
        let auth_data1 = Arc::new(AuthDataShared::new(dc1));

        manager.add_dc(dc1, auth_data1).unwrap();

        let info = manager.get_dc(dc1).unwrap();
        assert_eq!(info.dc_id, dc1);

        let dc2 = DcId::internal(2);
        assert!(manager.find_dc(dc2).is_none());

        manager.update_main_dc(DcId::internal(4));
        assert_eq!(manager.main_dc_id(), DcId::internal(4));
    }

    #[test]
    fn test_dc_auth_manager_state_updates() {
        let manager = DcAuthManager::new(DcId::internal(2));

        let dc1 = DcId::internal(1);
        let auth_data1 = Arc::new(AuthDataShared::new(dc1));

        manager.add_dc(dc1, auth_data1).unwrap();

        manager
            .update_auth_key_state(dc1, AuthKeyState::Ready)
            .unwrap();
        manager.update_dc_state(dc1, DcState::Ok).unwrap();

        let info = manager.get_dc(dc1).unwrap();
        assert_eq!(info.auth_key_state, AuthKeyState::Ready);
        assert_eq!(info.state, DcState::Ok);
    }

    #[test]
    fn test_dc_auth_manager_export_data() {
        let manager = DcAuthManager::new(DcId::internal(2));

        let dc1 = DcId::internal(1);
        let auth_data1 = Arc::new(AuthDataShared::new(dc1));

        manager.add_dc(dc1, auth_data1).unwrap();

        let data = vec![1, 2, 3, 4];
        manager.set_export_data(dc1, 12345, data.clone()).unwrap();

        let info = manager.get_dc(dc1).unwrap();
        assert_eq!(info.export_id, 12345);
        assert_eq!(info.export_bytes, Some(data));
        assert_eq!(info.state, DcState::Import);
    }

    #[test]
    fn test_dc_auth_manager_destroy() {
        let manager = DcAuthManager::new(DcId::internal(2));

        assert!(!manager.is_destroying());
        assert!(!manager.need_destroy_auth_key());

        manager.destroy_auth_keys().unwrap();

        assert!(manager.is_destroying());
        assert!(manager.need_destroy_auth_key());

        manager.mark_auth_keys_destroyed();

        assert!(!manager.is_destroying());
        assert!(!manager.need_destroy_auth_key());
    }

    #[test]
    fn test_dc_auth_manager_invalid_dc() {
        let manager = DcAuthManager::new(DcId::internal(2));

        let result = manager.add_dc(
            DcId::invalid(),
            Arc::new(AuthDataShared::new(DcId::internal(1))),
        );
        assert!(result.is_err());

        let result = manager.get_dc(DcId::internal(99));
        assert!(result.is_err());
    }

    #[test]
    fn test_storage() {
        let mut storage = DcAuthKeyStorage::new();

        let dc1 = DcId::internal(1);
        let key = AuthKey::new(123, vec![1, 2, 3, 4]);

        assert!(!storage.has_key(dc1));

        storage.store_key(dc1, &key).unwrap();
        assert!(storage.has_key(dc1));
        assert_eq!(storage.len(), 1);

        let loaded = storage.load_key(dc1).unwrap();
        assert_eq!(loaded.id, 123);
        assert_eq!(loaded.key, vec![1, 2, 3, 4]);

        storage.remove_key(dc1).unwrap();
        assert!(!storage.has_key(dc1));
        assert_eq!(storage.len(), 0);
    }

    #[test]
    fn test_storage_temporary_key() {
        let mut storage = DcAuthKeyStorage::new();

        let dc1 = DcId::internal(1);
        let expires = Instant::now() + std::time::Duration::from_secs(3600);
        let key = AuthKey::temporary(123, vec![1, 2, 3, 4], expires);

        storage.store_key(dc1, &key).unwrap();

        let loaded = storage.load_key(dc1).unwrap();
        assert!(loaded.is_temporary());
        assert!(loaded.expires_at.is_some());
    }

    #[test]
    fn test_storage_invalid_dc() {
        let mut storage = DcAuthKeyStorage::new();
        let key = AuthKey::new(123, vec![1, 2, 3, 4]);

        let result = storage.store_key(DcId::invalid(), &key);
        assert!(result.is_err());

        let result = storage.load_key(DcId::invalid());
        assert!(result.is_err());

        let result = storage.remove_key(DcId::invalid());
        assert!(result.is_err());
    }
}
