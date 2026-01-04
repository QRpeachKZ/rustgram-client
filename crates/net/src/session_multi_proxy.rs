// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Session multiproxy for managing multiple session types.
//!
//! This module implements TDLib's SessionMultiProxy from `td/telegram/net/SessionMultiProxy.h`.
//!
//! Manages multiple sessions for different purposes:
//! - Main session: Regular RPC calls
//! - Download session: File downloads
//! - Upload session: File uploads
//! - Download small session: Small file downloads

use crate::auth::AuthDataShared;
use crate::dc::DcId;
use crate::query::{NetQuery, NetQueryType};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;
use thiserror::Error;

/// Error types for session multiproxy operations.
#[derive(Debug, Error)]
pub enum SessionProxyError {
    /// Invalid session count
    #[error("Invalid session count: {0}")]
    InvalidSessionCount(u32),

    /// Session not found
    #[error("Session not found: {0}")]
    SessionNotFound(u32),

    /// All sessions are busy
    #[error("All sessions are busy")]
    AllSessionsBusy,

    /// Query dispatch failed
    #[error("Failed to dispatch query: {0}")]
    DispatchFailed(String),

    /// Auth key destruction failed
    #[error("Failed to destroy auth key: {0}")]
    DestroyFailed(String),
}

/// Session type identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[derive(Default)]
pub enum SessionType {
    /// Main session for general queries
    #[default]
    Main = 0,

    /// Download session for files
    Download = 1,

    /// Upload session for files
    Upload = 2,

    /// Download small session for small files
    DownloadSmall = 3,
}

/// Information about a single session.
pub struct SessionInfo {
    /// Session proxy (abstract - in real impl, would be a SessionProxy actor)
    pub proxy: Arc<Mutex<Option<Box<dyn SessionProxy>>>>,

    /// Number of active queries
    pub query_count: Arc<AtomicU32>,

    /// Session type
    pub session_type: SessionType,

    /// Last activity timestamp
    pub last_activity: Arc<Mutex<Instant>>,

    /// Whether this session is currently active
    pub is_active: Arc<AtomicBool>,
}

impl fmt::Debug for SessionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionInfo")
            .field("session_type", &self.session_type)
            .field("query_count", &self.get_query_count())
            .field("is_idle", &self.is_idle())
            .field("last_activity", &self.last_activity.lock())
            .finish()
    }
}

