// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Connection management for Telegram network layer.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dc::{DcId, DcOption, DcOptionsSet};
use crate::proxy::Proxy;
use crate::query::NetQuery;
use crate::stats::NetType;

/// Connection mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionMode {
    /// TCP mode
    Tcp,
    /// HTTP mode
    Http,
}

impl Default for ConnectionMode {
    fn default() -> Self {
        Self::Tcp
    }
}

/// Connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    /// Connection is empty/not initialized
    Empty,
    /// Currently connecting
    Connecting,
    /// Connection is ready
    Ready,
    /// Connection is closed
    Closed,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self::Empty
    }
}

/// Connection error.
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ConnectionError {
    /// Invalid DC ID
    #[error("Invalid DC ID: {0:?}")]
    InvalidDcId(DcId),

    /// No DC options available
    #[error("No DC options available for DC {0:?}")]
    NoDcOptions(DcId),

    /// Connection failed
    #[error("Connection failed: {0}")]
    Failed(String),

    /// Timeout
    #[error("Connection timeout after {0:?}")]
    Timeout(Duration),

    /// Proxy error
    #[error("Proxy error: {0}")]
    Proxy(String),

    /// SSL/TLS error
    #[error("SSL/TLS error: {0}")]
    Ssl(String),

    /// Socket error
    #[error("Socket error: {0}")]
    Socket(String),
}

/// Connection statistics.
#[derive(Debug, Clone, Default)]
pub struct ConnectionStats {
    /// Bytes sent
    pub bytes_sent: u64,

    /// Bytes received
    pub bytes_received: u64,

    /// Number of connections
    pub connection_count: u64,

    /// Number of failures
    pub failure_count: u64,

    /// Average round-trip time
    pub avg_rtt: Duration,

    /// Last successful connection time
    pub last_success: Option<Instant>,
}

impl ConnectionStats {
    /// Records a successful connection.
    pub fn record_success(&mut self, bytes_sent: u64, bytes_received: u64, rtt: Duration) {
        self.bytes_sent += bytes_sent;
        self.bytes_received += bytes_received;
        self.connection_count += 1;

        // Update average RTT
        if self.connection_count > 1 {
            let total_rtt = self.avg_rtt * (self.connection_count - 1) as u32;
            self.avg_rtt = (total_rtt + rtt) / self.connection_count as u32;
        } else {
            self.avg_rtt = rtt;
        }

        self.last_success = Some(Instant::now());
    }

    /// Records a failed connection.
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
    }

    /// Returns the success rate (0.0 to 1.0).
    pub fn success_rate(&self) -> f64 {
        let total = self.connection_count + self.failure_count;
        if total == 0 {
            0.0
        } else {
            self.connection_count as f64 / total as f64
        }
    }
}

/// Raw connection data.
#[derive(Debug)]
pub struct RawConnection {
    /// DC ID
    pub dc_id: DcId,

    /// Connection mode
    pub mode: ConnectionMode,

    /// Socket fd (conceptually - actual implementation uses Tokio)
    pub socket: Option<tokio::net::TcpStream>,

    /// Statistics
    pub stats: ConnectionStats,

    /// Creation time
    pub created_at: Instant,

    /// Whether this is a media connection
    pub is_media: bool,
}

impl RawConnection {
    /// Creates a new raw connection.
    pub fn new(dc_id: DcId, mode: ConnectionMode, is_media: bool) -> Self {
        Self {
            dc_id,
            mode,
            socket: None,
            stats: ConnectionStats::default(),
            created_at: Instant::now(),
            is_media,
        }
    }

    /// Returns the age of this connection.
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Returns `true` if the connection is still valid.
    pub fn is_valid(&self) -> bool {
        self.socket.is_some() && self.age() < Duration::from_secs(300)
    }
}

/// Connection creator.
///
/// Manages creation of connections to Telegram servers.
/// Based on TDLib's ConnectionCreator from `td/telegram/net/ConnectionCreator.h`.
pub struct ConnectionCreator {
    /// DC options set
    dc_options: Arc<parking_lot::Mutex<DcOptionsSet>>,

    /// Current proxy
    proxy: Arc<parking_lot::Mutex<Proxy>>,

    /// Network type
    net_type: Arc<parking_lot::Mutex<NetType>>,

    /// Whether network is available
    network_flag: Arc<AtomicBool>,

    /// Network generation (incremented when network changes)
    network_generation: Arc<AtomicU32>,

    /// Connection statistics
    stats: Arc<parking_lot::Mutex<ConnectionStats>>,
}

impl ConnectionCreator {
    /// Creates a new connection creator.
    pub fn new() -> Self {
        Self {
            dc_options: Arc::new(parking_lot::Mutex::new(DcOptionsSet::new())),
            proxy: Arc::new(parking_lot::Mutex::new(Proxy::none())),
            net_type: Arc::new(parking_lot::Mutex::new(NetType::Other)),
            network_flag: Arc::new(AtomicBool::new(true)),
            network_generation: Arc::new(AtomicU32::new(0)),
            stats: Arc::new(parking_lot::Mutex::new(ConnectionStats::default())),
        }
    }

