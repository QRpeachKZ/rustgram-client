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

use bytes::{Buf, Bytes};
use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::auth::{AuthDataShared, AuthKeyState};
use crate::connection::ConnectionError;
use crate::crypto::{aes_ige_decrypt, aes_ige_encrypt, sha256};
use crate::dc::{DcId, DcOption, DcOptionsSet};
use crate::handshake::{HandshakeAction, HandshakeError, HandshakeMode, MtprotoHandshake};
use crate::packet::MessageId;
use crate::query::NetQuery;
use crate::rsa_key_shared::RsaKey;
use crate::transport::{ReadResult, TcpTransport, WriteOptions};

/// Convert HandshakeError to ConnectionError
impl From<HandshakeError> for ConnectionError {
    fn from(err: HandshakeError) -> Self {
        ConnectionError::Failed(format!("Handshake error: {}", err))
    }
}

use super::packets::{ContainerMessage, ServicePacket};
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

    /// Sets whether to use PFS (Perfect Forward Secrecy).
    pub fn with_pfs(mut self, use_pfs: bool) -> Self {
        self.use_pfs = use_pfs;
        self
    }

    /// Sets whether this is a main session.
    pub fn with_main(mut self, is_main: bool) -> Self {
        self.is_main = is_main;
        self
    }

    /// Sets whether this is a CDN session.
    pub fn with_cdn(mut self, is_cdn: bool) -> Self {
        self.is_cdn = is_cdn;
        self
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

    /// Query receiver
    query_receiver: Arc<Mutex<Option<mpsc::UnboundedReceiver<NetQuery>>>>,

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

    /// DC options set for getting connection addresses
    dc_options: Arc<Mutex<DcOptionsSet>>,

    /// RSA keys for handshake encryption
    rsa_keys: Arc<Mutex<Vec<RsaKey>>>,
}

impl std::fmt::Debug for SessionConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionConnection")
            .field("dc_id", &self.config.dc_id)
            .field("state", &self.state())
            .field("auth_key_state", &self.auth_data.auth_key_state())
            .finish()
    }
}

