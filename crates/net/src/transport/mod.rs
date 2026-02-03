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

pub use header::{
    CryptoHeader, CryptoPrefix, EndToEndHeader, EndToEndPrefix, NoCryptoHeader, NoCryptoPrefix,
};
pub use http::{HttpTransport, HttpTransportFactory};
pub use http_proxy::{HttpProxyTransport, HttpProxyTransportFactory};
pub use mtproto_proxy::{MtprotoProxyTransport, MtprotoProxyTransportFactory};
pub use read::{ReadResult, TransportRead};
pub use socks5::{Socks5Transport, Socks5TransportFactory};
pub use tcp::{TcpReadHalf, TcpTransport, TcpTransportFactory, TcpWriteHalf, MAX_PACKET_SIZE};
pub use write::{TransportWrite, WriteOptions};

/// Magic number for Intermediate transport mode (without padding)
///
/// TDLib sends this at the start of a connection to indicate the transport mode.
/// See `td/mtproto/TcpTransport.cpp:76`
pub const INTERMEDIATE_MAGIC: u32 = 0xeeeeeeee;

/// Magic number for Intermediate transport mode with padding
pub const INTERMEDIATE_MAGIC_PADDED: u32 = 0xdddddddd;

/// Magic number for Abridged transport mode
///
/// Abridged mode starts with this single byte to indicate the transport mode.
/// See https://core.telegram.org/mtproto/mtproto-transports#abridged
pub const ABRIDGED_MAGIC: u8 = 0xef;

/// Returns the initial magic bytes for the given transport mode.
///
/// For Intermediate mode, this returns a 4-byte magic number that must be
/// sent at the start of the connection. For other modes, returns an empty vec.
///
/// # Arguments
///
/// * `mode` - Transport mode to get magic for
///
/// # Returns
///
/// Magic bytes to send at connection start.
///
/// # Examples
///
/// ```
/// use rustgram_net::transport::{get_transport_magic, TransportMode};
///
/// // Intermediate mode has a magic number
/// let magic = get_transport_magic(TransportMode::Intermediate);
/// assert_eq!(magic, vec![0xEE, 0xEE, 0xEE, 0xEE]);
///
/// // Abridged mode also has a magic number
/// let magic = get_transport_magic(TransportMode::Abridged);
/// assert_eq!(magic, vec![0xEF]);
/// ```
pub fn get_transport_magic(mode: TransportMode) -> Vec<u8> {
    match mode {
        TransportMode::Intermediate => INTERMEDIATE_MAGIC.to_le_bytes().to_vec(),
        TransportMode::Abridged => vec![ABRIDGED_MAGIC],
        TransportMode::NoCrypto | TransportMode::Full => vec![],
    }
}

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
    /// Creates a new transport with default (Intermediate) mode.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mode: TransportMode::Intermediate,
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
#[derive(Default)]
pub enum TransportMode {
    /// No encryption (used during initial handshake)
    NoCrypto = 0,

    /// Abridged mode: 1-byte length prefix for packets < 127 bytes
    Abridged = 1,

    /// Intermediate mode: 4-byte length prefix (TDLib default)
    #[default]
    Intermediate = 2,

    /// Full mode: Encrypted with AES-IGE
    Full = 3,
}

/// Encodes a packet length for the given transport mode.
///
/// # Arguments
///
/// * `mode` - Transport mode to use for encoding
/// * `length` - Packet length in bytes
///
/// # Returns
///
/// A vector containing the encoded length prefix.
///
/// # Errors
///
/// Panics in abridged mode if length >= 127.
///
/// # Examples
///
/// ```
/// use rustgram_net::transport::{encode_length, TransportMode};
///
/// // Abridged mode: single byte for lengths < 127 (with left shift)
/// let enc = encode_length(TransportMode::Abridged, 20);
/// assert_eq!(enc, vec![0x28]); // 20 << 1 = 40 = 0x28
///
/// // Intermediate mode: 4-byte little-endian
/// let enc = encode_length(TransportMode::Intermediate, 28);
/// assert_eq!(enc, vec![0x1C, 0x00, 0x00, 0x00]);
/// ```
pub fn encode_length(mode: TransportMode, length: usize) -> Vec<u8> {
    match mode {
        TransportMode::Abridged => {
            if length >= 0x7f {
                panic!("Abridged mode max length is 126, got {}", length);
            }
            // Abridged mode: encode length with left shift: (length << 1)
            vec![(length as u8) << 1]
        }
        TransportMode::Intermediate => {
            (length as u32).to_le_bytes().to_vec()
        }
        TransportMode::NoCrypto | TransportMode::Full => {
            // These modes don't use transport-level framing
            vec![]
        }
    }
}

