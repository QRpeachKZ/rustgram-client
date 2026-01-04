// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Ping management for MTProto sessions.
//!
//! This module implements ping/pong handling for connection health monitoring.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use rand::Rng;

/// Ping configuration.
#[derive(Debug, Clone, Copy)]
pub struct PingConfig {
    /// Ping interval
    pub ping_interval: Duration,

    /// Ping timeout
    pub ping_timeout: Duration,

    /// Maximum failed pings before disconnecting
    pub max_failed_pings: usize,
}

impl Default for PingConfig {
    fn default() -> Self {
        Self {
            ping_interval: Duration::from_secs(15),
            ping_timeout: Duration::from_secs(10),
            max_failed_pings: 3,
        }
    }
}

/// Ping request.
#[derive(Debug, Clone)]
pub struct PingRequest {
    /// Unique ping ID
    pub ping_id: u64,

    /// Timestamp when ping was sent
    pub sent_at: Instant,

    /// Ping type
    pub ping_type: PingType,
}

/// Ping type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PingType {
    /// Regular ping
    Regular,

    /// Delay disconnect ping (PingDelayDisconnect)
    DelayDisconnect(u32), // delay in seconds

    /// Pings container (multiple pings)
    PingsContainer,
}

/// Ping manager.
///
/// Manages ping/pong messages for connection health monitoring.
/// Based on TDLib's PingManager from `td/telegram/net/Ping.cpp`.
pub struct PingManager {
    /// Configuration
    config: PingConfig,

    /// Next ping ID
    next_ping_id: AtomicU64,

    /// Active pings (ping_id -> PingRequest)
    active_pings: Mutex<HashMap<u64, PingRequest>>,

    /// Failed ping count
    failed_pings: Mutex<usize>,

    /// Last successful ping time
    last_success: Mutex<Option<Instant>>,

    /// Current ping time in milliseconds
    current_ping_ms: Mutex<Option<u64>>,
}

impl PingManager {
    /// Creates a new ping manager.
    pub fn new(config: PingConfig) -> Self {
        Self {
            config,
            next_ping_id: AtomicU64::new(1),
            active_pings: Mutex::new(HashMap::new()),
            failed_pings: Mutex::new(0),
            last_success: Mutex::new(None),
            current_ping_ms: Mutex::new(None),
        }
    }

    /// Creates a new ping request.
    pub fn create_ping(&self) -> Option<PingRequest> {
        let ping_id = self.next_ping_id.fetch_add(1, Ordering::Relaxed);

        let request = PingRequest {
            ping_id,
            sent_at: Instant::now(),
            ping_type: PingType::Regular,
        };

        self.active_pings.lock().insert(ping_id, request.clone());

        Some(request)
    }

    /// Creates a delay disconnect ping.
    pub fn create_delay_disconnect_ping(&self, delay: u32) -> Option<PingRequest> {
        let ping_id = self.next_ping_id.fetch_add(1, Ordering::Relaxed);

        let request = PingRequest {
            ping_id,
            sent_at: Instant::now(),
            ping_type: PingType::DelayDisconnect(delay),
        };

        self.active_pings.lock().insert(ping_id, request.clone());

        Some(request)
    }

    /// Creates a pings container (multiple pings).
    pub fn create_pings_container(&self, count: usize) -> Vec<PingRequest> {
        let mut requests = Vec::with_capacity(count);

        for _ in 0..count {
            if let Some(request) = self.create_ping() {
                requests.push(request);
            }
        }

        requests
    }

    /// Handles a pong response.
    pub fn on_pong(&self, ping_id: u64) {
        let mut pings = self.active_pings.lock();

        if let Some(request) = pings.remove(&ping_id) {
            let rtt = request.sent_at.elapsed();

            *self.current_ping_ms.lock() = Some(rtt.as_millis() as u64);
            *self.last_success.lock() = Some(Instant::now());
            *self.failed_pings.lock() = 0;

            tracing::trace!("Pong received: ping_id={}, rtt={:?}", ping_id, rtt);
        }
    }