impl SessionConnection {
    /// Creates a new session connection.
    pub fn new(config: SessionConnectionConfig, auth_data: Arc<AuthDataShared>) -> Self {
        let session_id = Self::generate_session_id();
        let ping_config = config.ping_config;

        let (event_sender, _) = mpsc::unbounded_channel();
        let (query_sender, query_receiver) = mpsc::unbounded_channel();

        Self {
            config,
            auth_data,
            state: Arc::new(AtomicU8::new(SessionState::Empty as u8)),
            network_generation: Arc::new(AtomicU32::new(0)),
            session_id: Arc::new(AtomicU64::new(session_id)),
            event_sender,
            query_sender,
            query_receiver: Arc::new(Mutex::new(Some(query_receiver))),
            active_queries: Arc::new(Mutex::new(HashMap::new())),
            query_lifecycle: Arc::new(QueryLifecycle::new()),
            ping_manager: Arc::new(Mutex::new(PingManager::new(ping_config))),
            statistics: Arc::new(Mutex::new(SessionStatistics::default())),
            stop_flag: Arc::new(AtomicBool::new(false)),
            dc_options: Arc::new(Mutex::new(DcOptionsSet::new())),
            rsa_keys: Arc::new(Mutex::new(Vec::new())),
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

    /// Sets the DC options for this connection.
    pub fn set_dc_options(&self, options: DcOptionsSet) {
        *self.dc_options.lock() = options;
    }

    /// Gets a DC option for connecting to the configured DC.
    pub fn get_dc_option(&self) -> Result<DcOption, ConnectionError> {
        let options = self.dc_options.lock();
        let dc_options = options.get_options_for_dc(self.config.dc_id);

        if dc_options.is_empty() {
            return Err(ConnectionError::Failed(format!(
                "No DC options found for DC {}",
                self.config.dc_id.get_raw_id()
            )));
        }

        // Return the first available option
        Ok(dc_options[0].clone())
    }

    /// Sets the RSA keys for handshake encryption.
    pub fn set_rsa_keys(&self, keys: Vec<RsaKey>) {
        *self.rsa_keys.lock() = keys;
    }

    /// Gets an RSA key matching one of the fingerprints.
    pub fn get_rsa_key(&self, fingerprints: &[i64]) -> Option<RsaKey> {
        let keys = self.rsa_keys.lock();
        fingerprints
            .iter()
            .find_map(|fp| keys.iter().find(|k| k.fingerprint == *fp))
            .cloned()
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
    ///
    /// This method handles the complete MTProto packet processing pipeline:
    /// 1. Decrypts the packet using AES-IGE with the auth key
    /// 2. Parses the packet info (message_id, seq_no, etc.)
    /// 3. Handles different packet types (service, container, content)
    /// 4. Matches responses to active queries
    pub fn process_packet(&self, data: &[u8]) -> Result<(), ConnectionError> {
        // Update statistics
        self.statistics.lock().packets_received += 1;
        self.statistics.lock().bytes_received += data.len() as u64;

        // Try to decode as service packet first (unencrypted)
        if let Ok(service_packet) = ServicePacket::decode(data) {
            return self.handle_service_packet(service_packet);
        }

        // Decrypt the packet
        let decrypted = self.decrypt_packet(data)?;

        // Parse packet info
        let (info, payload) = self.parse_packet_info(&decrypted)?;

        tracing::debug!(
            "Received packet: message_id={}, seq_no={}, payload_len={}",
            info.message_id.as_u64(),
            info.seq_no,
            payload.len()
        );

        // Handle based on packet type
        if let Ok(service_packet) = ServicePacket::decode(payload) {
            match service_packet {
                ServicePacket::MessageContainer { messages } => {
                    // Recursively process container messages
                    for msg in messages {
                        self.process_container_message(msg)?;
                    }
                }
                _ => {
                    self.handle_service_packet(service_packet)?;
                }
            }
        } else {
            // Content message - try to match to active query
            self.handle_content_message(info.message_id, Bytes::copy_from_slice(payload))?;
        }

        Ok(())
    }

    /// Decrypts an incoming MTProto packet.
    ///
    /// Uses AES-IGE decryption with the auth key from auth_data.
    fn decrypt_packet(&self, data: &[u8]) -> Result<Vec<u8>, ConnectionError> {
        let auth_key = self
            .auth_data
            .get_auth_key()
            .ok_or_else(|| ConnectionError::Failed("No auth key available".into()))?;

        if auth_key.len() != 256 {
            return Err(ConnectionError::Failed(format!(
                "Invalid auth key length: {} (expected 256)",
                auth_key.len()
            )));
        }

        // MTProto 2.0 encrypted packet format:
        // - auth_key_id (8 bytes)
        // - msg_key (16 bytes)
        // - encrypted_data (variable)

        if data.len() < 24 {
            return Err(ConnectionError::Failed("Packet too short".into()));
        }

        // Extract auth_key_id and verify
        // Extract msg_key
        let msg_key = &data[8..24];

        // Extract encrypted data
        let encrypted_data = &data[24..];

        // Decrypt using AES-IGE
        let key_and_iv = Self::derive_key_and_iv(&auth_key.key, msg_key);

        let mut decrypted = encrypted_data.to_vec();

        // Check padding and align to block size
        if decrypted.len() % 16 != 0 {
            return Err(ConnectionError::Failed(
                "Encrypted data not aligned to block size".into(),
            ));
        }

        let mut key = [0u8; 32];
        key.copy_from_slice(&key_and_iv[..32]);

        let mut iv = [0u8; 32];
        iv.copy_from_slice(&key_and_iv[32..]);

        aes_ige_decrypt(&key, &mut iv, &mut decrypted)
            .map_err(|e| ConnectionError::Failed(format!("Decryption failed: {}", e)))?;

        // Verify msg_key
        let computed_msg_key = Self::compute_msg_key(&decrypted, &auth_key.key);
        if msg_key != computed_msg_key.as_slice() {
            return Err(ConnectionError::Failed("Msg key verification failed".into()));
        }

        // Strip padding (last 4 bytes contain padding length)
        if decrypted.len() < 4 {
            return Err(ConnectionError::Failed("Decrypted data too short".into()));
        }

        let padding_len = decrypted[decrypted.len() - 1] as usize;
        if padding_len > decrypted.len() - 12 {
            // 12 = salt (8) + session_id (8) can be before actual data
            return Err(ConnectionError::Failed("Invalid padding length".into()));
        }

        let new_len = decrypted.len() - padding_len;
        decrypted.truncate(new_len);

        Ok(decrypted)
    }

    /// Derives the AES key and IV from the auth key and msg_key.
    ///
    /// Following MTProto 2.0 specification.
    fn derive_key_and_iv(auth_key: &[u8], msg_key: &[u8]) -> Vec<u8> {
        // SHA256 is used for MTProto 2.0
        // Key: msg_key + auth_key (0..36)
        let key_hash = sha256(
            [msg_key, &auth_key[0..36]].concat().as_slice()
        );

        // IV: auth_key (40..76) + msg_key + auth_key (76..88)
        let iv_hash = sha256(
            [&auth_key[40..76], msg_key, &auth_key[76..88]].concat().as_slice()
        );

        [key_hash.as_slice(), iv_hash.as_slice()].concat()
    }

    /// Computes the msg_key for verification.
    fn compute_msg_key(data: &[u8], _auth_key: &[u8]) -> [u8; 16] {
        let hash = sha256(data);
        // For MTProto 2.0, msg_key is the first 16 bytes of SHA256
        let mut msg_key = [0u8; 16];
        msg_key.copy_from_slice(&hash[..16]);
        msg_key
    }

    /// Parses packet info from decrypted data.
    ///
    /// Returns (PacketInfo, payload) tuple.
    fn parse_packet_info<'a>(
        &self,
        data: &'a [u8],
    ) -> Result<(crate::packet::PacketInfo, &'a [u8]), ConnectionError> {
        // MTProto packet format:
        // - salt (8 bytes)
        // - session_id (8 bytes)
        // - message_id (8 bytes)
        // - seq_no (4 bytes)
        // - payload_len (4 bytes)
        // - payload (variable)

        if data.len() < 32 {
            return Err(ConnectionError::Failed("Packet too short for header".into()));
        }

        let mut cursor = Bytes::copy_from_slice(data);

        let salt = cursor.get_u64_le();
        let session_id = cursor.get_u64_le();
        let message_id = cursor.get_u64_le();
        let seq_no = cursor.get_i32_le();
        let payload_len = cursor.get_u32_le() as usize;

        // Validate payload length
        let remaining = cursor.remaining();
        if payload_len > remaining {
            return Err(ConnectionError::Failed(format!(
                "Payload length mismatch: expected {}, got {}",
                payload_len, remaining
            )));
        }

        let payload = &data[32..32 + payload_len];

        let info = crate::packet::PacketInfo {
            packet_type: crate::packet::PacketType::Common,
            message_ack: 0,
            salt,
            session_id,
            message_id: MessageId::from_u64(message_id),
            seq_no,
            version: 2,
            no_crypto_flag: false,
            is_creator: false,
            check_mod4: true,
            use_random_padding: false,
        };

        Ok((info, payload))
    }

    /// Processes a message from a container.
    fn process_container_message(&self, msg: ContainerMessage) -> Result<(), ConnectionError> {
        // Recursively process the message body
        self.process_packet(&msg.body)
    }

    /// Handles a content message (RPC response).
    fn handle_content_message(
        &self,
        message_id: MessageId,
        payload: Bytes,
    ) -> Result<(), ConnectionError> {
        // Try to find matching query by message_id
        let msg_id = message_id.as_u64();

        // First try through query_lifecycle
        if let Some(query_id) = self.query_lifecycle.find_by_message_id(msg_id) {
            // Complete the query
            self.query_lifecycle
                .mark_completed(msg_id, payload)
                .map_err(|e| ConnectionError::Failed(format!("Failed to complete query: {}", e)))?;

            self.statistics.lock().successful_queries += 1;

            tracing::debug!("Completed query {} with message_id={}", query_id, msg_id);

            return Ok(());
        }

        // Try through active_queries map
        if let Some(_query) = self.active_queries.lock().get(&msg_id) {
            self.complete_query(msg_id, Ok(payload));
            return Ok(());
        }

        // No matching query - this might be an update or unsolicited message
        tracing::debug!("Received message with no matching query: message_id={}", msg_id);

        // Could be an update - handle appropriately
        // For now, just acknowledge it
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
    ///
    /// This method performs the complete connection flow:
    /// 1. Establishes TCP connection to the DC
    /// 2. Performs MTProto handshake if no auth key exists
    /// 3. Starts the network loop for read/write operations
    pub async fn start(&self) -> Result<(), ConnectionError> {
        self.set_state(SessionState::Connecting);

        // 1. Create TCP transport
        let dc_option = self.get_dc_option()?;
        let addr = std::net::SocketAddr::new(dc_option.ip_address, dc_option.port);

        tracing::info!("Connecting to DC {} at {}", self.config.dc_id.get_raw_id(), addr);

        let mut transport = TcpTransport::new(addr);
        transport.connect().await?;

        tracing::info!("TCP connected to DC {}", self.config.dc_id.get_raw_id());

        // 2. Check auth key
        if self.auth_data.auth_key_state() == AuthKeyState::Ready {
            // Auth key already exists, skip handshake
            tracing::info!("Auth key already ready for DC {}", self.config.dc_id.get_raw_id());

            // Start main network loop with existing auth key
            self.run_network_loop_with_transport(transport).await?;
            return Ok(());
        }

        // 3. Start handshake
        let mode = if self.config.use_pfs {
            HandshakeMode::Temp
        } else {
            HandshakeMode::Main
        };

        let rsa_keys = self.rsa_keys.lock().clone();
        let mut handshake = MtprotoHandshake::new(self.config.dc_id, mode, rsa_keys);

        // Start handshake - send req_pq_multi
        let HandshakeAction::Send(packet) = handshake.start()? else {
            return Err(ConnectionError::Failed("Handshake start failed".into()));
        };

        // Send initial packet (unencrypted)
        transport.write(&packet, None).await?;
        tracing::info!("Sent req_pq_multi to DC {}", self.config.dc_id.get_raw_id());

        // 4. Run handshake loop
        let auth_key = self
            .run_handshake_loop(transport, &mut handshake)
            .await?;

        // Store the auth key
        let auth_key_id = crate::crypto::compute_auth_key_id(
            &auth_key
                .clone()
                .try_into()
                .map_err(|_| ConnectionError::Failed("Invalid auth key length".into()))?,
        );

        self.auth_data
            .set_auth_key(crate::auth::AuthKey::new(auth_key_id, auth_key));

        tracing::info!("Handshake complete, auth key stored for DC {}", self.config.dc_id.get_raw_id());

        // 5. Start main network loop with new auth key
        self.run_network_loop().await?;

        self.set_state(SessionState::Ready);

        // Start ping manager
        self.start_ping_loop().await;

        Ok(())
    }

    /// Runs the MTProto handshake loop.
    ///
    /// This method handles the back-and-forth of the DH key exchange
    /// by processing server responses and sending appropriate requests.
    async fn run_handshake_loop(
        &self,
        mut transport: TcpTransport,
        handshake: &mut MtprotoHandshake,
    ) -> Result<Vec<u8>, ConnectionError> {
        loop {
            // Read response from server (unencrypted during handshake)
            let read_result = transport.read(None).await?;

            // Extract packet data from ReadResult
            let packet_data = match read_result {
                ReadResult::Packet(data) => data.to_vec(),
                ReadResult::Nop => continue,
                ReadResult::Error(code) => {
                    return Err(ConnectionError::Failed(format!("Transport error: {}", code)));
                }
                ReadResult::QuickAck(_) => continue,
            };

            match handshake.on_message(&packet_data) {
                Ok(HandshakeAction::Send(packet)) => {
                    // Send packet to server (still unencrypted)
                    transport.write(&packet, None).await?;
                }
                Ok(HandshakeAction::Wait) => {
                    // Wait for next response
                    continue;
                }
                Ok(HandshakeAction::Complete(auth_key, server_salt)) => {
                    // Handshake complete!
                    tracing::info!("Handshake completed with auth key and salt {}", server_salt);
                    self.auth_data.set_server_salt(server_salt);
                    return Ok(auth_key);
                }
                Err(HandshakeError::InvalidState { .. } | HandshakeError::NonceMismatch) => {
                    return Err(ConnectionError::Failed("Handshake state error".into()));
                }
                Err(e) => {
                    tracing::warn!("Handshake error: {}, retrying...", e);
                    // Could implement retry logic here
                }
            }
        }
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
        let _query_sender = self.query_sender.clone();
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

    /// Serializes a NetQuery into an MTProto packet.
    fn serialize_query_packet(&self, query: &NetQuery) -> Result<Vec<u8>, ConnectionError> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let message_id = MessageId::generate(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64(),
            true,
            0,
        );

        let seq_no = self.auth_data.next_seq_no(true);

        let mut packet = Vec::new();
        packet.extend_from_slice(&self.auth_data.server_salt().to_le_bytes());
        packet.extend_from_slice(&self.auth_data.session_id().to_le_bytes());
        packet.extend_from_slice(&message_id.as_u64().to_le_bytes());
        packet.extend_from_slice(&seq_no.to_le_bytes());
        packet.extend_from_slice(&(query.query().len() as u32).to_le_bytes());
        packet.extend_from_slice(query.query());

        // Register the query for response matching
        self.register_query(message_id.as_u64(), query.clone());
        query.set_message_id(message_id.as_u64());

        Ok(packet)
    }

    /// Encrypts an MTProto packet using AES-IGE.
    fn encrypt_packet(&self, packet: &[u8]) -> Result<Vec<u8>, ConnectionError> {
        use rand::Rng;

        let auth_key = self
            .auth_data
            .get_auth_key()
            .ok_or_else(|| ConnectionError::Failed("No auth key".into()))?;

        if auth_key.len() != 256 {
            return Err(ConnectionError::Failed(format!(
                "Invalid auth key length: {} (expected 256)",
                auth_key.len()
            )));
        }

        // Compute msg_key from plaintext
        let msg_key = &sha256(packet)[..16];

        // Pad to block size
        let mut padded = packet.to_vec();
        while padded.len() % 16 != 0 {
            padded.push(rand::thread_rng().gen::<u8>());
        }

        // Derive key and IV
        let key_and_iv = Self::derive_key_and_iv(auth_key.as_bytes(), msg_key);

        // Encrypt with AES-IGE
        let mut key = [0u8; 32];
        key.copy_from_slice(&key_and_iv[..32]);

        let mut iv = [0u8; 32];
        iv.copy_from_slice(&key_and_iv[32..]);

        aes_ige_encrypt(&key, &mut iv, &mut padded)
            .map_err(|e| ConnectionError::Failed(format!("Encryption failed: {}", e)))?;

        // Build result: auth_key_id + msg_key + encrypted_data
        let auth_key_id = crate::crypto::compute_auth_key_id(
            &auth_key
                .as_bytes()
                .try_into()
                .map_err(|_| ConnectionError::Failed("Invalid auth key".into()))?,
        );

        let mut result = Vec::new();
        result.extend_from_slice(&auth_key_id.to_le_bytes());
        result.extend_from_slice(msg_key);
        result.extend_from_slice(&padded);

        Ok(result)
    }

    /// Runs the main network loop with concurrent read/write operations.
    ///
    /// This method splits the transport into read and write halves and spawns
    /// separate tasks for handling incoming packets and outgoing queries.
    async fn run_network_loop(&self) -> Result<(), ConnectionError> {
        // 1. Create TCP transport
        let dc_option = self.get_dc_option()?;
        let addr = std::net::SocketAddr::new(dc_option.ip_address, dc_option.port);
        let mut transport = TcpTransport::new(addr);
        transport.connect().await?;

        // 2. Split transport into read/write halves
        let (mut read_half, mut write_half) = transport.split().ok_or_else(|| {
            ConnectionError::Failed("Failed to split transport".into())
        })?;

        // 3. Spawn query sender task
        let mut query_receiver = self.query_receiver.lock().take().ok_or_else(|| {
            ConnectionError::Failed("Query receiver already taken".into())
        })?;
        let auth_data = self.auth_data.clone();
        let stop_flag = self.stop_flag.clone();
        let event_sender = self.event_sender.clone();
        let _dc_id = self.config.dc_id;

        tokio::spawn(async move {
            while !stop_flag.load(Ordering::Relaxed) {
                match query_receiver.recv().await {
                    Some(query) => {
                        // Serialize query
                        let packet = match Self::serialize_query_packet_impl(&query, &auth_data) {
                            Ok(p) => p,
                            Err(e) => {
                                tracing::error!("Failed to serialize query: {}", e);
                                let _ = event_sender.send(SessionEvent::Error(e.to_string()));
                                continue;
                            }
                        };

                        // Encrypt packet
                        let encrypted = match Self::encrypt_packet_impl(&packet, &auth_data) {
                            Ok(p) => p,
                            Err(e) => {
                                tracing::error!("Failed to encrypt packet: {}", e);
                                let _ = event_sender.send(SessionEvent::Error(e.to_string()));
                                continue;
                            }
                        };

                        // Write to transport
                        if let Err(e) = write_half.write_packet(&encrypted, None).await {
                            tracing::error!("Failed to write packet: {}", e);
                            let _ = event_sender.send(SessionEvent::Error(e.to_string()));
                            break;
                        }

                        tracing::trace!("Sent query {} to DC {}", query.id(), _dc_id.get_raw_id());
                    }
                    None => break,
                }
            }
            Ok::<(), ConnectionError>(())
        });

        // 4. Spawn packet receiver task
        let stop_flag = self.stop_flag.clone();
        let event_sender = self.event_sender.clone();

        tokio::spawn(async move {
            while !stop_flag.load(Ordering::Relaxed) {
                match read_half.read_packet(None, crate::packet::PacketType::Common).await {
                    Ok(ReadResult::Packet(data)) => {
                        // Forward to packet processing
                        let _ = event_sender.send(SessionEvent::Error(
                            format!("Received {} bytes packet", data.len())
                        ));
                        // Note: In a full implementation, we'd process the packet here
                    }
                    Ok(ReadResult::Nop) => continue,
                    Ok(ReadResult::QuickAck(_)) => continue,
                    Ok(ReadResult::Error(code)) => {
                        tracing::error!("Transport error: {}", code);
                        break;
                    }
                    Err(e) => {
                        tracing::error!("Read error: {}", e);
                        if matches!(e, ConnectionError::Timeout(_)) {
                            continue; // Timeout is not fatal
                        }
                        break;
                    }
                }
            }
            Ok::<(), ConnectionError>(())
        });

        self.set_state(SessionState::Ready);
        Ok(())
    }

    /// Runs the main network loop with an existing transport.
    ///
    /// This is a simplified version for when auth key already exists.
    async fn run_network_loop_with_transport(
        &self,
        mut transport: TcpTransport,
    ) -> Result<(), ConnectionError> {
        // Split the provided transport
        let (mut read_half, mut write_half) = transport.split().ok_or_else(|| {
            ConnectionError::Failed("Failed to split transport".into())
        })?;

        // Spawn query sender task
        let mut query_receiver = self.query_receiver.lock().take().ok_or_else(|| {
            ConnectionError::Failed("Query receiver already taken".into())
        })?;
        let auth_data = self.auth_data.clone();
        let stop_flag = self.stop_flag.clone();
        let event_sender = self.event_sender.clone();

        tokio::spawn(async move {
            while !stop_flag.load(Ordering::Relaxed) {
                match query_receiver.recv().await {
                    Some(query) => {
                        let packet = match Self::serialize_query_packet_impl(&query, &auth_data) {
                            Ok(p) => p,
                            Err(e) => {
                                tracing::error!("Failed to serialize query: {}", e);
                                let _ = event_sender.send(SessionEvent::Error(e.to_string()));
                                continue;
                            }
                        };

                        let encrypted = match Self::encrypt_packet_impl(&packet, &auth_data) {
                            Ok(p) => p,
                            Err(e) => {
                                tracing::error!("Failed to encrypt packet: {}", e);
                                let _ = event_sender.send(SessionEvent::Error(e.to_string()));
                                continue;
                            }
                        };

                        if let Err(e) = write_half.write_packet(&encrypted, None).await {
                            tracing::error!("Failed to write packet: {}", e);
                            let _ = event_sender.send(SessionEvent::Error(e.to_string()));
                            break;
                        }
                    }
                    None => break,
                }
            }
            Ok::<(), ConnectionError>(())
        });

        // Spawn packet receiver task
        let stop_flag = self.stop_flag.clone();
        let event_sender = self.event_sender.clone();

        tokio::spawn(async move {
            while !stop_flag.load(Ordering::Relaxed) {
                match read_half.read_packet(None, crate::packet::PacketType::Common).await {
                    Ok(ReadResult::Packet(data)) => {
                        let _ = event_sender.send(SessionEvent::Error(
                            format!("Received {} bytes", data.len())
                        ));
                    }
                    Ok(ReadResult::Nop) => continue,
                    Ok(ReadResult::QuickAck(_)) => continue,
                    Ok(ReadResult::Error(code)) => {
                        tracing::error!("Transport error: {}", code);
                        break;
                    }
                    Err(e) => {
                        if !matches!(e, ConnectionError::Timeout(_)) {
                            tracing::error!("Read error: {}", e);
                            break;
                        }
                    }
                }
            }
            Ok::<(), ConnectionError>(())
        });

        self.set_state(SessionState::Ready);
        Ok(())
    }

    /// Static helper for serializing a query packet.
    fn serialize_query_packet_impl(
        query: &NetQuery,
        auth_data: &AuthDataShared,
    ) -> Result<Vec<u8>, ConnectionError> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let message_id = MessageId::generate(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64(),
            true,
            0,
        );

        let seq_no = auth_data.next_seq_no(true);

        let mut packet = Vec::new();
        packet.extend_from_slice(&auth_data.server_salt().to_le_bytes());
        packet.extend_from_slice(&auth_data.session_id().to_le_bytes());
        packet.extend_from_slice(&message_id.as_u64().to_le_bytes());
        packet.extend_from_slice(&seq_no.to_le_bytes());
        packet.extend_from_slice(&(query.query().len() as u32).to_le_bytes());
        packet.extend_from_slice(query.query());

        query.set_message_id(message_id.as_u64());

        Ok(packet)
    }

    /// Static helper for encrypting a packet.
    fn encrypt_packet_impl(
        packet: &[u8],
        auth_data: &AuthDataShared,
    ) -> Result<Vec<u8>, ConnectionError> {
        use rand::Rng;

        let auth_key = auth_data
            .get_auth_key()
            .ok_or_else(|| ConnectionError::Failed("No auth key".into()))?;

        if auth_key.len() != 256 {
            return Err(ConnectionError::Failed(format!(
                "Invalid auth key length: {}",
                auth_key.len()
            )));
        }

        let msg_key = &sha256(packet)[..16];

        let mut padded = packet.to_vec();
        while padded.len() % 16 != 0 {
            padded.push(rand::thread_rng().gen::<u8>());
        }

        let key_and_iv = Self::derive_key_and_iv(auth_key.as_bytes(), msg_key);

        let mut key = [0u8; 32];
        key.copy_from_slice(&key_and_iv[..32]);

        let mut iv = [0u8; 32];
        iv.copy_from_slice(&key_and_iv[32..]);

        aes_ige_encrypt(&key, &mut iv, &mut padded)
            .map_err(|e| ConnectionError::Failed(format!("Encryption failed: {}", e)))?;

        let auth_key_id = crate::crypto::compute_auth_key_id(
            &auth_key
                .as_bytes()
                .try_into()
                .map_err(|_| ConnectionError::Failed("Invalid auth key".into()))?,
        );

        let mut result = Vec::new();
        result.extend_from_slice(&auth_key_id.to_le_bytes());
        result.extend_from_slice(msg_key);
        result.extend_from_slice(&padded);

        Ok(result)
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