impl SessionInfo {
    /// Creates new session info.
    pub fn new(session_type: SessionType) -> Self {
        Self {
            proxy: Arc::new(Mutex::new(None)),
            query_count: Arc::new(AtomicU32::new(0)),
            session_type,
            last_activity: Arc::new(Mutex::new(Instant::now())),
            is_active: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Returns the current query count.
    pub fn get_query_count(&self) -> u32 {
        self.query_count.load(Ordering::Relaxed)
    }

    /// Increments the query count.
    pub fn increment_query_count(&self) {
        self.query_count.fetch_add(1, Ordering::Relaxed);
        *self.last_activity.lock() = Instant::now();
    }

    /// Decrements the query count.
    pub fn decrement_query_count(&self) {
        self.query_count.fetch_sub(1, Ordering::Relaxed);
        *self.last_activity.lock() = Instant::now();
    }

    /// Returns `true` if the session is idle.
    pub fn is_idle(&self) -> bool {
        self.get_query_count() == 0
    }

    /// Returns the last activity time.
    pub fn last_activity(&self) -> Instant {
        *self.last_activity.lock()
    }
}

/// Trait for session proxy implementations.
///
/// In a real implementation, this would wrap the actual SessionProxy actor.
pub trait SessionProxy: Send + Sync {
    /// Sends a query through this session.
    fn send(&self, query: NetQuery) -> Result<(), SessionProxyError>;

    /// Updates the main flag for this session.
    fn update_main_flag(&self, is_main: bool);

    /// Returns the session type.
    fn session_type(&self) -> SessionType;

    /// Destroys the auth key for this session.
    fn destroy_auth_key(&self) -> Result<(), SessionProxyError>;

    /// Returns the number of pending queries.
    fn pending_count(&self) -> u32;
}

/// Default implementation of SessionProxy for testing.
#[derive(Debug)]
pub struct DummySessionProxy {
    session_type: SessionType,
    is_main: AtomicBool,
    pending: AtomicU32,
}

impl DummySessionProxy {
    /// Creates a new dummy session proxy.
    pub fn new(session_type: SessionType) -> Self {
        Self {
            session_type,
            is_main: AtomicBool::new(false),
            pending: AtomicU32::new(0),
        }
    }
}

impl SessionProxy for DummySessionProxy {
    fn send(&self, _query: NetQuery) -> Result<(), SessionProxyError> {
        self.pending.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn update_main_flag(&self, is_main: bool) {
        self.is_main.store(is_main, Ordering::Relaxed);
    }

    fn session_type(&self) -> SessionType {
        self.session_type
    }

    fn destroy_auth_key(&self) -> Result<(), SessionProxyError> {
        // Dummy implementation
        Ok(())
    }

    fn pending_count(&self) -> u32 {
        self.pending.load(Ordering::Relaxed)
    }
}

/// Session multiproxy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMultiProxyConfig {
    /// Number of sessions to create
    pub session_count: u32,

    /// Whether this is the primary connection
    pub is_primary: bool,

    /// Whether this is the main DC
    pub is_main: bool,

    /// Whether to use Perfect Forward Secrecy
    pub use_pfs: bool,

    /// Whether to allow media-only DCs
    pub allow_media_only: bool,

    /// Whether this is a media connection
    pub is_media: bool,

    /// Whether this is a CDN connection
    pub is_cdn: bool,

    /// Whether to destroy auth key on close
    pub need_destroy_auth_key: bool,
}

impl Default for SessionMultiProxyConfig {
    fn default() -> Self {
        Self {
            session_count: 1,
            is_primary: true,
            is_main: false,
            use_pfs: false,
            allow_media_only: false,
            is_media: false,
            is_cdn: false,
            need_destroy_auth_key: false,
        }
    }
}

impl SessionMultiProxyConfig {
    /// Creates a new config.
    pub fn new(session_count: u32, is_primary: bool) -> Self {
        Self {
            session_count,
            is_primary,
            ..Self::default()
        }
    }

    /// Returns `true` if PFS should be used.
    pub fn get_pfs_flag(&self) -> bool {
        // Use PFS for main sessions when enabled
        self.use_pfs && self.is_main && !self.is_cdn
    }
}

/// Session multiproxy.
///
/// Based on TDLib's SessionMultiProxy from `td/telegram/net/SessionMultiProxy.h`.
///
/// Manages multiple sessions and dispatches queries to them based on:
/// - Query type (upload, download, common)
/// - Current load balancing
/// - Session availability
pub struct SessionMultiProxy {
    /// Configuration
    config: Mutex<SessionMultiProxyConfig>,

    /// Auth data shared across all sessions
    auth_data: Arc<AuthDataShared>,

    /// Sessions indexed by session ID
    sessions: Mutex<Vec<SessionInfo>>,

    /// Sessions generation (for tracking session recreation)
    sessions_generation: AtomicU32,

    /// DC ID
    dc_id: DcId,
}

impl SessionMultiProxy {
    /// Creates a new session multiproxy.
    pub fn new(
        dc_id: DcId,
        config: SessionMultiProxyConfig,
        auth_data: Arc<AuthDataShared>,
    ) -> Result<Self, SessionProxyError> {
        if config.session_count == 0 {
            return Err(SessionProxyError::InvalidSessionCount(0));
        }

        let mut sessions = Vec::new();

        // Initialize sessions based on count
        for i in 0..config.session_count {
            let session_type = if config.is_cdn {
                SessionType::Download
            } else if config.is_media {
                SessionType::Download
            } else {
                SessionType::Main
            };

            sessions.push(SessionInfo::new(session_type));

            // Create dummy proxy for testing
            // In real implementation, would create actual SessionProxy actors
            let proxy = Box::new(DummySessionProxy::new(session_type)) as Box<dyn SessionProxy>;
            sessions[i as usize].proxy.lock().replace(proxy);
        }

        Ok(Self {
            config: Mutex::new(config),
            auth_data,
            sessions: Mutex::new(sessions),
            sessions_generation: AtomicU32::new(0),
            dc_id,
        })
    }

    /// Returns the DC ID.
    pub fn dc_id(&self) -> DcId {
        self.dc_id
    }

    /// Returns the auth data.
    pub fn auth_data(&self) -> Arc<AuthDataShared> {
        Arc::clone(&self.auth_data)
    }

    /// Sends a query through the appropriate session.
    pub fn send(&self, query: NetQuery) -> Result<(), SessionProxyError> {
        let sessions = self.sessions.lock();
        let config = self.config.lock();

        let session_id = self.select_session(&query, &sessions, &config)?;

        if let Some(session) = sessions.get(session_id as usize) {
            // Update query count
            session.increment_query_count();

            // Get the proxy and send the query
            let proxy_guard = session.proxy.lock();
            if let Some(proxy) = proxy_guard.as_ref() {
                let result = proxy.send(query);

                // If send failed, decrement count
                if result.is_err() {
                    session.decrement_query_count();
                }

                result
            } else {
                session.decrement_query_count();
                Err(SessionProxyError::SessionNotFound(session_id))
            }
        } else {
            Err(SessionProxyError::SessionNotFound(session_id))
        }
    }

    /// Selects the best session for a query.
    fn select_session(
        &self,
        query: &NetQuery,
        sessions: &[SessionInfo],
        _config: &SessionMultiProxyConfig,
    ) -> Result<u32, SessionProxyError> {
        // For now, use simple round-robin
        // In a real implementation, would consider:
        // - Query type (upload vs download vs common)
        // - Current load on each session
        // - Session health status

        match query.query_type() {
            NetQueryType::Upload => {
                // Find least loaded session
                let session_id = sessions
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, s)| s.get_query_count())
                    .map(|(i, _)| i as u32)
                    .ok_or_else(|| {
                        SessionProxyError::DispatchFailed("No sessions available".into())
                    })?;

                Ok(session_id)
            }
            NetQueryType::Download | NetQueryType::DownloadSmall => {
                // For downloads, use first available session
                let session_id = sessions
                    .iter()
                    .position(|s| s.is_idle())
                    .or_else(|| sessions.first().map(|_| 0))
                    .ok_or_else(|| {
                        SessionProxyError::DispatchFailed("No sessions available".into())
                    })? as u32;

                Ok(session_id)
            }
            NetQueryType::Common => {
                // For common queries, use main session (first one)
                Ok(0)
            }
        }
    }

