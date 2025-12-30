// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! MTProto Transport Layer.
//!
//! This module implements the MTProto transport layer based on TDLib's
//! `td/mtproto/Transport.h` and `td/mtproto/Transport.cpp`.
//!
//! # Overview
//!
//! The transport layer handles encoding and decoding of MTProto packets
//! in different modes:
//! - **No Crypto**: Unencrypted packets (used during handshake)
//! - **Abridged**: Compact encoding for small packets
//! - **Intermediate**: 4-byte length prefix
//! - **Full**: Encrypted with AES-IGE (MTProto 2.0)
//!
//! # References
//!
//! - TDLib: `td/mtproto/Transport.h`, `td/mtproto/Transport.cpp`
//! - MTProto 2.0: <https://core.telegram.org/mtproto/description>

mod header;
mod http;
mod http_proxy;
mod mtproto_proxy;
mod read;
mod socks5;
mod tcp;
mod write;

pub use header::{CryptoHeader, CryptoPrefix, EndToEndHeader, EndToEndPrefix, NoCryptoHeader};
pub use http::{HttpTransport, HttpTransportFactory};
pub use http_proxy::{HttpProxyTransport, HttpProxyTransportFactory};
pub use mtproto_proxy::{MtprotoProxyTransport, MtprotoProxyTransportFactory};
pub use read::{ReadResult, TransportRead};
pub use socks5::{Socks5Transport, Socks5TransportFactory};
pub use tcp::{TcpReadHalf, TcpTransport, TcpTransportFactory, TcpWriteHalf, MAX_PACKET_SIZE};
pub use write::{TransportWrite, WriteOptions};

/// MTProto transport layer.
///
/// Provides methods for reading and writing MTProto packets in various
/// transport modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transport {
    /// Transport mode
    mode: TransportMode,
}

impl Default for Transport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport {
    /// Creates a new transport with default (abridged) mode.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mode: TransportMode::Abridged,
        }
    }

    /// Creates a new transport with the specified mode.
    #[must_use]
    pub const fn with_mode(mode: TransportMode) -> Self {
        Self { mode }
    }

    /// Returns the current transport mode.
    #[must_use]
    pub const fn mode(&self) -> TransportMode {
        self.mode
    }

    /// Sets the transport mode.
    pub fn set_mode(&mut self, mode: TransportMode) {
        self.mode = mode;
    }
}

/// MTProto transport mode.
///
/// Defines how packets are encoded and decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TransportMode {
    /// No encryption (used during initial handshake)
    NoCrypto = 0,

    /// Abridged mode: 1-byte length prefix for packets < 127 bytes
    Abridged = 1,

    /// Intermediate mode: 4-byte length prefix
    Intermediate = 2,

    /// Full mode: Encrypted with AES-IGE
    Full = 3,
}

impl Default for TransportMode {
    fn default() -> Self {
        Self::Abridged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_default() {
        let transport = Transport::default();
        assert_eq!(transport.mode(), TransportMode::Abridged);
    }

    #[test]
    fn test_transport_new() {
        let transport = Transport::new();
        assert_eq!(transport.mode(), TransportMode::Abridged);
    }

    #[test]
    fn test_transport_with_mode() {
        let transport = Transport::with_mode(TransportMode::Full);
        assert_eq!(transport.mode(), TransportMode::Full);
    }

    #[test]
    fn test_transport_set_mode() {
        let mut transport = Transport::new();
        transport.set_mode(TransportMode::Intermediate);
        assert_eq!(transport.mode(), TransportMode::Intermediate);
    }

    #[test]
    fn test_transport_mode_default() {
        assert_eq!(TransportMode::default(), TransportMode::Abridged);
    }

    #[test]
    fn test_transport_mode_values() {
        assert_eq!(TransportMode::NoCrypto as u8, 0);
        assert_eq!(TransportMode::Abridged as u8, 1);
        assert_eq!(TransportMode::Intermediate as u8, 2);
        assert_eq!(TransportMode::Full as u8, 3);
    }
}
