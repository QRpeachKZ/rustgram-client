// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto session connection.
//!
//! This module implements TDLib's SessionConnection from `td/telegram/net/Session.h`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::auth::{AuthDataShared, AuthKeyState};
use crate::connection::ConnectionError;
use crate::dc::DcId;
use crate::query::NetQuery;
use crate::transport::WriteOptions;

use super::packets::ServicePacket;
use super::ping::{PingConfig, PingManager};
use super::query::QueryLifecycle;
use super::{SessionState, SessionStatistics};

/// Default MTProto timeout.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

/// Session connection configuration.
#[derive(Debug, Clone)]
pub struct SessionConnectionConfig {
    /// DC ID
    pub dc_id: DcId,

    /// Whether to use PFS (Perfect Forward Secrecy)
    pub use_pfs: bool,

    /// Whether this is a main session
    pub is_main: bool,

    /// Whether this is a CDN session
    pub is_cdn: bool,

    /// Transport write options
    pub write_options: WriteOptions,

    /// Ping configuration
    pub ping_config: PingConfig,

    /// Query timeout
    pub query_timeout: Duration,
}

impl Default for SessionConnectionConfig {
    fn default() -> Self {
        Self {
            dc_id: DcId::internal(2),
            use_pfs: true,
            is_main: false,
            is_cdn: false,
            write_options: WriteOptions::default(),
            ping_config: PingConfig::default(),
            query_timeout: DEFAULT_TIMEOUT,
        }
    }
}

impl SessionConnectionConfig {
    /// Creates a new configuration.
    pub fn new(dc_id: DcId) -> Self {
        Self {
            dc_id,
            ..Default::default()
        }
    }
}

/// Session connection events.
#[derive(Debug, Clone)]
pub enum SessionEvent {
    /// Connection state changed
    StateChanged(SessionState),

    /// Auth key state changed
    AuthKeyChanged(AuthKeyState),

    /// Query completed
    QueryCompleted(u64), // Query ID

    /// Error occurred
    Error(String),
}

/// MTProto session connection.
///
/// Manages a single MTProto session with a Telegram DC.
pub struct SessionConnection {
    /// Configuration
    config: SessionConnectionConfig,

    /// Auth data
    auth_data: Arc<AuthDataShared>,

    /// Connection state
    state: Arc<AtomicU8>,

    /// Network generation (incremented on reconnect)
    network_generation: Arc<AtomicU32>,

    /// Session ID
    session_id: Arc<AtomicU64>,

    /// Event sender
    event_sender: mpsc::UnboundedSender<SessionEvent>,

    /// Query sender
    query_sender: mpsc::UnboundedSender<NetQuery>,

    /// Active queries (message_id -> query)
    active_queries: Arc<Mutex<HashMap<u64, NetQuery>>>,

    /// Query lifecycle manager
    query_lifecycle: Arc<QueryLifecycle>,

    /// Ping manager
    ping_manager: Arc<Mutex<PingManager>>,

    /// Statistics
    statistics: Arc<Mutex<SessionStatistics>>,

    /// Stop flag
    stop_flag: Arc<AtomicBool>,
}