    /// Updates the main flag for all sessions.
    pub fn update_main_flag(&self, is_main: bool) {
        let mut config = self.config.lock();
        config.is_main = is_main;

        let sessions = self.sessions.lock();
        for session in sessions.iter() {
            let proxy_guard = session.proxy.lock();
            if let Some(proxy) = proxy_guard.as_ref() {
                proxy.update_main_flag(is_main);
            }
        }
    }

    /// Updates the session count.
    ///
    /// Will recreate sessions if the count changes.
    pub fn update_session_count(&self, new_count: u32) -> Result<(), SessionProxyError> {
        if new_count == 0 {
            return Err(SessionProxyError::InvalidSessionCount(0));
        }

        let mut config = self.config.lock();
        let old_count = config.session_count;

        if old_count == new_count {
            return Ok(());
        }

        config.session_count = new_count;
        drop(config);

        // Recreate sessions
        self.init()?;

        Ok(())
    }

    /// Updates PFS setting.
    pub fn update_use_pfs(&self, use_pfs: bool) {
        let mut config = self.config.lock();
        config.use_pfs = use_pfs;
    }

    /// Updates multiple options at once.
    pub fn update_options(
        &self,
        session_count: u32,
        use_pfs: bool,
        need_destroy_auth_key: bool,
    ) -> Result<(), SessionProxyError> {
        let mut config = self.config.lock();

        let recreate = config.session_count != session_count;
        config.session_count = session_count;
        config.use_pfs = use_pfs;
        config.need_destroy_auth_key = need_destroy_auth_key;

        drop(config);

        if recreate {
            self.init()?;
        }

        Ok(())
    }

