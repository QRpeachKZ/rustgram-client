// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto session management.
//!
//! This module implements TDLib's session system for managing MTProto connections.

mod connection;
mod handlers;
mod packets;
mod ping;
mod query;

pub use connection::{SessionConnection, SessionConnectionConfig, SessionEvent};
pub use handlers::{PacketHandler, PacketHandlerResult, ServicePacketHandler};
pub use packets::{ContainerDecoder, MessageContainer, ServicePacket};
pub use ping::{PingConfig, PingManager};
pub use query::{QueryLifecycle, QueryState};

/// Session state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Session is not initialized
    Empty,
    /// Session is connecting
    Connecting,
    /// Session is ready
    Ready,
    /// Session is closing
    Closing,
    /// Session is closed
    Closed,
}

impl Default for SessionState {
    fn default() -> Self {
        Self::Empty
    }
}

/// Session statistics.
#[derive(Debug, Clone, Default)]
pub struct SessionStatistics {
    /// Number of sent packets
    pub packets_sent: u64,

    /// Number of received packets
    pub packets_received: u64,

    /// Number of bytes sent
    pub bytes_sent: u64,

    /// Number of bytes received
    pub bytes_received: u64,

    /// Number of successful queries
    pub successful_queries: u64,

    /// Number of failed queries
    pub failed_queries: u64,

    /// Current ping time in milliseconds
    pub ping_ms: Option<u64>,
}