    /// Updates DC options.
    pub fn on_dc_options(&self, options: DcOptionsSet) {
        *self.dc_options.lock() = options;
    }

    /// Returns current DC options.
    pub fn dc_options(&self) -> DcOptionsSet {
        self.dc_options.lock().clone()
    }

    /// Sets the proxy.
    pub fn set_proxy(&self, proxy: Proxy) {
        *self.proxy.lock() = proxy;
    }

    /// Returns the current proxy.
    pub fn proxy(&self) -> Proxy {
        self.proxy.lock().clone()
    }

    /// Sets the network type.
    pub fn set_net_type(&self, net_type: NetType) {
        *self.net_type.lock() = net_type;
    }

    /// Returns the network type.
    pub fn net_type(&self) -> NetType {
        *self.net_type.lock()
    }

    /// Sets network availability.
    pub fn set_network_flag(&self, flag: bool) {
        self.network_flag.store(flag, Ordering::Relaxed);
        self.network_generation.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns network availability.
    pub fn network_flag(&self) -> bool {
        self.network_flag.load(Ordering::Relaxed)
    }

    /// Returns the current network generation.
    pub fn network_generation(&self) -> u32 {
        self.network_generation.load(Ordering::Relaxed)
    }

    /// Returns connection statistics.
    pub fn stats(&self) -> ConnectionStats {
        self.stats.lock().clone()
    }

    /// Requests a raw connection to a DC.
    pub async fn request_raw_connection(
        &self,
        dc_id: DcId,
        allow_media_only: bool,
        is_media: bool,
    ) -> Result<RawConnection, ConnectionError> {
        // Check network flag
        if !self.network_flag() {
            return Err(ConnectionError::Failed("No network".into()));
        }

        // Find best DC option
        let dc_options = self.dc_options.lock();
        let option = dc_options
            .find_best_option(dc_id, allow_media_only)
            .ok_or_else(|| ConnectionError::NoDcOptions(dc_id))?;
        drop(dc_options);

        // Create connection
        self.create_connection(option, is_media).await
    }

    /// Creates a connection to a specific DC option.
    async fn create_connection(
        &self,
        option: DcOption,
        is_media: bool,
    ) -> Result<RawConnection, ConnectionError> {
        let _proxy = self.proxy.lock().clone();

        // Determine address to connect to
        let addr = option.socket_addr();

        tracing::debug!("Connecting to {}", addr);

        // Attempt connection with timeout
        let conn = tokio::time::timeout(
            Duration::from_secs(10),
            tokio::net::TcpStream::connect(addr),
        )
        .await
        .map_err(|_| ConnectionError::Timeout(Duration::from_secs(10)))?
        .map_err(|e| ConnectionError::Socket(e.to_string()))?;

        let mode = if option.is_obfuscated_tcp_only() {
            ConnectionMode::Tcp
        } else {
            ConnectionMode::Tcp
        };

        let mut raw_conn = RawConnection::new(option.dc_id, mode, is_media);
        raw_conn.socket = Some(conn);

        // Record success
        self.stats
            .lock()
            .record_success(0, 0, Duration::from_millis(50));

        Ok(raw_conn)
    }

    /// Pings the main DC.
    pub async fn ping_main_dc(&self) -> Result<Duration, ConnectionError> {
        let main_dc = DcId::main();

        let start = Instant::now();
        self.request_raw_connection(main_dc, false, false).await?;
        let rtt = start.elapsed();

        Ok(rtt)
    }
}

impl Default for ConnectionCreator {
    fn default() -> Self {
        Self::new()
    }
}

/// Session for MTProto communication.
///
/// Based on TDLib's Session from `td/telegram/net/Session.h`.
pub struct Session {
    /// Raw DC ID
    raw_dc_id: i32,

    /// DC ID
    dc_id: DcId,

    /// Whether this is a primary session
    is_primary: bool,

    /// Whether this is the main DC session
    is_main: bool,

    /// Whether to use Perfect Forward Secrecy
    use_pfs: bool,

    /// Whether this is a CDN session
    is_cdn: bool,

    /// Connection mode
    mode: ConnectionMode,

    /// Network generation
    network_generation: u32,

    /// Pending queries
    pending_queries: parking_lot::Mutex<Vec<NetQuery>>,

    /// Sent queries (message_id -> query)
    sent_queries: parking_lot::Mutex<std::collections::HashMap<u64, NetQuery>>,

    /// Connection state
    connection_state: parking_lot::Mutex<ConnectionState>,

    /// Session statistics
    stats: parking_lot::Mutex<SessionStats>,
}

/// Session statistics.
#[derive(Debug, Clone, Default)]
pub struct SessionStats {
    /// Number of queries sent
    pub queries_sent: u64,

    /// Number of queries received
    pub queries_received: u64,

    /// Number of failures
    pub failures: u64,