    /// Destroys all auth keys.
    pub fn destroy_auth_key(&self) -> Result<(), SessionProxyError> {
        let sessions = self.sessions.lock();

        for session in sessions.iter() {
            let proxy_guard = session.proxy.lock();
            if let Some(proxy) = proxy_guard.as_ref() {
                proxy.destroy_auth_key()?;
            }
        }

        // Clear auth data
        self.auth_data.clear();

        Ok(())
    }

    /// Returns the current session count.
    pub fn session_count(&self) -> u32 {
        self.config.lock().session_count
    }

    /// Returns `true` if using PFS.
    pub fn use_pfs(&self) -> bool {
        self.config.lock().use_pfs
    }

    /// Returns `true` if this is the main DC.
    pub fn is_main(&self) -> bool {
        self.config.lock().is_main
    }

    /// Returns `true` if this is a CDN connection.
    pub fn is_cdn(&self) -> bool {
        self.config.lock().is_cdn
    }

    /// Returns the sessions generation.
    pub fn generation(&self) -> u32 {
        self.sessions_generation.load(Ordering::Relaxed)
    }

    /// Initializes/recreates all sessions.
    fn init(&self) -> Result<(), SessionProxyError> {
        let config = self.config.lock();
        let mut sessions = self.sessions.lock();

        // Clear existing sessions
        sessions.clear();

        // Increment generation
        self.sessions_generation.fetch_add(1, Ordering::Relaxed);

        // Create new sessions
        for i in 0..config.session_count {
            let session_type = if config.is_cdn {
                SessionType::Download
            } else if config.is_media {
                SessionType::Download
            } else {
                SessionType::Main
            };

            sessions.push(SessionInfo::new(session_type));

            let proxy = Box::new(DummySessionProxy::new(session_type)) as Box<dyn SessionProxy>;
            sessions[i as usize].proxy.lock().replace(proxy);
        }

        Ok(())
    }

    /// Handles query completion.
    pub fn on_query_finished(&self, session_id: u32, _generation: u32) {
        let sessions = self.sessions.lock();

        if let Some(session) = sessions.get(session_id as usize) {
            session.decrement_query_count();
        }
    }

    /// Returns statistics for all sessions.
    pub fn get_stats(&self) -> Vec<SessionStats> {
        let sessions = self.sessions.lock();
        let config = self.config.lock();

        sessions
            .iter()
            .enumerate()
            .map(|(i, s)| SessionStats {
                session_id: i as u32,
                session_type: s.session_type,
                query_count: s.get_query_count(),
                is_idle: s.is_idle(),
                last_activity: s.last_activity(),
                is_main: config.is_main,
                use_pfs: config.get_pfs_flag(),
            })
            .collect()
    }
}

/// Statistics for a single session.
#[derive(Debug, Clone)]
pub struct SessionStats {
    /// Session ID
    pub session_id: u32,

    /// Session type
    pub session_type: SessionType,

    /// Number of active queries
    pub query_count: u32,

    /// Whether the session is idle
    pub is_idle: bool,

    /// Last activity timestamp
    pub last_activity: Instant,

    /// Whether this is a main session
    pub is_main: bool,

    /// Whether PFS is enabled
    pub use_pfs: bool,
}

/// Factory for creating session multiproxies.
pub struct SessionMultiProxyFactory;

impl SessionMultiProxyFactory {
    /// Creates a main session multiproxy.
    pub fn create_main(
        dc_id: DcId,
        auth_data: Arc<AuthDataShared>,
    ) -> Result<SessionMultiProxy, SessionProxyError> {
        let config = SessionMultiProxyConfig {
            session_count: 1,
            is_primary: true,
            is_main: true,
            use_pfs: true,
            ..Default::default()
        };

        SessionMultiProxy::new(dc_id, config, auth_data)
    }

