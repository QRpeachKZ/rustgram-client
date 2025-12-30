// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Packet information for MTProto protocol.
//!
//! Based on TDLib's `td/mtproto/PacketInfo.h`.

use crate::packet::MessageId;

/// Type of MTProto packet.
///
/// Distinguishes between common (server-client) packets and
/// end-to-end encrypted packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PacketType {
    /// Common packet (encrypted with server auth key).
    Common = 0,
    /// End-to-end encrypted packet.
    EndToEnd = 1,
}

impl Default for PacketType {
    fn default() -> Self {
        Self::Common
    }
}

/// MTProto packet metadata.
///
/// Contains all the metadata fields needed for MTProto packet processing,
/// including salt, session ID, message ID, sequence number, and various flags.
///
/// # References
///
/// - TDLib: `td/mtproto/PacketInfo.h`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketInfo {
    /// Packet type (common or end-to-end).
    pub packet_type: PacketType,

    /// Message acknowledgment flag (bit 31 set).
    pub message_ack: u32,

    /// Server salt for key derivation.
    pub salt: u64,

    /// Session identifier.
    pub session_id: u64,

    /// Message ID (time-based, modulo 2^32).
    pub message_id: MessageId,

    /// Sequence number for message ordering.
    pub seq_no: i32,

    /// MTProto version (1 = SHA1-based, 2 = SHA256-based).
    pub version: i32,

    /// If true, packet is not encrypted (used during initial handshake).
    pub no_crypto_flag: bool,

    /// For end-to-end packets: whether this side is the creator.
    pub is_creator: bool,

    /// If true, check that message length is divisible by 4.
    pub check_mod4: bool,

    /// If true, use random padding size (MTProto 2.0).
    pub use_random_padding: bool,
}

impl Default for PacketInfo {
    fn default() -> Self {
        Self {
            packet_type: PacketType::default(),
            message_ack: 0,
            salt: 0,
            session_id: 0,
            message_id: MessageId::default(),
            seq_no: 0,
            version: 2,
            no_crypto_flag: false,
            is_creator: false,
            check_mod4: true,
            use_random_padding: false,
        }
    }
}

impl PacketInfo {
    /// Creates a new default `PacketInfo`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            packet_type: PacketType::Common,
            message_ack: 0,
            salt: 0,
            session_id: 0,
            message_id: MessageId::EMPTY,
            seq_no: 0,
            version: 2,
            no_crypto_flag: false,
            is_creator: false,
            check_mod4: true,
            use_random_padding: false,
        }
    }

    /// Creates a `PacketInfo` for common (non-e2e) packets.
    #[must_use]
    pub fn common() -> Self {
        Self::new()
    }

    /// Creates a `PacketInfo` for end-to-end encrypted packets.
    #[must_use]
    pub fn end_to_end() -> Self {
        Self {
            packet_type: PacketType::EndToEnd,
            ..Self::new()
        }
    }

    /// Sets the salt value.
    #[must_use]
    pub const fn with_salt(mut self, salt: u64) -> Self {
        self.salt = salt;
        self
    }

    /// Sets the session ID.
    #[must_use]
    pub const fn with_session_id(mut self, session_id: u64) -> Self {
        self.session_id = session_id;
        self
    }

    /// Sets the message ID.
    #[must_use]
    pub const fn with_message_id(mut self, message_id: MessageId) -> Self {
        self.message_id = message_id;
        self
    }

    /// Sets the sequence number.
    #[must_use]
    pub const fn with_seq_no(mut self, seq_no: i32) -> Self {
        self.seq_no = seq_no;
        self
    }

    /// Sets the MTProto version.
    #[must_use]
    pub const fn with_version(mut self, version: i32) -> Self {
        self.version = version;
        self
    }

    /// Sets the no-crypto flag.
    #[must_use]
    pub const fn with_no_crypto(mut self, no_crypto: bool) -> Self {
        self.no_crypto_flag = no_crypto;
        self
    }

    /// Sets the is-creator flag (for e2e packets).
    #[must_use]
    pub const fn with_creator(mut self, is_creator: bool) -> Self {
        self.is_creator = is_creator;
        self
    }

    /// Sets whether to check mod4 alignment.
    #[must_use]
    pub const fn with_check_mod4(mut self, check: bool) -> Self {
        self.check_mod4 = check;
        self
    }

    /// Sets whether to use random padding.
    #[must_use]
    pub const fn with_random_padding(mut self, random: bool) -> Self {
        self.use_random_padding = random;
        self
    }

    /// Sets the packet type.
    #[must_use]
    pub const fn with_packet_type(mut self, packet_type: PacketType) -> Self {
        self.packet_type = packet_type;
        self
    }

    /// Returns true if this is an end-to-end encrypted packet.
    #[must_use]
    pub const fn is_end_to_end(&self) -> bool {
        matches!(self.packet_type, PacketType::EndToEnd)
    }

    /// Returns true if this is a common (server) packet.
    #[must_use]
    pub const fn is_common(&self) -> bool {
        matches!(self.packet_type, PacketType::Common)
    }

    /// Returns true if the message has the acknowledgment bit set.
    #[must_use]
    pub const fn is_ack(&self) -> bool {
        self.message_ack & (1 << 31) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_type_default() {
        assert_eq!(PacketType::default(), PacketType::Common);
    }

    #[test]
    fn test_packet_info_default() {
        let info = PacketInfo::default();
        assert_eq!(info.packet_type, PacketType::Common);
        assert_eq!(info.message_ack, 0);
        assert_eq!(info.salt, 0);
        assert_eq!(info.session_id, 0);
        assert_eq!(info.seq_no, 0);
        assert_eq!(info.version, 2);
        assert!(!info.no_crypto_flag);
        assert!(!info.is_creator);
        assert!(info.check_mod4);
        assert!(!info.use_random_padding);
    }

    #[test]
    fn test_packet_info_new() {
        let info = PacketInfo::new();
        assert_eq!(info.packet_type, PacketType::Common);
    }

    #[test]
    fn test_packet_info_builder() {
        let msg_id = MessageId::from_u64(0x62000000_00000000);
        let info = PacketInfo::new()
            .with_salt(12345)
            .with_session_id(67890)
            .with_message_id(msg_id)
            .with_seq_no(42)
            .with_version(2);

        assert_eq!(info.salt, 12345);
        assert_eq!(info.session_id, 67890);
        assert_eq!(info.message_id, msg_id);
        assert_eq!(info.seq_no, 42);
        assert_eq!(info.version, 2);
    }

    #[test]
    fn test_packet_info_end_to_end() {
        let info = PacketInfo::end_to_end();
        assert!(info.is_end_to_end());
        assert!(!info.is_common());
    }

    #[test]
    fn test_packet_info_common() {
        let info = PacketInfo::common();
        assert!(info.is_common());
        assert!(!info.is_end_to_end());
    }

    #[test]
    fn test_message_ack() {
        let mut info = PacketInfo::new();
        assert!(!info.is_ack());

        info.message_ack = 1 << 31;
        assert!(info.is_ack());
    }

    #[test]
    fn test_packet_info_clone() {
        let info1 = PacketInfo::new().with_salt(123);
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_packet_info_debug() {
        let info = PacketInfo::new().with_salt(123);
        let debug_str = format!("{info:?}");
        assert!(debug_str.contains("PacketInfo"));
        assert!(debug_str.contains("123"));
    }
}