    /// Last activity time
    pub last_activity: Option<Instant>,
}

impl Session {
    /// Creates a new session.
    pub fn new(
        raw_dc_id: i32,
        dc_id: DcId,
        is_primary: bool,
        is_main: bool,
        use_pfs: bool,
        is_cdn: bool,
    ) -> Self {
        Self {
            raw_dc_id,
            dc_id,
            is_primary,
            is_main,
            use_pfs,
            is_cdn,
            mode: ConnectionMode::Tcp,
            network_generation: 0,
            pending_queries: parking_lot::Mutex::new(Vec::new()),
            sent_queries: parking_lot::Mutex::new(std::collections::HashMap::new()),
            connection_state: parking_lot::Mutex::new(ConnectionState::Empty),
            stats: parking_lot::Mutex::new(SessionStats::default()),
        }
    }

    /// Returns the raw DC ID.
    pub fn raw_dc_id(&self) -> i32 {
        self.raw_dc_id
    }

    /// Returns the DC ID.
    pub fn dc_id(&self) -> DcId {
        self.dc_id
    }

    /// Returns `true` if this is a primary session.
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }

    /// Returns `true` if this is the main DC session.
    pub fn is_main(&self) -> bool {
        self.is_main
    }

    /// Returns `true` if using PFS.
    pub fn use_pfs(&self) -> bool {
        self.use_pfs
    }

    /// Returns `true` if this is a CDN session.
    pub fn is_cdn(&self) -> bool {
        self.is_cdn
    }

    /// Returns the connection mode.
    pub fn mode(&self) -> ConnectionMode {
        self.mode
    }

    /// Returns the session statistics.
    pub fn stats(&self) -> SessionStats {
        self.stats.lock().clone()
    }

    /// Sends a query through this session.
    pub fn send(&self, query: NetQuery) {
        self.pending_queries.lock().push(query);
        self.stats.lock().queries_sent += 1;
        self.stats.lock().last_activity = Some(Instant::now());
    }

    /// Closes the session.
    pub fn close(&self) {
        *self.connection_state.lock() = ConnectionState::Closed;
    }

    /// Returns `true` if the session is loaded.
    pub fn is_high_loaded(&self) -> bool {
        self.sent_queries.lock().len() > 1024
    }
}

/// Session proxy for managing multiple sessions.
///
/// Based on TDLib's SessionProxy from `td/telegram/net/SessionProxy.h`.
pub struct SessionProxy {
    /// Main session
    main_session: Option<Session>,

    /// Download session
    download_session: Option<Session>,

    /// Upload session
    upload_session: Option<Session>,
}

impl SessionProxy {
    /// Creates a new session proxy.
    pub fn new() -> Self {
        Self {
            main_session: None,
            download_session: None,
            upload_session: None,
        }
    }

    /// Returns the main session.
    pub fn main_session(&self) -> Option<&Session> {
        self.main_session.as_ref()
    }

    /// Returns the download session.
    pub fn download_session(&self) -> Option<&Session> {
        self.download_session.as_ref()
    }

    /// Returns the upload session.
    pub fn upload_session(&self) -> Option<&Session> {
        self.upload_session.as_ref()
    }

    /// Sets the main session.
    pub fn set_main_session(&mut self, session: Session) {
        self.main_session = Some(session);
    }

    /// Sets the download session.
    pub fn set_download_session(&mut self, session: Session) {
        self.download_session = Some(session);
    }

    /// Sets the upload session.
    pub fn set_upload_session(&mut self, session: Session) {
        self.upload_session = Some(session);
    }
}

impl Default for SessionProxy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_stats() {
        let mut stats = ConnectionStats::default();

        stats.record_success(100, 200, Duration::from_millis(100));
        stats.record_success(50, 100, Duration::from_millis(200));

        assert_eq!(stats.bytes_sent, 150);
        assert_eq!(stats.bytes_received, 300);
        assert_eq!(stats.connection_count, 2);
        assert_eq!(stats.avg_rtt, Duration::from_millis(150));
        assert!((stats.success_rate() - 1.0).abs() < f64::EPSILON);

        stats.record_failure();
        assert_eq!(stats.failure_count, 1);
        assert!((stats.success_rate() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_connection_creator() {
        let creator = ConnectionCreator::new();

        assert!(creator.network_flag());
        assert_eq!(creator.network_generation(), 0);

        creator.set_net_type(NetType::WiFi);
        assert_eq!(creator.net_type(), NetType::WiFi);

        creator.set_network_flag(false);
        assert!(!creator.network_flag());
        assert_eq!(creator.network_generation(), 1);
    }

    #[test]
    fn test_session() {
        let session = Session::new(2, DcId::internal(2), true, false, true, false);

        assert_eq!(session.raw_dc_id(), 2);
        assert_eq!(session.dc_id(), DcId::internal(2));
        assert!(session.is_primary());
        assert!(!session.is_main());
        assert!(session.use_pfs());
        assert!(!session.is_cdn());
    }

    #[test]
    fn test_session_proxy() {
        let mut proxy = SessionProxy::new();

        assert!(proxy.main_session().is_none());
        assert!(proxy.download_session().is_none());
        assert!(proxy.upload_session().is_none());

        let main = Session::new(1, DcId::internal(1), true, true, true, false);
        proxy.set_main_session(main);

        assert!(proxy.main_session().is_some());
        assert!(proxy.main_session().unwrap().is_main());
    }
}