    /// Checks for timed out pings.
    pub fn check_timeouts(&self) -> usize {
        let mut pings = self.active_pings.lock();
        let mut timed_out = Vec::new();
        let now = Instant::now();

        for (&ping_id, request) in pings.iter() {
            if now.duration_since(request.sent_at) > self.config.ping_timeout {
                timed_out.push(ping_id);
            }
        }

        let count = timed_out.len();

        for ping_id in &timed_out {
            pings.remove(ping_id);
        }

        if count > 0 {
            *self.failed_pings.lock() += count;
            tracing::warn!("{} ping(s) timed out", count);
        }

        count
    }

    /// Returns `true` if the connection should be disconnected due to failed pings.
    pub fn should_disconnect(&self) -> bool {
        *self.failed_pings.lock() >= self.config.max_failed_pings
    }

    /// Returns the current ping time in milliseconds.
    pub fn ping_ms(&self) -> Option<u64> {
        *self.current_ping_ms.lock()
    }

    /// Returns the last successful ping time.
    pub fn last_success(&self) -> Option<Instant> {
        *self.last_success.lock()
    }

    /// Returns the number of active pings.
    pub fn active_count(&self) -> usize {
        self.active_pings.lock().len()
    }

    /// Returns the number of failed pings.
    pub fn failed_count(&self) -> usize {
        *self.failed_pings.lock()
    }

    /// Clears all active pings.
    pub fn clear(&self) {
        self.active_pings.lock().clear();
        *self.failed_pings.lock() = 0;
        *self.current_ping_ms.lock() = None;
        *self.last_success.lock() = None;
    }

    /// Generates a random ping ID.
    pub fn random_ping_id() -> u64 {
        rand::thread_rng().gen()
    }
}

impl Default for PingManager {
    fn default() -> Self {
        Self::new(PingConfig::default())
    }
}

/// Ping utility functions.
impl PingManager {
    /// Creates the TL bytes for a ping request.
    pub fn encode_ping(ping_id: u64) -> Vec<u8> {
        // ping#7abe77ec ping_id:long = Pong;
        let constructor = 0x7abe77ecu32;

        let mut result = Vec::with_capacity(12);
        result.extend_from_slice(&constructor.to_le_bytes());
        result.extend_from_slice(&ping_id.to_le_bytes());

        result
    }

    /// Creates the TL bytes for a ping_delay_disconnect request.
    pub fn encode_ping_delay_disconnect(ping_id: u64, delay: u32) -> Vec<u8> {
        // ping_delay_disconnect#34a27b63 ping_id:long disconnect_delay:int = Pong;
        let constructor = 0x34a27b63u32;

        let mut result = Vec::with_capacity(16);
        result.extend_from_slice(&constructor.to_le_bytes());
        result.extend_from_slice(&ping_id.to_le_bytes());
        result.extend_from_slice(&delay.to_le_bytes());

        result
    }

    /// Decodes a pong response.
    pub fn decode_pong(data: &[u8]) -> Option<(u64, Option<u32>)> {
        // pong#2b0f7de3 ping_id:long = Pong;
        const PONG_CONSTRUCTOR: u32 = 0x2b0f7de3;

        if data.len() < 12 {
            return None;
        }

        let constructor = u32::from_le_bytes(data[0..4].try_into().ok()?);

        if constructor == PONG_CONSTRUCTOR {
            let ping_id = u64::from_le_bytes(data[4..12].try_into().ok()?);
            Some((ping_id, None))
        } else {
            None
        }
    }