impl SessionConnection {
    /// Creates a new session connection.
    pub fn new(config: SessionConnectionConfig, auth_data: Arc<AuthDataShared>) -> Self {
        let session_id = Self::generate_session_id();
        let ping_config = config.ping_config;

        let (event_sender, _) = mpsc::unbounded_channel();
        let (query_sender, _) = mpsc::unbounded_channel();

        Self {
            config,
            auth_data,
            state: Arc::new(AtomicU8::new(SessionState::Empty as u8)),
            network_generation: Arc::new(AtomicU32::new(0)),
            session_id: Arc::new(AtomicU64::new(session_id)),
            event_sender,
            query_sender,
            active_queries: Arc::new(Mutex::new(HashMap::new())),
            query_lifecycle: Arc::new(QueryLifecycle::new()),
            ping_manager: Arc::new(Mutex::new(PingManager::new(ping_config))),
            statistics: Arc::new(Mutex::new(SessionStatistics::default())),
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Generates a random session ID.
    fn generate_session_id() -> u64 {
        use rand::Rng;
        rand::thread_rng().gen()
    }

    /// Returns the session ID.
    pub fn session_id(&self) -> u64 {
        self.session_id.load(Ordering::Relaxed)
    }

    /// Returns the DC ID.
    pub fn dc_id(&self) -> DcId {
        self.config.dc_id
    }

    /// Returns the connection state.
    pub fn state(&self) -> SessionState {
        match self.state.load(Ordering::Relaxed) {
            0 => SessionState::Empty,
            1 => SessionState::Connecting,
            2 => SessionState::Ready,
            3 => SessionState::Closing,
            4 => SessionState::Closed,
            _ => SessionState::Empty,
        }
    }

    /// Sets the connection state.
    pub fn set_state(&self, state: SessionState) {
        self.state.store(state as u8, Ordering::Release);

        let _ = self.event_sender.send(SessionEvent::StateChanged(state));
    }

    /// Returns the auth data.
    pub fn auth_data(&self) -> &Arc<AuthDataShared> {
        &self.auth_data
    }

    /// Returns the network generation.
    pub fn network_generation(&self) -> u32 {
        self.network_generation.load(Ordering::Relaxed)
    }

    /// Increments the network generation.
    pub fn increment_network_generation(&self) {
        self.network_generation.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns the event sender for subscribing to events.
    pub fn event_sender(&self) -> mpsc::UnboundedSender<SessionEvent> {
        self.event_sender.clone()
    }

    /// Returns the query sender.
    pub fn query_sender(&self) -> mpsc::UnboundedSender<NetQuery> {
        self.query_sender.clone()
    }

    /// Returns the statistics.
    pub fn statistics(&self) -> SessionStatistics {
        self.statistics.lock().clone()
    }

    /// Returns `true` if the connection is ready.
    pub fn is_ready(&self) -> bool {
        self.state() == SessionState::Ready
            && self.auth_data.auth_key_state() == AuthKeyState::Ready
    }

    /// Sends a query through this session.
    pub fn send_query(&self, query: NetQuery) -> Result<(), ConnectionError> {
        if !self.is_ready() {
            return Err(ConnectionError::Failed("Session not ready".into()));
        }

        self.query_sender
            .send(query)
            .map_err(|_| ConnectionError::Failed("Failed to send query".into()))?;

        Ok(())
    }

    /// Processes an incoming packet.
    pub fn process_packet(&self, data: &[u8]) -> Result<(), ConnectionError> {
        // Try to decode as service packet first
        if let Ok(service_packet) = ServicePacket::decode(data) {
            return self.handle_service_packet(service_packet);
        }

        // Try to match to active query
        // For now, just acknowledge
        tracing::debug!("Received {} bytes packet", data.len());

        Ok(())
    }

    /// Handles a service packet.
    fn handle_service_packet(&self, packet: ServicePacket) -> Result<(), ConnectionError> {
        match packet {
            ServicePacket::Pong(ping_id) => {
                self.ping_manager.lock().on_pong(ping_id);
            }
            ServicePacket::NewSessionCreated { .. } => {
                tracing::debug!("New session created");
            }
            ServicePacket::Ack { msg_ids } => {
                tracing::debug!("Received ack for {} messages", msg_ids.len());
                // Acknowledge received messages
            }
            _ => {
                tracing::debug!("Unhandled service packet: {:?}", packet);
            }
        }

        Ok(())
    }

    /// Starts the session connection.
    pub async fn start(&self) -> Result<(), ConnectionError> {
        self.set_state(SessionState::Connecting);

        // Check auth key
        if self.auth_data.auth_key_state() != AuthKeyState::Ready {
            // Need to create auth key - this should be done externally
            tracing::warn!("Auth key not ready for DC {}", self.config.dc_id);
        }

        self.set_state(SessionState::Ready);

        // Start ping manager
        self.start_ping_loop().await;

        Ok(())
    }

    /// Stops the session connection.
    pub async fn stop(&self) -> Result<(), ConnectionError> {
        self.set_state(SessionState::Closing);
        self.stop_flag.store(true, Ordering::Relaxed);
        self.set_state(SessionState::Closed);

        Ok(())
    }

    /// Starts the ping loop.
    async fn start_ping_loop(&self) {
        let ping_manager = self.ping_manager.clone();
        let query_sender = self.query_sender.clone();
        let stop_flag = self.stop_flag.clone();

        tokio::spawn(async move {
            while !stop_flag.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_secs(10)).await;

                if let Some(ping) = ping_manager.lock().create_ping() {
                    // Send ping query
                    tracing::trace!("Sending ping: {}", ping.ping_id);
                }
            }
        });
    }

    /// Registers a query.
    pub fn register_query(&self, message_id: u64, query: NetQuery) {
        self.active_queries.lock().insert(message_id, query);
    }

    /// Completes a query.
    pub fn complete_query(&self, message_id: u64, result: Result<Bytes, String>) {
        if let Some(query) = self.active_queries.lock().remove(&message_id) {
            match result {
                Ok(data) => {
                    query.set_ok(data);
                    let _ = self
                        .event_sender
                        .send(SessionEvent::QueryCompleted(query.id()));
                }
                Err(error) => {
                    use crate::query::QueryError;
                    query.set_error(QueryError::Generic(error));
                }
            }
        }
    }

    /// Processes a timeout for a query.
    pub fn on_query_timeout(&self, query: NetQuery) {
        use crate::query::QueryError;

        query.set_error(QueryError::Generic("Query timeout".into()));

        self.statistics.lock().failed_queries += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_state_default() {
        let state = SessionState::default();
        assert_eq!(state, SessionState::Empty);
    }

    #[test]
    fn test_session_config_default() {
        let config = SessionConnectionConfig::default();
        assert_eq!(config.dc_id, DcId::internal(2));
        assert!(config.use_pfs);
        assert!(!config.is_main);
        assert!(!config.is_cdn);
    }

    #[test]
    fn test_session_config_new() {
        let config = SessionConnectionConfig::new(DcId::internal(4));
        assert_eq!(config.dc_id, DcId::internal(4));
        assert!(config.use_pfs);
    }

    #[test]
    fn test_session_statistics_default() {
        let stats = SessionStatistics::default();
        assert_eq!(stats.packets_sent, 0);
        assert_eq!(stats.packets_received, 0);
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.bytes_received, 0);
        assert_eq!(stats.successful_queries, 0);
        assert_eq!(stats.failed_queries, 0);
        assert!(stats.ping_ms.is_none());
    }

    #[tokio::test]
    async fn test_session_connection_new() {
        let config = SessionConnectionConfig::new(DcId::internal(2));
        let auth_data = Arc::new(AuthDataShared::new(DcId::internal(2)));

        let conn = SessionConnection::new(config, auth_data);

        assert_eq!(conn.dc_id(), DcId::internal(2));
        assert_eq!(conn.state(), SessionState::Empty);
        assert!(!conn.is_ready());
    }

    #[test]
    fn test_session_state_transitions() {
        let config = SessionConnectionConfig::new(DcId::internal(2));
        let auth_data = Arc::new(AuthDataShared::new(DcId::internal(2)));

        let conn = SessionConnection::new(config, auth_data);

        conn.set_state(SessionState::Connecting);
        assert_eq!(conn.state(), SessionState::Connecting);

        conn.set_state(SessionState::Ready);
        assert_eq!(conn.state(), SessionState::Ready);
    }
}