    /// Creates a download session multiproxy.
    pub fn create_download(
        dc_id: DcId,
        auth_data: Arc<AuthDataShared>,
    ) -> Result<SessionMultiProxy, SessionProxyError> {
        let config = SessionMultiProxyConfig {
            session_count: 4, // Multiple download sessions
            is_primary: false,
            is_main: false,
            use_pfs: false,
            is_media: true,
            ..Default::default()
        };

        SessionMultiProxy::new(dc_id, config, auth_data)
    }

    /// Creates an upload session multiproxy.
    pub fn create_upload(
        dc_id: DcId,
        auth_data: Arc<AuthDataShared>,
    ) -> Result<SessionMultiProxy, SessionProxyError> {
        let config = SessionMultiProxyConfig {
            session_count: 2, // Multiple upload sessions
            is_primary: false,
            is_main: false,
            use_pfs: false,
            is_media: true,
            ..Default::default()
        };

        SessionMultiProxy::new(dc_id, config, auth_data)
    }

    /// Creates a CDN session multiproxy.
    pub fn create_cdn(
        dc_id: DcId,
        auth_data: Arc<AuthDataShared>,
    ) -> Result<SessionMultiProxy, SessionProxyError> {
        let config = SessionMultiProxyConfig {
            session_count: 4,
            is_primary: false,
            is_main: false,
            use_pfs: false,
            is_cdn: true,
            ..Default::default()
        };

        SessionMultiProxy::new(dc_id, config, auth_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    fn create_test_query(query_type: NetQueryType) -> NetQuery {
        NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            query_type,
            crate::query::AuthFlag::On,
            crate::query::GzipFlag::Off,
            0,
        )
    }

    #[test]
    fn test_session_info() {
        let info = SessionInfo::new(SessionType::Main);

        assert_eq!(info.session_type, SessionType::Main);
        assert_eq!(info.get_query_count(), 0);
        assert!(info.is_idle());

        info.increment_query_count();
        assert_eq!(info.get_query_count(), 1);
        assert!(!info.is_idle());

        info.decrement_query_count();
        assert_eq!(info.get_query_count(), 0);
        assert!(info.is_idle());
    }

    #[test]
    fn test_dummy_session_proxy() {
        let proxy = DummySessionProxy::new(SessionType::Download);

        assert_eq!(proxy.session_type(), SessionType::Download);
        assert!(!proxy.is_main.load(Ordering::Relaxed));

        proxy.update_main_flag(true);
        assert!(proxy.is_main.load(Ordering::Relaxed));

        let query = create_test_query(NetQueryType::Download);
        assert!(proxy.send(query).is_ok());
        assert_eq!(proxy.pending_count(), 1);

        assert!(proxy.destroy_auth_key().is_ok());
    }

    #[test]
    fn test_config() {
        let config = SessionMultiProxyConfig::new(4, true);

        assert_eq!(config.session_count, 4);
        assert!(config.is_primary);
        assert!(!config.use_pfs);

        assert!(!config.get_pfs_flag());

        let mut config = config;
        config.use_pfs = true;
        config.is_main = true;

        assert!(config.get_pfs_flag());
    }

    #[test]
    fn test_multi_proxy_creation() {
        let dc_id = DcId::internal(2);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let config = SessionMultiProxyConfig::new(2, false);

        let proxy = SessionMultiProxy::new(dc_id, config, auth_data).unwrap();

        assert_eq!(proxy.dc_id(), dc_id);
        assert_eq!(proxy.session_count(), 2);
        assert!(!proxy.is_main());
        assert!(!proxy.use_pfs());
    }

    #[test]
    fn test_multi_proxy_send() {
        let dc_id = DcId::internal(2);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let config = SessionMultiProxyConfig::new(1, false);

        let proxy = SessionMultiProxy::new(dc_id, config, auth_data).unwrap();

        let query = create_test_query(NetQueryType::Common);
        assert!(proxy.send(query).is_ok());

        let stats = proxy.get_stats();
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].query_count, 1);
    }

    #[test]
    fn test_multi_proxy_send_multiple_sessions() {
        let dc_id = DcId::internal(2);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let config = SessionMultiProxyConfig::new(3, false);

        let proxy = SessionMultiProxy::new(dc_id, config, auth_data).unwrap();

        let query1 = create_test_query(NetQueryType::Upload);
        let query2 = create_test_query(NetQueryType::Upload);

        assert!(proxy.send(query1).is_ok());
        assert!(proxy.send(query2).is_ok());

        let stats = proxy.get_stats();
        let total_queries: u32 = stats.iter().map(|s| s.query_count).sum();
        assert_eq!(total_queries, 2);
    }

    #[test]
    fn test_multi_proxy_update_session_count() {
        let dc_id = DcId::internal(2);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let config = SessionMultiProxyConfig::new(1, false);

        let proxy = SessionMultiProxy::new(dc_id, config, auth_data).unwrap();

        assert_eq!(proxy.session_count(), 1);
        assert_eq!(proxy.generation(), 0);

        proxy.update_session_count(4).unwrap();

        assert_eq!(proxy.session_count(), 4);
        assert_eq!(proxy.generation(), 1);
    }

    #[test]
    fn test_multi_proxy_update_flags() {
        let dc_id = DcId::internal(2);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let config = SessionMultiProxyConfig::new(1, false);

        let proxy = SessionMultiProxy::new(dc_id, config, auth_data).unwrap();

        proxy.update_main_flag(true);
        assert!(proxy.is_main());

        proxy.update_use_pfs(true);
        assert!(proxy.use_pfs());
    }

    #[test]
    fn test_multi_proxy_destroy_auth_key() {
        let dc_id = DcId::internal(2);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let config = SessionMultiProxyConfig::new(1, false);

        // Set some auth data before creating proxy
        let key = crate::auth::AuthKey::new(123, vec![1, 2, 3, 4]);
        auth_data.set_auth_key(key);

        let proxy = SessionMultiProxy::new(dc_id, config, auth_data.clone()).unwrap();

        assert_eq!(auth_data.auth_key_state(), crate::auth::AuthKeyState::Ready);

        proxy.destroy_auth_key().unwrap();

        assert_eq!(auth_data.auth_key_state(), crate::auth::AuthKeyState::Empty);
    }

    #[test]
    fn test_multi_proxy_invalid_session_count() {
        let dc_id = DcId::internal(2);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let config = SessionMultiProxyConfig::new(0, false);

        let result = SessionMultiProxy::new(dc_id, config, auth_data.clone());
        assert!(result.is_err());

        let proxy =
            SessionMultiProxy::new(dc_id, SessionMultiProxyConfig::new(1, false), auth_data)
                .unwrap();

        let result = proxy.update_session_count(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_factory() {
        let dc_id = DcId::internal(2);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));

        let main = SessionMultiProxyFactory::create_main(dc_id, Arc::clone(&auth_data)).unwrap();
        assert!(main.is_main());
        assert!(main.use_pfs());

        let download =
            SessionMultiProxyFactory::create_download(dc_id, Arc::clone(&auth_data)).unwrap();
        assert!(!download.is_main());
        assert_eq!(download.session_count(), 4);

        let upload =
            SessionMultiProxyFactory::create_upload(dc_id, Arc::clone(&auth_data)).unwrap();
        assert!(!upload.is_main());
        assert_eq!(upload.session_count(), 2);

        let cdn = SessionMultiProxyFactory::create_cdn(dc_id, auth_data).unwrap();
        assert!(cdn.is_cdn());
    }

    #[test]
    fn test_on_query_finished() {
        let dc_id = DcId::internal(2);
        let auth_data = Arc::new(AuthDataShared::new(dc_id));
        let config = SessionMultiProxyConfig::new(1, false);

        let proxy = SessionMultiProxy::new(dc_id, config, auth_data).unwrap();

        let query = create_test_query(NetQueryType::Common);
        proxy.send(query).unwrap();

        let stats = proxy.get_stats();
        assert_eq!(stats[0].query_count, 1);

        proxy.on_query_finished(0, proxy.generation());

        let stats = proxy.get_stats();
        assert_eq!(stats[0].query_count, 0);
    }
}