    /// Decodes a pong_delay_disconnect response.
    pub fn decode_pong_delay_disconnect(data: &[u8]) -> Option<(u64, u32)> {
        // pong_delay_disconnect#f3427b8c ping_id:long = Pong;
        const PONG_DELAY_CONSTRUCTOR: u32 = 0xf3427b8c;

        if data.len() < 12 {
            return None;
        }

        let constructor = u32::from_le_bytes(data[0..4].try_into().ok()?);

        if constructor == PONG_DELAY_CONSTRUCTOR {
            let ping_id = u64::from_le_bytes(data[4..12].try_into().ok()?);
            Some((ping_id, 0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_config_default() {
        let config = PingConfig::default();
        assert_eq!(config.ping_interval, Duration::from_secs(15));
        assert_eq!(config.ping_timeout, Duration::from_secs(10));
        assert_eq!(config.max_failed_pings, 3);
    }

    #[test]
    fn test_ping_manager_new() {
        let config = PingConfig::default();
        let manager = PingManager::new(config);

        assert_eq!(manager.active_count(), 0);
        assert_eq!(manager.failed_count(), 0);
        assert!(manager.ping_ms().is_none());
        assert!(manager.last_success().is_none());
    }

    #[test]
    fn test_ping_manager_create_ping() {
        let manager = PingManager::default();

        let ping = manager.create_ping();
        assert!(ping.is_some());

        if let Some(ping) = ping {
            assert_eq!(ping.ping_type, PingType::Regular);
            assert_eq!(manager.active_count(), 1);
        }
    }

    #[test]
    fn test_ping_manager_pong() {
        let manager = PingManager::default();

        if let Some(ping) = manager.create_ping() {
            manager.on_pong(ping.ping_id);

            assert_eq!(manager.active_count(), 0);
            assert!(manager.ping_ms().is_some());
            assert_eq!(manager.failed_count(), 0);
        }
    }

    #[test]
    fn test_ping_manager_should_disconnect() {
        let config = PingConfig {
            max_failed_pings: 2,
            ..Default::default()
        };

        let manager = PingManager::new(config);

        assert!(!manager.should_disconnect());

        manager.check_timeouts(); // No active pings, no timeout
        assert!(!manager.should_disconnect());

        // Simulate failed pings
        *manager.failed_pings.lock() = 2;
        assert!(manager.should_disconnect());
    }

    #[test]
    fn test_ping_manager_clear() {
        let manager = PingManager::default();

        manager.create_ping();
        manager.clear();

        assert_eq!(manager.active_count(), 0);
        assert_eq!(manager.failed_count(), 0);
        assert!(manager.ping_ms().is_none());
    }

    #[test]
    fn test_encode_ping() {
        let ping_id = 0x123456789ABCDEF0;
        let encoded = PingManager::encode_ping(ping_id);

        assert_eq!(encoded.len(), 12);
        let bytes: [u8; 4] = encoded[0..4]
            .try_into()
            .expect("should have 4 bytes for constructor");
        assert_eq!(u32::from_le_bytes(bytes), 0x7abe77ec);
        let bytes: [u8; 8] = encoded[4..12]
            .try_into()
            .expect("should have 8 bytes for ping_id");
        assert_eq!(u64::from_le_bytes(bytes), ping_id);
    }

    #[test]
    fn test_encode_ping_delay_disconnect() {
        let ping_id = 0x123456789ABCDEF0;
        let delay = 60;
        let encoded = PingManager::encode_ping_delay_disconnect(ping_id, delay);

        assert_eq!(encoded.len(), 16);
        let bytes: [u8; 4] = encoded[0..4]
            .try_into()
            .expect("should have 4 bytes for constructor");
        assert_eq!(u32::from_le_bytes(bytes), 0x34a27b63);
        let bytes: [u8; 8] = encoded[4..12]
            .try_into()
            .expect("should have 8 bytes for ping_id");
        assert_eq!(u64::from_le_bytes(bytes), ping_id);
        let bytes: [u8; 4] = encoded[12..16]
            .try_into()
            .expect("should have 4 bytes for delay");
        assert_eq!(u32::from_le_bytes(bytes), delay);
    }

    #[test]
    fn test_decode_pong() {
        let ping_id: u64 = 0x123456789ABCDEF0;

        let mut encoded = Vec::new();
        encoded.extend_from_slice(&0x2b0f7de3u32.to_le_bytes()); // pong constructor
        encoded.extend_from_slice(&ping_id.to_le_bytes());

        let decoded = PingManager::decode_pong(&encoded);
        assert!(decoded.is_some());

        if let Some((decoded_ping_id, _)) = decoded {
            assert_eq!(decoded_ping_id, ping_id);
        }
    }

    #[test]
    fn test_random_ping_id() {
        let id1 = PingManager::random_ping_id();
        let id2 = PingManager::random_ping_id();

        // They should be different with high probability
        assert_ne!(id1, id2);
    }
}