/// Wraps an MTProto packet with transport-level framing.
///
/// Adds the appropriate length prefix based on the transport mode.
///
/// # Arguments
///
/// * `mode` - Transport mode to use for framing
/// * `packet` - Raw MTProto packet bytes
///
/// # Returns
///
/// A vector containing the framed packet ready to send over the network.
///
/// # Examples
///
/// ```
/// use rustgram_net::transport::{frame_packet, TransportMode};
///
/// let packet = vec![0x00, 0x01, 0x02, 0x03];
/// let framed = frame_packet(TransportMode::Abridged, &packet);
/// // Result: [0x08, 0x00, 0x01, 0x02, 0x03] (length << 1)
/// assert_eq!(framed[0], 0x08); // Length prefix (4 << 1 = 8)
/// assert_eq!(&framed[1..], &packet[..]);
/// ```
pub fn frame_packet(mode: TransportMode, packet: &[u8]) -> Vec<u8> {
    let mut framed = encode_length(mode, packet.len());
    framed.extend_from_slice(packet);
    framed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_default() {
        let transport = Transport::default();
        assert_eq!(transport.mode(), TransportMode::Intermediate);
    }

    #[test]
    fn test_transport_new() {
        let transport = Transport::new();
        assert_eq!(transport.mode(), TransportMode::Intermediate);
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
        assert_eq!(TransportMode::default(), TransportMode::Intermediate);
    }

    #[test]
    fn test_get_transport_magic_intermediate() {
        let magic = get_transport_magic(TransportMode::Intermediate);
        assert_eq!(magic, vec![0xEE, 0xEE, 0xEE, 0xEE]);
    }

    #[test]
    fn test_get_transport_magic_abridged() {
        let magic = get_transport_magic(TransportMode::Abridged);
        assert_eq!(magic, vec![0xEF]);
    }

    #[test]
    fn test_get_transport_magic_no_crypto() {
        let magic = get_transport_magic(TransportMode::NoCrypto);
        assert_eq!(magic, vec![]);
    }

    #[test]
    fn test_get_transport_magic_full() {
        let magic = get_transport_magic(TransportMode::Full);
        assert_eq!(magic, vec![]);
    }

    #[test]
    fn test_transport_mode_values() {
        assert_eq!(TransportMode::NoCrypto as u8, 0);
        assert_eq!(TransportMode::Abridged as u8, 1);
        assert_eq!(TransportMode::Intermediate as u8, 2);
        assert_eq!(TransportMode::Full as u8, 3);
    }

    #[test]
    fn test_encode_length_abridged() {
        // Abridged mode: length << 1
        let enc = encode_length(TransportMode::Abridged, 20);
        assert_eq!(enc, vec![0x28]); // 20 << 1 = 40 = 0x28

        let enc = encode_length(TransportMode::Abridged, 0);
        assert_eq!(enc, vec![0x00]);

        let enc = encode_length(TransportMode::Abridged, 126);
        assert_eq!(enc, vec![0xFC]); // 126 << 1 = 252 = 0xFC
    }

    #[test]
    fn test_encode_length_intermediate() {
        let enc = encode_length(TransportMode::Intermediate, 28);
        assert_eq!(enc, vec![0x1C, 0x00, 0x00, 0x00]);

        let enc = encode_length(TransportMode::Intermediate, 0x12345678);
        assert_eq!(enc, vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_encode_length_no_crypto() {
        let enc = encode_length(TransportMode::NoCrypto, 28);
        assert_eq!(enc, vec![]);

        let enc = encode_length(TransportMode::Full, 100);
        assert_eq!(enc, vec![]);
    }

    #[test]
    #[should_panic(expected = "Abridged mode max length is 126")]
    fn test_encode_length_abridged_too_large() {
        encode_length(TransportMode::Abridged, 127);
    }

    #[test]
    fn test_frame_packet_abridged() {
        let packet = vec![0x00, 0x01, 0x02, 0x03];
        let framed = frame_packet(TransportMode::Abridged, &packet);
        assert_eq!(framed, vec![0x08, 0x00, 0x01, 0x02, 0x03]); // length 4 << 1 = 8 = 0x08
    }

    #[test]
    fn test_frame_packet_intermediate() {
        let packet = vec![0xAB, 0xCD];
        let framed = frame_packet(TransportMode::Intermediate, &packet);
        assert_eq!(framed, vec![0x02, 0x00, 0x00, 0x00, 0xAB, 0xCD]);
    }

    #[test]
    fn test_frame_packet_empty() {
        let packet = vec![];
        let framed = frame_packet(TransportMode::Abridged, &packet);
        assert_eq!(framed, vec![0x00]);
    }
}
